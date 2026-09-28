//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1997Mutation, En1997Snapshot};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Replaces the live document with a named geotechnical example subject (DSL assets as PRIMARY_TEXT).
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1997Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1997Mutation, NoConfigMutation>, Fault> {
    let Some(snapshot) = crate::app_surface::roster_example_snapshot(<crate::editor::en1997::En1997PlayApp as ArtifactEditor>::examples(), &payload.example_id)? else {
        return Ok(Emit::default());
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
