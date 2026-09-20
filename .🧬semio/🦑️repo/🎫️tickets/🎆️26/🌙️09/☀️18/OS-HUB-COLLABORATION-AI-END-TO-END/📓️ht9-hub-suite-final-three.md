# HT9 — hub suite, the final three (outcome 2's gate to green)

Slice HT9. Spec: `📓️ht8-hub-suite-final-four.md` (§9–§15) + the 22:04 rerun
`🗑️generated/coordinator-hub-nextest-full.txt` — **321 tests: 318 passed / 3 failed / 0 skipped**,
suite wall time **33.0 s** (58 red this morning; 315/6 at 21:03).
Rule 26 binds me: no `cargo test` / `nextest` / `build` on `-p semio-hub`;
`cargo check -p semio-hub --all-targets` and kernel crate checks under rule 25's private
`CARGO_TARGET_DIR=…/target-ht9` only.

## 0. What the 22:04 rerun printed — HT8's three instruments all spoke

| # | law | 22:04 evidence | reading |
|---|---|---|---|
| **L1** | `…gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint…` | `🔬️unit:590` `assert_ne!` **"the actor WAL chain and Store content revision are intentionally distinct hash domains"**, left == right (same 32 bytes) | HT8 §9's `retire_parsed_document` worked — the `Drop` abort is gone and the law now runs 100+ lines further, to its *last* conjunct |
| **L3** | `…quick::gis_map_abandoned_pre_witness_request…` | `Assembly phase=ClosingDrawing [failure=true prepared=false host=false parent=Closing/terminal_is_empty=false/admitted=1/staged=0 drawing=retired value=retired]` + a second panic `durable-group:1521` "reached Drop before mounted or terminal owner handoff" | **drawing and value are BOTH retired**; the owner that refuses is the **parent** publication, `Closing` with 1 admitted / 0 staged |
| **L4** | `tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `retained pool-use owners 3`, census `{ capability_open: 0, catalog_read: 0, catalog_bootstrap: 0, create_catalog: 0, sync_hello: 0, artifact_retirement: 0, runner_handoff: 0, mount_work: 0 }` | **all eight families 0** — HT8 §12's mount-work hypothesis is falsified too |

(L2 — `prepared_approval_survives_restart_and_reconciles_exactly_once` — is **GREEN** at 22:04: HT8
§3's corpus alignment + §10's per-query control were the roots.)

## 1. L1 — the "distinct hash domains" claim is false by construction *(oracle corrected to the real invariant)*

HT8 §9 worked: the `Drop` abort at `🌿️vcs:612` is gone and the law now runs ~100 lines further, to
its **last** conjunct before the undo attempt. That conjunct is

```
assert_ne!(actor.frontier.chain_hash, parent_revision, "the actor WAL chain and Store content revision are intentionally distinct hash domains");
```

and it is false: the two are the SAME 32 bytes. I looked for a product defect first and found a
deliberate three-site contract instead — for a **fixed-three durable-group decision** the actor's
frontier chain hash IS the parent Store's post content revision:

| site | code |
|---|---|
| `🛢️db/🗿️artifact/🦀️.rs:2415` `append_durable_group_decision` | `next_frontier = Frontier { …, chain_hash: record.parent_post_revision(), … }` |
| `🛢️db/🗿️artifact/🦀️.rs:2057` WAL replay of the same record | `replay_frontier = Frontier { …, chain_hash: record.parent_post_revision(), … }` |
| `🛢️db/🗿️artifact/🦀️.rs:434` `recovered_checkpoint` | `chain_hash: self.record.parent_post_revision()` |

and the journal record's carrier is pinned by a **green** kernel law,
`🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs:539` `assert_eq!(record.parent_post_revision(), decision.parent.post_revision)`.
The Store side is exact too: after the decision the parent's `content_revision()` reads the
unadopted durable-group root's `content_revision`, which `🗄️durable-group:577` sets to that same
`post_revision`. So equality is not a collapse — it is the anchor.

Why the product cannot do otherwise: an ordinary command batch sets `chain_hash` from
`self.state.content_hash()` (`🗿️artifact:2282`), but a durable-group decision never touches the
actor's PMap state, so that hash would not advance at all while `head_seq`/`commit_seq` do. The law
already accepts `actor.head_edit_id` coming from `record.parent_edit_id()` two lines earlier; taking
the edit id from the record and forbidding the revision from it is self-contradictory.

**Landed** (`🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:590`) — the conjunct is **stronger**,
not looser: the stale inequality is replaced by an equality against the parent Store
(`assert_eq!`, exact) **plus** a new inequality against the law's own genesis read
(`assert_ne!(actor.frontier.chain_hash, initial.frontier.chain_hash)`), which keeps the claim the
conjunct was really there to make — that the committed decision advanced the frontier — and can no
longer be satisfied by a stale hash. Nothing was deleted, `#[ignore]`d or weakened; no product
source was touched for L1.

