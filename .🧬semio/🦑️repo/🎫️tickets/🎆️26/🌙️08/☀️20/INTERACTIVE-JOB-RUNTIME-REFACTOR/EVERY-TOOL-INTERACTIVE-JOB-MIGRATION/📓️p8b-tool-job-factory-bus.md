# P8b Tool Job Factory Bus

## Outcome

The framework action bus no longer exposes the prototype `ActionHandler::handle(...) -> Vec<String>` callback path. UI tool routing is now a typed `ToolOperationSpec -> ToolJobFactory -> ToolJobDispatch<InteractiveJob>` boundary.

## Contract

- `ToolOperationSpec` owns controller id, tool id, DSL arguments, and the complete operation identity (`operation_id`, base revision, generation, preview sequence, deterministic seed).
- `ToolJobFactory` returns a concrete closed-set `InteractiveJob` type; a product that aggregates several tool jobs supplies an enum implementing the job protocol.
- Registration accepts only `InteractiveJobClassification::Migrated` factories. Unclassified, batch-only, UI-forbidden, and deleted entries are rejected before they can become UI-reachable.
- Dispatch returns explicit unknown-controller or factory errors. It never silently returns an empty callback result.
- The no-factory platform state is represented by uninhabited `NoToolJobFactories` and `NoInteractiveJobs` enums.
- Small tools use the same path by returning a job that completes in its first bounded step.

## Verification

- Unit coverage proves operation identity survives dispatch, all four non-migrated classifications are rejected, and unknown controllers return a typed error.
- `CARGO_TARGET_DIR=<ticket>/🧪️target-action-bus bun nx run @semio-tech/framework-rs:test-quick --skip-nx-cache` passed: Rust 161/161 and TypeScript 87/87. The attributable log is `📝️p8-action-bus-test-quick-r2.txt`.
- The first run reached this seam successfully and exposed ten stale awaits elsewhere in the active de-async sweep. Those exact workflow/manifest residues were removed before the green rerun; the failed inventory remains in `📝️p8-action-bus-test-quick.txt`.
