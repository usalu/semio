//! 🎒️ Declared companion payload arguments keep fixed semantics and addressed current siblings.

use super::*;
use semio_framework_value::DslValue;

/// 🧾️ A companion's authored text or a current value relative to the edited leaf's parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetFieldArgumentSource {
    Text(&'static str),
    Sibling(&'static str),
}

/// 🪢️ Completes one exact path-and-kind setter without replacing its selectors or authored value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFieldArgument {
    pub path: &'static str,
    pub kind: &'static str,
    pub argument: &'static str,
    pub source: SetFieldArgumentSource,
}

impl NormEditRules {
    pub(super) fn append_set_field_arguments(&self, rule: &SetFieldRule, tree: &DslValue, segments: &[PathSegment], path: &str, entries: &mut Vec<(String, DslValue)>) -> Result<(), String> {
        for argument in self.set_field_arguments.iter().filter(|argument| argument.path == rule.path && argument.kind == rule.kind) {
            if argument.argument.is_empty() || entries.iter().any(|(key, _)| key == argument.argument) {
                return Err(format!("duplicate or empty companion argument '{}' for {path}", argument.argument));
            }
            let value = match argument.source {
                SetFieldArgumentSource::Text(text) => DslValue::String(text.to_string()),
                SetFieldArgumentSource::Sibling(sibling) => {
                    let (_, parent) = segments.split_last().ok_or_else(|| format!("'{path}' has no parent for sibling '{sibling}'"))?;
                    let mut source = parent.to_vec();
                    source.extend(parse_path(sibling)?);
                    get_value_at_path(tree, &segment_path(&source))?.clone()
                }
            };
            entries.push((argument.argument.to_string(), value));
        }
        Ok(())
    }
}
