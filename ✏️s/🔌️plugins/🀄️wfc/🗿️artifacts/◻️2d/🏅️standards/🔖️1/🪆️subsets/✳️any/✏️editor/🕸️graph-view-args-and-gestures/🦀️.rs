//! 🕸️ Graph view, args and gestures: the graph view, the editor, the command argument readers, the graph gestures and node graph edit, the example document and the guards. See [`super`].

use super::*;

//#region 🔖️GraphView
/// 🔭️ Canvas world units per DOMAIN unit on the slot graph. Slot coordinates are authored small (a
/// corridor two units wide); the node canvas sizes a node box in the tens of units, so at 1:1 every
/// node of every example lands inside one overlapping speck at the origin — unreadable, and
/// impossible to drag one node out of. This is the one place that scaling lives: the window file
/// stays artifact-agnostic (and copyable by `wfc3d`, which declares its own constant), and
/// `wfc2d_host_snapshot_edit` divides by exactly this when a gesture comes back.
pub const WFC_2D_GRAPH_VIEW_SCALE: f64 = 140.0;

/// 🕸️ The `wfc-graph` window's whole view of this document — see that window's own doc comment for
/// why the projection lives here and not there (so `wfc3d` can copy the window file verbatim).
pub struct Wfc2dGraphView<'a>(pub &'a Wfc2dSnapshot);

impl graph::SlotGraphView for Wfc2dGraphView<'_> {
    fn graph_slots(&self) -> Vec<graph::GraphSlotView> {
        self.0
            .slots
            .iter()
            .map(|slot| graph::GraphSlotView {
                id: slot.id.clone(),
                x: slot.x * WFC_2D_GRAPH_VIEW_SCALE,
                y: slot.y * WFC_2D_GRAPH_VIEW_SCALE,
                width: slot.width * WFC_2D_GRAPH_VIEW_SCALE,
                height: slot.height * WFC_2D_GRAPH_VIEW_SCALE,
                pinned_tile_id: slot.pinned_tile_id.clone(),
            })
            .collect()
    }
    fn graph_edges(&self) -> Vec<graph::GraphEdgeView> {
        self.0.edges.iter().map(|edge| graph::GraphEdgeView { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }).collect()
    }
}
//#endregion 🔖️GraphView

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Wfc2dEditor;

//#region 🔖️Args
fn arg<'a>(args: Option<&'a semio_framework_value::DslValue>, key: &str) -> Option<&'a semio_framework_value::DslValue> {
    match args {
        Some(semio_framework_value::DslValue::Object(entries)) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
        _ => None,
    }
}

pub(super) fn arg_f64(args: Option<&semio_framework_value::DslValue>, key: &str, fallback: f64) -> f64 {
    match arg(args, key) {
        Some(semio_framework_value::DslValue::Number(number)) => number.as_f64(),
        Some(semio_framework_value::DslValue::String(text)) => text.parse().unwrap_or(fallback),
        _ => fallback,
    }
}

pub(super) fn arg_u64(args: Option<&semio_framework_value::DslValue>, key: &str, fallback: u64) -> u64 {
    let value = arg_f64(args, key, fallback as f64);
    if value.is_finite() && value >= 0.0 {
        value as u64
    } else {
        fallback
    }
}

pub(super) fn arg_bool(args: Option<&semio_framework_value::DslValue>, key: &str, fallback: bool) -> bool {
    match arg(args, key) {
        Some(semio_framework_value::DslValue::Bool(value)) => *value,
        Some(semio_framework_value::DslValue::String(text)) => text == "true",
        _ => fallback,
    }
}

pub(super) fn arg_string(args: Option<&semio_framework_value::DslValue>, key: &str) -> String {
    match arg(args, key) {
        Some(semio_framework_value::DslValue::String(text)) => text.clone(),
        Some(other) => semio_framework_pack_json::to_json_string(other),
        None => String::new(),
    }
}

