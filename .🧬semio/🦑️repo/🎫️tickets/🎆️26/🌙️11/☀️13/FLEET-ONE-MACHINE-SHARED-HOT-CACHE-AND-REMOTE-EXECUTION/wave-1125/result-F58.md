# F58 result: BUILD_SOURCE_CONFLICT on g1-m02 / g1-m15

## Finding
- The conflicting rows are two `global` `closure-graph` rows with the same key (the walk question for the `artifact-package/provider` script) and different facts (closure digest), recorded by `boundedScriptInputsV1` (`script-inputs/🟨️.mjs:497`) inside one inference: a file of the closure changed between two passes. The key (path) is the same, the facts are not; the error message names only the key.
- Evidence for churn, not non-determinism or a fleet-vs-local record: m02 failed 01:16:31 and m15 01:14:05 (two minutes apart, same replicated edit), both with the same reference 1c7c6ae4 (a hash of the key text, so identical text gives an identical reference). The walk is canonical (roots sorted, digests over sorted keys); passes memoize their answers per pass, so a mismatch needs two passes around an edit.
- Why it was fatal: `settleObservedComputationV1` repeats a failed round, but its rule "same code and message in the next round is genuine" compares only the message, which never contains the facts. Round 2 failed with the same text and the owner failed at once, with no pause. The hold shortcut (nothing young, no change) had the same effect.

## Fix
- Settle loop: a `BUILD_SOURCE_CONFLICT` failure is treated as a concurrent edit: up to `conflictRounds` (5) repetitions, pausing 25 ms x 4^n (25, 100, 400, 1600, 6400 ms, 8.5 s in all), the equal-message and the no-young-row shortcuts do not apply while patience remains; only then the unchanged conflict fails the owner. Cancellation during the pause ends with the original reason. Constants are schema-first limits with EN and DE texts.
- Ready classification: new `sourceTransientPatterns` entry `BUILD_SOURCE_CONFLICT\]: Conflicting inference observations`, so a workspace stage that still fails this way is `source-transient` and retries a debounce after the next replicated tree change (F1 pattern).
- Not changed: the fleet/local record question did not apply (no fleet record involved), the closure walk itself.

## Proof
settle-conflict test (heals after two pauses: 3 computations; persists: 6 computations and >= the Python-computed pause sum; cancel in pause), source-transient 4/0 incl. two new rows (conflict true, other BUILD_SOURCE_CONFLICT text false), settle-backoff 3/0, observation suite 69/0. Details in `T/landings.md` Wave 1125 F58.

## Not verified
- No live run: no worker slice was started, so it is unproven that m02/m15 pass the workspace stage; the churn cause is inferred from timing and code, the original two digests were not recorded (the message drops them). Supervisors restart on the replicated code; check `ready.json` of m02/m15 for the next workspace attempt.
- The throw at `observeInferenceComputationV1:1465` is unchanged; a conflict raised outside `settleObservedComputationV1` (not wrapped by a settle) is still immediate.
