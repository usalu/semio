//! ⚡️ Procedural 2D app — operation enum + laws (constitutional: op).

use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use playbook::{apply_generation_operation, invert_generation_operation, GenerationOperation};
use procedural_2d::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_id, widget_to_dsl,
    CameraJsonDsl, FormGenerationDsl, Procedural2dDocument, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
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
/// 🩹️ Sparse procedural-2d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
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

/// 🧮️ Procedural-2d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single {@link GenerationOperation} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Procedural2dOperation {
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

impl Operation<Procedural2dDocument> for Procedural2dOperation {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        let mut diff = Procedural2dDiff::default();
        match self {
            Procedural2dOperation::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural2dOperation::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural2dOperation::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural2dOperation::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural2dOperation::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural2dOperation::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural2dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural2dOperation::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural2dOperation::Generation(operation) => diff.generation.push(operation.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural2dOperation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural2dOperation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural2dOperation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural2dOperation::RemoveLayout { id: id.clone() }],
            },
            Procedural2dOperation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetCamera { .. } => vec![Procedural2dOperation::SetCamera { camera: fixture.camera.clone() }],
            Procedural2dOperation::SetSchema { .. } => vec![Procedural2dOperation::SetSchema { schema: fixture.schema.clone() }],
            Procedural2dOperation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural2dOperation::Generation).collect(),
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is ephemeral
/// view state (plugin runtime), never a document operation. Lets action handlers keep computing the target
/// fixture via `FlowHost` while emitting granular operations.
pub fn procedural2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dOperation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural2dOperation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural2dOperation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural2dOperation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural2dOperation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖️Operations

//#region 🔖️OpText
/// ⚡️ Local twin of `Procedural2dOperation` — flattens the `Generation(GenerationOperation)` newtype
/// variant into its own four top-level keyword variants (mirroring the OLD hand-rolled op-line
/// keywords `generation-add`/`generation-remove`/`generation-rename`/`generation-update-values`)
/// since a `#[derive(dsl::DslOps)]` enum's variants are each their own tagged record, not a nested
/// enum-in-enum.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural2dOperationDsl {
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
        value: dsl::DslValue,
    }
}

fn procedural2d_operation_to_dsl(operation: &Procedural2dOperation) -> Procedural2dOperationDsl {
    match operation {
        Procedural2dOperation::SetWidget { index, widget } => Procedural2dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural2dOperation::RemoveWidget { id } => Procedural2dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural2dOperation::SetSynapse { index, synapse } => Procedural2dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural2dOperation::RemoveSynapse { id } => Procedural2dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural2dOperation::SetLayout { id, layout } => Procedural2dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural2dOperation::RemoveLayout { id } => Procedural2dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural2dOperation::SetCamera { camera } => Procedural2dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural2dOperation::SetSchema { schema } => Procedural2dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Add { generation }) => Procedural2dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural2dOperation::Generation(GenerationOperation::Remove { id }) => Procedural2dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural2dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural2dOperationDsl::GenerationUpdateValues {
                id: id.clone(),
                question_id: question_id.clone(),
                value: dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null),
            }
        }
    }
}

fn procedural2d_operation_from_dsl(operation: Procedural2dOperationDsl) -> Result<Procedural2dOperation, store::TextError> {
    Ok(match operation {
        Procedural2dOperationDsl::SetWidget { index, widget } => Procedural2dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural2dOperationDsl::RemoveWidget { id } => Procedural2dOperation::RemoveWidget { id },
        Procedural2dOperationDsl::SetSynapse { index, synapse } => Procedural2dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural2dOperationDsl::RemoveSynapse { id } => Procedural2dOperation::RemoveSynapse { id },
        Procedural2dOperationDsl::SetLayout { id, layout } => Procedural2dOperation::SetLayout { id, layout: layout_from_dsl(layout) },
        Procedural2dOperationDsl::RemoveLayout { id } => Procedural2dOperation::RemoveLayout { id },
        Procedural2dOperationDsl::SetCamera { camera } => Procedural2dOperation::SetCamera { camera: camera_from_dsl(camera) },
        Procedural2dOperationDsl::SetSchema { schema } => Procedural2dOperation::SetSchema { schema },
        Procedural2dOperationDsl::GenerationAdd { generation } => Procedural2dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural2dOperationDsl::GenerationRemove { id } => Procedural2dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural2dOperationDsl::GenerationRename { id, name } => Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural2dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural2dOperation::Generation(GenerationOperation::UpdateValues {
            id,
            question_id,
            value: dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null),
        }),
    })
}

