//! ✏️ Layout play app command — `patch-document`.

use crate::mutations::change_data_fields::ChangeDataFields;
use crate::mutations::change_print_target::ChangePrintTarget;
use crate::mutations::rename_layout::RenameLayout;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-document")]
pub struct PatchDocument {
    pub field: String,
    pub value: String,
}

fn present(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub fn handle(payload: &PatchDocument, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let mutation = match payload.field.as_str() {
        "name" => LayoutMutation::RenameLayout(RenameLayout { new_name: payload.value.clone() }),
        "printTarget" => LayoutMutation::ChangePrintTarget(ChangePrintTarget { new_print_target: present(&payload.value) }),
        "dataFields" => LayoutMutation::ChangeDataFields(ChangeDataFields { new_json: present(&payload.value) }),
        _ => return Ok(Emit::default()),
    };
    Ok(Emit::mutations(vec![mutation]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
