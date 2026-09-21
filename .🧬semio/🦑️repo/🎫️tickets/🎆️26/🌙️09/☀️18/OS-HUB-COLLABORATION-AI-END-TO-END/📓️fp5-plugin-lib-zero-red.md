# FP5 — `semio-framework-plugin --lib`: driving the 62 hard-core reds to zero

Slice FP5, 2026-09-21 (session 8). Continues FP4 (`📓️fp4-plugin-lib-gate.md`).
All runs: the lib unittests binary built once with private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp5` and the shared build-dir, driven directly
with `RUST_MIN_STACK=67108864` (FP4 §2.1: without it a phantom 12-law stack-overflow bucket appears).
Whole-suite numbers are the **serial** (`--test-threads=1`) reading, which is FP4's order-independent one.

## 1. Baseline on this tree

| run | passed | failed | capture |
|---|---:|---:|---|
| FP4 final (serial) | 756 | 62 | `fp4-round3-serial.txt` |
| FP5 round 0 (serial, this tree) | **755** | **63** | `fp5-round0-serial.txt` |

The 63 are FP4's exact 62 (`fp4-hard-core.names`) plus `guest_turn_execution_resets_for_every_turn`,
which FP4 §3.3 already recorded as one of the two load-dependent wall-clock laws. Nothing regressed.

## 2. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| 0 | baseline | 755 | 63 | `fp5-round0-serial.txt` |
| 1 | §3.1 member owner catalog on `new_test_child` | 755 | 63 | `fp5-round1-serial.txt` |
| 2 | §3.1 bare-child twin for the three refusal laws | 757 | 61 | `fp5-round2-serial.txt` |
| 3 | §3.2 lying owner planted on the INITIAL snapshot slot | **759** | **59** | `fp5-round3-serial.txt` |
| 4 | kind guard first placed in `dispatch_emit_inner` (never reached — §3.4) | 757 | 61 | `fp5-round4-serial.txt` |
| 5 | §3.3 intent route implemented + §3.4 kind guard on the publication lane | **761** | **57** | `fp5-round5-serial.txt` |
| 6 | §3.5 `targetWindow` / `mode.increment` given their own fixture verbs | 761 | 57 | `fp5-round6-serial.txt` |
| 7 | §3.5 the three manifest-addressed laws settle their dispatch | **765** | **53** | `fp5-round7-serial.txt` |
| 8 | §3.6 section-carrier sizing + command-log label | **767** | **51** | `fp5-round8-serial.txt` |
| 9 | §3.6 completed-record maintenance swept by stage, not by ordinal | 767 | 51 | `fp5-round9-serial.txt` |
| 9b | same tree, second draw | **768** | **50** | `fp5-round9b-serial.txt` |

**Net: 755 / 63 → 768 / 50. Thirteen laws landed; no law was deleted, `#[ignore]`d or loosened, and no
ceiling constant was changed anywhere.** Three of the 13 are product defects (§3.3, §3.4 and the guard's
two placements), ten are laws or fixtures re-expressed for the landed design.

## 3. Landed fixes

### 3.1 The composed-child fixture minted members with no owner catalog

`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3885` — `new_test_child` built an
`ArtifactStore` straight from an envelope and never installed `DocumentStoreOwners`. Every product path
that mints a member goes through `store::create_member_store` (`🏪️store/🦀️.rs:3381`) or
`open_member_store` (`:3405`), and **both** call `install_document_store_owners_exact`. The fixture was
the only minting path in the repo that did not, so every law that edits a child hit
`🏪️store/🦀️.rs:15837` — `edit history insertion requires its exact mutation retirement factory` — and
every law that closes a registered child hit `🔌️plugin/🦀️.rs:9244`.

Class: **stale fixture** (a pre-catalog member shape), not a product defect. `new_test_child` now adopts
the same catalog the product's own factory arms do. The one law that had been hand-patching the slot
(`retained_child_group_…`, line 1731) lost its redundant install.

