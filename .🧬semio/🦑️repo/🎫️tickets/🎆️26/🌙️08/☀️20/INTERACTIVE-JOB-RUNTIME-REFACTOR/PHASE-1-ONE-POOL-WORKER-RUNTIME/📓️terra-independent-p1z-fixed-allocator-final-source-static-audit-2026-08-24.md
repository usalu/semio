# P1z Fixed Allocator Final Source/Static Audit

Date: 2026-08-24  
Auditor: Terra, independent read-only source/static audit  
Verdict: **RED — do not accept P1z.**

## Scope And Method

Read completely: repository `AGENTS.md`; the prior P1z allocator RED audit; the P1z retained-job caller census; Sol's P1z implementation report; the live protocol wire, DB sync, engine facade, WAL, store-sync fixture, hub caller, and root P1z verifier. No production source or verifier was edited. No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

The required static gates were run. The P1z command runs its baseline predicate and then every hostile mutation in memory; a passing command therefore proves that none of the listed mutated sources was falsely accepted by that predicate.

## Positive Fixed-Chunk Findings

- `SnapshotChunkBytes` retains `Option<Box<[u8; SNAPSHOT_CHUNK_BACKING_BYTES]>>`, where the unit is exactly `4 * 1024`; fixed construction uses `Box::new([0; ...])`, bounded append rejects an end above the unit, measured backing uses `size_of_val`, and explicit close takes the sole box. See `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️component.rs:98-165`.
- The direct `ServerFrame::SnapshotChunk` decoder first reads and rejects a declared length above 4 KiB, borrows the encoded input slice, and builds `SnapshotChunkBytes` directly. It does not materialize a raw snapshot `Vec`. See `.../wire/🦀️component.rs:569-579`.
- The follow-up chunk branch caps caller preference at the fixed unit, reserves exactly one 4 KiB item/byte debit before `SnapshotChunkBytes::allocate_fixed`, copies only into that owner, and publishes one frame. See `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1058-1090`. The snapshot-specific reservation checks the observed fixed backing equals its 4 KiB debit; mismatch explicitly closes the owner, releases that debit, and refuses. See `:481-499`.
- The fixed returned frame receives the observed `(1, 4096)` credit in its generation-qualified lease (`:1902-1929`). Explicit acknowledgement mounts its close only for the matching generation (`:1944-1978`); the driver performs one close step and only then releases the precise lease debit and removes the matching lease (`:1507-1539`). The hub sends first and acknowledges second (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:791-815`).

## Blocking Counterexample: The Actual Snapshot Branch Still Uses The Generic Whole-Remaining Reservation

The repaired fixed frame allocator is not the only backing-construction path on the actual P1z snapshot route.

1. The production `DatabaseSyncHelloFollowUp::Snapshot` route is selected in `database_sync_hello_execute` when the client is behind the retained floor (`db/🔄️sync/🦀️component.rs:1235-1292`).
2. Before it obtains the page owner with `snapshots.read_generation`, the route computes all currently remaining item credit (`:1241`) and calls the generic allocator reservation: `ledger.reserve_allocation(page_reserved_items, 0, "database sync hello snapshot preallocation backing")` (`:1242`).
3. `DatabaseSyncHelloBackingLedger::reserve_allocation` defines `reserved` as `DATABASE_SYNC_HELLO_MAX_BYTES - self.bytes`, then pre-debits exactly that whole remainder (`:423-430`). It is therefore not constrained to 4 KiB.
4. Only after `read_generation` returns does the route observe `pages.page_count()`/`pages.len()`, reject an overrun, and settle the whole reservation to observed page backing (`:1243-1262`).

Thus a real snapshot request reaches generic `reserve_allocation` and can pre-debit almost the complete 256 MiB budget before its snapshot page backing is constructed. This refutes the stated P1z-wide property that actual snapshot production pre-debits no more than 4 KiB before backing construction and cannot reach generic `reserve_allocation` or `Vec::try_reserve_exact`. The fixed 4 KiB *returned chunk* path is correct, but it does not erase this earlier snapshot-source backing allocation.

## P1z Verifier False Green

`bun ./📜️script.ts verify interactivity p1z` reports clean even with the counterexample above.

The false green is structural:

- The root predicate slices its `snapshotAllocation` only from `struct DatabaseSyncHelloSnapshotBackingReservation` through `fn database_sync_hello_clone_string` (`📜️script.ts:11224-11227`). The live generic route at sync `:1241-1262` is outside that slice.
- The predicate therefore verifies the fixed `SnapshotChunkBytes` reservation but has no prohibition on `database_sync_hello_execute`'s snapshot `read_generation` path invoking `reserve_allocation`.
- Its hostile `snapshot-whole-remaining-reservation-restored` mutation replaces only the fixed-chunk `ledger.observe(1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES, ...)` call (`📜️script.ts:11326`), not the already-live generic page reservation at sync `:1242`.

Accordingly, the command passing is not acceptance evidence for the broader actual-snapshot-production claim.

## Required Mutation Reproduction

The live P1z verifier's in-memory hostile corpus was executed with `bun ./📜️script.ts verify interactivity p1z`. Its self-test applies each mutation, calls `interactivityDatabaseSyncHelloFailures`, and throws if that mutation yields no failure (`📜️script.ts:11394-11400`). The command passed, so all four requested mutations were rejected by the present static predicate:

| Mutation | Binding | Static outcome |
| --- | --- | --- |
| Restore whole-remaining reservation | `snapshot-whole-remaining-reservation-restored` (`📜️script.ts:11326`) | Fails P1z predicate |
| Remove observed backing ceiling | `snapshot-observed-cap-ceiling-removed` (`:11327`) | Fails P1z predicate |
| Reintroduce `Vec::try_reserve_exact` backing | `snapshot-vec-reserve-exact-restored` (`:11328`) | Fails P1z predicate |
| Settle debit before returned lease transfer | `snapshot-debit-settled-before-lease` (`:11329`) | Fails P1z predicate |

This is a genuine mutation result, not a substitute for the counterexample: every listed mutation attacks the fixed-frame subpath, while the live generic snapshot-page reservation remains untested.

## Checks Executed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — hostile fixed-chunk mutations rejected; false-green counterexample above remains |
| `bun ./📜️script.ts verify interactivity p1y` | PASS |
| `bun ./📜️script.ts verify interactivity p1x` | PASS |
| `bun ./📜️script.ts verify interactivity p1w` | PASS |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on protocol wire, DB sync, store-sync fixture, engine, WAL, and hub | PASS |
| Scoped `git diff --check` on those sources, root verifier, and P1z contract | PASS |

## Required Closure

Make the actual snapshot-source page acquisition obey its own fixed bounded ownership contract, or narrow the P1z claim explicitly and coherently if the page backing is outside this property. If P1z retains the present claim, the snapshot branch at sync `:1241-1262` must not call `reserve_allocation` and must not pre-debit the whole remaining byte budget. Extend the root P1z verifier to inspect that branch and add a hostile mutation that restores/inserts its generic whole-remaining reservation. Only then can the fixed frame owner, bounded decoder, observed-cap refusal, and returned-frame lease evidence support a GREEN P1z verdict.
