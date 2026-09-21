# FP6 — `semio-framework-plugin --lib`: 768 / 50 → 0 failed

Slice FP6, 2026-09-21 (session 9). Continues FP5 (`📓️fp5-plugin-lib-zero-red.md`), FP4
(`📓️fp4-plugin-lib-gate.md`), FP3 (`📓️fp3-framework-plugin-suite-to-green.md`).

Method (unchanged from FP5): lib unittests binary built once with private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp6` + the shared build-dir, driven directly with
`RUST_MIN_STACK=67108864`. Whole-suite numbers are the **serial** (`--test-threads=1`) reading.

## 1. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| 0 | baseline (FP5's 50 + the load-flaky `guest_turn_execution_resets_for_every_turn`) | 767 | 51 | `fp6-round0-serial.txt` |
| 1 | §2.1 `ArtifactApp::ephemeral` invoked on the migrated route + §2.2 settled `ContractApp::dispatch_action` | 769 | 49 | `fp6-round1-serial.txt` |
| 2 | §2.2 the same shadow on `ContractComposedApp` + the raw-cell history verbs, §2.3 close-ladder staging | **774** | **44** | `fp6-round2-serial.txt` |
| 3 | §2.4 the emit backstops moved onto the migrated publication lane + §2.5 fault codes framed onto the Fault page (first framing broke the fault-bound oracle; re-cut) | 774 | 44 | `fp6-round3-serial.txt` |
| 4 | §2.5 re-cut so `as_bytes` stays message-only, §2.6 envelope job re-driven after its own pump | **777** | **41** | `fp6-round4-serial.txt` |
| 5 | §2.7 the two DFF wire-cap laws given the declared contracts they price, §2.8 executor slot-reuse walked over the rotation | **782** | **36** | `fp6-round5-serial.txt` |
| 6 | §2.9 interaction revalidation on the migrated lane + emptied-domain prune (first cut pruned hover too) | 782 | 36 | `fp6-round6-serial.txt` |
| 7 | §2.9 prune scoped to the document-change pass | **783** | **35** | `fp6-round7-serial.txt` |
| final | same tree, second draw | 781 | 37 | `fp6-final-serial.txt` |

The final draw differs from round 7 by exactly two wall-clock laws and no source change:
`maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode` (an 8 ms
first-continuation ceiling) and `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_
nakagin_sized_ticks`. **783 / 35 is the tree's reading on a quieter machine; 781 / 37 under the fleet.**

**Net: 767 / 51 → 783 / 35. Sixteen of FP5's 50 landed; no law was deleted, `#[ignore]`d or loosened,
and no ceiling constant was changed anywhere.** Six of the sixteen are product defects (§2.1, §2.4 —
four dead backstops, §2.5, §2.6, §2.9), ten are laws or fixtures re-expressed for the landed design.

## 2. Landed fixes

### 2.1 PRODUCT DEFECT — `ArtifactApp::ephemeral` was never invoked by the runtime

`🔌️plugin/🦀️.rs:12015` declares the hook and its own doc says it is "**Called on every dispatched
command, right before `handle`**" and that its two lanes "are applied unconditionally and cannot fail".
Measured: the string `A::ephemeral(` appears nowhere in the product. The only three call sites
(`:7847`, `:33443`, `:33738`) are inside the Editor/View adapters' OWN `ArtifactApp::ephemeral` impls —
they forward to `ArtifactEditor::ephemeral`, and nothing ever calls the outer one. `PresenceView` was
never constructed anywhere in the repo, and `PresencePeersView`'s `root` field had no constructor: the
hook was structurally unreachable, so **every app's presence and transient lane was dead on the
migrated route**, which is what `a_command_reaches_both_ephemeral_lanes_without_touching_history`
failed on (`presence lane never received the command's emission`, left `0`).

