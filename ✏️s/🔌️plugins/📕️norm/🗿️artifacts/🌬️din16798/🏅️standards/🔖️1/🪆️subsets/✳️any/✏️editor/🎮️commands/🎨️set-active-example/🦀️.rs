//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Din16798Mutation, Din16798Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the live document with a named bundled example, or clears when id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Din16798Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din16798Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => Din16798Snapshot::default(),
        "demo" | "compliant-office" => Din16798Snapshot::compliant_office(),
        "noncompliant-office" => Din16798Snapshot::noncompliant_office(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
