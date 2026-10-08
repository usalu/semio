//! 🧬️ Flow diff schema — sparse field delta over the artifact.

use crate::FlowContentChild;
use framework_schema::ArtifactSchema;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the flow artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff). `content` carries a whole-handle replacement (content-
/// addressed, so a changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; flow's `content` slot is never absent, only
/// ever replaced, so a single `Option<FlowContentChild>` — not the double-`Option` an optional slot
/// needs — is the sparse-vs-unchanged signal here, matching writer's `document` field exactly).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub content: Option<FlowContentChild>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct FlowStringList {
    pub values: Vec<String>,
}
//#endregion 🔹DeltaHelpers

use crate::FlowSnapshot;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

impl MutationDiff<FlowSnapshot> for FlowDiff {
    fn apply(&self, snapshot: &FlowSnapshot, _capability: ApplyCapability) -> MutationApplyResult<FlowSnapshot> {
        Ok(FlowSnapshot { schema: self.schema.clone().unwrap_or_else(|| snapshot.schema.clone()), content: self.content.clone().unwrap_or_else(|| snapshot.content.clone()) })
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.content.is_some() {
            self.content = other.content;
        }
    }
}

impl DiffAlgebra<FlowSnapshot> for FlowDiff {
    fn inverse(&self, base: &FlowSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), content: self.content.as_ref().map(|_| base.content.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.content.is_none()
    }
}

/// 🔺️ Mints a new content-addressed `content` handle for the whole-scene replacement `(widgets, synapses, layout)` and seeds the working-scene cache with it
/// (`flow_content_child_handle_and_cache`) — real handcrafted construction, never apply-then-capture.
pub fn diff_replace_content(widgets: Vec<semio_framework_artifact_flow_flow::Widget>, synapses: Vec<semio_framework_artifact_flow_flow::SynapseSpec>, layout: flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>) -> FlowDiff {
    FlowDiff { content: Some(crate::flow_content_child_handle_and_cache(widgets, synapses, layout)), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
