//! 🧬️ 🧬️ Generation3d play app commands command — `add-generation`.

use crate::artifacts::generation3d::op::{generation_mutation_to_generation3d, Generation3dMutation};
use crate::artifacts::generation3d::schema::evaluate_generation_preview;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{apply_generation_mutation, generation_operations, selected_generation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `flow::playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&dsl::DslValue>, projection: &Generation3dSnapshot, cfg: &Generation3dConfig) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.as_state().clone();
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
        artifact_mutations: operations.into_iter().map(generation_mutation_to_generation3d).collect(),
        config_mutations: vec![Generation3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-generation")]
pub struct AddGeneration {}

pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("addGeneration", None, doc.snapshot, cfg.snapshot))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::commands::select_generation;
    use crate::editor::generation3d::testkit::{app, dispatch};
    use crate::editor::generation3d::Generation3dCommand;
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        assert_undo_redo_round_trip(&mut app, Generation3dCommand::AddGeneration(AddGeneration {}), |app| app.snapshot().expect("snapshot").generation.generations.len(), 0, 1);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        assert!(crate::editor::generation3d::testkit::render(&mut app, crate::editor::generation3d::modes::generate::windows::generations::GENERATION_3D_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }

    #[test]
    fn select_generation_does_not_mutate_the_document() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Generation3dCommand::AddGeneration(AddGeneration {}));
        let before = app.snapshot().expect("snapshot");
        let generation_id = before.generation.generations.first().expect("generation").id.clone();
        dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
