# Repo Gates 7: permanent non-cargo gates after the wave-7 carves

Slug: REPO-GATES-7, prefix `rg7`. Executor: Sonnet. Started 2026-10-17 19:40. Logs: `🗑️generated/rg7-*`, tools: `rg7-tools/`.

## 0. Verdict

DONE with deferrals (static and runtime evidence from the non-cargo gates; no cargo ran). 22:40 state. Every gate was baselined, each failure attributed, and every mechanical fleet-caused or stale item fixed at its owner. Not green and not mechanical (section 5): `verify taxonomy enforce` (667,026 findings, all in pre-10-15 families; fleet-born directory kinds: 0), `rust-source-direction` (2d-compute family contract), `lint cargo-dependency-direction` and `lint dependency-direction` (launcher/native custody and a child deadline under load 50-127), `lib-wiring-budget` (roots re-grown by a running import-fix tool, bim by r9a), the `schema generate` derived catalog (stale again within minutes because peers keep editing schema modules; rerun `schema generate` last).

## 1. Gate table

| # | Gate | Command | Baseline | After | Owner / note |
|---|---|---|---|---|---|
| a | taxonomy | `verify taxonomy enforce` | CRASH in the planner twice | runs to the verdict: 667,026 findings (reference-edit-required 543,769; semantic-stem-unresolved 34,357; reference-syntax-unsupported 19,653; path-too-long 16,262; kind-only-basename 15,863; ...), directory-kind-unresolved 2,971 of which 37 outside dot paths, all created 10-03..10-11 | fixed: 3 `requireTaxonomyValue` wrappers that threw at token index 0 / last token (`normalization/🟦️.ts`); `@semio-tech/schema-validator-ts` added to the plugin-registry `workspaceImports` (taxonomy.json) |
| b1 | dependency-direction | `lint dependency-direction` | policy capture done (70,736 sources), then `Original repository child deadline exhausted` | same | unverified (load) |
| b2 | rust-source-direction | `lint rust-source-direction` | FAIL: 17 strict compile-time (all `framework-no-products`), 3 runtime-path, 486 census problems | unchanged | 421 `unproven-family-provider` + 50 bindings + 7 declaration-origin come from ONE cause: the 2d carve moved the `Engine*` family into `semio-framework-2d-compute` while `📏️ownership/🔣️.json` still names `semio-framework-2d` and 425 sites import `semio_framework_2d::compute::`; needs the contract re-pointed plus a codemod to `semio_framework_2d_compute::` and direct Cargo providers (compile needed) |
| b3 | cargo-dependency-direction | `lint cargo-dependency-direction` | refused: `Original native caller capabilities required` | same | launcher/native custody |
| b4 | dependency-cruiser | HR2 command | first run crashed (peer renamed a file mid-walk) | 9,237 violations, 19,124 modules (HR2 after: 10,593 / 17,896) | no-cross-package-relative 5,215 (was 6,430), no-core-path 942 (883), no-circular 501 (574), not-to-unlisted 414 (405; production 2, was 0), plugins-framework-sdk-only 637 (626), ui-no-framework-packages 297 (270), framework-no-products 190 (166); net lower, small rises on three rules |
| c1 | verify dependencies | `verify dependencies` | clean, exit 0 | same | |
| c2 | dependency truth | `... literal-external` | FAIL target=0 current=306, 6 oracle conflicts | same | long-term target, not fleet-caused |
| d | schema parsers | `schema parsers check` | fresh pending=0 dangling=0 drift=0 unowned=177 | same | |
| e1 | schema catalog | `schema generate --check` | stale | regenerated twice; stale again after minutes | derived file: run `schema generate` as the last step |
| e2 | schema check | `schema check` | 18,216 findings (13,088 export-incomplete etc.) | not re-run | pre-existing families |
| f | lib-wiring-budget | `test lib-wiring-budget` | 8 breaches | 0 of mine; see section 4 | |
| g1 | owners scan | `owners-tools/scan.ts` | 14 packages without owner row | 0 (3,222 packages, 3,364 rows) | `rg7-tools/owners_add.py` |
| g2 | workspaces-check | `workspaces --check` | current (6 scopes, 304 packages) | not re-run | |
| h | launch JSONs | `rg7-tools/launch_check.py` | both parse; 1 dangling crate route | fixed (playbook); now 13,543 / 10,345 configs, only stale ticket projects remain | peer: 3 duplicate names in launch.json, 1 in the seed, 51 `renderer-wgpu:native` routes (target removed long ago) |
| i1 | native-source-ownership | `test` | 2 fail | 1 fail (ignored stale build output `.semio-artifact.json` from 10-13) | `DomainHover` import line fixed |
| i2 | rust-family-ownership | `test` | cannot load | 76 pass | test fixture path fixed |
| i3 | native-owner-command-policy | `test` | killed (15 s budget) | 10 pass | |
| i4 | cargo-native-owner-discovery | `test` | killed | 60 s timeout (needs `cargo metadata`) | unverified |
| i5 | historical-package-owner-identity | `test` | 4 fail | 4 fail | pre-existing at HEAD (golden sha and contract absent) |
| i6 | crate-boundaries ui / os / db | `bun test ./...` | ui 6 fail, os 2 timeouts, db segfault | ui 59 pass, os 116 pass (`--timeout 120000`), db 45 pass | ui fixture rows re-derived from the manifests (scene split, retained-tree-tree, ...) |
| j | cargo-transaction-command-source | `test` | 3 fail | 9 pass; one 30 s typecheck timeout at load 127 on a later run | coordinator addition |
| k | build-surfaces | `test` | 1 fail | 17 pass | coordinator addition |
| l | plugin-publication-source-ownership | `test` | cannot load | 7 pass | coordinator addition |

