//! ⚡️ Procedural 3D app — operation enum + laws (constitutional: op).

use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use playbook::{apply_generation_operation, invert_generation_operation, GenerationOperation};
use procedural_3d::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_id, widget_to_dsl,
    CameraJsonDsl, FormGenerationDsl, Procedural3dDocument, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Collections
/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Disjoint `set`s on different ids merge cleanly, which
/// is what lets two backbone peers converge on concurrent edits to different widgets/synapses.
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

fn apply_widgets_diff(widgets: &mut Vec<Widget>, diff: &WidgetsDiff) {
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

fn apply_synapses_diff(synapses: &mut Vec<SynapseSpec>, diff: &SynapsesDiff) {
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
//#endregion 🔖️Collections

//#region 🔖️Operations
/// 🩹️ Sparse procedural-3d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
        let mut next = projection.clone();
        apply_widgets_diff(&mut next.fixture.widgets, &self.widgets);
        apply_synapses_diff(&mut next.fixture.synapses, &self.synapses);
        for id in &self.layout.removed {
            next.fixture.layout.remove(id);
        }
        for (id, layout) in &self.layout.set {
            next.fixture.layout.insert(id.clone(), layout.clone());
        }
        if let Some(camera) = &self.camera {
            next.fixture.camera = camera.clone();
        }
        if let Some(schema) = &self.schema {
            next.fixture.schema = schema.clone();
        }
        for operation in &self.generation {
            apply_generation_operation(&mut next.generation, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.widgets.removed.extend(other.widgets.removed);
        self.widgets.set.extend(other.widgets.set);
        self.synapses.removed.extend(other.synapses.removed);
        self.synapses.set.extend(other.synapses.set);
        self.layout.removed.extend(other.layout.removed);
        self.layout.set.extend(other.layout.set);
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.generation.extend(other.generation);
    }
}

/// 🧮️ Procedural-3d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single {@link GenerationOperation} generation edit with its true inverse.
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

/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is ephemeral
/// view state (plugin runtime), never a document operation. Lets action handlers keep computing the target
/// fixture via `FlowHost` while emitting granular operations.
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
//#endregion 🔖️Operations

//#region 🔖️OpText
/// ⚡️ Local twin of `Procedural3dOperation` — flattens the `Generation(GenerationOperation)` newtype
/// variant into its own four top-level keyword variants (mirroring the OLD hand-rolled op-line
/// keywords `generation-add`/`generation-remove`/`generation-rename`/`generation-update-values`)
/// since a `#[derive(dsl::DslOps)]` enum's variants are each their own tagged record, not a nested
/// enum-in-enum.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural3dOperationDsl {
    SetWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    RemoveWidget { id: String },
    SetSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    RemoveSynapse { id: String },
    SetLayout {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    RemoveLayout { id: String },
    SetCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    SetSchema { schema: String },
    GenerationAdd {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    GenerationRemove { id: String },
    GenerationRename { id: String, name: String },
    GenerationUpdateValues {
        id: String,
        question_id: String,
        value: serde_json::Value,
    }
}

fn procedural3d_operation_to_dsl(operation: &Procedural3dOperation) -> Procedural3dOperationDsl {
    match operation {
        Procedural3dOperation::SetWidget { index, widget } => Procedural3dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dOperation::RemoveWidget { id } => Procedural3dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural3dOperation::SetSynapse { index, synapse } => Procedural3dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural3dOperation::RemoveSynapse { id } => Procedural3dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural3dOperation::SetLayout { id, layout } => Procedural3dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural3dOperation::RemoveLayout { id } => Procedural3dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural3dOperation::SetCamera { camera } => Procedural3dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural3dOperation::SetSchema { schema } => Procedural3dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Add { generation }) => Procedural3dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural3dOperation::Generation(GenerationOperation::Remove { id }) => Procedural3dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural3dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural3dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: value.clone() }
        }
    }
}

fn procedural3d_operation_from_dsl(operation: Procedural3dOperationDsl) -> Result<Procedural3dOperation, store::TextError> {
    Ok(match operation {
        Procedural3dOperationDsl::SetWidget { index, widget } => Procedural3dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural3dOperationDsl::RemoveWidget { id } => Procedural3dOperation::RemoveWidget { id },
        Procedural3dOperationDsl::SetSynapse { index, synapse } => Procedural3dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural3dOperationDsl::RemoveSynapse { id } => Procedural3dOperation::RemoveSynapse { id },
        Procedural3dOperationDsl::SetLayout { id, layout } => Procedural3dOperation::SetLayout { id, layout: layout_from_dsl(layout) },
        Procedural3dOperationDsl::RemoveLayout { id } => Procedural3dOperation::RemoveLayout { id },
        Procedural3dOperationDsl::SetCamera { camera } => Procedural3dOperation::SetCamera { camera: camera_from_dsl(camera) },
        Procedural3dOperationDsl::SetSchema { schema } => Procedural3dOperation::SetSchema { schema },
        Procedural3dOperationDsl::GenerationAdd { generation } => Procedural3dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural3dOperationDsl::GenerationRemove { id } => Procedural3dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural3dOperationDsl::GenerationRename { id, name } => Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural3dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }),
    })
}

/// ⚡️ `Procedural3dOperation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural3dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural3dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural3d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural3dOperationDsl as protocol::OpText>::print_op(&procedural3d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Procedural3dOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Procedural3dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural3dOperationDsl::decode_op(bytes)?;
        procedural3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural3d operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpText

