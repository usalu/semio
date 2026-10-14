# Build Cohort Audit

Read-only audit on 2026-10-14. Root AGENTS.md read. Source changes were concurrent; schema and fixtures already request shared cohorts while implementation still requested exclusive access at the final read. No compilation, cache deletion, source edits, or modifying Git commands performed. No runtime success claimed.

## Required Completion Checks

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts:8,35–36,46`: identity, schema guard and actual queued profile acquisition must all switch to shared mode; profile acquisition must set `sharedCohort:true`. Changing only mode preserves head-of-queue serialization because queue tickets remain until release.
- The explicit profile maintenance API must acquire shared store then exclusive profile and release in reverse. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts:233` must use it. Incremental compaction currently calls the compiler API; after that API becomes shared, it no longer excludes compilers while deleting finalized sessions.
- `🔒️lease/🧪️tests/🟦️.ts:21` already imports the new maintenance API; fixture cases already distinguish compile/maintain. Preserve compiler-vs-maintenance cancellation and crash recovery and add compile→queued-maintenance→late-compile ordering in both store and profile queues. Existing compile/compile parallel case alone cannot establish writer fairness.
- `🧩️component-pass/🧪️tests/🟦️.ts:380–405` uses a Python `begin exclusive` oracle. The fixture's `unlanedStateOrder.oracle="held"` remains correct for a shared read lease too: exclusive SQLite access must fail with either reader or writer. Do not change this oracle to `unheld`. Its test title/comment should describe maintenance exclusion rather than exclusive mutable ownership.

## Concrete Findings

### P1: Existing Hub Consumer Uses A Different Lease Database Root

`🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1042` passes `repoCacheDirectory(root,"agents","resource-leases")`; repository artifact policy at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:28` and maintenance orchestration at `.../🧹️pruning/📋️orchestration/🟦️.ts:41,45` use `<root>/.cache/framework/leases`. Identical resource strings in different SQLite directories provide no exclusion. Unify actual consumer admission storage before claiming cache-prune protects every compiler.

### P1: Target Output Ownership Is Missing From Maintenance Identity

Store lease identity contains only canonical `buildDirectory` (`🔒️lease/🟦️.ts:53`). Maintenance orchestration nevertheless scans all cache areas, including uplifted Cargo target outputs (`🧹️pruning/📋️orchestration/🟦️.ts:46–55`; target scan at `🧹️pruning/🟦️.ts:246`). Two builders can share target directory while using different build directories; exclusive maintenance on default build directory does not exclude the second builder's target writes. Admission must account for target ownership, or maintenance must select only stores and outputs protected by its actual lease. The private target used by `buildCargoArtifacts` (`native-build/🟦️.ts:745`) reduces this for that entry point, but component driver warm passes use shared `generation.target` (`🧩️component-pass/🟦️.ts:421`).

### P1: Hub Releases Admission Before Reading Its Uplifted Artifact

The Hub consumer releases its lease at `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1045–1047`, then reads/transforms target output at line 1052. Maintenance can delete it in this gap even after lease-directory unification. Another same-profile compiler can replace it during transformation under shared cohorts. Capture the linked artifact into invocation-owned storage before releasing admission; read that immutable captured artifact afterwards.

## Deadlock, Fairness And Original Control Analysis

`🏃️process/🔒️leases/🟦️.ts:228` permits a shared cohort only when every preceding live claim is a shared cohort. An exclusive writer is a barrier, so later readers cannot continually overtake it. Store→profile ordering avoids the simple cross-resource cycle, provided callers do not recursively reacquire admission or invoke maintenance while retaining their build lease.

A nested reacquisition still deadlocks until cancellation when an outer shared store holder awaits an inner shared acquisition and an exclusive store maintenance ticket arrives between them: writer waits for outer; inner waits behind writer; outer waits for inner. Do not infer that shared cohorts make recursive admission safe. No verified concrete recursive production path was established in this audit; add a rejected-nesting contract or carry owned admission through nested entry points rather than reacquiring it.

Both stages must receive the original signal/control unchanged. Current lease API forwards them and releases store on profile refusal; the requested new helper should retain this behavior. Store queue and profile queue must not each restart the caller's deadline. Optional control in older consumers (Hub's polling-derived AbortController) does not preserve the actual caller's progress/yield interface; use the owned admission control where available.

Queue ordering uses millisecond arrival plus PID/owner tie-break (`🔒️leases/🟦️.ts:221`), so FIFO is deterministic within a millisecond rather than actual call-order FIFO. This is bounded overtaking, not unbounded reader starvation. A queued ticket remains live until release, which is essential for writer barriers.

## Cargo Lock Relation

`.cargo/config.toml:11–22` enables separate build and target directories, `fine-grain-locking`, `build-dir-new-layout`, and content freshness; `rust-toolchain.toml:2` pins nightly-2026-07-20. The repo's explicit coarse exclusive profile admission defeats intended same-profile Cargo concurrency even though Cargo is configured for per-unit locks. Shared cohorts remove that outer serialization while preserving a cooperative maintenance barrier. Cargo unit locks remain responsible for compiler intermediate safety; they do not establish application-owned immutable publication or protect maintenance that bypasses the cooperative resource database.

This audit did not independently run a real Cargo overlapping-build probe or inspect pinned Cargo implementation source. Config comments are implementation intent, not runtime evidence. Validation should include two same-profile processes both admitted before release, a maintenance writer that waits for all cohort members, a later compiler that cannot bypass that writer, cancellation with original deadline, and immutable capture during overlapping different-command builds. Use lightweight stub/SQLite probes before any real compilation given disk headroom.
