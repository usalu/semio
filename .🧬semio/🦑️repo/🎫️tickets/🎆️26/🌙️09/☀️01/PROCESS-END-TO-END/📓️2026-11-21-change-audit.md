# 🔍️ Process Plugin and Composition Change Audit (2026-11-21)

Read-only audit of `✏️s/🔌️plugins/🏭️process` and `🌎️hub/🧩️compositions/🏭️process`. No source files were modified and no repo-modifying git command was run.

Checker script: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/PROCESS-END-TO-END/check-process-paths.py`

## 1. Git history since 2026-09-16

Three commits touch the scope:

| Commit | Date | Files touched in scope | Summary |
|---|---|---|---|
| `3d62b2ef22c5` | 2026-11-18 | 546 added, 0 modified | Initial creation of the process plugin tree (crates, extensions, tests, Cargo.lock files, descriptors) |
| `5db034ffe534` | 2026-11-20 | 17 added, 81 modified | Large Rust edits across `process3d` and extensions, new `workpiece-replay.json`, `composed-steps.json`, `import-stock.obj`, new `project.json` for one crate |
| `b4927d2c311c` | 2026-11-21 | 5 added, 30 modified, 1 renamed | Mostly Rust edits, `note-mcp-replay` crate, descriptor rebuild for the composition |

Renames and moves:
- Only one rename in the window: `🗿️artifacts/🧊️process3d/.../✏️editor/🧪️tests/🔬️unit/🎮️interaction-behavior/🦀️.rs` moved to `.../✏️editor/🧪️tests/🎮️interaction-behavior/🦀️.rs` (dropped the `🔬️unit` segment). No stale references to the old path were found (see section 2).

Removed files:
- No deletions (`D`) in the window.

Cargo.toml dependency churn (notable):
- `b4927d2c311c` adds a path dependency on `semio-s-artifact-procedural-3d-graph-ports` (`../../🔌️graph-ports`). The target exists.
- `b4927d2c311c` adds `semio-framework-artifact-playbook-playbook` (workspace).
- `5db034ffe534` removes `semio-s-artifact-stdio-semio` from many crates, then re-adds it in others. This is churn that should be checked against intent.
- `5db034ffe534` removes `semio-s-artifact-gis-gismap-terminology-tests` from one crate.
- `5db034ffe534` adds `semio-framework-job` with `features = ["testkit"]` in one crate.
- Several Cargo.lock files gained or lost large numbers of entries (e.g. the `semio-s-artifact-stdio-gltf-mutations-mut-any-{4,5,6,10}` packages were removed from one lock file).

## 2. Structural consistency (check-process-paths.py)

Result after fixing the checker's own `project.json` name match (the files are named `📋️project.json`, and the first run silently checked none):

- `#[path = "..."]` attributes: 255 checked, 0 broken.
- `include_str!` / `include_bytes!`: 150 checked, 0 broken.
- Cargo.toml `path =` entries: 221 checked, 0 broken.
- `project.json` `cwd` values: 19 checked, 0 broken (all resolve from the workspace root).

Broken things: none found by the checker.

Note on `project.json` count: 7 files in scope (`📋️project.json`), none under the composition's `🧩️component`.

## 3. Built plugin artifacts

- No process wasm or descriptor exists under `🧰️framework/🛍️products/💻️os/`. The only `*descriptor*` hit there is `.../🔨️modules/🔌️plugin/🧪️testing/🧩️component/🛂️.descriptor.semio` (18:01), which is unrelated to process.
- Process descriptors exist only inside the plugin and composition trees:
  - `🌎️hub/🧩️compositions/🏭️process/🧩️component/🛂️.descriptor.semio`, 81756 bytes, mtime 2026-11-21 18:16.
  - Four extension descriptors under `✏️s/🔌️plugins/🏭️process/🧩️extensions/{🔩️metal,🪵️wood,🤖️robotic,🧱️concrete}/🧩️component/🛂️.descriptor.semio`, mtime 2026-11-21 18:01.
- No `*.wasm` files exist for process anywhere in the scanned trees.
- Staleness:
  - Composition descriptor (18:16) is newer than all composition sources. No composition files are newer.
  - Extension descriptors (18:01) are older than later source edits: `Cargo.lock` for metal, wood, robotic and concrete at 21:04. Each of those extensions also has `🦀️.rs` files whose mtime is 18:01, so the minute-level comparison is inconclusive.
  - Latest source edits in the plugin are at 22:04 (`🫧️transient/👆️placement/🧬️schema/🔣️.json`, `🧪️fixtures/⚖️retained-route-laws/🔣️.json`, `🧪️tests/🔬️typescript-package-commands/🟦️.ts`).
  - Caveat: all mtimes near 18:01 look like replica copy times, not build times, so they are weak evidence.

## 4. Dev-route definition

File: `✏️s/🔌️plugins/🏭️process/🧪️tests/🧭️dev-route/🔣️.json`

Path references (all resolve):
- `manifest` = `🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml` (exists; its playground block at line 21 points at the same `devContribution`, `app` and `ports`).
- `ownerRoot` = `✏️s/🧑‍💻dev/🎭️variants/🏭️process` (exists; contains `🏗️builder`, `🚀️entry`, `🧬️schema`).
- Dev contribution `ownerRoot/🧬️schema/🔣️.json` references `viteConfig` and `browserEntry`, both exist.
- The test file `🟦️.ts` imports resolve (`../../../../🧑‍💻dev/🧩️catalog/🟦️.ts`, framework `🔌️plugin/📇️registry/...` and `🧑‍💻dev/🧩️contribution/...`). The generated catalog dir `🤖️generated` exists.

Extension name mismatch:
- The dev-route lists `process-extension-{concrete,metal,robotic,wood}`.
- The crate names are `semio-s-plugin-process-{concrete,metal,robotic,wood}` (and `-component` variants).
- The dev-route names match the plugin IDs used in `🧰️framework/.../🧪️tests/...` fixtures and the npm names in `build-owners.json` (`@semio-tech/process-extension-*-rust`), so this is a naming-convention split, not a missing path. It should be stated explicitly or unified.
- `semio-s-plugin-process-catalog-json-oracle` (under `🧩️extensions/🧪️tests`) is not in the dev-route list, which is expected for a test oracle.

## Key findings (concrete)

1. Checker found 0 broken `#[path]`, `include_*`, Cargo `path=`, or `project.json` `cwd` references in scope.
2. Dev-route paths and manifest/ownerRoot references all exist.
3. Dev-route extension IDs (`process-extension-*`) differ from crate names (`semio-s-plugin-process-*`) by convention. Not a broken path.
4. No built process wasm or framework-level descriptor exists under `🧰️framework/🛍️products/💻️os/`.
5. Extension descriptors (mtime 18:01) predate Cargo.lock edits at 21:04. Rebuild status is not confirmed.
6. Cargo.toml churn: `semio-s-artifact-stdio-semio` removed and re-added across crates in `5db034ffe534`. Needs an intent check.
