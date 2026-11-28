# H62: ready supervisor suites triage (stale-stage, code-stop, main 🚦️ready file)

Baseline (before): 70 pass / 8 fail (H10), reproduced 71/7 and 69/9 here. After: 78 pass / 0 fail, the three files together (run3, after the product landing).

## Root cause shared by 5 tests: Bun drops `declare(...)` calls
The tests named their helper `declare` (writes `code/semantics.json`). Bun's transpiler reads a statement starting with `declare` as a TypeScript ambient declaration and removes it silently (proof: `const declare=(x)=>console.log(x);declare(1)` prints nothing under `bun run`). So every `declare(root,n)` / `bump(root)` was a no-op: no semantics source ever changed, the stage was never stopped for new code, tests waited for their 60 s timeout, and the named-holder test failed because `code/` was never created (ENOENT). Fix: identifier renamed to `declareSemantics` in the three files (test-only).

## Per-test table
| # | Test | Verdict | Cause | Fix |
|---|------|---------|-------|-----|
| 1 | main: named holder stopped only for a declared semantics source | test | `declare()` dropped, `code/` never created, ENOENT in 3 ms | rename helper |
| 2 | stale-stage: quiet read stopped when a source file changed after the stale age | test | same, `bump()` no-op | rename helper |
| 3 | stale-stage: quiet read whose loaded closure changed is stopped as stale | test | same | rename helper |
| 4 | stale-stage: never reports a phase, went quiet, keeps stale rule | test | same | rename helper |
| 5 | stale-stage: second read after the tasks stopped for semantics declared meanwhile | test | same | rename helper |
| 6 | code-stop: quiet read stopped as stale when the code changed | test | same (`declare(root,2)` no-op) | rename helper |
| 7 | main: output mirrored while running and at its end, 200 lines at most | PRODUCT (regression) | H51's `mirrorLinesV1` asked `failureLinesV1` for `mirrorLines-1` lines and appended the raw last line only when it differed; when the last line is a real line it was equal, so only 199 lines were mirrored and the oldest real line was lost. HEAD version mirrored `logTailV1(log,P.mirrorLines,...)` = 200. | `full=failureLinesV1(log,mirrorLines)`; use it when its last line equals the raw last line, else `mirrorLines-1` quiet lines + raw last. Staged `T/🧰️scripts/H62/apply-mirror-200.py`, landed in the 10:00 slot |
| 8 | main: stage that runs too long stopped as timed out, with its group | test (load) | default 5 s bun test timeout; stage kill + reap exceeds it at load 40 | explicit `30000*slow` timeout like its neighbours; assertions unchanged |

Pre-existing vs regression: rows 1-6, 8 are test-side (the `declare` identifier and the default 5 s timeout do not depend on H51; I did not prove when they began failing); row 7 is a regression of H51 against HEAD (`git show HEAD:<file>` mirrored 200).

## Final runs
- Three files together: 78 pass / 0 fail (scratch run3.txt). Mirror and mirrored-into-store tests alone: 2 pass.
- Not run: each of stale-stage / code-stop alone after the change (they passed inside the combined run), `repo typecheck` (only Bun transpile), dangling-imports printed nothing.
