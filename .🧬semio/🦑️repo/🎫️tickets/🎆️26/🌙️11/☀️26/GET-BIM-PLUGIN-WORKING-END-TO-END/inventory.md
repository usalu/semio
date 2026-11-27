# Inventory: "Unknown current repository Cargo package" for standalone bridges

## Root cause
All 34 `✏️s/🔌️plugins/*/🏭️bridge/Cargo.toml` are standalone `[workspace]`+`[package]` manifests without `workspace.metadata.semio.repository`, deliberately excluded from parent workspaces (`**/🏭️bridge/**`; dependency-truth tests treat bridges as not owned). `cargoWorkspacePreparationInvocationV1` (`🗂️workspaces/🦀️cargo/🟦️.ts`) pushed the cwd manifest's package name as an implicit `-p` and resolved it through the repository inventory (`cargoPackageSelectionsNearV1`), which cannot list unadmitted workspaces, so it threw an internal error. The explicit `--manifest-path` form skipped that lookup and then returned (unadmitted owner: no preparation), so the lookup was pure overhead.

## Decision
Framework fix, no per-bridge metadata: the inventory schema covers root packages only for admitted owners (opt-in via `member-manifests=["Cargo.toml"]`); admitting 34 bridges would contradict the intentional exclusion. Fix: an implicit cwd package is treated exactly like an explicit manifest (no inventory lookup); only user-named `-p` names go through the inventory. `-p <unadmitted>` still refuses with the clear message "Unknown current repository Cargo package: <name>".

## Files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` (3-line change in `cargoWorkspacePreparationInvocationV1`)
- `.../🗂️workspaces/🦀️cargo/🧪️tests/🪶️lock-only/🟦️.ts` (new test: standalone bridge cwd -> no preparation; `-p` unadmitted -> refusal; admitted member cwd -> preparation)

## Verification
- `repo test-file` lock-only 3/3, declared-selection 8/8, nested-owners 4/4, preparation 3/3 pass.
- From the bim bridge dir: `repo cargo --store=shared --configuration=prototype -- metadata --no-deps --format-version 1 --offline` now succeeds (lists semio-bim-mutation-bridge). No build run.
