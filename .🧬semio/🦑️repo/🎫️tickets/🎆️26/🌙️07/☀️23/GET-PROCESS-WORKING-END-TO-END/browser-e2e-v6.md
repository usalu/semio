# Browser E2E v6 (Process 3D, native route): error-free scenario

## Result: GREEN. `verify-process-firefox-bidi.ts` exits 0 on the full scenario: canvas renders, example switched away and back, Viewer and back to Editor, one action applied and undone, zero console errors other than `[DEBUG]` lines. Screenshot: `e2e-process-v6.png` (ticket root). Scenario: `process-interactions.json`. Runner: `run-e2e.sh`.

## Root causes and fixes
1. `setContributions command failed ... app-channel.disposed` and `setAppRegistrations` (`ShellHost/🔁️contributions-refresh/🟦️.ts`): the contributions install loop issued commands to an instance that a remount/hot-swap had retired, and logged the typed teardown as an error. It now skips instances in the retirement ledger (`pluginInstanceWasRetiredV1`) before issuing and treats a retirement that lands mid-flight with the established `isPluginInstanceRetiredV1` rule (the same rule `logUnlessRetiredV1` applies everywhere else); every other failure still logs. In a quiet tree (no concurrent edits, hence no remount) the error does not occur at all: all earlier occurrences coincided with HMR remounts caused by other agents' edits.
2. `input #1 setActiveExample refused: instance-retired` is the designed typed refusal of the input ledger (a `warn`, user-facing text "That app instance was restarted."), emitted only when an input meets a retired instance, i.e. the same remount race; it did not occur in any run of the scripted scenario. No change.
3. One Work overrun poisoned the instance: the generated bridge (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`) held every native poll refusal in the Source for all later calls, so one `authority (work-limit)` poll refusal turned every later progress refresh, completion effect and document transfer into `original-turn.native-return-refused: authority`. A refusal whose receipt was received (the Work is accounted) and whose kind is `authority/work-limit` now fails only its own poll; unaccounted failures and every other kind still latch. The thrown message now carries the refusal kind (`... authority (work-limit)`), and the reactor logs the retained cause for authority refusals too (`report_semantic` was `semantic` only; `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`).
   Law: `🧪️tests/🧬️component-codec-reply/🟦️.ts` "lets one Work overrun fail its own poll without latching the refusal for later calls" with fixture `🧫️fixtures/🚫️work-overrun-refusal.json`: red without the fix (held refusal count 1), green with it, next to the unchanged "preserves a held native poll refusal" law (still green). Run with bun against the generated bridge (`law-runner.test.ts`).
   The overrun itself (a poll that exceeds its Work when the example switches to "No example") reproduced on hub v17 (`Demo` to `No example`: `typed-operation progress refresh failed ... authority (work-limit)` then `load-failed`, all later calls refused). It does not reproduce on hub v18 (tree as of 2026-11-24 afternoon), so the underlying guest overrun was fixed by the concurrent Work-accounting changes; the latch fix is the product-level guarantee that a future overrun stays local.

## Harness
`verify-process-firefox-bidi.ts` now: waits for a canvas, runs `process-interactions.json` (pointer and keyboard input through BiDi `input.performActions`, DOM clicks, per-step settle), records per-step new errors, and fails (exit 1) on any console error that is not a `[DEBUG]` line, on zero canvases, on a step that throws or times out. Negative check done: an injected `console.error("boom injected")` gives `[FAIL] 1 console error(s)` and exit 1.

## Findings not fixed
- Selecting "Concrete Forest" in the example select fires no command (state stays `Demo`); "No example" and "Demo" switch correctly. Not investigated.
- The first canvas now takes 35-65 s after load (wasm compile); the scenario waits for it.

## Environment recovery needed by this run
After the machine restart the `.cache` backing was reset: re-staged the native transpile binary (`cargo build --locked`, which fetched crates), the preview2 vendor and shard worker, republished the hub and all four extension modules (extension wasm rebuilt: the old ones lacked `stageActorContextBudget`), and re-activated. Hub wasm rebuilt as v18 (`hub-v18.sh`, `hub-retry18.sh`).

## Cleanup
No `[DEBUG]` of mine remains. Generated logs, old screenshots and idle build dirs removed; scripts kept in `🗑️generated` (hub-v*.sh, publish-modules.ts, activate-native.ts, support.ts, law-runner.test.ts).
