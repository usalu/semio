# Compile Pass 2 Result (2026-10-16)

Status: **partial, stopped by the physical-headroom gate at 08:00** (same state as COMPILE-PASS-1). The framework native `--lib` pass ran (batch 0 passes with 0 errors; batches 1-3 produced the errors in section 3, six were fixed, one foreign). Verification of the fixes and the root, tests, wasm and test passes are waiting behind the physical-headroom gate (section 6). This file is rewritten after every batch.

## 1. Work list

Built from `🗑️generated/crate-census.csv` (04:50) by `cp2-tools/worklist.py`: 413 crates have `closureCompilable=true`. Workspace by membership lists: 240 framework-workspace members (`cp2-worklist-fw.txt`, batches `cp2-fw-batch-0..3` of 60), 99 root-workspace-only members (`cp2-worklist-root.txt`), 74 excluded as owned by wave-5 slices (`cp2-excluded.txt`). The census covers only the framework tree; the hub, plugin and teaching workspaces are not in it.

## 2. Commands and results (native `--lib`, framework workspace)

Command (all runs): `SEMIO_SCRIPT_POLICY=… perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check <-p crates> --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`.

| Run | Crates | Result | Log |
| --- | --- | --- | --- |
| framework `update --workspace --offline` (05:08) | lock | `Locking 0 packages`, exit 0 | `cp2-update-fw.log` |
| batch 0 (05:08-05:11) | 60 | exit 0, 0 errors, 26 crate warning summaries | `cp2-fw-lib-0.log` |
| batch 1 (05:11-05:19) | 60 | exit 101, 4 failing crates (section 3) | `cp2-fw-lib-1.log` |
| batch 2 (05:19-05:28) | 60 | exit 101, the same 4 failing crates (dependency of this batch) | `cp2-fw-lib-2.log` |
| batch 3 (05:28-05:40) | 60 | launcher failure `SQLITE_CANTOPEN` / `ENFILE: file table overflow` (machine load 99, system file table full), no compile | `cp2-fw-lib-3.log` |
| batches 1-3 combined, 3 attempts | 180 | two launcher failures (ENFILE; `Too many open files in system` while unpacking `yazi`), then a real run: exit 101, errors in 3 crates (section 3) | `cp2-fw-lib-123.log` |
| update + recheck after the fixes | - | refused by the headroom gate (section 6) | `cp2-update-fw-2.log` |

## 3. Errors found and attribution

| Crate | Error | Attribution | Action |
| --- | --- | --- | --- |
| `semio-framework-replication-interaction` | E0433 `semio_framework_pack_json` unresolved ×4 (`#[canonical_json(owner = …)]` on the 4 serde enums) | fleet carve (W4-REPLICATION): the carve left `pack-json` only in `[dev-dependencies]` | fixed: moved to `[dependencies]` in its `Cargo.toml` |
| `semio-framework-replication-mutation-messages` | E0432 `semio_framework_value::retirement::shared::factory` not found | fleet carve (value facade exposes `factory` as `semio_framework_value::factory`) | fixed: import is `factory::FactorySharedRetirement` in `🎮️mutation/♻️retirement/🦀️.rs` |
| `semio-framework-pack-format-catalog` | E0425 `RetainedPackSymbolSpan` + E0277 ×2 in `🏷️symbols/🎟️storage/🦀️.rs` | fleet carve (W4-PACK `#[path]` mount): the moved file never imported the type from its parent | fixed: added to the `use super::{…}` list |
| `semio-framework-replication` (facade, `deflate` off) | E0432 `semio_framework_deflate` unresolved ×4 | fleet carve (W4-REPLICATION WP7): `DeflateMeasure` / `RetainedInflateCloseStep` re-exports lost their `#[cfg(feature = "deflate")]` gate, the dependency is optional | fixed: gate restored on the 4 `pub use` lines |
| `semio-framework-pack-record-retained` | E0432 `RetainedWireByteSource`, `OriginalWireSourceIdentity` not in `semio_framework_replication_codec` | fleet carve (W4-REPLICATION moved both to `replication-wire-core`) | fixed: import from `semio_framework_replication_wire_core` and dependency added to its `Cargo.toml` |
| `semio-framework-ui-contract-presentation` | E0425 `crate::UI_NUMBER_PRECISION_MAX` not found (`🔢️number-format/🦀️.rs:50`) | **foreign** (peer-owned `ui-contract*` carve) | not touched, see section 7 |

