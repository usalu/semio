//! 🔧️ Process3d artifact — OpText/OpBinary codecs + grammar for serializing `Process3dMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::process3d::schema::mutations::Process3dMutation;
use crate::artifacts::process3d::schema::mutations::{
    change_cursor, change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps, replace_machine_capabilities,
    replace_step_measure, replace_stock_solid,
};
use crate::artifacts::process3d::{Capability, Pose, StepOrigin, WorkshopMachine};
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `Process3dMutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `Process3dMutation` itself,
/// and every consumer matching on it, is completely untouched.
///
/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `ProcessStep`/`ProcessMeasure`
/// dropped their `dsl` derives (now ephemeral working-scene types, containing `WorkingSolid` —
/// itself never `dsl::DslField`, same wall every composed-child migration hits). `CreateStep`/
/// `ReplaceStepMeasure` carry those as JSON-then-hex strings now (matching `📐️cad`'s
/// `enc_json`/`dec_json` convention for structured fields with no dedicated grammar) — harmless
/// since both are DOCUMENTED NO-OPS pending a resolver anyway (see the sibling `🧬️mutations/**`
/// triads' own doc comments); `ReplaceStockSolid.new_solid` is now a real
/// `store::ArtifactChild<SemioBrepSnapshot>` handle, JSON-encoded the same way (it already derives
/// `Serialize`/`Deserialize` regardless of `S`).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Process3dMutationDsl {
    CreateStep {
        index: usize,
        step_json: String,
    },
    DeleteStep {
        id: String,
    },
    RenameStep {
        id: String,
        new_label: String,
    },
    ChangeStepEnabled {
        id: String,
        new_enabled: bool,
    },
    ChangeStepOrigin {
        id: String,
        #[dsl(block)]
        new_origin: Option<StepOrigin>,
    },
    ReplaceStepMeasure {
        id: String,
        new_measure_json: String,
    },
    ReorderSteps {
        id: String,
        to_index: usize,
    },
    CreateMachine {
        index: usize,
        #[dsl(block)]
        machine: WorkshopMachine,
    },
    DeleteMachine {
        id: String,
    },
    RenameMachine {
        id: String,
        new_label: String,
    },
    ChangeMachineIcon {
        id: String,
        new_icon_id: String,
    },
    ReplaceMachineCapabilities {
        id: String,
        new_capabilities: Vec<Capability>,
    },
    MoveStock {
        #[dsl(block)]
        new_pose: Pose,
    },
    ChangeStockLabel {
        new_label: String,
    },
    ReplaceStockSolid {
        new_solid_json: String,
    },
    ChangeCursor {
        new_resolved_up_to: Option<usize>,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for Process3dMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Process3dMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn process3d_mutation_to_dsl(mutation: &Process3dMutation) -> Process3dMutationDsl {
    match mutation {
        Process3dMutation::CreateStep(payload) => Process3dMutationDsl::CreateStep { index: payload.index, step_json: serde_json::to_string(&payload.step).expect("ProcessStep is always JSON-serializable") },
        Process3dMutation::DeleteStep(payload) => Process3dMutationDsl::DeleteStep { id: payload.id.clone() },
        Process3dMutation::RenameStep(payload) => Process3dMutationDsl::RenameStep { id: payload.id.clone(), new_label: payload.new_label.clone() },
        Process3dMutation::ChangeStepEnabled(payload) => Process3dMutationDsl::ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.new_enabled },
        Process3dMutation::ChangeStepOrigin(payload) => Process3dMutationDsl::ChangeStepOrigin { id: payload.id.clone(), new_origin: payload.new_origin.clone() },
        Process3dMutation::ReplaceStepMeasure(payload) => Process3dMutationDsl::ReplaceStepMeasure { id: payload.id.clone(), new_measure_json: serde_json::to_string(&payload.new_measure).expect("ProcessMeasure is always JSON-serializable") },
        Process3dMutation::ReorderSteps(payload) => Process3dMutationDsl::ReorderSteps { id: payload.id.clone(), to_index: payload.to_index },
        Process3dMutation::CreateMachine(payload) => Process3dMutationDsl::CreateMachine { index: payload.index, machine: payload.machine.clone() },
        Process3dMutation::DeleteMachine(payload) => Process3dMutationDsl::DeleteMachine { id: payload.id.clone() },
        Process3dMutation::RenameMachine(payload) => Process3dMutationDsl::RenameMachine { id: payload.id.clone(), new_label: payload.new_label.clone() },
        Process3dMutation::ChangeMachineIcon(payload) => Process3dMutationDsl::ChangeMachineIcon { id: payload.id.clone(), new_icon_id: payload.new_icon_id.clone() },
        Process3dMutation::ReplaceMachineCapabilities(payload) => Process3dMutationDsl::ReplaceMachineCapabilities { id: payload.id.clone(), new_capabilities: payload.new_capabilities.clone() },
        Process3dMutation::MoveStock(payload) => Process3dMutationDsl::MoveStock { new_pose: payload.new_pose.clone() },
        Process3dMutation::ChangeStockLabel(payload) => Process3dMutationDsl::ChangeStockLabel { new_label: payload.new_label.clone() },
        Process3dMutation::ReplaceStockSolid(payload) => Process3dMutationDsl::ReplaceStockSolid { new_solid_json: serde_json::to_string(&payload.new_solid).expect("ArtifactChild is always JSON-serializable") },
        Process3dMutation::ChangeCursor(payload) => Process3dMutationDsl::ChangeCursor { new_resolved_up_to: payload.new_resolved_up_to },
    }
}

fn process3d_mutation_from_dsl(mutation: Process3dMutationDsl) -> Process3dMutation {
    match mutation {
        Process3dMutationDsl::CreateStep { index, step_json } => Process3dMutation::CreateStep(create_step::mutation::CreateStep { index, step: serde_json::from_str(&step_json).expect("valid ProcessStep json") }),
        Process3dMutationDsl::DeleteStep { id } => Process3dMutation::DeleteStep(delete_step::mutation::DeleteStep { id }),
        Process3dMutationDsl::RenameStep { id, new_label } => Process3dMutation::RenameStep(rename_step::mutation::RenameStep { id, new_label }),
        Process3dMutationDsl::ChangeStepEnabled { id, new_enabled } => Process3dMutation::ChangeStepEnabled(change_step_enabled::mutation::ChangeStepEnabled { id, new_enabled }),
        Process3dMutationDsl::ChangeStepOrigin { id, new_origin } => Process3dMutation::ChangeStepOrigin(change_step_origin::mutation::ChangeStepOrigin { id, new_origin }),
        Process3dMutationDsl::ReplaceStepMeasure { id, new_measure_json } => {
            Process3dMutation::ReplaceStepMeasure(replace_step_measure::mutation::ReplaceStepMeasure { id, new_measure: serde_json::from_str(&new_measure_json).expect("valid ProcessMeasure json") })
        }
        Process3dMutationDsl::ReorderSteps { id, to_index } => Process3dMutation::ReorderSteps(reorder_steps::mutation::ReorderSteps { id, to_index }),
        Process3dMutationDsl::CreateMachine { index, machine } => Process3dMutation::CreateMachine(create_machine::mutation::CreateMachine { index, machine }),
        Process3dMutationDsl::DeleteMachine { id } => Process3dMutation::DeleteMachine(delete_machine::mutation::DeleteMachine { id }),
        Process3dMutationDsl::RenameMachine { id, new_label } => Process3dMutation::RenameMachine(rename_machine::mutation::RenameMachine { id, new_label }),
        Process3dMutationDsl::ChangeMachineIcon { id, new_icon_id } => Process3dMutation::ChangeMachineIcon(change_machine_icon::mutation::ChangeMachineIcon { id, new_icon_id }),
        Process3dMutationDsl::ReplaceMachineCapabilities { id, new_capabilities } => Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::mutation::ReplaceMachineCapabilities { id, new_capabilities }),
        Process3dMutationDsl::MoveStock { new_pose } => Process3dMutation::MoveStock(move_stock::mutation::MoveStock { new_pose }),
        Process3dMutationDsl::ChangeStockLabel { new_label } => Process3dMutation::ChangeStockLabel(change_stock_label::mutation::ChangeStockLabel { new_label }),
        Process3dMutationDsl::ReplaceStockSolid { new_solid_json } => Process3dMutation::ReplaceStockSolid(replace_stock_solid::mutation::ReplaceStockSolid { new_solid: serde_json::from_str(&new_solid_json).expect("valid ArtifactChild json") }),
        Process3dMutationDsl::ChangeCursor { new_resolved_up_to } => Process3dMutation::ChangeCursor(change_cursor::mutation::ChangeCursor { new_resolved_up_to }),
    }
}

impl OpText for Process3dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(process3d_mutation_from_dsl(<Process3dMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Process3dMutationDsl as OpText>::print_op(&process3d_mutation_to_dsl(self))
    }
}

/// 🧱️ Delegates binary ownership to the bounded structural Process3d codec; text conversion
/// remains isolated to explicit text routes.
impl protocol::OpBinary for Process3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        crate::artifacts::process3d::spr::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        crate::artifacts::process3d::spr::decode_op(bytes)
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::process3d::schema::mutations::*;
    use crate::artifacts::process3d::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, Pose, Process3dSnapshot, WorkingSolid};
    use protocol::Mutation;

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn circular_saw_machine() -> WorkshopMachine {
        use crate::artifacts::process3d::{CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity};
        WorkshopMachine {
            id: "circularSaw".into(),
            label: "Circular Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: Some("wood".into()),
            capabilities: vec![Capability {
                id: "crosscut".into(),
                label: "Crosscut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
                rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }],
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_create_step() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::CreateStep(create_step::mutation::CreateStep { index: 0, step: cut_step("cut-1") }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_delete_step() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::DeleteStep(delete_step::mutation::DeleteStep { id: "cut-1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_rename_step() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::RenameStep(rename_step::mutation::RenameStep { id: "cut-1".into(), new_label: "Renamed".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_step_enabled() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepEnabled(change_step_enabled::mutation::ChangeStepEnabled { id: "cut-1".into(), new_enabled: false }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_step_origin_set() {
        let new_origin = Some(StepOrigin { machine_id: "tableSaw".into(), capability_id: "crosscut".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepOrigin(change_step_origin::mutation::ChangeStepOrigin { id: "cut-1".into(), new_origin }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_step_origin_clear() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepOrigin(change_step_origin::mutation::ChangeStepOrigin { id: "cut-1".into(), new_origin: None }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_replace_step_measure() {
        let new_measure = ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } };
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceStepMeasure(replace_step_measure::mutation::ReplaceStepMeasure { id: "cut-1".into(), new_measure }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_reorder_steps() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReorderSteps(reorder_steps::mutation::ReorderSteps { id: "cut-1".into(), to_index: 2 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_create_machine() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::CreateMachine(create_machine::mutation::CreateMachine { index: 0, machine: circular_saw_machine() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_delete_machine() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::DeleteMachine(delete_machine::mutation::DeleteMachine { id: "circularSaw".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_rename_machine() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::RenameMachine(rename_machine::mutation::RenameMachine { id: "circularSaw".into(), new_label: "Big Saw".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_machine_icon() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeMachineIcon(change_machine_icon::mutation::ChangeMachineIcon { id: "circularSaw".into(), new_icon_id: "drill".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_replace_machine_capabilities_full() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::mutation::ReplaceMachineCapabilities {
            id: "circularSaw".into(),
            new_capabilities: circular_saw_machine().capabilities,
        }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_replace_machine_capabilities_empty() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::mutation::ReplaceMachineCapabilities { id: "circularSaw".into(), new_capabilities: vec![] }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_move_stock() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::MoveStock(move_stock::mutation::MoveStock { new_pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_stock_label() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStockLabel(change_stock_label::mutation::ChangeStockLabel { new_label: "Timber Beam".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_replace_stock_solid() {
        let new_solid = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceStockSolid(replace_stock_solid::mutation::ReplaceStockSolid { new_solid }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_cursor_some() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeCursor(change_cursor::mutation::ChangeCursor { new_resolved_up_to: Some(3) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn process3d_op_text_round_trips_change_cursor_none() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeCursor(change_cursor::mutation::ChangeCursor { new_resolved_up_to: None }));
    }

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `CreateStep` is a documented
    /// no-op now (`steps` composes an `s.stdio.semio.flow` CHILD HANDLE — no resolver, see
    /// `ProcessWorkingScene`'s doc comment), so per the sanctioned `MutationKind::inverse` contract
    /// ("a mutation with nothing to undo returns `Vec::new()`"), there is nothing to invert.
    #[semio_framework_async_macros::async_test]
    async fn inverse_of_create_step_is_empty_since_it_is_a_documented_no_op() {
        let snapshot = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(create_step::mutation::CreateStep { index: 0, step: cut_step("a") });
        assert!(mutation.inverse(&snapshot).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_of_create_machine_is_delete_machine() {
        let snapshot = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateMachine(create_machine::mutation::CreateMachine { index: 0, machine: circular_saw_machine() });
        let inverse = mutation.inverse(&snapshot);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dMutation::DeleteMachine(payload) => assert_eq!(payload.id, "circularSaw"),
            _ => panic!("expected DeleteMachine"),
        }
    }

    /// 📸️ Sanity: the stock fields (unrelated to the mutation vocabulary) still round-trip through
    /// the artifact's DSL document codec.
    #[semio_framework_async_macros::async_test]
    async fn imported_mesh_stock_round_trips_document_dsl() {
        let stock_solid = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }));
        let snapshot = Process3dSnapshot { stock_id: "stock".into(), stock_label: "Imported GLB".into(), stock_pose: Pose::default(), stock_solid, ..empty_process3d_snapshot() };
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }
}
//#endregion 🧪️Tests
