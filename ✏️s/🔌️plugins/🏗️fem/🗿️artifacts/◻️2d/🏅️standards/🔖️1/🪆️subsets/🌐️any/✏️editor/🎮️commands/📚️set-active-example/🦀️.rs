//! 📚️ 📚️ Fem2d play app commands command — `set-active-example`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ The bundled example's own `ExampleSource` id (`📚️examples/🎬️demo`) loads that fixture; any other
/// id resets to an empty document. The id is not a local literal because the shell's navbar switcher
/// dispatches this action straight from `PluginManifest.examples`, whose entries carry exactly that id.
/// Also resets camera and result display to their defaults as two granular rows — never a whole-config
/// `Snapshot`, which would also wipe the session locale.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `editor::fem2d::reset_document_effect`
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let document = if payload.example_id == crate::examples::demo::ID {
        Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot())
    } else {
        crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot()
    };
    eprintln!("[DEBUG] fem2d setActiveExample '{}': loading nodes={} elements={} regions={} loadCases={}", payload.example_id, document.nodes.len(), document.elements.len(), document.regions.len(), document.load_cases.len());
    let defaults = Fem2dConfig::default();
    Ok(Emit {
        effects: vec![crate::editor::fem2d::reset_document_effect(&document)],
        config_mutations: vec![
            Fem2dConfigMutation::SetResultDisplay { source_id: defaults.result_source_id.clone(), mode: defaults.result_mode.clone(), mode_index: defaults.result_mode_index },
            Fem2dConfigMutation::SetCamera { camera: defaults.camera },
        ],
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
