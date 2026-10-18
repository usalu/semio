# Compiler Journal Recovery and Detached Snapshot Checkpoint

Production progress toward the unchanged monorepo build/cache goal. Compiler charge recovery now uses the existing bounded, serialized Native journal observer, and all three shared journal snapshot sites detach nested child/namespace DATA. **All eight latest Source commands pass.** This is not physical quota, actual concurrent Native build, precise crate invalidation or whole-repository completion proof.

## Actual Integration Defects Fixed

When a compiler-mode receipt was missing, compiler recovery independently read `storage-events.jsonl` with readFileSync, checked the 1 MiB limit after reading, and replayed the journal under only its mode-specific charge lease. Another mode uses the same allocation journal; that lease does not serialize its journal writer. The generic cache store already used the canonical observer. Compiler recovery now awaits observeNativeStorageStateV1 with its exact policy, control and actual image port inside the existing charge try/finally. The canonical observer authenticates actual backing, enters its operation/journal scope, acquires backing-specific global journal exclusion and performs bounded descriptor I/O before projection. Duplicate compiler journal filename/read/replay code is removed.

Compiler receipt validation, duplicate-owner refusal, exact recovered owner/kind/bytes/phase checks, reserve/commit, mode charge exclusion and close behavior remain. The actual framework observer is reused; no provider, grant, quota declaration or cleanup authority is synthesized. Source AST guards cover argument forwarding, await/lease/try/finally and acquire/recover ordering. Real Native simultaneous writers, missing-receipt recovery and oversize-journal memory behavior remain to be exercised under genuine current authority.

Three shared snapshots previously copied/froze allocation rows and namespace identities while sharing the nested child request object. One first-party snapshotNativeStorageStateV1 operation now copies/freezes validated ordinary State, its allocation list/rows, each child request, namespace and both namespace identities. Bounded journal-read results, private memo state and ordinary observation results all use it. This is DATA only. The existing privately held reservation constructor already takes its own child/namespace copies; its custody identities are preserved.

Canonical JSON bytes, state sequence, conservative reserved/committed/retiring charges, journal/event wire formats, backing/queue capacities, reserves, frontiers, controls and actual providers are unchanged. This is an alias boundary repair, not a cache-size increase or migration.

## Test-First and Verification Evidence

Nine closed language-neutral event histories cover empty/reserved/committed, interleaved prototype/release, retiring/retired, duplicate owner, checkpoint and foreign owner cases. Actual first-party event projection is compared with independent SQLite state, byte sums, sequence and owner selection. Ajv independently validates the corpus and actual Event schema. A separate nested State fixture checks byte identity against SQLite JSON, deep freezing, detached child/namespace identities, and mutation of the original input. No Native port/backing/control is created by either DATA test.

Genuine journal-integration red: 1.370s, four passes and exactly the new Source invariant fails (expected canonical observer count 1, actual 0). Journal-only implementation then passes both test modes (1.569/1.957s) and strict modes (37.077/35.828s). Snapshot red: 1.353s, exactly the new absent-helper test fails and all five prior cases pass. Both failures remain in raw generated logs. Final combined implementation passes:

| Gate | Prototype Seconds | Release Seconds | Selected Result |
| --- | --- | --- | --- |
| test | 3.497 | 2.454 | 6 tests / 267 assertions / 1 file |
| types | 51.088 | 47.529 | Existing complete compiler caller strict roots |
| source | 90.975 | 79.766 | 157 tests / 1,643 assertions / 11 files |
| artifact | 46.128 | 67.463 | 73 tests / 713 assertions / 6 files; 27 Native cases filtered |

Every listed gate is terminal status 0. The four broad Source/artifact test invocations also run the separate mandatory adapter cohort (10 tests / 215 assertions). The existing compiler-caller flag keeps its prior adapter policy; the six-case dedicated cohort is not reported as also running an adapter. Final dedicated test/types jobs use at most two outer lanes; broad complete cohorts run sequentially outside, with unchanged full internal lists/order and original controls. No failed Original is resumed, no unchanged failing command is retried for green, and no cap, clock, reserve or vector is raised.

