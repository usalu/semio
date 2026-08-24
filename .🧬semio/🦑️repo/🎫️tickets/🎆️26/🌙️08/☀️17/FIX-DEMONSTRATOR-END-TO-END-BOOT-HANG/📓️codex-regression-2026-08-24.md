# Demonstrator Regression Continuation

## Goal

Restore the `mit-bestand` demonstrator end to end and verify every included app at runtime.

## Red Baseline

`bun nx run-many -t test-quick -p @semio-tech/framework-job-rs,@semio-tech/ui-contract-rs --skipNxCache --outputStyle=stream` failed on 2026-08-24.

The production-library blockers reproduced from the supplied log are:

- `semio-framework-job`: five `ManuallyDrop<Option<_>>` assignment mismatches and one rejected-page borrow-lifetime conflict.
- `semio-framework-ui-contract`: missing `SurfaceDoc` clone/equality support, a mutable/shared callback mismatch, and two iterator-temporary borrow conflicts.

The focused UI test build also exposes stale tests that still construct the former unbounded `UiText`, `UiFixedBytes`, binding-list, and builder APIs. These must be updated to the bounded contract before the target can become green.

## Verification Route

1. Make the two focused Nx test targets green.
2. Run the demonstrator Nx build from the registered launch path.
3. Start the registered demonstrator dev target on port `6029`.
4. Exercise all configured panes/apps and record browser/runtime evidence, including worker boot failures and console errors.

## Focused Fixes

- The job payload writer now preserves exact rejected-page ownership across `ManuallyDrop`, separates fallible reservation from grant creation, and retires pre-admitted faults only after close begins.
- The UI contract's bounded owners now use pre-sized heap slices instead of recursively nested multi-megabyte stack values. The arena itself is likewise heap-backed while retaining fixed capacities.
- Bounded lists, maps, bytes, and recursive UI values again admit and serialize populated schema fixtures with exact quota rejection.
- Retained patch removal now runs only after the candidate snapshot has been cloned and advances one subtree owner per opportunity.
- Text edit admission immediately returns byte credits after a failed single-page admission, and persistent rope edits reuse unaffected subtrees instead of scanning multi-megabyte roots.
- Stale tests were ported to the bounded constructors and ownership APIs; the generated TypeScript contract mirror was refreshed through its Nx target.

## Focused Green Baseline

- `@semio-tech/framework-job-rs:test-quick`: 11/11 passed.
- `@semio-tech/ui-contract-rs:test-quick`: 88/88 passed.
- `@semio-tech/ui-contract-rs:conformance`: 6/6 passed.
- `@semio-tech/ui-contract-rs:check`: generated schema mirror fresh.
- `@semio-tech/ui-contract-rs:check-wasm`: `wasm32-wasip2`, `wasm32-unknown-unknown`, and type-generation feature checks passed.

## Demonstrator Build Integration

- Updated the OS store artifact-fault path to use the bounded artifact envelope payload and the current interactive-job close lifecycle.
- Restored parsed-document text equality support and added coverage for closing an interactive job through the OS store.
- Updated the actor bridge to project the current bounded UI values and retain a payload page for each projected turn.
- Updated the World3D host freshness comparison to use a deterministic flat lexicographic comparison compatible with the current TypeScript target.

## UI Runtime Migration

- Replaced ordinary command cloning with explicit credited clones and reduced gateway tracking to command tickets.
- Migrated runtime text, children, operations, bindings, and state handling to the bounded UI contract types.
- Changed fixed-list and built-children backing storage to lazy or boxed slices so empty/default structures no longer overflow ordinary thread stacks.
- Boxed the runtime fixed-vector, semantic traversal, and retirement stacks and added size regressions for the reconciler, cursor, retained map, tree node, and UI node record.
- Raised the page capacity to 32 KiB so the bounded base node fits the page accounting invariant.
- Updated stale runtime tests to exercise the current bounded and credited APIs, including dynamic page-fault progression and bounded wide-tree traversal.

## UI Runtime Green Gates

- `@semio-tech/ui-runtime-rs:test-quick`: 81/81 passed on the default stack.
- `@semio-tech/ui-runtime-rs:check-wasm`: both Wasm targets passed.

## WGPU Integration

- Removed the obsolete toggle callback argument from the borrowed window-measure adapter.
- Separated retained-document key discovery from removal so incremental close no longer overlaps immutable and mutable table borrows.
- Added a regression proving retained-document close retires exactly one record per step and then completes.
- `@semio-tech/ui-rs:test-quick`: 145/145 passed.

## Fresh Plugin Build Integration

- Made the kernel activation event cloneable so manifest descriptors, activation decisions, and lifecycle events can retain it.
- Added a regression for retained activation-event ownership.
- Moved the private command-batch descriptor destructor assertion into its owning channel module and kept the public kernel test focused on exported owners.
- Updated the turn-patch transport test to unwrap without imposing an unrelated `Debug` requirement on the lease owner.
- Updated the action-bus wire-dispatch regression to observe the bounded payload page release before terminal completion.
- `@semio-tech/framework-rs:test-quick`: 191/191 Rust tests and 87/87 TypeScript tests passed.
