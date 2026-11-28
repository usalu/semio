# H91: failure-class registry, `repo doctor failures`, fail-open audit (STAGED, lands after the seal)

Nothing was applied. The full build-system freeze (16:50) holds, so everything is a `repo land` stage.
Stage: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H91/stage` (lane H91, 27 files, bases remembered; 1 base went stale during the work and needs `--rebase`).
Land (after the seal): `repo land --wave "Wave 1127" --lane H91 --title "Failure-class registry, doctor failures and fail-open monitors" --rebase <stage>`.
Kept scripts: `T/🧰️scripts/H91/` (`gen-registry.py` writes the registry data, `mirror.py` builds a symlink mirror for direct test runs).

## 1. Failure-class registry

Schema first: `B/🧯️failure-classes/🧬️schema/🔣️.json` (closed Registry and Summary), data `B/🧯️failure-classes/🔣️.json`, code `B/🧯️failure-classes/🟦️.ts` (`readFailureRegistryV1`, `failureClassOfReadyCodeV1`, `failureClassOfRunV1`, `failureClassOfLinesV1`, `failureClassOfThrowV1`, `failureClassLogLineV1`).
- 53 classes: code `<area>.<name>`, bilingual title and explanation, owner module path (must exist), retry class (`none|wait|backoff|after-change|user`), bilingual user action (required for `user`).
- Rules: all 28 `ReadyCodeV1` codes, 12 run rules (outcome, signal, exit), 12 log patterns, 8 typed catalog refusal families plus key-prefix refinements, 18 named errors, 2 error codes, 131 module path rules (longest prefix) for plain exceptions (the "Internal error" sites), `internal.unclassified` for the CLI boundary.
- Law `🧯️failure-classes/🧪️tests`: (a) schema + Ajv, referential integrity; (b) every class has both languages, a real owner, is used; (c) every ReadyCodeV1 (parsed from the ready source) is mapped and every mapped code exists; (d) every `TYPED_REFUSAL_CLASSES_V1` entry maps; (e) **every throw site in B, C and process (10911 sites in 1830 files) resolves to a class with an owner; a new `refuse<X>V1` family or a new top-level folder without a module rule fails the law**; (f) fixtures equal the independent Python implementation (`🐍️.py`); (g) the worker's bilingual log line reads back as the same class. The first overlay run passed 6 of 7 (7th: user classes without action, fixed).
- Honest limit: plain `throw Error(...)` sites (8152) map to the class of their module folder (`internal.<area>`), not per site; typed families and named errors are per class. A site-level class needs a typed refusal at that site.

Workers record the class (ready sidecar, `🚦️ready`):
- `ready-timings.json` run rows get `failureClass` (coarse from outcome/signal/exit when the row is written, amended by the precise class of the ReadyCodeV1 when the failure is judged via new port `classified`); Timings schema extended.
- `ready-log-<stage>-failed.txt` gets the first line `[failure-class] <code> | <owner> | <retry> | <title> / [Fehlerklasse] ...`.
- Test `🚦️ready/🧪️tests/🧯️failure-class`: row classes, amend, schema, mirror line, and a timing store that cannot be written (one bilingual warning, no throw).

`repo doctor failures [--since 90m|6h|2d|date] [--limit n] [--json] [--lang en|de]` (`B/🩺️doctor/🧯️failures`, routed from `doctor ⌨️command`; MCP tool `doctor_failures`):
- reads `S/v1/origins/*/ready-timings.json` and the `-failed`/`-stalled` logs; groups the last N (default 200, window default 24 h) failures by class: count, machines, last time, owner module, retry class, example line, user action. A log within 2 min of a run refines a run without a recorded class; a log no run explains counts alone; unreadable records are counted, never fatal. Both languages when none is chosen. Exit 0, 2 without a store.
- Surfaces edited: operations catalog (`failures` subcommand, `--since`, `--limit` for doctor), MCP policy + golden fixture (tool `doctor_failures`, one golden vector), suites table (`build-doctor-preflight` trees).
- Test `🩺️doctor/🧯️failures/🧪️tests`: grouping, window, limit, log refinement, unreadable counting, both languages, JSON validates against the schema, bad options refused in both languages, routing from the doctor command, class counts equal an independent Python count of the same store.
- Not read: `service-failure-*.txt` (graph service faults, no class yet) and the `repo run` failure of a machine that is not a worker (no fleet store).

## 2. Fail-open audit

Method: injected a failure into every monitor/estimator/telemetry/metrics source next to a compile and a stage and observed the real process (`script.ts` of the unit cache with a fake compiler; the host sampler; the ready port). Only real gaps were fixed; guarded sources are registered and pinned by the law.

Gaps found and fixed (both were "fail closed" by design comments, now reversed):
1. **Compile slot** (`unit-cache/⏳️admit`): a slot folder that is unwritable or not a folder (`ENOTDIR`) failed the compile with exit 1 before the compiler ran. Now: bilingual `[fail-open] unit-cache.slot ...` warning, event + counter `slot-unavailable`/`alone-unavailable`, the unit compiles without a slot. Aborts and slot timeouts keep their old paths.
2. **Host sampler** (`host-capacity/📡️sampling`): on macOS a failing/timed-out `vm_stat` or `sysctl`, on Linux a missing `/proc/meminfo` threw `countersUnavailable`; the capacity gate threw whenever no earlier sample existed, so the run/compile failed. Now `readHostCountersV1` answers unknown counters (`partial`, nothing available measured, pressure normal), says it once per process in both languages (`availableUnread`), cancellation passes through. The old test "missing Linux counter file fails closed" was replaced (greenfield: no compatibility). `countersUnavailable` is now unused in the HC schema (left, not touched).
3. **Ready timing store**: write failures were swallowed silently; now one bilingual warning through `say`.

Registered (`B/🛡️fail-open`): schema `🧬️schema`, registry `🔣️.json` (9 monitors with kind, owner, failure modes, "what the work does without it", bound), helpers `failOpenV1`/`failOpenAsyncV1` (timeout bound, abort passes through, `MeasuredOverLimitV1` is the only fault that may fail work), `failOpenNoticeV1` (bilingual, once per process, notice never throws). Failure class `monitor.degraded` is the class of the warning (cross-checked with the class registry).
- Warning `once`: `unit-cache.slot`, `host-capacity.sample`, `ready.timing-store`.
- `quiet` with a bilingual reason in the registry (a warning would print one line per compiler process, i.e. per unit): `unit-cache.events`, `.lookups`, `.counters`, `.broker`, `.threads-estimate`, `.offload-estimate`. They were already fail-open (caught and degraded), pinned by injection now.
- Law `🛡️fail-open/🧪️tests`: registry closed + Ajv + owners exist + quiet needs a reason; wrapper unit behaviour (throw, parse error, missing file, timeout, cancel, over-limit pass-through, broken sink); **every registered monitor has exactly one injection that runs the real code and asserts the compile still ran (fake rustc executed once, output produced, exit 0), with the warning present for `once` monitors**. 12 tests.
- Not found failing (guarded already): offload gate, unit-peak and cost history readers, admission memo, broker exchange, event/lookup/counter writers, headroom (`unmeasured` returns unlimited), ready mirror, slice report.
- Not audited: warden loop, remote cache/execution clients (other lanes), Windows paths, `repo run` observation/profile code outside the unit-cache and ready paths.

## Verification
- Direct runs in a symlink mirror (live tree untouched): fail-open 12/12, failure-classes 7/7, doctor failures 5/5, HC sampling 13/13, unit-cache admit-related tests (slot-deadline, wedge, recheck, headroom-first, light-slot, alone) 29/29, ready tests (failure-class, failure-log, external-stop, stop-sender, mirror, source-transient, units, main file) 91 pass, 1 timeout (process-group cancel test, 5 s timeout under load 50, unrelated to the change).
- `repo land check` on the overlay: typecheck 0 errors (80 s), dangling imports 0, debug lines 0. Declared-test gate: the unit-cache member's own suite cannot run in the overlay on this host ("A directory of the native tools is not a plain directory: .../.cache/b/t/m", managed Python/cargo), independent of this change. Final check result: see end of file.

## Unverified / risks
- No worker or compile ran the new code; the lib order of operations was only exercised by tests.
- Land touches areas under freeze (B, C, HC, ready, catalog, MCP policy, suites): needs a slot after the seal; `host-capacity/📡️sampling/🟦️.ts` changed live after staging (use `--rebase`).
- MCP golden vector `doctor-failures` flag order (`--json --limit=50 --since=6h`) was derived from the `leases` vector, not generated; the MCP surface test must confirm it (a Go/Rust implementation also reads the policy).
- `doctor_failures` and the `failures` subcommand were not run through the real `repo` binary (freeze); routing was tested through `runDoctorCommandV1`.
- The coarse timing class is derived before the failure code exists; the precise class arrives via `classified` (second write of ready-timings.json per failed stage).

## Final `repo land --rebase check` (last run, 26 staged files)
- Passed: debug-lines, dangling-imports, typecheck (0 errors), and every declared test file that can run in the overlay: HC sampling and main, operations/help (after regenerating the help snapshot rows `root`, `doctor`, `doctor failures`, `fleet*` with `T/🧰️scripts/H91/regen-help-snapshot.ts`), ready main + `🧯️failure-class`, fail-open, failure-classes, doctor, doctor failures.
- Failed only on the overlay's native-tools directory ("A directory of the native tools is not a plain directory: .../.cache/b/t/m"): the unit-cache member suite and the route-table suite tests (`⚡️caching/🗂️suites`, `🏗️build/🗂️suites`). Not caused by the change: the same unit-cache test (`cache hits are reported to the build broker ...`) also fails on the live tree before the change ("the 'cargo' binary ... is not applicable to the 'stable-aarch64-apple-darwin' toolchain"). The route-table law was therefore not seen green on the overlay; the suites table edit is two trees added to `build-doctor-preflight` (both new members have a `🧪️tests/🟦️.ts`).
- Not run by the gate: the MCP surface test (`🔌️mcp/🧪️tests/🏗️build-surface`, vitest + Go), see unverified list.
- Fixed on the way: help snapshot of the operations catalog; `--since` is validated eagerly; the fake compiler fixture is `.mjs` (a `.ts` there was run as a test).
