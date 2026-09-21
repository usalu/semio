# FP8 — `semio-framework-plugin --lib`: 794 / 28 → 806 / 16

Slice FP8, 2026-09-21 (sessions 11–12). Continues FP7 (`📓️fp7-plugin-lib-zero-red.md`), FP6, FP5, FP4, FP3.

Method (unchanged from FP5/FP6/FP7): the lib unittests binary built once with private
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp8` + the shared build-dir, copied out of the shared
build-dir (a peer's rebuild must not overwrite a running binary) and driven directly with
`RUST_MIN_STACK=67108864`. Whole-suite numbers are the **serial** (`--test-threads=1`) reading.

## 1. Round table

| round | what landed | passed | failed | capture |
|---|---|---:|---:|---|
| 0 | baseline — FP7's 28 reproduced **exactly**, name for name | 794 | 28 | `fp8-round0-serial.txt` |
| 1 | §2.1 registry-less catalog abort, §2.2 orphaned peer-roster generation, §2.3 generation-0 presence pack, §2.9 three fixture closes/drops | 799 | 23 | `fp8-round1-serial.txt` |
| 2 | §2.4 the merge fixture settles its seeded edit, §2.5 the resident-aggregate oracle re-recorded with its derivation | 800 | 22 | `fp8-round2-serial.txt` |
| 3 | §2.6 the reactor test driver acknowledges and drains its publication, §2.7 the continuation-pacing ceiling made load-free, §2.8 three source anchors re-cut | 805 | 17 | `fp8-round3-serial.txt` |
| 4 | §2.10 the UI-safety backstop moved ahead of the proof lookup | 804 | 18 | `fp8-round4-serial.txt` |
| 5 | §2.11 keyed-task supersession disposes its previous owner, §2.10's two laws re-expressed | 805 | 17 | `fp8-round5-serial.txt` |
| 6 | (an attempted instance-cancellation trade — reverted, §4.2) | 805 | 17 | `fp8-round6-serial.txt` |
| 7 | §2.10's unproved fixture given its exact clause | 807 | 15 | `fp8-round7-serial.txt` |
| final | same tree, the trade of round 6 reverted, one load draw | **806** | **16** | `fp8-final-serial.txt` |

**Net: 794 / 28 → 806 / 16. Thirteen of FP7's 28 landed; no law was deleted, `#[ignore]`d or loosened,
and no ceiling constant was changed anywhere.** Six of the thirteen are product defects (§2.1, §2.2,
§2.3, §2.10, §2.11, plus the store-level §4.1 finding), the rest are laws, oracles or fixtures
re-expressed for the landed design.

The thirteen that went green (`fp8-final.names` vs `fp8-round0.names`):
`new_app_constructs_a_registry_less_wrapper`, `registry_less_construction_rejects_before_the_reducer`,
`peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority`,
`retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers`,
`local_interaction_registered_query_channel_continuation_ack_and_close`,
`merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads`,
`mounted_document_tree_publishes_nested_interactive_rows`,
`revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance`,
`fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers`,
`full_operation_source_rejects_generic_reducers_and_old_monolithic_shells`,
`host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate`,
`ui_dispatch_backstop_rejects_every_non_migrated_action_and_command`,
`key_dedupe_cancels_the_previously_live_task_under_the_same_key`.

The final draw's sixteenth is `retained_operation_continues_after_command_admission_until_publication_and_retirement`
(`instance busy or poisoned: 7`) — green 5 runs out of 5 in isolation, red only in the loaded
whole-suite draw. That is exactly FP7's red #36 and the load signature FP3 §6.3 recorded; it is NOT one
of the fifteen with a source cause.

## 2. Landed fixes

### 2.1 PRODUCT DEFECT — a registry-less app could not be CONSTRUCTED at all

