# HT6 — hub suite, the last seven (outcome 2's gate to green)

Slice HT6. Spec: `📓️ht5-hub-suite-last-nine.md` §10–§14 +
`🗑️generated/coordinator-hub-nextest-full.txt` (18:57 run: **321 — 314 passed / 7 red**).
Rule 26 binds me: no `cargo test`/`nextest`/`build` on `-p semio-hub`; `cargo check -p semio-hub
--all-targets` and kernel tests under rule 25's private `CARGO_TARGET_DIR=…/target-ht6` only.

## The seven (18:57)

| # | law | 18:57 symptom | HT6 status |
|---|---|---|---|
| 1 | `…approval_committed_event_reaches_actor_frontier…` | `SIGABRT`; real panic `:495 got Some(Conflict)` | abort **fixed** §1; `Conflict` narrowed §3 |
| 2 | `…sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once` | `:309` `unwrap()` on `Err(Conflict)` | behind law 1's root, §3 |
| 3 | `…terminal_close_waits_for_unpolled_cleanup…` | `:808` `Busy { retained_uses: 1 }` | **root found + fixed** §2 |
| 4 | `quick::…abandoned_pre_witness_request…` | timeout in phase `Preflight` | **narrowed to a lost wakeup** §4 |
| 5 | `checkpoint_publication_route_rejects_stale_or_cross_scope_inputs…` | law-fixture defect | unprovable as written, **re-proven** §5 |
| 6 | `admin_removal_revokes_visible_plan_presence…` | `database shutdown deadline elapsed` | narrowed to shutdown phase `PoolUse`, §2 |
| 7 | `retained_short_admin_request…_is_exact` | `admin-authority-unavailable` (product contract) | **split landed** §6 |

## 1. The abort's next unguarded witness — `SharedRetirement::drop` *(fixed)*

**Measured, from the 18:57 stderr.** After the real panic at `:495`, the second panic is
`♻️retirement/🦀️.rs:339:9` — *"shared value retired before terminal-empty"* — from
`SharedRetirement<SemioDrawingSnapshot>::drop`, reached through the identical chain HT5 named
(`ArtifactStore::drop` → `ArtifactStoreDisplacedRetirements::drop` → `VecDeque<Box<dyn
ErasedSnapshotRetirement>>` → the cursor), itself dropped by `tokio` cancelling
`RetainedGisMapApprovalCommitterV1::spawn_abandoned_request` at runtime shutdown. HT5 guarded four
witnesses in that file (`:96` `Bytes`, `:204` `Sequence`, `:234` `ValueRetirement`, `:269`
`CursorStack`); **four in the same retirement-cursor family were still unguarded and all four sit on
that chain.** Guarded under F1's rule (assertion kept, skipped only while a panic is in flight):

| line | witness |
|---|---|
| `:117` | `Collection<T>` (`Vec<T>` retirement cursor) |
| `:138` | `OrderedMap<K, V>` (`BTreeMap` / `MapDelta` cursor) |
| `:295` | `OwnedRetirement<T>` |
| `:339` | `SharedRetirement<T>` — **the one that aborted** |

`♻️retirement/🦀️.rs` now has **no** unguarded `Drop` assert. The `🧩️composition/🚪️open` and
`🫧️ephemeral` witnesses are not on the store-drop chain and are left alone, as HT5 decided.

## 2. D1 / E2 — the leaked pool use is the **DB I/O backend registry slot's**, not a stray clone *(root found; D1 fixed)*

HT5's `Arc::strong_count(&backend_witness) == 1` assert **passed** in the 18:57 run (the panic is at
`:808`, the assert is at `:806`), so the surviving `WorkerPoolUse` is owned by neither the
`Database` nor any `Arc<DbBackend>` clone. Traced from source:

- `MemoryStorage::new` (`🗄️storage/🦀️.rs:6833`) takes the use and hands it to
  `register_db_io_backend_reserved_with_use` — the use lives in the **process-global**
  `db_io_backend_registry()` slot (`:3074`), not in the `MemoryStorage` value.
- `impl Drop for MemoryStorage` (`:6876`) only calls `retire_db_io_backend` (`:3114`), which sets
  `close_requested = true` and *requests* a close. It does not release the slot.
- The slot's `pool_use` is taken exactly once, at `:3337`, inside the backend close lane step —
  reached only by awaiting `close_db_io_backend` (`:3127`), i.e. by `MemoryStorage::close()`.

