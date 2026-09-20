# HT7 — hub suite, the final four (outcome 2's gate to green)

Slice HT7. Spec: `📓️ht6-hub-suite-last-seven.md` §15–§18 hand-over +
`🗑️generated/coordinator-hub-nextest-full.txt` (**19:47 run: 321 — 317 passed / 4 failed**,
from 58 red this morning).
Rule 26 binds me: no `cargo test` / `nextest` / `build` on `-p semio-hub`;
`cargo check -p semio-hub --all-targets` and kernel/db checks under rule 25's private
`CARGO_TARGET_DIR=…/target-ht7` only.

## 0. The four, as the 19:47 stderr prints them

| # | law | 19:47 printed reason |
|---|---|---|
| L1 | `inference::runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:496:5` — *"public checkpoint refusal remains a retained nonterminal publication, got Some(Conflict)"* (wants `Storage`) |
| L2 | `inference::sqlite::tests::gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once` | `🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309:169` — `unwrap()` on `Err(Conflict)` |
| L3 | `inference::runtime::tests::quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `🔬️unit/🦀️.rs:1334:156` — 5 s timeout, *"approval reaches its retained cancellation phase Preflight"* |
| L4 | `tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `🔬️bin-unit/🦀️.rs:3658:133` — *"database shutdown deadline elapsed in phase Some(PoolUse): open artifacts 0, version graph complete true, emit started true, **retained pool-use owners 3**"* |

## 1. L3 — the lost wakeup is a **global map lock held across an actor round-trip** *(root found and fixed)*

This is the one real product defect of the four and it is now closed.

**Measured from source, exactly.** `finish_document_recovery`
(`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`, old `:1015`) opened with

```rust
let mut documents = self.documents.lock().await;      // the committer's ONE document map
…
match owner.advance().await {                          // ← awaited while still holding it
```

and `ArtifactDurableGroupRecoveryOwnerV1::advance`
(`🛢️db/🗿️artifact/🦀️.rs:884`) awaits `self.address.ask(Priority::Command, … RecoverDurableGroupDecisions …)`
— a **document-actor mailbox round-trip**, which is `Pending` on its first poll by construction.
So the whole committer's `documents` mutex was held across document I/O.

Why that is exactly this law's 5 s timeout, and not the helper's own 4 096-iteration panic:
`poll_approval_to_phase` (`🔬️unit/🦀️.rs:222`) polls the commit future **by hand** with its own
`ApprovalPollWakeV1` waker and then, on the very next statement, does
`committer.documents.lock().await` to read the phase. On the first mount the commit future parks
inside `advance()` holding that lock; the only poller of that future is the helper, and the helper is
now parked on the lock it holds. Hard deadlock, broken only by the outer
`tokio::time::timeout(5 s, …)` — which is precisely the reported exit, and it fires on the **first**
loop iteration (`Preflight`), the only one that mounts the document. HT3a's WAL `scan`→`Done` work is
the plausible trigger: it made that `ask` park where it previously completed inside one turn.

It is a live-hub defect too, not only a harness one: every approval on **any** document serialised
behind one document's recovery scan.

**Landed** (`🏃️runtime/🦀️.rs`):

| change | what it does |
|---|---|
| `RetainedGisMapDocumentStateV1::Recovery.owner` is now `Arc<tokio::sync::Mutex<GisMapRecoveryOwnerV1>>` | the scan owner lives *in* the map, shared, instead of on the caller's stack |
| `finish_document_recovery` rewritten into three phases | clone the owner handle under the map lock → **release the map lock** → `resume`/`advance` under the owner's own per-document lock → re-take the map lock and install `Recovered`/`Ready`/`Recovery` |

The rewrite is **cancel-safe**: `advance()` retains its boxed `ask` future in the owner (it clears
`self.request` only after the await returns), and the owner is now owned by the map, so a commit
future dropped mid-scan loses nothing and the next `finish_document_recovery` re-polls the same
request. Every guard, every error taxonomy and every inserted state is unchanged.

I verified from source that this was the **only** such site: a scripted scan of every
`self.documents.lock().await` region in that file found no other `.await` under the guard
(`drive_turn` at `:1391` holds the lock but is fully synchronous, `mount_document` takes its actor
snapshot *before* locking).