## 2. L3 — `cancel()` rewinds the close cursor on every maintenance turn *(product defect, fixed)*

HT8 §11's `closing_witness()` named the owner, and it is not the one the phase label suggested:

```
phase=ClosingDrawing failure=true prepared=false host=false
parent=Closing/terminal_is_empty=false/admitted=1/staged=0  drawing=retired  value=retired
```

Only the **parent** publication exists (1 admitted item); drawing and value were never created —
the assembly was cancelled while still preparing. So the loop is not "drawing refuses to retire":
`close_assembly_publication` on a `None` publication returns `Ok(true)` immediately.

The root is `DurableOwnedThreeStoreMapAssemblyV1::fail()`
(`🧰️framework/…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1239`), whose last statement was an
**unconditional** `self.phase = ClosingValue`. `cancel()` calls `fail(Cancelled)` for every phase
except `Mounted`/`Terminal`/`Empty` — *including the three `Closing*` phases* — and
`drive_abandoned_turn`'s `Assembly` arm (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1662`) calls
`owner.cancel()` **before every** one-item `advance()`. The cycle is therefore exact and eternal:

| turn | cancel() | advance(1 item) | result |
|---|---|---|---|
| n | `ClosingDrawing` → **`ClosingValue`** | value publication is `None` → `true` → `ClosingDrawing` | `Progress(ClosingDrawing)` |
| n+1 | identical | identical | identical |

`ClosingParent` is unreachable, so the parent publication is never closed, the assembly never
reaches `Terminal`, `take_terminal_owners()` is never called — and the law's three observations
follow directly: `maintenance_owns=true`, `gate_free=false`, `ingress_released=false`, plus the
secondary `🗄️durable-group:1521` "reached Drop before mounted or terminal owner handoff" when the
test aborts with the assembly still alive. It is exactly the memory-proven `FixedOperationRegistry`
shape — a close cursor restarted from slot 0 on every grant.

**Landed:** `fail()` now records the failure and enters the close sequence **idempotently** — the
phase is set to `ClosingValue` only when it is not already one of `ClosingValue`/`ClosingDrawing`/
`ClosingParent`. Everything else in `fail()` was already idempotent (`failure.is_none()` guard;
`ArtifactStoreBatchPublication::begin_close` is `close_started`-guarded at `🏪️store:3937`), so a
repeated cancel now costs nothing and the cursor walks value → drawing → parent → `Terminal`.
The one kernel law that cancels an assembly
(`durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers`,
`🗄️durable-group/🧪️tests/🔬️unit:436`) cancels once from `PreparingDrawing`, which the guard does not
touch. A docstring on `fail()` states the invariant.

## 3. L4 — the census is a TERMINAL census; every family reads 0 because the owner is ALIVE *(instrumented decisively)*

`mount_work: 0` falsifies HT8 §12, and the reason all eight families read 0 while
`retained pool-use owners 3` is structural, not accidental:

