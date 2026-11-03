# r2 ep: export-performance vitest check

Scope: read-only. No source edited, no git modifying commands. Test file: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🚀️export-performance/🟦️.ts` (28 tests; feature `🥒️.feature` alongside).

## Result

- Run 1 (15:39): 28 of 28 pass, rc 0, 17 s.
- Run 2 (15:40): 28 of 28 pass, rc 0.
- The 3 failures reported by the pa round (walker-thermal digest, memo, session evaluation count) did NOT reproduce. The export-performance suite is green on two consecutive runs.

## Verdict

The failure is not deterministic in the current tree. It is most likely a transient state while agents were rewriting the same files (pf on the cast exporter and measure module, ts on the thermal session, ph on the phase-change path). Nothing needs to change in the expected values. Do not update the pinned digests or the evaluation-count threshold on the basis of the earlier failure.

## What was checked

- Uncommitted changes in the involved code: `🔨️modules/🌡️thermal/🟦️.ts` (session, `stepRegion`, `phaseKg`, phase-change terms in `assemble`), `🔨️modules/🌡️thermal/🧱️balance/🟦️.ts`, `🔨️modules/🖨️representation/📐️measure/🟦️.ts`, `🔨️modules/📤️export/🖼️svg/🟦️.ts`, `🔨️modules/📤️export/🎬️cli/🟦️.ts`. The cast exporter `🔨️modules/📤️export/🎭️cast/` is untracked and was still being written (mtime 15:39).
- The phase-change path is neutral for walker-thermal: `phaseW` and `phaseKgS` are 0 when `phase === null`, so `x.latent + 0` and `moisture + 0` keep the same bits. The transient step for non-phase regions (`stepRegion`, line `phase === null` branch) uses the same `restoringConductance` and `relax` arguments as the old inline code.
- The session test (line 160, `shared < alone * 0.8`) passed on both runs with the current `thermalSession` chain code (`🌡️thermal/🟦️.ts` around lines 221-232).
- The digest test (line 178) passed on both runs against the fixture `🧫️fixtures/🚀️export-performance/🔣️.json`, which is still untracked and was last written at 15:35:58, i.e. during the round. Its pinned values match the present exporter output.

## Recommended change

None to the test or the expected values. Re-run the staffage vitest once all exporter agents have finished (command in `🧰️cr/round.sh` section (c)). If `export-performance` fails again with the same message, capture the log in the ticket and compare the failing digest with the pinned value in `🔣️.json` before changing anything.

## Open

- The round-1 log at `🗑️generated/cr/round-1/vitest-lib.log` shows different failures (`thermal-span` and `cast-export` suites, ajv strictRequired on `transport`). Those are not part of this check. They point at the cast schema (`🔨️modules/📤️export/🎭️cast/🧬️schema/`) and should be checked by pf.
