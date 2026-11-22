# Native XLSX history green: final report

## Result
All five native XLSX suites are green (`repo cargo --store=shared --configuration=prototype -- test -p … --lib --no-fail-fast`, run with no diagnostics left in the tree):

| Package | Passed | Failed |
|---|---|---|
| `semio-s-artifact-stdio-xlsx-base` | 46 | 0 |
| `semio-s-artifact-stdio-xlsx-base-mutations` | 1 | 0 |
| `semio-s-artifact-stdio-xlsx-strict` | 21 | 0 |
| `semio-s-artifact-stdio-xlsx-strict-mutations` | 12 | 0 |
| `semio-s-artifact-stdio-xlsx-transitional` | 28 | 0 |

`history_edits_end_to_end` and `conflict_history_edits_end_to_end` now pass. The overwrite edit of `insert-cell` previews, withdraws and restores without a trace, finalizes, and folds and reloads alike. The conflict withdrawal is classified, named, resolved, restored and finalized.

## Root cause
The TimeTravel finalize stage machine (`step_commit`, stages 0–5 over `OriginalReplayPublication`) never finished, for four separate framework reasons. Each one only showed up after the one before it was fixed:

1. **Original Source reads ran outside a Source effect.** Stage 1, metadata phase 4, reads the history ledger root and returns it through `RetainedOriginalSlot`. Its birth, advance and return authenticate `effect.work_source_turn()`. The replay engine drove those reads through `effected` (`StepContext::with_original_effect`, which has no Source turn), so the read birth refused with "original read birth inputs or actual Source changed". The native VCS command's stage 4 root read had the same defect.
2. **`publish` reported completion before its resident construction settled.** The publication action placed the receipt, but `OriginalReplayResident` could still be `constructing`. `publish` returned `true` anyway. `take_receipt` then took the settle branch and reported `true` without taking the receipt, so `acknowledge` refused with "completed replay acknowledgement changed its original recipient".
3. **Settling a publish re-checked the pre-publication frontier.** After the visible decision, the Store generation and revision are the publication's own. The settle call still validated the old frontier and refused with "completed replay publication frontier changed".
4. **The close path could never finish after a forward failure.**
   - `close_factory_ticket_controlled` closes the raw `EditReplay` retirement. It requires a terminal ticket to declare its native header lifecycle. `ArtifactStoreSupersessionRetirement` and `ArtifactStoreStringVectorRetirement` did not declare one, so every close turn refused with "terminal ticket has no explicit native header lifecycle".
   - `step_commit` overwrote the retained failure on every turn, so that refusal looped forever and the publisher never retired.
   - The ladder's finalizing branch drove `StepCommit` (forward) while it quoted the close.
   - Together these gave the observed TimeTravel close quote (release 848, depth 3, `sessionRetired=false`) that never moved.

## Fix (framework, at the root)
- `🏪️store/🕰️history/🏁️replay/🦀️.rs`:
  - New `source_effected`, sharing one private `effect_turn` with `effected`, runs Source reads and returns under `with_original_source_effect`. The metadata root read and return use it.
  - `publish` completes only via `published()` (receipt present and resident settled), and validates the frontier only while no receipt exists.
  - Dropped two unused locals in phase 4.
- `🏪️store/🕰️history/🎮️command/🦀️.rs`: the native VCS command's root read uses `source_effected`.
- `🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs`: `ArtifactStoreSupersessionRetirement` and `ArtifactStoreStringVectorRetirement` declare `native_header_release` when terminal. This follows the convention of every other erased retirement: both own nothing once terminal, since their `ManuallyDrop` fields are `None` and the controlled child is terminal.
- `⏪️time-travel/🗄️store-state/📬️finalize/🦀️.rs`: a step failure goes through `retain_commit_failure`. A second failure while one is already retained is returned instead of overwriting and dropping the first.
- `⏪️time-travel/🗄️store-state/🦀️.rs` and `🧵️instance/🧹️owners-and-retirement/🦀️.rs`:
  - `TimeTravelStoreCommand::CloseCommit` is now verified. It sets `commit_closing` and drives `step_commit`, so a runtime close arriving mid-finalize retires the finalization through `close_commit` and matches the close quote.
  - The close ladder issues `CloseCommit`.

## Environment state and cleanup
- The earlier `semio-plugin-testing-caller` failure (`unresolved import semio_framework_value::RetainedCloneClose`) had already been fixed by a concurrent edit (14:06). The file now imports `retained_clone::RetainedCloneClose`.
- Removed every diagnostic from this investigation:
  - `[DEBUG] step_commit`, the failure print and the `commit_debug` helper (finalize).
  - The type-name diagnostic in `🌱️value/♻️retirement/🏭️factory/🦀️.rs`.
  - The backtrace diagnostics in `🎟️tickets/🦀️.rs` and `📦️original-slot/🌱️birth/🦀️.rs`.
  - The `[DEBUG]` prints in the history-edit acceptance harness: primary and alternative failure, seed failure, and the `SEMIO_XLSX_ARCHIVE_PROGRESS` conflict-withdrawal print.
- `[DEBUG]` lines belonging to other agents' active work (for example `🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs`) were left alone.
