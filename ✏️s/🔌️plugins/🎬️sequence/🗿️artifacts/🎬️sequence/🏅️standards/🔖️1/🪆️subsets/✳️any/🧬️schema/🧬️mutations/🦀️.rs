//! 🎬️ Sequence semantic mutation aggregate and leaf detection registry.

use crate::diff::SequenceDiff;
use crate::SequenceSnapshot;

pub use crate::standards::v1::subsets::step::schema::mutations::change_step_collapsed::{change_step_collapsed, ChangeStepCollapsed};
pub use crate::standards::v1::subsets::dependency::schema::mutations::connect_steps::{connect_steps, ConnectSteps};
pub use crate::standards::v1::subsets::step::schema::mutations::create_step::{create_step, CreateStep};
pub use crate::standards::v1::subsets::step::schema::mutations::delete_step::{delete_step, DeleteStep};
pub use crate::standards::v1::subsets::dependency::schema::mutations::disconnect_steps::{disconnect_steps, DisconnectSteps};
pub use crate::standards::v1::subsets::step::schema::mutations::duplicate_step::{duplicate_step, DuplicateStep};
pub use crate::standards::v1::subsets::step::schema::mutations::edit_step_params::{edit_step_params, EditStepParams};
pub use crate::standards::v1::subsets::step::schema::mutations::move_step::{move_step, MoveStep};
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Closed sequence mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SequenceSnapshot, diff = SequenceDiff, schema = "sequence.sequence")]
pub enum SequenceMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    MoveStep(MoveStep),
    EditStepParams(EditStepParams),
    ChangeStepCollapsed(ChangeStepCollapsed),
    ConnectSteps(ConnectSteps),
    DisconnectSteps(DisconnectSteps),
    DuplicateStep(DuplicateStep),
}
impl neural_engine::ColdRetire for SequenceMutation {
    fn retire_cold(self) {
        match self {
            Self::CreateStep(value) => value.step.retire_cold(),
            Self::EditStepParams(value) => value.params.retire_cold(),
            Self::DeleteStep(_) | Self::MoveStep(_) | Self::ChangeStepCollapsed(_) | Self::ConnectSteps(_) | Self::DisconnectSteps(_) | Self::DuplicateStep(_) => {}
        }
    }
}
//#endregion 🔖️Aggregate

//#region 🔎️DetectionRegistry
pub const DETECTORS: &[SequenceMutationDetector] =
    &[
        crate::standards::v1::subsets::step::schema::mutations::create_step::detect,
        crate::standards::v1::subsets::step::schema::mutations::delete_step::detect,
        crate::standards::v1::subsets::step::schema::mutations::move_step::detect,
        crate::standards::v1::subsets::step::schema::mutations::edit_step_params::detect,
        crate::standards::v1::subsets::step::schema::mutations::change_step_collapsed::detect,
        crate::standards::v1::subsets::dependency::schema::mutations::connect_steps::detect,
        crate::standards::v1::subsets::dependency::schema::mutations::disconnect_steps::detect,
    ];
//#endregion 🔎️DetectionRegistry

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
