# Read-Only Native Cache Recovery Boundary

The canonical incremental scanner found 60 stale finalized sessions totaling 1,492,937,000 logical bytes (1.390 GiB) at 2026-10-08 19:00:33.629 UTC. This selection cannot provide the requested recovery above 3 GiB. No cache was deleted, no lease was acquired, and no peer was signalled.

## Actual Inventory

The first-party `scanCargoIncrementalUnits` ran against `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build` in 3921.19 ms. It observed 211 crate units, two units containing working sessions, and 60 stale finalized sessions, all within the debug profile. The complete candidate inventory and read-only SQLite identity query are retained in `🗑️generated/ui-execution/oct8-cache-stale-incremental-readonly.json`.

These are logical file sizes from the canonical scanner, not a measurement of physically reclaimable APFS blocks. The broader Cargo build directory measured approximately 59 GiB, but that size does not establish an inactive or disposable selection.

The scanner keeps the most recent finalized session for each crate unit. It marks older finalized sessions as separate removal units with `lockHeld: false`; a crate unit with a `-working` child receives protection as a whole unit. The stale-session flag is a scanner projection, not proof that a compiler cannot concurrently use the directory. Session deletion also removes the corresponding sibling session lock file.

## Exact Compiler Lease

`cargoBuildLeaseIdentityV1(buildDirectory, [])` resolved the existing build directory to its physical normalized path, profile `debug`, and exclusive mode. The exact resource identity is:

```json
["semio.cargo.build-lease/v1","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build","debug"]
```

Its existing SQLite resource file is:

```text
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/agents/resource-leases/eb9261d1db2fa74c1988430efb68ef803e20bc762964158c59a60cf9f684808b.sqlite
```

A read-only `bun:sqlite` query confirmed version 1 and this exact resource string. This query confirms identity only; it does not establish lease availability or current ownership.

The repository native-build policy supplies `.🧬semio/🦑️repo/⚡️cache/agents/resource-leases` as the lease directory. The lower native-build owner acquires the Cargo build/profile lease before starting its compiler and holds it through process completion and capture. The canonical pruning orchestrator instead acquires only resource `cache-prune` in the same lease directory. Its age guard does not admit compiler exclusion.

## Actual Nextest Gap

The current native Nextest test path does not acquire that compiler profile lease. `Process/🧪️testing/🦀️cargo/🟦️.ts::runCargoTestsV1` loops over its build and assertion plan and invokes `runBudgetedTestCommand`. The latter directly spawns the process with budget, cancellation, and owned-tree shutdown. Neither function acquires `acquireCargoBuildLeaseV1`. The repository `runRepositoryCargoTests` route and native `owner-command` wrapper likewise do not wrap this Nextest invocation in a Cargo profile lease.

Consequently, holding the existing compiler profile lease during pruning would exclude the compiler routes that already honor it, but would not alone exclude current Nextest builds. This gap prevents a safe concurrent recovery claim under that lease alone. An instantaneous process census without Cargo/rustc is also insufficient mutual exclusion against a new invocation.

The locally installed `cargo nextest list --help` distinguishes `--cargo-profile` (the Cargo artifact profile) from `--profile` (the Nextest configuration profile). The existing compiler lease identity parser accepts Cargo `--profile`. A future Nextest admission must project its actual Cargo artifact profile separately; passing `--profile long` directly would incorrectly select a `long` compiler lease while the normal build uses debug.

## Bounded Recovery Plan

Before any concurrent deletion, admit all compiler entry points against the same physical build/profile resource, including the Nextest build phase, and test profile selection against the existing independent process/lease laws. Keep Nextest configuration and Cargo profile identities distinct. The cleanup owner must then acquire `cache-prune` and the affected exact compiler profile lease in a consistent order, rescan under both authorities, and delete only still-stale finalized sessions while retaining the newest and working sessions. Cancellation must leave both authorities releasable and must not signal unrelated owners.

The current read-only candidate set remains insufficient for recovery above 3 GiB even after that admission is implemented. A larger selection requires a separate measured inventory and authority decision; this report does not authorize deleting entire crate units, active sessions, current compiler outputs, or build-profile roots.

## Source Authority

- Repository Library: `⚡️caching/🧹️pruning/🟦️.ts`, `⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts`, and `⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`.
- Process: `📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts`, `🧪️testing/🦀️cargo/🟦️.ts`, and `🧪️testing/🎛️execution/🟦️.ts`.
- Repository compiler policy: `⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`.

This is a read-only safety audit. No implementation test or native runtime receipt is claimed by the inventory.

## Lossless Terminal Log Compaction

On October 8, 153 managed output logs from terminal pre-October-8 runs were losslessly gzip-compressed after recorded PID absence and an open-file-handle census. Every original SHA256 matched decompressed output before removing only the original generated log. States, inputs, reports, current logs and compiler outputs were preserved. Reclaimed logical bytes: 367645488. Exact plan and digest receipt are under generated/ui-execution/oct8-old-log-compaction-*.json. No compiler process was signalled.

## Current Full Canonical Policy Audit

The full canonical scan executed under the actual shared debug compiler profile lease. Cargo contains 47202140601 bytes across 6737 units. Canonical age policy selected 196 Cargo units totalling 1206468288 bytes. Profile/kind summary: {('wasm32-unknown-unknown', 'cargo-build'): [3, 79060071], ('x86_64-pc-windows-msvc', 'cargo-build'): [9, 5076474], ('x86_64-unknown-linux-gnu', 'cargo-build'): [10, 5145701], ('wasm32-wasip2', 'cargo-build'): [20, 6030804], ('wasm-dev', 'cargo-build'): [4, 6192625], ('debug', 'cargo-build'): [126, 350929022], ('release', 'cargo-target-file'): [1, 1129088], ('wasm32-wasip2', 'cargo-target-file'): [15, 741201785], ('debug', 'cargo-target-file'): [8, 11702718]}. No full-policy deletion was performed. These candidates still require exact debug/current-unit/source exclusion before a contained deletion can be admitted; release candidates were only inventoried, not covered by the held debug lease. Actual debug candidates: 134 / 362631740 bytes. The second stale-finalized-session-only invocation separately removed three selected superseded sessions, 69,374,089 logical bytes, preserving current/newest sessions; its actual Nx command passed despite graph-cache ENOSPC fallback.
