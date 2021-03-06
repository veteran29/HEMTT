use handlebars::*;
use serde_json::value::{Value as Json};

use std::collections::BTreeMap;

use crate::error::*;

pub fn render(source: &String, data: &BTreeMap<&'static str, Json>) -> String {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("date", Box::new(crate::helpers::date));
    handlebars.register_helper("git", Box::new(crate::helpers::git));
    handlebars.render_template(&source, &data).unwrap_or_print()
}
