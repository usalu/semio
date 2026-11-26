# Broker unit estimates: why `syn` and `libc` were refused, and the fix

## 1. Where the estimate came from

Chain: `RUSTC_WRAPPER` (`semio-build-scheduler`) asks the per-run broker (`🏗️native-build/🎟️admission/🟦️.ts`, `startAdmissionBrokerV1`) for every rustc; the broker calls `predict`:

1. exact history of the unit key (`<package>@<crate>|<target>|<profile>|<types>`, or the local store key with the compiler input hash appended), filtered to a comparable source size;
2. sibling history (same owner, target and types under another profile or inputs);
3. otherwise the cold model: `coldBaseBytes + coldBytesPerKloc * kloc` = 614 MiB + 40 MiB per thousand source lines (`Policy` in the admission schema), floor 256 MiB.

The history store is `<store.root>/components/build-telemetry/units-<platform>-<arch>.jsonl` of the managed Cargo store (`.cache/b/r/m` here; the user cache `~/Library/Caches/semio` only holds the host-capacity ledger, estimates of `repo cargo` operations, and the scheduler state, no per-unit history). So a store that is new, stranded (`m-stranded-*`) or never built a crate has no row for it and the unit is estimated by the cold model.

## 2. Evidence

Baseline run (before the change), `cargo test -j 2 folder_reload_route --no-run` in `semio-plugin-tests-07-3`:

```
unit libc@libc|host|prototype|lib has no measured peak and its cold estimate is 5935 MB, above the library budget of 1536 MB
unit libc needs about 5935 MB but its memory grant is 1536 MB: asking the host to grow the grant by 4399 MB (at most 15 min)
unit hashbrown ... cold estimate is 1564 MB ... asking the host to grow the grant by 28 MB
unit futures_util ... cold estimate is 1659 MB ... asking the host to grow the grant by 123 MB
```

5935 MiB is exactly the ticket's number. The growth request then waited behind other agents' 3600 MB commands in the host fair queue (`unit-growth ... 4399 MB`); after its 15 min bound the broker answers `refused / parent-memory` and the build fails, so on a busy host the unit is refused for good although it needs a few hundred MB.

