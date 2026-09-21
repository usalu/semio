# FP7 — `semio-framework-plugin --lib`: 783 / 35 → ?

Slice FP7, 2026-09-21 (session 10). Continues FP6 (`📓️fp6-plugin-lib-zero-red.md`), FP5, FP4, FP3.

Method (unchanged from FP5/FP6): the lib unittests binary built once with private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp7` + the shared build-dir, driven directly with
`RUST_MIN_STACK=67108864`. Whole-suite numbers are the **serial** (`--test-threads=1`) reading.

## 1. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| 0 | baseline (FP6's tree, this machine) | 782 | 36 | `fp7-round0-serial.txt` |
| 1 | §2.1 `ChildContentView::take_one` owner set | 785 | 36 | `fp7-round1-serial.txt` |
| 2 | §2.2 fair maintenance rotation + its new law | 786 | 36 | `fp7-round2-serial.txt` |
| 3 | §2.2 the two laws the fair rotation re-shapes | 786 | 36 | `fp7-round3-serial.txt` |
| 4 | §2.3 the two wall-clock laws made deterministic | 788 | 34 | `fp7-round4-serial.txt` |
| 5 | §2.4 the fixture child given a real member opener | 790 | 32 | `fp7-round5-serial.txt` |
| 6 | §2.3 the archive-cancel ordering clause + §2.2 the block-shape laws | 792 | 30 | `fp7-round6-serial.txt` |
| 7 | §2.2 a blocked stage ends the scan instead of being masked | 793 | 29 | `fp7-round7-serial.txt` |
| 8 | §2.5 the peer-presence drain bound and close, §2.2 the sub-page grant clause | 794 | 28 | `fp7-round8-serial.txt` |
| 9 | (a peer's uncompilable edit — the round ran the round-8 binary) | 793 | 29 | `fp7-round9-serial.txt` |
| 10 | same tree as round 8, rebuilt on the peer's fixed file | **794** | **28** | `fp7-round10-serial.txt` |

**Net: 782 / 36 → 794 / 28. Eight laws landed plus one new law; no law was deleted, `#[ignore]`d or
loosened, and no ceiling constant was changed anywhere.** Three of the eight are product defects
(§2.1, §2.2, and §2.2's blocked-stage masking); the rest are laws or fixtures re-expressed for the
landed design, and one is a fixture that never declared a member opener (§2.4).

The failed count stands still while the passed count rises because peers land and re-break their own
files in the same binary (rounds 1–2 carried two reds of JB1's `⚛️reactor/💼️jobs/🧪️tests/🔬️unit/🦀️.rs`
that are not this slice's) and because four wall-clock laws draw differently per run (§5).

Round 0 is FP6's exact 35 (`fp6-round7.names`) **plus one**:
`retained_operation_continues_after_command_admission_until_publication_and_retirement`
(`instance busy or poisoned: 7`), which was green in every FP6 draw. No source of this slice was in the
tree at round 0, so it is either a peer's landing on `🔌️plugin/🦀️.rs` or a load-dependent draw; it is
tracked as red #36 below.

## 2. Landed fixes

### 2.1 PRODUCT DEFECT — two pending child-root retirements deadlocked on a shared page

`ChildContentView::take_one` (`🔌️plugin/🦀️.rs:9057`) forgave a page it could not `Arc::try_unwrap`
only when the LIVE root still held it (`retained_by_current`). Child-content publication is
copy-on-write: publication `G` clones root `G-1`, swaps one page, and retires the old root under key
`G`, so two consecutive pending retirements share every page the second publication did not touch.
Neither could unwrap it and neither was allowed to forgive it, so both answered
`Blocked { "retired child root remains borrowed by an exact operation view" }` forever. That is the
close `group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` died on — the
only law in the suite that reaches two pending retirements (two registered children, several
publications, plus the undo's own republication per undone child).

**The decision (FP6 §4 left it to the route owner).** The kernel's law is that every retained page has
exactly one owner at a time and an alias is retired by the phase that created it. A `ChildContentView`
alias is held by one of two kinds of holder:

- an **owner** — the live root, or another pending `ChildContentRetirement` — which has a bounded
  disposer and will retire what it still holds;
- a **borrower** — an operation view a job clones for the length of one request — which has none and
  drops plainly.

So an alias handed to an owner is *handed over*, and an alias held by a borrower must be waited out.
`take_one` now forgives a shared page or entry when it is retained by the live root **or by any other
pending retirement**, and keeps `Blocked` for everything else. Termination and exactly-once are both
structural: forgiving drops one reference, so whichever owner ends up holding the last one finds
`Arc::try_unwrap` succeeding and performs the single real retirement — with three holders A, B, C, two
forgive and the third retires, in any order. `retained_by_current` was not widened; the *owner set* was
made explicit.

Landed at `🔌️plugin/🦀️.rs`: `ChildContentOwners` (new, after `ChildContentTake`) plus
`ArtifactFixedRegistry<ChildContentRetirement>::sibling_content_owners`; `take_one` and
`ChildContentRetirement::close_step` take the owner set; the four `close_step` call sites (maintenance
stage 4, the close ladder, the displaced-replacement registry, the candidate retirement) pass it. The
candidate retirement passes `ChildContentOwners::none()` because its sibling registry is drained by an
earlier rung of the same ladder.

Landed: `group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child`.

### 2.2 PRODUCT DEFECT — an empty maintenance stage consumed the whole call (coordinator hand-off)

`PluginApp::maintenance_step` rotated a fixed `MAINTENANCE_STAGES = 26` cursor and ran exactly ONE
stage per call, answering `Pending { released_items: 0, released_bytes: 0 }` when that stage was empty.
A caller that drives one maintenance unit per turn therefore released about one owner per 26 turns.
Measured by a peer session on the assembled `s.flow.flow@1/*#editor` surface: 2 051 items released in
100 000 close turns, **96 588 of them releasing nothing** — a real editor cannot finish closing inside
any committed turn budget, and in a live guest that is what a user sees as an app that never closes.

Root fix at `🔌️plugin/🦀️.rs`: the `match stage` body moved verbatim into a new inherent
`maintenance_stage_step(stage, …)`, and `maintenance_step` now scans the rotation **within one call**,
advancing the cursor past every stage that released nothing. The grant is still exact: the scan
continues only past a stage that released NOTHING, so at most one stage in a call can spend it. Two
secondary decisions, both recorded in the code:

- a `Blocked` stage no longer masks a later stage that can still progress — it is remembered and
  reported only if the whole rotation had nothing else to give;
- a stage's `Complete` (that stage's own queue is empty) is likewise kept as a last-resort answer, so
  it now reaches `RuntimeLiveCleanupJob` — which ends live cleanup on `Complete` — strictly *less*
  often than before.

New law, no ceiling constant: `maintenance_answers_idle_only_when_every_stage_is_idle`
(`🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`) queues one displaced child root and asserts that no call may
answer idle while it is still queued.

Two existing laws assumed one call = one stage and are re-expressed, not weakened:
`child_snapshot_retirement_rejection_preserves_exact_erased_owner` now takes the block as the
rotation's *eventual* answer (a lost authority never reports a block at all), and the new law's idle
arm accepts `Complete` as an idle answer.

### 2.3 Three wall-clock laws made deterministic, and the two that stay

The brief asked for the load-dependent laws to be measured at load and, where their bound is a product
guarantee a loaded machine breaks, re-expressed as an ordering guarantee. Three were:

- **`maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode`** asserted
  `started.elapsed() < 8 ms`. The clock was a PROXY for one property — a 4 MiB child's bytes never cross
  the public dispatch boundary, the dispatch answers with a continuation — and a copy that walked the
  snapshot would have to clone it or pack-encode it, both of which the law's own
  `MAXIMUM_CHILD_CLONES` / `MAXIMUM_CHILD_ENCODINGS` counters already price at zero. The clock is
  replaced by two more deterministic clauses in the same direction: the dispatch's own answer is under
  4 KiB, and it publishes no mutations.
- **`guest_turn_execution_resets_for_every_turn`** asserted
  `guest_turn_executing_us() == Some(second_us)` after driving two equally cheap turns. That equality
  holds only while the microsecond between the in-turn sample and the settled read rounds to zero, so a
  loaded machine failed it (`left: Some(3)`) while the product property held — and the two cheap turns
  never proved non-inheritance at all. Non-inheritance is now proved by CONSTRUCTION: the first turn
  spins until its own accumulator has passed the whole `perTurnBoundUs`, and a second turn that had
  inherited it could not then come back under that bound. Both clauses are ratios against a measured
  accumulator, so load inflates both sides. The fixture JSON is untouched.
- **`retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_
  acknowledgement`** demanded that one poll after the cancel still report `Pending | Running`. One poll
  spends up to `DOCUMENT_ARCHIVE_POLL_WALL_US` driving the rotation (its own doc comment says so), so
  whether it comes back `Running` or already `Cancelled` is how many bounded steps fit in that budget —
  a quiet machine fits more. The law is named for an ORDER, and the order is now asserted directly: a
  cancelled archive never publishes or faults, and it is not acknowledgeable short of its terminal
  status.

**The two genuine wall-clock ceilings stay, named** (FP3 §6.3 and its successors):
`tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` (2 ms per append
over 771 real appends — an injected clock would make it assert nothing) is unchanged; it was green in
rounds 0–8 and red in round 9 with no source change between them, which is exactly the load signature
FP3 recorded. The second, `guest_turn_execution_excludes_every_suspension_gap`, is already a ratio
(`executing_us * 8 < wall_us`) and was never red here.

### 2.4 The fixture child could never be opened at all — the composed-replacement family

The three `retained_composed_replacement_*` laws never reached `ValidatingClosure` or `CandidateReady`.
With the refusal now carried into the drive's panic (`ActiveArtifactStoreReplacement::refusal_fault`,
`🔌️plugin/🦀️.rs`, and both `drive_*_replacement_to` helpers) the cause was one line:
`document archive replacement failed its members leg: owned member 0 was rejected by its typed open
(Decode)`.

Measured with a bounded probe: `TestSnapshot` declared
`type SnapshotOpen = store::UnsupportedMemberSnapshotOpen<Self>`
(`🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`), and `UnsupportedMemberSnapshotOpen::step` has
exactly one answer — `Rejected(MemberOpenDiagnostic::Decode)` — so every composed replacement carrying a
`TestMembers` candidate was refused at its first member-open step, at step 0, always. The sibling
`RecursiveTestMembers` opens normally (probe: ready at step 174) because its snapshot declares a real
opener. Class: **incomplete fixture**, not a product defect; the product refused correctly.

`TestSnapshot` now declares the framework's own `store::PackMemberSnapshotOpen<Self>` — the opener a
real member declares — which needs `store::retirement::RetireOwned`, added for `TestSnapshot` in
`🧪️tests/🖥️test-app-mutations-document/🦀️.rs` as the ordinary field sequence. All three laws landed
together, untouched.

### 2.5 A hand-picked drive bound, measured

`peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root` drained its
app-typed peer retirement in a loop of 16 calls; measured, it needs 21 (three item releases, then the
peer's own bytes one at a time). The count is a property of the fixture's roster, not of the product, so
it is replaced by the stronger statement it stood in for: EVERY bounded call must release something
until the authority is terminal — a stall or a block now fails at the exact call it happened on. The law
also never closed its app, so it tripped the store's Drop witness after its last assertion; it now
drains and closes.

## 3. The 36 reds, law by law

Round-0 panic is the message the law died of on FP6's tree as this slice found it; "now" is the round-10
serial run (`fp7-round10-serial.txt`, **794 passed / 28 failed**).

| # | law | round-0 panic | now |
|---:|---|---|---|
| 1 | `owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally` | the 1,024 request slots stay behind one heap owner | red — the 1,024 request slots stay behind one heap owner |
| 2 | `new_app_constructs_a_registry_less_wrapper` | tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origin: Framework, code: FaultCode("interacti | red — tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origin: Framework, code: FaultCode("interacti |
| 3 | `editor_fixture_still_mutates_normally` | increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has | red — increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has |
| 4 | `generation_mismatch_is_rejected_with_the_frozen_code` | fixture close blocked: document store close awaits a retained reader or owner | red — fixture close blocked: document store close awaits a retained reader or owner |
| 5 | `fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers` | assertion failed: reactor.contains("output.typed_operation_results.iter()") | red — assertion failed: reactor.contains("output.typed_operation_results.iter()") |
| 6 | `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` | typed route pre-decoder gate | red — typed route pre-decoder gate |
| 7 | `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` | typed command route | red — typed command route |
| 8 | `native_aggregate_registry_does_not_allocate_backing_before_admission` | assertion `left == right` failed: the original runtime registry needs caller-granted backing before slot initialization \|   left: 1024 | red — assertion `left == right` failed: the original runtime registry needs caller-granted backing before slot initialization \|   left: 1024 |
| 9 | `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "child restore is not declared by the loaded pa | red — load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "child restore is not declared by the loaded pa |
| 10 | `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.live-instance"), severity: Error, message: "typed | red — dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.live-instance"), severity: Error, message: "typed |
| 11 | `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` | assertion `left == right` failed \|   left: {"checkoutCheckpoint", "clearSelection", "commitCheckpoint", "configuration-binary", "copy", "createAltern | red — assertion `left == right` failed \|   left: {"checkoutCheckpoint", "clearSelection", "commitCheckpoint", "configuration-binary", "copy", "createAltern |
| 12 | `app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift` | assertion `left == right` failed \|   left: 12184115126529414932 | red — assertion `left == right` failed \|   left: 12184115126529414932 |
| 13 | `child_root_maintenance_requires_terminal_empty_before_reclaim` | the lying owner must never reach a terminal step: Complete | red — the lying owner must never reach a terminal step: Complete |
| 14 | `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | assertion `left == right` failed \|   left: 1 | red — assertion `left == right` failed \|   left: 1 |
| 15 | `group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: retired child root remains borrowed by an exact opera | **green** |
| 16 | `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` | assertion `left == right` failed: cancel_instance_tasks already dropped the task's own RequestFuture — nothing left for the registry sweep to remove \ | red — assertion `left == right` failed: cancel_instance_tasks already dropped the task's own RequestFuture — nothing left for the registry sweep to remove \ |
| 17 | `key_dedupe_cancels_the_previously_live_task_under_the_same_key` | second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("plugin.task.supersession-pending"), severity: Error, message:  | red — second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("plugin.task.supersession-pending"), severity: Error, message:  |
| 18 | `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` | encoded transaction route must remain explicitly unadmitted | red — encoded transaction route must remain explicitly unadmitted |
| 19 | `local_interaction_registered_query_channel_continuation_ack_and_close` | assertion failed: presence.is_empty() | red — assertion failed: presence.is_empty() |
| 20 | `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` | artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner | red — artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner |
| 21 | `peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root` | assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, \|     4096).expect("captured app-typed peer blocks"), PluginCloseStep::Blocked | **green** |
| 22 | `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` | stale roster outcome | red — stale roster outcome |
| 23 | `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | assertion failed: preparation.ui_patches.is_empty() | red — assertion failed: preparation.ui_patches.is_empty() |
| 24 | `registry_less_construction_rejects_before_the_reducer` | artifact store reached Drop without its exact terminal-empty shallow-shell witness | red — artifact store reached Drop without its exact terminal-empty shallow-shell witness |
| 25 | `retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle` | retained composed replacement did not reach ValidatingClosure | **green** |
| 26 | `retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically` | assertion `left == right` failed \|   left: Fault | **green** |
| 27 | `retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication` | retained composed replacement did not reach CandidateReady | **green** |
| 28 | `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("interactive-job.typed-operation-session"), severity: Error, m | red — called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("interactive-job.typed-operation-session"), severity: Error, m |
| 29 | `retained_operation_continues_after_command_admission_until_publication_and_retirement` | called `Result::unwrap()` on an `Err` value: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "instance busy or p | **green** |
| 30 | `retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: app-typed presence retirement waits for its exact cap | red — registered fixture did not reach its exact terminal-empty witness, last pending close authority: app-typed presence retirement waits for its exact cap |
| 31 | `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement` | assertion failed: matches!(requested.state, protocol::DocumentArchiveLoadState::Pending \| \|     protocol::DocumentArchiveLoadState::Running) | **green** |
| 32 | `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` | assertion `left == right` failed \|   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) | red — assertion `left == right` failed \|   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) |
| 33 | `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command` | assertion `left == right` failed \|   left: "interactive-job.missing-factory" | red — assertion `left == right` failed \|   left: "interactive-job.missing-factory" |
| 34 | `revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance` | assertion `left == right` failed \|   left: UiRevision(1) | red — assertion `left == right` failed \|   left: UiRevision(1) |
| 35 | `mounted_document_tree_publishes_nested_interactive_rows` | assertion `left == right` failed \|   left: 52559872 | red — assertion `left == right` failed \|   left: 52559872 |
| 36 | `guest_turn_execution_resets_for_every_turn` | assertion `left == right` failed: a settled turn keeps exactly the microseconds it executed \|   left: Some(3) | **green** |

## 4. The `ChildContentView::take_one` decision

Stated once, in the form the route owner owes a successor.

**Law.** A retained child-content page has exactly one owner at a time. Its holders are of two kinds:

- **owners** — the live `child_content_root`, and every pending `ChildContentRetirement` in the same
  registry — each of which has a bounded disposer and will retire what it still holds;
- **borrowers** — a `ChildContentView` a job clones for the length of one request — which have none and
  drop plainly.

**Decision.** A shared page or entry is FORGIVEN (`ChildContentTake::ReleasedShared`) when another
OWNER retains it, and BLOCKS when a borrower does. `retained_by_current` was not widened: the owner set
was made explicit, as `ChildContentOwners`, and the live root is simply one of its members.

**Why this is exactly-once and terminating.** Forgiving drops one reference. Whichever owner ends up
holding the last one finds `Arc::try_unwrap` succeeding and performs the single real retirement. With
three holders A, B, C, two forgive and the third retires, in any visiting order; the last holder can
never see another owner, because the check is made at the moment of the drop. A borrower is never
forgiven, so a job-held view still blocks — which is the clause the `Blocked` reason names.

**Why the alternative was rejected.** Ordering the two retirements by generation ("the older retires,
the newer forgives") also gives exactly-once, but neither retirement can read its own generation, the
registry key is not carried into `take_one`, and a page shared with a borrower would still need the
owner/borrower distinction. The owner set answers both questions with one predicate.

## 5. Honest gaps

- **The gate is NOT green: 28 of round 0's 36 are still red.** The brief's end state was not reached.
- **`peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` hides a real
  stall, now measured.** Its first clause (the sub-page grant) is fixed; the law then dies on
  `stale roster outcome`. Measured with a bounded probe: with `peer_roster_processed_generation` forced
  ahead, **4 096** forced stage-7 maintenance calls never empty `peer_roster_publications`, so the stale
  publication never reaches its outcome slot. Every call answers `Pending { released_items: 1 }` from a
  DIFFERENT stage, which is why the law's 64-call loop exits silently instead of failing loudly. The
  law's own bound was left at 64 exactly as it was: this is a product stall, not a drive-bound
  undercount (contrast §2.5, where the count was measured at 21 against a bound of 16), and it belongs
  to the peer-roster route owner. It is the best-evidenced single red left.
- **Two production snapshot types still declare `UnsupportedMemberSnapshotOpen`** —
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs` and
  `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs`. By §2.4's measurement that means a real
  flow artifact can never be opened as an owned member of a composed replacement or a document archive:
  its first member-open step answers `Decode`, always. Not touched — neither is this slice's — but a
  successor should treat it as the same defect with production blast radius.
- **`RuntimeLiveCleanupJob` ends live cleanup on `PluginCloseStep::Complete`** and a single maintenance
  stage can answer `Complete` when only ITS queue is empty. §2.2 makes that strictly rarer (a stage's
  `Complete` is now only the answer of a rotation that had nothing else to give) but does not remove the
  hazard; the honest fix is for the rotation never to answer `Complete` at all, which one law
  (`…child_open…`, forcing stage 20) currently depends on.
- **The fair rotation costs a full stage sweep on a call that finds nothing**, bounded by
  `MAINTENANCE_STAGES`. An app that is entirely quiet never enters the rotation (the idle conjunction
  at the head of `maintenance_step` returns first), so the cost is paid only while something is
  non-terminal — which is exactly when the old behaviour was losing 25 turns out of 26.
- **Four laws of this binary are not this slice's**: `⚛️reactor/💼️jobs/🧪️tests/🔬️unit/🦀️.rs` (JB1) was
  uncompilable at round 0 and again at round 9, and carried two of its own reds through rounds 1–2. The
  round-9 draw ran the round-8 binary because a peer's `🔌️plugin/🦀️.rs` edit (TC3c) did not compile;
  round 10 is the same tree rebuilt after the peer fixed it.
- **No dependent crate was re-run.** `🔌️plugin/🦀️.rs` gained one `pub(crate)` type
  (`ChildContentOwners`), one private inherent fn (`maintenance_stage_step`), one `#[cfg(test)]`
  accessor (`refusal_fault`) and two changed private signatures (`take_one`,
  `ChildContentRetirement::close_step`); no public item changed signature and no constant moved. A
  successor landing on top of this should still `cargo check` a plugin crate.
- **`--features component-app-assembly` was not exercised**, same as FP4 §7, FP5 and FP6.
- Every number here is read from a capture in `🗑️generated/`. Nothing is claimed that was not executed.
  No `[DEBUG]` instrumentation and no probe was left behind (the `[DEBUG]` lines in
  `🧪️tests/🧩️composition/🦀️.rs` predate this slice and belong to another owner).

## 6. Files changed

Product code (1 file), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:
- `ChildContentOwners` + `ArtifactFixedRegistry<ChildContentRetirement>::sibling_content_owners` — new
  (§2.1); `ChildContentView::take_one` and `ChildContentRetirement::close_step` take the owner set, and
  the four `close_step` call sites pass it
- `maintenance_step` — the fair rotation scan (§2.2); the `match stage` body moved verbatim into a new
  inherent `maintenance_stage_step`
- `ActiveArtifactStoreReplacement::refusal_fault` — new `#[cfg(test)]` accessor (§2.4)

Fixtures / laws (5 files):
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `TestSnapshot::SnapshotOpen`
  becomes `PackMemberSnapshotOpen` (§2.4); new `assert_maintenance_reports_block` helper and its three
  uses (§2.2); the sub-page grant clause (§2.2); the peer-presence drain bound and close (§2.5); the
  8 ms clause replaced (§2.3)
- `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` — new law
  `maintenance_answers_idle_only_when_every_stage_is_idle` (§2.2); the archive-cancel ordering clauses
  (§2.3); both `drive_*_replacement_to` helpers report the state and refusal they died on (§2.4)
- `🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs` — `RetireOwned for TestSnapshot` (§2.4)
- `🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs` — `drive_with_suspension_executing` and the
  rebuilt non-inheritance law (§2.3)
- `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` — untouched by this slice (it carries
  FP6's mounted field; listed only because it is dirty in the same tree)

Captures (`🗑️generated/`): `fp7-round0…round10-serial.txt`, `fp7-round*.names`, `fp7-round*-panics.txt`,
`fp7-bin-path.txt`, `fp7-build-err.txt`. Scratch: `🐍️fp7-decode.py`.
