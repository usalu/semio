# P8a Interactive-Job Classification Schema

## Outcome

The manifest schema now carries an explicit `InteractiveJobClassification` on every action and
command execution contract:

- `Unclassified`
- `Migrated`
- `BatchOnlyPendingRewrite`
- `ForbiddenFromUi`
- `Deleted`

The serialized-field default is deliberately `Unclassified`. Definitions created through the
owned `ActionDefinition` and `CommandDefinition` constructors are marked `Migrated`; missing fields
decoded from stored or generated manifests therefore remain distinguishable and release-blocking.

`validate_interactive_job_classification` checks complete action/command iterators, returns every
unclassified `(owner, id)` in deterministic order, and never silently upgrades a decoded omission.
MCP-native job definitions that replace their execution semantics wholesale explicitly preserve
the `Migrated` disposition.

## Verification

A permanent manifest unit test covers constructor classification, a mixed action/command catalog,
and deterministic rejection of an unclassified mode command. Rust formatting and scoped
`git diff --check` are clean. The focused Nx manifest gate remains pending behind the active shared
stdio compiler-repair wave; no test pass is claimed yet.

## Remaining P8 work

- Wire validation into app/plugin release catalog construction.
- Reject batch-only, forbidden, deleted, and unclassified declarations at UI dispatch.
- Route the generic action/command path through the universal `InteractiveJob` factory/driver.
- Classify and migrate every non-generic import/export and expensive command from the Phase-0
  inventory; prove zero direct UI-reachable callbacks.

The plugin component is owned by the active stdio compiler-wall agent. That owner has been given
the catalog-validation integration packet to avoid concurrent edits to the same file.