Fixed at `🔌️plugin/🦀️.rs:28512`, in `start_typed_command_operation`, right after the window
config/transient authorities are captured and before the reducer is handed to the worker — the last
point at which the dispatch still holds the document, config, presence and transient roots. The hook's
`presence`/`transient` lanes are applied with the same `PresenceStore::apply`/`TransientStore::apply`
the framework-reserved clipboard route uses (`:27011`), NOT through the job's declared publication
lanes: the hook is not the reducer, and its lanes have no op log, no undo group and no failure mode.
The transient root and both generations handed to the job context are re-read after the apply, so the
job never sees a stale root. A `window_transient` emission from the hook is refused explicitly (no
`ArtifactEditor`/`ArtifactViewer` adapter can produce one — both hard-code `Vec::new()` — so this is a
fail-closed guard, not a dropped lane).

**Correction to FP5 §4.** FP5 recorded the second owner-less finding as "`ArtifactOwnedToolJobContext`
carries no presence root, so a job cannot build the `PresenceView`". Measured here: the job never needed
to. The context is the wrong place — the hook is framework-invoked, before the job exists, and the
dispatch already holds every root it takes. No field was added to `ArtifactOwnedToolJobContext`.

### 2.2 The fixture's `dispatch_action` was never settled — 6 laws

`ContractApp` shadows `dispatch_typed` with a settle (FP5), but `dispatch_action` fell through `Deref`
to the raw wrapper. `dispatch_action` admits in **two** different two-phase shapes and a law that reads
the store right after a bare one observes the pre-dispatch revision in both:

- a framework-reserved history verb (`undo`/`redo`/`commitCheckpoint`/`checkoutCheckpoint`) answers with
  `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }` the caller has to run before the store ever
  sees `ArtifactCommand::Undo` — the product's own `artifact_app_laws::settle_history_verb`
  (`🔌️plugin/🦀️.rs:7452`) documents exactly this and no fixture law used it;
- an app verb hands its reducer to a worker, like every migrated dispatch.

Class: **stale law shape** (pre-migration synchronous dispatch). Added `ContractApp::dispatch_action`
(`🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2144`) and the identical
`ContractComposedApp::dispatch_action` (`:2300`), each driving
`settle_framework_reserved_admission` → `settle_registered_typed_operation` → the same
`test_result_from_last_edit` rebuild `dispatch_typed` uses. The two raw-cell history verbs inside
`retained_child_group_publishes_…` (`:1842`, `:1849`) got the same three steps inlined, because that law
drives a raw `VcsArtifactApp` inside a `PluginRuntime` cell under its own instance id.

Landed: `a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them`,
`the_child_content_view_never_goes_stale_across_undo_and_redo`,
`maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode`,
`retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`,
`a_command_reaches_both_ephemeral_lanes_without_touching_history` (with §2.1).

### 2.3 NOT a product defect — the close ladder drains the segmented download correctly

FP5 §4 left this as "the app close ladder retires a sealed segmented download in ONE slice without
draining its chunks … the entry is already gone from `app.segmented_downloads` while the shared
`ArtifactOutputChunks` still reports `chunks_remaining() == 2`". **Measured here with a bounded probe
(`fp6-close-probe`): false.** Over 400 consecutive `close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES)` calls the
entry stays in `segmented_downloads`, nothing is released, and at step 40 the ladder reaches the segment
stage and answers `Pending { released_items: 1, released_bytes: 4096 }` with `chunks_remaining() == 1` —
exactly what the law demands. The ladder was never wrong; the law's staging was.

Two stale things in the staging, both ceilings that moved:
- `app.close_cancellation_cursor = TOOL_CANCELLATION_SLOTS` while the stage's own guard is
  `< TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS` (`🔌️plugin/🦀️.rs:29721`) — 64 unskipped steps;
- ~40 further stages (mounted jobs, tool runs, snapshot-read returns, presence local-read maintenance)
  the law's three hand-pinned cursors never covered.

Re-expressed at `🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2718` by walking the ladder to the
segment stage with a **sub-chunk grant**: every pre-segment stage is bookkeeping that releases no bytes,
and the segment stage alone refuses a grant under one chunk (`🔌️plugin/🦀️.rs:29819`), so
`close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES - 1)` walks the ladder and then stands still on the segment
stage. That parking point IS the bounded-grant refusal, so the re-expression **adds** an assertion
rather than weakening one; the three original segment-slice assertions are byte-identical.