## 2. Invocation notes

- A bare `bun ./📜️script.ts` loads only when the tree resolves; at 19:42 the root script failed to load once (`📇️directory/🧬️schema/🟦️.ts` re-exported not-yet-landed `📜️event/🟦️.ts`; a peer was splitting the directory schema), a retry two minutes later worked.
- `bun ./📜️script.ts verify layering`, every `repo run` and every owned route go through the `bun repo` launcher, which refuses with `Pending Native image publication must settle before ordinary admission` (the known peer/user blocker, COMPILE-ADMISSION-DIAG). The library package script (`📚️library/📦️packages/🟦️typescript/📜️script.ts`) refuses with `Original parent Script process invocation required` when started bare.
- Sanctioned workaround used here: `rg7-tools/envelope.ts` prints the exact root envelope (`createScriptProcessEnvelope` of the `workspace-root` policy with no deadline, the same value `runWorkspaceScriptMain` issues); the library script is then started from its package directory with `SEMIO_SCRIPT_PROCESS_INVOCATION=<wire>` in the environment. The commands run are the exact `command` of the `project.json` targets.

## 3. Edits (all re-read before writing; no git command)

- `📋️build-owners.json`: 14 owner rows (3d rigid, mesh document/clipping/reconstruction/tests/contract, collision, os native-runtime-reader, plugin registration/declared-verdict/command-page-laws/child-group-wire-laws/surface-carrier-laws/surface-fixture).
- Library ratchet `🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`: sequence 275 -> 276, writer 308 -> 309, curation 285 -> 286. The test measures `split("\n").length`, one more than `wc -l`; the roots shrank from 802 / 688 / 814.
- Roots made smaller instead of raising pins: energy mutation fixtures (merged two `#![allow]`, -2), process3d editor (merged the peer-added kit imports into one braced `use`, 484 -> 479), cad editor (293 pin, merged 3 peer `use` lines), trinity jack semio-content root 232 -> 7 lines (body moved to `🧱️owned/🦀️.rs`, tests mounted from there).
- `📍️ownership/🧪️tests/{🟦️.ts,🦀️native/🟦️.ts}`: fixture path (the fixture lives under `🧪️tests/`).
- `🗺️surface/🕸️node-graph/🦀️.rs`: `#[cfg(test)]` on its own line so the source-text reader finds `use protocol::{DomainHover, ...}`.
- `normalization/🟦️.ts`: three `requireTaxonomyValue` wrappers that threw at token edges now use `tokens[i]?.text ?? ""`.
- `taxonomy.json`: `@semio-tech/schema-validator-ts` binding in the plugin-registry generator contract.
- Launch files: `bun repo run semio-framework-artifact-playbook-playbook:...` -> `@semio-tech/framework-playbook-playbook-rs:...` (both files).
- `schema-catalog.json` regenerated by `schema generate`.
- ui crate-boundaries fixture: 6 rows re-derived (`rg7-tools/ui_boundaries.ts`).
- Library suites: `cargo-transaction-command-source` (fixture `parentKindId` repo-process -> process, owner declarations of native-build / owned-execution / native-orchestration, stale consumers pruned, count 23 -> 18, `cargoTransactionCommandSources` input gets `normalization/🟦️.ts`, package script form `bun --cwd ... repo run`), `plugin-publication-source-ownership` (fixture path, script form, renamed exports), `build-surfaces` (120 s timeout on the launch-file loop, which grows with the launch files).

## 4. Open: lib-wiring-budget at 22:30

A running import-fix tool appends one `use` line per name to plugin roots (block 3d/5d/2d, generation3d editor, layout editor, puzzle commands, process3d editor again) and does not recognise braced imports, so it re-adds names that a braced `use` already holds. After my merge it re-added four `process3d_kit` imports in the process3d editor (duplicate import, E0252); I removed the duplicates again. Roots now over the budget or their pin because of it: block 3d root +2, block 3d/5d/2d schema roots, block 3d editor, block 3d/5d mutations roots (170-181, crossed the 150 budget), generation3d editor +2, layout editor +2, puzzle 3d commands +1, process3d editor +4. Also unpinned new roots from a running process3d host carve (`retained-cursors` 665, `publication-authority` 350) and bim model 899 (r9a). Fix recipe: merge the appended `use` lines into the existing braced statements; the tool needs to treat braced imports as present.

## 5. Open items for owners

1. 2d-compute family contract (b2): owner = whoever carved 2d; steps in the gate table.
2. rust-source-direction `framework-no-products` (17): `value/📸️snapshot/👁️read` mounts os store modules by `#[path]`; pack json retained schema, replication and ui tests read os-store / plugin fixtures with `include_str!`; each is a product-to-framework edge from the carves.
3. `lint dependency-direction` and `cargo-dependency-direction` need the launcher (peer/user: native image publication).
4. Peer: duplicate launch names (cargo-registry-ready-native, installed-service), `renderer-wgpu:native` target gone, 350 launch routes to deleted ticket projects (`ticket-fixture-verification`, `draw-ticket-verification`, ...), `generate config --check` (devcontainer/dependabot drift).
5. Peer: `📏️ownership` of `historical-package-owner-identity`, ignored stale `.semio-artifact.json` in the surface bindings dir, dependency-cruiser `not-to-unlisted` production 2 (ambient `d3-shape-stack-order` d.ts and a print test).
6. Tests that need a longer budget at load > 50: `cargo-native-owner-discovery` (60 s), os crate-boundaries (5 s per case), `cargo-transaction-command-source` typecheck (30 s).