**Also landed**, in the helper (`🔬️unit/🦀️.rs:239`): its phase read is now
`committer.documents.try_lock().expect("a retained approval never parks holding the committer
document map")`. In a current-thread runtime nothing can hold that mutex between a synchronous
`poll()` return and the next statement *unless* it parked holding it — so this converts any
recurrence of the defect from an opaque 5 s timeout into a named assertion, which is the
diagnosability HT6 §11 wanted from widening the `Preflight` predicate. I did **not** widen the
predicate: with the deadlock gone the window is genuinely reachable — `drive_turn`'s `Ready` arm
(`:1412`) inserts `pending: Some(candidate)` and returns `Preflight`, and `commit_retained` then
awaits `handle.checkpoint_publication_snapshot()`, a real actor round-trip that parks with the state
at exactly `Ready { pending: Some(_), document_write: Some(_) }`. Widening would have made the law
cancel a turn earlier and prove something weaker.

## 2. L1 / L2 — two typed refusal vocabularies, wire unchanged *(HT3b's technique; not a fix)*

Both laws die on an opaque `Conflict` and both taxonomies are 1:1 with no body, so five reruns have
produced no evidence. Ranking the arms from source again would add nothing HT5/HT6 have not already
written down (`📓️ht6…` §3, §13). Instead every reachable arm now **names itself**, exactly as HT3b
did for the checkpoint/plan refusals.

### 2.1 `GisMapCommitConflictArmV1` — the runtime committer (`🏃️runtime/🦀️.rs`)

23 variants, recorded through `gis_map_commit_conflict(arm)`, which records only under `cfg(test)`
and returns the **identical** `GisMapApprovalCommitErrorV1::Conflict` in production. Instrumented:
`finish_preflight`'s 3 arms, `publish_checkpoint`'s 6, `verify`'s 8 (including `:1901`'s WAL-verifier
mapping and `:1923`'s post-witness state re-read), `commit_retained`'s `:2014` preflight generation
check and its undo twin, and the test publisher's own 3 (`🔬️unit/🦀️.rs:155`, `:160`, `:192`).
Two eager sites were converted to lazy (`ok_or` → `ok_or_else`, `map_or` → `map_or_else`) so a
success never records a refusal.

L1's assertion at `:496` now prints `… got {:?} refused as {:?}`. **One rerun names the arm.**

On the spec's question — whether today's changes turned a storage fault into a conflict by
re-verifying a half-written log: the injected fault is the publisher's `attempt == 0` →
`Err(Storage)` (`🔬️unit/🦀️.rs:162`), and it is guarded by two `Conflict` arms *before* it
(`has_expected_region != (attempt < 2)` at `:155`, and `proposal_state`/`document_write` gate at
`:160`). Those two are now `PublisherRegionDiffers` and `PublisherProposalStateOrGate`; if either is
what the rerun prints, the fault never fired and the defect is upstream of the publisher, in the
snapshot the committer assembled. If the rerun prints a `Verify*` arm instead, `verify()` refused
before `publish_checkpoint` ran at all. The two hypotheses are now one line of output apart.

### 2.2 `ApprovalReconciliationConflictV1` — the sqlite ledger (`🪶️sqlite/🦀️.rs`)

**L2 is not L1's taxonomy.** HT6 §16 assumed it was; it is not. `:309` unwraps
`InferenceJobLedgerV1::reconcile_committed_approval`, which returns `InferenceErrorV1`, and the law's
*previous* line already asserts that the same call with generation `18` **is** `Err(Conflict)` — so
the two laws share a symptom, not a producer, and L2 will **not** move with L1.

