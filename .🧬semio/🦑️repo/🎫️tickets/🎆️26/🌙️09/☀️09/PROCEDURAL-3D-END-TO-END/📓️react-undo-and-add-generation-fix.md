# Undo chord and Add Generation chord (2026-11-22)

## 2. Add Generation (`mod+shift+g`): already fixed in source, not re-fixed
`📓️react-generate-chord-restage-2026-09-14.md` fixed both layers (dock seeds an active window via `dockSeedActiveWindowIdV1`; `resolveKeybindingTargetWindowV1` targets the active mode's owning window). The code is still present (ShellHost `window-scope`, its fixture and TS/Rust laws). The 09-13 note §5.2 is stale. Only a runtime recheck is outstanding.

## 1. Undo does not re-arm the preview: root cause found, fixed
Source changed since September: the undo digest (`applied_document_edits_digest(history)`) is no longer used. `ArtifactPendingView` carries no history, and both surfaces' `pending_effects` passed `doc.provisional_generation()` (the tool-machine scrub counter) as the "applied edits" value. Undo/redo (framework-reserved, never reach `dispatch_action`, no emit) do not move that counter, so the poll that follows the history commit saw no document move and owed nothing.

Fix (event/revision driven, route independent): the poll now reads the framework's committed content revision (`doc.source_version().canonical_revision`, an accumulator over the applied edit ids and checkpoint, moved by edit, undo, redo, revert, checkout) plus the scrub generation, as `PreviewEvalDocumentStamp`. `preview_eval_run_effects` owes every attached window one evaluation when the stamp moves. The orphaned history digest was removed (greenfield, no compat).
Poll cadence: a committed history verb returns `UiDirtyScope::Full`, the dirty window render re-arms `pending_effects`, and the same kernel turn polls it, against the live store revision. So no framework change was needed; this must be confirmed at runtime.

## Files
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/` (below, `✳️any`):
  - `🧵️preview-eval/⏯️tool-run/🦀️.rs` (stamp type, link field, signature)
  - `🧵️preview-eval/🪟️window-addressing/🦀️.rs` (removed digest)
  - `✏️editor/🪟️artifact-editor-impl/🦀️.rs`, `👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs` (feed the stamp)
  - `🧪️tests/🧫️fixtures/⏯️preview-eval-run.json` (new `documentMoves` rows, language-agnostic)
  - `🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` (law `a_history_verb_owes_the_previews_the_evaluation_a_gesture_would_have` now drives the fixture)
  - `🧪️tests/🩹️gesture-rearm/🟦️.ts` (independent TS oracle of the same rows)

## Tests
- TS twin: run with `node --experimental-strip-types`; all 3 `documentMoves` rows pass (bun is not installed here).
- Rust law: NOT RUN. `.cache/b/r/m` still ENOSPC (`touch` fails). Run once recovered: `repo test` for the generation3d preview-eval crate (law name above). Also `generation3d` editor/viewer crates must compile (call sites changed).

## Runtime check outstanding
React 6018: edit a slider, `mod+z`, confirm console `toolRunStart`/`flowEvalTick` after the chord and the preview box returning to height 6 (`editor-verbs-keyboard-probe.mjs`, rows `undo-armed-history`); also `mod+shift+g` in edit and generate mode.