Fixes are not yet re-verified by a compile (section 6).

## 6. Blocker: single-slot headroom gate

Since 06:05 every launcher call (update and check) stops at `Insufficient physical compiler initial headroom` (image `/dev/disk4s2`: 4,287,512,576 B available at 06:05, 4,262,… B (4,162,984 KiB, unchanged through 08:00) at 07:30, 4,294,967,296 B required; the recovery plan finds only protected inactive sessions). Until 07:00 I retried with a blind `sleep 240` loop (14 attempts). At 07:00 the coordinator introduced the single-slot compile mutex (`📓️WAVE5-BRIEF.md`, "Compile admission is single-slot"); I stopped my loop and now use `🗑️generated/cargo-mutex.lock` (acquire by `mkdir`, one batched call of 180 crates, release at once, retry after 10 minutes on refusal). Rounds 1-6 (07:00, 07:10, 07:20, 07:31, 07:41, 07:51) were each refused at the gate and released the mutex immediately. No gate was bypassed and nothing was deleted on the image. The sanctioned fix is Root's historical retirement (currently failing with `Live Original resource control issuance required`).

Consequence: the six fixes below are **not yet verified by a compiler**. Remaining sequence once admitted: (a) the batched framework `--lib` recheck of batches 1-3 (`cp2-fw-batch-123`, log `cp2-fw-lib-123b.log`; the framework `Cargo.lock` carries a hand-added `pack-record-retained -> replication-wire-core` edge, a `cargo update --workspace --offline` would confirm it), (b) the root workspace `--lib` pass (`cp2-worklist-root.txt`, 99 crates), (c) `--tests`, (d) wasm targets, (e) `test` runs.

## 7. Foreign errors

- `semio-framework-ui-contract-presentation` (peer-owned `ui-contract*` carve): E0425 `crate::UI_NUMBER_PRECISION_MAX` not found at `🖱️ui/🧬️contract/🖼️presentation/🔢️number-format/🦀️.rs:50`. The constant is used by `♿️accessibility` and `🏗️builder` in the same family but is not in scope in the presentation crate. It also blocks the crates that depend on `ui-contract-presentation` (not enumerated: they were not reached).

## 8. Hand-offs

- To the ui-contract owner: the foreign error above.
- To Root: the headroom gate (section 6). The framework image has been 7 MB to 31 MB below the 4 GiB gate since 06:05; the mutex and my retries do not change it.

## 9. Not done (no compile possible)

Root-workspace `--lib` pass (99 crates), `--tests` pass, wasm targets, `test` runs, the hub, teaching and plugin workspaces, and the known existing breaks of the checklist (the dag vcs `::graph::manifest` crates and the puzzle2d plugin are not `closureCompilable` / outside the census). Counts: framework native lib: 60 crates pass (batch 0), 120 crates (batches 1-2) hit 4 failing crates (all fixed in source, unverified), batch 3 (60) never compiled (launcher/ENFILE, then the gate); after the fixes the crates behind the failures (the 5 errors of section 3) are unreached. Pass 0 / fail 0 for tests, wasm and test runs (not run). Static dependency scan (`cp2-tools/depscan.py`, `cp2-depscan.tsv`) is too noisy to act on and produced no edit.

## Files changed

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🕹️interaction/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎟️storage/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/📖️source/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/📡️replication/🧪️tests/🔬️facade/🦀️.rs` (the facade test imported the two now gated deflate names unconditionally; they are imported under `#[cfg(feature = "deflate")]`)
- `🧰️framework/Cargo.lock` (hand-added dependency edge for pack-record-retained)
