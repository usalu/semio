//! 🩹 `setField`: sets one authored parameter of one or more entities of one kind. The field row of the entity table turns the edited text into the `set-*` mutation that owns the
//! parameter; a parameter without a mutation yet is refused, never half-applied.

use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-field")]
pub struct SetField {
    pub ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &SetField, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let ids: Vec<&String> = if payload.ids.is_empty() { ctx.selected.iter().chain(&ctx.library_selected).collect() } else { payload.ids.iter().collect() };
    if ids.is_empty() {
        return Ok(Emit::default());
    }
    let mut mutations = Vec::new();
    for id in ids {
        let row = kind_holding(doc.snapshot, id).ok_or_else(|| fault("bim.set.target-missing", format!("no entity '{id}'")))?;
        let field = row.fields.iter().find(|field| field.key == payload.field).ok_or_else(|| fault("bim.set.field-unknown", format!("a {} has no parameter '{}'", row.kind, payload.field)))?;
        let write = field.write.ok_or_else(|| fault("bim.set.read-only", format!("the parameter '{}' of a {} has no set mutation yet", payload.field, row.kind)))?;
        mutations.push(write(doc.snapshot, id, &payload.value).ok_or_else(|| fault("bim.set.value-invalid", format!("'{}' is not a valid value for '{}'", payload.value, payload.field)))?);
    }
    Ok(Emit::mutations(mutations))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
