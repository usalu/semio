//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Din4108Mutation, Din4108Snapshot};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the live document with the named example snapshot.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Din4108Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din4108Mutation, NoConfigMutation>, Fault> {
    let Some(snapshot) = crate::app_surface::roster_example_snapshot(<crate::editor::din4108::Din4108PlayApp as ArtifactEditor>::examples(), &payload.example_id)? else {
        return Ok(Emit::default());
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
