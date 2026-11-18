# 🔎 Shooting React/TS Lane Audit

Read-only audit before serving the shooting artifact editor. Nothing was run; findings come from code reading and scripted static checks (import targets, named-export resolution through `export *`, bare `@semio-tech` specifier resolution against workspace `package.json` exports, en/de pairing across 327 JSON files, TS/Rust id parity, wasm artifact naming). Paths are repo-relative. `✏️s` = `✏️s/`, `🧰️framework` = `🧰️framework/`.

## Confirmed defects (prioritized)

### 1. HIGH: `serve` resolves its variant from an env var that the serve target never sets

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:52-53`: `playgroundCatalog().find(...)` then `throw Error("Declared playground variant required")` when empty.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts:69`: `playgroundCatalog = () => loadFrameworkOsPlaygroundCatalog(...)` with no argument.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/📚️catalog/🟦️.ts:8-9`: `catalogRoot=process.env.SEMIO_PLUGIN_CATALOG`; `if(catalogRoot===undefined)return [];`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:120` sets `env.SEMIO_PLUGIN_CATALOG` only for the `session` target. The `serve-*` and `dev-*` targets at `:224-225` (`repo ./📜️script.ts serve ...`) set no env, and `dependsOn` does not propagate env.
- No setter of `SEMIO_PLUGIN_CATALOG` exists for `serve` in `🧰️framework` or `✏️s`.