`AppActionRegistry::tool_job_registration` derives `expected` from `migrated_tool_ids()`. The documented
registry-less path (`AppActionRegistry::default()`, whose `controller_id` doc says "Empty for the
registry-less test path") declares nothing, so `migrated` is `{}` and EVERY
`bounded_first_step_tool_proofs!` row of the app is rejected with `interactive-job.catalog-authority`.
`VcsArtifactApp::with_registry_on_bus` takes that result with `.expect(…)`, so construction PANICS:
`semio_framework_plugin::testkit::new_app` is unusable for any app that declares a tool proof — the exact
class recorded in memory `project-registryless-testkit-new-app-unusable`, but the fault was the
constructor aborting rather than the app failing closed.

Root fix at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: a new
`AppActionRegistry::is_registry_less` (no controller, no action, no window action, no app command, no
mode command) and an early return in `validate_tool_job_rows` that answers an EMPTY proof set. Nothing is
loosened: a registry that declares anything still goes through the full row-by-row join, and a
registry-less app then fails closed at its own first dispatch with `interactive-job.unknown-key`
("typed command … has no exact manifest declaration"), which is what its sibling law
`registry_less_construction_rejects_before_the_reducer` already asserted for the same situation.

Landed: `new_app_constructs_a_registry_less_wrapper` (re-expressed to the truthful code plus a clause
that the reducer is never reached), `registry_less_construction_rejects_before_the_reducer`.

### 2.2 PRODUCT DEFECT — a peer-roster publication behind the ordered cursor is orphaned forever

FP7 §5 measured it and left it: with `peer_roster_processed_generation` forced ahead, 4 096 forced
stage-7 maintenance calls never empty `peer_roster_publications`. The cause is arithmetic: stage 7 only
ever looks at `peer_roster_processed_generation + 1`, so a publication at a generation the cursor has
ALREADY passed is invisible to the rotation. It answers `Pending { 0, 0 }` for it forever, the
publication never reaches its outcome slot, `take_presence_outcome` never answers, and the app's close
conjunction — which demands `peer_roster_publications.is_empty()` — is unreachable: **the app can never
close.**

Root fix at `🔌️plugin/🦀️.rs`: a new `orphaned_peer_roster_generation` (the LOWEST mounted publication
whose generation is not ahead of the processed cursor, read from the fixed registry's own
`each_id`), consulted only when the exact next generation is absent, and
`peer_roster_processed_generation` advanced with `.max(generation)` so working an out-of-order
publication can never move the cursor backwards. The orphan then walks the SAME validate/outcome ladder,
where `validate_peer_roster_publication` refuses its generation with
`interactive-job.peer-roster-publication-authority` — the exact fault the law names — and the publication
reaches its outcome slot and is removed. The in-order path is byte-for-byte unchanged.

Landed: `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` (plus §2.9:
its three fixture apps never closed).

### 2.3 PRODUCT DEFECT — an untouched presence lane shipped a pack on every ephemeral frame

`VcsArtifactApp::ephemeral_snapshot` carries `presence: self.presence_store.local().encode_pack()`
unconditionally. The neighbouring interaction lane has an explicit "nothing to say ⇒ empty bytes"
short-circuit (contract-freeze §C7.6) precisely because its codec always writes a header; the presence
lane had none, so a local presence root that was never published still shipped a record on EVERY
ephemeral frame of every turn. Fixed in the same shape: generation 0 ⇒ empty bytes, generation read once
and reused for the frame's own `presence_generation`.

Landed: `local_interaction_registered_query_channel_continuation_ack_and_close`.

### 2.4 The merge-channel fixture never settled its seeded edit

`merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` dispatches one durable edit
and then calls `ArtifactStore::reset`. On a raw (non-`ContractApp`) fixture `dispatch_typed` alone leaves
the typed operation unsettled, so the store's durable group is not idle and `set_state` refuses — see
§4.1 for what that refusal does. The fixture now settles the edit it seeded
(`settle_registered_typed_operation`) and closes its app.

### 2.5 The resident-aggregate oracle, re-recorded with its derivation

`mounted_document_tree_publishes_nested_interactive_rows` joins
`🧫️fixtures/📃️document-surface.json` to the live constants. `aggregateBytes` read `33 554 432`; the live
`UI_RESIDENT_AGGREGATE_BYTES` is `52 559 872`. It is not a dial: it is
`UI_RESIDENT_SLOTS (64) × UI_DOCUMENT_NODES (128) × size_of::<UiNodeRecord>()`, and
`size_of::<UiNodeRecord>()` grew from exactly 4 096 to 6 416 bytes in a LANDED change
(`🖱️ui/🧬️contract/📃️document/🦀️.rs`, last changed 2026-09-18; the fixture, 2026-09-08). The literal is
re-recorded AND the law now also asserts the derivation itself, so a drift in either factor fails on the
clause that names the cause instead of on a stale number.

### 2.6 The reactor test driver never acknowledged, and never drained, its own publication

`patches_diff` (`⚛️reactor/🧪️tests/⚛️reactor-driver/🦀️.rs`) published a patch and closed its owners, but
never called `PatchTracker::mark_published_ack`. `can_begin` requires
`slot.acknowledged_revision >= reconciler.revision()`, so the SECOND `patches_diff` of the same surface
returned `None` and the revision never rose above 1 — which is why
`revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance` read `UiRevision(1)` where it
wanted 3. The driver now does what a real session's `poll` does
(`⚛️reactor/🔄️turn/🦀️.rs:700`): `SurfaceReconcilePublishedPatch::acknowledge_into` → `mark_published_ack`
→ bounded close of the acknowledgement, then drives the tracker until the surface is admissible again.

### 2.7 The continuation-pacing ceiling, made load-free without moving it

`retained_operation_continues_after_command_admission_until_publication_and_retirement` asserts
`spent <= receipts + TYPED_OPERATION_CONTINUATION_SLACK`. Measured under the fleet (the law now records
and prints its own per-turn trace): a failing draw is
`[(false, true, true), (true, false, true), (true, false, true), (true, false, true), (false, false, true)]`
— five turns, the FIRST of them `contended` without being `runnable`: the shared worker pool was busy
with somebody else, the operation was not even schedulable, and it still cost a counted turn. That is the
machine's pacing, not the continuation's. `spent` now counts every turn EXCEPT a
contended-and-not-runnable one; every runnable turn still carries its exact receipt and the ceiling
constant is untouched. Measured 1 failure in 5 before, 0 in 12 after (in isolation).

### 2.8 Three source-anchor laws re-cut onto the code they name

- `fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers` greps
  `⚛️reactor/🦀️.rs` for four typed-result receivers. All four moved into `⚛️reactor/🔄️turn/🦀️.rs`; the law
  now reads BOTH sources concatenated, so every clause — including the negative one
  (`!contains("typed_operation_results: _")`) — still covers the old location.
- `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` demanded the full-operation
  gate BEFORE the generic command decoder. It cannot be: `admit_command_wire` admits the command's own
  ENCODED wire, so the decoder necessarily precedes its admission. The clause is re-expressed as the
  order it has always stood for — decoder → gate → `dispatch_typed_command_inner` — so nothing generically
  constructed can reach the reducer ahead of the gate.
- `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` delimited the manifest
  command route with `async fn dispatch_typed_command(`, which no longer exists (it folded into
  `dispatch_typed_command_inner`); the route now ends at the next item, `fn addressed_preview_view(`.

### 2.9 Four fixtures that retained an owner past their own close

- `retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers` held
  `let typed_root = app.presence_store.peers_root();` — an `Arc` clone alive until the end of scope, i.e.
  past `drain_and_close_fixture` — so the close blocked on "app-typed presence retirement waits for its
  exact captured root". It now drops the capture after the assertions that use it.
- `peer_roster_saturation_…` never closed any of its three fixture apps; `merge_channel_commands_…` and
  `registry_less_construction_rejects_before_the_reducer` never closed theirs;
  `ui_dispatch_backstop_…` never closed either of the two it builds per classification.

### 2.10 PRODUCT DEFECT — the UI-safety backstop ran AFTER the factory-proof lookup

`admit_command_wire` resolved `qualified_tool_proof(verb)` first and only then (inside
`*_with_proof`) read the verb's manifest declaration and called `validate_ui_dispatch_classification`.
A non-`Migrated` verb has no factory PRECISELY BECAUSE it is not migrated, so every such dispatch
answered `interactive-job.missing-factory` — the consequence — and never
`interactive-job.not-ui-safe`, the cause. In a live guest that is the difference between "this verb was
never migrated" and "this app's factory registration is broken" (memory
`project-interactive-job-classification-gates-dispatch`, `project-bare-bounded-factory-means-every-action-dead`).

