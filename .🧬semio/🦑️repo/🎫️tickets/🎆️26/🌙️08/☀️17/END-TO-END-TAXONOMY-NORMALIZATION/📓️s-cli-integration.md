# CLI and Consumer Integration

## Scope

- Integrated explicit `clean taxonomy inventory|plan|apply|verify` routing without changing bare `clean`.
- Registered the four Nx targets and launch seed entries.
- Refactored root policy consumers onto taxonomy v7 kind, contract, and location identifiers.
- Refactored the test Nx plugin, test-domain production consumer, and plugin-registry consumer onto the v7 contract.
- Preserved the user's intentional staged deletion of `compose/`; no restoration, traversal, rewrite, or Git-state mutation is part of this slice.

## Checks

- `NX_VERBOSE_LOGGING=true bun nx show projects`: passed after the test Nx plugin refactor.
- Scoped `clean taxonomy inventory`: executed successfully and emitted deterministic canonical JSON.
- `bun nx run @semio-tech/plugin-registry:generate`: passed; refreshed 59 plugin crates, 60 playgrounds, 38 framework packages, and `.vscode/launch.json`.
- The following registry check reached the enforced clean-area taxonomy audit and reported the expected pre-migration kind-only leaf violations; generated-output staleness was cleared.
- Direct test-domain module import: passed.
- Standalone TypeScript check reached unrelated pre-existing cross-module type failures and reported none in the edited test-domain file.
- Focused repo-library lint reached unrelated import-meta and rootDir failures outside this ticket slice.

## Git-State Rule

The apply engine must not write the Git index. Convergence uses read-only cached-plus-untracked discovery and filesystem existence checks. The user-owned Compose deletion remains staged and untouched.

## Intentional Compose Deletion Integration

The root Bun, Cargo, and Go workspace manifests still referenced packages beneath the intentionally deleted `compose/` tree. Those stale membership rows were removed from `package.json`, `Cargo.toml`, and `go.work`. `bun install --lockfile-only` then passed and refreshed `bun.lock`, removing the deleted Compose workspaces and their no-longer-reachable package graph while adding the test-only `fast-glob` workspace dependency.

Dead Compose-only root package scripts, Nx Storybook targets, package override, and 21 launch-seed configurations were removed. The registry generator passed again and regenerated `.vscode/launch.json`; a subsequent Bun lock refresh and Nx project-graph load both passed.

`cargo metadata --no-deps --format-version 1` and `go work edit -json` also passed after removing the stale Compose members.
