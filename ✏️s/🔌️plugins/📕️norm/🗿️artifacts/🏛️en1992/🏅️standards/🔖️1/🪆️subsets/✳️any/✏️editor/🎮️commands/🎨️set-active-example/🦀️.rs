//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1992Snapshot, En1992Mutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1992Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1992Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1992Snapshot::default(),
        id if id == crate::compliant_office_frame::ID => <En1992Snapshot as store::ArtifactDsl>::parse_dsl(crate::compliant_office_frame::PRIMARY_TEXT)
            .map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?,
        id if id == crate::failing_under_reinforced::ID => <En1992Snapshot as store::ArtifactDsl>::parse_dsl(crate::failing_under_reinforced::PRIMARY_TEXT)
            .map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?,
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
