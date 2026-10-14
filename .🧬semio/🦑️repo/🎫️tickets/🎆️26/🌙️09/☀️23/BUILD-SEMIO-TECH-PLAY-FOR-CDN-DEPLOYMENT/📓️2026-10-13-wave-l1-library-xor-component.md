# 🧩️ Wave L1: Library XOR Component, Thin Component Crates (2026-10-13)

Lane C, wave L1 of `📓️2026-10-13-scalable-build-architecture.md` (law L5, §2.3, §7 L7, §10 L1). Builds on `📓️2026-10-13-lane-c-semio-split.md`, `📓️2026-10-13-wave-a2-single-profile.md`, `📓️2026-10-13-wave-n1-n2.md` and `📓️2026-10-13-wave-e2-plugin.md`.

Rules kept:

- No git-modifying command, no worktree, no sub-agent, no ticket operation, `🗑️generated` untouched apart from this wave's own `🗑️generated/l1` logs.
- Every cargo invocation ran in the foreground through `🚦️cargo-slot.sh`, with the private `⚡️cache/l1/{target,build}` dirs (`l1-library-xor-component/🦀️cargo.sh`). Long passes ran as bounded 570–585 s chunks that resume from the private build dir. Exception: once peer edits to `os-kernel`/plugin roots invalidated every chunk before it reached the stdio crates (load average 100–125), the stdio test build ran as one slot-gated 2400 s pass launched in the background and awaited in the same turn.

## 1. Outcome

| Item | Result |
|---|---|
| Dual crate-type manifests | **85 → 0.** Every `crate-type` list in the repository is now either absent (rlib) or exactly `["cdylib"]`. |
| §2.3 libraries made rlib-only | All 8: `semio-framework-actor`, `-editor`, `-math`, `-os`, `-os-flow`, `-os-kernel`, `-surface`, `semio-s-spatial-kernel-semio-session`. The six of them that also publish a wasm-bindgen browser module (`editor`, `surface`, `actor`, `os`, `os-flow`, the spatial session) got a `🧩️component` crate for that module (§3.3); `os-kernel` and `math` simply lost their dead cdylib. |
| Hub crates split | **44 → 44 rlib libraries `semio-hub-<x>` + 44 cdylib crates `semio-hub-<x>-component`**, each at `<owner>/🧩️component/📦️packages/🦀️rust` with a one-line root `<owner>/🧩️component/🦀️.rs`. |
| Extension crates split | 26 libraries + 26 `…-component` crates, same pattern. |
| Remaining dual manifests | 8 module owners split (library + browser-module component), 2 module-only crates and 2 test fixture components made cdylib-only, 3 libraries made rlib-only (§3.3). |
| Entry-selection features | **Deleted, not aliased:** `plugin-entry` (11 hubs), `plugin-root` (10 stdio crates), `extension-entry` (5 imperative extensions), `component-guest` (plugin crate, 9 flow extensions, scale fixture) and `component-extension-guest` (plugin crate). `semio-hub-stdio`'s own `component-app-assembly` went with `plugin-root`. |
| Component identity | `[package.metadata.component]`, the deployment keys of `[package.metadata.semio]`, every `[[package.metadata.semio.playground]]`/`[[…assets]]`, `[package.metadata.semio.sources]` and the wasm-pack profiles moved to the component crate. The committed descriptor pair (`🔣️.json` + `🛂️.descriptor.semio`, 70 pairs) moved into `<owner>/🧩️component/`, where `descriptorRelativePath = ../../🔣️.json` and the generated `descriptor_is_fresh` test already look. |
| Nx detection | `metadata.component` lives only on component crates. `componentBuildTargets` now refuses any component crate that is not cdylib-only. Measured graph: 71 `component` targets, all on `🧩️component` crates (70 plugin/extension components + the host test fixture); **0 on libraries**. Playground site targets (156, one per playground row) moved with the playground rows to the component projects. |
| Generators | The stdio composition generator (`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`) emits the 10 stdio component roots and publishes the playground regions into the component manifests; a second run changes 0 files. |
| Taxonomy | New kinds `component-crate` (🧩️ `^component$`, scoped to the owner kinds that carry a component facet) and `hub-compositions` (🧩️ `^compositions$` under `hub`), plus the registry `members-of-hub-compositions` (35 owners). 76 of 78 new facets resolve to `component-crate`; 0 of the 62 existing `🧩️component` directories change kind (§4). |
| G2 (L5 gate) | Added to `os-hub:unit-census-check`: `cargoNonComponentCdylibsV1` over the unit graphs plus a `cargo metadata` oracle. Live run: **0 non-component cdylibs**, 467 keys / 467 units at both profiles, no feature variance (2026-10-13), and 511 / 511 again on 2026-10-14. |
| L4 | 79 component crates: longest package name 46 (`semio-s-spatial-kernel-semio-session-component`), longest source path 193 UTF-16 units, worst-case uplifted path 233 (≤ 240). |