Root fix at `🔌️plugin/🦀️.rs`: one `require_ui_safe_declaration(verb)` helper carrying the declaration
lookup and the backstop, called at the HEAD of `admit_command_wire` and `admit_host_configuration_json`,
before any proof is resolved, and reused inside both `*_with_proof` bodies (so the host-configuration
route keeps it too).

Landed: `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command` (which had been getting
`missing-factory`). `unproved_command_fails_before_an_overrun_reducer_can_start` is re-expressed, not
weakened: its fixture is on `contract_registry` (every verb `BatchOnlyPendingRewrite`), so the truthful
first refusal IS `not-ui-safe`; the law now also asserts the app registers no owned factory for that verb
and that the over-budget reducer never started. The migrated registry is not an option for it — an app
declaring migrated verbs with no proof row cannot be CONSTRUCTED at all
(`interactive-job.catalog-incomplete`), which is the same fail-closed join from the other side, and that
is recorded in the fixture's own doc.

### 2.11 PRODUCT DEFECT (test reactor) — keyed latest-wins dedupe was dead

`reactor::spawn_task`'s dedupe doc says a task spawned with the same `(instance, key)` as a live one
"cancels the live one FIRST". The code instead REFUSED, `plugin.task.supersession-pending`, and offered
no route that disposes the previous owner — so the second spawn under a key could never be admitted
while the first was live (a search box respawning a keyed task per keystroke fails on every keystroke
after the first). It now does what the doc says, with the executor's own bounded primitive: `detach` the
previous task's future (dropping it and everything it owns, never polling it, so its body still never
runs) and remove its record, then admit. The refusal is kept for the one case that is genuinely
inconsistent — a key index entry with neither a future nor a record.

