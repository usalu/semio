//! 🖼️ 🖼️ Animate present app commands command — `set-active-example`.

use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{default_present_snapshot, PresentSnapshot};
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so "reset to demo" builds
/// `editor::animate::reset_present_document_effect` (a `Effect::LoadDocument`, outside undo
/// history) instead of an `artifact_mutations` entry.
pub async fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    if payload.example_id == "demo" || payload.example_id.is_empty() {
        Ok(Emit { effects: vec![crate::editor::animate::reset_present_document_effect(&default_present_snapshot()), interaction_select_effect(&[], "replace")], ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}
