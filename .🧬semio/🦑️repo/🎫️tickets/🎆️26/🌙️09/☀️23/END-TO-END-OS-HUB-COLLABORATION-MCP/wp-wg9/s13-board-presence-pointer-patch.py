#!/usr/bin/env python3
"""👥️ WG9 session 13 — a peer's cursor never reached the other shell's board (wasm32 run s13d, puzzle2d on 7800 B3).

Measured: A's presence heartbeats carried both board views (`2d-overview`, `2d-detail`: camera + size) in 293 of 293 frames, and a
pointer in 0 of them, while Playwright moved A's pointer across its board; B painted nothing. Root cause (renderer, not guest-linked):
`Renderer::handle_pointer_move` hands a move to `ShellState::handle_pointer_move_for` — the only writer of `presence_pointer` — ONLY
when no published scene surface claims it, so the pointer the board-presence view reads (`board_presence_view(…, presence_pointer)`)
froze the moment the pointer entered a board: exactly when a peer should see it. The existing Shell law set `presence_pointer`
directly and never exercised the move path.

Fix: the renderer notes the presence pointer for every move, before the scene/chrome routing; law
`every_pointer_move_is_the_presence_pointer_even_over_a_painted_board` (board-presence Shell laws). Dry run by default; `--apply` writes.
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
RENDERER = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs"

RENDERER_EDITS = [
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
]

LAW_APPEND = '''

/// 👥️ LAW (ticket 26/09/23 session 13, wasm32 run s13d: 293 heartbeats, 0 pointers): the renderer notes the presence pointer for
/// EVERY pointer move, before a painted board (a published scene surface) claims the move — the board view's world point comes from
/// it, so a pointer over a board is exactly the one a peer must see.
#[test]
fn every_pointer_move_is_the_presence_pointer_even_over_a_painted_board() {
    let renderer = include_str!("../../../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
    let handler = renderer.split("async fn handle_pointer_move(&mut self").nth(1).expect("the renderer's pointer-move handler");
    let routing = handler.find("let target = match self.pointer_capture.holder(pointer_id)").expect("the scene/chrome routing");
    let noted = handler.find("self.shell.presence_pointer = Some((x, y));").expect("the handler notes the presence pointer");
    assert!(noted < routing, "the presence pointer is noted before a board surface claims the move");
}
'''


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    renderer_before = RENDERER.read_text(encoding="utf-8")
    laws_before = LAWS.read_text(encoding="utf-8")
    if "every_pointer_move_is_the_presence_pointer_even_over_a_painted_board" in laws_before:
        sys.exit("law already present")
    plans = [(RENDERER, renderer_before, replaced(RENDERER, renderer_before, RENDERER_EDITS)), (LAWS, laws_before, laws_before.rstrip("\n") + "\n" + LAW_APPEND)]
    for path, before, after in plans:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), path.name, path.name + " (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: {len(plans)} files")


if __name__ == "__main__":
    main()
