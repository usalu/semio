# Result H111: verification of H110 (live foreign-grant-row isolation)
Deutsch: Ergebnis H111: Pruefung von H110 (live eingespielte Isolation fremder Zusagezeilen).

## Verdicts
| Item | Verdict | Evidence |
|---|---|---|
| `repo land check` on H110's 12 live files (stage `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/stage`, every file touched by one trailing newline because a stage without edits is refused: "changes nothing") | gates passed: debug-lines, dangling-imports, typecheck (100 s), tests | `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/check2.log` |
| Hole in the gate | the same check printed 4 FAILED tests in `🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts` (repo product command tests, run twice) and still said "tests: passed" and exit 0 | `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/check2.log` lines 61-296 |
| H110 fault found: the 4 failing command tests | REAL, caused by H110: tests count/list `grants/` entries and now also see the `<id>.format` sidecar (readdir length 2, `toEqual([id.json])`, and `readdirSync(...)[0]` picked the sidecar and the test overwrote it with the row, so status printed "older format"). Product code is right, tests were not updated | live run `repo test-file` 16 pass / 4 fail |
| Fix | STAGED: helper `rowsV1(directory)` filters `.json` rows in 4 places; `repo land check` of the stage: typecheck passed, 20 pass 0 fail | stage `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/stage2` (1 file: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts`), `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/check3.log` |
| `🤲️lend/🚪️gate` | passes now (1/1, also `🤲️lend` 7/7) with H110's code live, via `repo test-file` and raw bun. The earlier lending-arithmetic failure was transient: peers rewrote `🧪️tests/🚪️gate`, `🎚️elastic-reservation`, `🤝️handover`, `🧾️blockers`, `🔼️upgrade/👥️agents` at 01:37-01:40 while H110 ran. Not H110 (lend files and lend arithmetic do not touch grant reading) | runs above |
| `👻️orphan` | passes 3/3 (broken-pipe case included) | test-file |
| `🔬️footprint` | passes 12/12 | test-file |
| `🦀️cargo-grant` | passes 16/1 skip under the managed runner (`repo test-file` binds managed Python 3.14 via `SEMIO_TEST_PYTHON_BINARY`); fails only under raw `bun test` because `testPythonV1()` falls back to system `/usr/bin/python3` 3.9.6 (no `tomllib`). The test already uses the managed Python when run properly: no fault in the test | test-file |
| `🚪️gate/🟦️.ts` whole file (not asked, found) | 29-33 pass, 1-5 fail, a different set every run: "waiter cannot fit" (queue empty because the 40 ms sleep is shorter than the enqueue under load 95-107; debug run showed the queue entry appears a moment later), peak/ramp/marker tests (`ps` and sleeps under load). Each passes alone (marker group 18/18). Load flake, not H110 | scratch overlay debug run, raw bun |

## Baseline note
No clean pre-H110 baseline exists: `git HEAD` is 2026-11-27 11:17, before the 07:00 backfill landing and other host-capacity landings, and the landings rollback has no rollback point for H110 (it never landed). I therefore proved causation by isolated runs of the failing tests on the live tree plus reading the failing assertions (the 4 command tests name the sidecar directly; the lend/gate failure no longer reproduces).

## Findings
1. `repo land check` passes while the test file itself prints failures (4 FAILED shown twice, gate verdict "passed"). Needs a separate fix in `repo land`'s gate (treat any `(fail)` or failing test-file summary as gate failure); outside this lane, not changed.
2. `repo land check` refuses a stage without edits ("changes nothing") and gates only changed files; H110 must stage real edits to be gated.

## Stage paths (for H101)
- `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/stage2` (land: the command-tests fix).
- `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H111/stage` is verification only, do NOT land (whitespace-only edits).
