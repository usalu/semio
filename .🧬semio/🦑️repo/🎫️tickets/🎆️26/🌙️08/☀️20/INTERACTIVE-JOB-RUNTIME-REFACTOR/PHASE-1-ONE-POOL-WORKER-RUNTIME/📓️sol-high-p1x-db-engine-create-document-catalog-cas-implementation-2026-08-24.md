# P1x DB Engine Create-Document Catalog CAS Implementation

Date: 2026-08-24  
Status: **SOURCE/STATIC IMPLEMENTATION COMPLETE — READY FOR INDEPENDENT ACCEPTANCE AUDIT.**

## Scope

This packet changes only the DB engine P1x transaction, the root P1x verifier plus the exact post-P1x wait expectations in preserved Phase 1 verifiers, and this report. P1q/P1w production regions and peer P5/P6/stdio work were not rewritten.

## Census Counterexample To Repair

| Census counterexample | Implemented repair | Source evidence |
|---|---|---|
| `create_document` held mutable catalog state through whole scan/clone/encode and native `block_on(cas_root)` | `DatabaseCreateCatalogFuture` owns a mounted `Scan → Reserve → Clone → Snapshot → Encode → Seal → Claim → Handoff → Poll → CloseWork → Revalidate → Retire → Publish → Terminal` transaction; `create_document` only awaits its retained result | engine `CreateDocumentCatalogCas` region beginning line 5000; caller line 6907 |
| Catalog base identity could change across suspension | `CatalogState` carries checked `revision`, `Arc<Vec<CatalogEntry>>`, epoch and one generation-qualified pending token; claim and revalidation compare all four identities | engine lines 4992–4997, 6042–6071, 6154–6199 |
| Whole-tree scan/copy/serialization was run-to-completion | Persistent entry/byte/string/JSON fragment cursors advance one byte, one 256-byte UTF-8-safe fragment, one fixed JSON fragment, or one page-seal transition per pool opportunity | engine lines 5418–5527, 5806–6039 |
| Admission used dynamic work before an exact bound | 32 generation-qualified slots reserve fixed worst-case item/byte/page ownership before the base `Arc` is cloned; both requested and pre-existing ID allocated capacities are bounded | engine lines 5001–5121, 5806–5880, 6435–6528 |
| Caller could spawn or emit before durable catalog publication | Exact backend epoch is validated, catalog snapshot/revision/epoch are published, then the result is exposed; only after `actual?` does the facade spawn, emit and register | engine lines 6154–6199, 6907–6925 |
| Concurrent creators could overwrite a newer base or duplicate a document | Duplicate scan is incremental; pending-token claim serializes the selected base; revision/epoch/`Arc` identity and checked successor epoch are revalidated without retry | engine lines 5806–5880, 6042–6071, 6154–6199 |
| Backend work could run on a facade executor or be lost under pressure | The unique `Idle → Queued → Driving` atomic authority submits only to shared typed `Lane::Io`; refusal republishes `error.into_job()` before `Queued → Retry`, then timer retry claims `Retry → Queued` | engine lines 5677–5765 |
| Cancellation could race Handoff/Pending/Ready | Cancellation is checked before the backend poll claim and after Pending owner publication; a Ready result publishes exact work/outcome before poll release, and late cancel/deadline cannot discard a durable successful CAS before revalidation | engine lines 5767–5804, 6087–6152 |
| Refusal, CAS mismatch, panic, Drop or stale generation could drop nested owners | Rejection has a pre-mounted retained close ledger; work/result/terminal owners are republished before claim release; intermediate close retires one pending token, future, page, writer page, snapshot, entry/String backing or base control per opportunity | engine lines 5123–5416, 6201–6425, 6575–6638 |
| Completion could lose a wake | Public polling performs check-register-recheck; publication writes completion before taking/waking the registered waker; Drop hands completion to the generation registry | engine lines 6548–6608 |
| Native/Wasm had distinct semantics and production wait census was three | The P1x region has no target split; production `block_on` census is exactly two, limited to compaction and sync hello | engine lines 6980 and 6989 |

## Hostile Rust Law Bodies

The live test module contains production-path laws for:

1. ID byte-cap `MAX+1`, catalog entry `MAX+1`, exact storage pointer and `String` capacity handback — line 10092.
2. A 128-entry catalog forcing repeated scan/copy/encode/seal grants and exact epoch/revision publication — line 10120.
3. Duplicate plus concurrent same-base creators, one winner, one exact conflict and no duplicate publication — line 10142.
4. pre-service cancellation, deadline, generation-slot ABA and exact owner identity — line 10164.
5. frozen Handoff driver claim, accepted cancellation before backend poll, exact three-page identity and one active driver — line 10193.
6. Pending, Ready-with-in-poll cancellation, and panic publication before poll release on a real shared worker thread — line 10242.
7. real queue saturation, exact retry closure retention and recovery — line 10281.
8. lost-handle terminal take/close with at most one logical retained owner retired per grant — line 10323.
9. one production opportunity below 8 ms and a source assertion proving one native/Wasm state machine — line 10343.
10. facade ordering evidence that catalog durability precedes authority creation/emit/registration — line 10360.
11. deterministic completion publication between the first completion check and waker registration, proving the second check closes the lost-wake race — line 10371.

These Rust laws were added as source evidence but were not executed because this overlapping packet expressly forbids Cargo/native/Wasm builds.

## Permanent Verifier

`bun ./📜️script.ts verify interactivity p1x` is dispatched at root script line 5949. The live semantic verifier begins at line 10255 and checks exact two-wait caller cutover, fixed admission, base identity, every retained phase, cursor granularity, typed I/O lane, atomic driver/retry ordering, Pending/Ready/panic owner publication, checked revalidation, single-owner retirement, result/refusal/Drop authority, facade ordering, forbidden constructs and every hostile law body.

The self-test at line 10362 installs 41 hostile mutations. They include third wait, caller `block_on`, mutable base vector, missing revision/pending identity, len accounting, unchecked generation/revision, admission after clone, shallow scan/copy/encode/seal, missing snapshot identity, combined handoff/poll, wrong lane, dropped saturated job, removed/reordered driver claims, Ready/Pending/panic owner loss, durable-Ready cancellation regression, publication before revalidation, bulk retirement, lost completion recheck, result/rejection Drop loss, facade spawn-before-durability, and a shallow mutation for each Rust law body.

## Isolated Gates

Executed after the final source edits:

- `rustfmt --edition 2021 <db-engine component>` — parse/format clean.
- `bun ./📜️script.ts verify interactivity p1x` — live source and all hostile mutations clean.
- `bun ./📜️script.ts verify interactivity p1w` — live source and hostile mutations clean with the exact post-P1x two-wait census.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — live source and hostile mutations clean.
- scoped `git diff --check` for engine/root script — clean.
- scoped source census — exactly two production `db_actor::block_on` sites; no `block_on`, target split, loop, direct thread/pool construction, `unwrap()` or `expect()` in the P1x production region.

No Cargo, Nx, Wasm, browser or broad workspace build/test was run.
