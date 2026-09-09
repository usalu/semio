//! 🔺️ Flow artifact — sparse field-delta diff codec and apply/absorb.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

use crate::FlowSnapshot;
use protocol::MutationDiff;

//#region 🔹Apply
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
            if let Some(value) = &self.camera {
                next.camera = value.clone();
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
            if let Some(value) = &self.camera {
                next.camera = value.clone();
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
        take!(camera);
        take!(content);
    }
}
//#endregion 🔹Apply

//#region 🔹Helpers
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
//#endregion 🔹Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type FlowDiffText = String;
//#endregion 🚚️Carrier
