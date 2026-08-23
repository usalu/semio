# Phase 1s DB Engine VCS Retained Bridge

Date: 2026-08-23

## Source Status

The cohesive VCS integration bridge packet is ready for independent Terra source audit. It does not claim Phase 1 acceptance or ticket closure. Cargo, Nx, Wasm, browser, network, root lint, compilation, and runtime timing were intentionally not run under the packet constraints.

## Production Bridge Census

The census strips every `#[cfg(test)]` item/module and classifies authored production source in DB engine, DB facade, DB CLI, and Hub.

| Surface | Initial forbidden bridge count | Classification |
| --- | ---: | --- |
| `db_engine` | 12 `db_actor::block_on` calls | Five VCS calls were network/product reachable through Hub submit. Seven distinct residuals remain below. |
| DB facade `db/🦀️component.rs` | 0 | Its only bridge is inside the test module. |
| DB CLI | 18 production `db::actor::block_on` calls + one test-only call | The 18 production calls are synchronous command/process entry boundaries: storage open; WAL inspect/replay/repair; snapshot inspect/verify; conflict/replica simulations; migration; and profile submit. The nineteenth whole-file occurrence is only the `#[cfg(test)]` `seed_document` helper. None runs in a Hub/UI/plugin handler or pool closure. |
| authored Hub | 0 `block_on`, `submit_blocking`, or `ask_blocking` calls | Two `.recv().await` sites are Tokio event-stream receive branches, not synchronous executor bridges. |

The initial 12 engine sites were:

- one `replay_history` WAL bridge;
- five `VcsVersionGraph::{ensure_store,record_change,checkpoint,merge_base,head}` nested executor bridges;
- three `Database::open_with` capability/catalog bootstrap bridges;
- one `Database::create_document` catalog-CAS bridge;
- one `Database::compact_document` bridge;
- one `Database::hello` bridge.

Reachability is concrete. Hub `document_handle` accepts network/product work and calls `Database::{document,create_document}`; submission reaches accepted P1r `ArtifactHandle::submit`, retained `ArtifactRunner`, `ArtifactEngine::submit`, then `VcsVersionGraph::record_change`. Hub's wire handshake handler calls `Database::hello`. Hub startup alone reaches `Database::open`; the DB CLI alone reaches `history` and compaction in authored call sites. `checkpoint_document` has no authored production caller, but shares the VCS implementation selected here.

## Selected Cohesive Group

This packet converts all five VCS integration sites. They previously created a local async `work`, kept a `std::sync::MutexGuard<HashMap<_, ArtifactStore>>` alive across its awaits, and invoked `db_actor::block_on(work)` on native targets to force the enclosing future to remain `Send`.

The replacement moves the exact per-document `HashStore` owner out of the registry before any await. `VcsStoreAcquire` is a retained future with a generation-keyed fixed waiter slot. It returns either the exact existing-store lease or a unique build permit. `VcsStoreLease::drop` returns the store owner; cancellation of a pending acquire clears only its exact waiter; cancellation of construction releases only its exact build permit. No mutex guard crosses an await.

Released stores reserve the oldest admitted waiter generation before waking it. A later request therefore cannot overtake a woken FIFO owner, and dropping the selected-but-not-yet-polled acquire atomically reserves and wakes the next owner. Admission-slot reuse cannot consume or wake work from the prior generation.

The retained VCS authority has:

- 64 fixed generation-keyed operation slots;
- 16 KiB logical pages;
- four pages / 64 KiB maximum retained request credit per operation;
- 256 pages / 4 MiB process aggregate;
- checked item credit for the operation, optional parent, change IDs, every moved author-name
  String, and every simultaneously retained derived author-ID String;
- checked byte credit for document/parent/author/message owners, source and derived Vec backing allocations, every nested change-ID String, every nested author identifier, the derived checkpoint-author ID Strings, and the fixed record-mutation backing.

`record_change` moves the source author String directly into `HashMutation`; no source-plus-derived
author clone remains. Its one-mutation command uses `Vec::from([operation])`, whose fixed backing is
claimed by `record_credit` before store acquisition. `checkpoint` must preserve the authored VCS
shape (`Author.id` and `Author.name` both carry the actor identifier), so its one unavoidable ID
clone is explicitly preflighted for every author. The source `Vec<ActorId>`, the preallocated
derived `Vec<vcs::Author>`, each source String, and each derived ID String are all simultaneously
covered before conversion starts. Rejection leaves the exact borrowed input owner unchanged.

The selected live route needs no second pool job. The accepted P1r `ArtifactRunner` already owns the actor-turn future, polls it once per UserVisible-lane grant, validates generation/cancellation before mutation, uses a weak one-shot waker, retains rejected jobs, retries through the process timer wheel, and exposes exact result/work/job take-resume-close. The VCS future now composes with that authority rather than running a nested executor.

## Fixtures and Verifier

Direct Rust fixtures cover:

