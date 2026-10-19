# Result: owners-gate (build-owner registry consistency law)

Paths below are relative to `LIB = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Changes
New (permanent law, language-agnostic, schema/fixture-driven):
- `LIB/🏗️build/🧬️schema/🧾️owner-consistency.json`: schema of the fixture cases and the finding kinds.
- `LIB/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts`: core, next to the reader `🚪️admission` (`readBuildOwnerRegistryV1`). Pure `findOwnerFindings` and `syncOwnerRegistry` over an `OwnerTree` port (in-memory tree for fixtures, git-backed tree on disk with a directory-walk fallback outside git), plus `checkBuildOwnerRegistry` and `reconcileBuildOwnerRegistry` (re-reads the file at call time, scope-aware, validates through `readBuildOwnerRegistryV1`, atomic write).
- `LIB/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🔣️.json` + `🟦️.ts`: 8 fixture cases (stale rows, unregistered project.json modules, Cargo rows shadowing a project.json, moved module, renamed project, ignored generated rows, scope, root module and example collections, native roots, catalog providers) and 11 bun tests.
- `LIB/⚖️laws/📋️build-owner-registry/🟦️.ts`: `policyBuildOwnerRegistryBreaches` (BreachRecord adapter).
Wiring (same mechanism as the other root verify laws: `verify <segment>` route plus root target):
- `LIB/🎛️commands/✅️verify/🟦️.ts`: route `verify build-owners`.
- `LIB/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts`: `runBuildOwners` (`verify build-owners` checks, `verify build-owners sync` repairs then fails only on authored findings).
- `LIB/🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts`: `runGate` (so bare `verify` / `workspace:verify`) now runs the registry law after the language-purity step.
- `📋️project.json` (root): added targets `verify-build-owners` (cached, `outputs: []`) and `verify-build-owners-sync` (uncached). Only these two targets were added; no other project.json was touched.
Writers now keep rows in sync (all three use `reconcileBuildOwnerRegistry` with a scope, instead of the Cargo-only row inserter):
- `LIB/🏗️authoring/📦️crate-tree/🟦️.ts` `registerBuildOwnerRows` (carve; scope = artifact dir; backup lists now also carry `relocated`).
- `LIB/🏗️authoring/📦️crate-tree/🧱️scaffold/🟦️.ts` (scope = subset dir) and `LIB/🏗️authoring/🧬️mutation-tree/🟦️.ts` (scope = new crate dir).
- `LIB/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts`: removed `syncBuildOwnerRows`, `insertBuildOwnerRows`, `BuildOwnerRow`, `BuildOwnerSync` (no compat layer).
- `LIB/🧪️tests/📦️crate-tree/🔒️workspace-registration/🟦️.ts`: the two old row tests replaced by one `registerBuildOwnerRows` test (add, rename at same root, drop, out-of-scope rows untouched, backup files, idempotent).

What the sync handles now (so the law does not fire on normal work): rows of vanished modules dropped; a moved module (same declared name, new root) relocates its row including native roots; a renamed project/crate at the same root renames its row (re-sorted by byte order); a Cargo row beside a `📋️project.json` hands over to the project manifest; package lists follow `package.json`; every unregistered `📋️project.json` module and Cargo package gets a row at its byte-order position. Rows whose manifest sits under a git-ignored path (generated crates such as the puzzle-2d graph manifest, open item 3 of the previous report) are exempt from the existence checks, so a fresh clone does not fire the law. Catalog providers and native roots stay authored (reported, not synced).

## Verification
- `bun test ./LIB/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts ./LIB/🧪️tests/📦️crate-tree/🔒️workspace-registration/🟦️.ts ./LIB/🏗️build/🕸️graph/🚪️admission/🧪️tests/🟦️.ts`: 20 pass, 0 fail (new law 11, crate-tree registration 7, admission 2). Note `bun test` needs the `./` prefix; without it the filter matches nothing.
- Third-party cross-checks inside the tests: Ajv 2020 validates the fixture against its schema and every synced registry against `📋️owners.json`; `Bun.TOML.parse` agrees with our `cargoPackageName` on every fixture Cargo manifest; sync is idempotent (second sync is a no-op) and leaves no residual findings except the declared authored ones.
- `SEMIO_TEST_ARTIFACT_DIR=<scratch> bun test ./LIB/🧪️tests/🧬️mutation-family/🟦️.ts`: 3 pass (exercises the mutation-tree writer path). Without that env var the file fails with its own precondition message (not caused by this change).
- Real registry: `checkBuildOwnerRegistry(repo)`: 3674 owners, 0 findings (about 60 s, dominated by `git ls-files -o`). `reconcileBuildOwnerRegistry(repo, {write:true})` (what `verify build-owners sync` runs): changed=false, removed/added/renamed/relocated/repackaged all empty, file byte-identical afterwards (`cmp`). The `trinity-jack-lsp` row had already been removed by a peer before my run (only `trinity-jack-lsp-worker` remains), so nothing was left to fix.
- Lane runtime (harness calling `runBuildOwners` on the lane prototype, because the CLI route is blocked, see below): `['bogus']` throws "Expected verify build-owners [sync]"; `[]` prints `passed.`.
- `tsc --noEmit --strict` (repo flags) on the core, test, law, crate-tree, scaffold, mutation-tree and test files: clean for my files; on `verify/🟦️.ts` plus gate-lanes only unrelated pre-existing errors in `⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts`.
- `bun build --target=bun 📜️script.ts`: bundled 2481 modules, no resolution errors.
- Not run: `bun ./📜️script.ts verify build-owners [sync]` and `bun repo run workspace:verify-build-owners` through the real CLI. Both fail before reaching the route because `dispatchOwnedScriptRoute` throws `Invalid owned command .../✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json:verify-combination-stress` (a peer's `metadata.semio.workspaceCommand` declaration in the fem TypeScript project.json). That breaks every `verify ...` command at the moment, independent of this change; the fem owner must fix the declaration.

## Open items
1. The fem project.json workspaceCommand declaration above blocks all `verify` CLI routes until fixed.
2. Launch registration for `verify-build-owners` and `verify-build-owners-sync` is not added: `.vscode/🧩️launch.seed.jsonc` is retired by the dashboard control-plane work (instruction: do not touch `.vscode/`). Add them to whatever replaces the seed.
3. The legacy ticket tool `C/owners-tools/owners-check.ts` is superseded by the product law; left in place (ticket inputs are not deleted), nothing imports it.
4. The directory move/rename tooling (`🚚️move`, taxonomy normalization apply) still does not call the reconciler: a directory rename is caught by the law (and fixed by `verify-build-owners-sync`), not repaired automatically. Hooking `reconcileBuildOwnerRegistry` after a taxonomy apply (scope = destination paths) is the follow-up.
5. The row order in `📋️build-owners.json` is not strictly byte-sorted (25 existing pairs); the sync only inserts and re-sorts rows it touches and the law does not check order.
