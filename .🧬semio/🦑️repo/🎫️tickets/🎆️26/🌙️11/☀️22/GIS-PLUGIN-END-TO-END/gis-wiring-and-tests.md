# 🌍 GIS plugin wiring, tests and end-to-end launch

Read-only exploration. No builds were started and no files were edited outside this note.

## 1. Where the GIS plugin is wired

| Layer | Path | Role |
|---|---|---|
| Plugin source (s) | `✏️s/🔌️plugins/🌍️gis` | Artifacts `🗺️gismap` (2D) and `🏔️gisterrain` (3D), with Rust and TS packages |
| Plugin component (hub) | `🌎️hub/🧩️compositions/🌍️gis` | Wasm component `semio_hub_gis_component`, crate `🧩️component/📦️packages/🦀️rust`, package `semio-hub-gis-component`, project `@semio-tech/gis-plugin` |
| Plugin metadata | `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` (line ~566), `🧩️plugins/🟦️.ts` (line 60) | Generated plugin row: `pluginId: gis`, `dependsOn: [stdio]`, `activationEvents: on-artifact-kind:s.gis.gismap / s.gis.gisterrain`, `contributes: stdio.artifact-catalog.v1, semio.os.document-http-port/v1` |
| Playground rows | `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts` (lines 66-67) | `gis2d` (app `s.gis.gismap@1/*#editor`, ports react 6040 / wgpu 6140) and `gis3d` (app `s.gis.gisterrain@1/*#editor`, ports react 6083 / wgpu 6183) |
| s instance (dev variant) | `✏️s/🧑‍💻dev/🎭️variants/🌍️gis` | Variant folder with `🧬️schema/🗺️map.json`, `🧬️schema/🏔️terrain.json`, `🚀️entry`, `🏗️builder/🌐️vite`, `🧩️service-composition`, `💡️services` |
| s instance (catalog default) | playground row `variant: "s"` in `🎮️playgrounds/🟦️.ts` (line 86) | `catalogDefault: true`, `pluginId: space`, `devContribution: ✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧬️schema/🔣️.json`, ports react 6070 / wgpu 6066, userPorts react 6072, 6073 / wgpu 6067, 6068, hub `http://127.0.0.1:8787` |
| Native and MCP hosts for gis | `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust` (`semio-s-gis-services-native`, `semio-s-gis-services-mcp`) | nativeHost and mcpHost of the gis2d row |
| Plugin registry | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry` (generator `framework/plugin/registry/script.ts`) | Produces the generated catalog above (`🤖️generated/*`). Do not edit generated files |
| Dev catalog composition | `✏️s/🧑‍💻dev/📋️project.json` (`@semio-tech/s-dev-composition`) | Typecheck depends on `@semio-tech/gis-gisterrain:build` and `@semio-tech/gis-gismap-js:build`; also `test-catalog-publication` lists `🕹️gis-map-process/🟦️.ts` as a closure input |
| Hub | `🌎️hub/📦️packages/🦀️rust` (`os-hub`) | Local hub on port 8787. Its `test-all-features` and `component-codec-*` depend on `semio-hub-gis-component:component` |

Discovery (where to look when wiring changes): `grep -rIl -E 'gismap|gisterrain|semio-hub-gis-component' --exclude-dir=node_modules`. The generated catalog files list the GIS rows; `🌍️gis` is the `directoryName` in `🗂️sources.json` (line 349) and `🔌️plugins.json`.

## 2. Projects and targets that exercise GIS

Projects are named from `project.json` `name`. Targets are run as `repo run <project>:<target>`.

### Plugin packages (`✏️s/🔌️plugins/🌍️gis`)
- `@semio-tech/gis-js` (`✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript`): `test`
- `@semio-tech/gis-gismap-rs` (`🗿️artifacts/🗺️gismap/📦️packages/🦀️rust`): `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-gis-map-window-ownership`, `verify-gis-map-window-ownership-native`, `verify-gis-map-window-ownership-oracle`, `verify-inference-client`, `verify-inference-client-native`, `verify-inference-mcp`, `verify-inference-native-service`, `verify-map-document-contract`, `verify-map-document-contract-native`
- `@semio-tech/gis-gismap-js` (`🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript`): `build`, `check`, `test`, `cold-document-pair-check`, `inference-bridge-process-check`, `inference-bridge-source-check`, `inference-check`, `inference-service-receiving-check`, `tiles-check`, `tiles-check-native`, `tiles-prefetch`, `verify-physical-codecs`
- `@semio-tech/gis-gisterrain` (`🗿️artifacts/🏔️gisterrain/📦️packages/🟦️typescript`): `build`, `check`, `test`
- `@semio-tech/gis-gisterrain-rs` (`🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust`): `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-gis-terrain-window-config`, `verify-gis-terrain-window-config-native`, `verify-terrain-document-contract`, `verify-terrain-document-contract-native`

### Hub composition (`🌎️hub/🧩️compositions/🌍️gis`)
- `@semio-tech/gis-plugin` (`📦️packages/🦀️rust`): `test`, `installed-service-source`, `component-cold-map-patch-check`, `component-cold-map-patch-native-check`, `durable-three-store-assembly-check`, `durable-three-store-assembly-native-check`, `hub-edit-durability-check`, `inference-discovery-check`, `inference-discovery-oracle`, `inference-transport-check`, `map-create-region-group-check`, `map-create-region-group-native-check`, `native-codec-check`, `native-codec-oracle`, `native-codec-projection`

### s variant services (`✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust`)
- `@semio-tech/s-gis-services-native` (application): `source-check`, `native-check`, `native-build` (prototype or release), `mcp-build`, `mcp-check`, `check`, `fixture-ownership-check`, `agent-reply-check`, `canonical-pair-composition-check`, `hub-agent-participant-check`, `inference-quartet-check`, `live-agent-loop-check`, `mcp-composition-source-check`, `plugin-coverage-check`, `security-check`, `untrusted-content-check`, `user-path-check`

### Hub gates (`os-hub`, project `🌎️hub/📦️packages/🦀️rust`)
The hub test folders (`🌎️hub/🧪️tests/...`) have no `project.json` of their own. They are run by these `os-hub` targets:
- `trusted-stdio-gis-bundle-check` (source; imports `🌎️hub/🧪️tests/🎁️trusted-stdio-gis-bundle-check/🟦️.ts`), `trusted-stdio-gis-bundle-browser-check` (`--browser`), `trusted-stdio-gis-bundle-process-check` (`--process`)
- `gis-map-proposal-source-check`, `gis-map-proposal-native-check`, `gis-map-proposal-process-check` (`gis-map-proposal-check --source|--native|--process`; the process gate is `🌎️hub/🧪️tests/🕹️gis-map-process/🟦️.ts`)
- `gis-map-frozen-binding-source-check`, `gis-map-frozen-binding-native-check`
- `gis-inference-ledger-check`, `gis-inference-ledger-oracle`
- `browser-actor-gis-describe-check`
- `component-codec-check`, `component-codec-budget-check` (both depend on `semio-hub-gis-component:component`)
- `test-all-features` (depends on `semio-hub-gis-component:component`)

Test folders and what they are:
- `🌎️hub/🧪️tests/🕹️gis-map-process`: the two-author shell process gate (main e2e gate for GIS map)
- `🌎️hub/🧪️tests/🎮️gis-map-process-support`: document sockets, MCP clients and checkpoint helpers, imported by the process gate
- `🌎️hub/🧪️tests/🌍️gis-map-fixtures`: proposal, ingress and visibility fixtures, imported by the process and trusted-bundle gates
- `🌎️hub/🧪️tests/🎁️trusted-stdio-gis-bundle-check`: trusted stdio and GIS bundle gate (source, native, process, browser, two-author)
- `🌎️hub/🧪️tests/🏛️trusted-catalog-fixtures/🚀️stdio-gis-bootstrap-and-compiled-dependencies-fixtures` and `🗺️gis-retained-collaboration-and-publication-fixtures`: trusted catalog fixtures
- `🌎️hub/🔬️bin-unit`: `🚀️boot/🔬️unit/🧭️gis-map-proposal-and-inference-laws` and `💡️gis-map-approval-and-inference-fixtures`
- `🌎️hub/🧩️compositions/🌍️gis/🧪️tests`: composition tests (durable assembly, native codecs, inference control, hub edit durability, cold map patch, io boundary)

Note: `🎮️gis-map-process-support`, `🌍️gis-map-fixtures` and `🕹️gis-map-process` have no `project.json`. There is no `gis-map-undo` folder under `🌎️hub/💡️inference/🪶️sqlite`. The closest matches are `🌎️hub/🧩️compositions/🌍️gis/💡️inference` and the sqlite snapshot targets in `@semio-tech/gis-gismap-rs`.

### s-side acceptance and playground
- `✏️s/🧪️tests/🏘️plugin-composition/🗺️gismap-inference-panel`, `🗺️gismap-shell-history-route`, `🗺️gismap-mounted-probe`: GIS UI composition probes. No owning `project.json` was found. They are not referenced by any target.
- `✏️s/🧑‍💻dev` targets: `composition-check`, `typecheck`, `catalog-check`, `source-check`, `test-catalog-publication`, `test-catalog-exports`, `check-generated`, `generate`, `mcp-client-e2e`. None is GIS-specific, but `typecheck` and `test-catalog-publication` depend on or read GIS inputs.
- Playwright: no `playwright.config*` exists in the repo. Browser checks run through the hub `*-browser-check` targets above.

Other projects that mention GIS (not GIS-specific): `✏️s/🔌️plugins/✒️writer` (`@semio-tech/writer-writer`), `✏️s/🔌️plugins/🗄️stdio/...` (`stdio-*` contracts), `🎓️teaching/🏛️architecture/📚️course`, `🏢️semio-tech/🎡️play` (`@semio-tech/semio-tech-play`), `♻️mit-bestand/🧺️demonstrator`, and the framework plugin/registry modules.

## 3. Dev launch of the s shell with GIS

There is no `.claude/launch.json` and no `serve` target for the s instance. Launch goes through the `repo` binary:

- Shell (catalog default `s`, with GIS contribution): `repo dev playground s react prototype`
  - Ports: react 6070 (userPorts 6072, 6073), wgpu 6066 (userPorts 6067, 6068)
  - Needs the hub at `http://127.0.0.1:8787` (`dataDir .🧬semio/🔗space/s-dev`)
- GIS 2D editor: `repo dev playground gis2d react prototype`
  - Ports: react 6040, wgpu 6140. App `s.gis.gismap@1/*#editor`. Native host `semio-s-gis-services-native`
- GIS 3D editor: `repo dev playground gis3d react prototype`
  - Ports: react 6083, wgpu 6183. App `s.gis.gisterrain@1/*#editor`
- Use `wgpu` or `native` instead of `react` for the other renderers. `release` instead of `prototype` for release builds.
- Hub, if it is not already running: `repo run os-hub:dev` (its `dev` target, port 8787). Secure variants: `dev-secure-suite`, `dev-secure-native`, `dev-secure-mcp`.
- The root `os` and `semio` targets are not launchers. `os` is headless (`os run <bundle>.studio`). `semio` is `inspect|verify|open|convert`.

## 4. End-to-end verification plan (to run, not run here)

Run in this order (`repo run <project>:<target>`):
1. `@semio-tech/gis-gismap-rs:test` and `@semio-tech/gis-gisterrain-rs:test`
2. `@semio-tech/gis-gismap-js:test` and `@semio-tech/gis-gisterrain:test`, plus `@semio-tech/gis-js:test`
3. `@semio-tech/gis-plugin:test` and `@semio-tech/gis-plugin:native-codec-check`
4. `@semio-tech/s-gis-services-native:native-build` (prototype), then `:mcp-build`, `:user-path-check`, `:security-check`
5. `os-hub:component-codec-check` and `os-hub:gis-map-proposal-process-check`
6. `os-hub:trusted-stdio-gis-bundle-process-check` (and `:trusted-stdio-gis-bundle-browser-check` for the browser path)
7. `@semio-tech/s-dev-composition:typecheck` (depends on the GIS builds)
8. Launch: start the hub (`os-hub:dev`), then `repo dev playground gis2d react prototype` and `repo dev playground s react prototype`. Open the gismap at the react port (6040 for gis2d, 6070 for s). Check that the `s.gis.gismap@1` editor and `s.gis.gisterrain@1` editor mount.

Caveat: the `gis-map-proposal-process-check` and `trusted-stdio-gis-bundle-process-check` gates drive a local hub and browser sockets. Start the hub before running them. This is inferred from the test sources (they wait for hub readiness and document sockets) and was not run.
