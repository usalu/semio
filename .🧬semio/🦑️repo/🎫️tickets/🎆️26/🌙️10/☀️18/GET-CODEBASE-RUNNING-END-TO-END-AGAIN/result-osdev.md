# Result osdev: os-dev, hub and workspace target references (static)

Executor `osdev`, 2026-10-18. No builds, no cargo, no graph run, nothing committed.

## Ground truth

`bun repo show project @semio-tech/framework-os-dev --json` does not run: the CLI fails with `Cannot find module '@semio-tech/process/wasm-build'` (imported from `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts`). The graph was therefore derived statically from the inference module, which lives in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` (the brief's path omits `🏗️build`). Read only.

Inference facts used (line numbers of `🟨️.mjs`):
- L1226-1241 `componentBuildTargets`: target `component` exists on every Cargo crate with `package.metadata.component.package` and `semio.component-kind` in `plugin|extension` (the cdylib sub-crate under `🧩️component/`). The project name of such a crate is its Cargo package name (`semio-hub-<id>-component`, `semio-s-plugin-<family>-<name>-component`; registered in `📋️build-owners.json`). The hub composition projects (`@semio-tech/gis-plugin` etc.) are not component crates and have no `component` target.
- L1359-1531 `playgroundPreparationTargets` (attached to `@semio-tech/framework-os-dev` at L1645/L1771) for each of the 156 playground variants and both profiles: `prepare|activate|serve|dev-<v>-<react|wgpu>-<prototype|release>`, `prepare|run|smoke-<v>-native-<profile>`, `mcp-<v>-<stdio|http>-<profile>` (only with `mcpHost`), `build-<v>-react-release`, `build-<v>-react`, `prepare-<v>-<react|wgpu>`, `build-all-playground-cdn-sites`. `session-<v>` on `@semio-tech/plugin-registry` (L1326).
- L1533 `rootCommandTargets`: the root project gets a target per `.register(...)` of the root router, but only in the full-graph path. `start` is not registered (StartScript retired in commit 18782fa4b19, together with the `start`, `dev` and `dev-mcp-engine` root targets).

A small Bun model of this (`osdev-targets.ts`, same folder, input script) loads all `project.json` + `Cargo.toml`, adds the inferred targets above, then checks every `dependsOn` and every `bun repo run <project>:<target>` launch row. The runtime-input admission function from the repo (`🕸️dependencies/🧩️runtime/🟨️.mjs`) was run per playground: 0 refused, 0 engines without a `wasm` producer. The inferred dependency targets of the playground lanes (`os-plugin-describe-rs:build`, `framework-plugin-web:support`, `deps-wasm-opt`, `os-infinite:fonts`, `canvas:stage-runtime-assets`, `assets:build`, wgpu `wasm`/`generate-*`, flow `wasm`) all exist.

## Findings against the audit

- os-dev `build` -> `build-s-react`, `cold-boot-check-s` -> `activate-s-react-prototype`: both are inferred (variant `s` exists). Not missing. No change.
- The 21 demonstrator cross-deps: all resolve (the demo executor's `prepare-deps` rewrite plus inference). No change.
- The 342 os-dev launch rows: all 402 `framework-os-dev:` rows in `launch.json` resolve (dev/serve/activate/prepare/build/run/mcp names). The audit counted inferred targets as missing. No change needed.
- `workspace:dev-mcp`, `dev-mcp-repo`: declared. Rows ``workspace:dev -- …`` are gone (demo executor); no launch row uses an inferred-only root target.
- Genuinely stale references (fixed below): hub/composition `component` dependencies, `workspace:start`, `workspace:dev-mcp-engine`, two component launch rows, 46 `framework-renderer-wgpu:native[-release]` launch rows.

## Changes

`dependsOn` (stale `<composition>:component` -> `<component crate>:component`):
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`: `test-all-features`, `component-codec-budget-check` -> `semio-hub-gis-component`; `component-codec-check` -> `semio-hub-{note,gis,stdio}-component`.
- `🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📋️project.json`: `inference-discovery-oracle`, `inference-discovery-check`: `"component"` -> `"semio-hub-gis-component:component"`.
- `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust/📋️project.json`: `guest-instance-check` -> `semio-hub-draw-component`.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/📋️project.json`: `test`, `verify-sqlite-snapshot-guest`, `verify-snapshot-guest` -> `semio-hub-note-component`.

Launch rows, edited identically in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (order, group, name untouched):
- `▶️start`: `workspace:start` -> `os-hub:local-hub-owner`. StartScript only launched the local hub owner outside the devcontainer; that is the successor (default URL and data dir come from the owner script; the existing row `🛠️dev🗄️os-hub🎫️local` additionally sets env for sign-in).
- `🛠️dev-mcp-engine` (launch.json only; not in the seed): row removed. Target and `dev mcp engine` route were removed together in 18782fa4b19; no successor.
- `🧩️Component Compiler 🛠️prototype|release`: `@semio-tech/stdio-json-rs:component` -> `semio-hub-stdio-component:component` (json is not a component crate).
- `📇️catalog🏢️semio-tech🎡️play🏢️aec-building`: `@semio-tech/cad-extension-aec-building-rust:component` -> `semio-s-plugin-cad-aec-building-component:component`.
- 46 native rows `🛠️dev<variant>🧊️wgpu🖥️native`: `@semio-tech/framework-renderer-wgpu:native -- <v>` -> `@semio-tech/framework-os-dev:run-<v>-native-prototype --configuration=prototype` (and `native-release -- s` -> `run-s-native-release --configuration=release`). `native` was removed from the wgpu project in 18782fa4b19; its job (`native-entrypoint run <variant> <profile>`) is the inferred `run-<v>-native-<profile>`. The old `trinity` argument is not a variant: mapped to `trinity-jack`.

## Verification

- `launch.json` parses (13593 configurations); the seed parses with `Bun.JSONC`.
- Model re-run after the edits: 0 unresolved `dependsOn` apart from the glob selectors of the root `lint|test|build` (`*:lint` etc.); 0 unresolved launch rows that mention os-dev, os-hub, workspace, wgpu or component.
- Not run: any `bun repo run`, since the graph cannot load (error above), and no inference test.

## Still open

- The CLI fails on `@semio-tech/process/wasm-build` (`🌊️flow/🕸️wasm/🌐️browser/📦️publication`): another executor has to restore that package or import path before any target can run. Also not verifiable here: native-input admission refusals in the inference (they would replace a variant's targets by `runtime-input-admission-<v>` stubs).
- Launch rows outside this scope that still do not resolve in the model: `@semio-tech/repo-cli-rs:{run,daemon,install,preferences,workflow}` (dashboard moved to `@semio-tech/repo-dashboard-rs`, which has no `workflow`), `@semio-tech/plugin-registry:{check-launch-seed,reconcile-launch-seed,test-launch-seed-reconcile,test-launch-name-contract,test-launch-placement-contract}` (the launch generator was deleted), assorted `test-*` rows of `repo-lib`, `framework-os-mcp-rs`, `framework-rs`, `value-rs`, `framework-io-schema-rs`, `stdio-xlsx-rs`, `fixture-generator-stdio-tiff-6-0-document`, and `current-native-domain-pick-1:verify`. Ticket-scoped `ticket-fixture-verification:*` and `draw-ticket-verification:*` rows are ticket projects. `${input:playComponentPackage}:component` is an input placeholder, not a stale name.
- The `workspace:dev` references listed as open by the demo executor (dashboard oracle, README) are untouched; cache-contracts test already asserts `workspace:dev` and `framework-os-dev:dev` are retired.
- `component` targets on composition-level projects could instead be declared as aliases; deliberately not done (the inference documents `component` as a per-crate target).
