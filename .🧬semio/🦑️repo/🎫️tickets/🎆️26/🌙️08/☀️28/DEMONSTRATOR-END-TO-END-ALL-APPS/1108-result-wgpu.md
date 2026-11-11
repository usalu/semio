# 1108 wgpu result

## Edits
- host-base 28 errors: NOT a manifest/lib fault. The whole wgpu partition (636 `cfg(target_arch="wasm32")` sites in 88 files, deps gated `all(wasm32, target_os="unknown")`) is browser-only; a11y2 checked it with `wasm` = wasm32-wasip2, where wasm-bindgen/js-sys/web-sys/semio-framework-async are (correctly) absent. Right check target: wasm32-unknown-unknown (chk.sh `native` mode + `--target wasm32-unknown-unknown`). No Cargo.toml edit, no LOCK NEEDED.
- semio-ui-wgpu-engine (5): ui-state-and-presented-input close_surface_one -> `close_original_effect_step(cx,demand,|effect|failure.step(bytes,effect))`; layout-stepping alias step -> `with_original_effect` result matched (ValueError -> WorkerJobDemandError::Refused), `consume_work(u128)`; dispatch-and-overlay-api `RetainedPaintPhase::Borders => "borders"`.
- dag/host (6): value `🔐️mutex/🦀️.rs` gained `RetireOwned for std::cell::Cell<T>` (reuses the mutex Cursor) for DagPaintState anim-phase cells; `DagHostRetirement` now `next_close_demand(body)` + `close_step(body,effect)->OriginalRetirementPoll`; NodeGraph element `GraphHostRetirement::close_step` uses `close_original_effect_step`.
- decl-staged-form (16): payload/control/body/output cursors are their own retirement cursors (`staged_close!` macro in payload.rs + `settle_close`), `native_retirement_reply!`, `next_computational_work`.

## Debt
- dag-host `🧪️tests/🎛️input` and flow host `🧹️retirement` tests still call the old `DagHostRetirement` grant API (`next_close_*_byte_demand`, `close_step(grant)`) and old `admit_owned_retirement`; not ported.

## Verification so far (11/10 03:00)
- W-wgpu-3 (norm app-surface, wasm32-wasip2, norm manifest): `semio-plugin-decl-staged-form` is "Checking" with no error: FIXED (16 -> 0). Norm still rc1 behind NEW unclaimed reds: `semio-plugin-window-kits` 6 (registry/original: `admit_shared_retirement` gone, `Arc<AppDefinition>: RetireOwned`, `RetainedWorkOutcome::map_err`, 2-arg step), `semio-plugin-decl-media` 21 (old `retirement()`/`SharedControlledRetirement::lease`), `semio-plugin-decl-emission` 73 (old `close_step(grant)`/`retirement()`). decl-views and window-transient no longer appear.
- W-wgpu-1 (shell-session, `native` + `--target wasm32-unknown-unknown`): only red is `semio-framework-job-context` 2 errors: `OriginalSystemClock` is `cfg(any(not(wasm32),env="p2"))` (job/clock-identity-and-budget/recorded), but `admit_system_clock` (job/context/source, called by receiving/native-turn/birth `advance`) uses it unconditionally. There is no browser Source clock producer anywhere (no web-sys/js-sys in the job tier by design). So NO browser (wasm32-unknown-unknown) build of the 733 job stack compiles; needs a job/clock lane to add a browser `OriginalClockProducer` supplied by the wgpu host-base (it owns web-sys) through `original_host_clock`, or cfg-split `advance`. Not taken: design call in the job core.
- W-wgpu-4 (shell-session native): ui-wgpu-engine, dag/host, host-base, NodeGraph, decl-staged-form not in the red list. Reds: `semio-ui-wgpu-hosting` 1 (clipboard fault `close_step_granted`; fixed afterwards, see W-wgpu-6), `semio-os-store-sync` 2 (`admit_clone(grant)` in envelopes_from_history_edit and actor-identity serde: needs an effect threaded through a public async api), `semio-os-services-native-io` 7 (`close_step_granted` payload, type annotations, mismatches). store-sync and native-io unclaimed, not taken.
- W-wgpu-2 invalid: `-p semio-ui-wgpu-engine` is not a package of that manifest's resolve (use the shell-session -p).

