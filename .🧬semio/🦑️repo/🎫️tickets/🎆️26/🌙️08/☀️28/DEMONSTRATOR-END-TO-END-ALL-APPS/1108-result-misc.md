# 1108 misc result
Libs FIXED-verified native + wasm32-wasip2: actor-job-state, job-owned-text, os-store-child, ui-prepared-staging (W-misc-3/4), ui-prepared (W-misc-10/11).
- actor-job-state: close_owned_actor_slot passes body=grant.maximum_copy_bytes to owner.step.
- job-owned-text: FundedText::close_step(body, effect); test fixed (step_granted(4096,..), local #[global_allocator] removed: shared testing allocator owns it). Rerun W-misc-12.
- ui-prepared-staging: SceneRasterLease::next_close_demand(body)/close_step(body,effect). prepared_close_authored now (cfg test|testkit) uses shared testing-admission `driven` + EffectBank instead of AuthoringWallet (coordinator ruling). Cargo.toml: optional dep behind testkit + dev-dep -> LOCK NEEDED (posted). Test W-misc-13.
- os-store-child: AliasRetirement::close_into; ArtifactChildCloneCursor advance_into/close_into/close_step(body); fields driven via native_reply helper.
- ui-prepared: render-job-state close_step(effect) (no pay), unused import removed.
Tests: ui-prepared-staging 19/19 (W-misc-15; local #[global_allocator] removed, bank stock 1<<40 like other harness users). actor-job-state builds (no tests, W-misc-9 rc 0).
Update (late):
- ui layout FIXED-verified: semio-ui-wgpu-layout native W-misc-27 + wasm W-misc-29; semio-ui-wgpu-mounted-layout native W-misc-31 + wasm W-misc-33. INTERFACE: FlexTree::fund_next_node/fund_scratch(effect), close_granted(effect); MountedLayoutJob::close_one() (no grant); close_step runs close_granted through cx.with_original_effect.
- os-store-child: cursor gained begin_close_into + native_terminal_release; receive test on shared driver (green W-misc-30, 6 s); selected_close_tests green W-misc-26 (spent-take runs a real paid close). Full-suite rerun W-misc-32 blocked by a peer mid-edit in job/context testing caller (E0424 self); rerun: tst.sh W-misc-<n> <store-child Cargo.toml> native -p semio-os-store-child.
- ui-prepared-staging tests 19/19 (W-misc-15).
Open:
- job-owned-text tests: wait for driver2 birth/close fix, rerun tst.sh W-misc-<n> <job-owned-text Cargo.toml> native -p semio-framework-job-owned-text.
- renderer-wgpu test target (W-misc-20): fails in store replay-operation native drivers (lane replayop).
- world-host test target (W-misc-34): fails in os infinite world model 🎒️prepared-resources (12 errors, grant-shaped close ladder, RELEASED unclaimed).
- LOCK NEEDED posted: staging, store-child, layout-stack dev-deps; ui-prepared, renderer-wgpu, world-host testkit features.
STATUS: BLOCKED job-owned-text awaits driver2; world model prepared-resources + replayop drivers unclaimed/other lane; libs all FIXED-verified
