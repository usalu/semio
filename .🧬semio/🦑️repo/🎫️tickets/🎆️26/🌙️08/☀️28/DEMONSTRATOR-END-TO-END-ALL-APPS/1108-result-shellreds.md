# shellreds lane result (11/10)

## Done
1. wgpu-dock (18): `StackCapLayout`, `StackCapGroupLayout`, `StackCapTabLayout` are now `pub` (they are the return/parameter types of the already-public `layout_stack_cap` / `stack_window_silhouette`; re-exported from the dock root with docstrings). `DockAxisSlots` is `pub(super)` like the `pub(super) dock_axis_slots` that returns it. Native `W-shellreds-1` rc 0.
2. wgpu-host-base (7): `📥️lifted/🔁️asset-decode-jobs`: no new dependency, no LOCK NEEDED. Closes now run through `cx.with_original_effect(grant, |_, effect| ..)` and `let Some((step, _)) = .. else { return Ok(..) }` (a refused quote runs nothing), work charged with `cx.consume_work(u128)`. The two cx-less closes (`close_detached_result`, `close_renderer_io_payload`): the detached result now closes inside a paid `caller.turn`; `close_renderer_io_payload` (+ its step constant) had no caller anywhere and is deleted. Native `W-shellreds-3` rc 0. Browser target: `W-shellreds-5` (wasm32-unknown-unknown host-base + dock), see Verification.
3. reactor-app `advance_instance_birth`: the identity-binding receipt no longer goes through `admit_original_construction` (6-arg API with a resident admission); it pays its one item + one fuel directly (`fits` check, Pending returns `Ok(false)` and keeps `birth`). Native rc 0.
4. wasmtime-node-host: one `PluginRegistration { actor, origin }` per plugin (`registrations` map), copied into the three routers. Origin comes from the process origin registry (see 6b). Removed: `ServiceActorOpening`, `prepare_service_original`, `service_identities/openings/identity_refusal`, `direct_identity` (also dropped from `WasmtimeNodeHost::new` and `run/bootstrap`). New `run_root_steps` (Run-root actor turns until the step settles, idle guard) and `retire_registration` (closes the origin on unload / hot reload).
5. mcp inference channel: `infer_real` registers a call-scoped origin through the headless host driver (`drive_origin`), builds the router with a `PluginRegistration`, calls `router.infer(&bytes, &drivers)` where `drivers` issues one fresh `GuestRelayWakeDriver` per hop from `caps` (cancel closure inside), polls the future with a noop waker (`poll_to_completion`, each Pending poll is a paid turn of the call's own driver), then closes the origin. No `block_on`, no `GuestRelayWakeAuthority`, no `PluginInstanceHandle`. Removed `open_original_instance`, `ensure_inference_route`, `settle_inference_operation`, `PluginInferenceRoute` and the fields `inference`, `startup_custody`, `startup_instance_custody`, `operation_refusal` (the call owns and retires its drivers). `startup_failure` stays (instances use it).
6a. `JOB_KIND_*`: one definition in `plugin/🧬️schema/💼️job-kinds/🦀️.rs` (next to `📜️jobs.wit`, whose kind strings it names), mounted as `semio_plugin_abi::job_kinds` and re-exported as `semio_plugin_guest_contract::job_kinds`. `reactor-jobs` and `host-routers` re-export from it; the duplicated strings are gone. `JOB_KIND_COMPOSE` lives there too (host-routed, not a reactor builtin). No Cargo.toml edits, no LOCK NEEDED (reactor-jobs already depends on plugin-abi, routers on guest-contract).
6b. Origin registry: `run/🧠️wasmtime-node-host/🗂️origin-registry.rs` was an unwired file in the run host; the mcp channel needs the same thing, so it moved to `post-turn-relay/🏠️native/🗂️origin-registry.rs` (`pub mod origin_registry`, + `next_identity()`), exported as `semio_plugin_host::origin_registry`. One process registry, one definition.

## Verification
- `chk.sh W-shellreds-5 <host-base Cargo.toml> native --target wasm32-unknown-unknown -p semio-renderer-wgpu-host-base -p semio-renderer-wgpu-dock`: rc 0 (the ported asset-decode closes are `cfg(not(wasm32))`, so this proves no browser regression only).
- `chk.sh W-shellreds-1 <dock Cargo.toml> native -p semio-renderer-wgpu-dock`: rc 0.
- `chk.sh W-shellreds-3 <host-base Cargo.toml> native -p semio-renderer-wgpu-host-base -p semio-plugin-reactor-app -p semio-plugin-host-post-turn-relay -p semio-plugin-host-routers -p semio-plugin-abi -p semio-plugin-guest-contract`: rc 0.
- UNVERIFIED (blocked behind other reds, my files show no diagnostics): `semio-os-run` (node host), `semio-os-mcp-workspace` (channel), `semio-plugin-reactor-jobs`. `W-shellreds-4` stops at `semio-plugin-plugin-program` (8 errors, custody lane: surfaces/native.rs, Cargo LOCK NEEDED 148), `semio-plugin-time-travel` (55), `semio-os-mcp-port` (7, dispatch/channel-port l.337), `semio-os-mcp-probe-document` (20), `semio-os-host-space` (6), `semio-framework-artifact-workflow-run` (14).
- Shell W-shellreds-2 (`-p semio-renderer-wgpu-shell-session` + routers + reactor-app + reactor-jobs, native): rc 1, crates red: plugin-program 8 (custody), publication-mounted 6 (factoryticket), time-travel 55 (no owner seen: `ErasedSnapshotRetirement::close_step` 3-arg, `retirement/🦀️.rs`, `🎮️decision/🗑️discard`), wgpu-host-plumbing 54, wgpu-agent-elements 1. (command lane's crate did not appear this run.)

## Remaining reds and owners
| crate | errors | owner |
|---|---|---|
| semio-plugin-publication-mounted | 6 | factoryticket |
| semio-plugin-plugin-program | 8 | custody (claimed 07:58, W-custody-1 unverified) |
| semio-plugin-time-travel | 55 | unowned (chained released UNVERIFIED edits) |
| semio-renderer-wgpu-host-plumbing | 54 | unowned (new in this run's reachable set) |
| semio-renderer-wgpu-agent-elements | 1 | unowned |
| semio-os-mcp-port / mcp-probe-document / os-host-space / artifact-workflow-run | 7 / 20 / 6 / 14 | unowned |

## Debts
- `poll_to_completion` in the channel is a noop-waker poll loop on the caller thread (the channel trait is synchronous); the real fix is an async `AppChannel::exchange`.
- A call_plugin future dropped mid-flight still trips the owners' Drop asserts (inherited).
- If a call leaves a retained refusal Home, `close_step` on the origin never reaches homes==0 and `drive_origin` ends with a typed budget fault after 65536 turns.
- Old `routers` result debt "JOB_KIND_* duplicated" is closed.

STATUS: FIXED-verified W-shellreds-1/3/5 (dock, host-base native+wasm32-unknown-unknown, reactor-app, routers, post-turn-relay, abi, guest-contract); node host, mcp channel and reactor-jobs EDITED-unverified behind plugin-program (custody), time-travel, mcp-port reds
