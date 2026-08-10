//! ⚡️ Procedural3d artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use flow::playbook::{invert_generation_operation, GenerationMutation};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Operation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Procedural3dMutation {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationMutation)}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Mutation<Procedural3dSnapshot> for Procedural3dMutation {
    type Diff = Procedural3dDiff;

    fn diff(&self, base: &Procedural3dSnapshot) -> Procedural3dDiff {
        use crate::artifacts::procedural3d::diff::{
            diff_fixture_from_helpers, diff_generation_from_ops, LayoutDiff, SynapsesDiff, WidgetsDiff};
        match self {
            Procedural3dMutation::SetWidget { index, widget } => diff_fixture_from_helpers(
                base,
                WidgetsDiff { removed: vec![], set: vec![(*index, widget.clone())] },
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural3dMutation::RemoveWidget { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff { removed: vec![id.clone()], set: vec![] },
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural3dMutation::SetSynapse { index, synapse } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff { removed: vec![], set: vec![(*index, synapse.clone())] },
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural3dMutation::RemoveSynapse { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff { removed: vec![id.clone()], set: vec![] },
                LayoutDiff::default(),
                None,
                None,
            ),
            Procedural3dMutation::SetLayout { id, layout } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff { removed: vec![], set: vec![(id.clone(), layout.clone())] },
                None,
                None,
            ),
            Procedural3dMutation::RemoveLayout { id } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff { removed: vec![id.clone()], set: vec![] },
                None,
                None,
            ),
            Procedural3dMutation::SetCamera { camera } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff::default(),
                Some(camera.clone()),
                None,
            ),
            Procedural3dMutation::SetSchema { schema } => diff_fixture_from_helpers(
                base,
                WidgetsDiff::default(),
                SynapsesDiff::default(),
                LayoutDiff::default(),
                None,
                Some(schema.clone()),
            ),
            Procedural3dMutation::Generation(operation) => diff_generation_from_ops(base, vec![operation.clone()])}
    }

    fn inverse(&self, projection: &Procedural3dSnapshot) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural3dMutation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural3dMutation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural3dMutation::RemoveWidget { id: widget_id(widget).to_string() }]},
            Procedural3dMutation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural3dMutation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural3dMutation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural3dMutation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural3dMutation::RemoveSynapse { id: synapse.id.clone() }]},
            Procedural3dMutation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural3dMutation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural3dMutation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural3dMutation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural3dMutation::RemoveLayout { id: id.clone() }]},
            Procedural3dMutation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural3dMutation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural3dMutation::SetCamera { .. } => vec![Procedural3dMutation::SetCamera { camera: fixture.camera.clone() }],
            Procedural3dMutation::SetSchema { .. } => vec![Procedural3dMutation::SetSchema { schema: fixture.schema.clone() }],
            Procedural3dMutation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural3dMutation::Generation).collect()}
    }
}

/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable operation set.
pub fn procedural3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural3dMutation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural3dMutation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural3dMutation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural3dMutation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural3dMutation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural3dMutation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural3dMutation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural3dMutation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖️Operation