### 2.4 PRODUCT DEFECT — every emit-time backstop was dead on the migrated route

`dispatch_emit_inner` (`🔌️plugin/🦀️.rs:25135`–`:25177`) holds four refusals the contract freezes:
the `AppRole::Viewer` read-only backstop (§2.3 clause 2), the tool-run freeze (`toolRun.busy`), the
transaction freeze (`transaction.instance-busy`, §5.10) and the composite-mutation **proposal** stash
(§5.1). FP5 §3.4 already measured that **the migrated publication ladder never calls
`dispatch_emit_inner` for the single-store artifact lane** — it only found that out for the kind guard.
The same measurement condemns all four: on every migrated dispatch a `Viewer` app could mutate, a
frozen tool run could be written through, a pending transaction did not block anything, and a mutation
carrying foreign steps was APPLIED instead of proposed.

All four now sit at `🔌️plugin/🦀️.rs:28153`, in the typed-operation publication lane, immediately after
the lane-contract check and the kind guard — the only point at which a migrated dispatch's mutations
exist. The proposal arm folds the mutations through the same `Mutation::diff`/`MutationDiff::apply`
pair, stashes the `TransactionProposalDraft`, clears the artifact lane and lets the ladder finish, so
the document never advances.

Landed: `a_mutating_command_while_pending_is_rejected_but_reads_still_work`,
`dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` (with the settle in
`artifact_app_laws::assert_proposes_transaction`, `🔌️plugin/🦀️.rs:7683`, which dispatched without
settling — the same stale shape as §2.2).

### 2.5 PRODUCT DEFECT — a publication fault lost its frozen `FaultCode`

`ArtifactBoundedToolFault::from_fault` (`🔌️plugin/🦀️.rs:14299`) copied only `fault.message`, and
`into_fault` rebuilt every fault as `interactive-job.app-owned-output`; the Fault-lane result page
carried the message alone. So **every refusal a migrated dispatch makes reached its caller code-less** —
`transaction.instance-busy`, `toolRun.busy`, `app.read-only`, `interactive-job.command-identity` all
arrived as one opaque string, and no shell, host or law could branch on why.

The bounded fault now retains a bounded `code` beside its detail, `into_fault` restores it, and the
Fault-lane page is framed `<code>\u{1f}<message>` (`TYPED_OPERATION_FAULT_SEPARATOR`, the one byte a
`FaultCode` cannot contain), decoded by `decode_typed_operation_fault_page` (`:14276`).
`ArtifactBoundedToolFault::as_bytes` stays the **message alone** — it is what the language-neutral
fault-bound oracle prices, and the first cut, which framed `as_bytes` itself, broke
`every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle` (round 3). No
fixture JSON was touched. `artifact_app_laws::settle_registered_typed_operation` (`:7286`) now rebuilds
the fault with its exact code.

### 2.6 PRODUCT DEFECT — an envelope decode job answered `Blocked` from before its own pump

`drive_envelope_decode_jobs` (`🔌️plugin/🦀️.rs:23877`) captured `active.drive(…)`'s answer, THEN ran the
field-decoder and completed-record drain pumps, then returned the pre-pump answer. A
`ClosingCancelled` envelope's only wait is `request_completed_close` → `ticket_reclaimed`, i.e. exactly
the pump that had just run, so the step reported `Blocked { "cancelled envelope output awaits the
completed-record close pump" }` on a ladder that had already unblocked itself. Every bounded close loop
in the product and the fixtures treats `Blocked` as terminal, so the whole close aborted.

Fixed at `:23898`: a `Blocked` answer is re-asked after the pumps, once, within the same bounded step.
Landed: `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault`,
`one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll`.

### 2.7 The DFF wire caps moved from a frozen table to the app's declared contract

