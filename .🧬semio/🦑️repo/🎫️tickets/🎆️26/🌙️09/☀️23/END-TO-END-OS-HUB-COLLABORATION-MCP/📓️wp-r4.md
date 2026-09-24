# WP-R4 — Artifact Authority Shutdown Parks In ClosingParked; Replay Reservation Destructor Panic

Slice: R4 (session 10). Native only. Private cargo target: `.tmp-ticket/wp-r4/target`. Captures: `wp-r4/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Repro `document_authority_submits_and_queries_over_finite_pool_turns` hang | DONE: the original command now passes in 0.68 s (`repro-final.txt`) |
| 2. Root cause + product fix for `ClosingParked` / `runner_handoff: 1` | LANDED: the close drains the pending group commit (`ArtifactTurn::CloseFlush`) |
| 3. Destructor panic "artifact history admission dropped a live replay reservation" | LANDED: unwind-safe `Drop` plus a shared history-capacity test lock |
| 4. Laws for both (+ follow-ups) | LANDED: 5 new laws, all green |
| 5. Coordinator add-on: db `--lib` red | DONE: 72 red → **765/765 green under nextest, twice in a row** (`nextest-6.txt`, `nextest-7.txt`) |
| 6. Kernel lib (`--features sync,ureq`) nextest | **1193/1193 green** (`kernel-nextest-1.txt`) |
| 7. Plain in-process `cargo test` gate for db `--lib` | **NOT MET**, see Gaps (a pre-existing resource-retention design issue, not a regression) |

## Findings

### Authority shutdown parks in `ClosingParked` — product defect, not a harness artifact
- `[DEBUG]` witness in the repro loop: `driver=8 … close_error=Some(InvalidArgument("WAL has pending records; force_flush is
  required before close"))`, repeated forever (`generated/repro-1.txt`).
- `SubmitOptions::default()` is `DurabilityClass::Memory`, so `ArtifactWal::submit` leaves the transaction pending under
  `GroupCommitPolicy`. `ArtifactEngine::close_step → ArtifactWal::close_step → SegmentWriter::close_step` refuses while
  records are pending, and nothing on the retained close path ever called `force_flush`. Every close step re-fails, the
  runner parks (`ClosingParked`) with the handoff still holding its `WorkerPoolUse` (`runner_handoff: 1`).
- The hub passes only because `🌎️hub/🏗️bootstrap` submits every batch with `DurabilityClass::Fsync` (never pending).
  Any `Memory`/`Os` submit through `Database` would hang `Database::shutdown` the same way. The harness is right.
- Fix: the close drains the pending group commit as one retained runner turn (`ArtifactTurn::CloseFlush`) before the
  synchronous close steps. `ArtifactWal::has_pending` / `ArtifactWal::close_flush` (a failed drain poisons the active
  segment so close still reaches terminal; only the never-fsynced suffix is lost, which is the `Memory`/`Os` contract).
  Cancellation keeps polling a `CloseFlush` turn instead of dropping it.

### Destructor abort under capacity contention
- `generated/par-1.txt`: `artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release` claims all
  8 global history admissions; a parallel peer held one, `try_claim().unwrap()` panicked, unwinding dropped the live
  `Vec<ArtifactHistoryAdmission>` and the held `HistoryReplayReservationCloseCursor`, both `Drop`s asserted →
  "panic in a destructor during cleanup" → SIGABRT of the whole test binary.
- Fix (product): `ArtifactHistoryAdmission::drop` retires a live reservation and releases the exact slot credit first,
  and only asserts when not already unwinding; `HistoryReplayReservationCloseCursor::drop` asserts only when not
  unwinding. The invariant still fires for real owner leaks; a failing test no longer aborts the binary nor leaks
  global capacity into its peers.

### db `--lib` red tests, by root cause (72 red under nextest `nextest-1-triage.txt`)

| Cluster | Root cause | Side fixed | Fix |
|---|---|---|---|
| 25 index/projection/fault laws: "index close lost retained entry" | `RunEntries::take(i)` legitimately leaves a hole (entry transferred), `close_step` treated the hole as a lost owner | product | `close_step` retires a hole as one step (`🔢️index`) |
| 3 compaction: "compaction page credit returned twice" | `DbIoPages::close_step` returns `Some(0)` for the shell/handback owner; the retained-page ledger decremented page credit for it | product | only page returns (`Some(n>0)`) decrement page credit (`🗜️compact`) |
| 4 fault laws SIGABRT (stack overflow at 64 MiB) | fixed-capacity inline arrays moved by value through async frames: `RunEntries` 208 KB, `WalRecordBatch` 132 KB; `ArtifactEngine::submit` future 2.24 MB | product | backing arrays boxed once at construction (`Box<[Option<_>]>`); submit future 318 KB, law passes at 64 MiB. New law `artifact_engine_submit_future_stays_within_the_worker_stack_budget` |
| history WAL-decoder partial write | `read_field_fragment` follows source page bounds, `DbIoPageWriter::write_fragment` stops at its own page end; the second half was reported as a fault | product | write the tail into the next writer page (`🗿️artifact`) |
| history replay law "cursor retained while" | page-read future looped `close_step` inside one poll | product | new `HistoryReplayPhase::PageClose` retires the read pages one per grant |
| CLI `doc` + facade round trip: "database shutdown retains 1 shared artifact authorities" | `HistoryView` kept `terminal_state` (→ authority `Arc`) after full retirement; CLI dropped the view without `close_step` | product | `HistoryView::close_step` releases `terminal_state` as its last owner; CLI retires the view |
| create-catalog (7) | (a) success release ran at future resolution while the result still owns the state, then went through an Io submission; (b) retry-callback cancel/deadline/exhaustion overrode an already-ready backend result; (c) closing retirement jumped to `Terminal` with storage/document/outcome unpublished (infinite loop); (d) a drive after release tripped the stale-generation check and re-created owners; (e) witness counted a published result as one owner | product | release only on `into_parts` (synchronous when roots are empty, else callback close); `accepts_interruption()` shared by drive and retry; publish before terminal while owners remain; `Terminal`/finished drives return before the stale check; witness counts result owners individually |
| catalog-bootstrap (4) | post-`Validate` drives re-staged `terminal_error = Closed` after it was consumed, blocking admission release | product | cancellation restages only before `Validate` |
| sync hello rejected close | `retain_retry` cleared `queued` while a retry job was retained, so the `Drop` schedule submitted a duplicate | product | `schedule()` coalesces with a retained retry job |
| stale test markers (≈15) | laws pinned source text that was refactored (CRC now in `WalAuthenticatedSource`, `Mutex` import, `rsplit`, vcs `include!`, split test module, `try_submit_with_use`, ...) or wrong include path (`../../../../⚙️engine` = OS engine, not db) | test | markers follow the current source; each law still pins the same property |
| stale test semantics | `authority_generation` 0 is `GenerationId::INITIAL` (hub doc); `ProjectionSource` explodes top-level containers (documented); observed-bytes cap is `LimitExceeded` (sibling law); lost `DbIoPages` park as one lost owner; `DbIoU64List` gained a result handback | test | assertions follow the documented contract |
| env-coupled laws | `SEMIO_TEST_ARTIFACT_DIR` required by 3 laws (unset under cargo/nextest) | test | `std::env::temp_dir()` like the rest of the crate |
| fixture races | job run while holding the controlled-submit queue lock (deadlock/timeout); replenishing saturation job dropped itself on refusal (queue drained); blocker admission not retrying `Contended`; per-iteration owner deltas while a background driver also retires | test | pop outside the lock; replenishing job re-submits until admitted; `admit_fixture_blocker`; cumulative owner-per-grant accounting (`driver_grants` cfg(test) counter) |
| vcs graph `block_on` inside async futures | nested executor inside `VersionGraphFuture` (law forbids it) | product | removed; compiles `Send` without it |
| async driver probe | a merge dropped the probe's close-thread recording | test | restored |

### New laws
- `db_artifact::tests::document_authority_close_drains_pending_group_commit_for_every_durability_class`: Memory/Os/Fsync submit → shutdown reaches terminal, and reopen sees `head_seq 1`.
- `db_artifact::tests::artifact_engine_submit_future_stays_within_the_worker_stack_budget`: submit future ≤ 512 KiB; `WalRecordBatch` and `RunEntries` stay boxed.
- `db_engine::tests::artifact_history_admission_unwind_under_contention_retires_live_reservations_and_returns_credit`: a contended claim panics while 8 live reservations and a started close cursor are held; there is no abort, and all 8 slots are claimable afterwards.
- `db_engine::tests::retained_retry_owners_release_their_guard_before_resubmission`: no `if let`/`while let` over `retry_job.lock()` whose body relocks it.
- `database_create_catalog_drop_terminal_close_…` / `database_catalog_bootstrap_lost_handle_…` now attribute every retired owner to a lane grant cumulatively (`opportunities` / new cfg(test) `driver_grants`).

### Late-found product defects (fixed)
| Defect | Evidence | Fix |
|---|---|---|
| **Deadlock**: bootstrap retry callback `if let Some(..) = state.retry_job.lock().take() { … submit_exact → *retry_job.lock() = … }` (edition-2021 `if let` keeps the guard for the whole body; a second refusal relocks) | watchdog: `retry_job` mutex held forever, phase Poll, authority Queued (~2 % of `Database::open_at` on a fresh root hung) | guard released before the body at 4 sites (bootstrap, submit, history, artifact runner) + law |
| Lost wake in bootstrap/create-catalog `schedule()`: CAS Idle→Queued fails, the driver releases and reads `wake_requested` before the scheduler stores it | analysis of the hang path | Dekker-safe SeqCst loop: store wake, re-read authority, reclaim if the driver released |
| `Database::shutdown` failed with `Conflict("… retains 1 shared artifact authorities")` right after a submit (≈4 %): internal retained owners (submit state, mount replies, finishing driver jobs) still shared the authority `Arc` | `counts=[2]` with no caller handle alive | shutdown blocks only on caller handles (`ArtifactHandleLease` count per mount); the submit state also drops its authority at `finish()` |
| Dropped authority parked in `ClosingReady` with no retirement request (`ArtifactRunnerPoll::drop` path) | stuck `driver=5 polls=1 maintenance=true` (40 % of the retirement-slot law under load) | the transition requests retirement maintenance, as `ArtifactRunnerClosePoll` already did |
| vcs graph ran `db_actor::block_on` inside `VersionGraphFuture`s | law | removed (the future compiles `Send` without it) |


### Follow-up (H5 report: `database_catalog_bootstrap_real_max_plus_one_refusal_…` hung 18 min under fleet load)
- Root cause: another guard-relock deadlock, this time across a function call. `DatabaseCatalogBootstrapRejectedClose::retry` did `if let Some(job) = self.retry_job.lock().take() { self.submit_exact(job) }`, and on a second Saturated refusal `submit_exact` relocks `retry_job` → self-deadlock on the timer thread, while the test spins on `terminal_is_empty()` (the 9 % CPU H5 saw). It only shows under saturation, which is why the test passes alone in 0.05 s.
- Fix: the guard is released before the body (`⚙️engine/🦀️.rs`). An interprocedural scan (one call level, whole db crate, `wp-r4/scan-iflet-relock.py` extended inline) found no other site where an `if let … = X.lock()…take()` body calls a function that locks X.
- Law widened: `retained_retry_owners_release_their_guard_before_resubmission` also rejects `submit_exact(` inside such a body.
- I tried scoping every one of the 78 `if let … lock().take()` sites mechanically. It broke atomicity assumptions elsewhere (storage StaleGeneration, backend maintenance), so I reverted it completely, verified by diff: the only net brace/guard changes are the 5 intended sites.
- Post-fix nextest (`nextest-9.txt`, `nextest-10.txt`): 757–759/765. The failing set changes between runs, and it matches an **in-progress peer rework of backend retirement in `🗄️storage/🦀️.rs`** (live `[DEBUG]` prints, `close_scheduled`/`close_lane_turn` removed, new `db_io_backend_retirement_turn`, file mtime 07:19). The failures are storage `StaleGeneration{actual:0}` and backend-maintenance-hook laws, in code R4 did not touch. I can't give a stable green claim until that edit lands; my last clean baseline is `nextest-6/7` (765/765).

## Evidence

| Command | Result | Capture |
|---|---|---|
| repro (before), `[DEBUG]` witness | `driver=8 … close_error=Some(InvalidArgument("WAL has pending records; force_flush is required before close"))` forever | `repro-1.txt` |
| repro (after) | ok, 0.12 s | `repro-2.txt` |
| original command `cargo test … -- --exact db_artifact::tests::document_authority_submits_and_queries_over_finite_pool_turns` | ok, 0.68 s | `repro-final.txt` |
| parallel `--lib` (before) | SIGABRT "panic in a destructor during cleanup" (history admission) | `par-1.txt` |
| db `--lib` nextest baseline | 689 pass / 67 fail / 5 timeout | `nextest-1.txt`, `nextest-1-triage.txt` |
| db `--lib` nextest final ×2 | **765/765**, 765/765 | `nextest-6.txt`, `nextest-7.txt` |
| kernel `--lib --features sync,ureq` nextest | **1193/1193** | `kernel-nextest-1.txt` |
| `database_shutdown_cancellation_…` ×300 (hit `open_at` + shutdown each run) | 0 anomalies (was ~2 % hang + ~4 % fail) | loop script `loop-until-anomaly.sh` |
| `law_sync_convergence` stack | needed >64 MiB (96 MiB ok); submit future 2.24 MB → 318 KB after boxing, passes at 64 MiB | console |
| in-process serial `--test-threads=1` | 592 pass / 173 fail (156 × `DB I/O backend process credit exhausted`) | `serial-1.txt` |
| in-process parallel | killed at 580 s: 320 ok / 124 FAILED / hangs | `par-3.txt` |

## Files changed (R4)
- `🛢️db/🗿️artifact/🦀️.rs`: `ArtifactTurn::CloseFlush`, `close_flush*`, runner cancel path, `ArtifactRunnerPoll` retirement request, WAL field tail write, `HistoryReplayPhase::PageClose`, cursor `Drop` unwind-safe, `history_capacity_test_lock`, retry guard.
- `🛢️db/📝️wal/🦀️.rs`: `ArtifactWal::has_pending`/`close_flush`, `WalRecordBatch` boxed backing.
- `🛢️db/⚙️engine/🦀️.rs`: history admission `Drop`, `HistoryView` releases `terminal_state`, create-catalog (release on `into_parts`, `accepts_interruption`, publish-before-terminal, terminal/finished drive guard, witness counts, Dekker `schedule`), bootstrap (cancel restage, Dekker `schedule`, `driver_grants`, guard), submit state authority release + guard, history guard, `ArtifactHandleLease` + shutdown selection, vcs `block_on` removal.
- `🛢️db/🔢️index/🦀️.rs` (hole retirement, boxed `RunEntries`), `🛢️db/🗜️compact/🦀️.rs` (page credit), `🛢️db/🔄️sync/🦀️.rs` (rejected-close coalescing), `🛢️db/⌨️cli/🦀️.rs` (history retirement).
- Tests: `🗿️artifact/🧪️tests/🔬️unit`, `⚙️engine/🧪️tests/🔬️unit`, `⚙️engine/🧪️tests/🔬️vcs-integration-retained`, `🔄️sync/🧪️tests/🔬️unit`, `🔍️query/🧪️tests/🔬️unit`, `🗄️storage/🧪️tests/🔬️db-io-retained-fixtures`, `🗜️compact/🧪️tests/🔬️unit`, `🧪️tests/🔬️unit` (facade), `⌨️cli/🧪️tests/🔬️unit`.
- Not R4: a peer's pack `SchemaHash` rework briefly reddened 5 durable-group laws; P5 regenerated the fixtures.

## Gaps
- **Plain in-process `cargo test -p semio-framework-os-kernel-db --lib` is not green, serial or parallel.** Root cause: DB I/O backends are process-global (64 controls, bounded process bytes). A dropped `MemoryStorage`/`FsStorage` retires only when its close lane can run, and that waits on `pending_operations`/leases/the writer-release controller of engines and WALs the tests drop without `close_step`. In one process these leaks accumulate: a census after 48 engine tests showed 14 backends `close=true retired=false` holding ~158 MB of process credit. The laws assume process isolation, which nextest (the gate runner) provides. Getting it green in-process needs (a) every law to retire its engines/WALs/storages and (b) a decision on whether backend retirement should also run on a pool maintenance hook rather than only opportunistic `db_io_maintenance_step` calls. That is a separate, crate-wide work package, not a regression from this slice.
- `artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity` spins its retirement (a noop-waker WAL release poll, re-requested each step). Under load average ~37 a single iteration can exceed its 10 s budget (1/20 runs). Green in both final nextest runs.
- `scan-iflet-relock.py` (in `wp-r4/`) only covers the db crate. The same edition-2021 `if let … = x.lock()` relock hazard should be scanned across the other kernel crates.
