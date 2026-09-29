#!/usr/bin/env python3
"""🪟️ WG11 session 14d — set for the first train after the chain (T6), AFTER `wg11-tree-window-patch.py`: the wgpu Shell observes and
schedules tree windows (ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING packet P5, the host half).

1. `🐚️Shell/🎯️targets/🧊️wgpu/🪟️tree-windows/🦀️.rs` (new): `TreeWindowScheduler` — the Rust twin of React's
   `createTreeWindowSchedulerV1` (`🛠️ShellHelpers/🟦️.tsx`: trailing debounce, one refresh in flight per body, re-send on
   settle, never-measured containers ask one viewport, unsendable paths dropped) — and the observer: every retained body
   that finishes a paint is measured (`Ui::tree_window_measures`), served (`tree_window::served_requests`) and reported.
2. `clear_document_paint_fault` becomes `note_retained_body_painted` (the fault clears AND the body is observed) at every
   completion site; the settle pump drives the scheduler (`pump_tree_windows`: a guest body refreshes through the owed
   partial scope, a shell-owned panel republishes); `live_view_state` stamps `tree_windows`/`tree_viewport_rows` — the GAP
   note it carried is closed.
3. `interpreter::tree_window_measures` reads the shared engine.
Laws (`🐚️Shell/🧪️tests/🪟️tree-windows/🦀️.rs`): the scheduler's coalescing / single-flight / open / unsendable rules.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/tree-window-shell/` (a new file's
backup is its absence) and applies; `--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/p5")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
SHELL = ENGINE / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
INTERPRETER = ENGINE / "🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
OBSERVER = ENGINE / "🐚️Shell/🎯️targets/🧊️wgpu/🪟️tree-windows/🦀️.rs"
OBSERVER_LAWS = ENGINE / "🐚️Shell/🧪️tests/🪟️tree-windows/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/tree-window-shell"

NEW_FILES = {
    OBSERVER: (WORK / "shell-tree-windows.rs").read_text(encoding="utf-8"),
    OBSERVER_LAWS: (WORK / "shell-tree-windows-tests.rs").read_text(encoding="utf-8"),
}

SHELL_EDITS = [
    (
        '''#[path = "📤️icon-export/🦀️.rs"]
mod icon_export;
''',
        '''#[path = "📤️icon-export/🦀️.rs"]
mod icon_export;

#[path = "🪟️tree-windows/🦀️.rs"]
mod tree_windows;
''',
    ),
    (
        '''    dock_input_identity: Option<DockInputIdentity>,
''',
        '''    dock_input_identity: Option<DockInputIdentity>,
    /// 🪟️ Host-owned tree windows of every body this shell paints — what crosses as `ViewModel.tree_windows`.
    tree_windows: tree_windows::TreeWindowScheduler,
    /// 🧠️ Per body, what the observer learned from the guest's answers (`TreeWindowServedMemoryV1`).
    tree_window_served: HashMap<String, Vec<(String, ui_wgpu::wgpu::tree_window::TreeWindowServedMemory)>>,
''',
    ),
    (
        '''            dock_input_identity: None,
''',
        '''            dock_input_identity: None,
            tree_windows: tree_windows::TreeWindowScheduler::default(),
            tree_window_served: HashMap::new(),
''',
    ),
    (
        '''    fn clear_document_paint_fault(&mut self, surface_id: &str) {
        if self.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            self.surface_faults.retain(|fault| fault.surface_id != surface_id);
            self.error = self.fault_status();
        }
    }''',
        '''    /// 🪟️ One retained body finished a paint: its paint fault (if any) clears, and its windowed tree containers are measured and
    /// reported ([`Self::observe_tree_windows`]).
    fn note_retained_body_painted(&mut self, surface_id: &str) {
        if self.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            self.surface_faults.retain(|fault| fault.surface_id != surface_id);
            self.error = self.fault_status();
        }
        self.observe_tree_windows(surface_id);
    }''',
    ),
    (
        '''    /// 🪟️ The host-owned fields every refresh restamps onto the session's view state.
    ///
    /// ⚠️ GAP (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §2.4, §5): `tree_windows` and
    /// `tree_viewport_rows` are NOT stamped here, because this shell mounts no tree viewport
    /// observer. React's `🗣️Interpreter` reports one `TreeWindowRequest` per open container through
    /// `TreeWindowHostV1::viewStateFields` (`🛠️ShellHelpers/🟦️.tsx`); until a wgpu scroll/open
    /// observer feeds the same pair, every construction site sends what React sends before its first
    /// report — no requests and no measured viewport — so a guest paints its own first-paint window
    /// and scrolling into a spacer band reveals pitch rather than streamed rows.
    fn live_view_state(&self, session: &ActiveSession) -> ViewModel {''',
        '''    /// 🪟️ The host-owned fields every refresh restamps onto the session's view state — including the tree windows this shell's
    /// observer measured (`tree_windows`/`tree_viewport_rows`, [`tree_windows::TreeWindowScheduler::view_state_fields`]), the
    /// same pair React's `TreeWindowHostV1::viewStateFields` stamps (`🛠️ShellHelpers/🟦️.tsx`).
    fn live_view_state(&self, session: &ActiveSession) -> ViewModel {''',
    ),
    (
        '''        view_state.session_identity = self.session_identity_view();
''',
        '''        view_state.session_identity = self.session_identity_view();
        (view_state.tree_windows, view_state.tree_viewport_rows) = self.tree_windows.view_state_fields();
''',
    ),
    (
        '''    pub async fn settle_pump_step(&mut self) -> ShellSettleStep {
        self.advance_icon_export();
''',
        '''    pub async fn settle_pump_step(&mut self) -> ShellSettleStep {
        self.advance_icon_export();
        self.pump_tree_windows();
''',
    ),
]

INTERPRETER_EDITS = [
    (
        '''pub fn retained_scene_target(window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {''',
        '''/// 🪟️ Every windowed tree container the retained surface `window_id` presents, measured against its scroll viewport
/// (`Ui::tree_window_measures`) — the Shell's tree window observer reads the shared engine through this.
pub(crate) fn tree_window_measures(window_id: &str) -> Option<(f64, Vec<ui_wgpu::wgpu::tree_window::TreeWindowContainerMeasure>)> {
    UI_ENGINE.with(|cell| cell.borrow().tree_window_measures(window_id))
}

pub fn retained_scene_target(window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {''',
    )
]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def shell_after(source: str) -> str:
    source = replaced(SHELL, source, SHELL_EDITS)
    calls = source.count("self.clear_document_paint_fault(")
    if calls == 0:
        sys.exit("no paint-completion site calls clear_document_paint_fault any more — re-measure")
    return source.replace("self.clear_document_paint_fault(", "self.note_retained_body_painted(")


def plans():
    for path in NEW_FILES:
        if path.exists():
            sys.exit(f"{path.relative_to(ROOT)} exists already — landed already")
    shell = SHELL.read_text(encoding="utf-8")
    interpreter = INTERPRETER.read_text(encoding="utf-8")
    return [(path, None, content) for path, content in NEW_FILES.items()] + [
        (SHELL, shell, shell_after(shell)),
        (INTERPRETER, interpreter, replaced(INTERPRETER, interpreter, INTERPRETER_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in NEW_FILES:
            if path.exists():
                path.unlink()
        for path in (SHELL, INTERPRETER):
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: new files removed, edited files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff((before or "").splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=0))
    if write:
        for path, before, _ in planned:
            if before is not None:
                backup = BACKUP / path.relative_to(ROOT)
                backup.parent.mkdir(parents=True, exist_ok=True)
                backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(NEW_FILES)} new + 2 edited files (crate: semio-framework-os-renderer-wgpu)")


if __name__ == "__main__":
    main()
