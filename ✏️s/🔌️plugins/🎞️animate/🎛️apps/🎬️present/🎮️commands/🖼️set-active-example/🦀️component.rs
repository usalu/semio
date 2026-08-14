//! 🖼️ 🖼️ Animate present app commands command — `set-active-example`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{interaction_select_effect, PresentDispatchCtx};
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{default_present_snapshot, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so "reset to demo" builds
/// `apps::present::reset_present_document_effect` (a `HostEffect::LoadDocument`, outside undo
/// history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    if payload.example_id == "demo" || payload.example_id.is_empty() {
        Ok(Emit { effects: vec![crate::apps::present::reset_present_document_effect(&default_present_snapshot()), interaction_select_effect(&[], "replace")], ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}
