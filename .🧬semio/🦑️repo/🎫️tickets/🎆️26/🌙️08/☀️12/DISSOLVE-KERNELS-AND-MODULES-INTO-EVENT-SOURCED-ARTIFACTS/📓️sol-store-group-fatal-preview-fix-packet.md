# Store Group Fatal Preview Fix Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Store SHA-256, stable across two coordinator samples: `24db6c9cd31c40e80dcc2a649c7f53a7aaebef4eb117b346ac2f71d01b8f6015`.
- The store is an externally authored dirty migration surface: 74 additions and 98 deletions. Preserve all of it. The current diff migrates `Mutation::diff` to `MutationOutcome`; it deliberately removed the old `Mutation::validate` fixture but left the group atomicity test expecting the old negative-op rejection.

## Diagnosis

`TransactionCoordinator::dispatch_relation_group` still performs a phase-one, no-side-effect preview through `SpaceMember::validate_wire`. The migrated `validate_wire` calls `apply_mutation` but discards its `MutationMessage`s. The test's `ValidatedMutation` now always emits a successful diff, so its negative operation is applied during preview and phase two succeeds. The failing assertion is therefore a real lost atomicity check, not a Pack, DAG-layout, or deleted-module failure.

## Focused Fix

Writable paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- one unique Terra acceptance Markdown in this ticket

Required implementation:

1. In the existing `SpaceMember::validate_wire` implementation, retain the dry-run state threading but inspect each returned message list before accepting the next state. A `Fatal` outcome is rejected with its deterministic message before any real member dispatch. Do not introduce an authority policy, change public signatures, or implement the broader cross-ticket merge/conflict lane.
2. Make `ValidatedMutation::SetN { n: -1 }` emit `MutationOutcome::fatal("mutation.invariant", ...)` with a stable target and default diff; non-negative values retain the existing successful diff.
3. Update only directly stale local prose. Keep the atomicity assertion unchanged: one fatal child operation must leave both parent and child histories empty.

Do not edit SPR command/wire/conflict, VCS, Cargo, registries, Pack, DAG, renderer, stdio, or unrelated store migration content.

## Verification

Run the focused store test first through the owner package's established script surface if supported; otherwise run the smallest Cargo test command from its `📜️script.ts` contract and record the exact exception. Then run:

```text
bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache
bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache
```

Require the prior atomicity test to pass, active stale validation prose to be coherent, current-hash preservation of unrelated dirty hunks, and scoped ordinary/cached diff checks.
