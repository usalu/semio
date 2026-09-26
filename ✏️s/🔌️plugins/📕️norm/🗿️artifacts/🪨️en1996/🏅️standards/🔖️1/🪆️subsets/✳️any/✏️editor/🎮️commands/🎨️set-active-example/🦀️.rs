//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1996Mutation, En1996Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1996Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1996Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1996Snapshot::default(),
        id if id == crate::loadbearing_wall::ID => crate::loadbearing_wall::snapshot(),
        id if id == crate::multi_fail_masonry::ID => crate::multi_fail_masonry::snapshot(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