- HT8 §12's exclusion of the authority family was invalid. `ArtifactRunnerHandoff.pool_use` is an
  `Option` **taken at the runner's terminal transition** (`🗿️artifact:5131`), while
  `ArtifactAuthority._pool_use` (`🗿️artifact:4527`) is a *second* clone that lives as long as the
  authority object. `artifact_runner_handoff_pool_use_live_slots() == 0` therefore says only that
  no runner is mid-flight — a live authority past its terminal transition still pins the use.
- The four `database_*_registry()` tables the census counts are **terminal slot tables**: a state
  claims its slot only when it is orphaned (`try_prepare_with_use` checks `registry[slot].is_some()`
  and refuses, it never inserts). So a LIVE `DatabaseCapabilityOpenState` / `DatabaseCatalogReadState`
  / `DatabaseCatalogBootstrapState` / `DatabaseCreateCatalogState` — each of which stores a
  `WorkerPoolUse` clone — is invisible to the census that was supposed to find it.

Every holder of a clone in the whole dependency set is enumerable by grep (`WorkerPoolUse` in
`⚙️engine` + `🗿️artifact` + `db_sync`), and after the two corrections above exactly five of them had
no live counter. **Landed — five new census families, all live-object counters (increment where the
struct that stores the clone is built, decrement in its `Drop`), so the census is now exhaustive
against that grep:**

| field | owner | file |
|---|---|---|
| `live_authority` | `ArtifactAuthority._pool_use` | `🗿️artifact` — `ARTIFACT_AUTHORITY_POOL_USES` + `artifact_authority_live_pool_uses()`, incremented in `spawn_with_pool_use`'s success arm, decremented at the top of `Drop for ArtifactAuthority` |
| `live_capability_open` | `DatabaseCapabilityOpenState._pool_use` | `⚙️engine` |
| `live_catalog_read` | `DatabaseCatalogReadState._pool_use` | `⚙️engine` |
| `live_catalog_bootstrap` | `DatabaseCatalogBootstrapState._pool_use` | `⚙️engine` |
| `live_create_catalog` | `DatabaseCreateCatalogState.pool_use` | `⚙️engine` |

