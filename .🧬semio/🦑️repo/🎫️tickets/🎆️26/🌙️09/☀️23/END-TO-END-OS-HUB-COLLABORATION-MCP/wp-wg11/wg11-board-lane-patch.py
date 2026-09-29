#!/usr/bin/env python3
"""🖐️ WG11 session 14d (T6, independent): a board's pointer lane is in flight from the PRESS, not from the drag threshold.

Measured (overlay build 6, `[DEBUG] board down scene.2.1 hosted=true at=(300,300) drag_active=false`): a press on the board's backdrop
opens `Interaction::SelectionPending` (the click-vs-marquee threshold), and the renderer's `board_drag_active` — the predicate that
keeps routing the lane's release to the host when it lands OUTSIDE the surface (its twin `tiled_map_drag_active` is true from the
map's press) — only knew `Selection`. A press released off-surface before the threshold left the host pending forever, and the
two-touch law (`board_and_map_two_touch_gestures_share_camera_math_but_keep_distinct_transfer_rules`) saw no lane to transfer.

1. `♾️infinite/🎲️board` `BoardHost::pointer_lane_in_flight`: every lane a press opens — a pending or live area select, or any
   gesture `defers_descriptor_sync_from_js` names — until its release. `is_dragging_area_select` keeps its own meaning (the LIVE
   marquee a descriptor round-trip must not fight).
2. `⚙️EngineCanvas` `board_drag_active` asks the host for that lane.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/board-lane/` and applies; `--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
BOARD = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs"
CANVAS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/board-lane"

EDITS = {
    BOARD: [
        (
            '''        pub fn is_dragging_area_select(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }
''',
            '''        pub fn is_dragging_area_select(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }

        /// 🖐️ True from the press that opens a pointer lane — a pending or live area select, or any gesture
        /// [`Self::defers_descriptor_sync_from_js`] names — until its release, so a host keeps routing that release to the board
        /// even when it lands outside the surface.
        pub fn pointer_lane_in_flight(&self) -> bool {
            self.defers_descriptor_sync_from_js() || matches!(&self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. })
        }
''',
        )
    ],
    CANVAS: [
        (
            '''/// 🖐️ True while a node drag or area-select gesture is in flight, so pointer-up outside the surface bounds still reaches the host (mirrors `tiled_map_drag_active`).
pub fn board_drag_active(surface_id: &str) -> bool {
    with_board_host(surface_id, |host| host.defers_descriptor_sync_from_js() || host.is_dragging_area_select()).unwrap_or(false)
}''',
            '''/// 🖐️ True from the press that opens a board pointer lane (a pending area select included) until its release, so pointer-up
/// outside the surface bounds still reaches the host (mirrors `tiled_map_drag_active`, which is live from the map's press).
pub fn board_drag_active(surface_id: &str) -> bool {
    with_board_host(surface_id, infinite_canvas::BoardHost::pointer_lane_in_flight).unwrap_or(false)
}''',
        )
    ],
}


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def main():
    if "--revert" in sys.argv:
        for path in EDITS:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: every file restored from its backup")
        return
    write = "--write" in sys.argv
    planned = [(path, path.read_text(encoding="utf-8")) for path in EDITS]
    planned = [(path, before, replaced(path, before, EDITS[path])) for path, before in planned]
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-os-infinite (board), semio-framework-os-renderer-wgpu)")


if __name__ == "__main__":
    main()
