//! ✏️ Assembly editor — the FIRST authored `ArtifactEditor` surface for `s.assembly@1/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, packet W2-P5). Assembly had zero document apps,
//! so there is no app tree to migrate — this is authored fresh straight against `AssemblySnapshot`'s
//! own real shape (seed/slots/edges/modules/weights/rules; the SOLVE itself is an inference, never
//! authored here). One real window, `🌳️structure` (`TreeWindowKit`), rendering/editing the whole
//! problem spec. Every command maps 1:1 onto a real `AssemblyMutation` builder from the schema tree
//! (`create_slot`/`delete_slot`/`create_rule`/`delete_rule`/`connect_slots`/`disconnect_slots`/
//! `change_weight`/`remove_weight`/`change_seed`) — no synthetic "set field" indirection, since the
//! domain's own mutations are already exactly this granular.

use crate::artifacts::assembly::mutations::{change_seed, change_weight, connect_slots, create_rule, create_slot, delete_rule, delete_slot, disconnect_slots, remove_weight};
use crate::artifacts::assembly::schema::snapshot::{AssemblyRule, AssemblySlot, AssemblySlotEdge};
use crate::artifacts::assembly::{AssemblyMutation, AssemblySnapshot, ASSEMBLY_DIALECT, ASSEMBLY_DOCUMENT_SCHEMA};
use crate::editor::assembly::modes::edit;
use crate::editor::assembly::modes::edit::windows::structure;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `AssemblyMutation` kind an editor UI
/// can trigger. `AssemblyRule::params` (a `SemioValue`, generic structured data) is left at its
/// default on `CreateRule` — a documented first-pass simplification, the same honest scope narrowing
/// `energy.model`'s own `SetStructureField` precedent uses for its two addressable leaves; the
/// underlying mutation already supports the full field, this command surface just doesn't expose an
/// editor affordance for it yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum AssemblyEditorCommand {
    #[dsl(key = "create-slot")]
    CreateSlot { index: usize, id: String, x: f64, y: f64, z: f64, pinned_module_id: Option<String> },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    #[dsl(key = "create-rule")]
    CreateRule { index: usize, id: String, module_a_id: String, module_b_id: String, allowed: bool },
    #[dsl(key = "delete-rule")]
    DeleteRule { id: String },
    #[dsl(key = "connect-slots")]
    ConnectSlots { index: usize, id: String, from_slot_id: String, to_slot_id: String },
    #[dsl(key = "disconnect-slots")]
    DisconnectSlots { id: String },
    #[dsl(key = "change-weight")]
    ChangeWeight { module_id: String, weight: f64 },
    #[dsl(key = "remove-weight")]
    RemoveWeight { module_id: String },
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct AssemblyEditor;

impl ArtifactEditor for AssemblyEditor {
    type Snapshot = AssemblySnapshot;
    type Mutation = AssemblyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AssemblyEditorCommand;

    const DIALECT: Dialect = ASSEMBLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ASSEMBLY_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> AssemblySnapshot {
        AssemblySnapshot::default()
    }

    /// ✏️ Dispatches straight onto the real schema-tree mutation builders — one `AssemblyMutation` per
    /// command, no `ReplaceModel`-style whole-document rewrite (unlike `energy.model`, this artifact's
    /// mutations are already field/id-addressed, so no working-scene decode/re-encode step is needed).
    async fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let (mutation, description) = match command {
            AssemblyEditorCommand::CreateSlot { index, id, x, y, z, pinned_module_id } => (create_slot(*index, AssemblySlot { id: id.clone(), x: *x, y: *y, z: *z, pinned_module_id: pinned_module_id.clone() }), format!("Create slot {id}")),
            AssemblyEditorCommand::DeleteSlot { id } => (delete_slot(id.clone()), format!("Delete slot {id}")),
            AssemblyEditorCommand::CreateRule { index, id, module_a_id, module_b_id, allowed } => {
                (create_rule(*index, AssemblyRule { id: id.clone(), module_a_id: module_a_id.clone(), module_b_id: module_b_id.clone(), allowed: *allowed, ..Default::default() }), format!("Create rule {id}"))
            }
            AssemblyEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
            AssemblyEditorCommand::ConnectSlots { index, id, from_slot_id, to_slot_id } => {
                (connect_slots(*index, AssemblySlotEdge { id: id.clone(), from_slot_id: from_slot_id.clone(), to_slot_id: to_slot_id.clone() }), format!("Connect {from_slot_id} -> {to_slot_id}"))
            }
            AssemblyEditorCommand::DisconnectSlots { id } => (disconnect_slots(id.clone()), format!("Disconnect {id}")),
            AssemblyEditorCommand::ChangeWeight { module_id, weight } => (change_weight(module_id.clone(), *weight), format!("Set weight {module_id}")),
            AssemblyEditorCommand::RemoveWeight { module_id } => (remove_weight(module_id.clone()), format!("Remove weight {module_id}")),
            AssemblyEditorCommand::ChangeSeed { seed } => (change_seed(*seed), "Change seed".to_string()),
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_assembly_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(ASSEMBLY_DIALECT)
        .document(["semio", "assembly"])
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::ASSEMBLY_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_assembly_editor_builds_a_definition_for_the_editor_role() {
        let def = create_assembly_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, ASSEMBLY_DIALECT.into());
    }

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<AssemblyEditor as ArtifactEditor>::DIALECT, ASSEMBLY_DIALECT);
    }

    #[test]
    fn editor_declares_the_structure_window() {
        let def = create_assembly_editor();
        assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
