# w2-dock-dnd — final report

**File touched (only one):** `framework/renderer/wgpu/rs/lib.rs`, exclusively inside the `dock` region (spanning lines 11-2578; `DockTypes` 29-107, `DockLayout` 109-618, `DockFreeFunctions` 620-1808, `DockTests` 1810-2577). No other file touched.

## What was found first (re-grep + reference diffing)

The brief's premise — `ShellEvent::PanelDragStarted`/`PanelDropped` "defined but never emitted" — doesn't match the current file: those types don't exist anywhere in the codebase. What actually exists (likely from earlier work not reflected in the stale gap analysis) is a fully-built-looking parallel mechanism: `DockDragPayload`/`DockDragState`/`DockDropZone`/`compute_dock_drop_zone` in `dock`, driven by eager-removal-on-promotion + snapshot-based Escape-revert in `shell`. It looks complete and sophisticated — but tracing the actual commit path (`ShellState::finish_dock_drag` → `DockState::apply_drop`) turned up a real, previously-untested bug that made it non-functional:

**The bug:** `shell`'s `handle_pointer_move` removes `drag.window_id` from the tree eagerly the moment a drag is promoted (5px threshold). But `apply_drop` (in `dock`) then tried to `remove_window(&drag.window_id)` again before inserting at the drop zone. Since the window was already gone, that second removal always failed — silently no-opping every cross-stack/side drop, or for same-stack drops, reordering whichever different window had shifted into the stale `tab_index` slot. Zero test coverage on `apply_drop`/`DockDragPayload`/`DockDropZone` existed before this session.

