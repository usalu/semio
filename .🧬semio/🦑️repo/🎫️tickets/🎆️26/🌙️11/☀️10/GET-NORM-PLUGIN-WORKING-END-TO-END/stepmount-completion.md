# Stepped mount completion

## Status
- `semio-plugin-instance-dispatch` lib: compiles (`🗑️generated/stepmount-dispatch-2.log`, 0 errors).
- `--all-targets` and all downstream checks: BLOCKED by other agents' crates (`semio-plugin-time-travel-turns` 22 errors; `semio-plugin-instance-typed-ops` 55 errors, plus its call site of `start_typed_command_operation`). Nothing downstream was compiled or run.

## 1. Owned child emits
- The stepped mount for emits is `dispatch_original_emission` (OriginalEmissionInput carries owned child emits, resident admission, pending until receipt). `dispatch_emit` stays the immediate path without `cx`; with owned children it refuses by naming that stepped mount (no cx added: callers of plain lanes are ~30 tests).
- Tests `direct_owned_child_dispatch_*` (owned-child-dispatch) re-driven through a new `mount_owned_emit` helper (stepped turns on `OriginalFixtureScope`); the refused-authority case now asserts a typed refusal and that the resident admission still holds the original owned output (new `ToolOperations::test_original_admission_emit_observation`).
- Test hooks gated `any(test, feature = "artifact-app-testing")`; `semio-plugin-instance-app` feature now forwards to dispatch and jobs.

## 2. Deleted admission tests
- Re-expressed in `owned-child-dispatch/admission`: `framework_reserved_emit_busy_completion_keeps_original_fixed_owner` and `reserved_mounted_source_survives_denied_first_birth_and_reaches_the_actual_worker_candidate`, on `DispatchCtx::test_mount_reserved_cut` (stepped clipboard `cut` mount). Unrun. The busy-lock/terminal-fault variant is not reproducible (the stepped `admit_original_reserved` refuses a faulted permit); the first-birth assertion (`session_rejected`) may need adjusting to observed behaviour once it runs.
- TS anchors in both `🟦️.ts` files re-pointed to the stepped mount symbols.

## 3. Typed mount
- `start_typed_command_operation(&mut self, pending: &mut PendingLatestWinsCommand<A>, cx) -> Result<bool, Fault>`: child content admitted first and kept resident in the pending owner (`children`, `children_generation`, `children_retirement`, with paid close in `PendingLatestWinsCommand::close_step`), then gesture funded, roots captured, worker mounted in one turn. Unfunded turns return `Ok(false)` with all sources intact. Name kept to avoid breaking 5 TS anchor tests.
- REQUIRED by the typed-ops owner: call site in `advance_latest_wins_command_one` must call `start_typed_command_operation(&mut pending, cx)`, not pop `latest_wins_order` / reinsert until it returns `true`.

## 4. Not done
- Compiling `consume_media_refused_then_funded`, surface-fixture test, stale no-`cx` calls (layout, cad, parts 5/6), and all requested downstream checks: blocked as above.
