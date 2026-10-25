# result-kern-d (executor kern-d, wave 1025e) - 08:25 DONE: every test target of `semio-framework-os-kernel` GREEN (graph-free + official Bazel), print-rs crate green

## FINAL 08:25
- Graph-free (`bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-kernel --no-fail-fast`, k14 rc 0): lib 6/6, `artifact_command_retirement` 2/2, `sqlite_snapshot_native_admission` 60/60, doc-tests 0 (bins have no tests).
- Official Bazel `bun repo --tool=bazel run @semio-tech/framework-os-kernel:test` (b1, 08:09-08:21): `//:t_72dfbc1baf494591 PASSED in 90.9s`, 1 of 1 test passes (script runs `cargo test -p semio-framework-os-kernel --features mutation-testing`).
- print-rs crate graph-free (p1: `-p semio-framework-print -p semio-framework-print-tests`): rc 0, print-tests 39/39.
- All 5 reds were STALE TESTS after the retirement/pack refactors, no production defect (decided by evidence below); no production source changed by me, nothing weakened.
  1. `native_allocation_bridge` x2: a refused/canceled receive keeps its refused child frame in the single original return slot until the caller drains it; the tests reused one owner without draining. Added `drain_decode/drain_encode/drain_phase` calls after every failing operation.
  2. `transfer_allocation` x2: (a) the test fixture `TransferSnapshot::{decode,encode}_sqlite_snapshot_native` built a fresh `NativeSnapshotDecodeOwner::new(..)` that dropped the codec frame's `pending` parent, so `receive` re-reserved the root slot (`no available explicit retirement slot`); rewritten to the production pattern `native_owner.scoped_native(..)`. (b) the shared `snapshotGrant` (256 items/64 KiB) is far below the retirement cost (one hop run ~1400 items/~52 KB; a test ~6000 items/~143 KB): raised to 1048576 items/16 MiB copy; `close_original` now fails with `WorkLimit` on a no-progress turn instead of spinning (it hung 19 min at 200 % CPU before). (c) import ledger law: ledger used 4850 B = 37 reconstruction copy + 4813 B native stage growth (nested frame 256 + retirement cursor 392 + retirement page 4096 + item 32 + literal 37); new exact law `used == literal*(copies-1) + staged growth measured on the original encoder` (+ `staged >= literal`).
  3. pack printer: the controlled literal printer is a streaming byte cursor (events have `total==0`, `owned_bytes` 2442, no owned literal copy); the old `owned_bytes>=count && total==count` condition can never hold. Now: cancel at fixture `cancelAt` inside the literal scan, `reached` asserted, and `owned_at_cancel < count` asserted (no owned copy).
- Notes for owners: (1) k4 died once on host `ENFILE: file table overflow` in the repo capacity sampler (fleet-wide fd exhaustion, kern.num_files 128k/245k at the time); (2) the `receive` retirement frame charges a 4096-byte page per retiring frame to the native (and via stages to the SQL) ledger: intended capacity accounting, but 4096+392 per nested frame is large for tiny payloads (perf note for the retirement owner, no action taken).

