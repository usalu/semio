//! 🧰️ Examples and graph editing of the WFC 3D editor: the example snapshots and the wire and slot graph edit gestures. See [`super`].

use super::*;

//#region 🔖️Editor
//#region 🔖️Examples
/// 📚️ Every bundled example the navbar picker offers, as `(id, snapshot)` — the ONE roster
/// `setActiveExample` resolves against, in the order `crate::examples::sources()` declares them.
pub const WFC_3D_EXAMPLE_IDS: [&str; 3] = [crate::examples::two_room_corridor::ID, crate::examples::wall_roof_facade_strip::ID, crate::examples::tower_stack::ID];

/// 📚️ The document one example id names. An EMPTY id is the shell's own "the app's default document"
/// (the picker's `No example` row and the boot announcement both send it), so it resolves to the boot
/// example rather than faulting; an id this artifact never registered faults instead of silently
/// opening a blank document.
pub fn example_snapshot(example_id: &str) -> Result<Wfc3dSnapshot, Fault> {
    match example_id {
        "" => Ok(crate::examples::two_room_corridor::snapshot()),
        id if id == crate::examples::two_room_corridor::ID => Ok(crate::examples::two_room_corridor::snapshot()),
        id if id == crate::examples::wall_roof_facade_strip::ID => Ok(crate::examples::wall_roof_facade_strip::snapshot()),
        id if id == crate::examples::tower_stack::ID => Ok(crate::examples::tower_stack::snapshot()),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.example.unknown"), format!("wfc3d has no example '{other}'"))),
    }
}

/// 🌱️ Swaps the live document for `document` OUTSIDE the undo ledger — an example switch is a
/// different document, not an edit to this one, so it must never mint a history row (the "phantom
/// boot edit" the sibling artifacts hit when they replayed a whole example as mutations).
pub fn load_document_effect(document: &Wfc3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Wfc3dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("wfc", WFC3D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Examples

//#region 🔖️GraphEdit
/// 🕸️ The `operations` array a `nodeGraphEdit` dispatch carries, as JSON text. Both graph canvases —
/// the wasm dag surface and React Flow — speak this one vocabulary (`move`/`connect`/`disconnect`),
/// so the command carries it verbatim and the document-aware mapping happens in
/// [`graph_edit_mutations`], where `z` and the edge roster are actually readable.
pub fn graph_operations_json(args: Option<&semio_framework_value::DslValue>) -> String {
    args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), |operations| serde_json::Value::from(operations).to_string())
}

/// 🕹️ What ONE `nodeGraphEdit` batch asks for: the wire and slot edits the gesture drew, cut or deleted (`prepared`), and
/// the node-drag records of the slots it moved (canvas units).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wfc3dGraphGesture {
    pub prepared: Vec<Wfc3dMutation>,
    pub records: Vec<NodeDragRecord>,
}

