//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1998Mutation, En1998Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the live document with the named example snapshot.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1998Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1998Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1998Snapshot::default(),
        id if id == crate::seismic_rc_frame::ID || id == "seismic-rc-frame" => crate::seismic_rc_frame::snapshot(),
        id if id == crate::seismic_rc_frame_fail::ID || id == "seismic-rc-frame-fail" => crate::seismic_rc_frame_fail::snapshot(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