Runtime [DEBUG] logs record nine actual first-party/SQLite event projections and the detached nested snapshot result. Earlier release Source/artifact deadline failures remain historical. These passes qualify a changed imported implementation under its recorded load; they do not establish a causal timing speedup or a general deadline repair.

## Launch and Current Source Qualification

Actual inspection found the existing compiler-state Script flag had no launch registration. The initial assumption that its UI rows could be reused was wrong. Four node-terminal rows are added after the runtime gates, following existing gate group/order/names, with the exact already executed test/types prototype/release Script commands and both Bun/Node cache-disabling variables. JSON parse, row uniqueness and exact command/env correspondence are checked. No new permanent Script or application/test change occurs after the runtime gates.

The prospective final verification window observes 1,821 inherited source/config/schema/test paths. All 113 owned identities and all five runtime/test/schema/fixture files are stable. Maximum observed absolute source path is 191 UTF-16 units. Two peer changes occur:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: `6c806fae33c13335e28c3340d984bc6f013cb69032961ec9db84ca416afdc45b` → `45ce281e9ad25d65ee554fe70531424a56359de6267378c0c4fe52f965359860` (140138 bytes).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json`: `60b206b4acd3cc6ef56fea7275b0d4522191df67afc4546232f3561459226d95` → `0b7ca12bc6c3af18936f7f0fe35140fdc9d546bcf90e8236e045228ad5686396` (116564 bytes).

These dependency storage runtime and install-contract changes are substantive possible execution/input drift. No harmlessness or loaded-child identity is assumed. The same three historical publication/SDK paths remain missing; exact paths are in source-after-verification.json. This inherited census does not discover every new file, establish every transient import revision, or constitute a complete current execution closure or physical repository/OS inventory. No peer change is reverted.

One later delivery census records only the known launch.json addition after the runtime snapshot. The frozen source audit preceding it correctly recorded zero rows; launch-verification.json records the correction and four valid rows. Earlier source executions keep their own recorded revision.

## Files Authored or Updated

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗃️compiler-state/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
- `.vscode/launch.json`

No AGENTS file, modifying Git command or worktree is used. Repository ticket/goal MCP capabilities remain absent; the same active ticket is used. All three existing workers are revalidated terminal after application network permission revocation; they are not restarted or routed around that restriction. Required generated failure/runtime evidence stays inside the unfinished ticket.

## Remaining Full Goal

Actual fixed backing, maximum allocated cache/repository bytes, in-flight/journal/recovery/staging/output/retained metadata, Native pressure/ENOSPC/close/reopen/historical images, full 65,536/65,537 physical inventory and platform behavior remain unqualified. Existing macOS/Linux/Windows providers and authored global partition budget were inspected as Source only; none is provisioned or sampled. Native/default SDK/publication/compiler/cache-image/OS-inventory holds persist.

Precise per-crate/dependent invalidation, no unaffected recompilation, current Nx/compiler/toolchain execution, simultaneous shared warm builds, all-resource CPU scaling, actual prototype/release runtime/size, crate growth and zero-touch native/devcontainer workflows remain open. Sealed compiler program/receipt inputs and task option coupling remain; the previous command recipe is not yet a sufficient artifact key. The goal and ticket remain active.

The current journal projection memo still compares bytes after full replay. The separate rust-journal-projection-cache-follow-up-current.md records the warm-path opportunity and required historical peak-capacity/physical-observation constraints. No warm replay avoidance or host speedup is claimed by this change.

Generated evidence: rust-compiler-journal-red-prototype.json/log, first-verification.json/logs, snapshot-red-prototype.json/log, snapshot-source-before-edit.json, final-verification.json/logs, broad-verification.json/logs, latest-verification.json, source-before-final-verification.json, source-after-verification.json, current-source-audit.json, launch-verification.json and delivery-source.json. All names have the rust-compiler-journal prefix where omitted above.
