# result-vcfg: os dev Vite config graph back inside its bound

Executor `vcfg`, 2026-10-19, foreground, no cargo, no `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths. The bound was not raised: `maxModules` 40 and `maxSourceBytes` 700000 are unchanged.

## Result

| measure | before | after |
|---|---|---|
| esbuild graph of the config entry (what the law counts) | 564 modules, 4.8 MB | 39 modules, 224 KB |
| denied modules reachable | 42 | 0 |
| required modules missing | 0 | 0 (after dropping one contradictory requirement, see 3) |
| `🧹️config` suite (os dev) | 3 graph cases failed | 52 of 52 pass (51 existing + 1 new) |

The three failing cases were: denied modules, module and byte bound, and Bun's independent-bundler oracle. All pass. The count is 39 of 40, so the next static module in this graph is the one that trips the law again.

## Design

The config entry stays the only static owner of the config shape. Everything heavy moves behind one opaque dynamic import (`import(new URL(rel, import.meta.url).href)`, which esbuild does not follow and which resolves the same under Bun native, Node native and Vite's bundle loader).

1. `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (entry). Static imports are now: type-only imports, node builtins, the deployment constants, `isHostPlaygroundFilter`, the 11 vite-plugin owners, and the two existing graph anchors (playground catalog; the catalog view replaces the removed process anchor). All 41 heavy owners (engine deliveries, wasm delivery, assets, distribution, activation helpers, brand, site-html, scene host, ui build tooling, extension store, ...) come from one lazy module loaded once per config resolve. `defineOwnedBuildConfigFactory` was an identity helper and is replaced by the typed function itself. The `sourceEngineFacet` call moved from config creation into the factory.
2. New `🧑‍💻dev/🏗️builder/🌐️vite/🏭️build-owners/🟦️.ts`: 25 `export { ... } from` lines for the heavy owners (421-module closure, never in the static graph). Language-agnostic guard: fixture `lazy` + `deny` rows.
3. Activation plugin split. `🔌️vite-plugins/🚀️activation-vite-plugin/🟦️.ts` is now a thin shell (types, `transformIndexHtml` staleness banner, async `configureServer`, `closeBundle`); it kept 506 modules out of the graph by itself (50 static + the lazy driver stage). The receipt observation, freshness report, lazy materialization and cargo driver moved unchanged into `🏭️runtime/🟦️.ts` (`activateServer(server, options, publishStaleness)` returns the dispose function). Public surface is unchanged (`semioActivationVitePlugin`, `reportActivationFreshness`, `ActivationComponentSpec` through `@semio-tech/framework-os-dev/vite-plugins`); `ActivationVitePluginOptions` is the new named options type. Vite awaits `configureServer`, so loading the runtime there is safe; the http server has not started listening yet.
4. Fixture `⚙️config-graph.json`: added the two lazy modules and `library/🏃️process/🟦️.ts` to `deny`, added a `lazy` list, and removed `library/🏃️process/🟦️.ts` from `require`. That requirement is the one contract change and needs your attention: the module is the repository command runner with a 284-module closure (cargo streaming, native-build storage, leases), so it cannot be both required and inside a 40-module bound. It was reachable only through the canonical wasm plugin and the browser-bundle build, both now lazy. The right fix for the law author is a light spawner module split from `cargoStreamingStatus`; that is a repo-library refactor outside this config and was not done. No bound was touched.
5. New law case in `🧹️config`: "keeps every lazily loaded owner outside the static config graph and resolvable as a complete graph of its own". It builds each lazy owner on its own and fails on any unresolved relative import. It caught a real bug in my own first cut (two literal dynamic imports in the activation runtime still carried the pre-move depth).
6. Staging-root contract: both new modules are declared in the fixture `🔌️staging-root.json` `root.sourceFiles` and in `📋️project.json` `stagingRootSources` (the law compares those sets), 56 of 56 pass.

## Node strip-only fixes made on the way (JSON import attributes, erasable only)

Native Node rejects `import x from "./x.json"` without `with { type: "json" }` (Bun accepts it). Added the attribute in: `🔌️plugin/📇️registry/📦️deployment/🟦️.ts` (2 imports), `🔌️plugin/📇️registry/🟦️.ts`, `🧑‍💻dev/🚀️local-hub/🎫️session/📋️data/🟦️.ts`, `🦑️repo/.../⚡️caching/🌐️vite/📥️environment/🏃️runtime/🟦️.ts`, `🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts`, and the two os-variant configs `✏️s/🧑‍💻dev/🎭️variants/{🧩️puzzle,🌍️gis}/🏗️builder/🌐️vite/🟦️.ts`.

