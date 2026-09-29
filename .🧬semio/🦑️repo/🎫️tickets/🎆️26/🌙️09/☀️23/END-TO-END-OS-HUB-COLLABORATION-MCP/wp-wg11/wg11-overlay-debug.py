#!/usr/bin/env python3
"""🔎️ WG11 session 14d: OVERLAY-ONLY runtime instrumentation for the renderer reds still unexplained after the first proof
(`isolated-2.tsv`). Every inserted line carries `[DEBUG] ` and is NEVER part of a set: `ROOT` IS the overlay, and the dev
loop runs this only while `wg11-overlay-debug.on` exists. `--write` applies; there is no revert — the next overlay sync
restores the files.
"""

import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-wg11-overlay")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"

EDITS = {
    ENGINE / "🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs": [
        (
            """                let step = drive_mounted_layout_text_one(&mut engine, window_id, ctx.atlas);
                cursor.own_work = layout_step_is_window_work(&step, window_id);
""",
            """                let step = drive_mounted_layout_text_one(&mut engine, window_id, ctx.atlas);
                if window_id.contains("projection") {
                    eprintln!("[DEBUG] layout {window_id} -> {step:?} dirty={}", engine.layout_is_dirty(window_id));
                }
                cursor.own_work = layout_step_is_window_work(&step, window_id);
""",
        ),
        (
            """                let paint = engine.frame_into_step(window_id, ui_wgpu::wgpu::geometry::Rect { x: bounds.x, y: bounds.y, w: viewport_w, h: viewport_h }, ctx.atlas, ctx.icons, Some(&mut scene_host), ctx.draw);
""",
            """                let paint = engine.frame_into_step(window_id, ui_wgpu::wgpu::geometry::Rect { x: bounds.x, y: bounds.y, w: viewport_w, h: viewport_h }, ctx.atlas, ctx.icons, Some(&mut scene_host), ctx.draw);
                if window_id.contains("projection") {
                    eprintln!("[DEBUG] paint {window_id} -> {paint:?} progress_before={progress_before:?}");
                }
""",
        )
    ],
    ENGINE / "🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs": [
        (
            """    let layout = graph_timeline_layout(bounds, &columns, &theme, label_track);""",
            """    let layout = graph_timeline_layout(bounds, &columns, &theme, label_track);
    eprintln!("[DEBUG] graph track shaped={label_track} layout={} builtin={} feature-editor={} ", layout.label_track_width, graph_timeline_label_track_width(&columns, &theme, &mut ui_wgpu::wgpu::FontAtlas::builtin()), atlas.measure_text("feature-editor", GRAPH_TIMELINE_CHIP_FONT_PX).0);""",
        )
    ],
    ENGINE / "🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs": [
        (
            """            if complete {
                break;
            }
        }
        let rows = input.staged_hits()""",
            """            if complete {
                break;
            }
        }
        eprintln!("[DEBUG] projection pane own-work opportunities={opportunities} polls={polls}");
        let rows = input.staged_hits()""",
        )
    ],
    ENGINE / "⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs": [
        (
            """    with_board_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, button.max(0) as u8, shift, ctrl_or_meta));
    board_set_pointer_inside(surface_id, true);""",
            """    let hosted = with_board_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, button.max(0) as u8, shift, ctrl_or_meta)).is_some();
    eprintln!("[DEBUG] board down {surface_id} hosted={hosted} at=({sx},{sy}) drag_active={}", board_drag_active(surface_id));
    board_set_pointer_inside(surface_id, true);""",
        )
    ],
}

def main():
    write = "--write" in sys.argv
    for key, edits in EDITS.items():
        path = Path(str(key).split("#")[0])
        source = path.read_text(encoding="utf-8")
        for old, new in edits:
            if source.count(old) != 1:
                sys.exit(f"debug anchor occurs {source.count(old)}x in {path.parent.name}: {old[:80]!r}")
            source = source.replace(old, new)
        if write:
            path.write_text(source, encoding="utf-8")
    print(f"{'WRITTEN' if write else 'DRY RUN'}: overlay debug lines in {len(EDITS)} files")


if __name__ == "__main__":
    main()
