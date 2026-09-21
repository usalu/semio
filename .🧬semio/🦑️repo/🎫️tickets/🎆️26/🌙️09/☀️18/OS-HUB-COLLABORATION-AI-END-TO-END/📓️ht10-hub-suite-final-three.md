# HT10 — hub suite final three (L1 / L3 / L4)

Successor to `📓️ht9-hub-suite-final-three.md` §17–§20. Slice = drive the hub suite from
**318 passed / 3 failed** to green, product-fix first, never loosening a law.

| law | 23:48 run | HT10 batch 1 |
|---|---|---|
| L1 `…gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint…` | `Conflict … prepare conflict Some("Published/stores_match")` at `🔬️unit:676` | **product fix landed** — the `Published` join arm no longer measures a post-commit state against the genesis base pack |
| L3 `…quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `SIGABRT` at 30.039 s; `last phase: awaiting the abandoned handoff after cancellation phase Journal`; `last abandoned turn Some("driver leaving on Complete")` | **named, not yet fixed** — every `Complete`/`Aborted` exit now carries its arm and its false conjunct, and the driver's exit record no longer overwrites it |
| L4 `tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `Database::shutdown` deadline in phase `PoolUse`, `owners 3`, **14 census families all 0** | **census closed** — three new families cover every remaining carrier of `Database::pool_use` |

## 0. The 23:48 capture

