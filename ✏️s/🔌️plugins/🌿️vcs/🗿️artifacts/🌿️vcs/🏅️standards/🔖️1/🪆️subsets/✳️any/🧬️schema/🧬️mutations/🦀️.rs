//! 🌿️ Transparent VCS semantic mutation aggregate.

use crate::VcsSnapshot;

pub use super::add_tag::{add_tag, AddTag};
pub use super::change_counter::{change_counter, ChangeCounter};
pub use super::change_notes::{change_notes, ChangeNotes};
pub use super::change_status::{change_status, ChangeStatus};
pub use super::remove_tag::{remove_tag, RemoveTag};
pub use super::rename_vcs::{rename_vcs, RenameVcs};
pub use crate::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = VcsSnapshot, diff = crate::VcsDiff, schema = "vcs.vcs")]
pub enum VcsDemoMutation {
    RenameVcs(RenameVcs),
    ChangeCounter(ChangeCounter),
    ChangeNotes(ChangeNotes),
    ChangeStatus(ChangeStatus),
    AddTag(AddTag),
    RemoveTag(RemoveTag),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
