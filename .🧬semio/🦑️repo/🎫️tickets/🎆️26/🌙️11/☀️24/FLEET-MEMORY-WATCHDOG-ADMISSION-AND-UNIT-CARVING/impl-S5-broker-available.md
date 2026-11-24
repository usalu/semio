# Implementation S5: the broker admits against measured available memory

`ADM` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission`

## Files and anchors

| file | change |
|---|---|
| `ADM/🧬️schema/🔣️.json` | `Policy.const.available` (marginBasisPoints 1500, waitMilliseconds 180000, pollMilliseconds 1000, cancelledFactorBasisPoints 12500, cancelledMemoryMilliseconds 3600000); `Messages.const.{en,de}.held.{available,warden,cancelled}` |
| `ADM/🟦️.ts` | new exports `AvailableAdmissionV1`, `AvailableAdmissionInputV1`, `availableAdmissionV1`, `formatAdmissionHeldV1` (after `admissionFrozenV1`); broker wiring in `startAdmissionBrokerV1` (done last, one atomic scripted edit) |
| `…/🧩️component-pass/🟦️.ts` | one token: `log` added to the `startAdmissionBrokerV1({…})` call (`…onCompleted:recordCompletedUnit,log})`), the driver's own `log` is the progress channel |
| `ADM/🧪️tests/🧯️available/` | `🟦️.ts` (law), `🔣️.json` (23 cases), `🐍️.py` (oracle), `🧬️schema/🔣️.json`, `🎫️broker/🟦️.ts` (7 broker tests) |

## The law as implemented

`availableAdmissionV1({predictedBytes, availableBytes, totalBytes, runningUnrealizedBytes, runningUnits, waitedMilliseconds, wardenLevel, cancelled?, policy})` (two inputs beyond the brief: `runningUnits` is needed for "no unit of this broker is running", a sum of unrealized bytes cannot say it; `cancelled` carries the doubled wait):

1. `availableBytes === undefined` -> admit, `unknown`.
2. `wardenLevel` guard/danger -> hold, `warden` (nothing overrides it).
3. room = available - floor(total x margin / 10000) - runningUnrealizedBytes; predicted <= room -> admit, `fits`.
4. else hold, `available`, unless `runningUnits === 0` and waited >= wait (x2 when `cancelled`) -> admit, `waited-out`.

## Broker wiring

- New options (all optional, defaults real): `available` (default `hostAvailableBytesV1`), `warden` (default: level of `readHostWardenStateV1()`, undefined when null), `cancels(sinceMs)` (default `readHostWardenCancelsSinceV1`), `totalBytes` (default `admissionMemoryBytesV1()`), `log`.
- The warden module is imported lazily once at broker start; `ensureHostWardenV1()` is called there (fire-and-forget, throws are swallowed; it does not spawn under `NODE_ENV=test`).
- `schedule()`: `admitWaitersV1` is unchanged; each index it returns is started only if `availableAdmissionV1` admits, with unrealized = sum over running of max(0, predicted - charged) (units started earlier in the same pass count in full). A held waiter stays queued; the wrapper keeps its "wait" verdict and token (no wire or Rust change). While any waiter was held, a 1000 ms unref'd timer re-runs `schedule()`; it is cleared on `close()`.
- `predict`: if a cancel record from the last 3 600 000 ms has `--crate-name <crate> ` in its command, prediction = max(prediction, megabytes x 12500/10000 MiB) and the entry is `cancelled` (needs fit; `waited-out` only after 2 x wait). The probe uses the same prediction.
- Progress: the broker calls `options.log` once per held waiter at first hold and then every 30 s (wait/6): `[admission] <EN> / <DE>` with unit, predicted MB, available MB, margin MB, warden level, seconds waited; kind `warden`, `cancelled` or `available` picks the text (schema `Messages.held`). The driver passes its `log`, so it appears with the `[native-build]` lines.

## Tests (`repo test-file`, exact)

| file | result |
|---|---|
| `ADM/🧪️tests/🧯️available/🟦️.ts` | 4 pass, 0 fail (23 cases equal expected and the Python oracle) |
| `ADM/🧪️tests/🧯️available/🎫️broker/🟦️.ts` | 7 pass, 0 fail (not fitting unit not started, starts on rising availability, poll timer, guard/danger refusal beyond the wait, waited-out and unknown, unrealized remainder, cancelled factor + doubled wait + `semio_x` vs `semio_x_other`, EN+DE log) |
| `ADM/🧪️tests/🔭️probe/🟦️.ts` | 3 pass, 0 fail |
| `ADM/🧪️tests/🔓️tokens/🟦️.ts` | 4 pass, 0 fail |
| `ADM/🧪️tests/♻️replay/🟦️.ts` | 5 pass, 0 fail |
| `ADM/🧪️tests/🧢️unit-ceiling/🟦️.ts` | 5 pass, 1 fail: "Law: no unit of this machine's real ledger…" reads `.cache/b/r/m/components` and finds `semio_s_artifact_stdio_dwg_snapshot_objects` at 4894 MB, unlisted; ledger data, not code |
| `ADM/🧪️tests/🟦️.ts` | 14 pass, 2 fail, both unrelated and untouched by this change: the real-Cargo test fails in the harness (`fixture.cargo.tightPhysicalGiB` missing, so `physicalBytes` is NaN), and "both actual production broker inputs…" fails with `ReferenceError: local is not defined` because it evaluates the launcher's `physicalBytes` expression, which now names `local`; no `fixed.dmg` message appeared this time |
| `…/🧩️component-pass/🧪️tests/🟦️.ts` (3 suites) | native-artifacts 48 pass / 24 fail, native-queued-cancellation 2 pass / 5 fail, component-reclaim 0 pass / 2 fail; causes: "Finite job-only prerequisite graph required" (16), "Live Original resource control issuance required", reclaim hard-failure expectations; all raised in planning before any broker, no `[admission]` line in the log; the folder is mid-churn from peers (job-slots, upfront, schema staged). Counts were identical before and after my `log` edit |
| `…/🧩️component-pass/🧪️tests/🧠️memo-wiring/🟦️.ts` | 1 pass |

## Not done, not verified

- Baseline of the component-pass failures was not taken (no stash/checkout allowed); they are attributed by their stacks, not by a before/after run.
- All units held back: the wrapper keeps its jobserver token and polls "wait", so the build idles; each unit leaves after at most 180 s (360 s for a cancelled crate) because the first waited-out start needs nothing running, then the others are judged against the unrealized remainder. The driver's stall watchdog (`stallMs` 900 000) is longer than both bounds and a start resets `lastActivityMs`. Not run live. Starvation caveat: a large unit can keep losing to small ones that fit while anything runs (waited-out needs an idle broker); the warden is its guard.
- Existing tests that start a real broker now read the real host: they passed here (32 GB, calm), but on a busy host a component final (6 GiB prediction) would be held up to 180 s; tests that must be host-independent should pass `available:()=>undefined`.
- The cargo launcher's second broker (`…/🦀️cargo/🚪️launcher/🟦️.ts:240`) calls `startAdmissionBrokerV1` through the public package entry without any of the new options, so it gets the same real defaults (available reader, warden level, cancel reader, `ensureHostWardenV1`). It passes no `log`, so it holds units back silently; give it `log` to say so.
- The probe (`headroom`) does not include the available-memory verdict (only the raised prediction of a cancelled crate); a probe answering "no wait" can still be followed by a hold.
- Not exercised live on a 16 GB worker (g1-m10) and not run: Rust wrapper, cargo builds.
