//! ⚡️ Procedural3d artifact — operation enum + laws (constitutional: op).

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::{widget_id, Procedural3dDocument};
use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use playbook::{invert_generation_operation, GenerationOperation};
use protocol::Operation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Procedural3dOperation {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationOperation),
}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Operation<Procedural3dDocument> for Procedural3dOperation {
    type Diff = Procedural3dDiff;

    fn diff(&self, _projection: &Procedural3dDocument) -> Procedural3dDiff {
        let mut diff = Procedural3dDiff::default();
        match self {
            Procedural3dOperation::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural3dOperation::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural3dOperation::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural3dOperation::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural3dOperation::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural3dOperation::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural3dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural3dOperation::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural3dOperation::Generation(operation) => diff.generation.push(operation.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural3dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural3dOperation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural3dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural3dOperation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural3dOperation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural3dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural3dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural3dOperation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural3dOperation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural3dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural3dOperation::RemoveLayout { id: id.clone() }],
            },
            Procedural3dOperation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetCamera { .. } => vec![Procedural3dOperation::SetCamera { camera: fixture.camera.clone() }],
            Procedural3dOperation::SetSchema { .. } => vec![Procedural3dOperation::SetSchema { schema: fixture.schema.clone() }],
            Procedural3dOperation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural3dOperation::Generation).collect(),
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable operation set.
pub fn procedural3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural3dOperation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural3dOperation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural3dOperation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural3dOperation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural3dOperation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural3dOperation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural3dOperation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖️Operation

pub type Procedural3dEnvelope = DocumentEnvelope<Procedural3dDocument, Procedural3dOperation>;
pub type Procedural3dStore = DocumentStore<Procedural3dDocument, Procedural3dOperation>;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::engine::empty_procedural3d_projection;

    fn round_trip(projection: &Procedural3dDocument, operation: &Procedural3dOperation) -> Procedural3dDocument {
        let forward = vcs::apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn store_applies_widget_add() {
        let mut store = DocumentStore::<Procedural3dDocument, Procedural3dOperation>::new(store::create_document_envelope(crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        assert!(store.projection().expect("projection").fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn set_widget_round_trips() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural3d_projection();
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural3dOperation::Generation(GenerationOperation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural3d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural3dOperation::SetCamera { .. })));
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
        assert!(operations.contains(&Procedural3dOperation::RemoveWidget { id: "w-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "w-keep".into(), text: "new".into() } }));
        assert!(operations.contains(&Procedural3dOperation::SetWidget { index: 1, widget: Widget::InputNote { id: "w-new".into(), text: String::new() } }));
        assert!(operations.contains(&Procedural3dOperation::RemoveSynapse { id: "s-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() } }));
        assert!(operations.contains(&Procedural3dOperation::SetSynapse { index: 1, synapse: SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } }));
        assert!(operations.contains(&Procedural3dOperation::RemoveLayout { id: "l-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-keep".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-new".into(), layout: WidgetLayout { x: 3.0, y: 3.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetSchema { schema: "new-schema".into() }));
    }

    #[test]
    fn set_widget_round_trip_replaces_existing_widget_by_id() {
        let mut before = empty_procedural3d_projection();
        before.fixture.widgets.clear();
        before.fixture.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
        let after = round_trip(&before, &Procedural3dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } });
        assert_eq!(after.fixture.widgets.len(), 1);
        assert_eq!(after.fixture.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
    }

    #[test]
    fn backwards_remove_widget_when_missing_returns_empty() {
        let projection = empty_procedural3d_projection();
        assert!(Procedural3dOperation::RemoveWidget { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_synapse_round_trip_replaces_existing_synapse_by_id() {
        let mut before = empty_procedural3d_projection();
        before.fixture.synapses.clear();
        before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
        let after = round_trip(&before, &Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } });
        assert_eq!(after.fixture.synapses.len(), 1);
        assert_eq!(after.fixture.synapses[0].to, "c");
    }

    #[test]
    fn backwards_remove_synapse_when_missing_returns_empty() {
        let projection = empty_procedural3d_projection();
        assert!(Procedural3dOperation::RemoveSynapse { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_layout_round_trip_inserts_when_absent() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn set_layout_round_trip_replaces_when_present() {
        let mut before = empty_procedural3d_projection();
        before.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let after = round_trip(&before, &Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    }

    #[test]
    fn remove_layout_backwards_present_restores_set_layout_missing_returns_empty() {
        let mut projection = empty_procedural3d_projection();
        projection.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        assert_eq!(Procedural3dOperation::RemoveLayout { id: "extrude".into() }.backwards(&projection), vec![Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }]);
        assert!(Procedural3dOperation::RemoveLayout { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_camera_round_trip_updates_camera() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(after.fixture.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    }

    #[test]
    fn set_schema_round_trip_updates_schema() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetSchema { schema: "flow.fixture.v2".into() });
        assert_eq!(after.fixture.schema, "flow.fixture.v2");
    }
}
//#endregion 🧪️Tests
