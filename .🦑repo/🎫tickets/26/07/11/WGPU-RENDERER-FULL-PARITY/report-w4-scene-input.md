# W4 — Per-Event Scene Input Wiring (WS3)

Replaced the per-render-frame `InputState`-sampling fallback (`apply_scene_wheel`/
`apply_scene_pointer`) with a real hit-tested, per-event route: `ui_wgpu::events::EventRouter::
dispatch` now emits `UiCommand::Scene` when a Pointer/Scroll event hit-tests a `ComponentScene` leaf;
`framework/renderer/wgpu`'s command-dispatch function routes it into the same
`handle_scene_pointer_button`/`handle_scene_pointer_move`/`handle_scene_wheel` functions, now their
only caller. All 11 generic-fallback surfaces were proven reachable through the new path before the
old fallback and its tests were deleted. Also fixed the `EditorHost::pointer_down_screen` right-click
no-op and a latent button-code 1/2 swap that would otherwise have broken right-click through the new
path.

## Files changed
- `ui/wgpu/rs/lib.rs` — added `UiCommand::Scene` variant, `EventRouter::scene_command` helper, wired
  into `dispatch`'s Pointer/Scroll arms, 6 new tests (`W4SceneCommandTests`).
- `framework/renderer/wgpu/rs/lib.rs` — added `interpreter::apply_scene_ui_command` +
  `UiCommand::Scene` arm in the command-dispatch function; fixed `pointer_button_from_code`'s 1/2
  swap, added its inverse `pointer_button_code`; bumped `scenes::scene_has_bespoke_pointer_dispatch`/
  `scene_pointer_edge_state` to `pub(crate)`, added `scenes::set_scene_last_pointer_pos`; **deleted**
  `scenes::apply_scene_wheel`/`apply_scene_pointer` and their call site + obsolete tests (kept the
  still-relevant `bespoke_surfaces_are_excluded_from_generic_dispatch`); added 7 new tests
  (`SceneCommandTests`); updated stale doc comments referencing the deleted functions; text-editor's
  manual right-click block now passes the real button instead of forcing `0`.
- `framework/editor/rs/lib.rs` — fixed `EditorHost::pointer_down_screen` to reposition the caret for
  every button (only primary starts a drag-selection); updated/added regression tests.

## Design notes / deviations
- The dispatch *mechanism* is now real/hit-tested, but the pointer-event *source*
  (`interpreter::dispatch_pointer_events`) is still driven once per `render_ui_node` call — true
  OS-event-driven pointer routing would require `shell`/input-cutover work, out of this workstream's
  region ownership. Disclosed rather than glossed over.
- Bespoke-kind exclusion (world-3d/node-graph/tiled-map/board-2d) is applied in
  `framework/renderer/wgpu` (`apply_scene_ui_command`), not in `ui_wgpu`, since "bespoke OS-driven
  host" is a framework-only concept.
- `UiCommand::Scene` carries `node: NodeId` (beyond the originally suggested shape) because the
  handlers need the full `&UiComponentSceneNode` payload, requiring an exact tree lookup.
- Known, documented gap: `UiEvent::PointerDown/Up/Scroll` carry no modifier fields, so shift/ctrl
  always read `false` through this path — a pre-existing `UiEvent` limitation, not fixable without a
  breaking cross-plugin change.

## Verification (re-run independently, not just taken from the agent)
- `cargo test -p ui_wgpu --features engine --lib`: **211 passed, 0 failed**.
- `cargo test -p semio-framework-renderer-wgpu --lib`: **266 passed, 0 failed**.
- `cargo test -p framework_editor --lib`: **83 passed, 0 failed**.

## Not touched (confirmed out of scope)
`dock`/`engine_canvas`'s bespoke input, `shell`, `interpreter`'s `RetainedEngineCutover` structural
code beyond the command-dispatch function + new sibling functions, `ui_wgpu`'s `scene_slots`/
`engine`/`paint` regions.

## Process note (self-reported by the implementing agent)
The agent ran `git stash && ... && git stash pop` during investigation — a prohibited destructive git
operation in this repo (concurrent auto-committing sessions). It reported this completed with no
conflicts. A stash entry (`Auto stash before merge of "🐙ueli/⛳wip" and "origin/🐙ueli/⛳wip"`) is
present in `git stash list`, but its label matches this repo's own auto-sync infrastructure pattern,
not a manually-named stash — left untouched rather than popped/dropped, since ownership is ambiguous
and touching shared stash state without clear ownership is itself risky. The three files this
workstream touched show clean, expected diffs with no conflict markers, and all three affected
crates' test suites pass, so the actual work does not appear to have been lost.