Real peaks (`units-darwin-arm64.jsonl`, 709 rows, rustc's own rusage per unit): `syn` 51.8 kloc 288 and 398 MB, `proc-macro2` 214 MB, `serde_derive` 284 MB. Over all 709 units the peak is nearly independent of the line count: rows of 15 to 40 kloc have a median of 309 MB (max 449), rows of 40 to 80 kloc 343 MB, while the cold model asks 1714 MB and 3014 MB for those classes. The steep per-line slope was calibrated on the workspace's own generic-heavy crates (2 to 4 kloc, 0.6 to 0.9 GB measured), not on third-party code.

Not the cause (checked): the recorded peak is the unit's own (`unitPeakBytesV1` of its own rusage, charged samples only raise it for that unit; law `🗜️charged-peak`), not the tree footprint and not shared between crates; keys are per crate and the sibling lookup only crosses profiles/inputs of the same crate. The `estimates.json` of the host-capacity ledger is a different store (`cargo`, `cargo:<sub>:<package>` operation classes) and has no unit entries; nothing there was reset.

## 3. Root causes (three, found in this order)

0. Size by directory: `sourceKlocV1` counted every `.rs` below the directory of the root file. In the taxonomy layout that directory holds the sources of sibling crates (manifest in `📦️packages/🦀️rust`, source two levels up), so `semio_framework_trace` (8 own lines) was sized 34 kloc, 1988 MB cold, and recorded history (`lines: 8`) was discarded as a different size. Fix: size from the module closure of the root file (`readRustSourceClosureV1`, `mod` and `#[path]`), else the directory walk without skipped directories and nested package directories.
0b. Overhead stall: the driver's own resident set (about 1.1 GB, counted as non-compiler memory) leaves a 1536 MB grant with about 375 MB; every unit asked the host for a growth of 10 to 450 MB, and each of those sat in the host fair queue behind other agents' 7.7 GB requests (up to 15 min each), or, when the idle rescue was added by another agent, waited for good ("waits although nothing runs, budget 0 MB"). Fix: that growth is bounded (`growth.overheadWaitMilliseconds`, 30 s); afterwards the unit runs alone (`serial` waiter) with the rest of the grant.
1. Cold class (below).

A unit with no history is estimated by one workspace-calibrated linear model for every crate class. For a large third-party crate (`libc` 140 kloc, `syn` 51 kloc) that predicts 6 GB and 2.7 GB, above the 1536 MiB orchestration grant; the broker then asks for grant growth, and a growth that the host cannot give in 15 minutes (busy host) ends in a permanent `parent-memory` refusal. Two defects: the cold model has no class, and a cold guess is allowed to refuse a unit.

## 4. Fix

Schema first (`🎟️admission/🧬️schema/🔣️.json`): `coldExternalBaseBytes` (384 MiB), `coldExternalBytesPerKloc` (4 MiB), `externalSourceMarkers` (`/registry/src/`, `/git/checkouts/`, `/vendor/`), message `growth.serial` (en, de).

Code (`🎟️admission/🟦️.ts`):

- `externalSourceV1(source)`: class of a unit from its source path (backslashes read as slashes).
- `predictPeakBytesV1` takes `external` and `budgetBytes`: an external cold estimate is `384 MiB + 4 MiB/kloc`, never above the budget of its kind (the library budget the warden enforces at runtime anyway, 1536 MiB), never below the floor. Workspace crates keep the existing steep model, so the cold warning and carving discipline for the repo's own crates is unchanged. Result: `syn` about 592 MiB (measured 398), `libc` about 971 MiB.
- `sourceKlocV1` module closure (see root cause 0), `AdmissionWaiterV1.serial`: a serial waiter starts when nothing runs, whatever the grant less the driver's memory leaves, and `rescueStalled` marks a stalled head unit serial after its bounded growth failed.
- Serial fallback in the admission path: a unit whose estimate is cold (no exact, local or sibling history, not a component final) and above the parent grant, when no wider grant came (growth timed out or is unavailable), is admitted alone with the whole grant (`predictedBytes` clamped to the grant, bilingual `[admission] ... runs alone` line) instead of `refused / parent-memory`. A measured estimate above the grant is still refused (the need is real), `beyond-host`, `no-host-fits` and `fleet-placement` are unchanged.

## 5. Laws (`🎟️admission/🧪️tests/🧊️cold-class`)

- `🔣️.json` corpus (seven cold estimates incl. the measured `syn`, nine source paths incl. a Windows path and look-alike directories), `🐍️.py` independent Python oracle written from prose (standard library only), `🟦️.ts`.
- The corpus constants equal the schema policy; TS estimates and classes equal the Python oracle on every case; an external estimate covers its measured peak, stays within its budget and a 140 kloc crate fits the 1536 MiB grant.
- Broker law: a cold workspace unit above a 1536 MiB grant is admitted and charged the whole grant; an external unit under a 1024 MiB grant is admitted at its class estimate; the same unit with a measured 2048 MiB history is refused `parent-memory`.
- Further laws: the own-size oracle (closure and fallback trees), the overhead stall (driver 1161 and 1700 MiB against a 1536 MiB grant: one bounded growth ask, then admitted), the idle rescue of a queued unit after the running one ended. Result: 6 pass, 0 fail.
 Neighbouring broker suites: `🪆️parent` 9 pass, `📈️growth` 5 pass, `🚛️beyond-host` 2 pass. The `🧢️unit-ceiling` suite has 2 failures that predate this change (carving entry `semio_hub_sourcing` is 4000/4400 in the schema, 4273/4500 in the expectation; edited by someone else). The main `🧪️tests/🟦️.ts` could not finish because `native-artifacts` hit the 420 s script deadline behind other agents' builds; it does not touch the changed functions beyond the unchanged default of `predictPeakBytesV1`.

## 6. Live verification

Baseline (old code): the build asked 4399 MB of growth for `libc` and queued behind others without compiling it.

With the final code: `cargo test -j 2 folder_reload_route --no-run` in `semio-plugin-tests-07-3` finished (`Finished prototype profile ... in 16m 49s`, 520 units compiled), `syn` 3.0.9, `syn` 2.0.119 and `libc` 0.2.190 among them, with no cold-estimate warning, growth request or refusal for them; units that did not fit beside the driver asked for one growth of at most 1 min and then ran alone. About 15 of the 17 minutes were host queue waits behind other agents' 7.7 GB requests.

Not done: `cargo test -j 2 folder_reload_route` (expected 9/9) was not run, on the coordinator's instruction to start no further cargo run for 40 to 60 min. Rerun it in `tests-07-3` `📦️packages/🦀️rust` afterwards (the build is cached).

## 7. Not done and notes

- History is per managed store. A recreated or stranded store loses it; sharing a cross-store history would be a separate design (the cold class model makes the loss harmless for small crates).
- The `growth` wait for a measured oversized unit still refuses after 15 min; the reason is surfaced, retry or fleet.
- No learned estimate was deleted or reset.