`🗑️generated/coordinator-hub-nextest-full.txt` (coordinator's run, 2026-09-20 23:48):
`321 tests run: 318 passed, 3 failed` in 34 s, **no** `semio-pool-worker` stack overflows —
HS1's stack fix holds, and the four wall-clock reds HT9 listed in §17 are gone. The three
remaining reds are exactly this slice's.

Verbatim readings:

- L1 — `fresh request joins the one publication: Conflict refused as None, identity mismatch None,
  prepare conflict Some("Published/stores_match")` (`🔬️unit:676`).
- L4 — `database shutdown deadline elapsed in phase Some(PoolUse): open artifacts 0, version graph
  complete true, emit started true, retained pool-use owners 3, retained slots
  DatabaseRetainedPoolUseCensus { capability_open: 0, … live_mount_future: 0 }` — all fourteen 0.
- L3 — `law hang watchdog: … exceeded 30s; last phase entered: awaiting the abandoned handoff after
  cancellation phase Journal; last abandoned turn Some("driver leaving on Complete")`.

## 1. L1 — `Published` is a POST-commit state, so its join must compare the base, not the stores *(product defect, fixed)*

`prepare_retained_document` has six join arms (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1301–1463`). They
fall into two families:

- `Ready` — a **pre-commit** state. Its owners hold exactly the snapshot a joining request names as
  its base, so `stores_match(owners, decode_pack(base.pack))` is the right precondition.
- `Verification` / `Publishing` / `Assembly` / `Journal` — **post-journal** states. Their owners have
  already advanced past the base, so those arms compare `identity.base_frontier == base.frontier &&
  identity.base_digest == base.digest()` and never call `stores_match`.

`Published` is the terminal member of the **second** family but was written with the first family's
check. The law's `retry` names the same base as the publication it is joining; by the time it
re-enters the loop the publication has applied, the three Stores hold the post-decision snapshot,
and `base.pack` is still the GENESIS pack (HT8's finding: the `pack` half of an `ArtifactPair` is
the genesis and the live state is pack + spr folded). So `stores_match` was guaranteed false for the
very request that produced the publication — the one request that must be allowed to join it.

**Fix** (`🏃️runtime/🦀️.rs:1346`): the `Published` arm now binds `identity` and folds like its
siblings — when the retained publication's `base_frontier`/`base_digest` are this request's base,
the advanced Stores are that request's own effect and `stores_match` is not required. A request
naming a *different* base still has to match the Stores exactly (that is a fresh decision stacking
on the published state, which `advance_turn`'s `Published` arm then routes through `Preflight`). The
refusal record now also prints which of the two conjuncts was false. The law is unchanged.

## 2. L3 — the driver leaves on `Complete`, and the exit was unattributable *(named this round)*

State of the evidence:

- HT9 landed `"driver leaving on Complete"` / `"…on Aborted"`, and 23:48 printed **`Complete`**
  during the **`Journal`** cancellation phase. So the driver did reach a terminal turn and left,
  abandoning a document whose writer and ingress are still held.
- Two facts contradict each other and neither can be resolved from the source alone:
  1. `wait_for_abandoned_approval_handoff` has its OWN `tokio::time::timeout(5 s)` with a fully
     named panic (`state …, last abandoned turn …, maintenance_owns=…, gate_free=…`). It did **not**
     fire — the 30 s OS-thread watchdog did. A `tokio` timeout that cannot fire means the
     current-thread runtime is wedged in a synchronous block, exactly the case HT9 armed the
     watchdog for.
  2. `drive_abandoned_turn` returns `Complete` from **nine** distinct places, and `Complete`/`Aborted`
     shared ONE recorder line in the driver that **overwrote** whatever the arm had recorded.

So the single recorded string could not say which of the nine, nor what the driver did next. Landed
this round (no law touched, product behaviour unchanged):

- Every `Complete` return in `drive_abandoned_turn` (`:1733–1891`) now records its state **and** its
  false conjunct: `Complete on Assembly/request=… identity=…`, `Complete on Journal/request=…
  identity=… receipt=…`, `Complete on Ready/lease=… pending=… writer=…`, `Complete on
  Published/matches=… writer=…`, `Complete on Closing`, `Complete on an absent document`, and the
  `Recovery` / `Recovered` / `Verification` / `Publishing` arms likewise.
- The `Journal` arm's `Ok(progress)` / `Err(error)` turns now record too (the `Assembly` arm already
  did), so a journal that spins instead of closing names its phase.
- `drive_abandoned_request` (`:1899`) no longer overwrites the arm's reason: it composes
  `driver leaving on <exit> [<arm reason>] — <what it did next>`, where the tail is one of
  `outbox abandoned` / `abandon refused <error>` / `ingress already surrendered` / `Undo, no outbox`.
  `driver retrying after …` distinguishes a live-locked driver (the `abandon_prepared_approval`
  retry loop) from one that really returned — those are the same symptom and different defects.
- The watchdog print (`🔬️unit:314`) now also carries `last_gis_map_identity_mismatch()`,
  `last_gis_map_prepare_conflict()` and `last_gis_map_commit_conflict_arm()`, so a wedged runtime
  still arrives with four names instead of one.

The product fix the hand-over calls for — the driver must keep driving close turns until the
assembly reports terminal and hand the Stores + document write back exactly — needs the arm named
before it can be written: whether the guard to relax is `request_matches`, `identity_matches`, or a
turn arriving at a state the driver must not treat as foreign. Guessing here costs a whole
coordinator rerun; the next capture answers it outright.

## 3. L4 — the census was never exhaustive; three carriers had no counter *(census closed)*

`Database::pool_use` is cloned out in exactly **four** places — `require_open_use()` has four call
sites (`⚙️engine/🦀️.rs:8599`, `:8673`, `:8867`, `:8894`) — plus the open-time probes and whatever
those four hand on. HT9's fourteen families cover the probes, the catalog futures, the mount
futures, the artifact authority, its runner handoff and the retirement cursors. Three carriers were
left uncounted, and any of them explains `owners 3` with every family reading 0:

1. **`DatabaseSyncHelloState`** (`🔄️sync/🦀️.rs:1504`) holds `_pool_use`. The census counted
   `database_sync_hello_live_slots()` — the **registry**, which is cleared at `:1813` the moment the
   hello stops being pending. The `Arc<DatabaseSyncHelloState>` outlives that clearing inside its
   future, its session and its `callback_at` deadline closure. A hub with live peer sessions is
   precisely the shape that keeps these alive, and two of them is exactly `owners 3`.
2. **`mount_document`'s local clone** (`:8599`). Taken before the registry lock; on the
   `Opening`-slot and `Ready`-slot branches it is NOT moved into the mount future, so it is held as
   a bare local across `reply.await` — in no slot table, no `live_*` struct family and no mount
   future.
3. **`checkpoint_document`'s local `_pool_use`** (`:8894`), held across three awaits including the
   version-graph checkpoint.

**Landed:**

- `db_sync::database_sync_hello_live_states()` — a counter incremented where the state is built
  (`🔄️sync/🦀️.rs:1893`) and decremented in a new `Drop for DatabaseSyncHelloState`.
- `DatabasePoolUseSiteGuardV1` (`⚙️engine/🦀️.rs`, next to the other live guards) — a two-atomic RAII
  guard placed at the two bare call sites.
- `DatabaseRetainedPoolUseCensus` gains `live_sync_hello`, `mount_wait`, `checkpoint`.

With these the census is **closed over every carrier of `Database::pool_use`**: the two remaining
`require_open_use` sites hand their clone to a future that already has a family. The next capture
therefore either names the blocking family outright, or — if all seventeen still read 0 — proves the
clone escaped into a submitted pool job, which is itself decisive and rules out HT9's
`retirement.commit` conjecture (`artifact_retirement` has read 0 in every capture, and
`ArtifactRunnerRetirementReservation::commit` installs unconditionally into the counted slot).

The product fix — `shutdown_step`'s `PoolUse` phase (`:8780`) currently only re-reads
`Arc::strong_count` and returns `Progress`, i.e. it **spins to the deadline and drives nothing** —
is written against the named family, not against a guess.

## 4. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht10 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors | `🗑️generated/ht10-check-hub-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht10 cargo check -p semio-framework-os-kernel-db --all-targets` | lib **clean**; `lib test` target red on a **pre-existing, not-mine** error | `🗑️generated/ht10-check-db-1.txt` |

The kernel-db `lib test` error is `E0432: unresolved import semio_framework_pack` at
`🛢️db/📦️packages/🦀️rust/🦀️.rs:12` — a missing dev-dependency in a file this slice never touched and
which has no working-tree diff. The `semio-hub --all-targets` check compiles the kernel-db **lib**
with my edits and is green, so every db change here type-checks.

Rule 26 honoured: no `cargo test` / `nextest` / `build` on `-p semio-hub` from this slice. The live
hub on 7611 and every serve were left alone.

## 5. Files changed

| file | change |
|---|---|
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | L1 product fix in `prepare_retained_document`'s `Published` arm; nine named `Complete` records + two `Journal` turn records in `drive_abandoned_turn`; composed exit record + `recorded_abandoned_assembly_turn()` in `drive_abandoned_request` |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | watchdog print carries the identity-mismatch, prepare-conflict and conflict-arm recorders (instrumentation only — no law, bound or assertion changed) |
| `🧰️framework/…/🛢️db/⚙️engine/🦀️.rs` | `DatabasePoolUseSiteGuardV1` + two site counters; three new census fields |
| `🧰️framework/…/🛢️db/🔄️sync/🦀️.rs` | `database_sync_hello_live_states()` + `Drop for DatabaseSyncHelloState` |

## 6. Honest gaps

- **L1 is fixed but NOT yet proven** — verified only by `cargo check`; rule 26 puts the run with the
  coordinator. If the join now succeeds, the law continues past `:676` into assertions HT9 never
  reached, and a later one may fail on its own merits.
- **L3 is instrumented, not fixed.** No product change landed for it this round, deliberately: the
  arm that quits is one of nine and the 5 s-timeout-that-never-fired says the runtime wedges, which
  changes which fix is correct. The 30 s watchdog is kept exactly as HT9 armed it.
- **L4 is counted, not fixed.** The `PoolUse` phase still spins rather than driving. The census is
  now exhaustive by construction over `Database::pool_use`, which is the step the hand-over asked
  for before the product fix.
- Nothing here was observed on the live hub; all three are unit/bin-unit laws.

**needs hub rerun.**

---

# Batch 2 — the 00:11 rerun answered all three

`🗑️generated/coordinator-hub-nextest-full-0011.txt`, 2026-09-21 00:11: **321 — 318 passed / 3 red**
in 33 s. Every instrument spoke.

## 7. L1 — the join fix HELD; the law now dies 95 lines further, on a poisoned committer *(second product defect, fixed)*

`prepare conflict` is **gone**. The `retry` joined the one publication, and lines 677–769 — the
`applied` receipt, the generation, the first-history-edit ordinal, the approved proposal state, the
four-entry publisher order, the gate and ingress releases, the three retained Store owners, the
durable undo target, the inverse reconstruction and the undo preparation — **all pass**. §1's fix is
proven by rerun.

The new death is `🔬️unit:771`, `retained durable undo: Unavailable`. `Unavailable` at that call can
only come from `undo()`'s very first gate, `reserve_cleanup_job` (`🏃️runtime/🦀️.rs:713–717`), which
refuses whenever `self.closing` is set. And `close()` raises `closing` in its **first statement**
(`:2489`) and nothing ever lowers it.

The law polls `close()` once at `:669` — deliberately, to prove that "close joins the active
post-witness driver instead of starting another verifier or publisher" — and **drops** it at `:671`.
The bit survived the dropped future, so from that point the committer refused every commit and undo
with `Unavailable`, with no close in flight to ever lift it. A cancelled close is retryable; only a
close that reached its terminal handoff may leave the committer closed.

**Fix:** `GisMapClosingCursorV1` — the `closing` bit now lives for the lifetime of the close future.
`close()` raises it and holds the cursor; the cursor is surrendered only on the terminal
`return Ok(())` (`:2544`), so every error path and every cancellation lowers the bit and announces a
state change. The law is unchanged.

## 8. L4 — `live_sync_hello: 2`. The census named it on the first try *(product defect, fixed)*

```
retained slots DatabaseRetainedPoolUseCensus { … live_mount_future: 0, live_sync_hello: 2, mount_wait: 0, checkpoint: 0 }
```

`owners 3` = the `Database`'s own use + **two live `DatabaseSyncHelloState`s**. HT9's
`retirement.commit` conjecture is ruled out: `artifact_retirement` has read 0 in every capture and
`ArtifactRunnerRetirementReservation::commit` installs unconditionally into that counted slot.

The owner is the **deadline timer**. `DatabaseSyncHelloFuture::try_submit_with_use`
(`🔄️sync/🦀️.rs:1933`) did `let deadline = state.clone(); pool.callback_at(deadline_ms, move ||
deadline.deadline_callback())` — an **owning** `Arc` parked in a `WorkerPool` timer that cannot be
cancelled, with `DATABASE_SYNC_HELLO_DEADLINE_MS = 30_000`. The registry slot is cleared at `:1813`
the moment the hello stops being pending, so `sync_hello` reads 0 — but the state, and with it the
`Database`'s `WorkerPoolUse` clone, stays pinned for the **full 30 s after the hello finished**. A
shutdown deadline of 5 s can never win that race.

This is a live-hub defect exactly as the slice framed it: **a hub that completed its peer sync
cannot shut down cleanly**, and P4's SIGTERM drain inherits it. It has nothing to do with sqlite
reopen — reopen is merely what makes the law run two hellos.

**Fix:** the deadline callback now holds a `Weak` and upgrades inside
(`🔄️sync/🦀️.rs:1933`, docstring on `deadline_callback`). Behaviour is identical while a real owner
exists; once every owner is gone there is nothing left to expire, and the `WorkerPoolUse` is
released at the instant the last real owner drops it instead of 30 s later. No shutdown-side
spinning was added: the carrier simply stops outliving its owners.

## 9. L3 — there is no wedge. The phase marker was lying, and `close()` spins unbounded

The recorder resolved the contradiction batch 1 could not:

```
last phase entered: awaiting the abandoned handoff after cancellation phase Journal
last abandoned turn Some("driver leaving on Complete [Complete on Published/matches=false writer=false] — ingress already surrendered")
```

`Complete on Published/matches=false writer=false` is **correct, expected behaviour, and it is not
from the phase loop at all**. It is the FINAL section of the law (`🔬️unit:1463`+): the last commit is
polled to `Committed` and dropped, the abandoned driver drives the receipt cutover to its autonomous
public checkpoint, the writer is released and the ingress surrendered — so the driver's next turn
sees a `Published` document that is no longer its own, records it, and correctly returns.

The phase loop therefore **passed**. The marker still read "cancellation phase Journal" because the
law's entire tail — `poll_approval_to_phase(Committed)`, the 10 s cutover wait, `committer.close()`,
`database.shutdown()`, `memory.close()`, `pool.shutdown()` — sets **no `watchdog.at`**. And three of
those calls have **no timeout at all**. So the 5 s tokio timeout "that never fired" was never
armed: the runtime is **not wedged**, the law is simply parked in an unbounded await, and the only
candidate that can park for ever is `committer.close().await` (`:1498`).

`close()` (`:2486`) is an unbounded loop with no budget. Its `Closing` arm drives the three owned
Stores one at a time through `close_store`, and a slot that answers `SnapshotRetirementStep::Pending`
or `Blocked` for ever makes `close()` spin on `yield_now` and never return — HT9's §12/§18 reading
(`Progress(ClosingParent)`, `closing_owner=none`: the parent publication drained every owner but
never reported terminal) is the same defect seen from the commit side.

**Landed this round** (instrumentation only — no law, bound or assertion changed):

- `record_close_store_step` + `last_close_store_step()`: `close_store` now takes its slot name and
  records `value|drawing|parent` with `Complete terminal_is_empty=…` / `Pending` / `Blocked`.
- The watchdog prints it alongside the other four recorders.
- Six honest `watchdog.at` markers across the law's tail, so the next abort names which of
  `close`, `database.shutdown`, `memory.close` or `pool.shutdown` is parked instead of pointing at a
  phase that finished long before.

The store-level fix is deliberately NOT guessed: it lives in `directory::os_store`'s owned-Store
retirement, and which of the three slots refuses to retire — and with which step — decides whether
the fix is in `close_owned_step`, in the fence invalidation `Self::closing` performs, or in
`close()`'s missing budget. The next capture names the slot outright.

## 10. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht10 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors | `🗑️generated/ht10-check-hub-2.txt` |

## 11. Files changed (batch 2)

| file | change |
|---|---|
| `🧰️framework/…/🛢️db/🔄️sync/🦀️.rs` | **L4 product fix** — the hello deadline timer holds the state weakly instead of owning it for 30 s |
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | **L1 product fix** — `GisMapClosingCursorV1` ties the `closing` bit to the close future's lifetime; `close_store` slot-named retirement recorder |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | watchdog prints the close-store step; six `watchdog.at` markers over the law's previously unmarked tail |

## 12. Honest gaps (batch 2)

- **L1**: the join fix is proven by rerun; the `closing`-cursor fix is `cargo check`-only so far. If
  it holds, the law continues past `:771` into the undo assertions, which nobody has ever reached.
- **L4**: the carrier is proven by the census; the weak-timer fix is `cargo check`-only. Two owners
  of a 30 s timer exactly account for `owners 3`, so the shutdown should now complete well inside
  its 5 s deadline — but only the rerun says so.
- **L3**: still no product fix. The defect is now localized to owned-Store retirement inside
  `close()`, and the next capture names the slot and step. `close()`'s missing budget is a real
  second defect (an unbounded `close()` can hang a hub drain) that should be fixed once the first
  one is understood — fixing it first would only convert the hang into a timeout and hide the slot.
- Nothing here was observed on the live hub; all three are unit/bin-unit laws. The live hub on 7611
  and every serve were left untouched, and rule 26 was honoured throughout.

**needs hub rerun.**

---

# Batch 3 — the 00:28 rerun: L4 GREEN, both survivors localized

`🗑️generated/coordinator-hub-nextest-full-0028.txt`: **321 — 319 passed (1 leaky) / 2 failed** in 33 s.

## 13. L4 is GREEN — the weak deadline timer was the whole defect

`tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` **passes**. §8's
one-line change (an owning `Arc` in an uncancellable 30 s `callback_at` → a `Weak`) was the entire
root. `owners 3` was the `Database` plus two hello states pinned by their own deadline timers. The
live-hub defect the slice named — *a hub that reopened sqlite cannot shut down cleanly, and P4's
SIGTERM drain depends on it* — is fixed, proven by rerun.

## 14. The 1 LEAKY test is NOT one of mine

```
LEAK [ 1.038s] ( 43/321) semio-hub artifact_authority::chunk_cas::tests::quick::artifact_chunk_cas_sqlite_roundtrip_crosses_legacy_payload_ceiling
```

It **passed**; nextest flags it because the process still held a child/handle open after the test
function returned. It is a chunk-CAS sqlite roundtrip law, untouched by this slice and unrelated to
L1/L3/L4. Not mine — reported, not taken.

## 15. L3 — the store is named: `value Blocked`, and the block is an OUTSTANDING SNAPSHOT READ LEASE

```
last phase entered: driving the committer close to its terminal Store handoff
last close-store step Some("value Blocked")
```

Both new instruments paid off. The phase marker confirms §9 exactly: **there was never a wedge** —
the law is parked in `committer.close().await`, the one call in its tail with no timeout.

`Blocked` traces to one line. `close_store` → `ArtifactStore::close_owned_step` →
`ArtifactStoreCursorDisposer::close_step` → phase `ReturnedReads`
(`🧰️framework/…/🏪️store/🦀️.rs:1967`):

```rust
None if !store.snapshot_read_leases_terminal_is_empty() => Ok(SnapshotRetirementStep::Blocked),
```

i.e. *nothing has been returned to retire, and the lease registry is not empty* — a **snapshot read
lease is still OUT** on the value store. This is a state **only the lease holder can leave**. The
closer cannot: nothing in `close()` returns a lease. So `Blocked` is **permanent for the closer, not
transient**, and `close()` treating it as "retry" was the defect that turned a leaked lease into an
unbounded hang.

**Landed (two real product defects, both the coordinator's):**

1. `close_store` now answers `Blocked` with `Storage` and records
   `value Blocked outstanding_reads=N returned_reads=M` — an unbounded hang becomes a named refusal.
   A hub drain that cannot close must fail loudly, never hang: P4's SIGTERM path inherits this.
2. The `yield_now` busy spin at the tail of `close()`'s loop is **gone**. A spin on a current-thread
   runtime starves every other task and every timer on that thread. It only ever existed to re-drive
   a non-progressing turn; `Pending` is real progress and loops on its own, and `Blocked` no longer
   loops at all.
3. `ArtifactStore::outstanding_snapshot_read_count()` + the registry's `occupied_count()`
   (`🏪️store/🦀️.rs:220`, `:16236`) — additive, so the next capture says whether the owner is a live
   reader (`outstanding>0`) or an unreclaimed returned lease (`returned>0`), which are different
   defects in different code.

Not fixed: **who** holds the value store's lease. No hub code leases it — the runtime leases only the
**parent** (`🏃️runtime:2118`, `:2217`), so the holder is inside `os_store`'s durable-group
composition. The census in the next capture picks the branch; guessing it now would be a third
speculative round.

## 16. L1 — `Unavailable` → `Conflict`: the closing cursor worked, the undo join did not

`:772` now reads `retained durable undo: Conflict`. §7's `GisMapClosingCursorV1` is proven: the
dropped close future no longer poisons the committer, and the undo reaches the join. The refusal
moved one layer in.

Neither recorder was printed, because `.expect("retained durable undo")` prints only the error, and
because **two bare `Conflict` returns on the undo's path recorded nothing at all**: `advance_turn`'s
`Published` else-arm and its `Closing` arm. Landed:

- `:772` now prints `prepare conflict`, `conflict arm` and `identity mismatch`.
- `advance_turn`'s `Published` else-arm records `drive Published/stores_match=… writer=…` and
  answers the new named arm `DrivePublishedIdentityAndStoresDiffer` instead of a bare `Conflict`.
  The `Closing` arm records too and keeps answering `Unavailable` — its error is unchanged.

The undo names `after_base` (the post-approval pack) against a `Published` state whose retained
identity names the genesis base, so it takes the `stores_match` branch §1 deliberately preserved for
a *different* base. Whether the false conjunct is the parent, the drawing or the derived value
child is exactly what the next capture prints.

## 17. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht10 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors | `🗑️generated/ht10-check-hub-3.txt` |

The `🏪️store` edits are two additive accessors (no collisions, grep-verified) and compile through
the hub check, which builds that kernel crate's lib.

## 18. Files changed (batch 3)

| file | change |
|---|---|
| `🧰️framework/…/🏪️store/🦀️.rs` | `occupied_count()` + `outstanding_snapshot_read_count()` — additive lease census |
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | **`Blocked` is a named `Storage` refusal, not a retry**; **`yield_now` spin removed from `close()`**; `DrivePublishedIdentityAndStoresDiffer` arm + records on the two bare `Conflict`/`Unavailable` drive arms |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | `:772` prints the three recorders (instrumentation only) |

## 19. Honest gaps (batch 3)

- **L3** will now fail with a *named* `Storage` instead of SIGABRT — that is correct product
  behaviour and strictly better evidence, but it is **not green**. Green needs the leaked value-store
  lease returned, in `os_store`, once the census says which kind it is.
- **L1**'s two fixes are proven by rerun (`Unavailable` → `Conflict` is progress through a real
  gate); the third round's changes are `cargo check`-only.
- The `🏪️store` crate's own test target was not run (rule 26 budget); the hub check compiles its lib.

**needs hub rerun.**
