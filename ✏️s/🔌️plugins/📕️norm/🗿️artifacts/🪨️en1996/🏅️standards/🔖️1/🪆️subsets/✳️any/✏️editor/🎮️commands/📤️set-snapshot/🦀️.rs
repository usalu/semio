//! 📤️ EN 1996 play app command — replace the whole compliance document.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::En1996Mutation;
use crate::En1996Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: En1996Snapshot,
}

pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, En1996Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1996Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(En1996Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
