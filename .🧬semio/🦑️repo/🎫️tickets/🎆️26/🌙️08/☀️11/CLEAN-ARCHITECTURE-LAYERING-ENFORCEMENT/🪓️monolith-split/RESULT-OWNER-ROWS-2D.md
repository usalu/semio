# Owner Rows for the 2d Module and Other Unregistered Workspace Members

## Verdict

The `Canonical native dependency owner required: …◻️2d/⚙️engine` failure came from crates that exist in the workspace but have no `📋️build-owners.json` row. The graph code throws it at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` line 1647 (`discoverBuildOwnerAdmissionFactsV1`). It walks the Cargo path-dependency closure of each admitted owner and requires every closure root to be an owner `root` or `nativeRoots` entry.

I registered 235 crates: the 2d module, 9 other framework crates and 209 plugin-artifact crates. After that, no Cargo package in the repo lacks an owner. The route still fails, but earlier and for another reason (storage headroom, below). I could not run the owner check itself through the route, so what I can show is an offline replica (see Verification).

## Reproduction

Command: `SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' bun repo run @semio-tech/quiz-schema-rs:check --configuration=prototype --parallel=1 -- --lib --offline`.

- The route did not reproduce the owner error at all. It stopped before graph inference with `error: Active graph readers retain required Nx pool headroom`, thrown from `prepareNxGraphCacheStorageV1` (`⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:43`). Log: `🗑️generated/owners-repro.log`. By that time a peer's earlier edits may already have changed the state, so the owner error was not observable through the route.
- I reproduced the failing condition offline instead. `owners-tools/closure.ts` replicates `nativeDependencyRoots` (path dependencies, workspace-inherited dependencies, target tables, dev-dependencies only at the owner root) and lists closure roots absent from the registry.
- Before my edit it listed 15 framework roots, each reached by 1089 to 1127 owners:
  - 9 under `◻️2d`: `⚙️engine`, `📝️text`, `🔀️booleans`, `🔍️trace`, `🛤️path/📏️flatten`, `🛤️path/🖊️stroke`, `🧮️compute`, `🧱️regions`, `🧹️retire`. `⚙️engine` sorts first in the sorted closure, which matches the reported error text.
  - 5 under `🎭️actor` (`retained-turn`, `retained-turn/🎟️context`, `job/📨️publication`, `📖️vocabulary`, `🧵️pack`).
  - 1 under `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw` (`semio-framework-ui-wgpu-render`).
  - Plus 195 plugin-artifact roots (procedural, fem, block, lowpoly, layout, norm, draw, space, process, cad).
- The member-versus-owner comparison (`owners-tools/scan.ts`: every tracked or untracked `Cargo.toml` with `[package]` against all owner `root` and `nativeRoots`) found 234 unregistered packages: the 15 closure roots above, 8 more `◻️2d/📝️text` crates, `⏯️tool-run/👥️entities`, `⏯️tool-run/📊️progress/🪟️presentation`, `🚪️io/📦️codec`, and 208 plugin crates.

## What was added

### Framework crates (26 owner rows, in `📋️build-owners.json`)

Convention followed: `PRODUCER-CRATE-RECIPE.md` steps 8 to 10 and `📓️CARVING-RULES.md` rule 1 (project file, owner row, six launch routes in both launch files). Rows have `name`, `root` (the `📦️packages/🦀️rust` directory), `manifest` (`📋️project.json`) and `packages: []`, inserted in name order.

