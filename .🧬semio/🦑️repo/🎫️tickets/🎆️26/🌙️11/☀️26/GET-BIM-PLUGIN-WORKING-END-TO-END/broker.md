# Compiler broker: stalled estimate closes the broker

## Findings
- Code: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` (`startAdmissionBrokerV1`, `predict`, `estimate`), Rust client `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs` (`Broker`, "broker closed").
- Chain of failure: `estimate()` raced `predict()` against `bounds.predictMilliseconds` (120 s) and on expiry **rejected**. The per-socket handler's `.catch` then ran `socket.destroy()`, which the Rust scheduler reads as EOF, "broker closed". The wrapper re-asks up to 5 times, and every attempt hit the same stalled estimate, so the compile failed. Nothing crashed or exited. The broker deliberately closed the socket of every unit whose estimate was slow.
- Why the estimate stalls: the logs (model-test22: all failures are external crates, mio/indexmap/log/anyhow/heck/serde_json, rejected in the same tick, 3 min into the build) show a wait on a shared await, not a slow scan. I did not observe the hang directly. `sourceKlocV1` took 30-40 ms on mio and heck in isolation, and the warden cancel dir is empty now. The candidates that fit the evidence:
  - Shared, never-evicted promises: `klocCache`, `closureCache` and `wardenLoading` hold a hung promise forever, so all 5 retries re-await the same one.
  - `readCancels` is a synchronous `readdirSync` plus JSON parse of the machine-wide warden cancel directory (shared by every session), run once per request.
- The old test even enshrined the bug: "a probe whose estimate never finishes is dropped at the estimate bound".

## Fix
- `estimate` no longer fails. At the bound it logs the stuck stage, evicts the unit's kloc and closure cache entries and the shared cancels read, and returns `conservative()`. That forecast is the larger of the unit ceiling of its class and its recorded peak. It is marked `cold`, so the existing serial and parent-allowance clamp applies. The stalled `predict` is detached and its errors are logged, not thrown. A `predict` failure now also falls back instead of destroying the socket.
- `predict` records its stage (`source`, `warden`). The log line names it ("stuck at warden" / "hängt bei warden").
- Concurrent estimates share one warden cancels read per `cancelsShareMilliseconds` (1 s). The shared read is dropped when it stalls.
- `predictMilliseconds` is cut from 120000 to 10000. The wrapper silence bound (180 s) and heartbeat are unchanged, and the deadlines test still holds.
- Not done: the broker stays up as long as the driver does. No idle exit was found in the code. If the broker is closed for another reason, the wrapper retry (5 attempts) already covers a restart.

## Changed files
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json` (bound, `late` message en/de with `{stage}`, `cancelsShareMilliseconds`, bounds description)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🐕️watchdog/🟦️.ts` (replaced the "dropped" test; added a stalled-admit test and a shared-cancels test)

## Tests (`repo test-file`, all pass)
watchdog 7, deadlines 6, waits 6, tests/🟦️.ts 18, growth 10, cold-external 5, available 4, replay 5. The stall is reproduced by a `cancels` port that never resolves and a `predictMilliseconds` of 150 ms. Logs are in `🗑️generated/broker-test-*.log`.

## Open
- The exact hung await in the real runs is unproven. With the fix, the next occurrence logs "stuck at <stage>" and the unit still compiles. If it shows `source`, look at `readRustSourceClosureV1` for external crates. If it shows `warden`, look at the warden module import or cancel directory.