/// 🕸️ Reads one released gesture of the closed node-graph record vocabulary (design §13.3, schema
/// `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`) — ONE `Emit`, so a drag is ONE undoable edit rather than one
/// per pointer tick.
///
/// - `move` is the drag record; it becomes a relative `drag-slots` in document units with `dz = 0` — the graph canvas
///   is a plan and never saw the third axis, so a drag cannot flatten a stack.
/// - `connect` mints a deterministic edge id from its endpoints and leaves nothing when that adjacency already exists,
///   because a duplicate id is a fatal mutation invariant, not a second edge.
/// - `disconnect` addresses an existing edge by id and leaves nothing when it is already gone.
/// - `delete` cuts the named adjacencies and deletes the named slots (each cascading its own wires).
/// - A malformed row refuses the whole batch by name; so do `setSlider` and `insertPort`, because a wfc slot graph has
///   no inline sliders and no variadic ports.
pub fn graph_edit_gesture(document: &Wfc3dSnapshot, operations_json: &str) -> Result<Wfc3dGraphGesture, Fault> {
    let refuse = |reason: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.node-graph.row"), format!("nodeGraphEdit refusal: {reason}"));
    let args: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&format!("{{\"operations\":{operations_json}}}"), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| refuse(format!("the operations are not JSON: {error}")))?;
    let rows = node_graph_edit_rows(&args).map_err(refuse)?;
    let mut edge_ids: Vec<String> = document.edges.iter().map(|edge| edge.id.clone()).collect();
    let mut gesture = Wfc3dGraphGesture::default();
    for row in rows {
        match row {
            NodeGraphEditRow::Move(record) => gesture.records.push(record),
            NodeGraphEditRow::Connect { source_node_id, target_node_id, .. } => {
                if let Some(connect) = new_wire(document, &mut edge_ids, &source_node_id, &target_node_id) {
                    gesture.prepared.push(connect);
                }
            }
            NodeGraphEditRow::Disconnect { synapse_id } => cut_wire(&mut edge_ids, &mut gesture.prepared, synapse_id),
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                for synapse_id in synapse_ids {
                    cut_wire(&mut edge_ids, &mut gesture.prepared, synapse_id);
                }
                for node_id in node_ids {
                    if document.slots.iter().any(|slot| slot.id == node_id) && !gesture.prepared.iter().any(|leaf| matches!(leaf, Wfc3dMutation::DeleteSlot(gone) if gone.id == node_id)) {
                        edge_ids.retain(|id| document.edges.iter().find(|edge| edge.id == *id).is_none_or(|edge| edge.from_slot_id != node_id && edge.to_slot_id != node_id));
                        gesture.prepared.push(delete_slot(node_id));
                    }
                }
            }
            NodeGraphEditRow::SetSlider { widget_id, .. } => return Err(refuse(format!("the wfc slot graph has no inline slider {widget_id:?}"))),
            NodeGraphEditRow::InsertPort { node_id, .. } => return Err(refuse(format!("wfc slot {node_id:?} has fixed ports"))),
        }
    }
    Ok(gesture)
}

/// ✂️ Cuts the adjacency `id` names, once per gesture, when the gesture still holds it.
fn cut_wire(edge_ids: &mut Vec<String>, prepared: &mut Vec<Wfc3dMutation>, id: String) {
    if edge_ids.contains(&id) {
        edge_ids.retain(|edge| edge != &id);
        prepared.push(disconnect_slots(id));
    }
}

/// 🧮️ The leaves one gesture means on `document` without a transaction — the wires and deletions first, then the drags.
pub fn graph_edit_mutations(document: &Wfc3dSnapshot, operations_json: &str) -> Result<Vec<Wfc3dMutation>, Fault> {
    let gesture = graph_edit_gesture(document, operations_json)?;
    let mut leaves = gesture.prepared;
    leaves.extend(drag::wfc3d_drag_leaves(document, &gesture.records, WFC_3D_GRAPH_UNIT));
    Ok(leaves)
}

/// 🔗 The `connect-slots` a wire from `from` to `to` adds, minting `edge-{from}-{to}` at its canonical index, or `None`
/// when either end is no slot, the wire loops, or the adjacency or id already exists.
fn new_wire(document: &Wfc3dSnapshot, edge_ids: &mut Vec<String>, from: &str, to: &str) -> Option<Wfc3dMutation> {
    if from.is_empty() || to.is_empty() || from == to {
        return None;
    }
    if !document.slots.iter().any(|slot| slot.id == from) || !document.slots.iter().any(|slot| slot.id == to) {
        return None;
    }
    if document.edges.iter().any(|edge| (edge.from_slot_id == from && edge.to_slot_id == to) || (edge.from_slot_id == to && edge.to_slot_id == from)) {
        return None;
    }
    let id = format!("edge-{from}-{to}");
    if edge_ids.contains(&id) {
        return None;
    }
    let connect = connect_slots(canonical_insertion_index(edge_ids.iter().cloned(), &id), SlotEdge { id: id.clone(), from_slot_id: from.to_string(), to_slot_id: to.to_string(), relation: WFC_3D_DEFAULT_RELATION.into() });
    edge_ids.push(id);
    edge_ids.sort();
    Some(connect)
}
//#endregion 🔖️GraphEdit

/// 📐️ A slot extent or tile weight the UI left at zero still has to be positive, or the mutation's
/// own invariant guard refuses it — the editor supplies the honest unit default rather than emitting
/// a command it knows will be rejected.
pub(super) fn positive_or(value: f64) -> f64 {
    if value > 0.0 {
        value
    } else {
        1.0
    }
}
//#endregion 🔖️Editor