Landed: `key_dedupe_cancels_the_previously_live_task_under_the_same_key`.

## 3. The two product findings FP7 flagged

### 3.1 `UnsupportedMemberSnapshotOpen` in two PRODUCTION snapshots — fixed at the root, with laws

`UnsupportedMemberSnapshotOpen::step` has exactly one answer, `Rejected(MemberOpenDiagnostic::Decode)`,
at step 0, always. Two production types declared it, so neither could ever be opened as an OWNED MEMBER
of a composed document — every composed replacement and every document archive carrying one was refused
before it began.

**Flow** (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`).
`FlowHostSnapshot` now declares `store::PackMemberSnapshotOpen<Self>`, which needs
`ArtifactPack` (flow already has the hand-written twin) and `store::retirement::RetireOwned`, which it
did not. Added as `FlowOwnedSnapshotCursor`: a `RetirementCursor` over flow's OWN reserve-then-close
frontier, driving it through `FlowRetirement::close_page` — the entry point flow's own doc names as "the
single entry point every retained driver should use", because a bare `close_step` answers `Blocked`
while an allocation is still owed and a driver that only closes "spins silently forever"
(ticket 26/09/09). New law `flow_opens_as_an_owned_member_through_its_own_pack_codec`
(`🌿️vcs/🧪️tests/🌿️vcs/🦀️.rs`): names the declared opener, round-trips a real fixture flow document
through the opener's own whole-pack decode, and drives the owner cursor to terminal-empty under a 4 KiB
grant, failing loudly on a stall. **RUN AND PASSING**
(`cargo test -p semio-framework-artifact-flow-flow --lib flow_opens_as_an_owned_member`, 1 passed).

**Hub** (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs`, `vcs_integration`).
`HashProjection` — the real `vcs`-backed `VersionGraph` projection, not a fixture — now declares
`store::PackMemberSnapshotOpen<Self>` and implements `RetireOwned` over its one `[u8; 32]` field. New law
`version_graph_member_opens_through_its_own_pack_codec` (`⚙️engine/🧪️tests/🔬️unit/🦀️.rs`), same shape.
**RUN AND PASSING** via the db kernel crate
(`cargo test -p semio-framework-os-kernel-db --lib version_graph_member_opens`, 1 passed);
`cargo check -p semio-framework-os-kernel-db --all-targets` is clean (`fp8-db-check.txt`).
`cargo check -p semio-hub --all-targets` could NOT complete: a peer's in-flight break in a dependency
(`semio-s-artifact-stdio-semio`, `📰️xml/🔖️1.0/✳️any/🦀️.rs:150`, `E0063: missing field quote in
initializer of XmlDeclaration`) fails the build before the hub crate is reached (`fp8-hub-check.txt`).
**Hub code changed → needs coordinator hub rerun.**

