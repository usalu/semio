//! 🔺️ Generation2d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::generation2d::schema::diff::*;
use crate::artifacts::generation2d::schema::Generation2dArtifact;
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};
use flow::playbook::{apply_generation_mutation, GenerationMutation, GenerationPlayState};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Collections
/// 🧬️ Sparse id-keyed collection helper used when constructing a whole `fixture` replacement.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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

fn apply_layout_diff(layout: &mut flow::OrderedMap<WidgetLayout>, diff: &LayoutDiff) {
    for id in &diff.removed {
        layout.remove(id);
    }
    for (id, entry) in &diff.set {
        layout.insert(id.clone(), entry.clone());
    }
}

/// 🧩 Applies sparse fixture-collection helpers onto a cloned fixture.
pub fn apply_fixture_helpers(fixture: &FlowFixture, widgets: &WidgetsDiff, synapses: &SynapsesDiff, layout: &LayoutDiff, camera: Option<&CameraJson>, schema: Option<&str>) -> FlowFixture {
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
impl Generation2dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Generation2dArtifact) -> protocol::MutationApplyResult<Generation2dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(fixture) = &self.fixture {
                next.fixture = fixture.clone();
            }
            if let Some(generation) = &self.generation {
                std::mem::replace(&mut next.generation, generation.clone()).retire_cold();
            }
            if let Some(list) = &self.selected_ids {
                next.selected_ids = list.values.clone();
            }
            if let Some(value) = &self.graph_camera {
                next.graph_camera = value.clone();
            }
            if let Some(value) = &self.show_mode {
                next.show_mode = value.clone();
            }
            if let Some(value) = &self.selected_generation_id {
                next.selected_generation_id = value.clone();
            }
            if let Some(value) = &self.generation_preview_text {
                next.generation_preview_text = value.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<Generation2dSnapshot> for Generation2dDiff {
    fn apply(&self, snapshot: &Generation2dSnapshot) -> protocol::MutationApplyResult<Generation2dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(fixture) = &self.fixture {
                next.fixture = fixture.clone();
            }
            if let Some(generation) = &self.generation {
                std::mem::replace(&mut next.generation, generation.clone()).retire_cold();
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
        take!(fixture);
        take!(generation);
        take!(selected_ids);
        take!(graph_camera);
        take!(show_mode);
        take!(selected_generation_id);
        take!(generation_preview_text);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Constructors
/// 🏗️ Whole-fixture field delta after applying sparse collection helpers.
pub fn diff_fixture_from_helpers(base: &Generation2dSnapshot, widgets: WidgetsDiff, synapses: SynapsesDiff, layout: LayoutDiff, camera: Option<CameraJson>, schema: Option<String>) -> Generation2dDiff {
    let fixture = apply_fixture_helpers(&base.fixture, &widgets, &synapses, &layout, camera.as_ref(), schema.as_deref());
    Generation2dDiff { fixture: Some(fixture), ..Generation2dDiff::default() }
}

/// 🏗️ Generation field delta after applying ordered generation mutations.
pub fn diff_generation_from_ops(base: &Generation2dSnapshot, ops: Vec<GenerationMutation>) -> Generation2dDiff {
    let generation = apply_generation_helpers(&base.generation, &ops);
    Generation2dDiff { generation: Some(generation.into()), ..Generation2dDiff::default() }
}
//#endregion 🔖️Constructors

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::generation2d::schema::empty_generation2d_snapshot;

    #[test]
    fn diff_absorb_prefers_incoming_fixture_and_scalars() {
        let base = empty_generation2d_snapshot();
        let mut first = diff_fixture_from_helpers(&base, WidgetsDiff { removed: vec!["w1".into()], set: vec![] }, SynapsesDiff::default(), LayoutDiff::default(), Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 }), None);
        let second = Generation2dDiff { show_mode: Some("wire".into()), locale: Some("de-DE".into()), ..Generation2dDiff::default() };
        first.absorb(second);
        assert!(first.fixture.is_some());
        assert_eq!(first.show_mode.as_deref(), Some("wire"));
        assert_eq!(first.locale.as_deref(), Some("de-DE"));
    }

    #[test]
    fn diff_apply_updates_fixture_widgets() {
        let snapshot = empty_generation2d_snapshot();
        let existing_id = widget_id(&snapshot.fixture.widgets[1]).to_string();
        let diff = diff_fixture_from_helpers(
            &snapshot,
            WidgetsDiff { removed: vec![], set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })] },
            SynapsesDiff::default(),
            LayoutDiff::default(),
            None,
            None,
        );
        let next = diff.apply(&snapshot).expect("valid mutation diff");
        assert_eq!(next.fixture.widgets.len(), snapshot.fixture.widgets.len() + 1);
        let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced");
        assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
    }
}
//#endregion 🧪️Tests