Next rerun prints the family and the count, and `3 = Database's own + N` must now balance. I did
**not** guess a shutdown step: HT6 (`create_catalog`), HT7 (`artifact_retirement`) and HT8
(`runner_handoff`, `mount_work`) each cost a rerun on a guess, and the remedy differs per family
(drive a live authority terminal vs. cancel-and-await a retained activity future). The live-hub
defect (P4's SIGTERM drain) stands until the family is named.

## 4. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht9 cargo check -p semio-hub --all-targets` (§1, §2, §3 — compiles the hub bin/lib test targets plus `semio-framework-os-kernel` and `…-kernel-db` as dependency libs) | **EXIT 0**, 0 errors, 316 warnings (unchanged count) | `🗑️generated/ht9-check-hub-1.txt` |

Rule 26 respected: no `cargo test`, `cargo nextest` or binary build on `-p semio-hub` from this
slice, so **no hub law was observed passing**. Live hubs on 7611/7501 and the serves on
6070/6071/6190/6191 were not touched. Nothing in `🗑️generated` that I did not create was removed,
and no `🗑️generated` folder was swept.

## 5. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` — §2's
  idempotent `fail()` phase guard + its docstring. **The only product-behaviour change.**
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §1's corrected + strengthened conjunct.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs` — §3's `ARTIFACT_AUTHORITY_POOL_USES`,
  its reader, the increment in `spawn_with_pool_use` and the decrement in `Drop for ArtifactAuthority`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` — §3's four live-state counters, their
  `Drop` impls and the five new `DatabaseRetainedPoolUseCensus` fields.

Nothing was deleted, `#[ignore]`d or loosened; no law lost an assertion; no bound was removed; no
gis or stdio plugin source was touched.

## 6. Honest gaps

- **Nothing was observed green.** Rule 26 forbids this slice from running the suite; everything here
  is source-proved plus a tree that checks clean.
- §2 is a landed product root. §1 is an oracle correction (the product is right and three sites plus
  a green kernel law say so) — if the intended contract really is a separate WAL chain domain, the
  product has no such hash anywhere and would need one invented; I did not invent it.
- §3 is an instrument, not a fix: L4 stays red and a hub that reopened sqlite still cannot shut down
  cleanly. The census is now exhaustive against the `WorkerPoolUse` grep, so this is the last
  instrument that family can need.
- L3's fix removes the *spin*; whether the parent publication then drains to `terminal_is_empty` under
  repeated one-item grants is proved only by source (`close_step` returns `Pending { released_items: 1 }`
  per item until `Complete`). If it does not, the rerun will print a `ClosingParent` witness instead
  of `ClosingDrawing` — which is itself the confirmation that the rewind was the outer defect.

**needs hub rerun.** Expect: **L1 green** (§1's conjunct now matches the product; the law continues
into the undo attempt, which HT8 §2's folded-pair oracle already covers); **L3 green** (§2 lets the
close cursor reach `ClosingParent` → `Terminal` → `take_terminal_owners` → `Aborted`, which releases
the document writer and the ingress) — or, if still red, a witness naming `ClosingParent`;
**L4 red**, now printing `live_authority` / `live_capability_open` / `live_catalog_read` /
`live_catalog_bootstrap` / `live_create_catalog`, one of which must account for the 2 clones beyond
the `Database`'s own.

---

# HT9 — batch 2 (rerun 22:27: **321 — 315 / 5 / 1 TIMED OUT**, suite 309 s)

## 7. The three new `SIGABRT`s are HS1's, not mine

All three print the same two lines and nothing else:

```
thread 'semio-pool-worker-0' (…) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

`semio-pool-worker-N` threads are spawned by `semio-framework-async`'s native pool.
`git diff` shows that file edited in flight at **22:17** (ten minutes before the run), adding a
stack bound to exactly those threads:

```
- thread::Builder::new().name(format!("semio-pool-worker-{index}")).spawn(…)
+ thread::Builder::new().name(format!("semio-pool-worker-{index}")).stack_size(WORKER_STACK_BYTES).spawn(…)
```

(`🧰️framework/🔨️modules/⏳️async/🦀️.rs`, plus 47 new lines in its `🔬️native-pool-unit` tests) — HS1's
declared pool-worker stack-overflow slice, caught mid-edit by the 22:27 build. **Not mine, left
alone.** My batch touches no pool-worker code path: the five census counters are `AtomicUsize`
`fetch_add`/`fetch_sub` with **no assertion and no panic in any `Drop`**, so none of them can abort
during unwinding or at thread exit, and none recurses.

## 8. L3 — the fix stands; the 300 s TIMEOUT is now guarded and will name itself

§2's `fail()` idempotence is a proven product defect (an unconditional phase rewind under a
per-turn `cancel()`), so I did not revert it. What changed is the *symptom*: the law used to lose
its 5 s `tokio::time::timeout` race and fail fast with a witness; at 22:27 it burned nextest's full
300 s kill budget, which means the current-thread runtime is wedged — and a `tokio::time::timeout`
cannot fire when it is. That is the exact condition HT4/HT8 built `LawHangWatchdogV1` for, but that
watchdog lives in the **bin** test target and L3 is in the **lib**'s `inference::runtime::tests`.

**Landed:** `RuntimeLawHangWatchdogV1` in `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — an OS thread (so wall
clock, not the wedged runtime), armed FIRST in L3 (so it drops LAST) with a **30 s** budget; on
expiry it prints the last `at()` phase the law entered plus `last_abandoned_assembly_turn()` and
aborts. `at()` names the three waits (`poll_approval_to_phase` per cancellation phase, the abandoned
handoff per phase, the unpolled prepared handoff). And `record_abandoned_assembly_turn` now carries
the witness inline — `Continue on Progress(<phase>) [phase=… parent=… drawing=… value=…]` — so the
watchdog's abort line names the publication that is stuck even though the law never reached its own
panic. Next rerun: L3 fails in **≤30 s** with a named phase, and the suite wall time returns to ~35 s.

## 9. L1 — the conjunct was right; the law now dies later

22:04: FAIL at **0.836 s** on `🔬️unit:590`. 22:27: FAIL at **2.110 s** — §1's corrected conjunct
passes (the product's anchor invariant holds) and the law runs on into the undo/retry region past
line 592. The 22:27 `-full` capture was overwritten by a newer coordinator build log before I could
read L1's new panic, so I have the timing but **not** the new assertion text. I need that stderr
from the next rerun to continue L1; nothing else about L1 is actionable this round.

## 10. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht9 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors | `🗑️generated/ht9-check-hub-2.txt` |

Rule 26 respected; live hubs and serves untouched; nothing deleted, `#[ignore]`d or loosened.

## 11. Files changed (batch 2)

- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §8's `RuntimeLawHangWatchdogV1` and L3's
  three `at()` phases.
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — §8's witness inlined into both recorded abandoned turns.

**needs hub rerun.** Expect: **L3 red in ≤30 s** with a named phase + witness (suite back to ~35 s),
never another 300 s kill; **L1 red with a NEW, later assertion** whose stderr I need; **L4 red** with
the five live-object families printed; the three `SIGABRT`s resolve when HS1's pool edit settles.

---

# HT9 — batch 3 (rerun 22:38: **321 — 315 / 6 in 36 s**; capture `🗑️generated/coordinator-hub-nextest-full-2238.txt`)

The watchdog worked: **36 s suite, no 300 s kill**, and all three laws named themselves.

## 12. L3 — the rewind is gone; the parent publication is the stuck owner *(one level down)*

```
law hang watchdog: … exceeded 30s; last phase entered: awaiting the abandoned handoff after
cancellation phase Journal; last abandoned turn Some("Continue on Progress(ClosingParent)
[phase=ClosingParent failure=true prepared=false host=false
 parent=Closing/terminal_is_empty=false/admitted=1/staged=0 drawing=retired value=retired]")
```

§2's fix is **confirmed**: the cursor no longer restarts at `ClosingValue` — it walks
value → drawing → **`ClosingParent`** and stops there. So the defect is now one level down, inside
`ArtifactStoreBatchPublication::close_step`, which drains seven owners in a fixed order
(`preparation` → `source` → `stage` → `coalesce_key` → `receipt` → `authority` → `fault`) and
answers only `Pending`/`Blocked`/`Complete` — the same "bool discards which owner" that §2 had to
instrument one level up. Two further facts are already in the witness: `staged=0` (so `stage` is
not the blocker) and `admitted=1` (one item still admitted). Note also that the law's own
`tokio::time::timeout(5 s)` never fired in 30 s while the OS-thread watchdog did — the executor is
starved by the abandoned driver's `yield_now()` spin, which is why the guard had to be an OS thread.

**Landed:** `ArtifactStoreBatchPublication::closing_owner_witness()` (`🏪️store/🦀️.rs`) — the first
owner `close_step` would try to retire, in drain order, plus a `close-not-started` answer when
`begin_close` never ran — and `closing_witness()` now prints `closing_owner=<owner>` for each of the
three publications. One rerun names the owner and whether `begin_close` reached it; a
`close-not-started` answer would itself be the root (only `preparation` is told to begin closing by
`begin_close`, at `🏪️store:14790`).

## 13. L1 — the corrected conjunct passes; the join is refused as `Conflict` *(named)*

22:27 → 22:38 the law now dies **100 lines later**, at `🔬️unit:673`:
`fresh request joins the one publication: Conflict`. §1 is therefore settled — the anchor invariant
holds and the frontier/Store/owner-ref block passes. What fails is the *fresh retry* that must JOIN
the retained `Publishing` publication rather than start a second one; `drive_turn`'s `Publishing`
arm (`🏃️runtime:1574`) returns `Conflict` when `identity_matches` is false, and that helper ANDs
twelve conjuncts into one bool.

**Landed:** `first_identity_mismatch()` + `record_identity_mismatch()` +
`last_gis_map_identity_mismatch()` (`🏃️runtime/🦀️.rs`) — `identity_matches` is now a thin wrapper
that records the first false conjunct by name (`actor`, `command_hash`, `base_frontier`,
`document_write`, `ingress`, …) — and the law's line 673 panic prints both the conflict arm and that
field. No conjunct was removed or weakened. One rerun says whether the retry's identity really
differs (a law bug) or the retained one drifted (a product bug), and in which field.

## 14. L4 — every struct field is 0, so the carrier is a FUTURE *(last family added)*

```
retained pool-use owners 3, retained slots DatabaseRetainedPoolUseCensus { capability_open: 0,
catalog_read: 0, catalog_bootstrap: 0, create_catalog: 0, sync_hello: 0, artifact_retirement: 0,
runner_handoff: 0, mount_work: 0, live_authority: 0, live_capability_open: 0, live_catalog_read: 0,
live_catalog_bootstrap: 0, live_create_catalog: 0 }
```

Thirteen families, all 0. §3's five live-object counters cover **every struct field in the repo that
stores an `Arc<WorkerPoolUse>`** (grep across `⚙️engine`, `🗿️artifact`, `🔄️sync`), so the two clones
beyond the `Database`'s own are held by no field at all — they are **captured inside a live async
future**. Exactly three functions take a `pool_use` clone by value into a future:
`publish_mount_catalog` (`⚙️engine:8484`), `run_open_document_mount` (`:8494`, also the `resume`
future a retry-open rejection parks at `:8562`) and `run_document_mount` (`:8520`). Only the last is
covered, and only while it sits inside a `DatabaseDocumentMountOwner` (`mount_work`).

**Landed:** `DatabaseMountFutureLiveGuardV1` + a `live_mount_future` census field — a guard
constructed **at each of the four call sites** and passed *into* the future, so it counts from the
moment the future is BUILT (not first polled) until it is dropped, whoever holds it. That closes the
enumeration: after this rerun either `live_mount_future` is non-zero — and the fix is that shutdown
must drive or drop the parked mount/resume futures before the pool deadline — or the three retained
owners are not reachable from this process's own owners at all, which would make the count itself
the defect.

## 15. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht9 cargo check -p semio-hub --all-targets` (§12, §13) | **EXIT 0**, 0 errors | `🗑️generated/ht9-check-hub-3.txt` |
| same, after §14's mount-future guard | **EXIT 0**, 0 errors | `🗑️generated/ht9-check-hub-4.txt` |

Rule 26 respected; live hubs and serves untouched; nothing deleted, `#[ignore]`d or loosened.

## 16. Files changed (batch 3)

- `🧰️framework/…/🏪️store/🦀️.rs` — §12's `closing_owner_witness()`.
- `🧰️framework/…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` — §12's `closing_owner=` in the witness.
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — §13's `first_identity_mismatch` / recorder / reader.
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §13's named panic at line 673.
- `🧰️framework/…/🛢️db/⚙️engine/🦀️.rs` — §14's mount-future guard, its four call sites and the
  `live_mount_future` census field.

**needs hub rerun.** Expect: **L3 red, naming `closing_owner=<owner>` for the parent publication**
(`close-not-started` would itself be the root); **L1 red, naming the identity field and the conflict
arm**; **L4 red with `live_mount_future: N`**. Suite stays ~36 s — the watchdog holds.
