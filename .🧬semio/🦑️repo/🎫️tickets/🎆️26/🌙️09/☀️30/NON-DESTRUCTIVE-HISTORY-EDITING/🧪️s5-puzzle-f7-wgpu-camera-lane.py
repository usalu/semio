"""🎥️ F7, wgpu half (S5-PUZZLE) — STAGED, NOT COMPILED: land after build B1 under `landing` + `puzzle`.

The React host already publishes a board camera move as the view verb `setCamera` (`🧪️s5-puzzle-f7-camera-lane.py`). The
wgpu host still carries the camera inside `applyBoardEvents` on two paths; this wave moves both onto the view lane and
then removes the guests' camera arm, so no host and no guest treats a camera move as a board event any more:

1. Board engine (`♾️infinite/🎲️board`): a finished pan seals NO board event. `BoardPointerPlan::view_camera()` names the
   camera the host publishes; `publishes()` says whether a plan puts anything on the action queue.
2. wgpu host: `coalesce_owned_board_events` answers the latest camera beside the board rows (the shared corpus, exactly);
   the buffered flushes publish `setCamera` for it; a retained plan with a view camera publishes `setCamera` when its
   commit is admitted.
3. Guest: puzzle 2d `applyBoardEvents` no longer reads a `camera` row; the two 2d laws that sent one go through
   `setCamera`. Puzzle 5d is NOT in this wave: its retained board-events work (`✏️editor/🦀️.rs` `camera2d` stages) and its
   retained-jobs vector `boardCameraPublishesWindowConfig` carry the camera through their own stages and go in their own wave.

All or nothing: every anchor must resolve the stated number of times (or its replacement must already be present) before
any file is written. `--check` writes nothing. After applying, in this order (gate v5, `CARGO_BUILD_JOBS=3`):
`cargo check -p semio-framework-os-infinite -p semio-framework-os-renderer-wgpu --lib`,
`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-5d --lib --features
semio-s-artifact-puzzle-2d/component-app-assembly,semio-s-artifact-puzzle-5d/component-app-assembly` (5d depends on the engine),
`cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-puzzle`, then the laws:
`cargo test -p semio-framework-os-infinite --lib -- pointer_pan pointer_plan selection_move`,
`RUST_MIN_STACK=67108864 cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d`,
`RUST_MIN_STACK=67108864 cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly
--lib -- camera apply_board_events`. The wgpu standalone test helpers (`🧪️tests/🧊️wgpu-standalone/🦀️.rs`
`board_take_buffer_coalesced`, `board_flush_events_action`, `board_drain_and_maybe_flush`) still compile; a law that
expected a camera row inside its `applyBoardEvents` action must expect a `setCamera` action instead.

Run: `python3 <this file> [--check]`.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
ENGINE = f"{OSM}/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs"
CANVAS = f"{OSM}/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas"
WGPU = f"{CANVAS}/🎯️targets/🧊️wgpu/🦀️.rs"
WGPU_LAW = f"{CANVAS}/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"
PUZZLE = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts"
P2 = f"{PUZZLE}/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
P5 = f"{PUZZLE}/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"

HUNKS = []


def hunk(path, old, new, count=1):
    HUNKS.append((path, old, new, count))


# 🎲️ Board engine: a finished pan is a view camera, not a board event.
hunk(
    ENGINE,
    """        pub fn event_count(&self) -> usize {
            match self.kind {
                BoardPointerPlanKind::FinishPan { .. } => 1,
                BoardPointerPlanKind::FinishDrag""",
    """        /// 🎥️ The camera this plan settles on the view lane: a finished pan's. A camera move is view state, so the host
        /// publishes it as its view verb (`setCamera`) and never as a board event.
        pub fn view_camera(&self) -> Option<[f64; 3]> {
            match self.kind {
                BoardPointerPlanKind::FinishPan { camera } => Some(camera),
                _ => None,
            }
        }

        /// 📮️ Whether committing this plan puts anything on the host's action queue: board events or a view camera.
        pub fn publishes(&self) -> bool {
            self.event_count() > 0 || self.view_camera().is_some()
        }

        pub fn event_count(&self) -> usize {
            match self.kind {
                BoardPointerPlanKind::FinishDrag""",
)
hunk(
    ENGINE,
    """        fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
            let mut output = String::with_capacity(BOARD_POINTER_BYTE_CAPACITY);
            self.write_events_json(&mut output)?;
            self.output[..output.len()].copy_from_slice(output.as_bytes());
            self.output_len = output.len() as u16;
            Ok(())
        }
""",
    """        /// 🈳️ Seals the empty event page of a hand-built plan that publishes no board event.
        #[cfg(test)]
        fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
            self.output_len = 0;
            self.output_raw("[]")
        }
""",
)
hunk(
    ENGINE,
    """        pub fn write_events_json(&self, output: &mut String) -> Result<(), BoardPointerPlanFault> {
            output.clear();
            match self.kind {
                BoardPointerPlanKind::FinishPan { camera } => {
                    output.push_str("[{\\"name\\":\\"camera\\",\\"payload\\":{");
                    output.push_str("\\"x\\":");
                    output.push_str(&camera[0].to_string());
                    output.push_str(",\\"y\\":");
                    output.push_str(&camera[1].to_string());
                    output.push_str(",\\"zoom\\":");
                    output.push_str(&camera[2].to_string());
                    output.push_str("}}]");
                }
                _ => output.push_str("[]"),
            }
            if output.len() > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            Ok(())
        }

    }
""",
    """    }