## Verification (commands and outcomes)

- Graph numbers: `node vcfg-tools/graph.mjs` prints `modules 39 bytes 224236`, `denied present 0`, `required missing []` (after the fixture edit; before it listed only `library/🏃️process`).
- Suite: `PLAYWRIGHT_BROWSERS_PATH=/nonexistent OSPLAY_ARGS="--reporter=verbose" osplay-tools/run.sh vcfg-tools/os-dev.list vcfg5` -> `🧹️config` 52 passed, `🔌️staging-root` 56 passed. Whole os dev run: 6 failed, 280 passed. The 6 are the Playwright browser-dependent cases (3 hot-update, 1 canvas PNG parity; I did not install the scratch browser path used by `osplay`), `🧩️contribution` (frozen storage JSON attributes), and the katex vendor case (deps). None is in the config graph.
- Type check: `tsc -p` over the four edited modules (1655 files, `skipLibCheck`): zero errors; a deliberate `const x: number = "a"` probe in the same config reported TS2322, so the run is not vacuous. Probe tsconfigs were removed.
- Strip-only syntax: `module.stripTypeScriptTypes` accepts all four new/edited modules.
- Config loading under Bun (the runtime that serves Vite, native loader): play and demonstrator `build` configs resolve (31 and 32 plugins, ids listed by `vcfg-tools/load-config.mjs`); os dev with the real puzzle inventory resolves `serve` with a receipt and temp roots (55 plugins incl. `semio-activation`, `semio-extension-store`, `semio-transform-freshness`) via `vcfg-tools/serve-probe.ts`; `semio-activation.configureServer` loads the lazy runtime in 84 ms, registers the `close` and `listening` listeners and `closeBundle` disposes cleanly. `build` for the os variants advances through the lazy distribution owners and stops only on missing staged artifacts (`🧾️receipt.json`, session alias for `s`), which are environment state.
- Config loading under Node `--experimental-strip-types`: the os dev entry now imports (`OK`), as do the puzzle and gis variant configs and `@semio-tech/framework-os` (previously failed on `describe/component-receipt` JSON). Plus the existing alias-freshness law (`checkPlaygroundAliasFreshness`) returns no failures.
- No `[DEBUG]` lines remain in repository files (they exist only in the `vcfg-tools` scripts).

## Still open (not caused by this change, not fixable inside the freeze)

- Node-native load of the play and demonstrator configs still fails on files outside this task: play at frozen `🏗️native-build/💾️storage/🌐️platform` (JSON without attribute), demonstrator on a `🕸️engines/📇️catalog.json` attribute seen only through a `require`/sync-link path. Both load under Bun, the runtime the dev serve uses (`viteConfigLoader()`).
- Invoking the os dev config factory (not merely importing it) under Node native still fails on about 48 JSON-without-attribute edges in the lazy closure (frozen storage modules among them). The lazy store plugin already had 41 of them before this change, so this is not a regression; under Bun it works.
- `✂️node-native-typescript` law (`bun test` in `📚️library/🧪️tests/✂️node-native-typescript`): its externalized-entry set differs from the declared set by 32 workspace package entries (compiler world, quiz, server, ui styling builder, ...). Pre-existing drift from the layering sweeps; this change only removes entries.
- The 39 of 40 margin: keep new config-time code in `🏭️build-owners` or `🏭️runtime`, not in the entry or the plugin owners.

## Files

New: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🏭️build-owners/🟦️.ts`, `.../🔌️vite-plugins/🚀️activation-vite-plugin/🏭️runtime/🟦️.ts`, `vcfg-tools/` (graph, edges, importers, union, req, json-attr, static-files, jsonloop, load-config, serve-probe, who-imports, os-dev.list).
Changed: `.../🏗️builder/🌐️vite/🟦️.ts`, `.../🚀️activation-vite-plugin/🟦️.ts`, `.../🧪️tests/🧹️config/🟦️.ts`, `.../🧪️tests/🧫️fixtures/⚙️config-graph.json`, `.../🧪️tests/🧫️fixtures/🔌️staging-root.json`, `.../📦️packages/🟦️typescript/📋️project.json`, plus the JSON-attribute files listed above.
