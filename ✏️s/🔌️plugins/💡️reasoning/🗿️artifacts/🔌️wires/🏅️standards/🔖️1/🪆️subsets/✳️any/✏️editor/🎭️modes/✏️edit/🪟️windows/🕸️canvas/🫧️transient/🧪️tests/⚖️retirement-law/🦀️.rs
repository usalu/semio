//! ⚖️ Wires owners retire under quoted grants: the document and its materialized board retire exactly as a lease, every canvas transient mutation passes the typed-command input lane, and a canvas drag prepares, inverts and re-prepares through the owner's own bundle.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_test_typed_input_law as typed_input_law;
use semio_framework_test_transient_preparation_law as transient_preparation_law;

use semio_s_artifact_reasoning_wires::editor::wires::window_transient::{SetDrag, WiresCanvasTransient, WiresCanvasTransientMutation, WiresCanvasTransientOwner};
use semio_s_artifact_reasoning_wires::{empty_wires_content, empty_wires_snapshot, SemioGraphSnapshot};
use std::sync::Arc;
use transient_preparation_law::transient_preparation_law;
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn drag() -> SetDrag {
    SetDrag { node_id: Some("node-1".into()), start_x: 1.0, start_y: 2.0, last_x: 3.0, last_y: 4.0, zoom: 2.0 }
}

fn dragged() -> WiresCanvasTransient {
    WiresCanvasTransient { drag_node_id: Some("node-1".into()), drag_start_x: 1.0, drag_start_y: 2.0, drag_last_x: 3.0, drag_last_y: 4.0, drag_zoom: 2.0 }
}

/// 🎟️ The canvas drag mutation and the canvas transient state retire through the typed-command input lane.
#[test]
fn wires_canvas_transients_retire_through_the_typed_command_lane() {
    let command = typed_command_retirement_law("wires set-drag", WiresCanvasTransientMutation::SetDrag(drag()));
    let state = typed_input_retirement_law("wires canvas transient", dragged());
    assert!(command.turns > 0 && state.turns > 0, "retiring costs paid turns");
}

/// 🔁️ A canvas drag prepares its post state, prepares its inverse back to the base, prepares again, and retires through the owner's own bundle.
#[test]
fn a_wires_canvas_drag_prepares_inverts_and_retires_through_its_owner() {
    transient_preparation_law::<WiresCanvasTransientOwner>("wires set-drag", WiresCanvasTransient::default(), WiresCanvasTransientMutation::SetDrag(drag()), dragged());
}

/// 📚️ A wires document retires exactly, and its materialized board retires as a lease whether the document is the last alias of the board or not.
#[test]
fn wires_snapshot_retires_with_its_board_lease() {
    let board = Arc::new(empty_wires_content());
    let mut sole = empty_wires_snapshot();
    sole.content = sole.content.with_local_owner(board.clone());
    drop(board);
    let sole_law = typed_input_retirement_law("wires snapshot with the last board alias", sole);
    let shared: Arc<SemioGraphSnapshot> = Arc::new(empty_wires_content());
    let mut aliased = empty_wires_snapshot();
    aliased.content = aliased.content.with_local_owner(shared.clone());
    let aliased_law = typed_input_retirement_law("wires snapshot with a live board alias", aliased);
    assert_eq!(Arc::strong_count(&shared), 1, "retiring a document releases exactly its board alias");
    assert!(sole_law.declared_releases > aliased_law.declared_releases, "only the last alias pays the board release");
    let bare = typed_input_retirement_law("wires snapshot without a board owner", empty_wires_snapshot());
    assert!(bare.declared_births < aliased_law.declared_births, "a board alias lease costs its own frame");
}
