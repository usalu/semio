//! 📚️ 📚️ FEM 3D app commands command — `set-active-example`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ Loads the chosen example through the host document replacement effect while preserving exact-window preferences.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let text = match payload.example_id.as_str() {
        id if id == crate::examples::demo::ID => Some(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT),
        id if id == crate::examples::concrete_forest::ID => Some(crate::examples::concrete_forest::PRIMARY_TEXT),
        id if id == crate::examples::house::ID => Some(crate::examples::house::PRIMARY_TEXT),
        _ => None,
    };
    let document = text.map(|text| <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap_or_default()).unwrap_or_default();
    eprintln!("[DEBUG] fem3d setActiveExample id={} nodes={} elements={} solids={}", payload.example_id, document.nodes.len(), document.elements.len(), document.solids.len());
    Ok(Emit { effects: vec![crate::editor::fem3d::reset_document_effect(&document)], ..Default::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
