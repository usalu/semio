//! ⚡️ Procedural2d artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use flow::playbook::{invert_generation_operation, GenerationMutation};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
/// 🧮️ Procedural-2d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single `GenerationMutation` generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Procedural2dMutation {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationMutation),
}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Mutation<Procedural2dSnapshot> for Procedural2dMutation {
    type Diff = Procedural2dDiff;

    fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
        use crate::artifacts::procedural2d::diff::{
            diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff,
        };
        match self {
            Procedural2dMutation::SetWidget { index, widget } => diff_fixture_from_helpers(
                base,
                WidgetsDiff { removed: vec![], set: vec![(*index, widget.clone())] },
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural2dMutation::RemoveWidget { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff { removed: vec![id.clone()], set: vec![] },
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural2dMutation::SetSynapse { index, synapse } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff { removed: vec![], set: vec![(*index, synapse.clone())] },
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural2dMutation::RemoveSynapse { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff { removed: vec![id.clone()], set: vec![] },
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural2dMutation::SetLayout { id, layout } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff { removed: vec![], set: vec![(id.clone(), layout.clone())] },
                None,
                None,
            ),
            Procedural2dMutation::RemoveLayout { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff { removed: vec![id.clone()], set: vec![] },
                None,
                None,
            ),
            Procedural2dMutation::SetCamera { camera } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff::default(),
                Some(camera.clone()),
                None,
            ),
            Procedural2dMutation::SetSchema { schema } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                Some(schema.clone()),
            ),
            Procedural2dMutation::Generation(operation) => diff_generation_from_ops(base, vec![operation.clone()]),
        }
    }

    fn inverse(&self, projection: &Procedural2dSnapshot) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural2dMutation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural2dMutation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural2dMutation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural2dMutation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural2dMutation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural2dMutation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural2dMutation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural2dMutation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural2dMutation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural2dMutation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural2dMutation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural2dMutation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural2dMutation::RemoveLayout { id: id.clone() }],
            },
            Procedural2dMutation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural2dMutation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural2dMutation::SetCamera { .. } => vec![Procedural2dMutation::SetCamera { camera: fixture.camera.clone() }],
            Procedural2dMutation::SetSchema { .. } => vec![Procedural2dMutation::SetSchema { schema: fixture.schema.clone() }],
            Procedural2dMutation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural2dMutation::Generation).collect(),
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched
/// widgets and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is
/// ephemeral view state (app config), never a document operation.
pub fn procedural2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural2dMutation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural2dMutation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural2dMutation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural2dMutation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural2dMutation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural2dMutation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural2dMutation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖️Operation

