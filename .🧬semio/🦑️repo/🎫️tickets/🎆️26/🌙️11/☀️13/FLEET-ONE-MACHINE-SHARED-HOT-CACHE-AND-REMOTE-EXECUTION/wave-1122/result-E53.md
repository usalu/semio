# Result E53 (stage-survival), halted by coordinator 14:43

## Landed
- g1-m12 diagnosability: closure `recheck` error now names the directory (bilingual, with prior entry count). File `.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts` (114913dccebe -> 28354855594d), census test 3/0. Logged in landings.md. Lock released.

## Finding (from code and `S/origins/*`)
A supervisor restart does NOT lose a stage attempt. The stage is a detached wrapper, `running.json` plus the restored watchdog let the next supervisor adopt it ("adopted by supervisor N" appears in ~20 logs, e.g. m10 survived 3 restarts in 3.6 h, m06/m08 adopted 14:11 CET). Adoption does not depend on code digest; an in-flight stage of a different name is adopted too. Existing test: `🧪️tests/🟦️.ts` "a stage outlives the runner...".
Observed ends are watchdog verdicts, not restarts: m10 killed at the workspace hard cap with the counter stuck at "taken (pass 5) 10 of 604" for the visible 6 min; m08 `stage-timeout` after 84 min stuck in `door.script`; m06 `stalled` (silent after "Planning ..."); m01/m02/m07/m12/m13 `stage-failed` with real errors (cargo build, retirement, bun move, etc.). No "stage-code-changed/stale" line exists in any worker log. The only code-change kill of a progressing stage is the stale rule (`readyStaleStopV1`: reading phase >=30 min and a declared semantics source changed: selected-exchange Policy.revision, observation record stamp). The E46 premise "one hour lost per landing" is not supported by the evidence.

## Open (staged, NOT landed: lock `remote-execution` held by E50 since 14:21)
Goal-aligned hardening: a progressing stage is never stopped for new code. `T/🧰️scripts/E53/apply.py` (idempotent, anchored): schema key `codeQuietMilliseconds` 300000, pure `readyCodeStopV1` in `🌲️process-tree/🟦️.ts`, wiring in `🚦️ready/🟦️.ts` (replaces the stale/holder condition), import. Tests staged in `T/🧰️scripts/E53/code-stop-tests/` (Python-oracle grid, moving read not stopped, quiet read stopped, restart-in-the-middle adopts the same pid and finishes); copy to `🚦️ready/🧪️tests/🪢️code-stop/` after applying. Neither applied nor run, so nothing is verified; run process-tree, stale-stage, watchdog and ready main tests after applying.
Also noted (E18): holder-wait branch still stops stale-code capacity waits; unchanged.
No locks held, no background processes, no files left in the tree beyond the above.