## 2. Design

### 2.1 One crate kind per role

- **Library** (`semio-hub-<x>`, `semio-s-plugin-<p>-<e>`, every framework crate): rlib only, no entry symbol, no entry feature, no component identity. It keeps its role (`hub`, `extension`, `framework`, …), its `[package.metadata.semio.preparation]` generator, its storybook roots, its tests and its `📋️project.json`/`📜️script.ts`.
- **Component crate** (`<library>-component`, `<owner>/🧩️component/📦️packages/🦀️rust`, `crate-type = ["cdylib"]`, `path = "../../🦀️.rs"`): depends on exactly its library and, for plugins and extensions, `semio-framework-plugin` (the export macro). Its root is one macro call, for example

  ```rust
  semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_wfc::plugin, semio_hub_wfc::WfcApps);
  ```

  It owns the deployable identity: `[package.metadata.component]`, `deployment-directory`, `component-kind`, `extends`/`contributes`/`depends-on`/`consumes`/`host`/`capabilities`, playgrounds, assets, artifact sources and the committed descriptor pair. Its inferred Nx project (name = package name) carries `component`, the playground site targets and the cargo `test` target that runs the generated `descriptor_is_fresh` law.
- The component is the only crate with an entry symbol, so the duplicate-symbol problem that `plugin-entry`/`plugin-root`/`extension-entry` worked around is gone by construction, and every embedding crate links plain libraries.

### 2.2 Why the jco module names did not change

The jco module base was `<package>_component`. It is now the component crate's own package name (`semio_hub_wfc_component`), which is byte-identical to the old base for every hub and extension. Changed in materialization (`componentBaseV1`), the trusted bootstrap module writer (hub script) and the browser-host staging. Only the 11 renamed extension libraries (§2.3) change their module file names.

### 2.3 Names (L4)

`<library>-component` is the one naming rule. Three libraries would have broken the 48-character limit (`…-flow-extension-dictionary-component` 50, `…-flow-extension-primitive-component` 49, `…-cad-aec-building-structure-component` 51). They were renamed rather than given an irregular component name:

- the nine `semio-s-plugin-flow-extension-<x>` → `semio-s-plugin-flow-<x>`, which also matches every other extension family (`cad`, `imperative`, `process`, `sourcing`, `playbook`);
- `semio-s-plugin-cad-aec-building-energy` → `semio-s-plugin-cad-aec-energy` and `…-aec-building-structure` → `…-aec-structure`.

Plugin ids, deployment directories and owner paths are unchanged.

### 2.4 `component-guest` in the plugin crate

The guest WIT bridge, the owned ABI and `component_export_anchor` were gated by `all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2")`. They are now gated by the target alone. This is exactly what workspace unification already selected for every wasm32-wasip2 unit, so the plugin crate compiles once per target; the export glue itself only exists where a component crate invokes `plugin_exports!`/`extension_exports!`. `component-guest-async` (a calling-convention selector no consumer enables) is unchanged.

### 2.5 Visibility the split needed

`removeExports` in the codemod opened exactly what the component names: wiring `mod plugin;`/`mod manifest;` became `pub mod`, extension `fn bundle()` became `pub fn bundle()` (dropping the imperative `#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))]` gates), and the flow extensions' `#[cfg(feature = "component-guest")] mod extension_guest` became `pub mod extension_guest` with its plugin dependencies made non-optional.

## 3. What changed

### 3.1 Manifests and sources (codemod `l1-library-xor-component/🧩️split.ts`)

- 70 library manifests rewritten, 70 component crates created, 140 descriptor files moved, 32 workspace member lists extended (hub, root, `🧰️framework`, `✏️s`, 26 extension workspaces, dev flow, dev composition), 32 lockfiles refreshed offline.
- `🔧️followups.ts`: plugin crate cfgs and features, every `features = ["component-guest"]`/`"semio-framework-plugin/component-guest"` consumer, the scale fixture, the registry rebuild guest checks, the 11 renames across sources, lockfiles, fixtures and taxonomy, and every `default-features = false` row that only switched an entry feature off.
- `🔁️paths.ts`: deliverable references moved to the component crate (`<owner>/🧩️component/📦️packages/🦀️rust/dist`, `<library>_component.wasm`, `cargoPackage: "<library>-component"`): 18 files including the hub trusted-bootstrap package table, the gis cold-map patch producer, the note/bim committed-descriptor smoke laws and the hub component-codec fixture.

### 3.2 Consumers updated by hand