Three laws' *subject* is a member that has NOT adopted its authority, so they keep a bare member through a
new `new_bare_test_child` (`:3897`) and their premise is untouched.

### 3.2 A lying terminal witness has to be planted on the initial-snapshot owner

`retire_snapshot_read_erased` (`🏪️store/🦀️.rs:19867`) and `take_returned_snapshot_read_retirement`
(`:16049`) both draw the child's returned-read retirement from `initial_snapshot_retirement_factory`,
never from `snapshot_retirement_factory`. `install_test_snapshot_retirement` planted its
`lie_about_terminal` factory on the latter, so the transfer was refused outright
(`PluginCloseStep::Blocked`) instead of reaching the lie. Class: **stale law** — the slot the design
draws from moved. The helper now installs a full catalog whose *initial* owner is a new
`TestLyingOwnedValueRetirementFactory` (`:3936`), which is the same law against the landed shape.

`child_snapshot_retirement_rejection_preserves_exact_erased_owner` additionally now adopts the catalog
(`install_test_member_owners`, `:3975`) after it has driven the refusal it asserts, so the app it
registered that member into can still reach terminal-empty.

## 5. Files changed

Product code (1 file):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
  - `:30643` `handle_intent_frame` — implemented (§3.3); was an unconditional
    `interactive-job.intent-raw-owner-required`
  - `:25377` `require_operation_emitting_kind` / `declared_dispatch_kind` — new (§3.4)
  - `:25200` `dispatch_emit_inner` and `:25395` `dispatch_emit_group` — kind guard on the two direct
    routes
  - `:28153` typed-operation publication lane — kind guard where a migrated emit's operations actually
    arrive (§3.4); this is the placement that fires

Fixtures (2 files):
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — member owner catalog on
  `new_test_child` (`:3885`), `new_bare_test_child` (`:3897`),
  `TestLyingOwnedValueRetirementFactory` (`:3936`), `install_test_member_owners` (`:3975`),
  `install_test_snapshot_retirement` installs a full catalog (`:3983`), `TestCommand::TargetWindow`
  and `TestCommand::ModeIncrement` (`:326`), `handle_intent_settled` (`:2136`),
  `settle_contract_app` (`:2162`), plus the nine re-expressed law bodies
- `🔌️plugin/🧪️tests/⏳️completion/🦀️.rs` — `targetWindow` / `mode.increment` rows in
  `TEST_APP_COMMAND_TOOL_IDS` (`:25`) and `TEST_APP_COMMAND_PUBLICATION_CONTRACTS` (`:49`)

Captures (`🗑️generated/`): `fp5-round0…round9b-serial.txt`, `fp5-round*.names`,
`fp5-round*-panics.txt`, `fp5-bin-path.txt`. Scratch: `🐍️fp5-decode.py`, `🐍️fp5-table.md`.

## 6. The 63 reds, law by law

Round-0 panic is the message the law died of on the baseline tree; "now" is the round-9b serial run.