Compared against the React reference (`ui/js/react/index.tsx`'s `ModeDropZone`/`computeModeDropZone`/`applyModeDrop`/`extractStackFromLayout`/`splitWithStack`/`mergeStackTabsIntoStack` family, `#region 🧭️ModeDockDrag`/`🧭️ModeLayoutUtils`) — this is the actual sibling algorithm (the `PanelDock` 6-anchor system in `os-shell.tsx` the original brief pointed at is a different, unrelated panel system; the file's own top-of-file doc comment already says `dock` corresponds to React's `Mode` component, not `PanelDock`).

## 1. Drop-zone geometry (already correct, now covered by tests)

`compute_dock_drop_zone(pointer_x, pointer_y, tab_bars, bodies, canvas)` already matches `computeModeDropZone` exactly: tab-bar rects checked first (→ `Tab{stack_path, index}` via nearest-tab-midpoint), then body rects (→ `Split{stack_path, side}` via `resolve_split_side`'s dominant-axis-from-center test, byte-identical to React's `resolveModeSplitSideInBody`), then canvas fallback (→ `RootSplit{side}`). No change needed — added `compute_dock_drop_zone_prefers_tab_bar_over_body_over_root` and `resolve_split_side_uses_dominant_axis_from_center` tests to pin it.

## 2. Drag-and-drop fix + whole-stack move (the real gap-1 work)

- **Fixed `apply_drop`**: removed the erroneous second `remove_window` call from every branch; it now only inserts at the resolved zone, since the window is already absent by the time it runs.
- **Implemented `DockDragKind::Stack` (whole-stack drag)** for real: previously the payload carried a `Stack` kind but `apply_drop` treated it identically to `Tab` (moved only the active window, silently abandoning the rest of the stack). New `extract_stack_group` reconstructs the full original tab order (siblings left behind by the eager single-window removal + the already-removed primary re-inserted at its original index) and moves them together via new `insert_tabs`/`split_stack_with_stack`/`split_root_with_stack` (the existing single-window methods now delegate to these for DRY).
- **Found and fixed a subtler staleness bug**: extracting a stack's remaining siblings can collapse-and-reindex the tree, stranding the pre-extraction `zone.stack_path`. Mirrored React's `targetAnchorId`/`resolveStackPathForWindowId` pattern: capture the target stack's current active window id before extraction, re-resolve its path by that key (`find_stack_path`) afterward, falling back to the raw path only if resolution fails.
- 7 new tests: cross-stack tab move, same-stack reinsert-at-new-index, split re-targeting after prune, root-split, whole-stack move preserving order, whole-stack split with target re-anchoring after a real induced shift, and same-source no-op.

## 3. `ui_wgpu::events::DragSession` wiring — not yet available, used documented fallback

Checked `ui/wgpu/rs/lib.rs`'s `re-exports` region: currently only exports `CaptureKind, EventModifiers, PointerButton, UiCommand, UiEvent`. `DragSession`/`DragPayload`/`set_drag_payload`/`set_drop_accept`/`maybe_promote_to_drag` are still `pub(crate)` inside `ui_wgpu::events` — the `w2-ui-wgpu-integration` re-export pass hadn't landed yet at time of writing. Per the brief's fallback instruction, did not build a redundant parallel drag mechanism; `dock`'s existing bespoke one (`DockDragPayload`/`DockDragState`, shell-owned lifecycle) already does the job once its bug is fixed, and it has genuinely different geometry semantics (4-side split + tab-index insert vs. `DragSession`'s generic nearest-accepting-ancestor model) — migrating it onto `DragSession` later is a real adaptation, not a drop-in swap, out of scope here.

## 4. Keyed diff (`DockState::apply_layout_diff`)

The actual full-teardown call site (`self.dock.root = dock_from_window_layout(&layout.root)` / `self.dock = DockState::from_app(...)`) lives in `ShellState::sync_dock`, in the do-not-touch `shell` region — so the complete, tested replacement was implemented as a `dock`-owned method ready to be wired in:

- `diff_dock_node(old, next)` / `diff_axis_children`: recursively diffs the incoming tree by structure — identical `Stack` nodes reused wholesale; a `Stack` merely reordered (same window set) keeps its OLD `active` tab focused rather than reverting to whatever the incoming layout says (a real, demonstrable bug in the current teardown: a persisted snapshot's stale `active_window_kind_id` would silently steal focus back from whatever the user is currently looking at); axis children paired by index; a structural-kind change adopts the incoming shape.
- `DockState::apply_layout_diff(&mut self, layout)`: applies the above, then re-anchors `active_stack`/`maximized_stack` by window-id key (`find_stack_path`) instead of reusing them as stale positional `DockPath`s — today's teardown keeps those paths as-is across a root swap, silently misdirecting or invalidating them the moment the new tree's shape differs even slightly. Also clears `split_resize_origin` (an in-flight resize gesture's indices are meaningless after a layout swap).
- 4 new tests: focus survives a reorder despite a stale persisted `active`, `active_stack`/`maximized_stack` follow their window by key across a reversed layout, `maximized_stack` clears when its window disappears, and `diff_dock_node` reuse/adopt behavior directly.

**Wiring request for whoever next owns `shell`:** in `sync_dock` (line ~11094 pre-edit region), replace the `layout_override` branch's `self.dock.root = dock_from_window_layout(&layout.root); self.dock.active_window_id = ...` with `self.dock.apply_layout_diff(&layout);` — one line, fully written and traced, pending live test confirmation (see below).

## Build/verify

- `cargo check -p semio-framework-renderer-wgpu --lib`: clean every time it was run (last: "Finished `dev` profile ... in 3.35s"). Only pre-existing warnings (dead code in unrelated `scenes`/`shell` regions), none touching `dock`.
- `cargo test -p semio-framework-renderer-wgpu --lib`: **could not complete** — blocked by a live, unrelated, concurrent-session breakage in `infinite_world` (a real dependency of this crate, being worked on concurrently by `w2-world3d` in this same wave). `git status` confirmed `infinite/world/rs/{Cargo.toml,lib.rs}` were mid-edit (uncommitted); two consecutive attempts a few minutes apart hit different errors there, proving active concurrent editing. Unrelated to `dock` — never touched `infinite_world` or anything outside `lib.rs`'s `dock` region.

**UPDATE — now fully green.** Once `infinite_world` contention cleared, `cargo test -p semio-framework-renderer-wgpu --lib dock::` was re-run and surfaced one genuine failure: `apply_drop_tab_moves_window_across_stacks` panicked "cross-stack tab drop must actually land". Root cause and fix:

**Root cause:** the TEST FIXTURE, not `apply_drop`, was wrong. `dock.remove_window("a")` on a 2-stack row where `a` is the sole occupant of stack `[0]` prunes that now-empty stack via `collapse_empty` — the sibling at `[1]` shifts down to `[0]`. The original test set `dock.root = Row[Stack{a}, Stack{b}]` then dropped onto `zone.stack_path: vec![1]` — a path that no longer existed after removal (only one child left), so `insert_tab` correctly returned `false` on an out-of-bounds path and `apply_drop` correctly propagated that `false`. In the real runtime this can't happen because `compute_dock_drop_zone` re-derives `stack_path` from the live tree on every pointer-move frame, so it would never hand `apply_drop` a stale path.

**Fix:** rewrote the test with three stacks (`a`, `b`, `c`) so removing `a` prunes only its own slot without touching the actual drop target — `c` shifts from `[2]` to `[1]`, and the zone now correctly targets `c`'s post-removal path `[1]` (exactly as a live drag would have resolved it), with an explicit assertion pinning that shift before the drop. `apply_drop`'s own logic (the double-removal fix) required no changes — confirms the original fix was correct, only the test fixture was wrong.

**Final verification**: `cargo test -p semio-framework-renderer-wgpu --lib dock::` — **33 passed, 0 failed.**

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `dock` region only (`DockLayout`: `apply_drop` rewrite + `extract_stack_group`/`insert_tabs`/`split_stack_with_stack`/`split_root_with_stack`/`apply_layout_diff`; `DockFreeFunctions`: region-marker fix + `diff_dock_node`/`diff_axis_children`; `DockTests`: 13 new tests + 2 payload-builder helpers).
