//! 📚️ 📚️ FEM 3D app commands command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ `examples::demo::ID` (the id the shell switcher dispatches) loads the bundled `.fem3d` fixture; any other id resets to an empty document —
/// fem3d only ships the one example (mirrors the pre-migration `handle_action` behavior). Also resets
/// the view config back to its defaults, expressed as the two granular field mutations that together
/// cover every `Fem3dConfig` field: `Fem3dConfigPreparationFactory` refuses a whole-config `Snapshot`
/// row on the Config publication lane (unbounded envelope), so a `Snapshot` here would fault this tool
/// at publication now that it runs as a retained job.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `editor::fem3d::reset_document_effect`
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let document = if payload.example_id == crate::examples::demo::ID { <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT).unwrap_or_default() } else { Fem3dSnapshot::default() };
    eprintln!("[DEBUG] fem3d setActiveExample id={} nodes={} elements={} solids={}", payload.example_id, document.nodes.len(), document.elements.len(), document.solids.len());
    let defaults = Fem3dConfig::default();
    Ok(Emit {
        effects: vec![crate::editor::fem3d::reset_document_effect(&document)],
        config_mutations: vec![
            Fem3dConfigMutation::SetResultDisplay { source_id: defaults.result_source_id.clone(), mode: defaults.result_mode.clone(), mode_index: defaults.result_mode_index },
            Fem3dConfigMutation::SetCamera { camera: defaults.camera },
        ],
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