/// 🏷️ An optional string argument: absent or empty answers `None`, so a rule with no relation scope
/// and a rule whose relation is the empty string cannot be confused.
pub(super) fn arg_optional_string(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<String> {
    let value = arg_string(args, key);
    (!value.is_empty()).then_some(value)
}
//#endregion 🔖️Args

//#region 🕹️GraphGestures
/// 🆔️ A fresh edge id for a wire the canvas just drew — `{from}-{to}`, suffixed until it is free, so
/// the same gesture repeated twice cannot collide with the edge it already made.
fn wfc2d_fresh_edge_id(document: &Wfc2dSnapshot, from: &str, to: &str) -> String {
    let base = format!("{from}-{to}");
    if !document.edges.iter().any(|edge| edge.id == base) {
        return base;
    }
    let mut index = 2u32;
    loop {
        let candidate = format!("{base}-{index}");
        if !document.edges.iter().any(|edge| edge.id == candidate) {
            return candidate;
        }
        index += 1;
    }
}

/// 🕹️ What ONE `nodeGraphEdit` batch asks for: the wire and slot edits the gesture drew, cut or deleted (`prepared`), and
/// the node-drag records of the slots it moved (canvas units).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wfc2dGraphGesture {
    pub prepared: Vec<Wfc2dMutation>,
    pub records: Vec<NodeDragRecord>,
}

