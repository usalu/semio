---
name: Fix wgpu Focus maximize and tab drag
overview: "Fix two confirmed regressions in the wgpu renderer's dock/window system: Focus does not resize the window to fill the dock area, and dragging a window tab does nothing."
todos: []
isProject: false
---

# Fix wgpu Focus Maximize and Window Tab Drag

## Bug 1: Focus does not fill the window space

**Root cause:** When a stack is maximized, three call sites in [framework/renderer/wgpu/rs/dock.rs](framework/renderer/wgpu/rs/dock.rs) compute the maximized stack's render rect with `solve_node_bounds(&self.root, bounds, path, &[])`, which walks the split tree and returns the stack's **original split-pane slot** (e.g. 33% width in a 3-column row), not the full dock `bounds`. In the React renderer (`ui/js/react/index.tsx`, `toggleMaximize` + the `maximizedStack` render branch around line 21242), maximizing instead renders only the target stack and gives it the **entire mode body rect** — so windows in a multi-pane layout truly go full-size, while in wgpu they stay pinned to their original slot with the rest of the canvas just painted over.

Affected call sites, all following the same pattern:

- `DockState::register_hits` — [framework/renderer/wgpu/rs/dock.rs:446-456](framework/renderer/wgpu/rs/dock.rs)
- `DockState::stack_body_rects` — [framework/renderer/wgpu/rs/dock.rs:243-265](framework/renderer/wgpu/rs/dock.rs)
- `DockState::stack_tab_bar_rects` — [framework/renderer/wgpu/rs/dock.rs:279-291](framework/renderer/wgpu/rs/dock.rs)

**Fix:** In all three maximized-branch call sites, use the full `bounds` directly for the target stack instead of resolving its original slot via `solve_node_bounds`. Concretely, replace:

```rust
if let (Some(node), Some(rect)) = (
    node_at(&self.root, path),
    solve_node_bounds(&self.root, bounds, path, &[]),
) {
```

with:

```rust
if let Some(node) = node_at(&self.root, path) {
    let rect = bounds;
```

(adjusting each call site's surrounding code to the no-longer-`Option` `rect`). This makes the maximized stack occupy the whole dock canvas area, matching React.

## Bug 2: Window tab drag does nothing

**Root cause:** In [framework/renderer/wgpu/rs/shell.rs](framework/renderer/wgpu/rs/shell.rs), pointer-down dispatch calls `handle_shell_hit` first (line 992), and only falls through to the drag-init block for `dock.tab.*` hits (lines 995-1023, which calls `begin_pending_dock_drag`) if `handle_shell_hit` returns `Ok(false)`. But `handle_shell_hit` has its own arm for the same prefix that unconditionally returns `Ok(true)`:

```rust
id if id.starts_with("dock.tab.") => {
    return Ok(true);
}
```

([framework/renderer/wgpu/rs/shell.rs:1545-1547](framework/renderer/wgpu/rs/shell.rs))

Because this arm reports "handled" without doing anything, the caller returns immediately at line 993 and the real drag-init logic at lines 995-1023 is dead code — `begin_pending_dock_drag` is never invoked for tab pointer-downs, so `pending_dock_drag` stays `None`, the drag-promotion threshold check in `handle_pointer_move` (lines 1171-1198) never fires, and dragging a tab is a no-operation. This also silently breaks plain tab-click activation, since activation happens on pointer-up keyed off `pending_dock_drag` being set.

**Fix:** Remove the no-operation `id if id.starts_with("dock.tab.") => { return Ok(true); }` arm from `handle_shell_hit` (the default arm already falls through to `Ok(false)`), so the caller's existing drag-init block in the pointer-down handler executes as designed.

## Secondary cleanup (same feature area)

- `promote_dock_drag()` ([framework/renderer/wgpu/rs/shell.rs:491-502](framework/renderer/wgpu/rs/shell.rs)) duplicates the inline threshold-promotion logic already present in `handle_pointer_move` (lines 1171-1185) and is never called — remove the dead function once the inline path is confirmed working, to avoid future drift between the two copies.
- No drop-zone highlight is rendered during an active tab drag today (only a floating ghost label, [shell.rs:3506-3519](framework/renderer/wgpu/rs/shell.rs)); React shows a split/tab drop indicator ([ui/js/react/index.tsx:21304-21329](ui/js/react/index.tsx)). This is a smaller polish item — add a drop-zone rect overlay using the already-computed `compute_dock_drop_zone` result while `dock_drag` is active, so users get visual feedback on where a tab will land.

## Verification

1. `cargo test -p semio-framework-renderer-wgpu` if it compiles natively for the affected modules (dock/shell unit tests); otherwise verify via `cargo build -p semio-framework-renderer-wgpu --target wasm32-unknown-unknown --release`.
2. Rebuild the wgpu renderer WASM (`bun framework/renderer/wgpu/script.ts wasm`) and reload the running dev server (`?plugin=s`, `?plugin=forms`).
3. Manual/browser verification via the `cursor-ide-browser` MCP:
   - Click Focus on a window in a multi-pane layout (e.g. `s` studio with multiple windows docked) and confirm it now fills the entire dock/canvas area, then Unfocus restores the original split layout.
   - Drag a window's tab label to reorder within a stack, to another stack, and to a split-drop-zone edge; confirm the tab moves and the layout updates, matching the already-implemented `dock.apply_drop` semantics.
   - Confirm plain tab clicks (no drag) still activate/select the tab.
4. Re-run the wgpu E2E smoke (`.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts`) for `s` and `forms` to confirm no regressions.
   </plan>
   <todos>
   <todo id="fix-focus-bounds" content="Fix DockState::register_hits, stack_body_rects, stack_tab_bar_rects in dock.rs to use full bounds instead of solve_node_bounds for the maximized stack"/>
   <todo id="fix-tab-drag" content="Remove the no-operation dock.tab.* early-return arm in handle_shell_hit (shell.rs) so begin_pending_dock_drag runs on tab pointer-down"/>
   <todo id="cleanup-dead-drag-code" content="Remove unused promote_dock_drag function and reconcile with inline threshold logic in handle_pointer_move"/>
   <todo id="drop-zone-overlay" content="Render a drop-zone highlight overlay during active dock_drag using compute_dock_drop_zone, matching React's split/tab indicator"/>
   <todo id="verify-e2e" content="Rebuild wgpu wasm, manually verify Focus and tab drag in browser via MCP, and re-run wgpu E2E smoke for s and forms"/>
   </todos>