The function has five distinct `Conflict` guards collapsed into one value. All five now record:
`WitnessOrFrontierDiffers` (the `witness.matches` / `after_frontier` fence — the one HT5's
`stamped_mutation_id()` and HT4's document-clock stamp could have moved, since it compares
`after_frontier.head_edit_id` against the outbox's `accepted_mutation`), `CommittedUndoRowMissing`,
`OutboxPhaseNotPrepared`, `JobOrProposalPhaseDiffers`, `OutboxCommandDiffers`. The law's `.unwrap()`
became an `unwrap_or_else` that panics with `… got {error:?} refused as {:?}`. **One rerun names the
guard.**

## 3. L4 — the owner count did not move, and why *(HT6's premise corrected; guard + census landed)*

**First fact: `retained pool-use owners` is still 3**, unchanged from the 19:32 reading HT6 §15
diagnosed. The reason is in the source: HT6 reported that
`DatabaseCreateCatalogState::release_success` had *zero call sites*, but it already had one —
`DatabaseCreateCatalogResult::into_parts` (`⚙️engine/🦀️.rs:5646`), which is exactly the call
`publish_mount_catalog` (`:8389`) makes on the mount-catalog path. HT6's addition in
`DatabaseCreateCatalogFuture::poll` is therefore idempotent and harmless, but it could not change the
number: that release was already happening.

I audited the four `release_success` siblings. Three retire **synchronously**
(`DatabaseCapabilityOpenState:966` and `DatabaseCatalogReadState:2321` retire under a lease;
`DatabaseCatalogBootstrapState:3710` takes its admission and clears its registry slot in-line).
`DatabaseCreateCatalogState:7223` is the only one that is *asynchronous* — it sets `closing`, sets
`wake_requested` and `schedule()`s, and the `WorkerPoolUse` is taken far away, in
`retire_terminal_one` (`:7215`), which `drive_claimed` reaches only through
`closing && phase() == Terminal` (`:6460`).

**Landed guard** (`⚙️engine/🦀️.rs`, `retire_intermediate_one`'s terminal transition): when the
cursor is fully drained the phase returns to `Terminal` **if `closing` is set**, instead of
unconditionally to `Publish`. Every `closing` setter in that state
(`release_success:7224`, `DatabaseCreateCatalogResult::drop:5683`, the terminal handle at `:7453`)
runs only after a completion exists, so a closing state re-entering `Publish` can only reach
`publish_one`'s catch-all — which re-stages `outcome = Some(Err(…))`, permanently falsifying
`roots_are_empty()` and pinning the pool use in a `Publish`↔`Retire` ping-pong on the I/O lane. The
guard makes that path unreachable. **I do not claim this is the observed leak**: on the happy path
`Retire` already runs *before* `Publish`, so the cursor is normally empty when `Terminal` is reached
and `retire_terminal_one` completes in one step. It is a correct invariant, not a proven root.

**Landed diagnostic — the decisive instrument.** `Arc::strong_count` names a number, never an owner,
which is why three reruns have produced "3" and no name. The shutdown interruption witness now
carries a `DatabaseRetainedPoolUseCensus`: the count of occupied slots in each process-global
registry whose state holds a `WorkerPoolUse` clone — `capability_open`, `catalog_read`,
`catalog_bootstrap`, `create_catalog` (`⚙️engine/🦀️.rs`) and `sync_hello`
(new `db_sync::database_sync_hello_live_slots()`, `🔄️sync/🦀️.rs`). nextest runs one process per
test, so those counts are this law's alone. The error now reads
`… retained pool-use owners N, retained slots DatabaseRetainedPoolUseCensus { … }`.

Two families are already excluded from source: the hub never calls sync-hello at all (`grep` over
`🌎️hub/` outside tests: zero hits), and capability-open/catalog-read retire synchronously on the
completion-consuming path. **If `create_catalog` is non-zero the guard above is the root and its
count is the leak; if every family is zero the surviving clones are not registry-held at all and the
next hop is `ArtifactAuthority::spawn_with_pool_use` (`:8403`/`:8456`) or the boxed
`run_open_document_mount` resume future at `:8460`, which carries a `pool_use.clone()` into a future
that is only polled if the caller retries.** One rerun decides it.

## 4. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-hub --all-targets` (§1, §2) | **EXIT 0, 0 errors**, 307 warnings (the warnings prove the expansion type-checked) | `🗑️generated/ht7-check-hub-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-hub --all-targets` (after §3) | **EXIT 0, 0 errors**, 307 warnings | `🗑️generated/ht7-check-hub-2.txt` |
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0, 0 errors**, 18 warnings | `🗑️generated/ht7-check-kernel-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-framework-os-kernel-db --all-targets` | **EXIT 101** — one **pre-existing peer breakage**, not mine (see below) | `🗑️generated/ht7-check-db-1.txt` |

The db-crate failure is `E0432: unresolved import semio_framework_pack` at
`🛢️db/📦️packages/🦀️rust/🦀️.rs:12`, in that crate's **`lib test`** target only; its `lib` target
compiled (65 warnings) and `semio-hub` links it, so my two db edits are green. That file is not one
of mine and the import is unrelated to the engine/sync change — a sibling is mid-flight on the
`semio-framework-pack` `corruption-testing` feature (rule 3/13). Consequence for this slice: I could
not run the db engine's own unit laws, so §3's guard is verified by `cargo check` only.

Rule 26: no hub test/nextest/binary build ran from this slice. No gis or stdio plugin source was
touched. Nothing in `🗑️generated` that I did not create was removed.

## 5. Files changed

- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — §1's `Recovery.owner` shape change + the
  `finish_document_recovery` rewrite; §2.1's `GisMapCommitConflictArmV1`, its process-local slot,
  `gis_map_commit_conflict` / `last_gis_map_commit_conflict_arm`, and 20 instrumented arms.
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §1's `try_lock` guard in
  `poll_approval_to_phase`; the test publisher's 3 instrumented arms; L1's assertion prints the arm.
- `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` — §2.2's `ApprovalReconciliationConflictV1`, its slot,
  recorder and reader, and the five instrumented guards of `reconcile_committed_approval`.
- `🌎️hub/💡️inference/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs` — L2's `:309` prints the guard.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` — §3's `closing → Terminal`
  guard and the `DatabaseRetainedPoolUseCensus` in the shutdown interruption witness.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` — `database_sync_hello_live_slots()`.

Nothing was deleted, `#[ignore]`d or loosened. No fixture JSON changed. No law lost an assertion.

## 6. Honest gaps

- **No hub law was observed passing.** Rule 26 forbids this slice from running the suite; everything
  here is source-level plus a compiling tree (`semio-hub` and `semio-framework-os-kernel` both
  EXIT 0 in the same tool round as their edits).
- §1 is the only **landed root fix** of the four. §2 is diagnosis, not repair: L1 and L2 will still
  be red on the next rerun, but each will print the arm/guard that refused, and that string is the
  whole remaining root.
- §3's guard is an invariant, not a proven root; §3's census is what actually decides L4.
- L2's producer is **different** from L1's (§2.2) — HT6 §16's "expect it to move with law 1" is
  wrong, and the two need separate roots.
- I could not run the db crate's own unit laws (§4's peer breakage), so the db-side edits are
  `cargo check`-verified only.

**needs hub rerun.** Expect: L3 (`quick::…abandoned_pre_witness_request…`) **green** on §1 — or, if
the defect recurs anywhere else, a named `"a retained approval never parks holding the committer
document map"` panic instead of a 5 s timeout. L1 and L2 still red, each now printing
`refused as <arm>` / `refused as <guard>`. L4 still red, now printing `retained slots
DatabaseRetainedPoolUseCensus { capability_open: _, catalog_read: _, catalog_bootstrap: _,
create_catalog: _, sync_hello: _ }` — that one struct is the whole diagnosis (§3).

---

# HT7 — batch 2 (rerun 20:09: **321 — 317 / 4**; every instrument spoke)

The count did not move, but the four laws are no longer opaque. What the 20:09 stderr prints:

| law | new evidence |
|---|---|
| L1 | `got Some(Conflict) refused as Some(PublisherRegionDiffers)` |
| L2 | `got Conflict refused as Some(WitnessOrFrontierDiffers)` |
| L3 | **moved** — `🔬️unit/🦀️.rs:269` *"abandoned approval returns every Store, ingress and document writer: Elapsed(())"*, plus a second panic `🗄️durable-group/🦀️.rs:1498` *"durable fixed-three Map assembly reached Drop before mounted or terminal owner handoff"* |
| L4 | `retained pool-use owners 3, retained slots DatabaseRetainedPoolUseCensus { capability_open: 0, catalog_read: 0, catalog_bootstrap: 0, create_catalog: 0, sync_hello: 0 }` |

## 7. L4 — the census excluded all five, and named the sixth family from source *(root mechanism found)*

**Every registry is 0**, so the three surviving clones are not registry-held — which falsifies HT6 §15
and my own §3 guard as the root, and sends the search where §3 said it would: the authority side.

`impl Drop for ArtifactAuthority` (`🛢️db/🗿️artifact/🦀️.rs:5575`) is the shape:

```rust
if self.handoff.terminal.load(Acquire) { return; }
…
retirement.commit(close, self.handoff.clone(), self._pool_use.clone());
```

A **non-terminal** authority drop hands *another* `WorkerPoolUse` clone to
`ArtifactRunnerRetirementReservation::commit` (`:4792`), which parks it in the process-global
`ARTIFACT_RUNNER_RETIREMENTS[index]` cursor (`:4795`, field `_pool_use`) until the
`artifact_runner_retirement_step` maintenance hook drains it. So an authority can be gone from
`open_artifacts` (hence **0**) while its pool use lives on in a retirement cursor. Two documents
closed non-terminally on the sqlite reopen path = 2 parked cursors = `1 Database + 2` = the
observed **3**.

**Landed**: `db_artifact::artifact_runner_retirement_live_slots()` (`🗿️artifact/🦀️.rs`) and an
`artifact_retirement` field in `DatabaseRetainedPoolUseCensus`. The next rerun's census decides
between "the retirement cursors are parked" (non-zero ⇒ the shutdown must drain them, or must close
its authorities terminally so `Drop` takes the early return and never parks one) and "nothing
process-global holds them" (zero ⇒ the clones are on a live `ArtifactHandle` the hub itself still
owns at shutdown).