/// 🕹️ Reads ONE `nodeGraphEdit` batch of the closed node-graph record vocabulary (design §13.3, schema
/// `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`) in row order, each row to its intent leaf: `move` → the drag
/// record (relative `drag-slots`), `connect` → `connect-slots` between two slots, `disconnect` → `disconnect-slots` of an
/// adjacency, `delete` → the named adjacencies cut and the named slots deleted (each cascading its own wires). A
/// malformed row refuses the whole batch by name; so do `setSlider` and `insertPort`, because a wfc slot graph has no
/// inline sliders and no variadic ports. A row whose target the document no longer holds (a wire already drawn, an
/// adjacency already cut) leaves nothing.
pub fn wfc2d_node_graph_edit(document: &Wfc2dSnapshot, operations_json: &str) -> Result<Wfc2dGraphGesture, Fault> {
    let refuse = |reason: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.node-graph.row"), format!("nodeGraphEdit refusal: {reason}"));
    let args: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&format!("{{\"operations\":{operations_json}}}"), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| refuse(format!("the operations are not JSON: {error}")))?;
    let rows = node_graph_edit_rows(&args).map_err(refuse)?;
    let mut gesture = Wfc2dGraphGesture::default();
    for row in rows {
        match row {
            NodeGraphEditRow::Move(record) => gesture.records.push(record),
            NodeGraphEditRow::Connect { source_node_id, target_node_id, .. } => {
                if let Some(edge) = wfc2d_new_wire(document, &gesture.prepared, &source_node_id, &target_node_id) {
                    gesture.prepared.push(connect_slots(edge));
                }
            }
            NodeGraphEditRow::Disconnect { synapse_id } => wfc2d_cut_wire(document, &mut gesture.prepared, synapse_id),
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                for synapse_id in synapse_ids {
                    wfc2d_cut_wire(document, &mut gesture.prepared, synapse_id);
                }
                for node_id in node_ids {
                    if document.slots.iter().any(|slot| slot.id == node_id) && !gesture.prepared.iter().any(|leaf| matches!(leaf, Wfc2dMutation::DeleteSlot(gone) if gone.id == node_id)) {
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

/// ✂️ Cuts the adjacency `id` names, once per gesture, when the document holds it.
fn wfc2d_cut_wire(document: &Wfc2dSnapshot, prepared: &mut Vec<Wfc2dMutation>, id: String) {
    if document.edges.iter().any(|edge| edge.id == id) && !prepared.iter().any(|leaf| matches!(leaf, Wfc2dMutation::DisconnectSlots(cut) if cut.id == id)) {
        prepared.push(disconnect_slots(id));
    }
}

/// 🔗️ The edge a wire from `from` to `to` adds, or `None` when either end is no slot, the wire loops, or the
/// adjacency already exists in the document or in this gesture.
fn wfc2d_new_wire(document: &Wfc2dSnapshot, prepared: &[Wfc2dMutation], from: &str, to: &str) -> Option<Wfc2dSlotEdge> {
    if from == to || !document.slots.iter().any(|slot| slot.id == from) || !document.slots.iter().any(|slot| slot.id == to) {
        return None;
    }
    let drawn = prepared.iter().any(|leaf| matches!(leaf, Wfc2dMutation::ConnectSlots(wire) if same_adjacency((&wire.edge.from_slot_id, &wire.edge.to_slot_id), (from, to))));
    if drawn || document.edges.iter().any(|edge| same_adjacency((&edge.from_slot_id, &edge.to_slot_id), (from, to))) {
        return None;
    }
    Some(Wfc2dSlotEdge { id: wfc2d_fresh_edge_id(document, from, to), from_slot_id: from.to_string(), to_slot_id: to.to_string(), relation: crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() })
}

/// 🔗️ Whether two endpoint pairs name the SAME adjacency. `wfc-graph`'s two ports exist only to give
/// the canvas something to drag a wire between — the relation itself is undirected and the solver
/// symmetrises it, so `(a, b)` and `(b, a)` are one adjacency and never a removal plus an addition.
fn same_adjacency(left: (&str, &str), right: (&str, &str)) -> bool {
    (left.0 == right.0 && left.1 == right.1) || (left.0 == right.1 && left.1 == right.0)
}
//#endregion 🕹️GraphGestures

//#region 🗃️Examples
/// 🗃️ The document one registered example id names, or `None` for an id this subset never shipped —
/// the picker offers exactly `crate::examples::sources()`, so an unknown id is a real fault, not a
/// silent blank document.
pub fn wfc2d_example_document(example_id: &str) -> Option<Wfc2dSnapshot> {
    match example_id {
        crate::examples::two_room_corridor::ID => Some(crate::examples::two_room_corridor::document()),
        crate::examples::wall_roof_facade_strip::ID => Some(crate::examples::wall_roof_facade_strip::document()),
        crate::examples::hex_ring::ID => Some(crate::examples::hex_ring::document()),
        crate::examples::terrain_ring::ID => Some(crate::examples::terrain_ring::document()),
        _ => None,
    }
}

/// 🗃️ The whole-document load the picker answers with. A `LoadDocument` effect is NOT an edit, so
/// re-selecting the example the app already booted on leaves `canUndo` false instead of minting the
/// phantom history entry a "replace every collection" mutation set would.
pub(super) fn wfc2d_load_document_effect(document: &Wfc2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Wfc2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("wfc", WFC_2D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🗃️Examples

//#region 🛂️Guards
/// 🛂️ Refuses a verb that names an entity the open document does not hold. A palette row carries a
/// STATIC argument default, so a default authored against one example reaches the guest unchanged
/// after the picker loads another — and a mutation against a missing id is either a fatal outcome the
/// ledger still journals or a silent no-op. Both read to a user as "the button did nothing"; a named
/// domain fault reads as what it is.
pub(super) fn require_slot(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.slots.iter().any(|slot| slot.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.slot.unknown-slot"), format!("this document declares no slot '{id}'"))),
    }
}

pub(super) fn require_tile(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.tiles.iter().any(|tile| tile.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.tile.unknown-tile"), format!("this document declares no tile '{id}'"))),
    }
}

pub(super) fn require_edge(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.edges.iter().any(|edge| edge.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.edge.unknown-edge"), format!("this document declares no adjacency '{id}'"))),
    }
}

pub(super) fn require_rule(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.rules.iter().any(|rule| rule.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.rule.unknown-rule"), format!("this document declares no rule '{id}'"))),
    }
}

pub(super) fn require_fresh(taken: bool, entity: &'static str, id: &str) -> Result<(), Fault> {
    match taken {
        false => Ok(()),
        true => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.id.taken"), format!("this document already holds a {entity} '{id}'"))),
    }
}
//#endregion 🛂️Guards
//#endregion 🔖️Editor