""",
)
hunk(
    ENGINE,
    """                    let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::FinishPan { camera });
                    plan.seal_events()?;
                    Ok(plan)
""",
    """                    Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::FinishPan { camera }))
""",
)

# 🧊️ wgpu host: the coalescer answers the camera beside the board rows.
hunk(
    WGPU,
    """pub struct CoalescedBoardEvents {
    pub flush_now: bool,
    pub events_json: String,
}
""",
    """/// 📬️ One coalesced board batch: the board rows for `applyBoardEvents`, the latest camera for the view lane (`setCamera`)
/// and whether the buffer flushes now.
pub struct CoalescedBoardEvents {
    pub flush_now: bool,
    pub events_json: String,
    pub camera: Option<[f64; 3]>,
}

impl CoalescedBoardEvents {
    /// 📬️ Whether any board row remains for `applyBoardEvents`.
    pub fn has_rows(&self) -> bool {
        self.events_json != "[]"
    }
}
""",
)
hunk(
    WGPU,
    """/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins both. Drops every transient row, keeps
/// only the latest `camera` (first), keeps every other row in order, and flushes now for a terminal row —
""",
    """/// `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins both. Drops every transient row, takes the
/// latest `camera` out for the view lane, keeps every other row in order, and flushes now for a terminal row —
""",
)
hunk(
    WGPU,
    """    let mut flush_now = false;
    if let Some(camera) = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).last() {
        append_board_owned_event(&mut output, &mut first, camera)?;
    }
    for event in queue.iter() {
""",
    """    let mut flush_now = false;
    let camera = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).filter_map(|event| engine_camera_from_json(event.payload_json())).last().map(|(x, y, zoom)| [x, y, zoom]);
    for event in queue.iter() {
""",
)
hunk(
    WGPU,
    """    output.push(']');
    Ok(CoalescedBoardEvents { flush_now, events_json: output })
}
""",
    """    output.push(']');
    Ok(CoalescedBoardEvents { flush_now, events_json: output, camera })
}

/// 🧮️ Reserves exactly the actions one coalesced board batch publishes: `setCamera` for its camera, `applyBoardEvents`
/// for its rows.
fn reserve_board_coalesced<'input>(input: &'input mut ui_wgpu::wgpu::InputState<ActionDescriptor>, controller_id: &str, coalesced: &CoalescedBoardEvents) -> Result<ui_wgpu::wgpu::BoundedActionBatchReservation<'input>, ui_wgpu::wgpu::BoundedActionFault> {
    let camera_bytes = if coalesced.camera.is_some() { ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "setCamera", "camera", "x", "y", "zoom"])? } else { 0 };
    let rows_bytes = if coalesced.has_rows() { ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", &coalesced.events_json])? } else { 0 };
    input.reserve_actions(usize::from(coalesced.camera.is_some()) + usize::from(coalesced.has_rows()), camera_bytes + rows_bytes)
}

/// 📤️ Writes one coalesced board batch: its camera on the view lane first, then its board rows.
fn write_board_coalesced_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, coalesced: &CoalescedBoardEvents) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if let Some(camera) = coalesced.camera {
        write_board_camera_flat(batch, controller_id, camera)?;
    }
    if coalesced.has_rows() {
        write_board_events_flat(batch, controller_id, &coalesced.events_json)?;
    }
    Ok(())
}
""",
)
hunk(
    WGPU,
    """fn board_peek_buffer_coalesced(surface_id: &str) -> Option<String> {
""",
    """fn board_peek_buffer_coalesced(surface_id: &str) -> Option<CoalescedBoardEvents> {
""",
)
hunk(
    WGPU,
    """        let coalesced = coalesce_owned_board_events(queue).ok()?;
        (coalesced.events_json != "[]").then_some(coalesced.events_json)
    })
}

