# FEM playback settlement stall (`playback_presses_are_one_config_edit_and_never_a_history_row`)

Status: NOT PASSING yet (3D and 2D). Root cause of the first jam found and fixed in framework; a second stall behind it is located but not resolved. 2D was not run (host RAM rule, and the 3D fix was not yet verified).

## What ran
- 3D `repo`-less native cargo, private dirs `C:\t\pb`, `C:\t\pbb`, TMP `C:\t\pbtmp`, script `pb-run.sh <3d|2d> <step>` (`-j 6` until the RAM notice, `-j 3` after, single `--test editor_app_2`). Logs: `🗑️generated/playback-a1..a15.log`.
- Test: `result_animation_tick::playback_presses_are_one_config_edit_and_never_a_history_row` (3D file `…/✏️editor/🎮️commands/⏱️result-animation-tick/🧪️tests/🔬️unit/🦀️.rs`).
- a1-a13 ran to the test; a10 failed to compile on another agent's `maintenance_quiescent_under_live_preview` edit, a14 on another agent's dsl-derive edit, a15 on another agent's `surface_name` edit in `semio-plugin` (shared tree in flux, not my code).

## Root cause chain (measured with temporary `[DEBUG]` prints, all removed)
1. The "20278 B grant" is not a per-turn grant: it is the remainder of the fixture caller's LIFETIME Source copy stock (528,482,304 B). `spent 528,450,889 + held 11,137` leaves 20,278 B. The ladder quoting 20,456 B can never be funded by what is left. So the earlier spend IS a leak: the settle loop burned the whole stock.
2. The burner is ladder unit 104 (ToolOverlay) -> `OriginalToolMachineFollow::advance` phase 1, lane 5 (`preview_leaf_step`, window-config semantic preview of the press leaf), re-entered ~650k times without completing, each call charging ~50 KB (later ~800 B) of receiving-scope overhead.
3. Why it never completed: `tool-run::receive` pre-checks `fits(cx, d)` against the whole turn grant, then `with_native_receiving_work` opens a receiving scope whose own claim/control shrinks the grant handed to the body (observed 14-15 KB vs demand 20,456 B; held return claim 19,328 B). `WindowSemanticPreview::advance` then sees `!preview_funds(g,d)` and returns an empty `Progress`, so the unfunded step repeats forever while the scope keeps charging. Quote and funded grant disagree: a framework contract bug, not a FEM problem and not a limit to raise.
4. With the scope sized to cover the quote (experiment: `with_native_receiving_quoted(original_receiving_scope_quote(d)+units)`, run a13) the preview steps DID advance (demands 20456 -> 192 -> 1), proving the diagnosis. The run then stalled at a later stage (see open items).

Another agent concurrently introduced the same remedy in a different shape: `StepContext::with_native_receiving_work(out, floor, op)` ("scope share raised to cover the floor"). Their change overwrote my edit of `receive`/`receive_effect` (last-writer-wins) and left `floor = Default::default()`. I re-applied the intended fix on top of their API: `floor = d`.

## Files changed (framework, minimal)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`: `receive` and `receive_effect` now pass the quoted demand `d` as the receiving floor (`with_native_receiving_work(&mut out, d, ...)`). NOT yet compiled/verified with the other agent's API (the semio-plugin lib was broken by someone else at a15).
- No other kept changes. Temporary `[DEBUG]` edits in: maintenance-ladder, tool-machine `🦀️.rs` (x2: tool-machine and follow), window-config preview, store replay native, artifact-app-laws settle file - all removed (grep verified 0 `DEBUG` from mine; line ~497 `close#` debug in the settle file belongs to another agent).
- Earlier (previous agent) FEM edits unchanged.

## Open items
1. Verify the `floor = d` change once `semio-plugin` compiles again, rerun 3D `bash pb-run.sh 3d <step>` (sequentially, never with 2D).
2. Next stall seen in a11-a13 (with the scope sized by the quote): stage 104 again, demand `copy 1` (= `size_of::<Stage>()`, a stage transition of `NativeSemanticPreparation`, `🏪️store/🔁️replay/🎮️operation/🧬️native`), preview producer `finished=false closing=false`, loop of ~43 `advance` calls per turn making only receipt-overhead "progress" (~800 B each) until the lifetime stock is gone (settle error "stopped after 15051 turns, grant 754"). Suspect: a stage that returns `Blocked`/no-op while the ladder keeps counting the scope acknowledgement as progress. Next step: print `stage as usize` and the `advance` return in `NativeSemanticPreparation::advance` (the edit exists in history: `Stage` is `Copy`), and make a receiving scope that did no semantic work not count as ladder progress (`maintenance_ladder_step` loops while `copied_items != 0`).
3. "terminal ticket has no explicit native header lifecycle" was not reached. Coordinator note: history agent added `native_header_release` for `ArtifactStoreStringVectorRetirement`/`ArtifactStoreSupersessionRetirement`; more store retirement types pending from a new history agent - re-read before adding.
4. 2D (`editor_app_2` in `◻️2d`) not run; same framework path, expected to behave identically.
5. Lifetime-stock exhaustion should become a fast, typed failure (the loop ran 15k turns before the idle bound tripped); consider failing the settle when a receiving scope is charged repeatedly with zero semantic progress.