A third declaration survives and is NOT this slice's:
`♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:424` (`UnsupportedMemberSnapshotOpen`) — same defect,
same blast radius, untouched.

### 3.2 The peer-roster saturation stall — root-fixed

§2.2. FP7 called it "the best-evidenced single red left"; it is fixed in the peer-roster owner, the law
is green, and the two genuine wall-clock ceilings FP3 §6.3 named
(`tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks`,
`guest_turn_execution_excludes_every_suspension_gap`) stay, unchanged and named.

## 4. Findings NOT fixed, with their evidence

### 4.1 PRODUCT DEFECT (store) — `ArtifactStore::set_state` ABORTS on every refusal

`set_state` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15534`) owns the
`ArtifactEnvelope<P, Mutation>` its caller hands it. `ArtifactEnvelope`'s `Drop` asserts
`owners_detached`. Every early return in `set_state` — `ensure_durable_group_idle()?`,
`validate_durable_history(…)?`, `validate_history_lanes(…)?`, `fold_history(…)?`,
`prepare_document_root_commit(…)?`, `seed_runtime_state(…)?` — drops that envelope plainly while
unwinding, so the assertion fires and the process ABORTS instead of the caller receiving the `VcsError`.
Measured: that is what `merge_channel_commands_…` died on, with the exact backtrace
`ArtifactEnvelope::drop ← drop_glue ← set_state::{closure#0} ← reset::{closure#0}` (the refusal itself was
the non-idle durable group §2.4 fixes). The caller cannot drop the envelope either — no type outside the
store can detach its owners — so the honest fix is for `set_state` to hand the rejected envelope to the
store's own bounded retirement, or to return it inside the error. Not landed: `🏪️store/🦀️.rs` is
`semio-framework-os-kernel`, a hot shared crate under heavy peer churn, and changing that signature
ripples across every caller. Named for the store route owner.

### 4.2 `cancel_instance_tasks` contradicts its own doc, and two laws contradict each other

`cancel_instance_tasks`'s doc says it "drops every task `instance` owns from `EXECUTOR` (dropping its
future, and everything IT owns including any parked `RequestFuture`)". The test-path
`cancel_instance_tasks_step` instead POLLS (`poll_one`) and refuses to remove a task that answers
`Pending` — so a parked task is never cancelled at all, which is why
`instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` finds one registry
slot left where it demands zero. I landed the `detach` the doc describes (round 6): it turns that law
green — and turns `checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume`, GREEN at
round 0, red, because THAT law needs the cancelled task to be polled once so it observes its retired
request (`plugin.request-registry` / "request already consumed or unknown", an oracle row of
`🧫️fixtures/⏳️completion/🔣️.json`). Reordering the second law so the request is retired first, while its
task is still live, passed once and then failed deterministically after a peer's landing on
`🔌️plugin/🦀️.rs` — the retirement does not wake the parked task on its own. **Both changes are
reverted**: trading a round-0 red for a previously-green law is not progress. The two laws state
incompatible things about one function whose doc agrees with neither implementation; that belongs to the
reactor route owner. The keyed-dedupe `detach` of §2.11 is independent and stays.

