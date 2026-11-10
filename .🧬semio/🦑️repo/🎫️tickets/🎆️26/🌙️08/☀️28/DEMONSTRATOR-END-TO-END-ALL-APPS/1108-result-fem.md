# Lane fem result
## Step 1 edits (unverified)
- semio-s-fem-material 🩹️patch/🦀️.rs: cursor ported. take(&mut out, effect) poll, advance maps NativeReceivingFailure via `native`, begin_close only flags (field start paid via begin_close_into), close_step/recover_close_step reply-shaped (OriginalEffectReply) through `close_route`.
- semio-s-fem-payload-turns: numerical_writer_close(slot, &mut StepContext) -> Option<Result<InteractiveJobCloseStep, OriginalTurnFailure>> (was grant, ValueError); finish_writer uses close_original_effect_step + builder.close_step(effect).
- semio-s-plugin-energy-engine-sim-retirement: energy_builder_close(slot, &mut RetainedAllocationEffect) (was grant); callers in sim (packets:309, close:70/77/81) must use energy_effect_step.
- INTERFACE changes: callers of numerical_writer_close (fem mesh-job-run:493, ldlt run-and-close:138/140, subspace run-and-publish:268, tests support:149) must pass cx.
## Labels
W-fem-2 native (queued behind W-fem-1)
## Step 2 caller ports (unverified, coordinator ordered)
- API settled effect-shaped: numerical_writer_close(slot, effect)->Option<Result<InteractiveJobCloseStep,ValueError>> (revised from cx form), NumericalBytesPublication::close_step(effect), energy_builder_close(slot, effect).
- sparse protocol: numerical_payload_close/numerical_restore_payload_close/desk.close_step take effect; new numerical_close_effect (numerical_close_original kept for pcg/analyses until ported); retire_delivered/restore_close_due use close_original_effect_step; desk.retire_all(cx), AssemblyJob::retire_outcome(cx).
- ldlt run-and-close + restore-cursor, subspace run-and-publish + restore-cursor, mesh-job-run (close_granted(effect)), energy sim packets/close ported.
- Open: pcg, analyses, assembly close_granted + their payload closes; fem tests support (close_writer, close_turn); patch tests.
## Step 3 verified native
- W-fem-3 rc1 overall but GREEN: semio-s-fem-material, semio-s-fem-payload-turns, semio-s-plugin-energy-engine-sim-retirement, semio-s-fem-sparse, -sparse-ldlt, -sparse-subspace.
- Caller ports done after W-fem-3 (assembly, pcg, mesh, analyses imports, energy sim wire/close/restore/batch/admission/packets; body arg = effect.grant().maximum_copy_bytes for step_granted/step): W-fem-4/5 show no error in them but downstream is masked by peer reds in semio-framework-actor (wire-turn StagedSourceAdmission is_projecting/project_owned/owned_projection_demand) and semio-framework-replication-mutation-messages (RetirementHeaderRelease). Recheck mesh/pcg/analyses/assembly/energy-sim once those are green:
  chk.sh W-fem-<n> 🌎️hub/Cargo.toml native -p semio-s-fem-analyses -p semio-s-fem-analyses-assembly -p semio-s-fem-mesh -p semio-s-fem-sparse-pcg -p semio-s-plugin-energy-engine-sim