`action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization` and
`same_schema_and_id_cannot_inherit_another_controllers_public_limit` called
`validate_public_action_envelope(body, owner, controller, &[])`. Since the public predecode was rebuilt
on `ArtifactToolPublicContract` (`🔌️plugin/🦀️.rs:36822` `dff_interactive_wire_limit`), the bound IS the
live app's declared `max_raw_wire_bytes` for that exact `(owner, controller, tool)` row — and an EMPTY
contract slice declares no bound at all, so every oversized body was admitted and both laws asserted
against a lookup that could never fire. Class: **stale law**. The fixture now restates the five
Draw/Flow/Forms rows it prices (`🔬️plugin-runtime-dff-public-action-admission/🦀️.rs:26`); the caps
(16_384 / 8_192) and both laws' bodies are byte-identical.

### 2.8 The executor's free list is a pre-filled FIFO, not a LIFO

`cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse` asserted the detached index came back on
the **very next** spawn. `ColdFutureExecutorInner::new` pre-fills `free` with every index `0..1_024`
(`⚛️reactor/🧵️executor/🦀️.rs:386`) and `detach` pushes the freed index to the BACK (`:609`), so the
handback is one full rotation later. Class: **stale law** — the arena became fixed and pre-filled.
The law now walks the rotation, bounded by `LOCAL_EXECUTOR_TASK_SLOTS`, and asserts the same two
clauses (the index returns; its generation advanced) on the exact handback. Nothing is weakened: a
leaked index still fails, because the walk never finds it.

### 2.9 PRODUCT DEFECT — interaction state was never revalidated after a migrated dispatch, and an
emptied domain could not be pruned

Two halves, both behind the clause `dispatch_emit_inner` carries at `🔌️plugin/🦀️.rs:25361`: "after
EVERY artifact (document) dispatch, re-derive fresh topology and prune any selection/hover id no longer
present".

- The call sits in `dispatch_emit_inner`, dead on the migrated route (§2.4). It now runs in
  `advance_typed_operation_publication_unit` (`:27577`), once per operation, gated by a new
  `MountedTypedCommandFullOperation::interaction_revalidated` flag and by `published_artifact` — the
  point the Artifact lane has actually committed. The mounted operation is out of the registry there,
  so the async pass may borrow the app.
- Even then the prune could not fire: `build_full_interaction_topology` (`:25889`) skipped any domain
  whose topology came back empty, which is exactly the "every id was deleted" case the prune exists
  for — `validate_state` was handed no existence set and the stale ids survived. A domain that still
  carries a **selection** is now included on the `DocumentChange` pass even when its topology is empty.
  Scoped to that origin deliberately: on a `Pick` pass an app that publishes no topology at all has made
  no statement about existence, and pruning there wrongly cleared a live hover
  (`empty_target_interaction_select_clears_selection_while_hover_remains`, round 6).

`set_selection_mode_and_set_interaction_granularity_persist_immediately` was the third law here and is a
**stale law**: `handle_action` only ADMITS `setInteractionGranularity` (a `framework_reserved_job!`
route), so the undeclared-granularity refusal rides the reserved job's commit, where the law now reads
it — the same place `reserved_action` already drove the accepted verbs.

## 3. The 50 reds, law by law

