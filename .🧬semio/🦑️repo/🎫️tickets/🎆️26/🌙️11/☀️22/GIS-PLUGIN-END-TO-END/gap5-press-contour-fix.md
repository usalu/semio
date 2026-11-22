# Gap 5 Press and Contour Fix (static, nothing compiled or run)

## 1. Slider press tests (settle stops at stage 104)

Cause: `settle_registered_typed_operation` looped on `!maintenance_terminal_is_empty()`. That witness includes `!tool_overlay_retirement_pending()`. A live press preview owns `refold_owed`/held action sources; the ToolOverlay unit answers `AwaitingInput("waits for the original foreground source")` while no `original_tool_follow` exists, so the witness can never become true after a tick.

Semantics decided: a transient preview is a deliberate overlay that only its own release or cancel retires, so it must not block settle quiescence; real terminal-empty stays strict (close, `maintenance_terminal_is_empty` unchanged).

Fix (framework, shared):
- `🔌️plugin/🪜️maintenance-ladder/🦀️.rs`: `maintenance_ladder_terminal_is_empty` now delegates to `maintenance_ladder_settled(false)`; new `maintenance_ladder_quiescent_under_live_preview` = `maintenance_ladder_settled(true)`, which ignores the overlay frontier only when `tool_overlay_waits_for_foreground()`.
- `🔌️plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`: `tool_overlay_waits_for_foreground` moved to `RenderRef` (the ladder ref needs it), `RenderCtx` delegates.
- `🔌️plugin/⚛️reactor/🔌️app/🦀️.rs`: `PluginApp::maintenance_quiescent_under_live_preview` (default = terminal-empty); implemented in `🧵️instance/🔌️plugin-app-impl/🦀️.rs` and delegated in `🧵️instance/🧱️construction/📦️original/🦀️.rs`.
- `🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`: settle loop condition is now `has_pending_typed_operations() || !maintenance_quiescent_under_live_preview()`. The inner per-turn maintenance step is unchanged. Forms fixtures use the same loop and benefit.

Restored coverage: `a_committed_press_edited_in_history_replays_deterministically` re-added verbatim to `gisterrain/.../✏️editor/🎮️commands/🧪️tests/🔬️unit/🦀️.rs` (before `//#endregion 🎚️Scrub`); its helpers (`edit_rows`, `release`, `assert_mounted_history_overwrite`) still exist.

Risk to check on first run: if a tick still stalls, the leaked source is a real foreground leak (stage 104 reports it); print `original_tool_follow.is_some()` and which of the five action-source options is `Some`.

## 2. Contour lifecycle close failure

Not fixed; root cause not provable statically. Narrowed:
- The quote `copy 32, release 32, depth 1` has depth 1 total, so it is `nested(<depth 0 demand>)` of a retained-field owner other than the command log (command log demands are at least depth 4 after `mounted_nested` plus `nested`). Candidates by branch order in `retained_fields_close_demand`: `composition`, `tools.registry`, `tool_jobs` bus, `declared_*`.
- The same quote and rung appear for the history-using tests in older captures (`terrain-worker-custody-lib.log` lines 7 etc.); the passing history-undo tests (grade region, overlay edits) close fine, so the difference is this test's no-op second dispatch and 512-route regeneration.
- The close loop leaves on 4096 idle turns, not on the 4M turn cap (whole suite ran 34 s), so the step returns default progress against a quote that fits.

Diagnostic added (framework, `🔌️plugin/🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs`): `close_rung_name` now reports `RetainedFields/<owner>` via new `retained_fields_frontier`, so the next failure names the exact unretired owner. Fix then belongs in that owner's close step/demand pair (check demand and step agree and that the step shrinks the owner).

## Rerun (coordinator)

1. `--lib` filters: `exaggeration::` (4 tests incl. restored history test), `mounted_terrain_contours` (expect `RetainedFields/<owner>` in the panic if still failing), then `history_operations_receive`, full lib.
2. Compile risk: trait default method and `RenderRef` method move were not compiled.
