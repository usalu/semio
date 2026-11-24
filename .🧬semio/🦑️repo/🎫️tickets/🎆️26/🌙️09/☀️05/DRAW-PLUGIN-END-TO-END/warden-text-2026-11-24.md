# Warden alarm text names the real trigger

Reporting only. Level classification, thresholds, mode, victims, actions and ledgers are untouched (the `level` line of `decideHostWardenV1` is unchanged). The installed warden was not touched or restarted, and nothing under `~/Library/Caches` was read or changed.

## Files changed (under `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/`)
- `🟦️.ts`: new `triggerHostWardenV1(level, numbers)` returns `available`, `pressure` or `swap`. Precedence: available memory below the line, then pressure, then swap. `formatHostWardenAlarmV1` picks the reason text from it.
- `🧬️schema/🔣️.json`: `Messages.en/de` gain `triggers` (guard: available/pressure/swap, danger: available/pressure). `reasons.guard` and `reasons.danger` are removed.
- `🧪️tests/🟦️.ts`: new test "the alarm text names the trigger that decided the level, from the same classification as the decision". It checks critical pressure with 2000 MB available (danger, pressure), low memory (danger, available), warn pressure (guard, pressure), swap rate (guard, swap) and low memory (guard, available), in en and de. Each case asserts `decideHostWardenV1` returns the expected level.

## Test
`repo test-file "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🟦️.ts"`
Result: 20 pass, 0 fail, 1249 expect() calls (5.8 s). The existing alarm text test passes unchanged.

## Not verified
- The decision line and the trigger helper are two code paths. The test pins their agreement for the cases above, but a future change to the level condition needs the helper updated too.
- The helper treats a guard level with normal pressure as `swap`. This is correct under the current level condition.
- The running warden still emits the old text until it is restarted on its own.
- Python jsonschema was not installed, so the Python schema check was skipped.
- No typecheck or Rust build was run.

## Coordinator review

Docstring and test title corrected (they claimed one shared code path; the trigger helper only words the alarm). The MOUNTED test fails intermittently on "resumed on the third calm tick" (2 of 4 runs, then 1 of 6 with a 2 s poll). The poll was tried and removed: the child is still stopped after 2 s, so the resume did not happen in that tick and a sampling delay is not the cause. Hypothesis, not verified: the children's real resident memory is below 40 MB on a loaded host, so the guard tick suspends two, and the warden resumes one per tick, largest first. Whether it was flaky before this change was not measured. The warden loop stops when its sources change, so a restarted warden has the new text.
