# Framework batch C (misc crates), lib migration

Check: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate> --lib` from 🧰️framework. Logs: 🗑️generated/batch-c-r*.log.

| Crate | Lib status | Change |
|---|---|---|
| semio-framework-job-conformance | 0 errors (r3) | `TortureJob::step` split into trait `step` (claim) plus inherent `step_original`; `settle` helper consumes `RetainedWorkOutcome` work and progress; payload closes via `with_original_effect` |
| semio-framework-graph-layout-run | 0 errors | `publication.close_step(effect)` via `with_original_effect` (2 sites) |
| semio-ui-wgpu-hosting | 0 errors | clipboard fault close via `close_original_effect_step` wrapping `RetainedWorkPoll::Complete` |
| semio-ui-backend-vulkan | 0 errors | none needed (gated `read_back`/`debug_force_device_loss` only conflicted under all-targets feature unification) |
| semio-framework-3d-mesh-journal | 0 errors | needed `finish_with_progress` public in the mesh surface module |
| semio-plugin-host-wake | 0 errors (finished) | `GuestRelayDriverPolicy::operation_caps` (orphan impl) became `GuestRelayOperationCaps::project(policy,..)`; `RetainedCloneProgress` import; bootstrap fns `pub(in super::super)`; history reserve/release migrated to effects (`with_original_effect`) |
| semio-framework-io-control-testing | 0 errors (r6) | retire driven with a local `OriginalWorkAuthority` + `CloseWork` recipient and local `with_effect`; added control-contract dependency |

## Shared-module / out-of-scope touches
- 🧊️3d/🥽️mesh/🎨️surface/🦀️.rs: `finish_with_progress` pub(super) -> pub.
- Callers of the removed `operation_caps`: 🌉️mcp/🌱️process-arena/🦀️.rs and 🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/💡️inference-route-and-actions/🦀️.rs now call `GuestRelayOperationCaps::project`.
- `OriginalWorkActionIssuer::loan` and `NativeSnapshotEncodeEffectQuote` were not needed in these crates.
- job-conformance check was blocked once by native-schema (private `NativeSourceKey`, another agent's crate); later passed.