### 4.3 `editor_fixture_still_mutates_normally` asserts what the design forbids

It builds `new_app::<EditorApp<SurfaceEditorFixture>>()` — registry-less by construction — and requires
the dispatch to MUTATE. `VcsArtifactApp::new`'s own doc says the opposite: "constructs a store-only
wrapper with an empty registry. Every action and typed command fails closed before decoding until a
registry-backed wrapper is constructed", and two sibling laws (§2.1) assert exactly that. It now dies on
`interactive-job.unknown-key` ("typed command 'typed-command' has no exact manifest declaration") instead
of `interactive-job.missing-factory`; both are the same fail-closed refusal. The honest fix preserving
the law's statement is to give `SurfaceEditorFixture` a real manifest, an owned factory and its proof row
— the ~80 lines `DummyApp` carries — so the law proves the EditorApp adapter really reduces. Not landed
for time; no clause was weakened in the meantime.

### 4.4 The remaining fifteen, with their current cause

| law | current panic | class |
|---|---|---|
| `owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally` | the 1,024 request slots stay behind one heap owner | not diagnosed |
| `editor_fixture_still_mutates_normally` | `unknown-key`: no exact manifest declaration | §4.3, fixture |
| `generation_mismatch_is_rejected_with_the_frozen_code` | fixture close blocked: document store close awaits a retained reader or owner | a rejected `transaction_commit` leaves a retained reader; the law never rolls back — sibling laws that leave a transaction merely PENDING close fine, so the leak is on the rejected-commit path |
| `native_aggregate_registry_does_not_allocate_backing_before_admission` | left: 1024 | not diagnosed |
| `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` | child restore is not declared by the loaded parent snapshot | not diagnosed |
| `a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta` | `interactive-job.live-instance` | the fixture never binds its live runtime instance (memory `project-registryless-testkit-new-app-unusable`, second trap) |
| `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations` | key-set inequality | catalog/oracle drift |
| `app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift` | left: 12184115126529414932 | language-neutral oracle drift |
| `checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume` | green on the final tree | §4.2, reverted |
| `child_root_maintenance_requires_terminal_empty_before_reclaim` | the lying owner must never reach a terminal step: Complete | plausibly FP7 §5's `RuntimeLiveCleanupJob`/`Complete` hazard |
| `composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles` | left: 1 | not diagnosed |
| `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` | encoded transaction route must remain explicitly unadmitted | not diagnosed |
| `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | `preparation.ui_patches.is_empty()` | the law loops twice over `late_clock` sharing one process-wide `PATCHES`; a patch queued by the first iteration is emitted by the second's first poll. The law does not say which iteration failed — a successor should record the iteration in the assertion first |
| `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | typed operation lost its persistent worker session before publication | not diagnosed |
| `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity` | TypeId inequality | framework-reserved factory identity drift |

## 5. Honest gaps

- **The gate is NOT green: 15 of round 0's 28 still have a source cause, and the sixteenth
  (`retained_operation_continues_…`) is the load draw named above.** The brief's end state was not reached.
- **`🔌️plugin/🦀️.rs` gained product behaviour that no dependent crate was re-checked against**: one new
  private predicate (`is_registry_less`), one new private method (`orphaned_peer_roster_generation`), one
  new private method (`require_ui_safe_declaration`), a changed `ephemeral_snapshot` body and a changed
  admission ORDER. No public item changed signature and no constant moved, but a successor landing on top
  should `cargo check` a plugin crate.
- **I unblocked a peer's uncompilable file to verify my own flow law.**
  `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🧵️retained/🦀️.rs` and its `📑️copy` sibling referenced a
  local `host_snapshot` that a rename had turned into `fixture` (4 occurrences, `E0425`), which made the
  whole `semio-framework-artifact-flow-flow` lib test target uncompilable. I renamed those four and
  nothing else. With the crate compiling, SIX of its laws are red — all in `retained::copy::tests` and
  `retained::tests`, none touching the member opener, all previously invisible because the target did not
  build. They are the flow owner's, not mine, and are listed in `fp8-flow-member-law.txt`.
