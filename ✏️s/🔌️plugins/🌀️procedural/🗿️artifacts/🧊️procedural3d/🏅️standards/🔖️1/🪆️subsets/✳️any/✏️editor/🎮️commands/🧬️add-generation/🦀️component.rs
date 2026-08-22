//! 🧬️ 🧬️ Procedural3d play app commands command — `add-generation`.

use crate::artifacts::procedural3d::op::{generation_mutation_to_procedural3d, Procedural3dMutation};
use crate::artifacts::procedural3d::schema::evaluate_generation_preview;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{apply_generation_mutation, generation_operations, selected_generation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `flow::playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&Value>, projection: &Procedural3dSnapshot, cfg: &Procedural3dConfig) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.selected_generation_id.clone();
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Emit::default();
    };
    for operation in &operations {
        apply_generation_mutation(&mut state, operation);
    }
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(&projection.fixture, &selected.values));
    let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit {
        artifact_mutations: operations.into_iter().map(generation_mutation_to_procedural3d).collect(),
        config_mutations: vec![Procedural3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
        coalesce_key,
        ..Default::default()
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddGeneration
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
//#endregion 🔖️RemoveGeneration

//#region 🔖️RenameGeneration
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
//#endregion 🔖️SelectGeneration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-generation")]
pub struct AddGeneration {}

pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(handle_generation("addGeneration", None, doc.snapshot, cfg.snapshot))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::commands::select_generation;
    use crate::editor::procedural3d::testkit::{app, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddGeneration(AddGeneration {}), |app| app.snapshot().expect("snapshot").generation.generations.len(), 0, 1);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        assert!(crate::editor::procedural3d::testkit::render(&mut app, crate::editor::procedural3d::modes::generate::windows::generations::PROCEDURAL_3D_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }

    #[test]
    fn select_generation_does_not_mutate_the_document() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::AddGeneration(AddGeneration {}));
        let before = app.snapshot().expect("snapshot");
        let generation_id = before.generation.generations.first().expect("generation").id.clone();
        dispatch(&mut app, Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
