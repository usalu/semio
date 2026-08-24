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

The self-test at line 10396 installs 62 hostile mutations. They include third wait, caller `block_on`, mutable base vector, missing revision/pending identity, len accounting, unchecked generation/revision, admission after clone, shallow scan/copy/encode/seal, missing snapshot identity, combined handoff/poll, wrong lane, dropped saturated job, removed/reordered driver claims, Ready/Pending/panic owner loss, durable-Ready cancellation regression, publication before revalidation, bulk retirement, lost completion recheck, result/rejection Drop loss, facade spawn-before-durability, bounded-retry/cancel/deadline/currentness loss, exact retry-job loss, callback-close bypass, every observed-capacity omission/reordering, blocking Claim/Revalidate/Retire locks, deep clone under lock, and a shallow mutation for each Rust law body.

## Isolated Gates

Executed after the final source edits:

- `rustfmt --edition 2021 <db-engine component>` — parse/format clean.
- `bun ./📜️script.ts verify interactivity p1x` — live source and all hostile mutations clean.
- `bun ./📜️script.ts verify interactivity p1w` — live source and hostile mutations clean with the exact post-P1x two-wait census.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — live source and hostile mutations clean.
- scoped `git diff --check` for engine/root script — clean.
- scoped source census — exactly two production `db_actor::block_on` sites; no `block_on`, target split, loop, direct thread/pool construction, `unwrap()` or `expect()` in the P1x production region.

No Cargo, Nx, Wasm, browser or broad workspace build/test was run.

## Independent RED Remediation

The independent Terra audit `📓️codex-p1x-independent-source-static-acceptance-audit-2026-08-24.md` identified three exact counterexamples. This revision closes each source trace:

| RED counterexample | Exact repair | Production/law evidence |
|---|---|---|
| A permanently saturated `Lane::Io` left both the main retry job and rejection-close job cycling forever at attempt eight; cancellation, deadline and stale generation could not reach retirement | Retry attempts are capped at eight. Main retry evaluates currentness, accepted cancellation and deadline before exhaustion, atomically claims `Retry → Driving`, publishes the refused job in `terminal_job`, stages the typed terminal result and advances one retained close opportunity from timer callback authority without another lane admission. Rejection-close performs the same exact-job terminal handoff on limit/deadline and closes one job/owner per callback opportunity. | engine lines 5315–5408 and 5861–5940; held-saturation law line 10745 proves cancel/deadline/exhaustion, no ninth refusal, zero backend polls, exact owner return, registry/admission release and rejection-close emptiness |
| Admission bounded requested lengths but never observed allocator-returned `Vec`/`String`/page/`Arc` backing capacities | `DatabaseCreateCatalogBackingLedger` uses checked `u64::try_from` and checked item/byte addition. Preflight records input `String::capacity()` and immutable base `Vec::capacity()` before cloning. Scan records every base string allocation; Reserve and Clone store newly allocated owners before validating their observed capacities; Snapshot stores its `Arc`, writer and fixed page allocation before checked refusal. Every overage therefore remains reachable by incremental retirement. | engine lines 5016–5047, 5802–5809, 6127–6279; hostile controlled-overallocation law line 10506 proves candidate and cloned String overage faults retain then retire exact backing and release admission |
| Claim/Revalidate could block a worker on the catalog mutex, and `Database::catalog` deep-cloned the maximum dynamic vector while holding it | Claim, Revalidate and the equivalent pending-token Retire cleanup use `try_lock`; `WouldBlock` preserves exact phase/ownership and arms one timer-wheel requeue. `Database::catalog` clones only the immutable `Arc` under the mutex, then deep-clones its view after the guard is gone. | engine lines 5811–5827, 6333–6371, 6454–6537 and 7295–7302; maximum-catalog contention law line 10871 holds the mutex across Claim, Revalidate and Retire and asserts each exact worker opportunity remains below 8 ms, retains pending ownership and rearms deterministically |

The permanent P1x verifier now binds these repairs to exact function bodies. Its mutations independently delete retry exhaustion/currentness/cancel/deadline checks, drop the refused job, bypass callback close, replace observed capacities with lengths, reorder owner publication after refusal, remove Arc/page observations, restore blocking locks in each catalog-touching phase, move deep clone under the mutex, and shallow all three new hostile laws. Every mutation is rejected by the isolated P1x gate.

The remediation Rust laws are source/static evidence only. They were not executed because the packet forbids Cargo/native/Wasm builds.

### Remediation Gate Rerun

- `bun ./📜️script.ts verify interactivity p1x` — live source and all 62 hostile mutations clean.
- `bun ./📜️script.ts verify interactivity p1w` — preserved live source and hostile mutations clean.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — preserved live source and hostile mutations clean.
- scoped `rustfmt --edition 2021 --check` — clean.
- scoped `git diff --check` over engine, root script and this report — clean.