So dropping the backend and immediately calling `pool.shutdown()` **always** reports
`Busy { retained_uses: 1 }`. This is the product's declared contract, already pinned by the db law
`db_io_all_five_backend_controls_require_explicit_terminal_close_witness`: every backend control
requires an explicit terminal close witness. The law's teardown simply never performed it.

**Landed** in `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`, with every existing assertion (including HT5's
`strong_count` witness) kept, in all **three** laws in that file whose teardown shuts the pool down:

| law | change |
|---|---|
| `…terminal_close_waits_for_unpolled_cleanup…` | `memory.close().await` between the `strong_count` assert and `pool.shutdown()` |
| `…approval_committed_event_reaches_actor_frontier…` | new `backend_witness` clone + the same terminal close before its `pool.shutdown()` (it never reached that line while it aborted at `:495`) |
| `quick::…abandoned_pre_witness_request…` | the same |

**E2** (`admin_removal…`, `database shutdown: Unavailable("database shutdown deadline elapsed")` at
`🔬️bin-unit/🦀️.rs:3658`) is the same family from the other side but **not** the same call: that
error is `control.interruption_error()`, raised because `Database::shutdown_step`
(`⚙️engine/🦀️.rs:8567`) kept returning `Progress` until the 5 s deadline. Its only unbounded
`Progress` arm is phase `PoolUse` at `:8634` — `Arc::strong_count(self.pool_use) != 1` — whose
clones go to `ArtifactAuthority::spawn_with_pool_use` (`:8385`, `:8438`). Naming which authority
clone survives needs the suite; I did not guess. Note `⚙️engine/🦀️.rs:8661` carries a leftover
`#[cfg(test)] eprintln!("[DEBUG] …")` that never fires for `os-hub` (db is a dependency there, not
under test), which is why E2's teardown is blind — that is the diagnostic the next owner should
make reachable.

## 3. Law 1 — which `Conflict`? *(narrowed from two producers to the verifier arms; not fixed)*

HT5 left two candidates at `🏃️runtime/🦀️.rs`: `:1823` (state not `Publishing`) and `:1832` (the
projected frontier triple). Both are **excluded from source**:

- `verify()` inserts `Publishing` at `:1910` under the same `documents` lock and calls
  `publish_checkpoint` on the very next statement (`:1927`). The law runs one current-thread
  runtime with no concurrent committer, so nothing can displace the state between the insert and
  either `documents.get(key)` at `:1811`/`:1822`. The "not `Publishing`" arms are unreachable here.
- `:1832`'s `projected.head_edit_id != identity.mutation_id` is now **satisfied** by HT5 §1. The
  chain is: the actor's `head_edit_id` is set at `🛢️db/🗿️artifact/🦀️.rs:2455` from
  `record.parent_edit_id()` (`:2410`), which `🗄️durable-group/🦀️.rs:300` reads out of the canonical
  decision pack as the parent edit's own `id` field — and HT5 made `edit_id()` return the stamp.
  The `head_edit_ordinal`/`last_commit_seq` conjuncts are `(1, 1)` against a genesis base, which the
  law's own `:500` asserts.

What remains reachable, in order: `verify()`'s WAL verifier mapping
(`:1901`, `InferenceErrorV1::Conflict | Invalid | Denied → Conflict`), `verify()`'s post-witness
state re-read (`:1923`), and the test publisher's own two `Conflict` returns
(`🔬️unit/🦀️.rs:155` region mismatch, `:160` proposal-state / document-write gate). I did **not**
guess between them: §1's guard makes the next rerun's first reported failure authoritative, and the
law is one line away from reading `Err(Storage)` if the verifier is the arm.

**Law 2** (`…sqlite_prepared_approval_survives_restart…`, `:309` `unwrap()` on `Err(Conflict)`) is
the same `Conflict` taxonomy re-entered after restart and was left to move with law 1.

## 4. Law 4 — the timeout is a **lost wakeup**, not a slow phase *(narrowed; not fixed)*

`quick::…abandoned_pre_witness_request…` fails on `tokio::time::timeout(5s, …)` around
`poll_approval_to_phase` (`🔬️unit/🦀️.rs:1334`), **not** on that helper's own
`"approval did not reach its exact retained cancellation phase"` panic at its 4 096-iteration
bound. Those two exits are different facts. The helper's inner loop is
`while !wake.ready.swap(false) { yield_now().await }` with `ApprovalPollWakeV1`
(`:208`) storing `true` from both `wake` and `wake_by_ref`. A 4 096-iteration overrun would finish
in milliseconds and panic with the other message; a 5 s timeout means the loop is spinning on
`yield_now` because **the first `Pending` poll of `commit` armed the supplied waker nowhere**. So
the defect is a lost wakeup on the `Preflight` turn (the first of the three phases the loop walks),
not a state-predicate mismatch. Confirming which awaited primitive drops the waker needs the suite.