pub type Procedural3dEnvelope = DocumentEnvelope<Procedural3dDocument, Procedural3dOperation>;
pub type Procedural3dStore = DocumentStore<Procedural3dDocument, Procedural3dOperation>;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use procedural_3d::PROCEDURAL_3D_SCHEMA;
    use procedural_3d_engine::empty_procedural3d_projection;
    use store::{create_document_envelope, test_support, DocumentCommand};
    use protocol::OpText;
    use vcs::apply_operation;

    fn round_trip(projection: &Procedural3dDocument, operation: &Procedural3dOperation) -> Procedural3dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn store_applies_widget_add() {
        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
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
        let mut before = FlowFixture::default();
        before.schema = "old-schema".into();
        before.widgets = vec![
            Widget::InputNote { id: "w-gone".into(), text: String::new() },
            Widget::InputNote { id: "w-keep".into(), text: "old".into() },
        ];
        before.synapses = vec![
            SynapseSpec { id: "s-gone".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "old".into() },
        ];
        before.layout.insert("l-gone".into(), WidgetLayout { x: 0.0, y: 0.0 });
        before.layout.insert("l-keep".into(), WidgetLayout { x: 1.0, y: 1.0 });

        let mut after = FlowFixture::default();
        after.schema = "new-schema".into();
        after.widgets = vec![
            Widget::InputNote { id: "w-keep".into(), text: "new".into() },
            Widget::InputNote { id: "w-new".into(), text: String::new() },
        ];
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
        assert!(operations
            .contains(&Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() } }));
        assert!(operations
            .contains(&Procedural3dOperation::SetSynapse { index: 1, synapse: SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } }));
        assert!(operations.contains(&Procedural3dOperation::RemoveLayout { id: "l-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-keep".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-new".into(), layout: WidgetLayout { x: 3.0, y: 3.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetSchema { schema: "new-schema".into() }));
    }
    //#region 🔖️WidgetIdTests
    #[test]
    fn widget_id_covers_all_widget_kinds() {
        let widgets: Vec<Widget> = vec![
            Widget::Neuron { id: "neuron-1".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "slider-1".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
            Widget::InputNote { id: "note-1".into(), text: String::new() },
            Widget::InputImage { id: "image-1".into(), src: String::new() },
            Widget::Variable { id: "variable-1".into(), name: "x".into(), schema: "number".into() },
            Widget::OutputPreview { id: "preview-1".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "action-1".into(), action: "run".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "c".into(), tree: Default::default(), flow: Default::default() },
        ];
        for widget in &widgets {
            assert_eq!(widget_id(widget), &widget_id(widget).to_string());
        }
        let ids: Vec<&str> = widgets.iter().map(widget_id).collect();
        assert_eq!(
            ids,
            vec!["neuron-1", "slider-1", "note-1", "image-1", "variable-1", "preview-1", "action-1", "export-1", "cluster-1"]
        );
    }
    //#endregion 🔖️WidgetIdTests

    //#region 🔖️CollectionDiffTests
    #[test]
    fn set_widget_round_trip_replaces_existing_widget_by_id() {
        let mut before = empty_procedural3d_projection();
        // 🩹️ Pre-existing bug fix (unrelated to the dsl:: engine conversion): `empty_procedural3d_projection`
        // returns `FlowFixture::default()`'s own demo widgets/synapses, not an empty fixture — this test
        // needs a clean slate to assert an exact post-replace length, matching the `.clear()` pattern
        // `fixture_ops_widget_id_matches_every_widget_kind` already uses for the same reason.
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
        // 🩹️ Pre-existing bug fix (unrelated to the dsl:: engine conversion): see the sibling widget
        // test above for why a clean slate is needed before asserting an exact post-replace length.
        before.fixture.synapses.clear();
        before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
        let after = round_trip(
            &before,
            &Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } },
        );
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
        assert_eq!(
            Procedural3dOperation::RemoveLayout { id: "extrude".into() }.backwards(&projection),
            vec![Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }]
        );
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

    #[test]
    fn diff_absorb_merges_collections_and_prefers_incoming_scalars() {
        let mut first = Procedural3dDiff::default();
        first.widgets.removed.push("w-a".into());
        first.widgets.set.push((0, Widget::InputNote { id: "w-b".into(), text: String::new() }));
        first.synapses.removed.push("s-a".into());
        first.layout.removed.push("l-a".into());
        first.camera = Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 });
        first.schema = Some("schema-1".into());
        first.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "First".into() });

        let mut second = Procedural3dDiff::default();
        second.widgets.removed.push("w-c".into());
        second.synapses.set.push((0, SynapseSpec { id: "s-b".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }));
        second.layout.set.push(("l-b".into(), WidgetLayout { x: 2.0, y: 2.0 }));
        second.camera = Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 });
        second.schema = None;
        second.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "Second".into() });

        first.absorb(second);

        assert_eq!(first.widgets.removed, vec!["w-a".to_string(), "w-c".to_string()]);
        assert_eq!(first.widgets.set.len(), 1);
        assert_eq!(first.synapses.removed, vec!["s-a".to_string()]);
        assert_eq!(first.synapses.set.len(), 1);
        assert_eq!(first.layout.removed, vec!["l-a".to_string()]);
        assert_eq!(first.layout.set.len(), 1);
        assert_eq!(first.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 }));
        assert_eq!(first.schema, Some("schema-1".to_string()));
        assert_eq!(first.generation.len(), 2);
    }
    //#endregion 🔖️CollectionDiffTests

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSynapse {
            index: 1,
            synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() },
        });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveSynapse { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveLayout { id: "extrude".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural3dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖️OpTextTests


    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Procedural3dOperation::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
        assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
    }
}
//#endregion 🧪️Tests
