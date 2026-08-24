# P1z Snapshot-Read Final Source/Static Re-Audit

Date: 2026-08-24  
Auditor: Terra, independent read-only source/static audit  
Verdict: **GREEN — no concrete source/static counterexample found.**

## Scope And Method

Read completely: repository `AGENTS.md`; the prior Codex deadline-cap and Terra fixed-allocator audits; the P1z retained-job caller census/contract; Sol’s P1z implementation report; and live DB sync, protocol wire, DB-engine, WAL, store-sync fixture, hub, and root verifier sources. No production source, verifier, or existing report was edited. No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

## Actual `database_sync_hello_execute` Snapshot-Read Trace

- The live branch selects snapshot bootstrap at `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1267-1324`. Before `snapshots.read_generation`, it calls `database_sync_hello_reserve_snapshot_pages` at `:1273-1274`.
- The constants bind the reservation to `DB_IO_OPERATION_PAGES` at `:296-297`; storage defines that as `64` and `DB_IO_PAGE_BYTES` as `16 * 1024` at `db/🗄️storage/🦀️component.rs:71-72`. Thus the pre-debit is exactly 64 items and `64 * 16 KiB = 1 MiB`.
- The reservation performs `ledger.observe(DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES, ...)` before construction at sync `:510-513`. It is not the generic allocator.
- Observation is physical: `pages.page_count() * DB_IO_PAGE_BYTES` at `:515-523`; it rejects a page/item/byte overrun and also rejects logical length above physical backing. `pages.len()` is never used as the settled debit. Settlement returns only the unused fixed reservation at `:525-527` and occurs after validation at `:1286-1294`.
- The actual execute-snapshot slice has no reachable `reserve_allocation` call from reservation through hash completion. The generic helper is separately defined at `:458-477`; the snapshot read branch invokes only `database_sync_hello_reserve_snapshot_pages`.

## Error And Terminal Ownership Trace

- `read_generation` refusal returns the full page reservation at `:1274-1279`.
- Post-read control failure and observed-bound refusal each explicitly close `pages`, then return the full reservation at `:1281-1291`.
- After settlement, missing hash page, cooperative cancellation/deadline opportunity error, backend latest-generation error, and stale generation each close the retained pages before returning the exact settled `(page_items, page_bytes)` debit at `:1296-1321`.
- Snapshot follow-up close retires a pending fixed chunk before `pages.close_step()` at `:1168-1177`. Normal terminal close drains prepared/follow-up owners first, then input owners, then the item/byte ledger; only after all are empty does it clear execution, release the registry entry, and release admission at `:1673-1753`. Page-close errors are captured in the typed close fault instead of being suppressed at `:1387-1396` and `:1701-1729`.

## Fixed Returned-Frame Path Remains Intact

- `SnapshotChunkBytes` still owns exactly `Option<Box<[u8; 4096]>>`, allocates the fixed box, bounds append, measures physical backing, and explicitly takes its sole backing on close: `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️component.rs:98-165`.
- Direct decode rejects declared size above 4 KiB before borrowing the input range and building that same owner; no raw snapshot `Vec` is materialized: `:569-579`.
- The stream caps caller preference at 4 KiB and reserves/debits exactly one fixed chunk before allocation/copy/publication at DB sync `:1090-1124`. The observed-backing mismatch closes the owner and releases its debit at `:491-501`.
- A returned frame receives its exact measured `(items, bytes)` lease at `:1934-1962`; acknowledgement mounts close only for its generation at `:1976-2005`; release happens only after close is terminal at `:1539-1571`. The hub sends before acknowledging welcome and every follow-up frame at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:775-815`.

## Hostile Mutation Reproduction

`bun ./📜️script.ts verify interactivity p1z` passed. Its self-test binds each mutation to the live source, applies it in memory, and fails the gate if the predicate accepts it (`📜️script.ts:11400-11406`). The three required execute-branch mutations are present and rejected:

| Mutation | Live binding | Why the P1z gate fails |
| --- | --- | --- |
| Restore generic whole-remaining reservation | `snapshot-read-whole-remaining-reservation-restored`, root verifier `:11333` | The execute-snapshot predicate rejects `reserve_allocation` in the `read_generation` slice and requires the fixed page-reservation call before the backend read (`:11243-11246`). |
| Replace fixed page-byte maximum with broad maximum | `snapshot-read-fixed-byte-bound-removed`, `:11334` | The predicate requires `DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES = ... DB_IO_PAGE_BYTES` (`:11246`). |
| Remove observed byte ceiling | `snapshot-read-observed-byte-ceiling-removed`, `:11335` | The predicate requires the `bytes > self.bytes` observed-backing guard (`:11246`). |

The same predicate also requires three full-reservation and four settled-debit cleanup calls plus six explicit page-close calls in the live branch (`📜️script.ts:11246`), so this is not the prior fixed-frame-only false green.

## Checks Executed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — live source and hostile mutations clean |
| `bun ./📜️script.ts verify interactivity p1y` | PASS |
| `bun ./📜️script.ts verify interactivity p1x` | PASS |
| `bun ./📜️script.ts verify interactivity p1w` | PASS |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on protocol wire, DB sync, engine, WAL, store sync, and hub | PASS |
| Scoped `git diff --check` on those sources, root verifier, and the P1z contract | PASS |

No Cargo, Nx, build, Wasm, browser, or runtime test was run.