## 5. Law 5 — E3 confirmed unprovable as written; the rewrite it needs *(not landed)*

I re-derived HT5 §8 and it holds, with the extra fact that the directory has **no** command that can
change or withdraw a live document's descriptor: the full `DirectoryCommand` set is
`AdvanceRetention, AnnounceDocument, ArchiveSpace, CreateInvite, CreateSpace, DeleteSpace,
RemoveMember, RenameSpace, RevokeInvite, SetVisibility, UpsertMember`, and `AnnounceDocument`
(`📇️directory/🦀️.rs:2106-2114`) answers an equal descriptor with a no-op and any other descriptor
with the exact `Conflict` the law's `:7057` `.expect` explodes on. Descriptors are therefore
write-once per `(space, document)`, so the final fence's
`descriptor.as_ref() == Some(&self.descriptor)` (`🏗️bootstrap/🦀️.rs:3821`) cannot be falsified by
replacement during materialization — the step the law performs is impossible by design.

I did **not** land a rewrite, because every available substitute changes *which* fence conjunct is
exercised and I cannot run the suite to confirm it: archiving or deleting the space during
materialization makes `get_document_descriptor` return `None`, but it also flips the first conjunct
(`subject.revalidate(…) != Active`), so the law would silently start proving the authority fence
instead of the descriptor fence. The honest rewrite is the one HT4 and HT5 both named — a **second
announced document** whose descriptor digest the command carries — and it belongs to an owner who
can rerun between attempts. The law's earlier `cross` segment (409 on another space's scope) is
already green and untouched.

## 6. Law 7 — E1 split exactly as HT5 §13 describes *(landed)*

HT5's reading is confirmed by the 18:57 numbers: the law's own `second_response.status == 200`
**passed** and the failure is one assertion later — the competing writer's terminal audit row is
`cancelled` / `admin-authority-unavailable` (a 200 receipt carrying a non-success state). The
competing writer was asked to contend for `ADMIN_OPERATION_DEADLINE + 2 s` (measured 10.18 s)
against a uniform 2 s binding-gate budget that three call sites map to 503, so the refusal is the
**correct** product answer. Corroboration from the law's own fixture: the frozen case
`admitted-writer-outlives-http-deadline`
(`📇️directory/🧫️fixtures/🏛️retained-short-admin/🔣️.json:53`) declares `writerEntries: 1`,
`effectCount: 1`, `terminalCount: 1` — **one** writer. The second writer on that scope was never in
the fixture's description of the case.

**Landed** in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, one segment split into two, with **every**
assertion the law made today preserved and moved to the claim it actually belongs to:

| claim | scope | assertions |
|---|---|---|
| (a) a competing same-scope writer is **blocked, not refused**, and succeeds when its predecessor finishes | new `retained-short-admitted-queued` | the 50 ms "cannot pass retained authority" block; grant count 0 while blocked; `second_response.status == 200`; both audit rows length 2; both terminal phases `succeeded` with their existing diagnostic messages; `first_rows[1].sequence < second_rows[1].sequence`; grant count 2 |
| (b) the HTTP waiter expires at its own deadline **without cancelling its admitted writer** | `retained-short-admitted-deadline`, one writer | `status == 503` after `ADMIN_OPERATION_DEADLINE + 2 s`; grant count 0 at the 503; the retained writer's audit terminal `succeeded`; grant count 1 after release |

(a) releases the predecessor's effect pause immediately after the blocked check, so the successor
contends for ≈ 50 ms — inside the 2 s contract — which is the only regime in which the claim is
provable. (b) needs no second writer at all, which is what makes it provable. The fixture file is
**not** touched; the `cases` count assertion (15) is unchanged. Nothing was deleted, `#[ignore]`d or
loosened, and the 2 s budget stays a product contract.

## 7. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0, 0 errors**, 6 warnings (expansion completed) | `🗑️generated/ht6-check-kernel-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-hub --all-targets` (§1, §2) | **EXIT 0, 0 errors**, 55 warnings | `🗑️generated/ht6-check-hub-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-hub --all-targets` (after §6's split) | **EXIT 0, 0 errors**, 55 warnings | `🗑️generated/ht6-check-hub-2.txt` |

