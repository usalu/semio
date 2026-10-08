//! 🧬️ Flow diff schema — sparse field delta over the artifact.

use crate::FlowContentChild;
use framework_schema::ArtifactSchema;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the flow artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `content` carries a whole-handle replacement (content-
/// addressed, so a changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; flow's `content` slot is never absent, only
/// ever replaced, so a single `Option<FlowContentChild>` — not the double-`Option` an optional slot
/// needs — is the sparse-vs-unchanged signal here, matching writer's `document` field exactly).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowDiff {
    #[state(artifact)]
    pub artifact: Option<Box<FlowArtifact>>,
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::FlowArtifact;
//#endregion 🔁️Re-exports

use crate::FlowSnapshot;
use protocol::MutationDiff;

impl FlowDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &FlowArtifact) -> protocol::MutationApplyResult<FlowArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.schema {
                next.schema = value.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
}

impl MutationDiff<FlowSnapshot> for FlowDiff {
    fn apply(&self, snapshot: &FlowSnapshot) -> protocol::MutationApplyResult<FlowSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.schema {
                next.schema = value.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(content);
    }
}

/// 📄 Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &FlowSnapshot) -> FlowDiff {
    FlowDiff { artifact: Some(Box::new(FlowArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🔺️ Mints a new content-addressed `content` handle for the whole-scene replacement
/// `(widgets, synapses, layout)` and seeds the working-scene cache with it
/// (`flow_content_child_handle_and_cache`) — real handcrafted construction, never apply-then-
/// capture. Every one of the nine widget/synapse mutation triads' `🔺️diff` leaf reads the CURRENT
/// scene off `base` (via `flow_working_scene`), applies its own specific semantics to that scene,
/// then calls this shared builder — mirrors writer's `diff_set_text`.
pub fn diff_replace_content(widgets: Vec<semio_framework_artifact_flow_flow::Widget>, synapses: Vec<semio_framework_artifact_flow_flow::SynapseSpec>, layout: flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>) -> FlowDiff {
    FlowDiff { content: Some(crate::flow_content_child_handle_and_cache(widgets, synapses, layout)), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
