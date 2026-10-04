# Merge Conflict Resolution

Stash pop of `Updated upstream` (index stage 2) onto `Stashed changes` (index stage 3). Nine unmerged paths. Every configuration name, oracle id, Cargo package, and bun workspace key present on either side is present in the resolution.

## Code

- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`: kept the stashed `cargoPolicy`, `testCargo`, and `captureCargo` helpers (the merged router calls them) together with upstream `seedGeneratedFile`. Generation seeds the mirror, then awaits the async export test. `seedGeneratedFile` already creates the parent directory.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`: kept upstream's hash gate and bootstrap refresh, and wired stashed `moduleSourceImports` through that same gate. The fact-cache callers below the conflict use it.
- `🧰️framework/🔨️modules/◻️2d/🧪️tests/🎚️config/🟦️.ts`: union of both `include` lists. All seven test modules exist on disk.

## Launch files

`.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, and `.claude/launch.json` are unions by configuration name (seed placeholders such as `@generated:*` stay too).

- Upstream-only and stash-only launchers are both inserted, using the other side's neighbors as anchors.
- Shared launchers keep upstream order. Stash-only `env` entries are added. Input `options` are the union of both lists.
- Two shared commands pointed at targets that no longer exist on this tree. Those now use the stashed commands, which resolve: `@semio-tech/plugin-registry:generate` (`workspace:generate` is gone) and `semio-framework-pixels:test` (`@semio-tech/pixels` has no `test-rust` target).

Counts: vscode 2462 configurations (46 upstream-only, 389 stash-only), seed 1371 (28 / 363), claude 79 (6 / 7).

## Data

- UI oracles: 10 ids. Upstream `three-damp` plus the stashed color, scale, splice, and decimal oracles.
- `Cargo.lock`: 977 packages. Includes both `semio-framework-pets` and `semio-framework-pack-error` / `semio-framework-pack-json`. Dependency lists are unions.
- `bun.lock`: 91 workspaces. Workspace and package keys from both sides are kept.

## Check

`bun build` transpiled the three source files. The JSON files parse. No conflict markers remain in the nine paths.
