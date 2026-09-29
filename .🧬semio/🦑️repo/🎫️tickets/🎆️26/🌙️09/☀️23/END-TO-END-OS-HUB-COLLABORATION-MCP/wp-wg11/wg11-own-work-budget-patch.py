#!/usr/bin/env python3
"""⏱️ WG11 session 14d — set for T6 (independent of the other WG11 sets; anchors avoid the completion call tree-window-shell
renames): a retained document's paint budget counts only the document's OWN work (coordinator 09:0x, open design item 1).

Measured (overlay build 4, `an_unfolded_projection_pane_publishes_its_rows_within_one_frame_budget`): the retained layout runs as a
worker-pool session, and the chrome walk polls it — 712 layout yields for the pane's two layouts, ~600 of them waits
(`Layout.WorkerPool.UserVisible` spins, then take / outcome / close). Every poll spent one of the pane's 1 024 "non-convergence"
opportunities, so under load a CONVERGING pane faulted (`record_document_paint_fault`, the pane paints nothing). The same counter
also paid for steps that advanced ANOTHER surface's layout (`step_layouts` serves the lane wheel, not the asking window).

1. `ui::engine::UiLayoutStep::Awaiting { window_id, lane }` — a layout whose step is on a pool worker and has not answered yet; the
   engine no longer dresses a wait as a `Yielded` stage.
2. `UiDocumentFrameCursor::last_step_was_own_work` — false for a step that only ran the shared retirement lanes, only waited on the
   pool, or advanced another surface's layout; every other phase step (a layout step on this surface, an `Idle` stall, and a wait
   on this document's OWN predecessor close, so a wedged close still faults) is the document's own work.
3. The Shell's four retained-document paint budgets (window bodies, window measures, Actions/Search panes, the Projection pane) spend
   an opportunity only on own work — ONE helper `document_opportunity_remains`; a body that stops converging still faults. The
   Projection pane's ceiling was derived without the layout's worker round trips: its converging body needs 1 405 own
   opportunities (overlay build 7: two layout passes of ~240 — outcome take + apply per job step, node collection, session close —
   and ~800 paint grants) against 1 024 — now 4 096.
Laws: the one-frame-body law counts the pane's own opportunities (and bounds its polls); the fold law polls until the body completes.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/own-work-budget/` and applies;
`--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE_UI = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs"
ELEMENTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
INTERPRETER = ELEMENTS / "🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL = ELEMENTS / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
NAVBAR_LAWS = ELEMENTS / "🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/own-work-budget"

ENGINE_EDITS = [
    (
        '''/// 🧭️Observable result of exactly one bounded surface-layout scheduling call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiLayoutStep {
    Idle,
    Yielded { window_id: SurfaceId, lane: SurfaceLane, stage: &'static str, nodes: usize, glyphs: usize },''',
        '''/// 🧭️Observable result of exactly one bounded surface-layout scheduling call. `Awaiting`: `window_id`'s layout step runs on a
/// pool worker and has not answered yet — nothing ran on this thread.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiLayoutStep {
    Idle,
    Yielded { window_id: SurfaceId, lane: SurfaceLane, stage: &'static str, nodes: usize, glyphs: usize },
    Awaiting { window_id: SurfaceId, lane: SurfaceLane },''',
    ),
    (
        '''            let poll = session.pump_one(pool, worker_lane(lane));
            self.enqueue_layout(window_id.as_ref());
            return UiLayoutStep::Yielded {
                window_id,
                lane,
                stage: if matches!(poll, Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) { "Layout.WorkerTake" } else { "Layout.WorkerPool.UserVisible" },
                nodes: 0,
                glyphs: 0,
            };''',
        '''            let poll = session.pump_one(pool, worker_lane(lane));
            self.enqueue_layout(window_id.as_ref());
            return match poll {
                Ok(semio_framework_job::WorkerJobPoll::Submitted) => UiLayoutStep::Awaiting { window_id, lane },
                Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerTake", nodes: 0, glyphs: 0 },
                _ => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerPool.UserVisible", nodes: 0, glyphs: 0 },
            };''',
    ),
]

INTERPRETER_EDITS = [
    (
        '''#[derive(Default)]
pub struct UiDocumentFrameCursor {
    phase: UiDocumentFramePhase,
    /// 🩺️ Consecutive `Pending` paint opportunities, reset whenever the phase changes.
    stalled: u32,
}

impl UiDocumentFrameCursor {''',
        '''#[derive(Default)]
pub struct UiDocumentFrameCursor {
    phase: UiDocumentFramePhase,
    /// 🩺️ Consecutive `Pending` paint opportunities, reset whenever the phase changes.
    stalled: u32,
    /// ⏱️ Whether the last step advanced THIS document's own ladder ([`Self::last_step_was_own_work`]).
    own_work: bool,
}

impl UiDocumentFrameCursor {
    /// ⏱️ Whether the last step was this document's own work — false when it only ran the shared retirement lanes, only waited on
    /// the layout worker pool ([`ui_wgpu::wgpu::UiLayoutStep::Awaiting`]) or advanced another surface's layout. A host's
    /// non-convergence budget spends an opportunity only on own work, so a slow worker or a busy neighbour never faults a document
    /// that is converging.
    pub fn last_step_was_own_work(&self) -> bool {
        self.own_work
    }
''',
    ),
    (
        '''    driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    hosts: &mut crate::scenes::SceneEngineHosts<'_>,
) -> bool {
    if close_retiring_focus_clipboard_one() {
        return false;
    }''',
        '''    driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    hosts: &mut crate::scenes::SceneEngineHosts<'_>,
) -> bool {
    cursor.own_work = false;
    if close_retiring_focus_clipboard_one() {
        return false;
    }''',
    ),
    (
        '''    if !forward_retired_component_scene_one(window_id) {
        return false;
    }
    if ui_document_close_pending_for(window_id) {
        return false;
    }''',
        '''    if !forward_retired_component_scene_one(window_id) {
        return false;
    }
    if ui_document_close_pending_for(window_id) {
        cursor.own_work = true;
        return false;
    }''',
    ),
    (
        '''    let viewport_w = bounds.w.max(1.0);
    let viewport_h = bounds.h.max(1.0);
    UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        engine.set_driver_drag(driver_drag);
        match cursor.phase {''',
        '''    let viewport_w = bounds.w.max(1.0);
    let viewport_h = bounds.h.max(1.0);
    cursor.own_work = !matches!(cursor.phase, UiDocumentFramePhase::Layout);
    UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        engine.set_driver_drag(driver_drag);
        match cursor.phase {''',
    ),
    (
        '''            UiDocumentFramePhase::Layout => {
                let _ = drive_mounted_layout_text_one(&mut engine, window_id, ctx.atlas);
                if engine.layout_is_dirty(window_id) {
                    engine.request_layout(window_id);
                } else {
                    cursor.phase = UiDocumentFramePhase::Paint;
                }
            }''',
        '''            UiDocumentFramePhase::Layout => {
                let step = drive_mounted_layout_text_one(&mut engine, window_id, ctx.atlas);
                cursor.own_work = layout_step_is_window_work(&step, window_id);
                if engine.layout_is_dirty(window_id) {
                    engine.request_layout(window_id);
                } else {
                    cursor.phase = UiDocumentFramePhase::Paint;
                    cursor.own_work = true;
                }
            }''',
    ),
    (
        '''fn drive_mounted_layout_text_one(engine: &mut ui_wgpu::wgpu::Ui, window_id: &str, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> ui_wgpu::wgpu::UiLayoutStep {''',
        '''/// ⏱️ Whether one layout scheduling call was `window_id`'s own work: a step of its own job (or an `Idle` stall of a dirty layout)
/// is; a wait on the worker pool or a step of another surface's job is not.
fn layout_step_is_window_work(step: &ui_wgpu::wgpu::UiLayoutStep, window_id: &str) -> bool {
    match step {
        ui_wgpu::wgpu::UiLayoutStep::Awaiting { .. } => false,
        ui_wgpu::wgpu::UiLayoutStep::Idle => true,
        ui_wgpu::wgpu::UiLayoutStep::Yielded { window_id: stepped, .. } | ui_wgpu::wgpu::UiLayoutStep::Ready { window_id: stepped, .. } | ui_wgpu::wgpu::UiLayoutStep::Cancelled { window_id: stepped, .. } => AsRef::<str>::as_ref(stepped) == window_id,
    }
}

fn drive_mounted_layout_text_one(engine: &mut ui_wgpu::wgpu::Ui, window_id: &str, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> ui_wgpu::wgpu::UiLayoutStep {''',
    ),
]

SHELL_EDITS = [
    (
        '''const SHELL_WINDOW_PAINT_OPPORTUNITIES: usize = 1 << 20;
''',
        '''const SHELL_WINDOW_PAINT_OPPORTUNITIES: usize = 1 << 20;

/// ⏱️ Spends one of a retained document's paint opportunities — only when the step was the document's OWN work
/// (`UiDocumentFrameCursor::last_step_was_own_work`: waiting on the layout worker pool or on another surface's layout costs
/// nothing) — and answers whether the document may keep stepping under `budget`.
fn document_opportunity_remains(cursor: &mut ShellChromeChildCursor, budget: usize) -> bool {
    if cursor.document.last_step_was_own_work() {
        cursor.scalar = cursor.scalar.saturating_add(1);
    }
    !cursor.document.terminal_is_fault() && cursor.scalar < budget
}
''',
    ),
    (
        '''        } else {
            cursor.scalar = cursor.scalar.saturating_add(1);
            if !cursor.document.terminal_is_fault() && cursor.scalar < SHELL_WINDOW_PAINT_OPPORTUNITIES {
                return false;
            }''',
        '''        } else {
            if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                return false;
            }''',
        2,
    ),
    (
        '''        } else {
            cursor.scalar = cursor.scalar.saturating_add(1);
            if !cursor.document.terminal_is_fault() && cursor.scalar < WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES {
                return false;
            }''',
        '''        } else {
            if document_opportunity_remains(cursor, WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES) {
                return false;
            }''',
    ),
    (
        '''                if !complete {
                    cursor.scalar = cursor.scalar.saturating_add(1);
                    if !cursor.document.terminal_is_fault() && cursor.scalar < SHELL_WINDOW_PAINT_OPPORTUNITIES {
                        return false;
                    }''',
        '''                if !complete {
                    if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                        return false;
                    }''',
    ),
    (
        '''/// ⏱️ The ceiling one unfolded projection body may spend inside the chrome walk — its own paint
/// opportunities, not the whole walk's. Fifteen rows of at most nine group phases and one glyph
/// grant per label scalar fit inside it several times over; a body that reaches it has stopped
/// converging, which is a fault, not a slow frame.
pub(crate) const WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES: usize = 1024;''',
        '''/// ⏱️ The ceiling one unfolded projection body may spend inside the chrome walk — its own paint opportunities, not the whole
/// walk's, and only its own WORK (`document_opportunity_remains`): polls of the layout worker pool and steps of other surfaces'
/// layouts are not the pane's. The pane's content is React's fixed taxonomy (18 retained nodes), and its own work is priced per
/// grant: two layout passes (the pane re-solves once it hugs its measured content) of ~240 own steps each — every worker job
/// step costs two opportunities on this thread (its outcome is taken, then applied), plus node collection and session close —
/// and ~800 paint grants of one glyph or node phase each: 1 405 in WG11's overlay build 7, where a 1 024 ceiling faulted the
/// converging pane at paint progress 418. Nearly three times that fits; a body that reaches it has stopped converging, which is
/// a fault, not a slow frame.
pub(crate) const WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES: usize = 4096;''',
    ),
]

NAVBAR_LAW_EDITS = [
    (
        '''        let mut cursor = ShellChromeChildCursor::default();
        let mut overlay = DrawList::default();
        let mut overlay = Some(&mut overlay);
        let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
        for _ in 0..8192 {
            if shell.paint_window_projection_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, "pane-top", window, &mut world_resources) {''',
        '''        let mut cursor = ShellChromeChildCursor::default();
        let mut overlay = DrawList::default();
        let mut overlay = Some(&mut overlay);
        let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
        for _ in 0..1_usize << 24 {
            if shell.paint_window_projection_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, "pane-top", window, &mut world_resources) {''',
    ),
    (
        '''/// run under [`WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES`].
///''',
        '''/// run under [`WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES`] of the pane's OWN work — a poll of the layout worker pool is not one
/// (it spent the pane's whole budget under load while the pane was converging).
///''',
    ),
    (
        '''        let mut opportunities = 0_usize;
        while opportunities < WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES {
            opportunities += 1;
            if shell.paint_window_projection_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, "pane-top", window, &mut world_resources) {
                break;
            }
        }''',
        '''        let (mut opportunities, mut polls) = (0_usize, 0_usize);
        while opportunities < WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES {
            polls += 1;
            assert!(polls < 1 << 24, "⏱️ the pane's layout worker answers");
            let complete = shell.paint_window_projection_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, "pane-top", window, &mut world_resources);
            if cursor.document.last_step_was_own_work() {
                opportunities += 1;
            }
            if complete {
                break;
            }
        }''',
    ),
]

EDITS = {ENGINE_UI: ENGINE_EDITS, INTERPRETER: INTERPRETER_EDITS, SHELL: SHELL_EDITS, NAVBAR_LAWS: NAVBAR_LAW_EDITS}


def replaced(path: Path, source: str, edits) -> str:
    for edit in edits:
        old, new, expected = (*edit, 1) if len(edit) == 2 else edit
        count = source.count(old)
        if count != expected:
            sys.exit(f"anchor occurs {count}x (expected {expected}) in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def main():
    if "--revert" in sys.argv:
        for path in EDITS:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(EDITS)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = []
    for path, edits in EDITS.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-ui, semio-framework-os-renderer-wgpu)")


if __name__ == "__main__":
    main()