- **`🧪️tests/🔬️tool-run/🦀️.rs` needed one arm** for a peer's new `BackboneMessage::Member` variant
  (a member lane's batch counts as a mutations batch of that document) — without it the whole lib test
  target did not compile at round 0.
- **`cargo check -p semio-hub` never completed** — blocked by a peer's break in
  `semio-s-artifact-stdio-semio` (§3.1). The hub change is type-checked only through
  `semio-framework-os-kernel-db`.
- **`--features component-app-assembly` was not exercised**, same as FP4 §7, FP5, FP6, FP7.
- **Two oracle rows of `🧫️fixtures/⏳️completion/🔣️.json`** (`completionFaultCode`,
  `completionFaultMessage`) are joined only by the law §4.2 leaves as it was.
- One `eprintln!("[DEBUG] …")` left by a predecessor in `retained_operation_continues_…` was removed;
  its information is now in the law's own failure message. **25 further `[DEBUG]` lines remain in
  `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` and 29 in `🧪️tests/🧩️composition/🦀️.rs** —
  they predate this slice and belong to other owners (FP7 §5 records the same), so they were not touched.
- Coordinator rule 30 (disk at 21 GiB) arrived after this slice's last build; `CARGO_INCREMENTAL=0`
  applies to any successor's cargo command. Disk read 32 GiB free at the end of this slice.
- Every number here is read from a capture in `🗑️generated/`. Nothing is claimed that was not executed.

## 6. Files changed

Product code (4 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `AppActionRegistry::is_registry_less` + the
  registry-less early return in `validate_tool_job_rows` (§2.1); `orphaned_peer_roster_generation` + the
  stage-7 selection and `.max()` cursor advance (§2.2); the generation-0 presence short-circuit in
  `ephemeral_snapshot` (§2.3); `require_ui_safe_declaration` and the admission reordering in
  `admit_command_wire` / `admit_host_configuration_json` / both `*_with_proof` (§2.10)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` — keyed-task supersession disposes its
  previous owner (§2.11)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs` — `FlowOwnedSnapshotCursor`
  + `RetireOwned for FlowHostSnapshot`, `SnapshotOpen` becomes `PackMemberSnapshotOpen` (§3.1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` — `RetireOwned for HashProjection`,
  `SnapshotOpen` becomes `PackMemberSnapshotOpen` (§3.1) — **hub, check-only**

Laws / fixtures / oracles (9 files):
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — four fixture closes and one capture
  drop (§2.9); the merge fixture's settle (§2.4); the continuation-pacing count and its turn trace (§2.7);
  the unproved fixture's clauses (§2.10)
- `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` — the three source anchors (§2.8)
- `🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs` — the registry-less law's truthful code (§2.1)
- `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` — the `BackboneMessage::Member` arm (§5)
- `🔌️plugin/⚛️reactor/🧪️tests/⚛️reactor-driver/🦀️.rs` — acknowledge + drain (§2.6)
- `🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` — the aggregate derivation clause (§2.5)
- `🔌️plugin/⚛️reactor/🩹️patches/🧫️fixtures/📃️document-surface.json` — `aggregateBytes` re-recorded (§2.5)
- `🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧪️tests/🌿️vcs/🦀️.rs` — new law (§3.1)
- `🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — new law (§3.1)

Peer file unblocked (2 files, 4 identifiers): `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🧵️retained/🦀️.rs`,
`…/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs` (§5).

Captures (`🗑️generated/`): `fp8-round0…round7-serial.txt`, `fp8-final-serial.txt`, `fp8-round*.names`,
`fp8-round*-panics.txt`, `fp8-final.names`, `fp8-final-panics.txt`, `fp8-bin-path.txt`,
`fp8-build-err.txt`, `fp8-flow-check.txt`, `fp8-flow-member-law.txt`, `fp8-db-check.txt`,
`fp8-hub-check.txt`. Scratch: `🐍️fp8-decode.py`.