- **Stdio:** generator (component roots, playground region, sources from the component manifest), library root (no cfg), catalog crate (component identity read from the component manifest), stdio script (component package/wasm/manifest constants, `catalog-root` build, editor-component check, editor catalog over component manifests), home-io-surface law and fixture **v3** (library feature-free, component cdylib-only depending on exactly library + plugin, component root is the export macro), editor-catalog fixture (`deployedComponents` = component crates), `shipped_fleet` (manifests of the component crates).
- **Registry projection:** owner publications declared on a library admit the bytes of the component in its `🧩️component` facet (`componentLibraryManifestV1`).
- **Nx plugin (`📚️library/🟨️.mjs`, region-local):** `componentBuildTargets` requires cdylib-only; `pluginSiteTargetsForCrate` runs the distribution build directly (no per-crate script) and `build` is an `nx:noop` union; site targets attach to inferred component projects. `registerPlaygroundSiteBuildCommands`/`playgroundVariantsFromCrateManifest` deleted with their 35 call sites.
- **wasm-bindgen modules:** `WasmPackWebBuildOptions.componentDir` (required) names the cdylib wasm-pack links; output stays in the owner's package. All 11 builders pass it. The wgpu trunk build links the renderer's component crate (`componentRoot`), so its artifacts are `semio-framework-os-renderer-wgpu-component{.js,_bg.wasm}`; the browser host, server, renderer boot and serving fixture follow. The 9 module owners declare their component crate in `namedInputs.nativeSources`.
- **Extensions:** `runExtensionComponentPackage({ componentDir })` packs the component crate; `.sxt` outputs moved to the component `dist`.
- **Fresh component producer:** `rootCdylib` deleted (component crates are cdylib-only); the request builds `cargo build -p <component> --lib`.
- **Tests and fixtures:** hub compositions ownership law (component crate + rlib library pair), space/reasoning identity laws and fixtures, gis/vcs native-codec laws, norm component budget, trusted-catalog profile, graph native-prerequisites, distribution output tests, wasm-build tests (cwd = component dir), trunk tests, unit census fixture (`crateTypeCases`).
- **Launch entries:** `semio-hub-demonstrator-component:component` and `semio-hub-draw-component:component` in both launch files. No new command was added (G2 runs inside the registered `os-hub:unit-census-check`).

### 3.3 The non-hub dual manifests

| Crate | Action | Why |
|---|---|---|
| `semio-framework-os-kernel`, `-math`, `-os-shell` | rlib only | No browser module; the cdylib was dead. |
| `semio-framework-editor`, `-surface`, `-actor`, `-os`, `-os-flow`, `-os-node-graph`, `-os-renderer-wgpu`, `semio-s-spatial-kernel-semio-session` | rlib library + `<name>-component` browser module crate (`extern crate <lib> as _;`) | Each is a Rust dependency of other crates **and** a published wasm-bindgen/trunk module. |
| `semio-s-plugin-trinity-jack-lsp`, `semio-s-dev-cad` | cdylib only | Browser modules nothing links as a library. |
| `semio-framework-plugin-host-test-component`, `semio-framework-os-scale-fixture` | cdylib only | Test guest components nothing links as a library. |

## 4. Taxonomy admission

Measured with `l1-library-xor-component/🔎️component-kind-impact.ts` against the live taxonomy:

- A parentless `component-crate` kind would have re-classified 61 of the 62 existing `🧩️component` directories (test cases, UI contract facets), so the kind is scoped to the owner kinds that carry a facet: `members-of-hub-compositions`, `members-of-plan`, `members-of-modules`, `members-of-operating-system`, `members-of-elements`, `distribution-puzzle`, `stdio`, `text`, `windows`.
- `🧩️compositions` and its 35 owners were unadmitted before this wave; `hub-compositions` + `members-of-hub-compositions` admit them.
- Result: 0 existing directories change kind; 76/78 facets resolve to `component-crate`. The renderer's facet under `🎯️targets/🧊️wgpu` resolves to the existing `members-of-wgpu-target` (the UI wgpu target already owns a `🧩️component` member; scoping `component-crate` to `wgpu-target` would re-classify it). The spatial-session facet stays unresolved because its owner chain (`🧠️semio/🌊️session`) is itself unadmitted.
- Both taxonomy loaders (discovery and normalization) load the file.

## 5. Verification

Every row below was run. "13th" rows ran on 2026-10-13 right after the codemods; "14th" rows re-ran on 2026-10-14 against the tree after about 24 h of peer work.

