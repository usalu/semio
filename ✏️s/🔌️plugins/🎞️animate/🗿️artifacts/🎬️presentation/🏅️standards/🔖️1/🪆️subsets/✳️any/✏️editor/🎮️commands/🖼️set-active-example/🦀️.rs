//! 🖼️ 🖼️ Animate presentation app commands command — `set-active-example`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::{default_presentation_snapshot, PresentationSnapshot};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so "reset to demo" builds
/// `editor::animate::reset_presentation_document_effect` (a `Effect::LoadDocument`, outside undo
/// history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    if payload.example_id == "demo" || payload.example_id.is_empty() {
        Ok(Emit { effects: vec![crate::editor::animate::reset_presentation_document_effect(&default_presentation_snapshot()), interaction_select_effect(&[], "replace")], ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}
