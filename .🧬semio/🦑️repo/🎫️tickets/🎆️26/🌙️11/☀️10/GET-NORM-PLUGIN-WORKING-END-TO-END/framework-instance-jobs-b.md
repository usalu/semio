# instance-jobs part B (typed-operations, command-routes, close-ladder, render)

## Edits (under instance/jobs/staging)
- typed-operations: `close_configuration_failure` now runs through `original_lifecycle_effect_step` (completion `step(effect)`, controlled owners `plugin_settle_poll(step(body,effect))`, failure place `close_step(body,effect)`). `advance_latest_wins_admission_unit` takes `cx` and runs the new effect-shaped `advance_latest_wins_admission_effect` inside `cx.with_original_effect` (registry advance/begin, pending close_step/close_key_step, take_key take the effect); the unused `advance_latest_wins_keys` was removed. Caller in instance/typed-operations inline-interaction file passes `cx`.
- command-routes: completion close via `close_original_effect_step`; lease close via `retirement_demands` + `close_original_effect_step`.
- close-ladder: `close_latest_wins_command_step` closes through `plugin_effect_close`.
- render: `work-tests.rs` wraps the Arc in `ArcLease::new`.

## Verification
Check ijobs-b-2 stopped upstream: `semio-plugin-decl-surface` (`editor-app:98`, `viewer-app:163`) has E0050 on `build_media_export_job` (3 params vs trait), another agent's area. instance-jobs was not reached, so my files and tests are unverified; rerun once decl-surface compiles.
