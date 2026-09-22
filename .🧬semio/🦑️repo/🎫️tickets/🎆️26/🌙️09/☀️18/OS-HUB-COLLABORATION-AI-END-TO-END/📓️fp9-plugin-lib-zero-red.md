# FP9 — `semio-framework-plugin --lib`: 806 / 16 → 816 / 6

Slice FP9, 2026-09-21 (session 13). Continues FP8 (`📓️fp8-plugin-lib-zero-red.md`), FP7, FP6, FP5.

Method (unchanged from FP5–FP8): the lib unittests binary is built once with the private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp9` + the shared build-dir, copied out of the
shared build-dir, and driven directly with `RUST_MIN_STACK=67108864 CARGO_INCREMENTAL=0`.
Whole-suite numbers are the **serial** (`--test-threads=1`) reading.

## 1. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| 0 | baseline — FP8's final 16 reproduced, name for name, with ONE load substitution (see below) | 806 | 16 | `fp9-round0-serial.txt` |
| 1 | §3.1 in-place member-ingress close, §3.2 lazy runtime-registry backing, §3.3 cancellation drops + the checkpoint law re-ordered, §3.4 the spawned-task instance bind, §3.5 the transaction fixture's rollback + backbone detach, §3.6 the framework owner type, §3.7 the bijection's two residues, §3.8 the identity oracle's domain, §3.9 the per-case patch surfaces | 811 | 11 | `fp9-round1-serial.txt` |
| 2 | §3.6's copy/cut raw-wire caps re-recorded from the `framework_reserved_job!` `$raw` column, §3.7's refusal clause re-expressed to `not-ui-safe` | 812 | 10 | `fp9-round2-serial.txt` |
| 3 | §3.6 and §3.7 close the fixture apps they build, §3.9's surface made unique to the law | 813 | 9 | `fp9-round3-serial.txt` |
| 4 | §5.2 the patch law restructured onto the landed same-call publication; the encoded-transaction refusal made self-describing | 815 | 7 | `fp9-round4-serial.txt` |
| 5 | §3.10 the encoded transaction route re-expressed to the answer it actually gives | 816 | 6 | `fp9-round5-serial.txt` |
| 6 | §5.4 the composed shim stops discarding what the composed pipeline produced, plus its diagnostic | **816** | **6** | `fp9-round6-serial.txt` |

**Net: 806 / 16 → 816 / 6.** Ten of the sixteen landed; no law was deleted, `#[ignore]`d or loosened,
and no ceiling constant changed anywhere. BOTH wall-clock ceilings drew green in the final round
(`tool_run_overlay_append_…` red in rounds 0 and 3 under load, green in 2, 4, 5 and 6;
`retained_operation_continues_…` green in every round of this slice), so all six survivors have a
source cause and each is diagnosed below: §4.3's fixture, §5.1's product gap, and four
composed-document / worker-session reds (§4.2, §5.4).

Round 0's set differs from `fp8-final.names` in exactly one row: FP8's load draw was
`retained_operation_continues_after_command_admission_until_publication_and_retirement`, mine was
`tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks`
(`14 of 771 overlay appends exceeded 2ms (worst 157.6 ms)`; the suite took 172 s against FP8's 25 s —
the fleet was at load ≈ 69). Both are the two genuine wall-clock ceilings FP3 §6.3 and FP8 §3.2 name
and neither has a source cause. **14 of the 16 have a source cause.**

## 2. The sixteen

