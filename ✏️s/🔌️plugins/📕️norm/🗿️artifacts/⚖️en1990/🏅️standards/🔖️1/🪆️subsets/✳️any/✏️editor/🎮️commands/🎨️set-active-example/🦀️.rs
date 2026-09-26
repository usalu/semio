//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1990Snapshot, En1990Mutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🎨️ Replaces the live document with the named example's `PRIMARY_TEXT`, or clears it when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1990Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1990Mutation, NoConfigMutation>, Fault> {
    let text = match payload.example_id.trim() {
        "" => <En1990Snapshot as store::ArtifactDsl>::print_dsl(&En1990Snapshot::default()),
        id if id == crate::standards::v1::subsets::any::examples::high_consequence_office::ID => crate::standards::v1::subsets::any::examples::high_consequence_office::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::road_bridge_compliant::ID => crate::standards::v1::subsets::any::examples::road_bridge_compliant::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::road_bridge_failing::ID => crate::standards::v1::subsets::any::examples::road_bridge_failing::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::accidental_seismic_compliant::ID => crate::standards::v1::subsets::any::examples::accidental_seismic_compliant::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::accidental_seismic_failing::ID => crate::standards::v1::subsets::any::examples::accidental_seismic_failing::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::fatigue_compliant::ID => crate::standards::v1::subsets::any::examples::fatigue_compliant::PRIMARY_TEXT.to_string(),
        id if id == crate::standards::v1::subsets::any::examples::fatigue_failing::ID => crate::standards::v1::subsets::any::examples::fatigue_failing::PRIMARY_TEXT.to_string(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { text }, doc, cfg)
}
//#endregion 🔖️Handler
