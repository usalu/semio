# Native Browser Guest Stall (Process 3D, Firefox BiDi)

Status: root cause narrowed, NOT fixed. Source untouched; debug edit to the published shim was reverted.

## Evidence (all via `verify-process-firefox-bidi.ts` with a Worker/console preload; new env knobs `PRELOAD`, `EVAL`, `FF_PREFS`; log cut raised to 3000, timeline poll fixed)
- Publish/activation are NOT the cause: the guest instantiates, JSPI is available (Firefox 160), bridge/shim/wasm load, `activate`, `instance-open`, `instance-lifecycle-ack` all succeed, the guest answers turns. No worker error, no panic, no guest log besides `[TRACE] typed-operation slots`.
- Clock is fine (`operationClockUs` bigint, deadline 3600 s away); `privacy.reduceTimerPrecision=false` changes nothing. Fuel (50M) and grants are not exhausted (49.4M left).
- Command ingress timeline (host commands arrive as paged commands): seq 2 completes at once; seq 4 (`setContributions`, 5 pages) executes in 4 real turns (fuel 20451, 59983, 9457, 1050) and then needs ~1830 more "exchange" turns (3 host crossings per tiny ~30-fuel step) = 2.3 s before `command-complete`; seq 5,6,7 complete in 1 turn each; seq 8 (the Set Active Example command) does 4 real work turns (13468, 1280, 2374, 1051) and then never reaches `command-complete`: only ~30-60 fuel per exchange turn, status `command-pending`, ingress cursor not advancing.
- After the real work, the guest is in the retirement/close ladder of the retained command/exchange owner (`⌨️command-ingress-turn` `exchange_generic`: `step_until_slice(close_exchange_owner ...)`, `command.close_original_step`), advancing one tiny step per 3 turns. The host drive cannot batch them because `shardTurnCarriesNothingV1` is false while ingress is `command-pending`, so each turn crosses the worker boundary.
- Host side: `🚚️handle-turn-drive` backs off to 25..250 ms per continuation after 15 s without an observable change (`COMMAND_CONTINUATION_BURST_MS`), measured 13 turns/s after 15 s, and gives up after `COMMAND_CONTINUATION_WALL_MS = 300 s` ("command-ingress-stalled"), after which the instance is dropped as retired (`dropForRetiredInstance`) -> `instance-retired`, "has not answered Set Active Example".
- The "69 staged plugin modules behind source" warning lists unrelated plugins as `unstaged`; process and its extensions are not reported stale.
- `program module unavailable ... NetworkError` was the HMR-reload race, not seen in later runs.

## Root cause (working conclusion)
(c)/(d) hybrid: not slow compute (fuel use is tiny), but the guest's close/retirement ladder for the retained command owner makes one bounded step per exchange turn and `step_until_slice` stops after one step (progress unchanged). That is the same defect as the native editor-test failures in `native-rust-report.md` (store close never leaves `Begin` under the harness grant, WorkLimit; store initializer not terminal-empty after `take_candidate`). A large retained value (the demo example) therefore needs tens of thousands of turns; the browser crossing cost (and the 300 s cap) makes it never finish. It is a framework retirement-ladder bug, shared with the second agent's Rust test work; coordinate before editing framework files (`⚛️reactor/🔄️turn/⌨️command-ingress-turn`, `🧵️job/⏱️context`, `retained-clone`).

## Files
Ticket only: `verify-process-firefox-bidi.ts`, `🗑️generated/preload-*.js`. Published shim restored to original.

## Next
Instrument `exchange_generic` with `[DEBUG]` prints (phase, retained_progress before/after) in a hub wasm rebuild (ncargo.sh, wasm-prototype, -j4), or fix the ladder so a step that frees memory counts as progress; then rerun the harness (screenshot `🗑️generated/e2e-process.png`).
