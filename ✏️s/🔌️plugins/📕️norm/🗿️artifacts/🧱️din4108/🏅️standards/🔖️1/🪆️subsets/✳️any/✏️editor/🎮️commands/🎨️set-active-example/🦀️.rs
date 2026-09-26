//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Din4108Mutation, Din4108Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the live document with the named example snapshot.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Din4108Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din4108Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => Din4108Snapshot::default(),
        "demo" | "compliant-etics-dwelling" => Din4108Snapshot::compliant_etics_dwelling(),
        "failing-thin-insulation" => Din4108Snapshot::failing_thin_insulation(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