## Changed files (all test-side)
1. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🧪️tests/💰️allocation/🦀️.rs` (drain helpers + calls)
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs` (fixture `scoped_native`, `close_original` guard, `ENCODE_STAGE_GROWTH` law)
3. `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🧫️fixtures/🔁️transfer/🔣️.json` (`snapshotGrant` maximumItems/maximumCopyBytes)
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs` (pack printer streaming law)
- Tools `C/kern-d-tools/{cargo.sh,bazel.sh}`; output `C/🗑️generated/kern-d/` (k1..k14, p1, b1 raw logs). No [DEBUG] lines remain (verified with rg), no process of mine runs.

---- history below ----

Goal: every test target of `semio-framework-os-kernel` GREEN (lib, `sqlite_snapshot_native_admission`, `artifact_command_retirement`; bins pack/spr/semio compile-only), then Bazel `:test` of the owning project, then print-rs crate graph-free.

## Status
- 04:35 first run k1 (`--test sqlite_snapshot_native_admission`) launched, pid 64968 (detach.py), output `C/🗑️generated/kern-d/k1.raw`; waited ~6 min on cargo capacity (load 40).
- Static finding: the two `native_allocation_bridge` reds (`canceled_input_and_output...`, `typed_refusal...`) reuse one `NativeSnapshotDecodeOwner`/`EncodeOwner` over iterations; `NativeDecodeControl::with_retirement_owner` refuses while the recipient slot still holds an owner (`native decode has no available explicit retirement slot`), so each operation must be drained before the next.

## Tools
`C/kern-d-tools/cargo.sh <id> <cargo test args>` -> `C/🗑️generated/kern-d/<id>.raw` (ends `rc=`).

## 05:50 findings so far (k1..k3)
- k1 baseline 55/60 (5 reds as in result-snap-d).
- Fixed (verified green in k3 run): the two `native_allocation_bridge` tests (stale: a refused/canceled op retains its refused child frame in the single original return slot until the caller drains; tests reused the owner without draining; added `drain_decode/drain_encode/drain_phase` in `...snapshot-capability/🧪️tests/💰️allocation/🦀️.rs`).
- Transfer fixture `TransferSnapshot::decode/encode_sqlite_snapshot_native` (stale test fixture): built a fresh `NativeSnapshotDecodeOwner::new(native,grant)` that dropped the codec frame's `pending` parent, so `receive` took the root path and double-reserved the recipient slot (`no available explicit retirement slot`). Rewritten to the production pattern `native_owner.scoped_native(...)`. Now the first export passes but `close_original` (test loop) spins forever (grant exhaustion suspected): k3's test pid 65271 killed by me (verified lstart/ppid chain). Diagnostics ([DEBUG]) added in the transfer test, k4 running to measure items per op.
- Pack printer test: the controlled literal printer is now a streaming byte cursor; its events have `total==0` (stage begun with 0) and `owned_bytes` 2442 (no owned String copy); the old condition `owned_bytes>=count && total==count` can never hold. [DEBUG] probe in `...native-encoding/🧪️tests/🦀️.rs` (remove before finishing).

## 06:30 update (k4-k6)
- k4 died on host `ENFILE: file table overflow` inside the repo capacity sampler (host-wide fd exhaustion, not mine); rerun k5/k6 fine.
- Transfer tests root cause (evidence k6 [DEBUG]): one direct export costs 52 original-grant items (retirement of the codec frame/database is stepwise, one item per retained object); the next `run` hit `WorkLimit "snapshot retirement exhausted original item grant"` at 256 items, then the test's `close_original` loop spun forever at 0 remaining items (no-progress turns). Fix: fixture `snapshotGrant.maximumItems` 256 -> 16384 (`...io/sqlite-snapshot/tests/fixtures/transfer/json`, shared grant of the whole test; the independent TS oracle only reads schemaSql/literal) + `close_original` now fails (`WorkLimit`) instead of spinning when a turn makes no progress.
- Pack printer: assertion rewritten for the streaming printer (cancel at fixture `cancelAt` inside the first scan, assert the interior was reached, assert `owned_at_cancel < count`, i.e. no owned literal copy).
- k7 (all targets of the package, one run) launched pid 53408.

## 07:25 update (k7, k8, k9)
- k7 (`-p semio-framework-os-kernel --no-fail-fast`, all targets): lib 6/6 ok, `artifact_command_retirement` 2/2 ok, `sqlite_snapshot_native_admission` 59/60 (the pack printer + both bridge tests + export transfer test now green); doc-tests 0 run (1 ignored); bins have no tests. One red left: `transfer_allocation::..._import_retains_physical_before_reconstruction_and_native_output` (`assert_eq!(used, literal.len()*importCopies)`: left 4850, right 74).
- k8 (+[DEBUG]): in the direct import the SQL ledger is 448 B down before `from_sqlite_database`, back to 37 B at the encode stage (transient frame charge released), and the fixture's native delta is 4813 B (before=485): frame/retirement capacity charged by `snapshot::receive` goes to the SQL ledger through `allocation_stage_native`. k9 adds [DEBUG] prints in `...io/control/snapshot/rs` (REMOVE) to decompose the 4813 B.

## 07:50 update (k9-k14): all 60/60 in k13 (filter transfer green); k14 = final full-package run launched 07:47 (pid 87274)
- Import transfer test: decomposed with [DEBUG] (now removed): the native stage growth charged to the SQL ledger = nested receiving frame 256 + retirement cursor 392 + retirement page 4096 + item 32 + output literal 37 = 4813 B (retirement capacity is real allocation; ledger used 4850 = 37 reconstruction copy + 4813). Old law `used == literal*importCopies` (74) is stale; new law: `used == literal*(copies-1) + staged growth` where `staged growth` is what the fixture's `encode_sqlite_snapshot_native` measured on the original encoder (`ENCODE_STAGE_GROWTH`), plus `staged >= literal`.
- Grant sizing evidence: one hop `run` costs ~1400 items / ~52 KB copy (SQLite pages retired stepwise), a whole transfer test ~6000 items / ~143 KB; `snapshotGrant` raised to maximumItems 1048576, maximumCopyBytes 16777216 (like the other receiving fixtures: 65536..1048576 items). `close_original` now fails with WorkLimit on a no-progress turn instead of spinning.

## 08:10 GRAPH-FREE GREEN
- k14 `-p semio-framework-os-kernel --no-fail-fast` rc 0 (07:47-08:00): lib 6/6, `artifact_command_retirement` 2/2, `sqlite_snapshot_native_admission` **60/60**, doc-tests 0 (1 ignored); the three bins have no test targets.
- p1 `-p semio-framework-print -p semio-framework-print-tests --no-fail-fast` rc 0: print-tests 39/39, lib 0 (print-rs unaffected by the kernel test-only edits; the kernel production code is unchanged by me).
- b1 Bazel `bun repo --tool=bazel run @semio-tech/framework-os-kernel:test` launched 08:08 (pid 14375, `C/🗑️generated/kern-d/b1.raw`).
