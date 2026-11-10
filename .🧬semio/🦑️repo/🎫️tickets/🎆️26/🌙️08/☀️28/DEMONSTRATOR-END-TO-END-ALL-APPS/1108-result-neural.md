# Lane neural: semio-os-kernel-neural-engine-model

Step 1: native lib GREEN W-neural-3 rc 0 (was 35 errors, all in engine/🧵️retirement + TreeSnapshot).
- Owner::Shared and TreeSnapshot.{tree,seeds} are SharedOwner (=SharedOriginalStorage); lease structs removed; ColdRetire for SharedOwner via elect_cold.
- ValueRetirement/InputRetirement/OperatorRetirement::close_step(body,effect)->poll; RetirementCursor via close_into + pub close_reply/CloseReply/CloseComplete; original_retirement_cursor! macro updated.
- insert_controlled: advance_native under control.with_effect(exact native_demands/native_work_demand).
- Input key/value admission: SharedOwner::admit_from on the turn effect (work not re-paid).
INTERFACE: TreeSnapshot fields/capture_retained now SharedOwner (flow-host, evaluation, topology callers must port).
Rerun: chk.sh W-neural-<n> 🌎️hub/Cargo.toml native -p semio-os-kernel-neural-engine-model
Next: wasm, tests.

Step 2: wasm32-wasip2 lib GREEN W-neural-4 rc 0.
Step 3 (tests, root Cargo.toml; hub manifest lacks dev-dep resolution, framework manifest lacks the package): test-code port EDITED-unverified:
- model testkit: observe_ownership delegates to semio_framework_value_testing_allocator::observe_retirement_allocations; ObservedSystem global allocator + SHELL_OBSERVATION removed (that crate is the one test global allocator; LOCK NEEDED logged for the model Cargo.toml dev/optional dep).
- retirement tests: global_allocator static removed, body-passing helpers, TreeSnapshot via SharedOwner::from_cold, BodyCursor via native_retirement_reply!.
- dictionary-and-registry-laws: global_allocator removed.
Blocker W-neural-8: peer red semio-framework-replication-mutation-messages (RetirementHeaderRelease missing in semio_framework_value) - not mine.
DEBT: engine/registry/evaluation tests still import ObservedSystem/ShellObservation/SHELL_OBSERVATION (removed); evaluation/topology/flow-host callers of TreeSnapshot use Arc.

Step 4: model tests GREEN native W-neural-15: 20/20 (root Cargo.toml, SLOT_HANG_SECS=90). Fixes: cold_effect via invocation_extent + run_invoked (reply-shaped children need effect.invocation()); input shared key/value quotes = SharedOwner::source_admission_demand; testkit nested scopes by difference via additive retirement_allocations_so_far in value testing allocator.
Step 5: semio-os-kernel-neural-engine-registry lib native GREEN W-neural-18 rc 0 (close_step(body,effect), value.step(body,effect)). Wasm and registry tests not run.
DEBT: registry/evaluation tests import removed ObservedSystem/SHELL_OBSERVATION; evaluation/topology/flow-host use Arc TreeSnapshot.
STATUS: FIXED-verified W-neural-15 (model native+wasm lib, tests 20/20) and W-neural-18 (registry native lib)

Step 6: registry wasm32-wasip2 lib GREEN W-neural-20 rc 0.
Step 7 (coordinator follow-up) BLOCKED/not started: semio-os-kernel-neural-engine-budgeted (+ engine) is a separate full port, W-neural-19 = ~45 errors, native red:
- evaluation 🦀️.rs (BudgetedEvalState state machine): Arc<Tree/seeds/infos/cache/channels> + *Lease wrappers -> SharedOwner; SharedControlledRetirement::lease gone; ControlledRetirement::step(body,effect); SharedOwner::admit -> admit_from; EvalRetirement close_step -> close_into; NeuralCacheRetirement (lease, next_*_demand, step).
- topology (TreeSourceLease), cache, tree/copy, wave: close_step(body,effect) / RetirementCursor close_into.
- input 🦀️.rs: OriginalOwnedEntry API changed (no step_progress/advance/close_step) -> needs the ordered owned-entry reply API.
- Then flow-host TreeSnapshot callers: evaluation-source work (capture_retained with Arc<Tree>) and read tests, plus Arc<TreeSnapshot> shared-retirement in flow-host/eval-session-state/publication (shared_retirement_birth_bytes::<TreeSnapshot>).
- Registry + evaluation tests off ObservedSystem/SHELL_OBSERVATION: registry tests rely on shell-address recording which the shared value allocator cannot provide; needs a replacement observation (design decision).
No half-ported edits left. Recommend a dedicated lane for budgeted engine; claim released.