Effect: a graph-run `serve shooting react prototype` gets an empty catalog and throws at `serve/🟦️.ts:53` before any boot. It only works if an operator exports the variable by hand. Fix direction: pass the catalog root explicitly (the session target's `publication.outputRoot`) or set the same env on `serve-*`/`dev-*` targets.

Caveat: not executed. I found no other setter, but I did not inspect the `repo` launcher's process environment.

### 2. MEDIUM: silent fallback drops the shooting dev contribution

- `serve/🟦️.ts:61` `loadDevContribution(this.repoRoot, resolved)` returns `undefined` when the playground row lacks `devContribution`, because `resolvePlaygroundFilter` (`🔌️plugin/🏗️build/📋️plan/🟦️.ts:84-87`) falls back to `{ pluginId: filterPlugin }`.
- `serve/🟦️.ts:65` then uses `config: contribution ? ... : "../../🏗️builder/🌐️vite/🟦️.ts"` and never sets a `browserEntry`, so the generic framework config serves without `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🚀️entry/🟦️.ts`.

Effect: a declared variant whose catalog row is missing or stale boots an editor-less page with no error. Fix direction: throw when `variant` is declared in the catalog but `devContribution` is absent.

### 3. LOW: stale doc comment on the playground catalog

- `🔌️plugin/🏗️build/📋️plan/🟦️.ts:68` says the catalog is "backed by `framework/plugin/registry/generated/🎮️playgrounds/🟦️.ts`". The implementation at `:69` and in `📚️catalog/🟦️.ts:10-11` reads `🚀️playgrounds.json` from the env root. The TS generated module is not what gets loaded. Misleading for whoever fixes item 1.

### 4. LOW: dangling workspace excludes in the artifact Cargo workspace

- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml:22` and `:29` exclude `**/👽️guest/**`. No `guest` directory exists under the plugin (checked with `find`). Harmless, but it is an inconsistency the greenfield rules say to remove.

## Checked and consistent (no defect)

- **Relative and workspace imports**: every relative import in the serve script, the variant entry and vite builder, the catalog (`✏️s/🧑‍💻dev/🧩️catalog`), and the 92 in-scope TS/TSX/MJS files resolves to an existing file. Every `@semio-tech/*` specifier used in scope resolves to a workspace package whose `exports` map contains the subpath and whose target file exists (57 unique specifiers). Named imports from in-repo targets resolve through `export *` chains with no missing names.
- **Transitive closure** from serve and the variant entry: 3487 files visited. Remaining unresolved items are either gitignored generated outputs or false positives (noted below).
- **Serve call signatures** match their targets: `frameworkOsEngineFacetV1`, `frameworkOsViteArtifactInputsV1` (its JSON schema allows `renderer:"react"`, `command:"serve"`, `engines:"renderer"|"play"`), `loadDevContribution`, `resolveDevContributionFile`, `developmentRuntimeRoot`, `readActivationReceipt`, `reportServeStagedModuleFreshness` (wrapped in try/catch, so the `void` call is safe), `withDevLocalHubSessionV1`, `runViteBunxDev` (option keys match), `scriptCompilerAdmissionControlV1(invocation, command, ms)`, `BundleScript`/`ActivationScript` constructors.
- **Variant entry**: `bootFrameworkOsDev` options match its signature (`variant`, `admission`, `catalogRows`, `brands`). `admission` matches `PluginCatalogAdmissionV1` exactly. `composeSpecificOsCatalogV1(...).rows` is the right shape. The `VITE_SEMIO_PLUGIN === "shooting"` guard matches the serve env.
- **Vite builder**: `createFrameworkOsDevConfig` receives all four required keys (`inventory`, `brands`, `browserEntry`, `browserSessionFactories`). Relative paths to `🧬️schema/🔣️.json` and the repo root resolve.
- **Route and contribution ids**: Cargo `devContribution` (`🌎️hub/🧩️compositions/🎥️shooting/🧩️component/📦️packages/🦀️rust/Cargo.toml:22`) equals `ownerRoot`/`viteConfig`/`browserEntry` in `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🧬️schema/🔣️.json`. The variant directory is the owner root. `variant = "shooting"` (Cargo `:21`) matches the entry guard. `ports.react = 6019` (`:25`) is what serve reads. The schema declares no `localHub`, which matches the route test (`✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🟦️.ts:24`). The only route id is the asset route `/mesh` (Cargo `:29`), which has no counterpart in the variant schema because the schema defines no route ids. Nothing to reconcile.
- **Wasm artifact names**: discovery derives `wasmOut` as `packageName.replace(/-/g, "_") + ".wasm"` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts:278`). The shooting component crate `semio-hub-shooting-component` (Cargo `:3`) therefore expects `semio_hub_shooting_component.wasm`, which is what cargo emits for that lib name. The browser wasm packages use explicit `wasmBaseName` values that match their loader specifiers: `framework_surface`, `framework_editor`, `flow_core`, `framework_os_node_graph`.
- **en/de text**: all 327 JSON files parse. 228 objects carry both `en` and `de`, and none carries only one. The TS text keys are `bodyKey`/`surfaceId` constants (`shooting.play.scene`, `shooting.play.icon`, `shooting.view.scene`). Each matches the literal in its Rust sibling (`🦀️.rs` `BODY_KEY`/`SURFACE_ID` constants) exactly. The Rust labels use `native_en`/`native_de`. No TS module references a label key missing from the schemas.
- **Shooting plugin TS**: the plugin contains 0 TSX files and 55 TS files. The React lane therefore lives in framework renderer code, not in shooting.

## Uncertain (needs a runtime or generation check)

1. **Gitignored generated inputs.** `✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts:1-2`, `🏗️builder/🟦️.ts:1-2`, `📦️packages/🟦️typescript/📚️catalog/*`, `🧊️wgpu/🟦️.ts` import `./🤖️generated/🧩️plugins/🟦️.ts`, `🎮️playgrounds/🟦️.ts`, and `🦀️catalog.json`. These are ignored by `.gitignore:97-98` and do not exist in the tree. Same for framework `🤖️generated/🪪️manifest`, `🎭️actor`, `📜️ui-contract`, and `🐚️shell`. Boot works only after the generators run. The serve target depends on `session-*`, which should run the generator, but I did not confirm the chain end to end.
2. **wasm-bindgen outputs.** Host ports `🪝️host-ports/🕸️wasm-bindings/🟦️.ts` import `@semio-tech/framework-surface-rs`, `framework-editor-rs`, `flow-core`, `framework-os-node-graph-rs`. Their `🕸️bindings` and `pkg/` directories are absent (`.gitignore:111-113`). They are produced by `buildAdmittedWasmWebV1`; names match, but presence depends on the wasm build.
3. **Vite query import.** `🪝️host-ports/🟦️.ts` imports `@semio-tech/plugin-registry/hub-source/service-worker?worker&url`. My exports check rejected it because the query suffix is not in the map. Vite strips queries before resolution, so this is probably fine.
4. **Budget `0` in serve.** `serve/🟦️.ts:62` passes `requestedMilliseconds = 0` to `scriptCompilerAdmissionControlV1`. Sibling calls in `withDevLocalHubSessionV1` use `0` to mean "inherit parent", so this is probably intentional, but I did not verify `controlFor` for this exact input.
5. **Test-only raw imports** (`?raw` in `🎬️owned-scene` fixtures). Out of the serve path.

## Scope note

Checks ran over `✏️s/🔌️plugins/🎥️shooting`, `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting`, `✏️s/🧑‍💻dev/🧩️catalog`, and the serve script. The framework transitive closure was walked as well because serve depends on it. Scripts used for the scans are in the ticket folder (`🧰️scripts-import-check.mjs`, `🧰️scripts-names-check.mjs`, `🧰️scripts-bare-check.mjs`, `🧰️scripts-closure-check.mjs`, `🧰️scripts-i18n-check.mjs`).
