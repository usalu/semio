# routers lane result (11/10)

Package: `semio-plugin-host-routers` (plugin/host/routers). Native lib `W-routers-2` rc 0 (was 12 errors). Shell native check: see Verification.

## Design deviation (evidence-based)
The coordinator design (router owns a `PluginInstanceHandle`, serialized by a command queue, each call supplying a driver to `handle.run`) cannot work: every lease in a handle is bound to its admitting Source. `OriginalNativeLease::parent` (job/receiving/source/native) refuses a `StepContext` of another Source ("native loan requires its genuine admitting Source"), and `NativeInstanceHome` says "the admitting Source owns the real instance". A handle therefore lives and dies inside one operation; a router-held handle driven by a later per-call driver would be refused at the first step. The cold-job API is `GuestRelayOperationAdmission::new(driver, actor, origin)` -> `admit` -> `GuestRelayOperation::run_bytes_from` -> `close`.

Implemented instead (same intent: exclusive ownership, per-call driver and output slot, Pending keeps state, nothing blocks):
- Routers keep a `Copy` `PluginRegistration { actor, origin: GuestRelayNativeOrigin }` per loaded plugin (no shared `Arc`, no mutable instance shared). Each call birth its own instance inside its own operation Source, so concurrent callers need no queue or lock across await (instances are never shared).
- `call_plugin(&registration, kind, input_bytes, driver) -> Result<Vec<u8>, PluginHostError>` (new `📮️plugin-call`): admits the operation, runs the kind via `run_bytes_from(GuestRelayBorrowedBytes)`, receives the result inside paid `dispatch_operation_step` turns (`Receiving`: reserve charged as retained capacity, copy charged per span as `copied_bytes`+fuel, Pending = state kept, `yield_once` and re-poll), then `operation.close()` returns result, handle and admission to the Source. Cancellation travels through the driver's `cancelled` closure; dropping a mid-flight call future would trip the owners' Drop asserts (same property as `GuestRelayOperation`).
- Failures: `retire_refusal` (new, post-turn-relay `GuestRelayOperationAdmission` and `GuestRelayOperation`, shared `retire_failure`) captures the original `GuestRelayAdmissionFailure` into a `GuestRelayFaultHome` under the owner's own driver, reads the cause in a paid callback (message copy is truncated to the paid grant) and retires it on its Source. Guest `Failure` results are read through `with_fault` / `with_guest_fault_bytes` the same paid way.
- Each call asks for its own driver through `PluginCallDrivers = &dyn Fn() -> Result<GuestRelayWakeDriver, ValueError>` so chained hops (`run_io`), identify fan-out and inference dependency recursion each get a fresh operation Source. The `cancel` / `funding` parameters of `infer` are gone (cancel lives in the driver).
- Old five methods map to kinds: `io_run` -> `semio.io-run`, `io_sniff` -> `semio.io-sniff` (first byte = rank), `infer` -> `semio.infer`, `mutation_plan` -> `semio.mutation-plan`, `compose` -> `semio.compose`. Inputs are the same JSON the old handle built (`IoRunInputWire`, `ComposeInput`, raw request bytes), passed as the job's byte source. Constants `JOB_KIND_*` live in routers (the reactor guest crate owns the originals, host cannot depend on it; DEBT: move to a shared schema crate).

## Edits
- routers: `🦀️.rs` (mod `plugin_call`, exports, dropped `GuestRelayWakeAuthority`/`PluginInstanceHandle` imports), new `📮️plugin-call/🦀️.rs`, `🚪️io-router` (registration instead of `Arc<handle>`, `compose/run_io/identify` take `drivers`, `io_job_input` helper), `🎯️mutation-router` (`plan(request, drivers)`), `💡️inference-router` (`infer(request, drivers)`).
- post-turn-relay: root re-exports `RuntimeActorId`, `GuestRelayWakeDriver`, `GuestRelayAdmissionFailure`; `GuestRelayOperationAdmission::retire_refusal`, `GuestRelayOperation::retire_refusal`, `retire_failure` (operation-admission.rs).
- host package root: dropped dead `GuestRelayWakeAuthority` re-export (type is not exported by post-turn-relay since the mounted registry left), added `GuestRelayOperation*`, `call_plugin`, `PluginCallDrivers`, `PluginRegistration`.
- No Cargo.toml edits, no LOCK NEEDED.