| Check | When | Result |
|---|---|---|
| `os-hub:unit-census-check` (`bun test 🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts`, private dirs, slot-gated) | 14th | **3 pass / 0 fail, 148 expects.** 511 unit keys = 511 compile units at `wasm-prototype` and at `wasm-release`, feature variance in none, G2 (`cargoNonComponentCdylibsV1` + `cargo metadata` oracle): 0 non-component cdylibs. On the 13th: 467 / 467. |
| Hub lockfile | 14th | The census first refused `--locked`: the Codex fleet and the owned-runtime lane had added first-party crates (fonts, `icon-contract`, `ui-*`, `os-store-*`, `plugin-guest-contract`, `plugin-interpreter-runtime`, …) without resyncing `🌎️hub/Cargo.lock`. Resynced offline with `cargo metadata --offline` three times; each diff had 0 `registry`/`checksum` lines, so no third-party version moved. |
| Hub workspace `cargo check --target wasm32-wasip2 --keep-going` | 13th | rc 0, 405 units. |
| Same | 14th | **Blocked upstream of every hub crate**: 7 framework crates fail, none from this wave: `semio-framework-ui-theme`, `-ui-layout`, `-os-pack`, `-os-command`, `-os-store-message-ledger` (new Codex crates with missing dependencies such as `serde_json`, `ui_contract`, `ToValue`), `-canvas` (foreign impl after `IconName` moved to `icon-contract`) and `-job` (borrow error in a file edited at 16:01). 154 units checked, 0 `semio-hub-*` reached. The Codex report says these leaves are not compiler-qualified yet. |
| Hub workspace native check | 13th | rc 0 right after the codemod. Later runs on the 13th were red only in peer in-flight crates (`os-kernel` store seams, `plugin-host` owned-runtime split, artifact `host/owned` traits). |
| nx graph | 13th | 71 `component` targets, all on `🧩️component` crates, 0 on libraries; 156 playground site targets on component projects. |
| Registry source discovery | 13th | 70 components found in their facets. |
| Hub compositions ownership law | 13th | 32 pass (component crate cdylib-only + rlib library). |
| Stdio production-preparation, distribution output, wasm-build (cwd = component dir) | 13th | 2, 1 and 9 pass. |
| Stdio composition generator | 13th | Second run changes 0 files. |
| Editor browser module through its component crate | 13th | wasm-bindgen exports survive the `extern crate … as _;` root (3,699 exports). Framework module components and the spatial session component: `cargo check --target wasm32-unknown-unknown` rc 0. |
| L4 budget (`📏️path-budget.ts`) | 13th | 79 component crates, longest name 46, longest source path 193, worst uplifted path 233. Each component crate is one source file of one line, far below the new 4,000-line / 262,144-byte / 32-file admission ceiling. |
| Taxonomy impact | 13th | 76/78 facets admitted as `component-crate`, 0 existing directories re-classified. |

Not completed. Each was attempted, and each was blocked by peer breakage, not by this wave:

- `cargo test -p semio-hub-stdio --test editor_catalog --test shipped_fleet`: on the 13th the build reached the stdio crates once, then hit a rustc ICE (exit 101 under `-Z threads=8`), then the `os-kernel` store and plugin crate broke mid-edit (8 retries, 16:11–16:37). The run on the 13th (lane-c split, before L1) was 177/177.
- Plugin manifest sweep (`🔎️manifest-sweep.py`): 4 of 320 rows ran before the same breakage.
- Small `workspace:release` driver run (`📜️script.ts demo note stdio-binary stdio-media imperative-extension-math`): the cold chunk compiled 131 units in a private generation. The resume then failed in `os-kernel`. The generation was deleted with the private dirs.

Fallout fixed on the 14th:

- A peer rewrite of the plugin manifest had restored `component-guest = []` and `component-extension-guest = []`. Both were deleted again; nothing enables them.
- The scale fixture's dead `[features] component-guest` was deleted too.
- Two doc comments that still named `component-guest` were reworded.

Open follow-ups for L2/WS1:

- `browser-publication` (spatial session) and `session-bindgen` (surface) are still consumer-selected first-party features. Per L2 they become adapter crates.
- The renderer's `🧩️component` facet resolves as `members-of-wgpu-target`.
- The spatial-session facet stays unadmitted until `🧠️semio/🌊️session` is admitted.
- The `🏗️native-build` fixture uses `component-guest` only as the name of a synthetic test feature. It is unrelated to the plugin crate and is unchanged.

## 6. Inputs (kept in `l1-library-xor-component/`)

| File | Purpose |
|---|---|
| `🗺️inventory.ts` | classifies every dual manifest |
| `🧩️split.ts` | library/component codemod |
| `🔧️followups.ts` | feature deletion, renames, default-features cleanup |
| `🔁️paths.ts` | deliverable path codemod |
| `🔎️taxonomy-kinds.ts`, `🔎️component-kind-impact.ts` | taxonomy resolution and impact |
| `📏️path-budget.ts` | L4 budget |
| `🔎️manifest-sweep.py` | plugin manifest sweep (native + wasm32 for component crates) |
| `🦀️cargo.sh`, `🌎️hub-check.sh` | slot-gated private cargo, hub workspace check |
| `📜️script.ts` | `demo`/`resume` release driver run, `script` owner command |
| `🔒️workspaces.txt` | workspaces whose lockfiles were refreshed |
