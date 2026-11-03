# Editor UX (wave 2). Nothing compiled, no cargo run.

All ten Eurocode editors (en1990..en1999) and their viewers delegate to ONE app-surface (`norm/📇️registry/🧬️contract/🖥️app-surface`), so the matrix below is identical for every editor; there is no per-artifact variant. S = shared surface, L = per-artifact local, none = missing.

## 1. Capability matrix (state before this pass -> after)
| Capability | Before | After |
|---|---|---|
| Undo/redo (event-sourced inverse) | S: store has `ArtifactCommand::Undo/Redo`, `setFields` / drag release / remedy are ONE emit = one undo step; chords Ctrl+Z, Ctrl+Shift+Z, Ctrl+Y documented (EN/DE). Host mapping of `norm.shortcut.undo` to the store NOT verified | `history_step(action)` names the step; request to confirm host routing |
| Multi-select | S: `NormSelection.entities` (picked set, rectangle/additive in viewport, `PageDown/Up` cycling) | same, plus `SelectAll` chord (Ctrl+A, viewport) |
| Bulk edit in properties | none (inspector edits one row) | S: `bulk_writes`, `bulk_delta_writes`, `shared_value` (same / mixed / absent) -> plain `setFields` bundle, one undo step. Properties UI rendering of "Mixed" not wired |
| Duplicate | S for design cases only (`duplicate_case_edit`); entities none | S: `duplicate_edits` any collection |
| Copy / paste with id regeneration | none | S: `copy_rows`, `paste_edits`, `fresh_id` (document-unique, `a-copy`, `a-copy-2`, references among pasted rows follow, optional offset); chords Ctrl+C / Ctrl+V / Ctrl+D |
| Keyboard nudging, unit-aware steps | S: handle scope arrows, Shift x10, Alt x0.1, step = increment, else grid, else 1 % of limit span; value announced with unit | unchanged |
| Snapping | S: 1-D handle rules (targets, catalogue, increment, grid, limits, Ctrl frees). Targets were only per-artifact hand lists; no scene geometry | S: `scene_snap_targets` / `with_scene_targets` (member ends + segment midpoints along the handle axis); not yet called by `live_handles` |
| Numeric entry while dragging | S: digits `. , -` and Backspace in `Dragging` (`DragEvent::Entry`), value in handle units, clamped | S: `parse_quantity_entry` / `entry_to_s` (`35 cm`, `1,5m`, dimension checked); statechart still digits-only (see requests) |
| Outliner with search and filter | S: `render_collection` filter input (all words, case-insensitive, id/name/path), counts `n/m`, add/remove, selection mirror | unchanged |
| Check to entity jump and reverse | S: `NormSelection::resolve` (check -> its entity + field path; entity -> governing check), `[`/`]`, Alt+G governing check | unchanged |
| Unit picker | S: `unit_picker` (engineering / SI) through `viewportAct`, view lane only | unchanged |
| EN/DE live switch | S: every label is `LocalizedCopy`/`chrome(en,de,locale)`, shortcut overlay carries both; no default language | 4 new shortcuts EN/DE |
| Touch gestures | S: `map_gesture` per device class (tap, long-press, rectangle, pinch, two-finger pan, +/- nudge buttons on phone, pencil hover), 44 px targets, bottom sheet | unchanged; no copy/paste gesture (long-press menu) |

## 2. Implemented (new, shared, pure)
- `app-surface/🧰️entity-editing/` (mounted once in `🧾️panels/🦀️.rs`): `📋️clipboard`, `🗃️bulk-edit`, `🧲️snap-targets`, `🔢️typed-entry`, `🦀️.rs` (`handle_edits` = one emit, `resolve_edits`, `apply_to_tree` reducer, `HistoryStep`).
- Shortcuts Copy, Paste, Duplicate (Global), SelectAll (Viewport) in `⌨️focus-order`; window actions for them are declared automatically (the viewport window declares one action per shortcut).
- Tests `🧰️entity-editing/🧪️tests/🦀️.rs` (chord -> events -> resulting value tree, oracle = framework `insert_value_at_path`/`set_value_at_path`), feature `🧰️entity-editing/🧪️tests/🧰️entity-editing/🥒️.feature`.

## 3. Not done and why
- No new command reaches the editors: `LiveCommand` and the tool list are SURFACE-owned (v7 active) and closed; adding a variant and a tool id touches the macro, contracts and ten artifacts, and cannot be verified without a build. Recorded as a request. Bulk edit needs no command (it is `setFields`).
- No per-artifact glue was written, so no artifact was touched.
- Not wired: "Mixed" display in the properties window, a touch long-press menu, snapping to the 3-D grid beyond existing `SnapRule.grid`.

## 4. Risks (unverified by rustc)
Glob name clashes of `Clip`, `Pasted`, `Shared` in the panels crate; `Interval::contains/clamp` and `Option::is_none_or` signatures; `Emit.artifact_mutations` field in the test; the panels crate budget (about 220 kB of 262 kB).