## 8. L3 — the deadlock fix held; the law advanced to its next claim *(progress, not green)*

§1 was right: `poll_approval_to_phase` no longer times out and no `"never parks holding the committer
document map"` assertion fired, so the commit future now reaches its cancellation phases. The law
died **35 lines later**, at `wait_for_abandoned_approval_handoff` (`:269`) — a different claim, about
the *abandoned* handoff rather than reaching the phase. The companion
`durable fixed-three Map assembly reached Drop before mounted or terminal owner handoff` proves an
`Assembly` state was still in the document map at teardown, so the stall is in the **Assembly**
iteration of the three-phase loop: `drive_abandoned_turn`'s `Assembly` arm (`🏃️runtime:1641`) calls
`owner.cancel()` then `advance(one-item grant)` and never returns `Mounted` or `Terminal` — either
spinning on `Ok(_)`→`Continue` or looping on `Err(_)`→`Retry` with its 1 ms→1 s backoff, both of
which exhaust the helper's 5 s budget. Those two are different defects and the helper discarded
which one it was.

**Landed**: the helper's `.expect` became an `unwrap_or_else` that prints the stuck document state
and all four of its conditions — `state Assembly | Journal receipt=… | Ready pending=… document_write=…`,
`maintenance_owns`, `gate_free`, `ingress_released`. One rerun says which turn is spinning and
whether the gate/ingress were already returned.