| # | law | round-0 panic | now |
|---:|---|---|---|
| 1 | `owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally` | the 1,024 request slots stay behind one heap owner | red — the 1,024 request slots stay behind one heap owner |
| 2 | `new_app_constructs_a_registry_less_wrapper` | tool proof catalog must exactly join migrated generated declarations to live concrete factories: Fault { origi | red — tool proof catalog must exactly join migrated generated declarations to live concrete fact |
| 3 | `editor_fixture_still_mutates_normally` | increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, mes | red — increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"),  |
| 4 | `a_mutating_command_while_pending_is_rejected_but_reads_still_work` | a command emitting artifact mutations must be rejected while a transaction is pending | red — a command emitting artifact mutations must be rejected while a transaction is pending |
| 5 | `dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` | dispatch_emit must stash a TransactionProposalDraft when foreign steps are present | red — dispatch_emit must stash a TransactionProposalDraft when foreign steps are present |
| 6 | `generation_mismatch_is_rejected_with_the_frozen_code` | fixture close blocked: document store close awaits a retained reader or owner | red — fixture close blocked: document store close awaits a retained reader or owner |
| 7 | `fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers` | assertion failed: reactor.contains("output.typed_operation_results.iter()") | red — assertion failed: reactor.contains("output.typed_operation_results.iter()") |
| 8 | `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` | typed route pre-decoder gate | red — typed route pre-decoder gate |
| 9 | `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` | typed command route | red — typed command route |
| 10 | `action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization` | setTryValue action must reject one byte beyond its maximum | red — setTryValue action must reject one byte beyond its maximum |
| 11 | `same_schema_and_id_cannot_inherit_another_controllers_public_limit` | assertion failed: admit_action(&padded_action("s.draw.draw@1/*#editor", "canvasPointerDown", /                 | red — assertion failed: admit_action(&padded_action("s.draw.draw@1/*#editor", "canvasPointerDown |
| 12 | `native_aggregate_registry_does_not_allocate_backing_before_admission` | assertion `left == right` failed: the original runtime registry needs caller-granted backing before slot initi | red — assertion `left == right` failed: the original runtime registry needs caller-granted backi |
| 13 | `a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them` | first composite edit: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "register | red — parent checkpoint exists |
| 14 | `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Error, message: "child  | red — load child pack: Fault { origin: Plugin, code: FaultCode("plugin.internal"), severity: Err |
| 15 | `a_command_reaches_both_ephemeral_lanes_without_touching_history` | assertion `left == right` failed: presence lane never received the command's emission /   left: 0 | red — assertion `left == right` failed: presence lane never received the command's emission /    |
| 16 | `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("interactive-job.live-inst | red — dispatching SpawnCountTask must succeed: Fault { origin: Framework, code: FaultCode("inter |
| 17 | `activate_intent_dispatches_through_the_typed_command_path_same_turn` | an Activate intent on a Mutation-kind action must dispatch: Fault { origin: Framework, code: FaultCode("intera | **green** |
| 18 | `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` | assertion `left == right` failed /   left: {"checkoutCheckpoint", "clearSelection", "commitCheckpoint", "confi | red — assertion `left == right` failed /   left: {"checkoutCheckpoint", "clearSelection", "commi |
| 19 | `addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler` | addressed window action: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severi | **green** |
| 20 | `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault` | fixture app has no external owner: cancelled envelope output awaits the completed-record close pump | red — fixture app has no external owner: cancelled envelope output awaits the completed-record c |
| 21 | `an_operation_action_appends_one_command_log_entry_linked_to_its_edit` | assertion `left == right` failed /   left: "Increment" | **green** |
| 22 | `app_close_step_drains_at_most_one_segment_and_one_chunk_budget` | assertion `left == right` failed /   left: Pending { released_items: 1, released_bytes: 0 } | red — assertion `left == right` failed /   left: Pending { released_items: 1, released_bytes: 0  |
| 23 | `app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty` | assertion `left == right` failed /   left: Pending { released_items: 0, released_bytes: 0 } | **green** |
| 24 | `app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift` | assertion `left == right` failed /   left: 12184115126529414932 | red — assertion `left == right` failed /   left: 12184115126529414932 |
| 25 | `child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: retired child | red — registered fixture did not reach its exact terminal-empty witness, last pending close auth |
| 26 | `child_root_maintenance_reclaims_completed_owner_for_later_publication` | terminal-empty retirement is reclaimed while the app remains live | **green** |
| 27 | `child_root_maintenance_requires_terminal_empty_before_reclaim` | assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, /     4096).expect("transfer retirement au | red — the lying owner must never reach a terminal step: Complete |
| 28 | `child_snapshot_retirement_rejection_preserves_exact_erased_owner` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: child member  | **green** |
| 29 | `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | composite edit: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "registered fix | red — assertion `left == right` failed /   left: 1 |
| 30 | `created_children_survive_absorb_into_the_child_store_map` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: child member  | **green** |
| 31 | `group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` | composite edit: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "registered fix | red — assertion `left == right` failed /   left: TestSnapshot { count: 0, label: "composite" } |
| 32 | `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` | assertion `left == right` failed: cancel_instance_tasks already dropped the task's own RequestFuture — nothing | red — assertion `left == right` failed: cancel_instance_tasks already dropped the task's own Req |
| 33 | `key_dedupe_cancels_the_previously_live_task_under_the_same_key` | second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("plugin.task.supersessi | red — second must be admitted, cancelling the first: Fault { origin: Plugin, code: FaultCode("pl |
| 34 | `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` | encoded transaction route must remain explicitly unadmitted | red — encoded transaction route must remain explicitly unadmitted |
| 35 | `local_interaction_registered_query_channel_continuation_ack_and_close` | assertion failed: presence.is_empty() | red — assertion failed: presence.is_empty() |
| 36 | `manifest_command_dispatch_validates_structural_app_ownership` | assertion `left == right` failed /   left: 0 | **green** |
| 37 | `manifest_mode_command_requires_the_active_structural_owner` | active mode-owned command: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), seve | **green** |
| 38 | `maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode` | seed maximum child: ValidationFailed("edit history insertion requires its exact mutation retirement factory") | red — maximum child public dispatch exceeded 8 ms before its first continuation |
| 39 | `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` | artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every | red — artifact envelope terminal shell reached Drop before its app-owned bounded retirement auth |
| 40 | `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` | fixture app has no external owner: cancelled envelope output awaits the completed-record close pump | red — fixture app has no external owner: cancelled envelope output awaits the completed-record c |
| 41 | `peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root` | assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, /     4096).expect("captured app-typed pee | red — assertion failed: matches!(PluginApp::maintenance_step(&mut app, 1, /     4096).expect("ca |
| 42 | `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` | stale roster outcome | red — stale roster outcome |
| 43 | `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | assertion failed: preparation.ui_patches.is_empty() | red — assertion failed: preparation.ui_patches.is_empty() |
| 44 | `registry_less_construction_rejects_before_the_reducer` | artifact store reached Drop without its exact terminal-empty shallow-shell witness | red — artifact store reached Drop without its exact terminal-empty shallow-shell witness |
| 45 | `reserved_section_carrier_pages_a_payload_past_one_node_of_children` | assertion failed: payload.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX | **green** |
| 46 | `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` | assertion `left == right` failed /   left: 9 | red — assertion `left == right` failed /   left: 9 |
| 47 | `retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle` | retained composed replacement did not reach ValidatingClosure | red — retained composed replacement did not reach ValidatingClosure |
| 48 | `retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically` | assertion `left == right` failed /   left: Fault | red — assertion `left == right` failed /   left: Fault |
| 49 | `retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication` | retained composed replacement did not reach CandidateReady | red — retained composed replacement did not reach CandidateReady |
| 50 | `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("interactive-job.typed | red — called `Result::unwrap()` on an `Err` value: Fault { origin: Framework, code: FaultCode("i |
| 51 | `retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers` | registered fixture did not reach its exact terminal-empty witness, last pending close authority: app-typed pre | red — registered fixture did not reach its exact terminal-empty witness, last pending close auth |
| 52 | `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement` | assertion failed: matches!(requested.state, protocol::DocumentArchiveLoadState::Pending / /     protocol::Docu | red — assertion failed: matches!(requested.state, protocol::DocumentArchiveLoadState::Pending /  |
| 53 | `set_selection_mode_and_set_interaction_granularity_persist_immediately` | undeclared granularity must be rejected: InvocationResult { output: Object([("operationId", String("19781")),  | red — undeclared granularity must be rejected: InvocationResult { output: Object([("operationId" |
| 54 | `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` | assertion `left == right` failed /   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) | red — assertion `left == right` failed /   left: TypeId(0x86af186c9b161e3b42b75fc54d29eacb) |
| 55 | `the_child_content_view_never_goes_stale_across_undo_and_redo` | composite edit: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "registered fix | red — assertion `left == right` failed: the view must reflect the child's undone state /   left: |
| 56 | `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command` | assertion `left == right` failed /   left: "interactive-job.missing-factory" | red — assertion `left == right` failed /   left: "interactive-job.missing-factory" |
| 57 | `validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it` | the deleted id must be pruned from selection automatically | red — the deleted id must be pruned from selection automatically |
| 58 | `view_action_emitting_ops_is_rejected` | a View command emitting operations must be rejected: InvocationResult { output: Object([("operationId", String | **green** |
| 59 | `view_kind_intent_returning_operations_hard_faults` | unexpected error: UI intent 'badView' has no retained raw-page owner; application intent parsing is not admitt | **green** |
| 60 | `cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse` | assertion `left == right` failed: a detached slot must be reusable by a later spawn /   left: 1 | red — assertion `left == right` failed: a detached slot must be reusable by a later spawn /   le |
| 61 | `revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance` | assertion `left == right` failed /   left: UiRevision(1) | red — assertion `left == right` failed /   left: UiRevision(1) |
| 62 | `mounted_document_tree_publishes_nested_interactive_rows` | assertion `left == right` failed /   left: 52559872 | red — assertion `left == right` failed /   left: 52559872 |
| 63 | `guest_turn_execution_resets_for_every_turn` | assertion `left == right` failed: a settled turn keeps exactly the microseconds it executed /   left: Some(2) | **green** |


### 3.3 `handle_intent_frame` was an unconditional refusal — the whole retained-UI intent lane was dead

`🔌️plugin/🦀️.rs:30643`. The method's own doc says it "resolves `A::command_from_intent(intent)` then
dispatches through the SAME `dispatch_typed_command_inner`"; the body was

```rust
let proof = self.qualified_tool_proof(verb)?;
let _ = (proof, meta);
Err(Fault::new(… "interactive-job.intent-raw-owner-required" …))
```

`plugin_dispatch_intents` (`:38234`) is the ONLY consumer, and the reactor turn
(`⚛️reactor/🔄️turn/🦀️.rs:1176`) routes every surviving `UiIntent` of every retained surface through it.
Class: **product defect** — every retained-UI intent in the product faulted, the fault riding
`AppFrame::Error` per intent. It now rejects a non-v1 `ActionId`, folds `intent.args`/`intent.input`
through `merge_ui_values`, resolves `A::command_from_action` and dispatches through `dispatch_typed`,
i.e. the same admission → `dispatch_typed_command_inner` → `finish_recorded` path every other command
takes. It resolves through `command_from_action` rather than `command_from_intent` because
`app-typed-command-full-operation`'s source law forbids `A::command_from_intent` in this route by name
(pre-admission work); no app in the repo overrides `command_from_intent`, so nothing loses a hook.

### 3.4 `View`/`Shell` kind discipline was never enforced for a migrated dispatch

`ArtifactApp::command_id`'s doc (`:12056`) promises "`View`/`🐚️Shell`-kind must not emit
`artifact_mutations`; `VcsArtifactApp::dispatch_typed_command_inner` enforces this when the registry has
a matching declaration". **No such check existed anywhere** — the string "must not emit operations"
survived only in the ticket's own pre-patch backups. A migrated dispatch hands its reducer to a worker
and answers with an admission receipt, so the refusal cannot be made at dispatch time at all: the
operations only exist when the worker's emit reaches the publication ladder.

Guard added where they arrive — `🔌️plugin/🦀️.rs:28153`, in the typed-operation publication lane, right
after the factory publication-lane contract check, with two new helpers
`require_operation_emitting_kind` / `declared_dispatch_kind` (`:25377`). A `View` verb that reaches the
document now faults with `"View-kind command '<verb>' must not emit operations"`, which the settle
receipt surfaces as a publication fault. An unresolved verb is not a declared `View` and is not refused.

Two false starts are worth recording because they cost the round-4 regression: the same guard placed in
`dispatch_emit_inner` and then at the head of `dispatch_emit_group` was **never reached** — the migrated
publication ladder does not call either for the single-store artifact lane.

Both intent laws were then re-expressed against the landed two-phase shape with a new
`handle_intent_settled` fixture helper (`🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2136`) that
mirrors `ContractApp::dispatch_typed`'s settle-and-rebuild, and the activate law reads its command-log
row from `test_history()` instead of the admission receipt's now-always-`None` `history_patch`.

### 3.5 An addressed action id has to BE a tool id, and an addressed dispatch has to be settled

`dispatch_typed_command_inner` admits under the invoked verb and then requires
`A::command_id(&command)` to decode back to that exact verb
(`interactive-job.command-identity`). The contract fixture mapped two addressed verbs onto other
commands — `"targetWindow"` → `SetLabelViaCommand` (id `setLabelViaCommand`) and `"mode.increment"` →
`IncrementViaCommand` — so neither could ever be admitted: `qualified_tool_proof` refused first with
`interactive-job.missing-factory`, because neither id is in `TEST_APP_COMMAND_TOOL_IDS`.

Class: **stale fixture**. Both verbs now have their own `TestCommand` variant (`TargetWindow`,
`ModeIncrement`), their own `command_id`, their own reduce arm, and rows in
`TEST_APP_COMMAND_TOOL_IDS` / `TEST_APP_COMMAND_PUBLICATION_CONTRACTS`
(`🧪️tests/⏳️completion/🦀️.rs:25`, `:49`). The `PickItem`→`compositeEdit` precedent in the same fixture
is the one legitimate collapse: several commands may share ONE tool id, but the id the dispatch is
addressed by must be that id.

The three manifest-addressed laws then needed the same settle every migrated dispatch needs, through a
new `settle_contract_app` fixture helper — `handle_command` / `handle_action_invocation` answer with an
admission receipt exactly like `dispatch_typed` does.

### 3.6 Two frozen literals and one label

- `reserved_section_carrier_pages_a_payload_past_one_node_of_children` asserted its own precondition
  `payload.len() > UI_TEXT_MAX_BYTES * UI_BUILT_CHILDREN_MAX` against a hand-frozen 1,200-row fixture
  payload (~36 KB) while the product of the two ceilings is 512 × 128 = 65,536. The row count is now
  derived FROM the two ceilings, so the precondition follows them. **No ceiling was changed.**
- `an_operation_action_appends_one_command_log_entry_linked_to_its_edit` asserted the command-log label
  was the emit's own `description` ("increment") with the comment "an action id with no declaring
  definition …". The fixture it runs on is registry-backed now, `increment` IS declared, and the row
  carries the declared label ("Increment") — still `LocalizedLabel::data`, so the locale-invariance the
  clause exists for is asserted unchanged, against the declared label.
- `app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty` pinned
  `maintenance_stage = 11`. Which ordinal owns the completed-record lane moves whenever a stage is
  added; the law now sweeps `0..MAINTENANCE_STAGES` and closes its `contract_app_raw` fixture at the
  end (it had been leaving the store to fail its own Drop witness).

## 4. Honest gaps

- **The gate is NOT green: 50 laws are still red** (`fp5-round9b.names`). The brief's end state was not
  reached. FP4's §4 costing held: 40 of the core are one question each and there was no lever left of
  the kind that buys six laws for one line.
- **Three laws still move with machine load** and are load artefacts, not defects, exactly as FP4 §3.3
  ruled: `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks`
  (8 of 771 appends over 2 ms, worst 84 ms, under a full fleet),
  `guest_turn_execution_resets_for_every_turn` and
  `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement`.
  Each was green in at least one run of the final tree. The cure is a quiet machine, not a weaker law.
- **A found, unfixed product defect: the app close ladder retires a sealed segmented download in ONE
  slice without draining its chunks.** `app_close_step_drains_at_most_one_segment_and_one_chunk_budget`
  seeds a 2-chunk sealed `ArtifactDownloadOutput` at key 37; the first `close_step(1,
  ARTIFACT_OUTPUT_CHUNK_BYTES)` answers `Pending { released_items: 1, released_bytes: 0 }` and the entry
  is already gone from `app.segmented_downloads`, while the shared `ArtifactOutputChunks` still reports
  `chunks_remaining() == 2`. So the bytes are dropped outside the bounded per-chunk grant and the close
  under-reports what it released. Measured by instrumenting the law with a bounded walk; the law itself
  was then **restored to its authored text** rather than left re-expressed around the defect. This is
  the next item a successor should take: it is a real bounded-disposal violation, not a stale law.
- **`ArtifactApp::ephemeral` is unreachable from the migrated route.** `a_command_reaches_both_ephemeral
  _lanes_without_touching_history` fails because the app-owned tool job completes with
  `EphemeralEmit::default()` — and it cannot do better: `ArtifactOwnedToolJobContext`
  (`🔌️plugin/🦀️.rs:14634`) carries `transient` but no presence root, so a job cannot build the
  `PresenceView` the app's `ephemeral()` hook takes. Fixing it means threading presence into the tool
  job context, a framework change wider than one law; it was scoped and declined here.
- **`child_root_maintenance_requires_terminal_empty_before_reclaim` is red on a retired premise.** Its
  lying disposer can no longer be reached: `ReturnedSnapshotReadRetirement`
  (`🏪️store/🦀️.rs:1587`) only consults the domain's owned factory once `Arc::into_inner` makes the
  snapshot unique, and while the member store is live its `current` root is a second alias — so the
  wrapper answers `Complete` with its own honest terminal witness and the domain never gets to lie. The
  backstop at `🔌️plugin/🦀️.rs:9191` is real but unreachable through this member. Re-expressing it needs
  the route owner's decision about where the lie may still be planted; the law is left as authored.
- **The tool-registration bijection family was scoped and declined** (`activated_tool_factory_keys_are_
  an_exact_bijection_with_migrated_declarations`, `shared_framework_actions_have_exact_registered_
  factory_and_joined_bus_identity`). Measured: the runtime registers 21 keys where the fixture registry
  declares 14 — six framework-reserved interaction verbs (`clearSelection`, `selectAll`,
  `interactionHover`, `interactionSelect`, `setSelectionMode`, `setInteractionGranularity`) plus
  `configuration-binary` and `import-media` are registered although undeclared, and `setActiveUtility`
  is declared `Migrated` but never registered. Both halves are real questions about
  `FRAMEWORK_SHARED_ACTION_DESCRIPTOR_ROUTES` (`🔌️plugin/🦀️.rs:16359`) and belong to the route owner.
- **`full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` is a source-text law with
  a real finding behind it.** It requires `require_complete_tool_operation_pipeline(&admission)` to
  appear BEFORE the first generic `A::command_from_action` decoder; in `dispatch_action` (`:27060`) and
  `dispatch_command` (`:27103`) the gate is two lines after. Satisfying it means those routes admit the
  raw JSON args (`admit_command_json_with_proof`) before constructing the command instead of admitting
  the encoded command wire — a change to what is priced against `max_raw_wire_bytes` on every dispatch
  in the product. Scoped and declined as too wide to land safely inside this slice.
- **`--features component-app-assembly` was not exercised**, same as FP4 §7: the gate runs the default
  feature set.
- **No dependent crate was re-run.** The one product file changed is `🔌️plugin/🦀️.rs`; no public item
  changed signature. A peer owning `semio-framework-ui-contract` broke and fixed that crate under me at
  13:40–13:56 (a missing `Clone` on `UiNodeRecord`); that was not this slice's change.
- Every number here is read from a capture in `🗑️generated/`. Nothing is claimed that was not executed.
  No `[DEBUG]` instrumentation is left in the tree.
