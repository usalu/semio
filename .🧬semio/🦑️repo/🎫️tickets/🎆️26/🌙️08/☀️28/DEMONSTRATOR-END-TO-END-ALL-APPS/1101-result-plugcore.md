STATUS: GREEN for the four crates (guest-contract, reactor-jobs, publication-mounted, retained-command): `chk.sh plugcore-2` rc 0, zero `error[` lines, `Finished` (wasm32-wasip2 --lib, hub manifest); log `C/🗑️generated/1101/plugcore-2.txt`, rc `plugcore-2.rc`. Tests not ported.

## Ported (framework/products/os/modules/plugin)
- publication-mounted `🧾️receipt`: `SharedUtf8::admit(..).result.map_err`.
- reactor-jobs: `💡️infer` `.result` on `close_factory_ticket` / `admit_owned_retirement`; `🏛️reserved` construction closure now returns `RetainedWorkOutcome{result:(||..)(),work:1}`.
- guest-contract `🌉️call`: `📤️failure::advance` returns `OriginalTurnFailure`; builder `advance_initialization/append_original/seal` outcomes received via `receive_original_retained_work`; `publish_failure_step/call` carry `OriginalTurnFailure`; `.result` on `close_step_granted` and publication `close_step`.
- retained-command: job `step` split into `step_original` + `InteractiveJobStep::from_callback`; `.result` on publication/fault publication/box close steps; `checkpoint/advance_publication` return `OriginalTurnFailure`; checkpoint-input close closure returns `RetainedWorkOutcome` (work 1). full-operation + emission: local `retired`/`built` wrappers (work 1, same as plugjob), `.result` on alias step, shared retirement and artifact retirement admits; frontier: `.result` on retirement/completion outcomes, fixture caller + `FIXTURE_STOCK` gated with `cfg(any(test, feature="artifact-app-testing"))` (`OriginalJobFixtureCaller` is testkit-only).
- Tests of these crates not ported (only --lib).
