# WAL Writer Authority

## Status

Current eight-law qualification: session 6141 exit 0, `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-AunM8K/00`, executable SHA256 `eab17c363a307f3f1018acda124f443485d6b5911ec57d471e02706123a2e0ca`. The new mounted maintenance scheduler law first failed in 59839 / `uLyRo9` with fourteen consecutive class-zero selections. Its atomic round-robin start cursor now services continuously ready and faulted classes within bounded rounds; all eight selected laws passed. Earlier paragraphs below preserve the seven-law/test-development chronology. Subsequent WorkerPool maintenance-hook implementation is separate and still under validation.

Latest native qualification: all seven primitive/result/backend-retirement laws passed, session 82992 exit 0, `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-vnVc72/00`, executable SHA256 `8d16be5d5bd7cf817786eb61be65d4f9ffd1b3db3ca7059d6c68b0a8e43d0499`. This includes exact operation pinning, same-operation resume while releasing, rejection of new work, unlock-error retention, explicit guard-terminal retirement, exact backend-owner retention under full rejected-registry pressure, and bounded writer-close fairness. The filesystem law includes a child-written PID-bound conflict sentinel, so success cannot come from a zero-test child selector. This is not a WAL/backend integration receipt.

The pressure regression first failed exactly as expected in session 50210 / `BzOjiy` (`retained=false` instead of true); returning nonterminal false corrected it. The next run 71458 / `PS6h5K` passed that law and failed exactly on the later guard still being retained after 32 close opportunities. A cursor advanced before each release attempt corrected that fairness bug, yielding the seven-law receipt above. Initial 32636 / `YI8aa2` was a test-helper compile error (`yield_now` instead of the repository's `yield_once`), not a runtime assertion.

An eighth neutral regression now covers the mounted maintenance class scheduler. The shared helper currently preserves the original ordered behavior for its expected failing test; the native run is active. The subsequent change will rotate classes so a pending/faulted lost owner cannot starve task-close or writer-release work. No eighth-law success is claimed yet. The root uses its released warm Hub target for these DB laws and does not overlap the Map owner's projection target.

In progress, 2026-09-05. Historical sequence: the strict neutral source oracle passed before its expected missing-implementation failure (95676 exit 1). The first native attempt (16433, `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-r62xCL/00`) failed during shared kernel compilation on concurrent staged-Store privacy/cursor-owner conversions, before any writer law. Those diagnostics were corrected before the successful current five-law receipt.

## Safety Boundary

The active-abort audit exposed a production corruption race: two stale openers can append the same repair suffix, and the old length comparison detects the conflict only after bytes have changed. The existing 23-law sequential recovery receipt does not establish exclusive writer ownership. A guard limited to open-time recovery is insufficient; the backend must retain the authority through all submissions, rotation, failure retirement and terminal close.

The first implementation step is a bounded internal capability core in `storage/🔐️writer/🦀️.rs`: non-cloneable permit; fixed 32-entry generic guard table; exact backend/document/slot/generation validation; checked generation increment; capacity rejection returning the unadmitted guard; and one-guard-at-a-time explicit retirement. The filesystem guard uses the current toolchain's cross-platform `File::try_lock` with read+write+create and never unlinks the sidecar. It releases only through explicit close or final handle destruction.

The registered three native laws cover the neutral generation/scope corpus, capacity+1/ABA/guard retirement, and native independent handles plus a separately spawned test process contending on the same sidecar. They are primitive tests, **not** an `ArtifactWal` integration claim. The file guard's platform calls are intended exclusively for the backend's admitted I/O lane; the test invokes them directly to isolate their OS behavior.

The fourth law was written before repairing the existing `DbIoLostOwner::ResultLease` terminal inversion. Native 68472 / `j0kvhH` passed the original three primitive laws and failed exactly as expected: all five retirement opportunities reported terminal, instead of `[false,false,false,false,true]`. The corrected two branches preserve the parked result through both page releases, shell retirement and result removal; only the final result handback is terminal. The native retry returned exact original ledger credit and passed all four laws. Prior build-only failures `e1oID1`/81782 and `C6M1aI`/31988 were concurrent Store helper-bound and inference Eq errors, both fixed by their owners before the native assertion.

Audit follow-up added the now-qualified resumable operation pin, retained releasing guard through unlock errors, and explicit close-step terminal witness. A further audit found the lowest occupied writer slot can starve later releases, and the existing lost-backend pressure path reports terminal despite restoring its still-owned executor and pool. The new sixth law reserves actual backend-owner credit, fills only empty rejected slots with test-owned sentinels, witnesses exact owner/pool/operation/credit retention, then requires ordinary lane retirement and exact ledger return. The seventh law requires a later guard to retire within one bounded round while the first operation remains pinned.

The detailed credit/result/drop integration packet is `📓️terra-wal-writer-permit-db-io-integration-current-packet.md`. A long-lived permit must not pin its short-lived acquisition task's result handback. There is an additional integration trap: submitting or polling a release operation from the locked lost-owner close path re-enters global maintenance, while fixed maintenance priority can starve the task retirement needed by that release. A separate retained-release design review is active before wiring the real mutation surface.

## Required Integration

The current raw `WalStorage` mutation API is still unchanged and therefore still vulnerable. Next: account the table and permit/result credits in the existing bounded backend owner; carry acquisition and release through typed `DbIoTask` and retained result/lost-owner cleanup; stamp all six mutating tasks; enforce exact live ownership before every fragment; retain the non-cloneable permit inside `ArtifactWal`; and make asynchronous terminal close release it only after pending writers/tasks are drained. Memory, filesystem, SQLite, Postgres, Neo4j, testkit and raw fixture writers must use the same authority surface, with no raw compatibility bypass.

The exact API and backend design is retained in `📓️terra-wal-active-incomplete-durable-abort-current-packet.md`. Directory-entry persistence (including the existing create TOCTOU and create/seal/delete parent barriers) is a separate uncovered filesystem contract in `📓️terra-wal-fs-directory-entry-durability-current-packet.md`.

## Files and Registration

- `storage/🔐️writer/🧪️fixtures/{🔣️.json,🧬️.schema.json}`: strict language-neutral three-trace corpus, mutation families, capacity and cross-process lock expectations.
- `storage/🔐️writer/🦀️.rs`, with internal path registration in `storage/🦀️.rs`.
- Kernel Rust `📜️script.ts` and `📋️project.json`: `wal-writer-authority-check` and `wal-writer-authority-native-check`; independent AJV plus exact-u64 JavaScript Map/Set models, native exact eight-law group (seven qualified, one newly registered).
- `.vscode/🧩️launch.seed.jsonc`: 411.075/.076 with owned warm target and ticket-local generated output. Normal registry generation and immediate freshness verification are assigned to Home.

Home completed normal registry generation and immediate freshness verification: both green, 59 plugins, 60 playgrounds, 45 framework packages. Generated launch entries are present at 5350–5367. No generated launch was hand-edited.