- semio-ui-wgpu-hosting (found in W-wgpu-4): `wgpu/🏃️host` clipboard fault close now `close_original_effect_step(cx,demand,|effect|RetainedWorkPoll::Complete(self.fault.close_step(effect)))`.

## Verified (11/10 04:20)
- W-wgpu-5 (= W-a11y2-19 command, wasip2): semio-ui-wgpu-engine and dag/host no longer fail (were 5 and 6); the navbar `self.prefs.active_locale()` code is in the shell-session closure but the crate is still never reached (see below).
- W-wgpu-3 (norm, wasip2): decl-staged-form compiles.
- W-wgpu-6 (shell-session native): hosting, ui-wgpu-engine, dag/host, host-base native all compile; remaining reds below.
- Not run: generation3d pane check (W-a11y2-21 command): its blockers (decl-views, window-transient, dag-host) are now gone on this tree, but the shared closure still hits window-kits/decl-media/decl-emission; would only reproduce those. Rerun after they are green: `"$C/1108-a11y2-run.sh" 21 "<generation3d Cargo.toml>|semio-s-artifact-procedural-generation3d"`.

## What remains (all unclaimed, none a11y-caused)
1. `semio-renderer-wgpu-media-slots` 29 (wgpu partition, both native and wasip2): `RetainedCloneSource::admit_owned/project_owned/take_authority` + `RetainedCloneSourceTake` are gone; the 733 API is `take_authority_into` / `*_into` with `OriginalEffectReply` and `NativeAllocationIdentity` (u64 compare in props). 30 KB phase machine in `🎬️media-slots/{🎟️collect,📤️slot,📥️props}`; needs its own lane.
2. `semio-os-store-sync` 2: `admit_clone(RetainedCloneGrant)` in `envelopes_from_history_edit` and actor-identity envelope serde (public async api takes a grant; needs an effect/turn threaded through callers).
3. `semio-os-services-native-io` 7: `close_step_granted` payload, E0283 annotations, mismatches at lib 86/231/335/385/386, path-return 51.
4. `semio-plugin-window-kits` 6 (`📇️registry/🌱️original`: `admit_shared_retirement` removed, `Arc<AppDefinition>: RetireOwned`, step arity), `semio-plugin-decl-media` 21, `semio-plugin-decl-emission` 73 (old `retirement()` / `close_step(grant)`): block norm + all panes.
5. Browser target (wasm32-unknown-unknown) cannot compile the job stack: `semio-framework-job-context` `admit_system_clock` needs `OriginalSystemClock`, cfg'd out on browser; no browser `OriginalClockProducer` exists (wgpu host-base, which owns web-sys, is the natural supplier via `original_host_clock`). `semio-renderer-wgpu-contracts` (needs `semio_framework_async_slots`, unknown-only) and host-base 28 are wasip2 target mismatches, expected red there; they are the right checks on unknown only after item 5.
6. Test debt: dag-host `🧪️tests/🎛️input` and flow host `🧹️retirement` tests use the old `DagHostRetirement`/`admit_owned_retirement` grant API.

Rerun: `"$C/1105b-tools/chk.sh" W-wgpu-<n> "<repo>/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚶️shell-session/📦️packages/🦀️rust/Cargo.toml" native -p semio-renderer-wgpu-shell-session` (add `--target wasm32-unknown-unknown` once item 5 is fixed; `wasm` mode is wasip2).

STATUS: BLOCKED shell-session/panes behind media-slots 29, store-sync 2, native-io 7, window-kits 6, decl-media 21, decl-emission 73 and the missing browser job clock; my four crates are FIXED-verified (ui-wgpu-engine, dag/host W-wgpu-5/6, decl-staged-form W-wgpu-3, host-base = target gating, no edit)
