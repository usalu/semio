//! 🔺️ Flow artifact — sparse field-delta diff codec and apply/absorb.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

use crate::schema::FlowArtifact;
use crate::FlowSnapshot;
use protocol::MutationDiff;

//#region 🔹Apply
impl FlowDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
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
            if let Some(list) = &self.selected_node_ids {
                next.selected_node_ids = list.values.clone();
            }
            if let Some(list) = &self.selected_edge_ids {
                next.selected_edge_ids = list.values.clone();
            }
            if let Some(list) = &self.selected_handle_ids {
                next.selected_handle_ids = list.values.clone();
            }
            if let Some(list) = &self.preview_off_node_ids {
                next.preview_off_node_ids = list.values.clone();
            }
            if let Some(value) = &self.lod_mode {
                next.lod_mode = value.clone();
            }
            if let Some(value) = self.proximity_distance {
                next.proximity_distance = value;
            }
            if let Some(value) = self.grid_visible {
                next.grid_visible = value;
            }
            if let Some(value) = self.grid_snap_enabled {
                next.grid_snap_enabled = value;
            }
            if let Some(value) = self.grid_factor {
                next.grid_factor = value;
            }
            if let Some(value) = &self.catalogue_sections_json {
                next.catalogue_sections_json = value.clone();
            }
            if let Some(value) = &self.automation_enabled_json {
                next.automation_enabled_json = value.clone();
            }
            if let Some(value) = &self.contributions_json {
                next.contributions_json = value.clone();
            }
            if let Some(value) = &self.generation_json {
                next.generation_json = value.clone();
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
        take!(selected_node_ids);
        take!(selected_edge_ids);
        take!(selected_handle_ids);
        take!(preview_off_node_ids);
        take!(lod_mode);
        take!(proximity_distance);
        take!(grid_visible);
        take!(grid_snap_enabled);
        take!(grid_factor);
        take!(catalogue_sections_json);
        take!(automation_enabled_json);
        take!(contributions_json);
        take!(generation_json);
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
