# 2026-10-10 Mechanical Breakage Scan

Read-only audit of the main tree before the cold release build of semio-tech play. No source file was edited. The only file written is this report. Python used: `/Users/ueli/.local/share/uv/python/cpython-3.12-macos-aarch64-none/bin/python3.12` (system `python3` is 3.9.6 and has no `tomllib`). Scope excludes `node_modules`, `.git`, `target`, `dist`, and `.🧬semio/🌐hub`.

## Result summary

| # | Category | Main tree result | Issues outside main tree |
|---|---|---|---|
| 1 | Cargo.toml parse, duplicate keys, path deps | 0 issues (584 manifests) | cache 12 path deps (stale), ticket scratch 505 path deps, generated 66 invalid-TOML fixtures (intentional) |
| 2 | JSON parse (framework, ✏️s, 🏢️semio-tech) | 1 non-strict file (JSONC, tsconfig) | none |
| 3 | Merge-conflict markers | 0 (313,558 files scanned) | none |
| 4 | nx dependsOn resolution | 0 unresolved (see residual risk) | none |

No mechanical breakage that would stop the build was found. The two Cargo.toml files named in the rules now parse cleanly.

## 1. Cargo.toml

2283 manifests found and parsed with `tomllib`. Manifests were split by location.

- Main tree (584): 0 TOML errors, 0 unresolved `path =` dependencies (`dependencies`, `dev-dependencies`, `build-dependencies`, `target.*`, `workspace.dependencies`, `patch.*`), 0 unmatched `workspace.members` globs.
- Fixed files verified:
  - `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml`: parses; `dev-dependencies` keys are `semio-framework-trace`, `serde_json`. No duplicate key.
  - `🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/Cargo.toml`: parses; `dependencies` keys are `machine`, `semio-framework-replication`, `semio-framework-job`, `semio-framework-value`. No duplicate key.
- Duplicate-key detection was sanity-checked: `tomllib` raises `TOMLDecodeError` on a duplicate key, so the zero-error result is meaningful.

Outside the main tree (informational, not part of the build):

- Cache `.🧬semio/🦑️repo/⚡️cache/tests/...` and `.🧬semio/🦑️repo/⚡️cache/🧪️tests/cargo-workspace-contract/...` (962 manifests): 12 unresolved path deps. These are stale copies of old test hosts. Examples:
  - `.🧬semio/🦑️repo/⚡️cache/tests/tasks/w2sc/raster-parity/hosts/.../Cargo.toml`, `dependencies`, `semio-s-plugin-stdio-test-oracle` points at `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust`, which does not exist.
  - `.🧬semio/🦑️repo/⚡️cache/🧪️tests/cargo-workspace-contract/workspace-native-*/Cargo.toml`, `workspace.dependencies`, `unused` points at `absent/package` (this appears intentional for a contract test).
  - Line numbers were not captured for these; the file paths identify them.
- Ticket scratch `.🧬semio/🦑️repo/🎫️tickets/...` (168 manifests, 0 TOML errors): 505 unresolved path deps across 11 day folders (☀️23: 256, ☀️11: 181, ☀️06: 48, others under 10 each). None are in the BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT ticket, which has no manifests.
- `🗑️generated` folders (569 manifests): 66 TOML parse errors, all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/🗑️generated/cargo-production-*/...`. The files intentionally contain the text "fixture-only invalid TOML" (checked on `.../ordinary/🔨️modules/🧪️fixtures/nested/🧪️fixtures/sample/Cargo.toml`). These are intended invalid fixtures, not breakage.

## 2. JSON

Parsed all `*.json` under `🧰️framework`, `✏️s`, and `🏢️semio-tech` (excluding node_modules, dist, target, .git): 41,831 files. No `.jsonc` files exist in those trees. Project manifests (`📋️project.json`, `package.json`) and all schema files parse.

One file fails strict JSON parsing:

- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json:1`: begins with `//` line comments (license header, tracked in git). This is valid tsconfig JSONC, which `tsc` and Bun accept, so it is not a build breakage. It would break any strict `JSON.parse` consumer. No action required unless a tool reads it as strict JSON.

## 3. Merge-conflict markers

Scanned 313,558 files with extensions `.rs`, `.ts`, `.tsx`, `.toml`, `.json` in the main tree. Result: 0 marker lines (`<<<<<<< `, `>>>>>>> `, `=======`). A second pass with a CRLF-tolerant regex over `*.lock`, `*.mjs`, `*.rs`, `*.ts`, `*.tsx`, `*.toml`, `*.json` found 0 matches. The three relocked Cargo.lock files (`✏️s/Cargo.lock`, `🌎️hub/Cargo.lock`, `🎓️teaching/Cargo.lock`) parse as TOML.

## 4. nx dependsOn

Play: `🏢️semio-tech/🎡️play/📋️project.json` contains 115 `dependsOn` entries.

- 10 resolve to explicit targets (local targets such as `activate-dev`, `prepare-release`, `catalog-release`, and `workspace:deps-browsers`, which exists in the root `workspace` project).
- 105 reference `@semio-tech/framework-os-dev:(prepare|activate)-<variant>-react-(dev|release)`. These targets are not literally written in `framework-os-dev`'s `📋️project.json` (55 explicit targets). They are inferred by the repo's Nx plugin `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`:
  - line 1375 merges `playgroundPreparationTargets(...)` into any project named `@semio-tech/framework-os-dev`.
  - Names are built from `playground.variant` (around lines 1192 to 1247).
  - All 28 variants referenced by play are declared in some `Cargo.toml` `[package.metadata.semio] playground` row (main tree, excluding hub-independent generated dirs). Example: `variant = "demonstrator"` in `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:54`. Variants `stdio-png`, `bim`, and others are declared under `🌎️hub/🧩️compositions/...`.
- Result: 0 unresolved references.

`@semio-tech/framework-os-dev` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`): 6 `dependsOn` entries, 2 local references not literally defined:

- `/targets/build` -> `build-s-react-release`
- `/targets/cold-boot-check-s` -> `activate-s-react-dev`

Both come from the same plugin template (`build-${variant}-react-release`, `activate-${variant}-react-dev`) for variant `s`, declared at `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml:23`. Not a breakage.

Project-name collisions (informational, nx-ignored):

- Same name in two directories: `@semio-tech/framework-actor-rs` (`.../🦀️rust` and `.../🦀️rust/pkg`), `@semio-tech/framework-surface-rs` (`.../🦀️rust` and `.../🦀️rust/🕸️bindings`), `@semio-tech/framework-editor-rs` (`.../🦀️rust` and `.../🦀️rust/pkg`), `@semio-tech/trinity-jack-lsp` (`.../🦀️rust` and `.../🦀️rust/pkg`), plus `@jupyterlab/application-top` inside `.venv`.
- All the duplicate directories are gitignored (`.gitignore:114 **/pkg/`, `🕸️bindings/.gitignore:1 *`, `.gitignore:361 .venv`), so nx project discovery should skip them. Not confirmed by running nx.

## Residual risk

- The 105 `prepare-*`/`activate-*` references are verified statically against the plugin source and the Cargo.toml declarations. They are not verified by an actual Nx graph run. A run such as `nx show project @semio-tech/framework-os-dev` would confirm, but it was not executed: the rules reserve builds for the coordinator, and an nx run writes to the shared nx cache.
- The rules file `📓️2026-10-10-redeploy-fleet-rules.md` changed on disk during this audit (new findings-log entries). This audit did not depend on its contents beyond the rules section.
