# FEM R4: keep the last results drawn through a revision change (2D + 3D)

## Root cause
- 2D: `with_live_fields` (session `job-bridge-and-live-mount`) returned `None` whenever the current generation's identity did not match the render's revision. A document-only edit (the deformation scale is `doc.analysis.deformation_scale`, so it bumps the revision) blanked the view (`Pending`) from the edit until the new solve was admitted and bound.
- 3D: `with_result_fields` already keeps the retained generation (`retained_results`, `select_result_shell`), so the view never went blank, but the frame was not clearly marked stale (only a stage text, "queued", because `result_stage` is `None` for a mismatched revision).

## Fix
- 2D `with_live_fields`: no longer drops on revision mismatch (still requires the same app instance). New pure `fields_generation(matches_revision, current_final, previous_final) -> FieldsGeneration { previous, stale }`; `MountedFieldsView.stale` added. The shown generation (current, else previous valid) does not depend on staleness; once the new solve binds, `stale` is false and the new generation shows.
- 2D results window: new pure `stale_caption`; the progress caption is prefixed with "Updating results… / Ergebnisse werden aktualisiert…" while stale. New label `updating_results` in the 2D terminology.
- 3D: label `updating_results` (EN/DE); stale caption is now `Updating results… · <case caption> · <stage>`.
- Design: the scale stays a DOCUMENT setting (persisted analysis display config shared by all windows, already read from `doc` at render). Rendering applies the new scale to the retained fields immediately, without waiting for a solve. NOT done: skipping the re-solve for display-only edits (needs a physics fingerprint excluding display-only analysis settings from request key / revision admission); the re-solve still runs in the background and swaps in on bind. Suggested follow-up.

## Laws (all passed natively)
- 2D `a_stale_generation_is_captioned_as_updating_in_both_languages` (results window unit).
- 2D `a_revision_change_keeps_the_last_generation_drawn_until_the_new_one_binds` (session unit, pure, all generation-state combinations).
- 3D `a_display_setting_edit_keeps_the_previous_generation_drawn_until_the_new_one_binds` (mode-wiring in `editor_app_2`): stale frame keeps results, EN/DE caption, scale redraw differs, bound frame unmarked. Fixture `Lent` now lends the document's real node ids plus a `current` flag; the 3 older mode_wiring laws still pass.
- Mounted (registry-driven) laws: not written; pure laws cover the decision. Not browser-verified; coordinator rebuilds wasm.

## Files
- 2D (`◻️2d/.../✏️editor/`): `🧵️session/💼️job-bridge-and-live-mount/🦀️.rs`, `🧵️session/🦀️.rs` (re-export), `🧵️session/🧪️tests/🔬️unit/🦀️.rs`, `🗣️terminology/🦀️.rs`, `🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`.
- 3D (`🧊️3d/.../✏️editor/`): `🗣️terminology/🦀️.rs`, `🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`, `.../🧪️tests/🎵️mode-wiring/🦀️.rs`.

## Build notes
- 3D builds hit Windows os error 206 (rustc command line over 32767 chars, about 350 `-L dependency=`). Workaround: short private dir `C:\q` plus `--profile p` (inherits dev). Script: `$T22/fem-r4-run.sh` (executed copy `C:\t\r4-run.sh`).
- 3D laws live in the editor crate `--test editor_app_2`, not the results-window lib.
- Earlier blocked by a peer's missing `PluginLifecycleFailure::plain_refusal` (since restored) and a transient peer Cargo.toml edit.