/// 📤️ Unconditional drain + coalesce + dispatch, mirroring `flushBoardEvents`""",
    """        let coalesced = coalesce_owned_board_events(queue).ok()?;
        (coalesced.has_rows() || coalesced.camera.is_some()).then_some(coalesced)
    })
}

/// 📤️ Unconditional drain + coalesce + dispatch, mirroring `flushBoardEvents`""",
)
hunk(
    WGPU,
    """    board_drain_into_buffer(surface_id);
    let Some(events_json) = board_peek_buffer_coalesced(surface_id) else {
        return Ok(false);
    };
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", &events_json])?;
    let mut reservation = input.reserve_actions(1, bytes)?;
    write_board_events_flat(&mut reservation, controller_id, &events_json)?;
    reservation.publish_with_checked(|| {
""",
    """    board_drain_into_buffer(surface_id);
    let Some(coalesced) = board_peek_buffer_coalesced(surface_id) else {
        return Ok(false);
    };
    let mut reservation = reserve_board_coalesced(input, controller_id, &coalesced)?;
    write_board_coalesced_flat(&mut reservation, controller_id, &coalesced)?;
    reservation.publish_with_checked(|| {
""",
)
hunk(
    WGPU,
    """    if let Some(events_json) = board_peek_buffer_coalesced(surface_id) {
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", &events_json])?;
        let mut reservation = input.reserve_actions(1, bytes)?;
        write_board_events_flat(&mut reservation, controller_id, &events_json)?;
        reservation.publish_with_checked(|| board_retire_pending_events(surface_id))?;
        return Ok(true);
    }
""",
    """    if let Some(coalesced) = board_peek_buffer_coalesced(surface_id) {
        let mut reservation = reserve_board_coalesced(input, controller_id, &coalesced)?;
        write_board_coalesced_flat(&mut reservation, controller_id, &coalesced)?;
        reservation.publish_with_checked(|| board_retire_pending_events(surface_id))?;
        return Ok(true);
    }
""",
)
hunk(
    WGPU,
    """    board_drain_into_buffer(surface_id);
    let events_json = board_peek_buffer_coalesced(surface_id);
    let retire_events = events_json.is_some();
    write_board_camera_flat(&mut reservation, controller_id, camera)?;
""",
    """    board_drain_into_buffer(surface_id);
    let buffered = board_peek_buffer_coalesced(surface_id);
    let retire_events = buffered.is_some();
    let events_json = buffered.filter(CoalescedBoardEvents::has_rows).map(|coalesced| coalesced.events_json);
    write_board_camera_flat(&mut reservation, controller_id, camera)?;
""",
)

# 🧊️ wgpu host: a retained plan with a view camera publishes `setCamera` when its commit is admitted.
hunk(
    WGPU,
    """    let emits = plan.event_count() > 0;
    let claim = if emits {
""",
    """    if let Some(camera) = plan.view_camera() {
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "setCamera", "camera", "x", "y", "zoom"])?;
        let mut reservation = input.reserve_actions(1, bytes)?;
        write_board_camera_flat(&mut reservation, controller_id, camera)?;
        let mut admitted = None;
        let published = reservation.publish_with_checked(|| {
            let result = ENGINE_SURFACES.with(|cell| {
                let mut map = cell.borrow_mut();
                let Some(entry) = map.get_mut(surface_id) else {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                };
                if entry.board_pointer_claim.is_some() || entry.board_pointer_controller_id.is_some() {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let Some(host) = entry.board_host.as_mut() else {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
                };
                host.begin_pointer_commit(plan).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                if let Some(pointer_inside) = pointer_inside {
                    entry.board_pointer_inside = pointer_inside;
                }
                Ok(())
            });
            let ok = result.is_ok();
            admitted = Some(result);
            ok
        });
        return admitted.unwrap_or(Ok(())).and(published);
    }
    let emits = plan.event_count() > 0;
    let claim = if emits {
""",
)
hunk(
    WGPU,
    """        let emits = plan.event_count() > 0;
        begin_board_pointer_commit(""",
    """        let emits = plan.publishes();
        begin_board_pointer_commit(""",
    3,
)