/// ⚡️ `Procedural2dOperation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural2dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural2dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural2dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural2dOperationDsl as protocol::OpText>::print_op(&procedural2d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Procedural2dOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Procedural2dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural2d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural2dOperationDsl::decode_op(bytes)?;
        procedural2d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural2d operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpText

pub type Procedural2dEnvelope = DocumentEnvelope<Procedural2dDocument, Procedural2dOperation>;
pub type Procedural2dStore = DocumentStore<Procedural2dDocument, Procedural2dOperation>;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use procedural_2d_engine::empty_procedural2d_projection;
    use store::test_support;
    use protocol::OpText;
    use vcs::apply_operation;

    fn round_trip(projection: &Procedural2dDocument, operation: &Procedural2dOperation) -> Procedural2dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
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
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural2dOperation::SetCamera { .. })));
    }

    #[test]
    fn remove_and_readd_widget_round_trips() {
        let base = empty_procedural2d_projection();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &Procedural2dOperation::RemoveWidget { id: removed_id.clone() });
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_add() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dOperation::SetWidget { widget, .. } if widget_id(widget) == "note-1")));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_projection();
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    //#region 🔖️DiffTests
    #[test]
    fn diff_absorb_merges_vecs_and_updates_scalars_when_present() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 }), ..Default::default() };
        diff.widgets.removed.push("w1".into());

        diff.absorb(Procedural2dDiff {
            widgets: WidgetsDiff { removed: vec!["w2".into()], set: vec![(0, Widget::InputNote { id: "note".into(), text: String::new() })] },
            synapses: SynapsesDiff { removed: vec!["s1".into()], set: vec![] },
            layout: LayoutDiff { removed: vec![], set: vec![("l1".into(), WidgetLayout { x: 3.0, y: 4.0 })] },
            camera: Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }),
            schema: Some("flow.fixture".into()),
            generation: vec![GenerationOperation::Remove { id: "g1".into() }],
        });

        assert_eq!(diff.widgets.removed, vec!["w1".to_string(), "w2".to_string()]);
        assert_eq!(diff.widgets.set.len(), 1);
        assert_eq!(diff.synapses.removed, vec!["s1".to_string()]);
        assert_eq!(diff.layout.set.len(), 1);
        assert_eq!(diff.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
        assert_eq!(diff.generation.len(), 1);
    }

    #[test]
    fn diff_absorb_keeps_scalar_when_incoming_is_none() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }), schema: Some("flow.fixture".into()), ..Default::default() };
        diff.absorb(Procedural2dDiff::default());
        assert_eq!(diff.camera, Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
    }

    #[test]
    fn diff_apply_inserts_new_widget_and_replaces_existing_by_id() {
        let projection = empty_procedural2d_projection();
        let existing_id = widget_id(&projection.fixture.widgets[1]).to_string();
        let diff = Procedural2dDiff {
            widgets: WidgetsDiff {
                removed: vec![],
                set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })],
            },
            ..Default::default()
        };
        let next = diff.apply(&projection);
        assert_eq!(next.fixture.widgets.len(), projection.fixture.widgets.len() + 1);
        let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced widget present");
        assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
        assert_eq!(widget_id(next.fixture.widgets.last().expect("inserted widget")), "brand-new");
    }

    #[test]
    fn diff_apply_updates_camera_and_schema_only_when_present() {
        let projection = empty_procedural2d_projection();
        let untouched = Procedural2dDiff::default().apply(&projection);
        assert_eq!(untouched.fixture.camera, projection.fixture.camera);
        assert_eq!(untouched.fixture.schema, projection.fixture.schema);

        let changed = Procedural2dDiff { camera: Some(CameraJson { x: 5.0, y: 6.0, zoom: 3.0 }), schema: Some("other.schema".into()), ..Default::default() }.apply(&projection);
        assert_eq!(changed.fixture.camera, CameraJson { x: 5.0, y: 6.0, zoom: 3.0 });
        assert_eq!(changed.fixture.schema, "other.schema");
    }
    //#endregion 🔖️DiffTests

    //#region 🔖️OperationBackwardsTests
    #[test]
    fn set_widget_backwards_restores_replaced_widget() {
        let base = empty_procedural2d_projection();
        let id = widget_id(&base.fixture.widgets[1]).to_string();
        round_trip(&base, &Procedural2dOperation::SetWidget { index: 1, widget: Widget::InputNote { id, text: "replaced".into() } });
    }

    #[test]
    fn set_widget_backwards_removes_newly_inserted_widget() {
        let base = empty_procedural2d_projection();
        let after = round_trip(&base, &Procedural2dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "brand-new".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "brand-new"));
    }

    #[test]
    fn remove_widget_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveWidget { id: "does-not-exist".into() };
        assert!(op.backwards(&base).is_empty());
        let after = round_trip(&base, &op);
        assert_eq!(after, base);
    }

    #[test]
    fn set_synapse_backwards_restores_replaced_synapse() {
        let base = empty_procedural2d_projection();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id, from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: "changed".into() } });
    }

    #[test]
    fn set_synapse_backwards_removes_newly_inserted_synapse() {
        let base = empty_procedural2d_projection();
        let synapse = SynapseSpec { id: "brand-new-synapse".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() };
        let after = round_trip(&base, &Procedural2dOperation::SetSynapse { index: 0, synapse });
        assert!(after.fixture.synapses.iter().any(|s| s.id == "brand-new-synapse"));
    }

    #[test]
    fn remove_synapse_backwards_restores_removed_synapse() {
        let base = empty_procedural2d_projection();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dOperation::RemoveSynapse { id });
    }

    #[test]
    fn remove_synapse_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveSynapse { id: "missing".into() };
        assert!(op.backwards(&base).is_empty());
    }

    #[test]
    fn set_layout_backwards_restores_prior_layout_entry() {
        let mut base = empty_procedural2d_projection();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 1.0, y: 1.0 });
        round_trip(&base, &Procedural2dOperation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 9.0, y: 9.0 } });
    }

    #[test]
    fn set_layout_backwards_removes_newly_created_layout_entry() {
        let base = empty_procedural2d_projection();
        assert!(base.fixture.layout.is_empty());
        let after = round_trip(&base, &Procedural2dOperation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } });
        assert!(after.fixture.layout.contains_key("slider"));
    }

    #[test]
    fn remove_layout_backwards_restores_removed_layout_entry() {
        let mut base = empty_procedural2d_projection();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 4.0, y: 5.0 });
        round_trip(&base, &Procedural2dOperation::RemoveLayout { id: "slider".into() });
    }

    #[test]
    fn remove_layout_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveLayout { id: "missing".into() };
        assert!(op.backwards(&base).is_empty());
    }

    #[test]
    fn set_camera_backwards_restores_prior_camera() {
        let base = empty_procedural2d_projection();
        round_trip(&base, &Procedural2dOperation::SetCamera { camera: CameraJson { x: 42.0, y: -3.0, zoom: 5.0 } });
    }

    #[test]
    fn set_schema_backwards_restores_prior_schema() {
        let base = empty_procedural2d_projection();
        round_trip(&base, &Procedural2dOperation::SetSchema { schema: "changed.schema".into() });
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
            assert!(operations.iter().any(|op| matches!(op, Procedural2dOperation::SetWidget { widget, .. } if widget_id(widget) == id)));
        }
    }
    //#endregion 🔖️FixtureOpsTests

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSynapse {
            index: 1,
            synapse: SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() },
        });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveSynapse { id: "s1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetLayout { id: "rect".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveLayout { id: "rect".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️OpTextErrorTests
    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Procedural2dOperation::parse_op("bogus-op id=\"x\"").unwrap_err();
        assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
    }

    #[test]
    fn op_text_parse_rejects_non_integer_index() {
        let error = Procedural2dOperation::parse_op("set-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
        assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖️OpTextErrorTests
}
//#endregion 🧪️Tests
