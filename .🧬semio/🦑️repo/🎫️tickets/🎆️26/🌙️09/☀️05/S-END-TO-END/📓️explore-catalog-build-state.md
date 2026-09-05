# Explore: `s` plugin catalog build state (React OS shell)

Generated: 2026-09-05 (Sonnet read-only explorer). Census of `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` (59 rows: 33 plugins + 26 extensions) cross-referenced against the dev shell's built-module cache (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/` and `🧩️extension-modules/`) and each plugin/extension source root under `✏️s/🔌️plugins/`.

## 1. Per-row census (all 59 registry rows)

"staged descriptor" = both `🔣️.json` and `🛂️.descriptor.semio` present in the cache dir for that id. "owner descriptor pair" = both files present at the plugin/extension *source* root (`<cratePath>/../..`), per `DESCRIPTOR_JSON_REL_PATH = "../../🔣️.json"` (`🔌️plugin/📇️registry/📜️script.ts:201`, sourced from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:3848`).

| pluginId | role | source root | core wasm (cache) | mtime | component JS | staged descriptor | owner descriptor pair | playground variant(s) | explicit app id |
|---|---|---|---|---|---|---|---|---|---|
| animate | plugin | `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust` | Y | 2026-08-18 20:26 | Y | Y | Y | animate | - |
| architect | plugin | `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust` | Y | 2026-08-18 20:44 | Y | Y | Y | architect | - |
| block | plugin | `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust` | Y | 2026-08-18 20:48 | Y | **N** | **N** | block2d, block3d, block5d | s.block.block{2d,3d,5d}@1/*#editor |
| cad | plugin | `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust` | Y | 2026-09-05 02:31 | Y | Y | Y | cad | s.cad.cad@1/*#editor |
| cad-extension-aec-building | extension | `📐️cad/🧩️extensions/🏢️aec-building` | Y | 2026-08-18 21:03 | Y | Y | Y (placeholder) | - | - |
| cad-extension-aec-building-energy | extension | `📐️cad/🧩️extensions/🔥️aec-building-energy` | Y | 2026-08-18 21:06 | Y | Y | Y (placeholder) | - | - |
| cad-extension-aec-building-structure | extension | `📐️cad/🧩️extensions/🏛️aec-building-structure` | Y | 2026-08-18 21:07 | Y | Y | Y (placeholder) | - | - |
| cad-extension-spatial-shape | extension | `📐️cad/🧩️extensions/📐️spatial-shape` | Y | 2026-08-18 21:08 | Y | Y | Y (placeholder) | - | - |
| dag | plugin | `🕸️dag` | Y | 2026-08-17 17:56 | Y | Y | Y | dag | - |
| demonstrator | plugin | `🎪️demonstrator` | Y | 2026-08-27 07:56 | Y | Y | Y | aggregator, aussuchen, bearbeiten, demonstrator, generator, koordinator, verfolgen | borrows cad/playground/gismap/generation3d/process3d/puzzle3d/curation editors |
| **draw** | plugin | `🖍️draw` | **N** | - | **N** | **N** | Y | draw | - |
| **energy** | plugin | `🔋️energy` | **N** | - | **N** | Y | Y | energy | s.energy.model@1/*#editor |
| fem | plugin | `🏗️fem` | Y | 2026-08-17 18:10 | Y | Y | Y | fem2d, fem3d | s.fem.fem{2d,3d}@1/*#editor |
| flow | plugin | `🌊️flow` | Y | 2026-08-18 14:47 | Y | Y | Y | flow | - |
| flow-extension-bim | extension | `🌊️flow/🧩️extensions/🏗️bim` | Y | 2026-09-03 12:52 | Y | **N** | **N** | - | - |
| flow-extension-brep | extension | `🌊️flow/🧩️extensions/📐️brep` | Y | 2026-09-03 12:53 | Y | Y | Y | - | - |
| flow-extension-dictionary | extension | `🌊️flow/🧩️extensions/📖️dictionary` | Y | 2026-09-03 12:54 | Y | Y | Y | - | - |
| flow-extension-draw | extension | `🌊️flow/🧩️extensions/🖍️draw` | Y | 2026-09-03 12:55 | Y | **N** | **N** | - | - |
| flow-extension-list | extension | `🌊️flow/🧩️extensions/📃️list` | Y | 2026-09-03 12:56 | Y | Y | Y | - | - |
| flow-extension-logic | extension | `🌊️flow/🧩️extensions/🧠️logic` | Y | 2026-09-03 12:57 | Y | Y | Y | - | - |
| flow-extension-math | extension | `🌊️flow/🧩️extensions/🧮️math` | Y | 2026-09-01 03:39 | Y | Y | Y | - | - |
| flow-extension-primitive | extension | `🌊️flow/🧩️extensions/🔤️primitive` | Y | 2026-09-01 03:39 | Y | Y | Y | - | - |
| flow-extension-text | extension | `🌊️flow/🧩️extensions/📝️text` | Y | 2026-09-01 03:39 | Y | Y | Y | - | - |
| forms | plugin | `📋️forms` | Y | 2026-08-17 21:20 | Y | Y | Y | forms | - |
| gis | plugin | `🌍️gis` | Y | 2026-08-27 15:50 | Y | Y | Y | gis2d, gis3d | s.gis.gismap, s.gis.gisterrain |
| imperative | plugin | `📜️imperative` | Y | 2026-08-07 13:33 | Y | Y | Y | imperative | - |
| imperative-extension-{control,effect,logic,math,text} | extension | `📜️imperative/🧩️extensions/…` | Y | 2026-08-17 18:23-25 | Y | **N** | **N** | - | - |
| **layout** | plugin | `📏️layout` | **N** | - | **N** | **N** | Y | layout | - |
| lowpoly | plugin | `💠️lowpoly` | Y | 2026-08-17 18:29 | Y | Y | Y | lowpoly | - |
| mathematical | plugin | `➗️mathematical` | Y | 2026-08-17 18:30 | Y | Y | Y | mathematical | s.mathematical.equation |
| norm | plugin | `📕️norm` | Y | 2026-08-17 18:31 | Y | Y | Y | 15 variants | s.norm.*@1/*#editor (15 apps) |
| note | plugin | `🗒️note` | Y | 2026-08-18 03:27 | Y | Y | Y | note | - |
| playbook | plugin | `📖️playbook` | Y | 2026-08-17 18:35 | Y | **N** | **N** | playbook | - |
| playbook-module-procedural | extension | `📖️playbook/🧩️extensions/🌀️procedural` | Y | 2026-08-17 18:35 | Y | **N** | **N** | - | - |
| procedural | plugin | `🌀️procedural` | Y | 2026-09-01 11:06 | Y | Y | Y | generation2d, generation3d | s.procedural.generation{2d,3d} |
| process | plugin | `🏭️process` | Y | 2026-09-02 14:23 | Y | Y | Y | process3d | s.process.process3d |
| process-extension-{concrete,metal,robotic,wood} | extension | `🏭️process/🧩️extensions/…` | Y | 2026-08-17 18:36 | Y | **N** | **N** | - | - |
| puzzle | plugin | `🧩️puzzle` | Y | 2026-09-03 16:06 | Y | Y | Y | puzzle2d, puzzle3d, puzzle5d | s.puzzle.puzzle{2d,3d,5d} |
| raster | plugin | `🖨️raster` | Y | 2026-08-17 18:40 | Y | Y | Y | raster | - |
| reasoning-mindmap | plugin | `💡️reasoning` | Y | 2026-08-17 18:41 | Y | Y | Y | reasoning-wires | - |
| remodel | plugin | `📸️remodel` | Y | 2026-08-17 20:02 | Y | Y | Y | remodel | - |
| s | plugin | `🪐️space` | Y | 2026-09-02 10:56 | Y | Y | Y | s | - |
| sequence | plugin | `🎬️sequence` | Y | 2026-08-18 14:40 | Y | Y | Y | sequence | - |
| shooting | plugin | `🎥️shooting` | Y | 2026-08-18 14:27 | Y | Y | Y | shooting | - |
| sourcing | plugin | `🪵️sourcing` | Y | 2026-09-01 12:30 | Y | Y | Y | sourcing | s.sourcing.curation |
| sourcing-module-{beams,slabs,windows} | extension | `🪵️sourcing/🧩️extensions/…` | Y | 2026-08-17 | Y | **N** | **N** | - | - |
| stdio | plugin | `🗄️stdio` | Y | 2026-08-18 11:14 | Y | **N** | **N** | - | - |
| trinity | plugin | `🔱️trinity` | Y | 2026-08-18 03:09 | Y | **N** | **N** | trinity-jack, trinity-rewriting | s.trinity.{jack,rewriting} |
| vcs | plugin | `🌿️vcs` | Y | 2026-08-17 18:48 | Y | Y | Y | vcs | - |
| writer | plugin | `✒️writer` | Y | 2026-08-17 18:48 | Y | Y | Y | writer | - |

Cache-dir sanity: `🔌️plugin-modules/` also holds `🧵️shard/🟨️shard-worker.js` (written by `publishShardWorker()`, `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:72-76`) and `🪞️vendor` (preview2 shim vendor dir from `ensurePreview2ShimVendor()`). Both are expected. Last hot-swap event (`♻️hot-swap.json`): `cad`. Last extension install event: `flow-extension-bim`.

**Bottom line: 57/59 rows have a real `.core.wasm` + component JS in the dev-served cache. Only `draw` and `layout` are completely unbuilt. 19/59 rows (4 plugins + 15 extensions) are built but carry no descriptor because their source root has no owner descriptor pair. `energy` has a staged descriptor but no wasm/JS.**

## 2. `plugin-registry:check`

The run (`bun nx run @semio-tech/plugin-registry:check --skip-nx-cache`) did not reach the descriptor/taxonomy gates. After ~20 minutes of a repo-wide walk it crashed with `ENOENT: scandir '…/target-block/debug/deps/rustcAWEOX6'` at `📚️library/🔍️discovery/🟦️.ts:8754` (`discoverCatalogPackages`, `:8769`). This is an environment race with a concurrent agent's isolated `CARGO_TARGET_DIR` (`target-block/`), not a catalog defect. `check` is not a reliable gate while sibling agents build into `target-*` roots under the repo root.

Static prediction from `validateDescriptors` (`📜️script.ts:1977-2033`): every entry with no `🔣️.json` pushes a **warning** (`:1984-1988`): `"<pluginId>: no <cratePath>/../../🔣️.json yet — run describe …"`. The 19 pluginIds: plugins `block`, `playbook`, `stdio`, `trinity`; extensions `flow-extension-bim`, `flow-extension-draw`, `imperative-extension-{control,effect,logic,math,text}`, `process-extension-{concrete,metal,robotic,wood}`, `sourcing-module-{beams,slabs,windows}`, `playbook-module-procedural`. Expected: 0 hard errors, 19 warnings.

## 3. Build commands the dev shell issues per plugin

All in `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`.

CARGO stage — `buildPluginCargo` (:963-972) via `pluginCargoArgs` (:102-107):
```
cargo rustc -p <packageName> --target wasm32-wasip2 --profile <wasm-dev|wasm-release> -- -C link-arg=-zstack-size=8388608
```
(`-C strip=none` when `SEMIO_PLUGIN_SYMBOLS=1`). `PLUGIN_WASM_TARGET = "wasm32-wasip2"` (:80). Profile via `selectComponentWasmProfile` (`📚️library/📦️packages/🟦️typescript/🟦️.ts:3222`), overridable by `SEMIO_PLUGIN_PROFILE`. Artifact: `<CARGO_TARGET_DIR|target>/wasm32-wasip2/<profile>/<package_name_underscored>.wasm` (:969-970).

MATERIALIZE stage — `materializePlugin` (:981-1005) → `transpilePluginComponentAsync` (`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:755-767`):
```
node @bytecodealliance/jco transpile <artifact> -o <plugin-modules/<pluginId>> --name <componentBase> --map semio:framework/pure=./🟨️.js --map semio:framework/host-async=./🟨️.js
```
then (ship only) wasm-opt (`🟦️.ts:722-738`), `describeBuiltPlugin` (:948), `stagePluginDescriptor` (:820-833, copies the owner-root pair into the module dir, **removing it if the owner root has none**), `publishBuiltExtension` (:860+).

`buildPlugin` (:1013-1017) = one crate end-to-end. `buildPlugins`/`buildPluginsStreaming` (:1187, :1207) → `buildPluginCatalog` (:1080-1114): cargo **strictly serial**, materialize in a 4-wide pool (`SEMIO_MATERIALIZE_CONCURRENCY`).

`SEMIO_PLUGIN_ONLY=<pluginId>` — `resolvePluginBuildTargets` (:1130-1146) forces exactly that crate even under the `s` host filter (single-crate hot-swap). Without it, host filter `s` returns the full entry list (:1141).

## 4. Ranked gaps preventing "every registered plugin loads in the React shell"

1. **`draw` and `layout` have zero build output** (valid owner descriptors in source, never built into the cache). The only two rows that cannot load at all.
2. **`energy` cache entry is a descriptor with no module** (`syncBuiltPluginDescriptors`, `:836-841`, re-stages descriptors regardless of module presence; `stagePluginDescriptor`, `:820-833`, never checks for the wasm). A false-positive trap for anything inferring loadability from descriptor presence.
3. **4 built plugins ship no descriptor: `block`, `playbook`, `stdio`, `trinity`.** They load via legacy Cargo `[package.metadata.semio] contributes` (`📜️script.ts:249-262`) without `hashes`/`executionMode`/`activationEvents`/`extensionPoints`.
4. **15 of 26 extensions share the no-owner-descriptor gap** (same mechanism).
5. **`plugin-registry:check` cannot police any of this** while concurrent `target-*` roots exist under the repo root.
6. (Informational) cached wasm predates newest source for 49/57 built entries, but deltas cluster into repo-wide sweep batches; not treated as confirmed staleness.