# 🧊️ The wgpu corpus law: exactly the corpus again.
hunk(
    WGPU_LAW,
    """/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same board rows in the same order,
/// same flush verdict — so one drag reaches the guest as ONE batch on both hosts. The corpus states the latest camera
/// beside the rows (the view lane, `setCamera`); this host still leads its board batch with that camera row, so the
/// law requires exactly the corpus rows behind exactly the corpus camera.
""",
    """/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same board rows in the same order,
/// same camera for the view lane, same flush verdict — so one drag reaches the guest as ONE batch on both hosts and
/// a pan is never a board event on either.
""",
)
hunk(
    WGPU_LAW,
    """        let camera = &case["expect"]["camera"];
        let mut expected: Vec<Value> = if camera.is_null() { Vec::new() } else { vec![serde_json::json!({ "name": "camera", "payload": camera })] };
        expected.extend(case["expect"]["events"].as_array().expect("events").iter().cloned());
        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), Value::Array(expected), "{name}");
""",
    """        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), case["expect"]["events"], "{name}");
        assert_eq!(coalesced.camera.map_or(Value::Null, |[x, y, zoom]| serde_json::json!({ "x": x, "y": y, "zoom": zoom })), case["expect"]["camera"], "{name}");
""",
)

# 🧩️ Guests: a camera row is no board event any more.
hunk(
    f"{P2}/🎮️commands/🎲️apply-board-events/🦀️.rs",
    """        match name {
            "camera" => {
                window_bodies = true;
            }
            "select" => {
""",
    """        match name {
            "select" => {
""",
)
hunk(
    f"{P2}/🎮️commands/🎲️apply-board-events/🦀️.rs",
    """        match name {
            "camera" => {
                set_runtime_camera(&mut envelope.runtime, &payload);
            }
            // 🕹️ Selection is framework-owned""",
    """        match name {
            // 🕹️ Selection is framework-owned""",
)
hunk(
    f"{P2}/🎮️commands/🎲️apply-board-events/🦀️.rs",
    "puzzle2d_selection_write, set_runtime_camera, Puzzle2dActionCtx,",
    "puzzle2d_selection_write, Puzzle2dActionCtx,",
)
hunk(
    f"{P2}/🎮️commands/🎲️apply-board-events/🦀️.rs",
    "/// `applyBoardEvents` fires on every select/drag/zoom, so getting this right is most of the\n",
    "/// `applyBoardEvents` fires on every select and drag (a camera move is the view verb `setCamera`), so getting this right is most of the\n",
)
hunk(
    f"{P2}/🧪️tests/🔬️unit/🦀️.rs",
    """/// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
/// *pre-action* value) against the runtime and blindly overwrite it, reverting a plain `camera`
/// board event (used for the live wheel-zoom echo) before it ever committed.
#[semio_framework_async_macros::async_test]
async fn apply_board_events_camera_event_commits() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), None).expect("camera event");
    assert_eq!(committed_edits(&result), 0, "a camera board event must never produce a document operation");
""",
    """/// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
/// *pre-action* value) against the runtime and blindly overwrite it, reverting a camera move (the view verb
/// `setCamera`, which a board pan, pinch and wheel all dispatch) before it ever committed.
#[semio_framework_async_macros::async_test]
async fn a_board_camera_move_commits_through_the_view_verb() {
    let mut app = app();
    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 5.0, "y": 6.0, "zoom": 1.2 } })), None).expect("camera move");
    assert_eq!(committed_edits(&result), 0, "a camera move must never produce a document operation");
""",
)
hunk(
    f"{P2}/🧪️tests/🔬️unit/🦀️.rs",
    """async fn camera_event_declares_window_only_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), None).expect("camera event");
""",
    """async fn a_camera_move_declares_window_only_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 1.0, "y": 2.0, "zoom": 1.0 } })), None).expect("camera move");
""",
)


def main():
    check = "--check" in sys.argv[1:]
    contents, pending = {}, 0
    for path, old, new, expected in HUNKS:
        text = contents.get(path)
        if text is None:
            text = (ROOT / path).read_text(encoding="utf-8")
        count = text.count(old)
        if count == 0 and new and new in text:
            contents[path] = text
            continue
        if count != expected:
            sys.exit(f"{path}: anchor resolves {count} times, expected {expected}:\n{old[:200]}")
        contents[path] = text.replace(old, new)
        pending += 1
        if check:
            print(f"pending: {path}: {old.splitlines()[0][:100]}")
    if check:
        print(f"{pending} pending of {len(HUNKS)} hunks in {len(contents)} files")
        return
    for path, text in contents.items():
        if (ROOT / path).read_text(encoding="utf-8") != text:
            (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {pending} of {len(HUNKS)} hunks in {len(contents)} files")


if __name__ == "__main__":
    main()