pub type Procedural3dEnvelope = ArtifactEnvelope<Procedural3dSnapshot, Procedural3dMutation>;
pub type Procedural3dStore = ArtifactStore<Procedural3dSnapshot, Procedural3dMutation>;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::engine::empty_procedural3d_snapshot;

    fn round_trip(projection: &Procedural3dSnapshot, operation: &Procedural3dMutation) -> Procedural3dSnapshot {
        let forward = vcs::apply_mutation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(projection) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn store_applies_widget_add() {
        let mut store = ArtifactStore::<Procedural3dSnapshot, Procedural3dMutation>::new(store::create_document_envelope(crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_snapshot(), None));
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Procedural3dMutation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        assert!(store.snapshot().expect("snapshot").fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn set_widget_round_trips() {
        let before = empty_procedural3d_snapshot();
        let after = round_trip(&before, &Procedural3dMutation::SetWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural3d_snapshot();
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural3dMutation::Generation(GenerationMutation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural3d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural3dMutation::SetCamera { .. })));
    }

    #[test]
    fn procedural3d_fixture_operations_detects_widget_synapse_layout_schema_changes() {
        let mut before = FlowFixture { schema: "old-schema".into(), ..Default::default() };
        before.widgets = vec![Widget::InputNote { id: "w-gone".into(), text: String::new() }, Widget::InputNote { id: "w-keep".into(), text: "old".into() }];
        before.synapses = vec![
            SynapseSpec { id: "s-gone".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "old".into() },
        ];
        before.layout.insert("l-gone".into(), WidgetLayout { x: 0.0, y: 0.0 });
        before.layout.insert("l-keep".into(), WidgetLayout { x: 1.0, y: 1.0 });

        let mut after = FlowFixture { schema: "new-schema".into(), ..Default::default() };
        after.widgets = vec![Widget::InputNote { id: "w-keep".into(), text: "new".into() }, Widget::InputNote { id: "w-new".into(), text: String::new() }];
        after.synapses = vec![
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() },
            SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
        ];
        after.layout.insert("l-keep".into(), WidgetLayout { x: 2.0, y: 2.0 });
        after.layout.insert("l-new".into(), WidgetLayout { x: 3.0, y: 3.0 });

        let operations = procedural3d_fixture_operations(&before, &after);
        assert!(operations.contains(&Procedural3dMutation::RemoveWidget { id: "w-gone".into() }));
        assert!(operations.contains(&Procedural3dMutation::SetWidget { index: 0, widget: Widget::InputNote { id: "w-keep".into(), text: "new".into() } }));
        assert!(operations.contains(&Procedural3dMutation::SetWidget { index: 1, widget: Widget::InputNote { id: "w-new".into(), text: String::new() } }));
        assert!(operations.contains(&Procedural3dMutation::RemoveSynapse { id: "s-gone".into() }));
        assert!(operations.contains(&Procedural3dMutation::SetSynapse { index: 0, synapse: SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() } }));
        assert!(operations.contains(&Procedural3dMutation::SetSynapse { index: 1, synapse: SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } }));
        assert!(operations.contains(&Procedural3dMutation::RemoveLayout { id: "l-gone".into() }));
        assert!(operations.contains(&Procedural3dMutation::SetLayout { id: "l-keep".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } }));
        assert!(operations.contains(&Procedural3dMutation::SetLayout { id: "l-new".into(), layout: WidgetLayout { x: 3.0, y: 3.0 } }));
        assert!(operations.contains(&Procedural3dMutation::SetSchema { schema: "new-schema".into() }));
    }

    #[test]
    fn set_widget_round_trip_replaces_existing_widget_by_id() {
        let mut before = empty_procedural3d_snapshot();
        before.fixture.widgets.clear();
        before.fixture.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
        let after = round_trip(&before, &Procedural3dMutation::SetWidget { index: 0, widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } });
        assert_eq!(after.fixture.widgets.len(), 1);
        assert_eq!(after.fixture.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
    }

    #[test]
    fn inverse_remove_widget_when_missing_returns_empty() {
        let projection = empty_procedural3d_snapshot();
        assert!(Procedural3dMutation::RemoveWidget { id: "ghost".into() }.inverse(&projection).is_empty());
    }

    #[test]
    fn set_synapse_round_trip_replaces_existing_synapse_by_id() {
        let mut before = empty_procedural3d_snapshot();
        before.fixture.synapses.clear();
        before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
        let after = round_trip(&before, &Procedural3dMutation::SetSynapse { index: 0, synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } });
        assert_eq!(after.fixture.synapses.len(), 1);
        assert_eq!(after.fixture.synapses[0].to, "c");
    }

    #[test]
    fn inverse_remove_synapse_when_missing_returns_empty() {
        let projection = empty_procedural3d_snapshot();
        assert!(Procedural3dMutation::RemoveSynapse { id: "ghost".into() }.inverse(&projection).is_empty());
    }

    #[test]
    fn set_layout_round_trip_inserts_when_absent() {
        let before = empty_procedural3d_snapshot();
        let after = round_trip(&before, &Procedural3dMutation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn set_layout_round_trip_replaces_when_present() {
        let mut before = empty_procedural3d_snapshot();
        before.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let after = round_trip(&before, &Procedural3dMutation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    }

    #[test]
    fn remove_layout_backwards_present_restores_set_layout_missing_returns_empty() {
        let mut projection = empty_procedural3d_snapshot();
        projection.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        assert_eq!(Procedural3dMutation::RemoveLayout { id: "extrude".into() }.inverse(&projection), vec![Procedural3dMutation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }]);
        assert!(Procedural3dMutation::RemoveLayout { id: "ghost".into() }.inverse(&projection).is_empty());
    }

    #[test]
    fn set_camera_round_trip_updates_camera() {
        let before = empty_procedural3d_snapshot();
        let after = round_trip(&before, &Procedural3dMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(after.fixture.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    }

    #[test]
    fn set_schema_round_trip_updates_schema() {
        let before = empty_procedural3d_snapshot();
        let after = round_trip(&before, &Procedural3dMutation::SetSchema { schema: "flow.fixture.v2".into() });
        assert_eq!(after.fixture.schema, "flow.fixture.v2");
    }
}
//#endregion 🧪️Tests


pub fn apply_procedural3d_mutation(projection: &mut Procedural3dSnapshot, mutation: &Procedural3dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_procedural3d_mutation(projection: &Procedural3dSnapshot, mutation: &Procedural3dMutation) -> Vec<Procedural3dMutation> {
    mutation.inverse(projection)
}