## 9. L1 — `PublisherRegionDiffers` is the publisher's *first* guard, before the injected fault

The injected storage fault is `attempt == 0 → Err(Storage)` at `🔬️unit/🦀️.rs:162`; the arm that
fired is the guard three statements earlier (`:155`), so **the fault never ran**. The publisher
decodes `request.pair.pack` — the parent Store's `snapshot_pack()` taken in `publish_checkpoint`
(`🏃️runtime:1843`) — and demands a region `inference-<job_id>` carrying `data.id == <same>` and
`data.kind == "inference-bounds"`. The producer of that region is
`GisMapInference::create_region_group_work` (`✏️s/🔌️plugins/🌍️gis/…/💡️inferences/🦀️.rs:90`, whose
`bounds_proposal` builds exactly those three fields) — **gis plugin source, which my slice is
forbidden to edit**, so I did not touch it and the fix, if it is there, belongs to the catalog owner.

That leaves two live hypotheses and they are one line of output apart: the region is **absent** (the
published pack is the pre-mutation snapshot — a store-side staleness that HT4's document-clock stamp
or HT5's `stamped_mutation_id()` could have introduced at `🏪️store/🦀️.rs:16607`), or it is
**present with different fields** (a gis-side rename). **Landed**: the guard now records the full
evidence — `attempt N wanted inference-<job> present=true, pack carried [(id, data.id, data.kind), …]`
— and L1's assertion prints it beside the arm. That string distinguishes the two outright.

## 10. L2 — `WitnessOrFrontierDiffers` was still composite; split into six *(named next rerun)*

The law's own frontier literals satisfy four of the five conjuncts by construction
(`head_edit_id: prepared.mutation_id`, `head_edit_ordinal: selected.head_ordinal + 1`,
`last_commit_seq: selected.last_commit_seq + 1`, `document_id: selected.document_id`), and the law
already asserts `prepared.mutation_id == outbox["mutationId"]` **and passes**, so the suspect is
`witness.matches(...)` — whose witness comes from a *different* fixture
(`🧾️wal/🧪️tests/🔬️unit/🦀️.rs:396 committed_fixture_witness`), and whose eleven conjuncts also
collapsed into one `false`.

**Landed**, both levels:

- `🪶️sqlite/🦀️.rs` — the one composite guard became five: `WitnessDiffers`,
  `FrontierDocumentDiffers`, `FrontierHeadEditIdDiffers`, `FrontierHeadOrdinalDiffers`,
  `FrontierCommitSeqDiffers`.
- `🧾️wal/🦀️.rs` — `CommittedInferenceWalWitnessV1::matches` is now a conjunct table that records
  `CommittedWitnessMismatchV1` (`Scope`, `Generation`, `FenceInactive`, `FenceGeneration`, `JobId`,
  `ProposalHash`, `MutationId`, `CommandHash`, `DecisionHash`, `TransactionId`, `RecordIndex`) and
  returns the identical `false`. `FenceInactive` is my own ranking: the fence is handed back live by
  the fixture helper, and HT3a's WAL `scan`→`Done` work is the only thing that could have moved when
  the verifier retires it.

L2's panic now prints `refused as <guard> / <conjunct>`. **One rerun names both.**

## 11. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors**, 309 warnings | `🗑️generated/ht7-check-hub-3.txt` |
| `CARGO_TARGET_DIR=…/target-ht7 cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht7-check-kernel-2.txt` |

Rule 26 respected; no gis or stdio plugin source touched in either batch; nothing deleted,
`#[ignore]`d or loosened; no fixture JSON changed.

## 12. Files changed (batch 2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs` — §7's
  `artifact_runner_retirement_live_slots()`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` — §7's `artifact_retirement`
  census field.
- `🌎️hub/💡️inference/🧾️wal/🦀️.rs` — §10's `CommittedWitnessMismatchV1`, its recorder/reader and
  the conjunct table in `matches`.
- `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` — §10's five split reconciliation guards.
- `🌎️hub/💡️inference/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs` — L2 prints guard **and** conjunct.
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §9's publisher region evidence + L1's
  printer; §8's stuck-state printer in `wait_for_abandoned_approval_handoff`.

## 13. Honest gaps (batch 2)

- **No law went green this batch.** §1 (batch 1) remains the only landed product root fix; it moved
  L3 forward one claim but the law is still red.
- §7, §8, §9 and §10 are diagnosis. Each replaces a guess with a string the next rerun prints, and
  each names the decision that string makes.
- L1's producer is in `🌍️gis` plugin source, which this slice must not edit. If the rerun shows the
  region is present-but-renamed, the fix belongs to the trusted-catalog owner, not here.
- §3's `closing → Terminal` guard is now known **not** to be L4's root (create_catalog is 0). It is
  a correct invariant and I left it; it costs nothing and closes a real `Publish`↔`Retire` spin.

**needs hub rerun.** Expect all four still red, each now naming its root in one line: L1
`refused as Some(PublisherRegionDiffers); Some("attempt 0 wanted … pack carried […]")`; L2
`refused as Some(<guard>) / Some(<conjunct>)`; L3 `state <X>, maintenance_owns=…, gate_free=…,
ingress_released=…`; L4 `retained slots … artifact_retirement: N`.