- operation slots at cap and cap plus one;
- nested item cap plus one and retained bytes plus one;
- record derived mutation backing at exact byte cap and cap plus one with exact source-owner witness;
- checkpoint source/output author Vecs and duplicate author IDs at exact byte cap/cap plus one;
- the discriminating checkpoint item boundary: 31 authors admit at 63 items, while 32 authors
  materialize 65 name/ID items and reject with the exact source Vec and first String unchanged;
- both record and checkpoint cohorts of 64 admitted maximum-credit operations filling the exact
  4 MiB process aggregate, with each next owner rejected without consuming or changing it;
- quiet pending with no wake before ownership release;
- FIFO one-shot wake and reservation against a later admission;
- cancellation clearing the exact waiter;
- admission-slot generation ABA;
- production VCS source with zero nested executor, blocking mailbox, loop, or guarded await.

The existing root `📜️script.ts` interactivity verifier reads the production-only VCS module and rejects:

- a reintroduced nested `block_on`, blocking mailbox, thread, runtime, or pool;
- dynamic waiter ownership;
- missing page/item/process admission;
- missing nested request byte preflight;
- a moved record author changed back to an uncredited clone or a dynamic one-mutation Vec;
- missing checkpoint derived-author Vec or per-author ID credit, credit terms declared but not fed
  into `vcs_credit`, and an output Vec not preallocated to the credited source capacity;
- missing checked derived author-ID item credit while every byte-credit term remains intact;
- freshness after mutable store state;
- wake-all or a FIFO wake without atomic reservation;
- a polling loop;
- missing waiter/build/store cancellation handback;
- missing semantic fixtures;
- any CLI census other than 18 production process-entry waits plus the one test-only
  `seed_document` wait, with mutations for both production misclassification and missing test wait.

## Permitted Gate Results

- Scoped `rustfmt --edition 2021 --check --config skip_children=true` over DB engine: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS in deny mode; one existing allowlisted blocking-bridge finding remains and this packet contributes zero findings.
- `bun 📜️script.ts verify interactivity`: PASS with the same exact baseline.
- Production VCS scan: zero `block_on`, `submit_blocking`, `ask_blocking`, thread/pool creation, poll loop, or guarded await.
- Exact remaining engine production bridge scan: seven calls, matching the residual list below.
- DB facade production scan: zero forbidden bridge calls.
- Hub authored production scan: zero `block_on`, `submit_blocking`, or `ask_blocking` calls.
- DB CLI scan: exactly 18 production process-entry `block_on` calls plus one call in the
  `#[cfg(test)]` `seed_document` helper (19 whole-file total).
- Scoped and whole working, staged, and HEAD whitespace checks: PASS.
- The report is included in the scoped HEAD whitespace check: PASS.

## Exact Remaining Engine Residuals

Seven production `db_engine` bridges remain and are deliberately outside this cohesive packet:

1. `replay_history`: direct WAL replay currently authored only from the DB CLI, though the public handle surface could be called elsewhere.
2. `Database::open_with` storage capabilities: Hub/CLI process-start construction.
3. `Database::open_with` catalog read: Hub/CLI process-start construction.
4. `Database::open_with` empty-catalog CAS: Hub/CLI process-start construction.
5. `Database::create_document` catalog CAS while holding the catalog authority: Hub network/product reachable and therefore a later blocking packet.
6. `Database::compact_document`: authored DB CLI process command, with backend compaction latency.
7. `Database::hello`: Hub network reachable and therefore a later blocking packet.

The retained VCS turn still polls compiler-generated `ArtifactStore` futures. A single poll may traverse more than one immediately-ready in-memory state transition; no syscall occurs in this VCS store, but runtime duration has not been measured. P1q's indivisible Fs/Rusqlite syscall latency remains unchanged. Runtime compile evidence, saturation timing, cancellation latency, and interruption ordering remain open. Phase 1 therefore remains open.

## Independent Rejection Remediation — 2026-08-23

The independent Sol audit rejected the first source packet because record/checkpoint conversion
created uncredited derived String and Vec owners and because the report counted one test helper as a
production CLI entry. The live source now moves the record author, reserves the fixed mutation Vec,
reserves both checkpoint Vec backings and the duplicate author IDs, and has direct +1/aggregate
fixtures plus verifier mutations for each former gap. The census is corrected to 18 production plus
one test-only call. This is a source-only remediation for independent re-audit; it does not claim
Phase 1 acceptance.

## Second Independent Rejection Remediation — 2026-08-23

The focused re-audit found that checkpoint byte ownership was exact but its item formula counted
each author only once. `checkpoint_credit` now adds `request.authors.len()` twice through checked
arithmetic: once for the moved `Author.name` String and once for the simultaneously retained cloned
`Author.id` String. A 31-author request proves the exact 63-item accepted boundary; a 32-author
request proves the defective 33-item formula would admit an actual 65-item materialization and is
rejected without changing the source Vec or first nested String owner. The verifier requires both
checked terms and includes a mutation that removes only the derived-ID item addition while leaving
the corrected byte ledger intact. This remains a source-only packet for independent re-audit.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `📜️script.ts`
- this report