- wasm: W-fem-6 (running).
- Open: fem tests support (close_writer, close_turn), patch tests, sparse/ldlt/subspace/pcg/mesh unit tests (still grant-based close_granted/close_turn callers).
## Step 4 wasm + tests
- wasm32-wasip2 W-fem-6 rc 0: material, payload-turns, sim-retirement, sparse, sparse-ldlt, sparse-subspace.
- fem tests support (tests/🧰️support) ported to payload_close / close_original_effect_step (uncompiled).
- Material `--tests` (W-fem-7) BLOCKED: lib-test target lacks dev-deps (serde_json, semio_framework_value_native_receiving_original, semio_framework_value_snapshot_projection => LOCK NEEDED 🧱️material Cargo.toml) and NativeReceive API drift (begin_receive, receive_birth_demand, NativeRecordSource gone); also semio-framework-job-native-turn birth/🦀️.rs:129 E0308 (peer). Patch tests still use grant advance/take; need a full harness port onto the effect-driven receive.
- Unverified downstream (masked by peer reds semio-framework-actor wire-turn, replication-mutation-messages): fem mesh, pcg, analyses, assembly, energy sim.
- Rerun: chk.sh W-fem-<n> 🌎️hub/Cargo.toml native -p semio-s-fem-analyses -p semio-s-fem-analyses-assembly -p semio-s-fem-mesh -p semio-s-fem-sparse-pcg -p semio-s-plugin-energy-engine-sim
STATUS: FIXED-verified W-fem-6 (material, payload-turns, sim-retirement; tests + downstream pending)
## Step 5 downstream + tests (final state)
- FIXED-verified native W-fem-8 rc 0: semio-s-fem-analyses, -analyses-assembly, -mesh, -sparse-pcg, semio-s-plugin-energy-engine-sim (effect-shaped closes; actor/messages reds cleared).
- Material `--tests` must run through the crate's own workspace: chk.sh W-fem-<n> ✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/Cargo.toml native --tests -p semio-s-fem-material. W-fem-10: dev-deps now resolve; 16 errors remain, all in the test harness (tests/🦀️.rs, 📋️table/🧪️tests, 🩹️patch/🧪️tests): removed APIs NativeRecordSource, NativeReceive::begin_receive/receive_birth_demand, NativeDecodeConstructionRecipient, double #[derive(Default)] on Recipient (tests/🦀️.rs:10-11), grant-based owner.advance/take(g).
- Patch-test harness port: BLOCKED (not done). Needs an effect-driven receiving harness (reference: energy bridge `receive_stepped` in ✏️s/🔌️plugins/🔋️energy/…/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs: OriginalSourceTurn witness, NativeReceiveSlot, AuthoringWallet-run effects) and the cursor cut/denied-frontier laws rewritten for reply-shaped close_step/take(poll). Cursor itself is lib-verified.
- fem tests support (tests/🧰️support) ported but never compiled (--tests of fem engine crates not run); ldlt/subspace/pcg/mesh unit tests still call grant-based close_granted/close_turn.
STATUS: FIXED-verified W-fem-8 (libs: material, payload-turns, sim-retirement, sparse family, mesh, pcg, analyses, assembly, energy sim; wasm W-fem-6 for first 6). Tests BLOCKED: harness port
## Step 6 wasm of downstream, test helper, smoke
- wasm32-wasip2 W-fem-11 rc 0: fem-analyses, -analyses-assembly, -mesh, -sparse-pcg, plugin-energy-engine-sim.
- tests/🧰️support: `granted(grant, run)` now drives a genuine effect through the shared EffectBank (`bank_run`, semio-framework-value-testing-admission); payload_close and close_writer use it; mesh unit tests call `granted(.., |effect| job.close_granted(effect))` (3 sites). UNCOMPILED: LOCK NEEDED for the support crate's new path dep (claims log). ldlt/subspace/pcg unit tests call only the support `close_turn(job, grant)` (cx based), so need no port.
- statik smoke W-fem-s1 (1105b-tools/statik/smoke.sh): rc 1, upstream-only red: semio-os-store-snapshot-clone 40 errors (RetainedCloneSourceTake missing, OriginalVectorCursor take_front/admit/step, ErasedSnapshotRetirement::close_step arity) = lane storewire, not fem.
- Open: run fem unit tests after lock; material patch/receive test harness port (effect-driven, EffectBank + receive_stepped reference).
STATUS: FIXED-verified W-fem-11 (libs native+wasm); tests/harness/smoke BLOCKED (lock + storewire red)
## Step 7 unit-test port (W-fem-t11, native --lib via ✏️s/Cargo.toml workspace)
- Ported to compile: sparse/ldlt/subspace/pcg/mesh unit tests (granted() EffectBank helper, reserve/release effect args, admit_original_construction admission+retire, numerical_batch_plain, step_plain bounds, restore cursor .step, mesh step_plain); removed 4 duplicate #[global_allocator] (testing-allocator wraps it); fem-test-support gained path dep on testing-admission (sync-128).
- Counts (pass/fail): mesh 9/7, sparse 5/6, ldlt 6/5, pcg 11/12, subspace 9/8 = 40 pass / 38 fail.
- Failure signature (all but a few): OriginalTurnFailure context_refusal "original work loan callback replaced its initiating context or refilled its work" (job ⏱️context with_original_work_loan, StepContext 🦀️.rs:275) raised when a fixture turn with fuel 1..4 runs a close_original_effect_step (retire_delivered, numerical_restore_close_due, numerical_close_effect) -> support close_job panics "a locally owned job close has no external owner and no refusal" (support 🦀️.rs:100); plus 2 `Admission` faults in restore tests. Suspect: effect-step work charged twice (with_original_effect performed + fold_original_close consume_work(turn.work)) against the 1-fuel loan, or nested effect inside with_original_work_loan. Needs the driver/jctests lane's ruling on whether close_original_effect_step may run inside a fixture work loan and what `work` a closure should return.
- Open: material patch harness (effect-driven receive); rerun statik smoke after storewire FIXED.
