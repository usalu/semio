# hostwake lane result (11/10)

Packages: `semio-plugin-host-wake` (plugin/host/wake), `semio-plugin-host-shard` (plugin/host/shard). Both were red in the wgpu shell native check (W-mediaslots-10: 9 + 13 errors).

## Edits (state: compiling, see Verification)
### host-wake
- `GuestRelayOperationCaps::from_policy(policy, ..)` replaces the orphan `impl GuestRelayDriverPolicy { operation_caps }` (the policy type lives in `semio-plugin-host-wake-policy`; no callers existed).
- `RetainedCloneProgress` import in the operation module.
- `GuestIssuedHistoryHome::reserve_step` / `close_on_source`: `reserve_one(effect)` / `release_empty_page(effect)` run inside `cx.with_original_source_effect(grant, ..)`; `Pending`/unadmitted quote = `Ok(false)` (state kept), `Denied` = `Ok(false)`, `Work` failure faults the home, allocation failure charges its actual copied bytes; the manual `cx.consume_fuel(step.work)` is gone for the two effect-shaped steps (the effect charges the calling turn).
- `apply_bootstrap_step` / `apply_bootstrap_transition_step`: visibility `pub(in super::super)` (called from the sibling `guest` module).

### host-shard
- `grant/event`: owner-less closes now run in one `OriginalRetirementPump` turn (`pumped`, fuel = the checked input's fuel, no clock deadline like the snapshot adapter): `source_close_turn(fuel, grant) -> (Result<progress>, work)`; `source_close_step` / `release_empty_cursor` / `close_mesh_in_effect` take the effect, cursors close with `close_step(body, effect)`, Event retirement admits with `admit_owned_retirement(&mut Option<Event>, effect)` (`Ok(None)` restores the exact source). Pending = no progress, state kept. The frame-turn caller reads the performed work from the turn.
- `shard-outcome-and-loop-state`: `OriginalActorFixedReceipt::write(call, escrow)` / `read -> (call, escrow)`; the shard pack carries no guest escrow pointer (empty extent written, extent dropped on read).
- `identity`: the removed `NativeEncodeAllocationPort` callback is replaced by a resident `NativeAllocationAuthority<ShardAllocationPort>` (policy-funded, `Context = ()`). `issue(policy, observer)` no longer takes an allocation box; `ShardIdentityIssueRefused` lost its `allocate` field. The short loan cannot outlive a closure, so `OriginalShardIdentityLoan` / `loan()` are replaced by `pub fn with_identity(async|identity| ..)`, which lends the loan, drives the (already ready-in-practice) guest call with `semio_framework_async::block_on` on the shard's own pool thread and returns the receipt to its unique custody. DEBT: `admit_actor_original`, `admit_owned_actor_original`, `checkpoint`, `restore`, `execute_turn` stay `async fn` but block inside; a loan that survives suspension needs a framework-level owned long loan.
- `Cargo.toml` (shard): `+semio-framework-job = { workspace = true }`. LOCK NEEDED logged.
- tests (identity): ported to `issue(policy, observer)` and `with_identity`; the "suspended future keeps the loan" case is gone by construction.
- test double (`shard-transports-and-test-doubles`): `issue(policy, observer)`.

### host-post-turn-relay (claimed as an unclaimed red of the same shell check; 13 errors)
- Owner-less step recipe (same as `reactor/.../call/input::retire_step`): `cx.with_original_effect(grant, |_, effect| owner.step(body, effect))`, `Pending` = zero-progress step with state kept, `receive_original_retained_work(cx, turn)`; ported at `output` (failure close), `unit` (input_close, failure_close), `unit/input` (payload_retirement).
- `unit/provider`: `GuestRelayOutput` imported through `crate::`.
- `GuestRelayScalarFlow::page(export, args)` and `page_extent() -> Option<(u32, usize)>` were called by the controller but never existed in any commit: added (a page flow ends `Complete` with the guest byte extent instead of an escrowed receipt).

## Verification
- Native libs: W-hostwake-3 rc0 (host-wake, host-shard), W-hostwake-6 rc0 (+ host-post-turn-relay).
- wgpu shell native (`chk.sh W-hostwake-N "<shell-session Cargo.toml>" native -p semio-renderer-wgpu-shell-session`): W-hostwake-4 and W-hostwake-7. host-wake, host-shard, host-post-turn-relay, tool-run are no longer red.

## Remaining reds in W-hostwake-7 (shell native) and owners
| crate | errors | owner |
|---|---|---|
| semio-plugin-document-media | 20 (was 29) | pubmedia (claimed 06:46) |
| semio-plugin-publication-mounted (composition/publication/group/mounted: receipt payload/receipt, full-operation mounted ui) | 6 | pubmedia |
| semio-plugin-retained-command | 1 (was 61) | pubmedia |
| semio-plugin-scrub-host (tool-machine/host) | 4 | toolmachine (tool-machine released 07:28; check claims) |
| semio-plugin-typing-edit (tool-machine/typing/edit) | 4 | toolmachine / actionbus3 area (released) |
| semio-plugin-host-routers | 12 | unclaimed, needs design (below) |

### host-routers (released, not started)
`PluginInstanceHandle` was rewritten at fbf4da6a5aa (726) to the Source-native cold-job API (`run` / `run_from` / `run_bytes_from(kind, input, &GuestRelayWakeDriver, &mut Option<GuestRelayResult>)`, `&mut self`). The routers still call the removed `Arc<PluginInstanceHandle>` methods `io_run`, `io_sniff`, `compose`, `mutation_plan`, `infer(request, cancel, funding)` and import `GuestRelayWakeAuthority` (defined in post-turn-relay `mounted-session-types`, not re-exported from the crate root). A port needs: a per-call wake driver/treasury, a `Mutex`/owned handle instead of the shared `Arc`, and Success-output bytes extraction from `GuestRelayResult`. That is a post-turn-relay API decision, not a mechanical port.

## Tests (`--tests`, W-hostwake-10 native)
- host-shard lib test (identity tests ported to `issue(policy, observer)` / `with_identity` / shared `close_recipient_step`) compiles; so do host-operation (testkit lib, new pump-funded `test_native_authority::close_recipient_step`) and the guest-runtime testkit doubles (`admit_step` / `close_step` now take a `StepBudget`).
- Still red (test-only, not run): host-wake `tests/*` and host-post-turn-relay lib test. Cause 1: both install their own `#[global_allocator]` (`semio_framework_trace::HeapWitness`) while the job `testkit` now links `semio-framework-value-testing-allocator` (its own global allocator, `observe_retirement_allocations`); fix = drop the local allocators and observe through the testing allocator (needs dev-dep + LOCK). Cause 2 (wake): `JobPayloadAuthority::admit(op, gen, grant, &mut None)` returns `RetainedWorkOutcome`; `close_step(effect)` needs a pump turn; guest `close_step(input, StepBudget)`.
- host-operation `tests/wasmtime` (3 sites) still builds identities from the removed closure allocation port; replace with `NativeAllocationAuthority` + `with_send_loan` (not built here: `-p semio-plugin-host-operation --tests` red with 3 errors).

## Open items / debts
- Shard identity: `async fn`s that block through `block_on` inside `with_identity` (see above).
- host-routers port (above) and the test-only items above.
- LOCK NEEDED (all logged): shard `Cargo.toml` (+semio-framework-job, +dev-dep host-operation testkit), host-operation `Cargo.toml` (+optional/dev semio-framework-job); no lock was needed for the native runs to pass.

## Rerun
- libs: `"$C/1105b-tools/chk.sh" W-hostwake-N Cargo.toml native -p semio-plugin-host-wake -p semio-plugin-host-shard -p semio-plugin-host-post-turn-relay`
- tests: same with `--tests`
- shell: manifest `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚶️shell-session/📦️packages/🦀️rust/Cargo.toml`, `native -p semio-renderer-wgpu-shell-session`

STATUS: FIXED-verified W-hostwake-6 (native libs host-wake, host-shard, host-post-turn-relay rc0) and shell native W-hostwake-7 (those crates green; remaining reds: document-media 20, publication-mounted 6, retained-command 1, scrub-host 4, typing-edit 4, host-routers 12); tests of wake/post-turn-relay not yet compiling (allocator conflict, see Tests)
