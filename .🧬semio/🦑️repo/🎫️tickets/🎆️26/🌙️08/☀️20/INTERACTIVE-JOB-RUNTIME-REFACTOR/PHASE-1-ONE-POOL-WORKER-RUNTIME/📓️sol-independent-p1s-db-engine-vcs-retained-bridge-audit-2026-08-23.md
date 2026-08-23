# Sol Independent Audit — P1s DB Engine VCS Retained Bridge

Date: 2026-08-23

## Verdict

**REJECT — source packet.** The retained per-document store arbitration and the removal of the five
VCS-local executor bridges are structurally sound, but the admitted byte authority does not cover
all owners created by the live record/checkpoint conversion. The P1s production CLI census is also
off by one. This rejects P1s source acceptance as currently evidenced; it does not reverse the
independently accepted P1r retained-submit cohort.

Terra admission for this P1s audit was scheduler-blocked. This report is an independent Sol review;
no prior P1s implementation authorship or Terra verdict was used as acceptance evidence.

No production source was edited. Cargo, Nx, compilation, native/Wasm/browser execution, network,
root lint, and runtime timing were not run.

## Evidence Reviewed

- `📓️p1s-db-engine-vcs-retained-bridge-2026-08-23.md`;
- `📓️p1r-artifact-handle-retained-submit-2026-08-23.md`;
- `📓️terra-p1r-artifact-handle-retained-submit-audit-2026-08-23.md`;
- the P1s commit-parent/current DB engine delta, current DB artifact runner, DB facade, DB CLI, Hub,
  and root verifier source; and
- current working, staged, and HEAD diffs.

## Census and Reachability

The stated initial engine census is internally reproducible as **12** production
`db_actor::block_on` calls: the five former VCS methods plus the seven current residuals. The five
deleted VCS calls were `ensure_store`, `record_change`, `checkpoint`, `merge_base`, and `head`.
Current production `vcs_integration` has zero `block_on`, `submit_blocking`, `ask_blocking`, thread,
WorkerPool, runtime, polling-loop, or job construction.

The exact seven remaining production engine bridges are:

1. WAL `replay_history`;
2. `Database::open_with` storage capabilities;
3. `Database::open_with` catalog read;
4. `Database::open_with` empty-catalog CAS;
5. `Database::create_document` catalog CAS;
6. `Database::compact_document`; and
7. `Database::hello`.

The authored DB facade has zero production `block_on`, `ask_blocking`, or `submit_blocking` calls.
Authored Hub production has zero such calls. Its `ensure_document` route awaits
`Database::{document,create_document}`, and its Commands route awaits P1r `ArtifactHandle::submit`.
That retained submission reaches `ArtifactRunner`, `ArtifactEngine::submit`, and
`VersionGraph::record_change` without a nested executor bridge. `Database::checkpoint_document`
reaches the same retained VCS store authority without a nested bridge, although it has no authored
Hub production caller.

The CLI source contains **19 total call expressions**, but only **18 production process-entry
calls**. The nineteenth is inside the `#[cfg(test)]` `seed_document` helper at CLI line 1053. P1s's
claim that the production scan contains exactly 19 process-entry calls is therefore false. All 18
actual production calls are at single-shot CLI boundaries; the test-only nested bridge is outside
production but cannot be counted as a production entry boundary.

## Accepted Retained-Owner Shape

The live VCS authority provides 64 fixed generation-keyed admission slots, 16 KiB pages, four pages
and 64 KiB declared per operation, and 256 pages and 4 MiB declared process aggregate. Checked
addition precedes admission mutation, slot release validates generation/bytes/items, and acquire
freshness is checked before touching per-document store ownership.

Each per-document cell holds the exact `HashStore` in an `Option`. `VcsStoreAcquire` moves it into a
lease or returns a unique build permit; the lease returns the exact store in `Drop`, and a cancelled
build permit releases the exact reserved generation. Waiters occupy a fixed 64-slot array. Release
selects the minimum admitted generation, removes exactly that waiter, stores its generation in
`busy_generation` before waking, and wakes one owner after releasing the mutex. A late acquire
cannot overtake the reserved generation. Dropping a selected-but-unpolled acquire clears that exact
reservation, selects the next generation, reserves it, and wakes only that owner. Slot/generation
freshness prevents ABA reuse from mutating the cell.

The Rust fixtures genuinely exercise operation-slot cap/+1, generic nested byte +1, quiet Pending,
FIFO one-shot wake with late arrival, waiter cancellation, and admission-slot ABA. The source-only
fixture confirms the absence of a nested executor in the production VCS region. The verifier's
mutations meaningfully reject nested `block_on`, dynamic waiters, a missing named nested byte term,
freshness after state access, wake-all, unreserved FIFO wake, missing lease handback, a poll loop,
and a missing fixture.

## Blocking Findings

### 1. Derived String and Vec owners are outside the admitted byte credit

`record_credit` charges `change.author.0.capacity()` once. After admission,
`record_change` clones that String into `HashMutation.author` while the original `ChangeRecord`
author remains live across the awaited store dispatch. It also constructs a new `Vec<HashMutation>`
with `vec![operation]` without charging that backing owner. An accepted request close to 64 KiB can
therefore retain materially more than its declared operation credit, and 64 simultaneous accepted
operations can exceed the declared 4 MiB aggregate.

`checkpoint_credit` charges the source `Vec<ActorId>` backing allocation and each source author
String once. After admission, `checkpoint` constructs a new `Vec<vcs::Author>` and clones every
author identifier into `Author.id` while moving the original String into `Author.name`. Neither the
new Vec backing allocation nor the second String owner is reserved. The same exact-authority gap
exists even though the input Vec and every original nested String were named in the preflight.

This is not a timing-only residual: the source claim that 64 KiB/operation and 4 MiB aggregate cover
retained ownership is false. Owner conversion must either move without duplication or reserve every
derived String and Vec backing owner before admission/mutation. Every overflow/error path must keep
the exact input owner recoverable.

The verifier currently checks only that selected source `capacity()` expressions exist. It accepts
the uncharged `clone()` and derived Vec construction, and its Rust capacity fixture does not build a
near-cap author request through the real conversion. Permanent mutations/fixtures must reject the
clone/derived-owner form and exercise record/checkpoint just below and above exact credits.

### 2. The production CLI count is 18, not 19

The report's production-only methodology and its total disagree. The source has 18 calls before the
test module and one inside the test module. The evidence must state either 18 production + one
test-only or explicitly define a 19-call whole-file census; it cannot classify the test helper as a
production process-entry bridge.

## Gates Run

| Gate | Result |
| --- | --- |
| scoped `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; DENY clean with the recorded test-only allowlist entry |
| `bun ./📜️script.ts verify interactivity` | PASS; same baseline |
| exact current VCS forbidden scan | PASS: zero nested executor/mailbox/thread/pool/runtime/job/loop |
| engine production bridge scan | PASS: exactly seven residual calls listed above |
| DB facade and authored Hub production scans | PASS: zero forbidden calls |
| CLI scan | **FAIL against report wording**: 18 production + one test-only = 19 whole-file calls |
| scoped working/staged/HEAD diff checks | PASS |
| whole working/staged/HEAD diff checks | PASS |
| builds/runtime timing | Not run; no compile or runtime claim |

## Residual Status

P1s remains RED pending exact derived-owner admission and corrected census/verifier evidence. Phase 1
also remains RED on the seven engine bridge groups listed above, P1q's indivisible filesystem/SQLite
syscall latency, compiler-generated future step duration, compilation, saturation/fairness,
cancellation/interruption timing, and the full runtime/thread matrix.