Rule 26: no hub test/nextest/binary build ran from this slice. Both crates were left green in the
same tool round they were edited (GM1's trusted-catalog bootstrap compiles them); no gis or stdio
plugin source was touched at all.

## 8. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs` — §1's four `Drop` guards
  (`:117`, `:138`, `:295`, `:339`).
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §2's terminal memory-backend close in the
  three laws whose teardown shuts the worker pool down (plus two `backend_witness` clones).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — §6's split of the retained-admin law's last segment.

Nothing was deleted, `#[ignore]`d or loosened. No fixture JSON changed.

## 9. Honest gaps

- Everything here is source-level plus a compiling tree. **No hub law was observed passing**;
  rule 26 forbids this slice from running the suite.
- Laws 1, 2, 4, 5 and 6 have narrowed roots (§3, §4, §5, §2's E2 paragraph) but **no landed fix**.
- §2's fix is verified only by `cargo check`; the claim that `MemoryStorage::close()` releases the
  registry slot's pool use is read from `🗄️storage/🦀️.rs:3337` + `:3127`, not measured.

**needs hub rerun.** Expect: the `SIGABRT` at `(147/321)` to become a reported `FAIL` at `:495`
(§1); `terminal_close…` to pass or to fail on a *different* line than `:808` (§2);
`retained_short_admin…` to pass both split claims (§6). Laws 2, 4, 5 and `admin_removal…` stay red
with §3–§5's roots.

---

# HT6 — batch 2 (rerun 19:22: **321 — 316 / 5, no SIGABRT**, from 314 / 7)

§1's four drop guards removed the abort, §2's terminal backend close made
`terminal_close_waits_for_unpolled_cleanup…` **GREEN**, and §6's split made
`retained_short_admin_request…` **GREEN**. Both roots were right.

## 10. Law 5 — E3 rebuilt the way a real stale client holds it *(landed)*

The `descriptor_swap` segment is gone; nothing announces, replaces or restores a descriptor any
more. Measured first: `checkpoint_publication` admission validates the command's
`descriptor_digest_v1` against this scope's live descriptor at `🏗️bootstrap/🦀️.rs:4073-4079` and
answers **409** with refusal code `DescriptorDigestDiffers` *before* `ensure_document` (`:4082`) and
therefore before the publication gate — so both new inputs are synchronous and cannot deadlock the
`checkpoint_publication_admitted` / `_release` permit balance the segment used to consume (one
acquire + one release, both removed together).

| input | how a real client gets it | assertion kept |
|---|---|---|
| **cross-scope**: the command carries `other_descriptor` — the *second* document the law already announces in `other_space` — posted to this scope's route | a client that cached the other space's copy of the same document id | `409` + `artifact_checkpoint_count == 1` |
| **stale checkpoint**: `fixture.command` (built against the pre-`checkpoint-fence-edit-2` snapshot) re-posted under a fresh correlation id | a client holding an older checkpoint of *this* document | `409` + `artifact_checkpoint_count == 1` |

Both refusals the segment asserted are kept (status and checkpoint count), on inputs the product can
actually produce. `other_descriptor` is now cloned into its announce so the law can reuse it.
`📇️directory/🧫️fixtures/…` is untouched.

## 11. Law 3 — the lost wakeup narrowed one level further *(not fixed)*

The driver is `commit_retained` (`🏃️runtime/🦀️.rs:1987-2031`). Its loop wakes correctly —
`Continue`/`Committed` use `tokio::task::yield_now()`, and `Preflight` awaits
`handle.checkpoint_publication_snapshot()`, which the WorkerPool wakes. The law enters the phase
loop with a **free gate and an empty document map** (`wait_for_abandoned_prepared_handoff` requires
`document_absent && gate.try_lock().is_ok()`), so the parked poll is inside
`prepare_retained_document(…).await` at `:1999`, *before* the loop — which is also why the state
the helper waits for (`Ready { pending: Some, document_write: Some }`) is never reached.
That single `await` is where the waker goes missing; the deterministic law the coordinator wants
belongs on `prepare_retained_document`, polled with a counting waker, and I did not write it blind
because I cannot run it. Note the `Preflight` observation window is itself one statement wide
(`drive_turn` returns `Preflight`, `finish_preflight` moves the state to `Assembly` two lines
later), so even once the wakeup is fixed the helper may need the phase predicate to widen.

## 12. Law 6 — made self-diagnosing instead of guessed at *(landed; root still open)*

`admin_removal…` fails with a bare `Unavailable("database shutdown deadline elapsed")`, and the only
diagnostic that existed was a `#[cfg(test)] eprintln!("[DEBUG] …")` at `⚙️engine/🦀️.rs:8661` whose
witness, `ArtifactAuthority::shutdown_debug_witness`, is itself `#[cfg(test)]` — so for `os-hub`,
where the kernel is a dependency and not under test, **neither ever compiles**. That is why five
reruns have produced no evidence for this law.

**Landed:** the `[DEBUG]` block is deleted (rule 10) and the facts it printed now travel in the
error every caller already receives. `DatabaseShutdownControl::interruption_error` takes a new
`DatabaseShutdownInterruptionWitness` and a deadline expiry now reads *"database shutdown deadline
elapsed in phase `PoolUse`: open artifacts N, version graph complete B, emit started B, retained
pool-use owners N"*, with the phase carried from the last `Progress` the loop saw. Cancellation
still answers `DbError::Closed` unchanged. The next rerun therefore separates the two candidates I
could only list before — phase `Authority` (a mount that will not close) versus phase `PoolUse`
(`⚙️engine:8634`, a surviving clone from `ArtifactAuthority::spawn_with_pool_use` at `:8385`/`:8438`
or from `DatabaseCreateCatalogFuture`, whose use is released only at `:7215`) — and
`pool_use_owners` names how many clones survive.

## 13. Laws 1 and 2 — unchanged, and why I did not guess again

Law 1 still reads `got Some(Conflict)` at `:496`. My §3 exclusions still hold (the `Publishing`
arms are unreachable in a single-threaded fixture; the frontier triple's `head_edit_id` conjunct is
satisfied by HT5's stamp), and `commit_prepared_approval` maps `GisMapApprovalCommitErrorV1` to the
route error **1:1** (`🏃️runtime/🦀️.rs:3061-3066`), so the `Conflict` is raised inside
`committer.commit`. The reachable arms are `verify()`'s WAL-verifier mapping (`:1901`), `verify()`'s
post-witness state re-read (`:1923`), `commit_retained`'s preflight generation check (`:2014`) and
the test publisher's own two `Conflict` returns. Separating them needs one run; ranking them again
from source would add nothing this slice has not already written down. Law 2 is the same taxonomy
re-entered after restart.

## 14. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-hub --all-targets` (§10) | **EXIT 0, 0 errors** | `🗑️generated/ht6-check-hub-3.txt` |
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-framework-os-kernel --all-targets` (§12) | **EXIT 0, 0 errors** | `🗑️generated/ht6-check-kernel-3.txt` |
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-hub --all-targets` (§12) | **EXIT 0, 0 errors** | `🗑️generated/ht6-check-hub-4.txt` |

Kernel and hub were both left green in the same tool round they were edited; no gis, stdio or other
plugin source was touched in either batch.

Files changed in batch 2: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (§10),
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` (§12).

**needs hub rerun.** Expect law 5 (`checkpoint…rejects_stale_or_cross_scope_inputs…`) to go green on
§10, and law 6 (`admin_removal…`) to keep failing but now **name its phase and its retained
pool-use owner count** in the panic text — that string is the whole root. Laws 1, 2 and 3 are
unchanged and need one run each to separate their remaining candidates (§11, §13).

---

# HT6 — batch 3 (rerun 19:32: **321 — 317 / 4**, from 316 / 5)

§10 made `checkpoint_publication_route_rejects_stale_or_cross_scope_inputs…` **GREEN** — the
cross-scope descriptor and the older checkpoint were the right inputs, and no fixture literal moved.

## 15. Law 6 — §12's read-out named the leak in one hop *(root found and fixed)*

The new error text is the whole diagnosis:

> `database shutdown deadline elapsed in phase Some(PoolUse): open artifacts 0, version graph
> complete true, emit started true, retained pool-use owners 3`

Every authority is closed (**0** open artifacts) and the graph and emit phases are done, yet the
`Database`'s own `pool_use` has **3** owners where `⚙️engine/🦀️.rs:8634` requires 1 — so two clones
survive with no artifact to hold them. Only four types ever clone it, and three of them release on
the completion-consuming path (`DatabaseCapabilityOpenFuture` `:1008`,
`DatabaseCatalogReadFuture` `:2346`, `DatabaseCatalogBootstrapFuture` `:2802`/`:5646`).

**`DatabaseCreateCatalogState::release_success` (`:7223`) had zero call sites** — the one sibling of
four that nothing called. Its `Future::poll` set `resolved = true` on both `Ready` arms, and
`Drop` (`:7425`) returns immediately when `resolved`, so the terminal retirement that takes
`pool_use` (`:7215`) and clears the process-global `database_create_catalog_registry()` slot
(`:7217-7220`) **ran only for a future that was abandoned before it resolved**. Every catalog
publish that actually completed leaked one clone and one registry slot.
`admin_removal…after_sqlite_reopen` publishes the mount catalog on the reopen path
(`Self::publish_mount_catalog`, `:8416`/`:8381`), which is exactly the resolved path — two of them,
hence `1 Database + 2 leaked = 3`.

**Landed:** `DatabaseCreateCatalogFuture::poll` now calls `self.state.release_success()` on both
`Ready` arms, mirroring `DatabaseCapabilityOpenFuture::take_completion` (`:1003-1008`). The call is
a scheduled, idempotent close — `retire_terminal_one` takes the `Retire` branch while the caller
still owns result owners and only reaches `:7215` once they are empty — so it cannot tear down a
live transaction. This is a **live-hub defect**, not a test artefact: any long-running hub leaks one
`WorkerPoolUse` and one global registry slot per document catalog it ever creates.

## 16. Hand-over — the three still red

| law | printed reason | root as far as it is proven | file:line to start at |
|---|---|---|---|
| `…approval_committed_event_reaches_actor_frontier…` | `:496` `got Some(Conflict)`, wants `Storage` | inside `committer.commit` (route mapping is 1:1); the `Publishing` arms and the frontier triple are excluded (§3, §13); four arms remain | `🏃️runtime/🦀️.rs:1901` verifier mapping, `:1923` post-witness re-read, `:2014` preflight generation, test publisher `🔬️unit/🦀️.rs:155`/`:160` |
| `inference::sqlite::…survives_restart…` | `:309` `unwrap()` on `Err(Conflict)` | the same `Conflict` taxonomy re-entered after restart; no independent evidence | `🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309` — expect it to move with law 1 |
| `quick::…abandoned_pre_witness_request…` | `:1334` 5 s timeout, phase `Preflight` | lost wakeup: the 5 s outer timeout (not the helper's 4 096-bound panic) means one `Pending` armed the supplied waker nowhere; the law enters with a free gate and empty document map, so the park is in `prepare_retained_document(…).await` before the driver loop | `🏃️runtime/🦀️.rs:1999`; also widen the helper's `Preflight` predicate — the window is one statement wide (`:1416` → `:2016`) |

## 17. GM1's `document-open.invalid-fields` — not from this slice

`documentOpenObject` (`📇️directory/🧬️schema/🟦️.ts:1310-1315`) is a **closed**-object check: it throws
`document-open.invalid-fields` when the payload carries *any* key outside `required ∪ optional`, or
is missing a required one, and it is used for the plan **and every nested object**. I diffed the
twins field-for-field and found **no drift**: `DocumentOpenPlanV1` 15 fields (`🦀️.rs:1622` vs
`🟦️.ts:1379`), `checkpoint` 4 (`:1404`), `baselineFrontier` 5 (`ArtifactFrontier`), `scope` 2,
`surface` 5, `grant` 3, `artifact` 3, `parentDialect` 3, `catalog` 1, `package` 7 +
`executionProtocol` 1, `revalidation` 2 + 2 optional, and the `browserActor` twin in
`🌐️browser-actor/`. So no field was added or renamed by today's hub work — nothing in my four files
touches a `ToValue`/`FromValue` type at all — and I cannot attribute it to HT4's `head_seq` pin
either, since `ArtifactFrontier`'s five fields are in twin. The next step is to print the offending
payload's key set at `🟦️.ts:1314` (the throw knows both sets and reports neither), which turns this
from a guess into one line of evidence.

## 18. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht6-check-kernel-4.txt` |
| `CARGO_TARGET_DIR=…/target-ht6 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht6-check-hub-5.txt` |

Files changed in batch 3: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` (§15, two lines).

**needs hub rerun.** Expect `admin_removal…` to go green on §15; if it does not, the same error now
prints the new owner count, which is the next hop. Laws 1, 2 and 3 are unchanged (§16).