| # | law | round-0 panic | class | status |
|---|---|---|---|---|
| 1 | `owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally` | `the 1,024 request slots stay behind one heap owner` | product (footprint) | §3.1 |
| 2 | `native_aggregate_registry_does_not_allocate_backing_before_admission` | `left: 1024 / right: 0` | product (eager 4 MiB) | §3.2 |
| 3 | `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` | `left: 1 / right: 0` | product (cancellation) | §3.3 |
| 4 | `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | `interactive-job.live-instance` | fixture | §3.4 |
| 5 | `generation_mismatch_is_rejected_with_the_frozen_code` | `fixture close blocked: document store close awaits a retained reader or owner` | fixture | §3.5 |
| 6 | `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` | TypeId inequality | law (wrong owner named) | §3.6 |
| 7 | `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` | key-set inequality | law (two named residues) | §3.7 |
| 8 | `app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift` | `left: 12184115126529414932` | oracle drift | §3.8 |
| 9 | `editor_fixture_still_mutates_normally` | `interactive-job.unknown-key` | fixture needs a manifest AND an owned factory | §4.3 |
| 10 | `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | `child restore is not declared by the loaded parent snapshot` | fixture: `TestSnapshot` declares no child refs at all | §5.5 |
| 11 | `child_root_maintenance_requires_terminal_empty_before_reclaim` | `the lying owner must never reach a terminal step: Complete` | the law pins one stage; `maintenance_step` now rotates all of them per call | §5.6 |
| 12 | `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | `left: 1 / right: 2` (only the parent's mutation) | product: the composed settle path drops the child lane | §5.4 |
| 13 | `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` | `encoded transaction route must remain explicitly unadmitted` | law pinned a deleted fault code | §3.10 **green** |
| 14 | `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | `preparation.ui_patches.is_empty()` | the clause contradicted a landed same-call publication | §5.2 **green** |
| 15 | `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | `interactive-job.typed-operation-session` | not diagnosed (worker-session admission leak class) | open |
| 16 | `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` | `14 of 771 appends exceeded 2 ms` | wall-clock ceiling under fleet load | green in rounds 2, 4, 5, 6 |

## 3. Landed fixes

### 3.1 PRODUCT DEFECT — the owned-document member ingress registry carried a request slot inline

`OwnedDocumentMemberIngressRegistry` keeps its 1,024 request slots behind one `Box<[MaybeUninit<…>]>`,
but it ALSO held `close_active: ManuallyDrop<Option<OwnedDocumentMemberIngress>>` — one whole
`MemberOpenRequest` inline — so the registry shell itself was past the kilobyte the law fixes as its
stack footprint. Root fix at `🔌️plugin/🦀️.rs`: the incremental close now runs IN PLACE on the
occupied slot (`slots[ordinal].assume_init_mut()`), detaching the ingress only once its own
`close_step` answers `Complete` with a truthful terminal witness; `close_active` is gone and
`terminal_is_empty` is `len == 0`. A refusal still leaves the request exactly where it was, and a
zero grant still answers `Pending { 0, 0 }`.

### 3.2 PRODUCT DEFECT — four runtime registries reserved ~4 MiB each before any instance existed

`RuntimeInstanceRegistry::new()` called `try_reserve_exact(PLUGIN_RUNTIME_INSTANCE_SLOTS)` and took
allocator success AS the admission (`allocation_admitted = slots.try_reserve_exact(..).is_ok()`). A
`PluginRuntime` holds four of them, and the actor one is 1,024 × `RuntimeActorAuthority` (a
`[u8; 4096]` each) = **4 202 496 bytes** measured by the law itself — reserved contiguously up front by
every actor, including one that never opens a single instance. In a guest whose `dlmalloc` never
returns memory that is the exact shape recorded in memory `project-guest-contiguous-request-ceiling`.

Root fix at `🔌️plugin/🦀️.rs`: `new()` constructs EMPTY; a new `admit_backing()` takes the one fixed
allocation once, idempotently, and a refused reservation leaves the registry as empty as it was.
`can_insert` now takes `&mut self` and admits the backing itself, so a preflight that answers `true`
is a promise `insert_admitted` can keep — the OOM refusal path the old `allocation_admitted &&` gate
gave is preserved exactly, not traded for a panic. Two call sites became `try_borrow_mut`
(`plugin_open_actor_instance`, `plugin_create_app_with_id`).

The sibling law `runtime_instance_registry_has_fixed_capacity_collision_and_reuse` asserted
`registry.allocation_admitted` immediately after `new()` — it contradicted the aggregate law outright.
It is re-expressed, not weakened: it now asserts backing is NOT admitted at construction and IS after
the first admission gate. The aggregate law gained the same second half plus the exact reserved-byte
derivation, and its `eprintln!("[DEBUG] …")` is gone (its numbers are in the assertion messages).

### 3.3 PRODUCT DEFECT — instance-close cancellation POLLED the tasks it was supposed to drop

`cancel_instance_tasks_step`'s `#[cfg(not(test))]` arm hands the sweep to
`ReactorExecutor::close_instance_step`, which DROPS each future. Its `#[cfg(test)]` arm instead called
`poll_one` and refused to remove a task that answered `Pending` — so a parked task was never cancelled
at all, which is why `instance_close_cancellation_…` found one registry slot left where it demands
zero. Three sources agree on the contract and none of them agrees with that arm: the function's own
doc ("drops every task `instance` owns … dropping its future, and everything IT owns including any
parked `RequestFuture`"), the production arm, and `RequestRegistry::begin_cancel_instance`'s doc
("No wake/notify: `cancel_instance_tasks` drops that instance's owning `AsyncTask` futures … so
nothing is left to observe a 'cancelled' resolution — this is cleanup, not notification"). AGENTS.md's
"cancellation for every expensive operation" is the same statement from the product side.

Root fix at `⚛️reactor/🦀️.rs`: the native arm `detach`es the future and removes the record, exactly
like the keyed-dedupe path FP8 §2.11 landed. The docstring now states that BOTH arms drop.

FP8 §4.2 reverted this because it turned `checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume`
red: that law needed the cancelled task polled once so it would observe its retired request. It is
re-expressed to the close ladder's OWN order instead — and to the order the completion oracle already
declared and nothing joined, `execution: [… "await-request-retirement-before-observation"]`.
`ReactorCloseState` completes `requests_complete` BEFORE `tasks_complete`, so a task still live when
the registry sweep runs can only ever see its request as retired, never as a successful host response.
The law now retires the requests first, polls the still-live task exactly once through a new driver
helper `poll_instance_tasks_once` (the executor's own `poll_one`; the retirement deliberately issues no
wake, so `run_until_idle` alone would never visit the parked task — that is why FP8's attempt was
flaky), and only then cancels. Both oracle rows `completionFaultCode`/`completionFaultMessage` stay
joined, `tasksAfterCancel` is now read from the fixture instead of a literal, and the `execution` row
is joined for the first time.

### 3.4 The spawned-task law never bound the instance it dispatched under

`a_spawned_task_awaits_a_real_request_…` uses instance 501 but `contract_app_raw()` binds the
fixture's default `meta().instance_id` (1), and `dispatch_typed_command_inner` refuses
`interactive-job.live-instance` for any command whose `meta.instance_id` is not the mounted live
instance. The law now binds 501 — the id `spawn_task` is keyed off — before dispatching.

### 3.5 The generation-mismatch fixture closed a store whose backbone was still attached

`generation_mismatch_is_rejected_with_the_frozen_code` is the one transaction law that attaches a
`MemoryBackbone` pair, and it closed `sender` with the near end still attached and `far` still alive:
the document-store close answers `Blocked { "document store close awaits a retained reader or owner" }`
(memory `project-memory-backbone-detach-before-close`). The law now detaches the backbone and drops the
far end before closing, and — since a rejected commit RESTORES the pending transaction rather than
discarding it (contract §5.8) — takes the explicit `transaction_rollback` that refusal invites, which
is the clause the law was missing about its own subject.

### 3.6 The framework-action law named the wrong owner

`shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` builds its wrapper on
`TestApp<false, TEST_APP_TOOLS_NONE>` but asserted `TypeId::of::<FrameworkCopyJobFactory<TestApp>>()` —
and a bare `TestApp` is `TestApp<false, TEST_APP_TOOLS_FULL>`, a DIFFERENT owner with a different
`TypeId`. Every framework-reserved factory is parameterized by its owning app type, so the law now
names the exact owner it built (`type UnprovedFrameworkOwner = TestApp<false, TEST_APP_TOOLS_NONE>`)
in all 24 places.

### 3.7 The factory/declaration bijection has exactly two residues — both now named

`activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` demanded set equality
between activated factory keys and migrated declarations. It is not equality, and the two differences
are structural, not drift:
- registered without a declaration: `clearSelection`, `configuration-binary`, `import-media`,
  `interactionHover`, `interactionSelect`, `selectAll`, `setInteractionGranularity`,
  `setSelectionMode` — framework-reserved SURFACE verbs registered unconditionally, which this fixture's
  manifest never declares (no interaction topology, no window kit to mint them). Each is refused by name
  at `require_ui_safe_declaration` before its factory is ever reached.
- declared migrated with no factory: `setActiveUtility` — which `handle_action_invocation` routes
  DIRECTLY to `dispatch_emit` (`} else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {`),
  so a missing registration is not the dead-action defect it would be for any other migrated verb.
Both residues are asserted as EXACT sets (nothing is ignored), and the `setActiveUtility` exemption is
joined to the source line that justifies it, so losing that arm fails here.

### 3.8 The request-context identity oracle belonged to an older digest domain

`app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift`
joins `🧵️retained-command/🧫️fixtures/🧬️request-context.json`. The digest is a pure FNV-1a seeded with
its own domain tag, `ARC-CONTEXT-4`, which is bumped exactly when the identity's input set changes —
and it has since grown the window-config and window-transient lanes. The fixture is re-recorded
(`f8576d057f24de86` → `a916ac27433fd714`) AND gains a `domainTag` row that the law joins against the
tag read out of the live source, so the next bump fails on the clause that names the cause instead of
on a bare number. No other consumer reads this fixture (grepped): it is not a cross-language parity
oracle despite its name.

### 3.10 The encoded transaction route pinned a fault code that no longer exists

`local_interaction_cold_transaction_receipts_and_encoded_route_rejection` demanded an
`AppFrame::Error` carrying `plugin.command-route-state-machine-required`. That string occurs NOWHERE
in the tree except the law itself: the route-level refusal it named is gone. Measured (the law's
refusal message was made self-describing first, round 4), the route is now ADMITTED and answers with
its own typed frame — `TransactionPrepared { txn_id: "denied", rejection: <wire Fault> }` plus
`Done { in_reply_to: 0 }`, the fault being `transaction.unknown-mutation`, "owner-mutation payload did
not decode as a semio.test/v1 mutation: Pack(Truncated(0))". The law is re-expressed to that: no
`Error` frame may hide the outcome, the rejection must be non-empty (an empty payload is never
silently prepared), its decoded code is `transaction.unknown-mutation`, its message names the decoder,
and the rejected command still closes its own ingress slot. The protected property — an encoded
`TransactionPrepare` cannot smuggle an unvalidated owner-mutation payload past the decoder — is
unchanged; only the mechanism that enforces it moved from the route to the payload.

### 3.9 The reactor patch law's two cases shared one process-wide patch authority

`reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` runs
`for late_clock in [false, true]`, and `PATCHES`/`PENDING_PATCHES` are ONE thread-local authority (the
actor is a singleton by design) while only the `PluginRuntime` is rebuilt per case. Residue from the
first case was emitted by the second case's very first poll, and the law reported it as "a freshly
queued patch was emitted immediately" without saying which case failed. Each case now runs on its own
instance pair and its own surface id (`7/8` → `7:window`, `17/18` → `17:window`), and every clause
names its case. Its `eprintln!("[DEBUG] …")` is gone.

## 4. FP8's four named gaps

### 4.1 §4.1 `ArtifactStore::set_state` ABORTS on every refusal — **impl already landed by the store peer, law added here**

By the time this slice started, `🏪️store/🦀️.rs` already carried the fix FP8 named: `set_state` holds
its candidate in a slot (`let mut candidate = Some(envelope)`), delegates the reload proper to a new
private `adopt_state(&mut Option<…>)` that takes the envelope only on success, and hands whatever is
left to `ArtifactEnvelope::retire_unadopted` (`🦀️.rs:2686`) — which pops every fixed ledger tail-first
so the terminal-shell witness never fires on a populated drop. `ArtifactStore::new`/`construct` carry
the identical shape. That is KN2's landing, not mine.

What was missing is the law. Added at `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (new region
`🔖️RefusedReloadTests`): `a_refused_reload_returns_its_error_and_retires_the_candidate_it_consumed`
resets a live store with an envelope whose message ledger names an edit the history never recorded,
asserts the caller RECEIVES the `VcsError` (the process is still running to assert it — that is the
whole point), asserts the live store survives unchanged, and drives it to terminal-empty afterwards.

### 4.2 §4.2 `cancel_instance_tasks` — decided from the product's cancellation semantics, impl + both laws fixed

See §3.3. The contract is "cancellation drops, never polls", stated identically by the function's own
doc, the production `close_instance_step` arm and `RequestRegistry::begin_cancel_instance`'s doc, and
by AGENTS.md's cancellation rule. The test arm now drops; `instance_close_cancellation_…` is
unchanged; `checkpoint_then_restore_…` is re-expressed to the close ladder's own request-before-task
order, which the completion oracle already declared in a row nothing joined.

### 4.3 §4.3 `editor_fixture_still_mutates_normally` — NOT LANDED, with a sharper spec

The honest fix needs `SurfaceEditorFixture` to become a proved app, and the cost is not the manifest —
it is the factory. Measured against `DummyApp`, the minimum is: (a) override `ArtifactEditor::command_id`
so the verb is not the generic `"typed-command"` (which `validate_tool_job_rows` EXCLUDES from
`expected` by name, so a proof row for it can never be authoritative); (b) a manifest declaring that id
as an `app_command` under `.interactive_jobs(Migrated)`; (c) a `bounded_first_step_tool_proofs!` row
whose `controller` is the EditorApp's derived `surface_app_id`, not `EditorApp::APP_ID`'s `"surface"`
placeholder; (d) a `ToolJobFactory` + `ArtifactOwnedToolJobFactory<Owner = EditorApp<SurfaceEditorFixture>>`
with a real `InteractiveJob` body (~120 lines, the `DummyFixtureJob`/`DummyFixtureFactory` pair);
(e) `register_tool_job_factories` forwarding it. A generic bounded proof is NOT a shortcut: it resolves
to `interactive-job.missing-owned-reducer` at dispatch, so the law would still not mutate. No clause was
weakened in the meantime; the law still dies on the fail-closed refusal (`interactive-job.unknown-key`).

### 4.4 The third `UnsupportedMemberSnapshotOpen` — fixed at the root, with a law

`♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:424` declared
`store::UnsupportedMemberSnapshotOpen<Self>` for `DagSnapshot`, whose `step` has exactly one answer —
`Rejected(MemberOpenDiagnostic::Decode)` at step 0 — so every composed replacement and every document
archive carrying a real DAG member was refused before it began. Fixed like flow's `🌿️vcs`:
`DagSnapshot` now declares `store::PackMemberSnapshotOpen<Self>` (it already had `ArtifactPack`) and
gains `RetireOwned` through a new `DagOwnedSnapshotCursor` over this module's own `DagRetirement`
frontier — one owner per turn under the caller's byte grant, with a zero grant reported as budget
exhaustion rather than as progress (flow's bare-`close_step` trap, ticket 26/09/09, in the DAG's shape:
`DagRetirement::close_step` answers `Blocked` for a zero grant). New law
`dag_opens_as_an_owned_member_through_its_own_pack_codec`
(`🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs`): names the declared opener, round-trips the real
`default_dag_document()` through the opener's own whole-pack decode, drives the owner cursor to
terminal-empty under a 4 KiB grant failing loudly on a stall, and proves a refused zero-grant turn
leaves the cursor able to continue.

## 5. Flow `retained::*` reds — all six are FLOW-side, named

`cargo test -p semio-framework-artifact-flow-flow --lib retained` → **8 passed, 6 failed**
(`fp9-flow-retained.txt`), the exact six FP8 §5 exposed when it unblocked that crate's test target.
Every one of them lives in flow's OWN retained module
(`🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/**`), not in the framework:

| law | panic | flow-side cause |
|---|---|---|
| `retained::copy::tests::flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages` | `positive copy close grant blocked` (`📑️copy/🧪️tests/📑️copy/🦀️.rs:44`) | `CopyCursor::close_step` (`🧵️retained/📑️copy/🦀️.rs:398`) drives the ACTIVE root retirement with a bare `active.close_step(1, maximum_bytes)` and returns its answer verbatim, so a `Blocked` owed-allocation turn bubbles straight out — the trap flow's own doc names (`close_page` is "the single entry point every retained driver should use"; a bare `close_step` answers `Blocked` while an allocation is still owed, ticket 26/09/09). The same body already uses `close_page` for `state.retirement` one line above. |
| `retained::copy::tests::flow_selected_copy_rejects_root_retirement_overgrant_and_closes_factory_owner` | same line, same message | same |
| `retained::copy::tests::flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots` | same, plus a poisoned-mutex `unwrap` at `📑️copy/🦀️.rs:82` | same; the second panic is the first one's fallout across the law's worker thread |
| `retained::copy::tests::flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close` | same, plus `Any { .. }` at `📑️copy/🦀️.rs:116` | same |
| `retained::tests::flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once` | `left: 1 / right: 8193` at `🧵️retained/🧪️tests/🧵️retained/🦀️.rs:70` | `FlowRetirement::allocated_bytes` (`🧵️retained/🦀️.rs:188`) no longer answers the owner's physical capacity — the law's `exact_direct_backing` helper reads 1 where the owner holds 8 193 bytes |
| `retained::tests::flow_physical_retirement_multi_root_ingress_records_fault_without_admission_then_admits_exactly` | `left: 2 / right: 31218` at `…/🧵️retained/🦀️.rs:176` | same accounting: the drained total is an ITEM count, not the admitted byte total |

Nothing here is framework-side, so nothing was changed in them. They belong to the flow route owner
together with FP8 §3.1's `FlowOwnedSnapshotCursor`, which already routes through `close_page` for
exactly this reason and is the shape `CopyCursor::close_step` needs.

### 5.1 PRODUCT GAP found while re-expressing `a_spawned_task_…` — the async-task lane is test-only

With the law's instance bound (§3.4) the dispatch is now ADMITTED and the refusal moves one step later:
`task_count_for_instance(501)` is 0, i.e. nothing was spawned. The cause is structural, not a fixture
fault. `⚛️reactor::spawn_task` is `#[cfg(test)]` and has NO caller anywhere in
`🔌️plugin/🦀️.rs` — `Emit::task` from a migrated verb reaches the retained typed-operation pipeline,
which answers by NAME: `TypedOperationResultLane::Fault`, `"typed-operation task lane has no bounded
retained publication factory"` (`🔌️plugin/🦀️.rs:~28615`). So an `AsyncTask` can only ever be started by
a test calling `reactor::spawn_task` directly, which is what this law's four siblings
(`key_dedupe_…`, `instance_close_cancellation_…`, `checkpoint_then_restore_…`) do. The law was NOT
re-expressed to spawn directly: that would be a law testing a copy of the reducer's task body rather
than the product's own route, and the honest statement is that the route does not exist yet. Landing it
means giving the task lane a bounded retained publication factory — a product feature, not a fixture fix.

### 5.2 `reactor_output_fault_…`'s first clause contradicts a LANDED fix

`assert!(preparation.ui_patches.is_empty())` fails even with a surface id no other law in the binary
uses (`7:output-fault-window` / `17:output-fault-window`), which rules out the shared-thread-local
residue FP8 suspected: the queued external patch IS carried by the first turn that runs after it is
queued. That is the 2026-09-14 reconcile-spin fix, recorded in `⚛️reactor/📨️pending/🦀️.rs`'s own doc —
the arms used to `publish_into` the turn handback and answer `None`, costing one whole host round trip
per published surface, and `publish_into` is atomic so "the extraction belongs to the same call". The law
is restructured to that truth: the preparation turn is gone, the FAULTING poll is the turn that stages
the patch, and the removed clause is replaced by a source-anchored assertion on that doc so the day the
two-turn behaviour returns, this law names it.

### 5.3 DAG crate state after §4.4

`cargo test -p semio-framework-artifact-infinite-dag --lib` → **57 passed, 1 failed**
(`fp9-dag-member-law.txt`), and the new law passes on its own:
`retained::tests::dag_opens_as_an_owned_member_through_its_own_pack_codec ... ok`, 1 passed.
The one red is pre-existing and not this slice's:
`vcs::dag_vcs_tests::dag_demo_ownership_matches_neutral_graph_identity`
(`🌿️vcs/🧪️tests/🔬️dag-vcs/🦀️.rs:349`, `String("dag.fixture")` vs `String("dag.hostDocument")`) — a
document-id drift in the `🌿️vcs` module, untouched by the `🧵️retained` change.

## 6. Honest gaps

- **The gate is NOT green.** Six of the sixteen still have a source cause after this slice's landings
  (§4.2's five composed-document / session reds plus §4.3's fixture), and the two wall-clock ceilings
  stay, named. See §1's round table for the measured end state.
- **`🔌️plugin/🦀️.rs` gained product behaviour that no dependent crate was re-checked against.**
  `RuntimeInstanceRegistry::can_insert` changed from `&self` to `&mut self` and two call sites became
  `try_borrow_mut`; a successor landing on top should `cargo check` a plugin crate with
  `--features component-app-assembly`, which this slice did NOT exercise (same gap as FP4 §7 through FP8).
- **The `♾️infinite` DAG change is `semio-framework-os-infinite`'s, not this crate's.** It is checked and
  its law run separately (§4.4); no `s` plugin that embeds the DAG artifact was rebuilt.
- **The store law (§4.1) is new in a crate a peer (KN2) is actively editing.** It was appended in its own
  region at the end of `🏪️store/🧪️tests/🔬️unit/🦀️.rs` to minimise the conflict surface.
- **§5.4's shim change touches every composed law in the contract file.** It only STOPS the shim from
  overwriting a non-empty composed result, so a law that was green because the parent-only rebuild
  happened to satisfy it could in principle change; round 6 measured no such regression (816/6, the
  same six names as round 5).
- **Two `[DEBUG]` lines removed** (the aggregate-admission law's, the reactor patch law's). The
  `[KN2PROBE]` lines in `🏪️store/🧪️tests/🔬️unit/🦀️.rs` and the 25 + 29 `[DEBUG]` lines FP7/FP8 recorded
  in `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` and `🧪️tests/🧩️composition/🦀️.rs` belong
  to other owners and were not touched.
- **No law was `#[ignore]`d, deleted or loosened, and no ceiling constant moved anywhere.** Three laws
  were re-expressed (§3.2's sibling, §3.6, §3.7) and each re-expression states MORE than it did before:
  the lazy-backing law gained the exact reserved-byte derivation, the bijection gained two exact residue
  sets plus a source-anchored justification, and the identity oracle gained its domain-tag join.
- **Two oracle rows became joined for the first time** (`execution[]`'s
  `await-request-retirement-before-observation`, `checkpoint.tasksAfterCancel`); one row was re-recorded
  with its derivation (`expectedIdentityDigestHex`).
- Every number in this report is read from a capture in `🗑️generated/`. Nothing is claimed that was not
  executed.

## 7. Files changed

Product code (3 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `OwnedDocumentMemberIngressRegistry`
  closes in place, `close_active` removed (§3.1); `RuntimeInstanceRegistry::new` constructs empty,
  new `admit_backing`, `can_insert` takes `&mut self`, `insert`/`insert_admitted` take the backing,
  and the two preflight call sites (`plugin_open_actor_instance`, `plugin_create_app_with_id`) become
  `try_borrow_mut` (§3.2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — `cancel_instance_tasks_step`'s
  native arm detaches instead of polling, with its docstring (§3.3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs` —
  `DagOwnedSnapshotCursor` + `RetireOwned for DagSnapshot`, `SnapshotOpen` becomes
  `PackMemberSnapshotOpen` (§4.4)

Laws / fixtures / oracles (9 files):
- `🔌️plugin/🚪️lifetime/🧪️tests/🧪️aggregate-admission/🦀️.rs` — the lazy-backing second half, exact
  reserved-byte derivation, `[DEBUG]` line removed (§3.2)
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-instance-registry/🦀️.rs` — admission-before-backing
  re-expressed (§3.2)
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — the checkpoint law's close-ladder
  order + `poll_instance_tasks_once` + the joined `execution` oracle row (§3.3); the spawned-task law's
  instance bind (§3.4); the framework-owner type alias, 24 sites (§3.6); the bijection's two named
  residues (§3.7); the identity oracle's domain-tag clause (§3.8)
- `🔌️plugin/⚛️reactor/🧪️tests/⚛️reactor-driver/🦀️.rs` — new `poll_instance_tasks_once` driver helper (§3.3)
- `🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs` — per-case instance pair and surface, every
  clause names its case, `[DEBUG]` line removed (§3.9)
- `🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs` — explicit rollback + backbone detach before
  close (§3.5)
- `🔌️plugin/🧵️retained-command/🧫️fixtures/🧬️request-context.json` — `domainTag` added,
  `expectedIdentityDigestHex` re-recorded (§3.8)
- `🏪️store/🧪️tests/🔬️unit/🦀️.rs` — new region `🔖️RefusedReloadTests` with its law (§4.1)
- `♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs` — new law
  `dag_opens_as_an_owned_member_through_its_own_pack_codec` (§4.4)

Round 4–6 additions:
- `🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs` — the patch law restructured onto the landed
  same-call publication, anchored to `📨️pending/🦀️.rs`'s own doc (§5.2)
- `🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs` — the encoded transaction route
  re-expressed to the typed `TransactionPrepared { rejection }` it actually answers (§3.10)
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `ContractComposedApp::dispatch_typed`
  no longer discards a composed result; `composite_gesture_…` reports the documents it saw (§5.4)

Captures (`🗑️generated/`): `fp9-round0…round6-serial.txt`, `fp9-round0…round6.names`,
`fp9-round0-panics.txt`, `fp9-bin-path.txt`, `fp9-dag-member-law.txt`, `fp9-flow-retained.txt`.

### 5.4 PRODUCT GAP — the composed typed-operation settle path drops the child lane

`composite_gesture_…` asks for two `KernelMutation`s (parent handle + CHILD handle) and a two-entry
`UndoGroup.member_edits`. Measured with the law's own new diagnostic:
`got documents [ArtifactHandle(1)] and 0 member edit(s)`. Two facts behind it:
- `result_from_last_edit` (`🔌️plugin/🦀️.rs:25170`) builds its result from `self.store.edit_mutations()`
  alone and hard-codes `member_edits: Vec::new()` — it has no composed variant.
- the `ContractComposedApp` shim then OVERWROTE `admitted.mutations`/`inverse_group` with that
  parent-only rebuild on every dispatch. That is fixed here: the shim now fills only what the composed
  pipeline left empty, so a composed result can survive it. The law is still red, and now for the
  truthful reason — the migrated composed dispatch produces no child mutation and no member edit at
  all. Landing it means teaching the typed-operation settle path to carry the child lane into
  `InvocationResult`; rebuilding it in the fixture would make the law test a copy of itself.

### 5.5 `a_child_survives_…` — the fixture parent cannot declare a member

`open_child`/`load_child_pack` go through `validate_parent_child_restore`
(`🔌️plugin/🦀️.rs:24401`), which admits a restore only if the loaded PARENT SNAPSHOT declares that
`(slot, child)` through `ArtifactCompositionFields::child_projection`. `TestSnapshot`'s implementation
(`🧪️tests/🖥️test-app-mutations-document/🦀️.rs:30`) is `fn visit_child_refs(…) { Ok(()) }` — it declares
nothing, ever. So `TestApp` can hold children LIVE (`register_child` does not consult the snapshot) but
can never reload one, whatever is persisted. The sibling law that does pass the same persist/reload
cycle uses `ComposedParentApp`, whose snapshot does declare its members. Fixing this law means giving
`TestSnapshot` a real child-ref lane (a field the composite edit maintains), which changes its pack,
DSL and serde shape and every oracle pinned to it — deliberately not attempted at the end of a session.

### 5.6 `child_root_maintenance_…` — the law pins a stage a landed fix no longer honours

The law sets `app.maintenance_stage = 4` before every call, expecting that call to spend itself on
stage 4 (the child-content retirement stage — confirmed, the stage number has not moved). But
`PluginApp::maintenance_step` now ROTATES: it walks all `MAINTENANCE_STAGES` in one call, continuing
past any stage that released nothing, because a caller driving one stage per turn released about one
owner per 26 turns (`🔌️plugin/🦀️.rs:~30705`, measured on the assembled flow editor as 2 051 items in
100 000 turns). So a later idle stage can answer `Complete` and that is what the call returns — which
is exactly the `Complete` the law reports as "the lying owner reached a terminal step". The law needs
to drive `maintenance_stage_step(4, …)` directly, or assert on the fault the stage itself raises
(`interactive-job.maintenance-child-root-terminal-not-empty`, which stage 4 does raise for a lying
owner) rather than on the rotation's verdict.
