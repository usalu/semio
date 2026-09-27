#!/usr/bin/env python3
"""👥️ WG11 session 14 — prepared patch for window 3 (re-derives WG9's `s13-board-presence-pointer-patch.py` with a behavioural law).

Measured (WG9, wasm32 run s13d, puzzle2d on 7800 B3): A's 293 presence heartbeats carried both board views and a pointer in 0 of
them while A's pointer crossed its board; B painted no cursor. Root cause (renderer, target-neutral): `AppInteractionState::
handle_pointer_move` hands a move to `ShellState::handle_pointer_move_for` — the only writer of `presence_pointer` — only when no
published scene surface claims it, so the presence pointer froze exactly when it entered a board.

Fix (as WG9's): the renderer notes the presence pointer for every move before the scene/chrome routing. Law (new, replaces WG9's
source-order text law): the fixture's `publish` cases are driven through the renderer's OWN move path — a painted board, the
`AppInteractionState` the winit app and the browser worker drive, `handle_pointer_move` over the board — and the heartbeat's
window view must equal the fixture's expected view; at least one case must be claimed by the board surface (the red path).
Red before the fix: every claimed case publishes the camera only.

Dry run by default; `--apply` writes; `--revert` restores (every anchor asserted exactly once).
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
RENDERER = ENGINE / "🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
INPUT_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs"
BOARD_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs"

EDITS = {
    RENDERER: [
        (
            """        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        let target = match self.pointer_capture.holder(pointer_id) {""",
            """        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        self.shell.presence_pointer = Some((x, y));
        let target = match self.pointer_capture.holder(pointer_id) {""",
        ),
    ],
    INPUT_LAWS: [
        (
            "\nfn pointer_interaction(shell: ShellState, input: InputState<ActionDescriptor>) -> crate::AppInteractionState {",
            "\npub(super) fn pointer_interaction(shell: ShellState, input: InputState<ActionDescriptor>) -> crate::AppInteractionState {",
        ),
    ],
    BOARD_LAWS: [
        (
            """/// 🖼️ A shell whose one board window, owned by `window_id`, was painted with `camera` at `bounds` and
/// mirrored into the pointer-state maps by the same body a presented frame runs.
fn painted_board(window_id: &str, bounds: Rect, camera: &Value) -> ShellState {""",
            """/// 🖼️ A shell whose one board window, owned by `window_id`, was painted with `camera` at `bounds` and
/// mirrored into the pointer-state maps by the same body a presented frame runs.
fn painted_board(window_id: &str, bounds: Rect, camera: &Value) -> ShellState {
    painted_board_with_input(window_id, bounds, camera).0
}

/// 🖱️ [`painted_board`] plus the input state its paint registered the board's retained hits in — what the renderer's pointer
/// routing reads to let a published board surface claim a move.
fn painted_board_with_input(window_id: &str, bounds: Rect, camera: &Value) -> (ShellState, InputState<ActionDescriptor>) {""",
        ),
        (
            """    let _input = shell_input_tests::paint_component_pointer_documents(&mut shell, &[(window_id, "s.test.board", &document, bounds)]);
    shell.sync_engine_surface_states();
    assert_eq!(shell.board2d_states.len(), 1, "the painted board registered its surface");
    shell
}""",
            """    let input = shell_input_tests::paint_component_pointer_documents(&mut shell, &[(window_id, "s.test.board", &document, bounds)]);
    shell.sync_engine_surface_states();
    assert_eq!(shell.board2d_states.len(), 1, "the painted board registered its surface");
    (shell, input)
}""",
        ),
    ],
}

LAW_APPEND = '''
/// 👥️ LAW (ticket 26/09/23 session 13, wasm32 run s13d: 293 heartbeats while the pointer crossed a painted puzzle board, 0 of them
/// with a pointer): the pointer a peer sees is the one the RENDERER receives. Every `publish` case is driven through the move path
/// the winit app and the browser worker drive (`AppInteractionState::handle_pointer_move`) over a painted board; a move the board
/// surface claims still becomes the presence pointer, so the heartbeat's window view is the fixture's expected view.
#[test]
fn a_pointer_the_renderer_routes_over_a_painted_board_is_the_presence_pointer_its_view_publishes() {
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    let mut claimed = 0;
    for case in fixture()["publish"].as_array().expect("publish cases").iter().filter(|case| case["pointer"].is_array()).map(under_own_window) {
        let (shell, input) = painted_board_with_input(case["windowId"].as_str().expect("window id"), rect(&case["bounds"]), &case["camera"]);
        let mut interaction = shell_input_tests::pointer_interaction(shell, input);
        let (x, y) = (case["pointer"][0].as_f64().expect("x") as f32, case["pointer"][1].as_f64().expect("y") as f32);
        if interaction.shell.scene_pointer_target_at(x, y, &interaction.input, &interaction.theme).is_some() {
            claimed += 1;
        }
        semio_framework_async::block_on(interaction.handle_pointer_move(ui_render::PointerId(1), x, y, false, 0, PointerModifiers::default()));
        let (views, _) = interaction.shell.board_presence_views();
        assert_eq!(views.iter().map(window_view_json).map(|view| numeric(&view)).collect::<Vec<_>>(), vec![numeric(&case["expected"])], "{}", case["id"]);
    }
    assert!(claimed >= 1, "at least one routed move is claimed by the painted board surface (the path that froze the presence pointer)");
}
'''
LAW_NAME = "a_pointer_the_renderer_routes_over_a_painted_board_is_the_presence_pointer_its_view_publishes"


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    revert = "--revert" in sys.argv
    apply = "--apply" in sys.argv or revert
    plans = []
    for path, edits in EDITS.items():
        before = path.read_text(encoding="utf-8")
        if path == BOARD_LAWS:
            if revert:
                if LAW_APPEND not in before:
                    sys.exit("law block absent — nothing to revert")
                before_law = before.replace(LAW_APPEND, "")
                after = replaced(path, before_law, [(new, old) for old, new in edits])
            else:
                if LAW_NAME in before:
                    sys.exit("law already present")
                after = replaced(path, before, edits).rstrip("\n") + "\n" + LAW_APPEND
        else:
            after = replaced(path, before, [(new, old) for old, new in edits] if revert else edits)
        plans.append((path, before, after))
    for path, before, after in plans:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'REVERTED' if revert else 'APPLIED' if apply else 'DRY RUN'}: {len(plans)} files")


if __name__ == "__main__":
    main()
