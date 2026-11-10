# 1108 world result
Step 1: ported `semio-os-infinite-world-model` 🎒️prepared-resources close ladder to the effect-shaped close.
- World3dBuildRejected::close_step(effect)->WorldClose (=PreparedClose); Upload takes the effect, raster owners/eviction string use effect.grant().
- World3dBuildContext::close_step(cx) = prepared_close_turn(cx,quote,close_effect); ladder is close_ladder(effect).
- Cursor wake: ControlledRetirement<ArcLease<Mutex<WorldCursorWakeState>>> (WorldCursorWakeClose), stepped via prepared_close_poll(owner.step(body,effect)); birth quote carries copy bytes.
- prepare_step: work u128.
Check: W-world-1 (native, own manifest) running.
Rerun: chk.sh W-world-<n> <model Cargo.toml> native|wasm
STATUS: IN PROGRESS
Step 2: model lib native W-world-2 rc 0, wasm32-wasip2 W-world-3 rc 0. Host test-target build W-world-4 next.
Step 3: host test-target build native W-world-9 rc 0 (semio-os-infinite-world-host --tests). Extra ports needed to get there:
- paged-list/🦀️.rs:46 captured_advance_selection UFCS (unclaimed upstream break).
- world-components placeholder-mesh cursor: step/close_step/retire_source_step take &mut RetainedAllocationEffect (png advance(effect), ControlledRetirement::step(body,effect)).
- world-interaction: step_world_placeholder_mesh(.., effect), batch(.., effect), INTERFACE pump_world_placeholder_mesh (OriginalRetirementPump turn; fuel 64, grant 4096/1MiB/16MiB/16MiB/256, clock Some(0)); host sync bridge + dynamic-retirement use context.with_original_effect; render uses the pump.
- world-scene terrain cursor: close_step/step_live(grant, context) with context.with_original_effect.
Debt: tests in components/scene/semio-os-infinite resources crates still use old signatures (not host target). Host tests W-world-10 next (SLOT_HANG_SECS=100).
Step 4: the host test targets need `--features testkit` (W-world-9/10 were hollow: 0 tests). W-world-11 (`chk.sh W-world-11 <host Cargo.toml> native --tests --features testkit`): 80 errors, all in host TEST sources, 4 targets (world_host_unit 8, unit_2 53, unit_3 13, bridge_empty_build 2) plus pointer-gestures/scene-input-residency/journal-sequences. Stale test APIs, not the close ladder: semio_framework_async::block_on gone, semio_framework_job::close_with_self_funded_context gone, read_static missing, WorldOverlayState::scalar_field_status gone, close_step(grant)->Result still used on cursors (world cursors now take effect/context), InteractiveJobStep no longer has terminal_is_empty/close_step. Needs its own test-port lane (`world-tests`).
Rerun: SLOT_HANG_SECS=100 tst.sh W-world-<n> <host Cargo.toml> native --features testkit
Verified libs: model native W-world-2 + wasm W-world-3; host lib (+components, interaction, scene) native W-world-9 rc 0.
STATUS: BLOCKED host test targets are stale test debt (80 errors), libs FIXED-verified
Step 5 (test-port lane): host test sources compile (W-world-14 rc 0, `--tests --features testkit`). Host run W-world-20 (SLOT_HANG_SECS=100, no hang): journal-sequences 14/14, pointer-gestures 9/9, scene-input-residency 3/3, surface-verbs 2/2, bridge-empty-build 1/3, unit 82/86, unit-2 34/71, unit-3 3/14.
Ported: read_static restored (fixture_text), block_on via async `entrypoint` dev feature, snapshot_fault/scalar_field_status fields, prepared render job drive via ui-prepared testing (drive_preparation_until_terminal, close_job_step, close_packet_step), draw-rebuild descriptor borrows, mesh_requests iteration.
Testkit (single sources, feature testkit): with_world_step_context (fixture-caller turn, admission fuel measured once and asserted equal, retirement fuel via OriginalJobFixtureCaller::fund_fuel, 4096-turn fail-fast), world_original_fixture_grant, publish_oracle_mesh family, world_test_gate (process-wide mesh authority/registries serialize all 202 laws; no Busy retry). 8 duplicate helper copies removed.
Quote-time check: interaction WORLD_PLACEHOLDER_PUMP_GRANT/FUEL + world_placeholder_pump_admits (pump faults the build if the grant cannot admit the cursor quote); law placeholder_pump_grant_admits_... (currently fails: WorldPlaceholderMeshCursor::inline(default mesh) -> Schema; use a minimal valid inline mesh).
Open failures (W-world-20): scene bridge stops Idle/Fault(Capacity) on degenerate payloads (bridge-empty-build 2); 'world pass never satisfied readiness' (GPU render path, unit-3 9); 'original world resources dropped before paid close' x3 and 'original slot page remains owned' x3 (tests drop World3dBuildContext without real close_step); camera-fit idle at turn 0 x3; unit 4 (dynamic_world_owners_retire..., six_reference_gpu..., scene_bridge_retains..., instance_lane...). Not started: components + scene crate tests, scene crate own with_world_step_context copy.
Rerun: SLOT_HANG_SECS=100 tst.sh W-world-<n> <host Cargo.toml> native --features testkit
STATUS: BLOCKED host tests 28 of ~60 failures remain after port (see open failures); libs FIXED-verified (W-world-2/3/9)
