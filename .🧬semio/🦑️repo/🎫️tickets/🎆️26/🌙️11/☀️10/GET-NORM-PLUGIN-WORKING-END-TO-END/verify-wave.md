# Verify wave

## Status
- instance-jobs: check --all-targets 0 errors; test 1 passed, 0 failed.
- instance-docs: check --lib --tests 0 errors (was 51 lib + 7 test errors). `--all-targets` blocked by semio-plugin-host-configuration (not mine): job/emission/host:127:270, 128:32 E0502 owners.encoding/owners.writer; its lib needs semio_framework_trace (host-configuration src :77:120, :77:409). Docs tests not run.
- instance-render: 13 errors left (was 15).
  - reserved-jobs:206,235,237: `A::build_reserved_tool_job` now takes (request-slot, body-slot, &mut OriginalConstructionAdmission, &mut StepContext); the async render builder has no cx. Needs a design decision (decl-app/tool-machine owner).
  - projections:19,66,107,136,151: `refresh_cache` unsatisfied `X: InstanceExtension` bound (plugin-extension owner).
  - tool-machine:62 `tool_machines.now_ms` field removed; :171:470, :297:225 remaining.
- instance-app, surface-fixture, semio-plugin facade: blocked by instance-render. time-travel tests: blocked by host-configuration. tool-run tests: blocked by job-receiving transaction:72:317, 74:210 (`advance_demands` private).
- Not run: infinite-board-normal-fill, editor workspaces (raster, writer, text-splice, window-transient).

## Fixes
- jobs `🏷️reserved`, `🧱️components/🏛️declarations/🌱️capture`; render `🛠️tool-machine`, `🏷️reserved-jobs`: `Ok::<_,OriginalTurnPrimary>` annotation on retained-work outcomes.
- docs: quoted typed-owner closes -> `close_original_effect_step` (instance-root, checkpoint); checkpoint ticket close via `plugin_controlled_effect_close`, receiving via `with_native_receiving` out-slot; lease/render lease and refusal closes on effect; maintenance-ladder archive close via `with_original_effect`, u128 work narrowed; ingress transient root demands/step signatures; media-download ManuallyDrop import; document crate now re-exports `envelope_decode_replacement`; ladder tests use `with_original_effect`.