- 19 new `📋️project.json` files, `{ "name": "@semio-tech/<id>-rs", "tags": ["scope:framework", "language:rust"] }`, where `<id>` is `[package.metadata.semio] id`. This covers all 17 unregistered `◻️2d` crates (including `unicode-properties` and `unicode-mappings` under `📝️text/…/🌐️unicode`) and the two `⏯️tool-run` crates. Names: `@semio-tech/2d-{engine,text,text-font-catalog,text-font,text-font-testing,text-font-run,text-font-shape-unicode,text-font-shape,booleans,trace,path-flatten,path-stroke,compute,regions,retire}-rs`, `@semio-tech/unicode-properties-rs`, `@semio-tech/unicode-mappings-rs`, `@semio-tech/tool-run-entities-rs`, `@semio-tech/tool-run-presentation-rs`. `semio-framework-2d` already had its row and project.
- 7 crates already had a `📋️project.json` but no owner row; I added only the row, using the name in the file: `@semio-tech/framework-actor-{retained-context,retained-turn,job-publication,vocabulary,pack}-rs`, `@semio-tech/ui-wgpu-render-rs`, `@semio-tech/framework-io-codec-rs`.
- `ui-wgpu-render-rs` (peer ticket `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`, with custom launch entries) and `framework-io-codec-rs` (custom `test-contract` and `test-native` targets and routes) got row only, no six standard routes.
- Six launch routes (build, check, test, each prototype and release) for the other 24 crates: 144 route objects, identical in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`. Format copied from the `semio-framework-surface-terrain` block, including the `env` line. Placed directly before the `semio-framework-repo-dashboard-core` build-prototype route. Order keys come from unused prefixes `900.97NNNk`, `922.097NNNk` and `903.597NNNk`, checked against both files numerically.

### Plugin-artifact crates (209 owner rows)

All were `semio-s-artifact-*` Cargo packages with no `📋️project.json` (procedural 57, fem 30, block 55, lowpoly 20, layout 17, norm 17, draw 6, space 3, process 2, cad 1). They follow the 709 registered siblings: `name` = Cargo package name, `manifest` = `<root>/Cargo.toml`, `packages: []`, no launch routes. A peer was still creating plugin crates while I worked (Cargo.toml mtimes up to 18:01), so the set grew by one between two scans. If that peer also adds rows for these crates, the registry reader will reject the duplicate with "Canonical native owner collision"; the fix is to delete the duplicate row.

## Decisions to know about

- The 2d crates could also have been registered as Cargo-manifest rows (the pattern of the 52 framework rows such as `semio-framework-3d-brep-*`, which have no project.json and no launch routes). I chose the recipe's project.json plus routes because the task and `📓️CARVING-RULES.md` name it. Both patterns are accepted by `readBuildOwnerRegistryV1` and by the graph code (line 1623 derives the owner name from `package.name` for Cargo manifests).
- I did not touch Cargo manifests, crate sources, `Cargo.lock`, or the taxonomy, and added no corpus rows (the 2d area has none).
- Crates under `◻️2d/📦️packages/🟦️typescript` and the TS owner `@semio-tech/2d-compute` are unchanged; the new Rust owners nest under their directories and the deepest root wins, as for other nested packages.

## Verification

| Check | Result |
| --- | --- |
| `bun owners-tools/validate.ts`: `readBuildOwnerRegistryV1` (schema `🧬️schema/📋️owners.json`, name/root/package collision checks) on the final registry | `registry valid, owners: 2212`. 1977 rows before, 235 added. |
| `.vscode/launch.json` parses as JSON; `.vscode/🧩️launch.seed.jsonc` parses with `Bun.JSONC.parse` | Both parse after the edit (done inside `apply-framework.ts`). |
| `owners-tools/closure.ts` after the edits | Only the false positive root `""` (the repo-root `workspace` owner, which is registered as root `.`; the replica prints `""`). No other closure root lacks an owner. |
| `owners-tools/scan.ts` after the edits | 2065 packages, 2212 owners, `missing: []`. |
| Route `quiz-schema-rs:check` re-run after the framework rows and again after the plugin rows (`🗑️generated/owners-after.log`, `owners-after2.log`) | Still exits 1 with `error: Active graph readers retain required Nx pool headroom`, from `prepareNxGraphCacheStorageV1`. The string "Canonical native dependency owner" appears in neither log. |
| Direct call of `discoverBuildOwnerAdmissionFactsV1` (`owners-tools/facts.ts`) | Refused with `Original bounded graph writer custody required`; it needs a caller invocation. I did not work around it. |

How far the route gets: it now stops at the same graph-storage admission as before, which runs before graph inference. The owner check is therefore not exercised by the route in either state. The Nx pool headroom failure is the image blocker described in `📓️FLEET.md` (16:30 section) and is not addressed here. Host disk itself has 118 GiB free, so this is the cache image, not the volume.

## Files

- Edited: `/Users/ueli/Documents/semio/📋️build-owners.json`, `/Users/ueli/Documents/semio/.vscode/launch.json`, `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc`.
- Created: 19 `📋️project.json` files under the `🧰️framework/🔨️modules/◻️2d/…/📦️packages/🦀️rust` crates (17) and `⏯️tool-run` (2), see the list above (the `2d` and `tool-run` crates are the only ones created; the 7 existing-project crates were not edited).
- Tools (kept, idempotent, rerun with a fresh scan): `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/owners-tools/` with `scan.ts`, `closure.ts`, `closure-one.ts`, `apply-framework.ts`, `apply-plugins.ts`, `validate.ts`, `facts.ts`. Usage: `bun scan.ts > list.json`, then `bun apply-framework.ts list.json [--dry]` and `bun apply-plugins.ts list.json [--dry]`.
- Logs and scan outputs: `🗑️generated/owners-*.log`, `owners-*.json` (not deleted).

## Follow-ups for the fleet

- Any crate a peer adds from now on needs its row the same way; rerun `scan.ts` after each carve wave. A new member without a row breaks every owner whose closure reaches it.
- The six standard routes were not added for the 209 plugin crates, in line with their registered siblings.
- The image headroom blocker is the next gate for any `bun repo run` route.
