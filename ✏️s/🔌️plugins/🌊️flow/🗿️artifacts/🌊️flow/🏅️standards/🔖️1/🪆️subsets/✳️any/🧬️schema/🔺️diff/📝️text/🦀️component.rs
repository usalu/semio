//! 🔺️ Flow artifact — sparse field-delta diff codec and apply/absorb.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::flow::schema::diff::*;

use crate::artifacts::flow::schema::FlowArtifact;
use crate::artifacts::flow::FlowSnapshot;
use flow::{SynapseSpec, Widget};
use protocol::{Identified, MutationDiff, Patchable};

//#region 🔹Apply
/// Applies an identified-collection delta to a widget list.
pub fn apply_widgets_delta(items: &[Widget], delta: &FlowWidgetsDelta) -> Vec<Widget> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|item| item.id() != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(item) = next.iter_mut().find(|item| item.id() == &entry.id) {
            item.apply_patch(&entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id().clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

/// Applies an identified-collection delta to a synapse list.
pub fn apply_synapses_delta(items: &[SynapseSpec], delta: &FlowSynapsesDelta) -> Vec<SynapseSpec> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|item| &item.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(item) = next.iter_mut().find(|item| item.id == entry.id) {
            item.apply_patch(&entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id.clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn absorb_widgets_delta(target: &mut Option<FlowWidgetsDelta>, incoming: Option<FlowWidgetsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_synapses_delta(target: &mut Option<FlowSynapsesDelta>, incoming: Option<FlowSynapsesDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl FlowDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &FlowArtifact) -> FlowArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.schema {
            next.schema = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(delta) = &self.widgets {
            next.widgets = apply_widgets_delta(&next.widgets, delta);
        }
        if let Some(delta) = &self.synapses {
            next.synapses = apply_synapses_delta(&next.synapses, delta);
        }
        if let Some(delta) = &self.layout {
            for (key, value) in &delta.entries {
                match value {
                    Some(v) => {
                        next.layout.insert(key.clone(), v.clone());
                    }
                    None => {
                        next.layout.remove(key);
                    }
                }
            }
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
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<FlowSnapshot> for FlowDiff {
    fn apply(&self, snapshot: &FlowSnapshot) -> FlowSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.schema {
            next.schema = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(delta) = &self.widgets {
            next.widgets = apply_widgets_delta(&next.widgets, delta);
        }
        if let Some(delta) = &self.synapses {
            next.synapses = apply_synapses_delta(&next.synapses, delta);
        }
        if let Some(delta) = &self.layout {
            for (key, value) in &delta.entries {
                match value {
                    Some(v) => {
                        next.layout.insert(key.clone(), v.clone());
                    }
                    None => {
                        next.layout.remove(key);
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_widgets_delta(&mut self.widgets, other.widgets);
        absorb_synapses_delta(&mut self.synapses, other.synapses);
        match (&mut self.layout, other.layout) {
            (Some(dst), Some(src)) => dst.entries.extend(src.entries),
            (None, Some(src)) => self.layout = Some(src),
            _ => {}
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
        take!(locale);
    }
}
//#endregion 🔹Apply

//#region 🔹Helpers
/// 📄 Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &FlowSnapshot) -> FlowDiff {
    FlowDiff {
        artifact: Some(Box::new(FlowArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔹Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::schema::mutations::FlowMutation;
    use protocol::Mutation;

    #[test]
    fn move_widgets_diff_touches_only_the_layout_slot() {
        let base = FlowSnapshot::default();
        let operation = FlowMutation::MoveWidgets(crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets {
            entries: vec![flow::FlowLayoutEntry { id: "slider".into(), layout: Some(flow::WidgetLayout { x: 3.0, y: 4.0 }) }],
        });
        let diff: FlowDiff = operation.diff(&base);
        assert!(diff.layout.is_some(), "MoveWidgets must produce a layout diff: {diff:?}");
        assert!(
            diff.artifact.is_none() && diff.widgets.is_none() && diff.synapses.is_none(),
            "MoveWidgets must touch only the layout slot: {diff:?}"
        );
        let after = diff.apply(&base);
        assert_eq!(after.layout.get("slider"), Some(&flow::WidgetLayout { x: 3.0, y: 4.0 }));
    }

    #[test]
    fn a_whole_artifact_diff_wins_over_every_collection_diff() {
        let base = FlowSnapshot::default();
        let mut replacement = base.clone();
        replacement.schema = "flow.replaced".into();
        let mut diff = FlowDiff {
            widgets: Some(FlowWidgetsDelta {
                removed: vec!["slider".into()],
                ..Default::default()
            }),
            ..Default::default()
        };
        diff.absorb(diff_set_snapshot(&replacement));
        assert_eq!(diff.apply(&base), replacement);
    }
}
//#endregion 🧪️Tests
