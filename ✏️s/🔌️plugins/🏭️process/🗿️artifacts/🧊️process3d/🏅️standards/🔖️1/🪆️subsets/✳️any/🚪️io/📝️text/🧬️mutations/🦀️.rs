//! 🔧️ Process3d artifact — OpText/OpBinary codecs + grammar for serializing `Process3dMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::Process3dMutation;
use crate::schema::mutations::{
    change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps, replace_machine_capabilities,
    replace_step_measure, replace_stock_solid,
};
use crate::{Capability, Pose, StepOrigin, WorkshopMachine};
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
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
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for Process3dMutationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for Process3dMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn process3d_mutation_to_dsl(mutation: &Process3dMutation) -> Process3dMutationDsl {
    match mutation {
        Process3dMutation::CreateStep(payload) => Process3dMutationDsl::CreateStep { index: payload.index, step_json: semio_framework_pack_json::to_json_string(&payload.step) },
        Process3dMutation::DeleteStep(payload) => Process3dMutationDsl::DeleteStep { id: payload.id.clone() },
        Process3dMutation::RenameStep(payload) => Process3dMutationDsl::RenameStep { id: payload.id.clone(), new_label: payload.new_label.clone() },
        Process3dMutation::ChangeStepEnabled(payload) => Process3dMutationDsl::ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.new_enabled },
        Process3dMutation::ChangeStepOrigin(payload) => Process3dMutationDsl::ChangeStepOrigin { id: payload.id.clone(), new_origin: payload.new_origin.clone() },
        Process3dMutation::ReplaceStepMeasure(payload) => Process3dMutationDsl::ReplaceStepMeasure { id: payload.id.clone(), new_measure_json: semio_framework_pack_json::to_json_string(&payload.new_measure) },
        Process3dMutation::ReorderSteps(payload) => Process3dMutationDsl::ReorderSteps { id: payload.id.clone(), to_index: payload.to_index },
        Process3dMutation::CreateMachine(payload) => Process3dMutationDsl::CreateMachine { index: payload.index, machine: payload.machine.clone() },
        Process3dMutation::DeleteMachine(payload) => Process3dMutationDsl::DeleteMachine { id: payload.id.clone() },
        Process3dMutation::RenameMachine(payload) => Process3dMutationDsl::RenameMachine { id: payload.id.clone(), new_label: payload.new_label.clone() },
        Process3dMutation::ChangeMachineIcon(payload) => Process3dMutationDsl::ChangeMachineIcon { id: payload.id.clone(), new_icon_id: payload.new_icon_id.clone() },
        Process3dMutation::ReplaceMachineCapabilities(payload) => Process3dMutationDsl::ReplaceMachineCapabilities { id: payload.id.clone(), new_capabilities: payload.new_capabilities.clone() },
        Process3dMutation::MoveStock(payload) => Process3dMutationDsl::MoveStock { new_pose: payload.new_pose.clone() },
        Process3dMutation::ChangeStockLabel(payload) => Process3dMutationDsl::ChangeStockLabel { new_label: payload.new_label.clone() },
        Process3dMutation::ReplaceStockSolid(payload) => Process3dMutationDsl::ReplaceStockSolid { new_solid_json: semio_framework_pack_json::to_json_string(&payload.new_solid) },
    }
}

fn process3d_mutation_from_dsl(mutation: Process3dMutationDsl) -> Process3dMutation {
    match mutation {
        Process3dMutationDsl::CreateStep { index, step_json } => Process3dMutation::CreateStep(create_step::CreateStep { index, step: semio_framework_pack_json::from_json_str(&step_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("valid ProcessStep json") }),
        Process3dMutationDsl::DeleteStep { id } => Process3dMutation::DeleteStep(delete_step::DeleteStep { id }),
        Process3dMutationDsl::RenameStep { id, new_label } => Process3dMutation::RenameStep(rename_step::RenameStep { id, new_label }),
        Process3dMutationDsl::ChangeStepEnabled { id, new_enabled } => Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id, new_enabled }),
        Process3dMutationDsl::ChangeStepOrigin { id, new_origin } => Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id, new_origin }),
        Process3dMutationDsl::ReplaceStepMeasure { id, new_measure_json } => {
            Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure { id, new_measure: semio_framework_pack_json::from_json_str(&new_measure_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("valid ProcessMeasure json") })
        }
        Process3dMutationDsl::ReorderSteps { id, to_index } => Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id, to_index }),
        Process3dMutationDsl::CreateMachine { index, machine } => Process3dMutation::CreateMachine(create_machine::CreateMachine { index, machine }),
        Process3dMutationDsl::DeleteMachine { id } => Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id }),
        Process3dMutationDsl::RenameMachine { id, new_label } => Process3dMutation::RenameMachine(rename_machine::RenameMachine { id, new_label }),
        Process3dMutationDsl::ChangeMachineIcon { id, new_icon_id } => Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id, new_icon_id }),
        Process3dMutationDsl::ReplaceMachineCapabilities { id, new_capabilities } => Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities }),
        Process3dMutationDsl::MoveStock { new_pose } => Process3dMutation::MoveStock(move_stock::MoveStock { new_pose }),
        Process3dMutationDsl::ChangeStockLabel { new_label } => Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label }),
        Process3dMutationDsl::ReplaceStockSolid { new_solid_json } => {
            Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid { new_solid: semio_framework_pack_json::from_json_str(&new_solid_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("valid ArtifactChild json") })
        }
    }
}

impl OpText for Process3dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
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
        crate::standards::v1::subsets::any::io::binary::mutations::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        crate::standards::v1::subsets::any::io::binary::mutations::decode_op(bytes)
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::change_machine_icon;
use crate::standards::v1::subsets::any::schema::mutations::change_step_enabled;
use crate::standards::v1::subsets::any::schema::mutations::change_step_origin;
use crate::standards::v1::subsets::any::schema::mutations::change_stock_label;
use crate::standards::v1::subsets::any::schema::mutations::create_machine;
use crate::standards::v1::subsets::any::schema::mutations::create_step;
use crate::standards::v1::subsets::any::schema::mutations::delete_machine;
use crate::standards::v1::subsets::any::schema::mutations::delete_step;
use crate::standards::v1::subsets::any::schema::mutations::move_stock;
use crate::standards::v1::subsets::any::schema::mutations::rename_machine;
use crate::standards::v1::subsets::any::schema::mutations::rename_step;
use crate::standards::v1::subsets::any::schema::mutations::reorder_steps;
use crate::standards::v1::subsets::any::schema::mutations::replace_machine_capabilities;
use crate::standards::v1::subsets::any::schema::mutations::replace_step_measure;
use crate::standards::v1::subsets::any::schema::mutations::replace_stock_solid;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Process3dMutation` nor
/// `Process3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔮️oracles/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn process3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Process3dSnapshot, String> {
        let decoded: Process3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Process3dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_pack_json::object([
        ("base".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&expected))),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&applied))),
        ("diff".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&forward.messages().to_vec()))),
        ("inverseSteps".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_codec::*;
