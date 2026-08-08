//! 🔺️ Procedural3d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::procedural3d::schema::{Procedural3dArtifact, Procedural3dPreviewCamera};
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use flow::playbook::{apply_generation_mutation, GenerationMutation, GenerationPlayState};
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Collections
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiff {
    pub removed: Vec<String>,
    pub set: Vec<(String, WidgetLayout)>,
}

pub(crate) fn apply_widgets_diff(widgets: &mut Vec<Widget>, diff: &WidgetsDiff) {
    for id in &diff.removed {
        widgets.retain(|widget| widget_id(widget) != id);
    }
    for (index, widget) in &diff.set {
        if let Some(pos) = widgets.iter().position(|entry| widget_id(entry) == widget_id(widget)) {
            widgets[pos] = widget.clone();
        } else {
            widgets.insert((*index).min(widgets.len()), widget.clone());
        }
    }
}

pub(crate) fn apply_synapses_diff(synapses: &mut Vec<SynapseSpec>, diff: &SynapsesDiff) {
    for id in &diff.removed {
        synapses.retain(|synapse| synapse.id != *id);
    }
    for (index, synapse) in &diff.set {
        if let Some(pos) = synapses.iter().position(|entry| entry.id == synapse.id) {
            synapses[pos] = synapse.clone();
        } else {
            synapses.insert((*index).min(synapses.len()), synapse.clone());
        }
    }
}

fn apply_layout_diff(layout: &mut std::collections::BTreeMap<String, WidgetLayout>, diff: &LayoutDiff) {
    for id in &diff.removed {
        layout.remove(id);
    }
    for (id, entry) in &diff.set {
        layout.insert(id.clone(), entry.clone());
    }
}

/// 🧩 Applies sparse fixture-collection helpers onto a cloned fixture.
pub fn apply_fixture_helpers(
    fixture: &FlowFixture,
    widgets: &WidgetsDiff,
    synapses: &SynapsesDiff,
    layout: &LayoutDiff,
    camera: Option<&CameraJson>,
    schema: Option<&str>,
) -> FlowFixture {
    let mut next = fixture.clone();
    apply_widgets_diff(&mut next.widgets, widgets);
    apply_synapses_diff(&mut next.synapses, synapses);
    apply_layout_diff(&mut next.layout, layout);
    if let Some(camera) = camera {
        next.camera = camera.clone();
    }
    if let Some(schema) = schema {
        next.schema = schema.to_string();
    }
    next
}

/// 🧩 Applies generation mutations onto a cloned play state.
pub fn apply_generation_helpers(state: &GenerationPlayState, ops: &[GenerationMutation]) -> GenerationPlayState {
    let mut next = state.clone();
    for operation in ops {
        apply_generation_mutation(&mut next, operation);
    }
    next
}
//#endregion 🔖️Collections

//#region 🔖️Apply
impl Procedural3dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Procedural3dArtifact) -> Procedural3dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(fixture) = &self.fixture {
            next.fixture = fixture.clone();
        }
        if let Some(generation) = &self.generation {
            next.generation = generation.clone();
        }
        if let Some(list) = &self.selected_node_ids {
            next.selected_node_ids = list.values.clone();
        }
        if let Some(value) = &self.lod_mode {
            next.lod_mode = value.clone();
        }
        if let Some(value) = &self.show_mode {
            next.show_mode = value.clone();
        }
        if let Some(value) = &self.selection_method {
            next.selection_method = value.clone();
        }
        if let Some(value) = &self.hovered_node_id {
            next.hovered_node_id = value.clone();
        }
        if let Some(value) = &self.graph_camera {
            next.graph_camera = value.clone();
        }
        if let Some(value) = &self.preview_camera {
            next.preview_camera = value.clone();
        }
        if let Some(value) = &self.sun_json {
            next.sun_json = value.clone();
        }
        if let Some(value) = &self.selected_generation_id {
            next.selected_generation_id = value.clone();
        }
        if let Some(value) = &self.generation_preview_text {
            next.generation_preview_text = value.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        next
    }
}

impl MutationDiff<Procedural3dSnapshot> for Procedural3dDiff {
    fn apply(&self, snapshot: &Procedural3dSnapshot) -> Procedural3dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(fixture) = &self.fixture {
            next.fixture = fixture.clone();
        }
        if let Some(generation) = &self.generation {
            next.generation = generation.clone();
        }
        next
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
        take!(fixture);
        take!(generation);
        take!(selected_node_ids);
        take!(lod_mode);
        take!(show_mode);
        take!(selection_method);
        take!(hovered_node_id);
        take!(graph_camera);
        take!(preview_camera);
        take!(sun_json);
        take!(selected_generation_id);
        take!(generation_preview_text);
        take!(active_utility_id);
        take!(locale);
        take!(contributions_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Constructors
/// 🏗️ Whole-fixture field delta after applying sparse collection helpers.
pub fn diff_fixture_from_helpers(
    base: &Procedural3dSnapshot,
    widgets: WidgetsDiff,
    synapses: SynapsesDiff,
    layout: LayoutDiff,
    camera: Option<CameraJson>,
    schema: Option<String>,
) -> Procedural3dDiff {
    let fixture = apply_fixture_helpers(
        &base.fixture,
        &widgets,
        &synapses,
        &layout,
        camera.as_ref(),
        schema.as_deref(),
    );
    Procedural3dDiff { fixture: Some(fixture), ..Procedural3dDiff::default() }
}

/// 🏗️ Generation field delta after applying ordered generation mutations.
pub fn diff_generation_from_ops(base: &Procedural3dSnapshot, ops: Vec<GenerationMutation>) -> Procedural3dDiff {
    let generation = apply_generation_helpers(&base.generation, &ops);
    Procedural3dDiff { generation: Some(generation), ..Procedural3dDiff::default() }
}
//#endregion 🔖️Constructors

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_absorb_prefers_incoming_scalars() {
        let mut first = Procedural3dDiff {
            show_mode: Some("shaded".into()),
            ..Procedural3dDiff::default()
        };
        first.absorb(Procedural3dDiff {
            locale: Some("de-DE".into()),
            preview_camera: Some(Procedural3dPreviewCamera::default()),
            ..Procedural3dDiff::default()
        });
        assert_eq!(first.show_mode.as_deref(), Some("shaded"));
        assert_eq!(first.locale.as_deref(), Some("de-DE"));
        assert!(first.preview_camera.is_some());
    }
}
//#endregion 🧪️Tests
