# 💾️ Incremental Save regression (round 7, 2026-11-27)

## Symptom

`semio-s-artifact-stdio-pdf-native-save --lib`: 12 of 15 passed. Three tests failed deterministically, and in all three the Save fell back to the compact route instead of appending to the original:
- `a_second_incremental_save_stacks_on_the_first` (`🧪️tests/🔬️original-save/🦀️.rs:561`)
- `incremental_over_an_xref_stream_original_appends_a_stream_section` (`:506`)
- `cancelling_an_incremental_save_midway_publishes_nothing` (`:149`, plan refused with `SaveCause::Page`)

The common factor is that every failing test inserts text on page index 1 or 2 using `fonts[0]`, a font that only page 0's resources name. The passing tests edit page 0, whose own reading sees the font.

## Root cause

`🧷️increment.rs`, `IncrementPlan::page` handles the existing-resource binding that slice 15 added (`ExistingResources::index`, `Graft::bind_existing`). An edit on page *n* that uses another page's resource binds the existing object (here F1 → object 4) instead of lowering a copy. The sparse reading of that page does not carry the bound resource as its own, so `Moved::between(reading, probe)` reports `resources` as moved.

The planner already cleared `moved.resources` before the first graft round, because resources are verified at document level by `resources_agree` and the final `coverage` pass. It did not clear it on the readings of later rounds: `moved = still` re-armed `resources`. The next `Graft::round` then demanded `retain_resources`, which fails for a sparse graft, returned `Ok(false)`, and the planner refused with `Compact::Page`. Temporary probes located it at the `round` → `false` branch, with `unseen=["Font:F1"]` bound to `Some(4 0 R)`. The probes are removed.

## Fix

`🧷️increment.rs`: each round's `still` reading clears `resources` the same way as the first round (`let mut still = …; still.resources = false;`). Nothing else changed in product code.

Test helper (`🧪️tests/🔬️original-save/🦀️.rs`): the leftover `[DEBUG] cause=` print became part of the plan assertion message (`refused for {refused:?}`).

## Evidence

`sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-native-save --lib` gives **15 passed, 0 failed** (`🗑️generated/incr-save/t8.log`). This includes the independent lopdf checks of the stacked and xref-stream saves (page count, `Tj` counts, reopened content equals the edited snapshot).

The browser needed a wasm rebuild for this, which round 7 did (`🌐️pdf-browser-journey.md`, round 7).