pub type Procedural2dEnvelope = DocumentEnvelope<Procedural2dSnapshot, Procedural2dMutation>;
pub type Procedural2dStore = DocumentStore<Procedural2dSnapshot, Procedural2dMutation>;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::diff::{SynapsesDiff, WidgetsDiff};
    use crate::artifacts::procedural2d::engine::empty_procedural2d_snapshot;
    use vcs::apply_mutation;

    fn round_trip(projection: &Procedural2dSnapshot, operation: &Procedural2dMutation) -> Procedural2dSnapshot {
        let forward = apply_mutation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(projection) {
            restored = apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural2dMutation::SetCamera { .. })));
    }

    #[test]
    fn remove_and_readd_widget_round_trips() {
        let base = empty_procedural2d_snapshot();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &Procedural2dMutation::RemoveWidget { id: removed_id.clone() });
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_add() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dMutation::SetWidget { widget, .. } if widget_id(widget) == "note-1")));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_snapshot();
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural2dMutation::Generation(GenerationMutation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    //#region 🔖️OperationBackwardsTests
    #[test]
    fn set_widget_backwards_restores_replaced_widget() {
        let base = empty_procedural2d_snapshot();
        let id = widget_id(&base.fixture.widgets[1]).to_string();
        round_trip(&base, &Procedural2dMutation::SetWidget { index: 1, widget: Widget::InputNote { id, text: "replaced".into() } });
    }

    #[test]
    fn set_widget_backwards_removes_newly_inserted_widget() {
        let base = empty_procedural2d_snapshot();
        let after = round_trip(&base, &Procedural2dMutation::SetWidget { index: 0, widget: Widget::InputNote { id: "brand-new".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "brand-new"));
    }

    #[test]
    fn remove_widget_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_snapshot();
        let op = Procedural2dMutation::RemoveWidget { id: "does-not-exist".into() };
        assert!(op.inverse(&base).is_empty());
        let after = round_trip(&base, &op);
        assert_eq!(after, base);
    }

    #[test]
    fn set_synapse_backwards_restores_replaced_synapse() {
        let base = empty_procedural2d_snapshot();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dMutation::SetSynapse { index: 0, synapse: SynapseSpec { id, from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: "changed".into() } });
    }

    #[test]
    fn set_synapse_backwards_removes_newly_inserted_synapse() {
        let base = empty_procedural2d_snapshot();
        let synapse = SynapseSpec { id: "brand-new-synapse".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() };
        let after = round_trip(&base, &Procedural2dMutation::SetSynapse { index: 0, synapse });
        assert!(after.fixture.synapses.iter().any(|s| s.id == "brand-new-synapse"));
    }

    #[test]
    fn remove_synapse_backwards_restores_removed_synapse() {
        let base = empty_procedural2d_snapshot();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dMutation::RemoveSynapse { id });
    }

    #[test]
    fn remove_synapse_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_snapshot();
        let op = Procedural2dMutation::RemoveSynapse { id: "missing".into() };
        assert!(op.inverse(&base).is_empty());
    }

    #[test]
    fn set_layout_backwards_restores_prior_layout_entry() {
        let mut base = empty_procedural2d_snapshot();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 1.0, y: 1.0 });
        round_trip(&base, &Procedural2dMutation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 9.0, y: 9.0 } });
    }

    #[test]
    fn set_layout_backwards_removes_newly_created_layout_entry() {
        let base = empty_procedural2d_snapshot();
        assert!(base.fixture.layout.is_empty());
        let after = round_trip(&base, &Procedural2dMutation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } });
        assert!(after.fixture.layout.contains_key("slider"));
    }

    #[test]
    fn remove_layout_backwards_restores_removed_layout_entry() {
        let mut base = empty_procedural2d_snapshot();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 4.0, y: 5.0 });
        round_trip(&base, &Procedural2dMutation::RemoveLayout { id: "slider".into() });
    }

    #[test]
    fn remove_layout_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_snapshot();
        let op = Procedural2dMutation::RemoveLayout { id: "missing".into() };
        assert!(op.inverse(&base).is_empty());
    }

    #[test]
    fn set_camera_backwards_restores_prior_camera() {
        let base = empty_procedural2d_snapshot();
        round_trip(&base, &Procedural2dMutation::SetCamera { camera: CameraJson { x: 42.0, y: -3.0, zoom: 5.0 } });
    }

    #[test]
    fn set_schema_backwards_restores_prior_schema() {
        let base = empty_procedural2d_snapshot();
        round_trip(&base, &Procedural2dMutation::SetSchema { schema: "changed.schema".into() });
    }
    //#endregion 🔖️OperationBackwardsTests

    //#region 🔖️FixtureOpsTests
    #[test]
    fn fixture_ops_widget_id_matches_every_widget_kind() {
        let widgets = vec![
            Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "w-slider".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
            Widget::InputNote { id: "w-note".into(), text: String::new() },
            Widget::InputImage { id: "w-image".into(), src: String::new() },
            Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "w-action".into(), action: String::new() },
            Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
            Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
        ];
        let mut before = FlowFixture::default();
        before.widgets.clear();
        let mut after = before.clone();
        after.widgets = widgets.clone();
        let operations = procedural2d_fixture_operations(&before, &after);
        for widget in &widgets {
            let id = widget_id(widget);
            assert!(operations.iter().any(|op| matches!(op, Procedural2dMutation::SetWidget { widget, .. } if widget_id(widget) == id)));
        }
    }

    #[test]
    fn widgets_diff_apply_replaces_by_id_and_removes_by_id() {
        let mut widgets = vec![Widget::InputNote { id: "a".into(), text: "1".into() }, Widget::InputNote { id: "b".into(), text: "2".into() }];
        let diff = WidgetsDiff { removed: vec!["b".into()], set: vec![(0, Widget::InputNote { id: "a".into(), text: "replaced".into() })] };
        crate::artifacts::procedural2d::diff::apply_widgets_diff(&mut widgets, &diff);
        assert_eq!(widgets, vec![Widget::InputNote { id: "a".into(), text: "replaced".into() }]);
    }

    #[test]
    fn synapses_diff_apply_replaces_by_id_and_removes_by_id() {
        let mut synapses = vec![SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }];
        let diff = SynapsesDiff { removed: vec![], set: vec![(0, SynapseSpec { id: "s1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() })] };
        crate::artifacts::procedural2d::diff::apply_synapses_diff(&mut synapses, &diff);
        assert_eq!(synapses[0].to, "c");
    }
    //#endregion 🔖️FixtureOpsTests
}
//#endregion 🧪️Tests


pub fn apply_procedural2d_mutation(projection: &mut Procedural2dSnapshot, mutation: &Procedural2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_procedural2d_mutation(projection: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Vec<Procedural2dMutation> {
    mutation.inverse(projection)
}