## Callers (not ported, owners needed)
- `🏃️run/🧠️wasmtime-node-host/🦀️.rs` (~l.505-575): still builds `Arc<PluginInstanceHandle::new(actor, runtime, instance, input, GuestRelayWakeAuthority{..})>` (constructor gone) and registers the same handle in three routers; must build one `PluginRegistration { actor, origin }` (origin = `GuestRelayNativeOrigin { cache_identity, package_hash, acquire, resolve, release }` from the compiled cache) and pass `Copy` clones; `unload_plugin` calls are unchanged (no retirement needed any more).
- `🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/💡️inference-route-and-actions`: `ensure_inference_route` / `infer_real` still use `PluginInstanceHandle::new`, `GuestRelayWakeAuthority`, `router.infer(request, cancel, funding)` under `block_on` (sync-over-reply). Port: register a `PluginRegistration`, call `router.infer(&bytes, &|| caps.issue(requested, clock, cancel_closure))`, await instead of `block_on`.
- host tests (`🧪️tests/🔬️unit`, `artifact-inference-router`, `artifact-mutation-router`, `guest-cold-relay/*`) still use the removed handle methods / `test_relay_wake_authority`; `--tests` for routers remains red (not built here).

## host-shard `with_identity`: no clean non-blocking shape without a framework change (documented, not implemented)
`with_identity` lends `NativeAllocationAuthority::with_send_loan(&mut (), |loan| ..)`: the loan, and the `EntityIdentityAuthority` resumed from it, exist only inside that synchronous closure; hence the guest call future is driven by `block_on` inside it. Options checked:
1. Complete inside one synchronous turn: `GuestRuntime::{admit_actor_context, checkpoint, restore, execute_turn}` are `async` wasmtime calls that may genuinely suspend (epoch yield, guest cold work), so they cannot be required to finish in one poll.
2. Re-issue the loan per resume: the future holds `&mut EntityIdentityAuthority<'loan>` for its whole life; re-lending per poll would need the identity to be rebuilt between polls while the future still borrows it (self-referential). `pause_forwarded`/`resume_forwarded` only work between synchronous hops, not under a live borrow.
3. Needed: the guest runtime (and the wasmtime store data) must take a re-lendable identity provider (`&dyn IdentityLoanProvider` or an owned long loan that calls `with_send_loan` on each host import) instead of `&mut EntityIdentityAuthority`. This is a guest-runtime + framework-value API change, tracked as the existing DEBT in 1108-result-hostwake.md. Until then `block_on` is confined to the shard's own pool thread.

## Verification
- `"$C/1105b-tools/chk.sh" W-routers-2 Cargo.toml native -p semio-plugin-host-routers` rc 0 (routers and post-turn-relay checked).
- Shell: `chk.sh W-routers-3 <shell-session Cargo.toml> native -p semio-renderer-wgpu-shell-session` rc 1; host-routers, host-wake, host-shard, post-turn-relay, document-media, retained-command, scrub-host, typing-edit are no longer red.

## Remaining reds in W-routers-3 (shell native) and owners
| crate | errors | owner / cause |
|---|---|---|
| semio-plugin-publication-mounted (receipt/payload, receipt) | 6 | factoryticket / pubmedia: `RetainedWorkOutcome` has no `map_err`, 3-arg call at receipt:356 |
| semio-plugin-command (`🎟️typed/📨️wire` l.51 `use of moved value p`) | 1 | command lane (claimed 09:00) |
| semio-plugin-reactor-app (`🦀️.rs:112` call takes 6 args, 4 given) | 1 | unclaimed (reactor lane / guestrt) |
| semio-renderer-wgpu-host-base (`📥️lifted/🔁️asset-decode-jobs`: missing `semio_framework_value` dep, 4 type mismatches) | 7 | unclaimed (wgpu renderer; the missing crate import may need a LOCK NEEDED Cargo.toml dep) |
| semio-renderer-wgpu-dock (private `DockAxisSlots`, `StackCapLayout`, `StackCapGroupLayout`) | 18 | unclaimed (wgpu dock element visibility) |

## Open items / debts
- Callers above; tests of routers/post-turn-relay; `JOB_KIND_*` constants duplicated from the reactor guest crate.
- Refusal cause text is truncated to the paid grant.
- A mid-flight dropped `call_plugin` future asserts in the owners' Drop (inherited contract).

STATUS: FIXED-verified W-routers-2 (host-routers native rc0); shell native W-routers-3 no longer red in host-routers, remaining reds: publication-mounted 6, command 1, reactor-app 1, wgpu-host-base 7, wgpu-dock 18; callers wasmtime-node-host and mcp plugin-artifact-channel not ported