Round-0 panic is the message the law died of on FP5's final tree; "now" is the round-7 serial run.
`guest_turn_execution_resets_for_every_turn` was red on round 0 too (FP4 §3.3's load-dependent law) and
is not counted among the 50.

| # | law | round-0 panic | now |
|---:|---|---|---|
| 1 | `owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally` | the 1,024 request slots stay behind one heap owner | red — the 1,024 request slots stay behind one heap owner |
| 2 | `new_app_constructs_a_registry_less_wrapper` | tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origi | red — tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origi |
| 3 | `editor_fixture_still_mutates_normally` | increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, mes | red — increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, mes |
| 4 | `a_mutating_command_while_pending_is_rejected_but_reads_still_work` | a command emitting artifact mutations must be rejected while a transaction is pending | **green** |
| 5 | `dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` | dispatch_emit must stash a TransactionProposalDraft when foreign steps are present | **green** |
| 6 | `generation_mismatch_is_rejected_with_the_frozen_code` | fixture close blocked: document store close awaits a retained reader or owner | red — fixture close blocked: document store close awaits a retained reader or owner |
| 7 | `fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers` | assertion failed: reactor.contains("output.typed_operation_results.iter()") | red — assertion failed: reactor.contains("output.typed_operation_results.iter()") |
| 8 | `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` | typed route pre-decoder gate | red — typed route pre-decoder gate |
| 9 | `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` | typed command route | red — typed command route |
| 10 | `action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization` | setTryValue action must reject one byte beyond its maximum | **green** |
| 11 | `same_schema_and_id_cannot_inherit_another_controllers_public_limit` | assertion failed: admit_action(&padded_action("s.draw.draw@1/*#editor", "canvasPointerDown", \|                 | **green** |
| 12 | `native_aggregate_registry_does_not_allocate_backing_before_admission` | assertion `left == right` failed: the original runtime registry needs caller-granted backing before slot initi | red — assertion `left == right` failed: the original runtime registry needs caller-granted backing before slot initi |
| 13 | `a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them` | parent checkpoint exists | **green** |
| 14 | `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "child  | red — load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "child  |
| 15 | `a_command_reaches_both_ephemeral_lanes_without_touching_history` | assertion `left == right` failed: presence lane never received the command's emission \|   left: 0 | **green** |
| 16 | `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.live-inst | red — dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.live-inst |
| 17 | `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` | assertion `left == right` failed \|   left: {"checkoutCheckpoint", "clearSelection", "commitCheckpoint", "confi | red — assertion `left == right` failed \|   left: {"checkoutCheckpoint", "clearSelection", "commitCheckpoint", "confi |
| 18 | `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault` | fixture app has no external owner: cancelled envelope output awaits the completed-record close pump | **green** |
| 19 | `app_close_step_drains_at_most_one_segment_and_one_chunk_budget` | assertion `left == right` failed \|   left: Pending { released_items: 1, released_bytes: 0 } | **green** |
| 20 | `app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift` | assertion `left == right` failed \|   left: 12184115126529414932 | red — assertion `left == right` failed \|   left: 12184115126529414932 |
| 21 | `child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: retired child | **green** |
| 22 | `child_root_maintenance_requires_terminal_empty_before_reclaim` | the lying owner must never reach a terminal step: Complete | red — the lying owner must never reach a terminal step: Complete |
| 23 | `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | assertion `left == right` failed \|   left: 1 | red — assertion `left == right` failed \|   left: 1 |
| 24 | `group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` | assertion `left == right` failed \|   left: TestSnapshot { count: 0, label: "composite" } | red — registered fixture did not reach its exact terminal-empty witness, last pending close authority: retired child |
| 25 | `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` | assertion `left == right` failed: cancel_instance_tasks already dropped the task's own RequestFuture — nothing | red — assertion `left == right` failed: cancel_instance_tasks already dropped the task's own RequestFuture — nothing |
| 26 | `key_dedupe_cancels_the_previously_live_task_under_the_same_key` | second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("plugin.task.supersessi | red — second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("plugin.task.supersessi |
| 27 | `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` | encoded transaction route must remain explicitly unadmitted | red — encoded transaction route must remain explicitly unadmitted |
| 28 | `local_interaction_registered_query_channel_continuation_ack_and_close` | assertion failed: presence.is_empty() | red — assertion failed: presence.is_empty() |
| 29 | `maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode` | assertion failed: result.requested_effects.iter().any(\|effect\| \|         matches!(effect, Effect::DispatchActi | **green** |
| 30 | `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` | artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every | red — artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every |
| 31 | `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` | fixture app has no external owner: cancelled envelope output awaits the completed-record close pump | **green** |
| 32 | `peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root` | assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, \|     4096).expect("captured app-typed pee | red — assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, \|     4096).expect("captured app-typed pee |
| 33 | `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` | stale roster outcome | red — stale roster outcome |
| 34 | `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | assertion failed: preparation.ui_patches.is_empty() | red — assertion failed: preparation.ui_patches.is_empty() |
| 35 | `registry_less_construction_rejects_before_the_reducer` | artifact store reached Drop without its exact terminal-empty shallow-shell witness | red — artifact store reached Drop without its exact terminal-empty shallow-shell witness |
| 36 | `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` | assertion `left == right` failed \|   left: 9 | **green** |
| 37 | `retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle` | retained composed replacement did not reach ValidatingClosure | red — retained composed replacement did not reach ValidatingClosure |
| 38 | `retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically` | assertion `left == right` failed \|   left: Fault | red — assertion `left == right` failed \|   left: Fault |
| 39 | `retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication` | retained composed replacement did not reach CandidateReady | red — retained composed replacement did not reach CandidateReady |
| 40 | `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("interactive-job.typed | red — called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("interactive-job.typed |
| 41 | `retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: app-typed pre | red — registered fixture did not reach its exact terminal-empty witness, last pending close authority: app-typed pre |
| 42 | `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement` | assertion failed: matches!(requested.state, protocol::DocumentArchiveLoadState::Pending \| \|     protocol::Docu | red — assertion failed: matches!(requested.state, protocol::DocumentArchiveLoadState::Pending \| \|     protocol::Docu |
| 43 | `set_selection_mode_and_set_interaction_granularity_persist_immediately` | undeclared granularity must be rejected: InvocationResult { output: Object([("operationId", String("20037")),  | **green** |
| 44 | `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` | assertion `left == right` failed \|   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) | red — assertion `left == right` failed \|   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) |
| 45 | `the_child_content_view_never_goes_stale_across_undo_and_redo` | assertion `left == right` failed: the view must reflect the child's undone state \|   left: 7 | **green** |
| 46 | `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command` | assertion `left == right` failed \|   left: "interactive-job.missing-factory" | red — assertion `left == right` failed \|   left: "interactive-job.missing-factory" |
| 47 | `validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it` | the deleted id must be pruned from selection automatically | **green** |
| 48 | `cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse` | assertion `left == right` failed: a detached slot must be reusable by a later spawn \|   left: 1 | **green** |
| 49 | `revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance` | assertion `left == right` failed \|   left: UiRevision(1) | red — assertion `left == right` failed \|   left: UiRevision(1) |
| 50 | `mounted_document_tree_publishes_nested_interactive_rows` | assertion `left == right` failed \|   left: 52559872 | red — assertion `left == right` failed \|   left: 52559872 |

## 4. Honest gaps

- **The gate is NOT green: 34 of the 50 are still red**, plus the load-dependent
  `guest_turn_execution_resets_for_every_turn` (`fp6-round7.names`). The brief's end state was not
  reached.
- **FP5's first owner-less finding was wrong and is corrected here (§2.3).** The close ladder drains a
  sealed segmented download exactly as its law demands; the law's staging was stale. FP5's
  instrumentation reported an entry "already gone from `app.segmented_downloads`" that a 400-step probe
  shows never leaves it.
- **FP5's second owner-less finding was fixed at a different root than FP5 proposed (§2.1).** Nothing
  was threaded into `ArtifactOwnedToolJobContext`; the hook is framework-invoked before the job exists.
- **The three load-dependent laws are unchanged** (FP3 §6.1/§6.3, FP4 §3.3): `tool_run_overlay_append_
  per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks`, `guest_turn_execution_resets_for_
  every_turn` and `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_
  before_acknowledgement`. Each was green in at least one run of this tree (rounds 4/5/7) and red in
  another with no source change between them. The cure is a quiet machine, not a weaker law. They are
  named here so a successor does not spend a slice on them.
- **`group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` now fails only on
  its close**, measured: the live child-content root's strong count is 1 and no typed operation is
  pending at the end of the law, so the blocked retirement's alias is neither the live root nor an
  operation view the law holds. `ChildContentView::take_one` (`🔌️plugin/🦀️.rs:9053`) forgives a shared
  PAGE only when it is `retained_by_current` — a page shared between two PENDING retirements has no
  such escape and blocks both, which is the shape this law (two registered children, several
  publications, plus the undo's own republication per undone child) is the only one in the suite to
  reach. Not fixed: widening `retained_by_current` is a bounded-disposal decision for the child-content
  route owner, and the suite has no law pinning the current behaviour either way.
- **The retained composed-replacement family (3), the tool-registration bijection family (2), the
  `typed_command_full_operation` source-text family (3) and
  `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` were not attempted.** FP5
  §4 and FP3 §5 already scoped the last two to their route owners; nothing measured here changes that
  reading. The composed-replacement three share one state machine
  (`ActiveArtifactStoreReplacementState`) and are the largest single-root group left.
- **`--features component-app-assembly` was not exercised**, same as FP4 §7 and FP5: the gate runs the
  default feature set.
- **No dependent crate was re-run.** `🔌️plugin/🦀️.rs` gained one `pub(crate)` item
  (`decode_typed_operation_fault_page`) and one private struct field; no public item changed signature.
  A successor landing on top of this should still run `cargo check` on a plugin crate.
- **Two behaviour changes are wider than the laws that forced them** and a reviewer should read them as
  product changes, not test fixes: §2.4 (four refusals that were dead now fire on every migrated
  dispatch) and §2.5 (fault codes now cross the publication boundary, so a caller that matched on
  `interactive-job.app-owned-output` will now see the real code). Both restore documented contract
  clauses; neither is guarded by a feature flag.
- Every number here is read from a capture in `🗑️generated/`. Nothing is claimed that was not executed.
  No `[DEBUG]` instrumentation was left by this slice (the `[DEBUG]` lines in
  `🧪️tests/🧩️composition/🦀️.rs` predate it and belong to another owner).

## 5. Files changed

Product code (2 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
  - `:14276` `TYPED_OPERATION_FAULT_SEPARATOR` / `TYPED_OPERATION_FAULT_CODE_BYTES` /
    `TYPED_OPERATION_FAULT_PAGE_BYTES` / `decode_typed_operation_fault_page` — new (§2.5)
  - `:14299` `ArtifactBoundedToolFault` — retains a bounded `code`; `into_fault` restores it;
    `framed_page_bytes` frames the Fault-lane page (§2.5); the three page constructions use it
  - `:18349` `MountedTypedCommandFullOperation::interaction_revalidated` — new (§2.9)
  - `:23898` `drive_envelope_decode_jobs` — a `Blocked` answer is re-asked after its own pumps (§2.6)
  - `:25889` `build_full_interaction_topology` — takes the revalidation origin; an emptied domain that
    still carries a selection is included on the document-change pass (§2.9)
  - `:27286` `artifact_app_laws::settle_registered_typed_operation` — rebuilds a publication fault with
    its exact code (§2.5)
  - `:27577` `advance_typed_operation_publication_unit` — interaction revalidation after the Artifact
    lane publishes (§2.9)
  - `:28153` typed-operation publication lane — the viewer backstop, the tool-run freeze, the
    transaction freeze and the composite-mutation proposal stash (§2.4)
  - `:28512` `start_typed_command_operation` — `A::ephemeral` invoked and its two lanes applied (§2.1)
  - `:7683` `artifact_app_laws::assert_proposes_transaction` — settles its dispatch (§2.4)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🧪️tests/🔬️unit/🦀️.rs` — test only (§2.8)

Fixtures (3 files):
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `ContractApp::dispatch_action`
  (`:2144`), `ContractComposedApp::dispatch_action` (`:2300`), `ContractComposedApp::drop` drains
  maintenance before closing, the raw-cell history verbs (`:1842`, `:1849`), the close-ladder staging
  (`:2718`), the command-capture drop in `child_content_publication_path_…`, the reserved-admission
  settle in `set_selection_mode_…`
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-dff-public-action-admission/🦀️.rs` — `dff_public_contracts` (§2.7)
- `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` — the new mounted field in four fixture
  initializers

Captures (`🗑️generated/`): `fp6-round0…round7-serial.txt`, `fp6-round*.names`, `fp6-round*-panics.txt`,
`fp6-bin-path.txt`, `fp6-build-err.txt`. Scratch: `🐍️fp6-decode.py`.
