# BIM plugin: anatomy and registration checklist (r1, read-only exploration)

Scope: how a plugin is laid out and registered end to end in `C:\git\semio`, studied on `🕸️dag` (small), `🌍️gis` (mid), `🖍️draw` (rich), with `🗒️note` as the cross-check. No repo file was modified. Only this report was written. The repo MCP (`repo`) and `semio` MCP servers were not connected in this session, so no ticket was opened through them; the ticket folder was created by hand at the path above.

## 0. Corrections to the brief (read first)

| Brief said | Reality (verified) |
| --- | --- |
| Rust crates named `semio-s-plugin-<name>` | Only extensions use that prefix (`semio-s-plugin-flow-extension-bim`, `semio-s-plugin-cad-aec-building`). Plugin crates are `semio-s-artifact-<plugin>-<artifact>` (`semio-s-artifact-note-note`, `semio-s-artifact-gis-gismap`). The WASM component crate is `semio-hub-<plugin>` in `🌎️hub`. Bridges are `semio-<plugin>-mutation-bridge`. |
| Framework path `🧰️framework/🛒️products/💻️os` | Correct path is `🧰️framework/🛍️products/💻️os` (`🛍️` not `🛒️`). |
| `🎛️apps` is a plugin child dir | Legacy. Only `🎞️animate` still has `🎛️apps`. New work uses `✏️editor` / `👁️viewer` surfaces under each subset (taxonomy `_surfaceComment`). Do not create `🎛️apps`. |
| `🧩️extensions`, `📦️packages`, `🧬️schema` at plugin root everywhere | Only present where needed. `🧩️extensions` exists in 7 plugins (flow, cad, process, sourcing, playbook, imperative, energy). Plugin-root `📦️packages` exists in gis, dag, draw (TypeScript test package only); note has none. |
| WASM component named like `semio:architect` | Yes: `[package.metadata.component] package = "semio:<plugin>"`. Output file is `semio_hub_<plugin>.wasm`. |
| `🎮️commands` as a folder with commands | Every plugin has `🎮️commands/📌️.empty.md` ("This owner currently declares no commands."). It is a required placeholder (taxonomy `pluginRequiredChildDirs = ["🎮️commands"]`). |

Prior art the lead should know about before naming the plugin: `🌊️flow/🧩️extensions/🏗️bim` (crate `semio-s-plugin-flow-extension-bim`, descriptor `semio:flow-extension-bim`), and `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/🏠️bim` (crate `semio-hub-stdio-bim`, depends on `stdio`). Stdio also has an IFC artifact `🗿️artifacts/🏗️ifc` (crate `semio-s-artifact-stdio-ifc`). The BIM plugin will overlap these.

## 1. Plugin directory skeleton

Census of `✏️s/🔌️plugins/` (34 plugin dirs + `🔒️policy-allowlist.json`). Presence of each child:

| Child | Used for | Present in (sample) |
| --- | --- | --- |
| `AGENTS.md` | Plugin spec, front matter `technology: <name>` / `emoji: <e>` then `# Title` | dag, gis, cad, animate, most others. Not in note, draw, raster, forms |
| `README.md` | Front matter `name: <name>` / `kind: user` then `# <name>` | gis (`🌍️gis/README.md`), note: no |
| `🎮️commands/📌️.empty.md` | Required placeholder for commands | all 34 |
| `🏭️bridge/` | Test-platform mutation bridge (own Cargo workspace) | all 34 |
| `🗿️artifacts/<artifact>/` | The artifacts (the actual document kinds) | all 34 |
| `📦️packages/🟦️typescript/` | Plugin-level TS owner-test package: `package.json`, `📋️project.json`, `📜️script.ts` | dag, gis, draw |
| `🔮️oracles/🔣️.json` | Plugin test-platform contribution (`schemaVersion 2`); holds the crate its adapters link, not an oracle | gis, draw |
| `🧪️tests/` | Plugin-level tests (e.g. `🧪️tests/🧪️mutation-inputs/🟦️.ts`) | gis, note |
| `🧫️fixtures/` | Plugin-level fixtures (e.g. `🧫️fixtures/🎛️mutation-inputs`) | gis, draw, note |
| `🧩️extensions/<ext>/` | Extensions of this plugin (own composition) | flow, cad, process, sourcing, playbook, imperative, energy |
| `🔨️modules/`, `🧬️schema/`, `⚙️engine/`, `🫀️core/`, `🌉️wasm/`, `🎯️targets/`, `🗂️catalog/` | Plugin-specific helpers, schema, engines | various |
| `📖️stories/` | Storybook stories | animate, fem, puzzle, block, remodel |
| `🎛️apps/` | Legacy, do not add | animate only |
| `🗟️artifacts/`, `🦀️.rs`, `🕸️manifest.json`, `📜️script.ts` | Oddities of `🗄️stdio` and `🌊️flow` | stdio, flow |

Required minimum for a new plugin (derived from the 34): `🎮️commands/📌️.empty.md`, `🏭️bridge/`, `🗿️artifacts/<≥1 artifact>/`. Recommended: `AGENTS.md`, `README.md`, `🧪️tests/`, `🧫️fixtures/`, `🔮️oracles/🔣️.json`.

Taxonomy constants (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`):
- `pluginAreas = ["✏️s/🔌️plugins"]`
- `pluginChildDirs = ["🎮️commands", "🔨️modules", "🧬️schema"]`
- `pluginRequiredChildDirs = ["🎮️commands"]`
- `artifactChildDirs = ["🧬️schema", "🚪️io", "📚️examples", "🔨️modules"]`
- `artifactsDirName = "🗿️artifacts"`, `standardsDirName = "🏅️standards"`, `standardDirPrefix = "🔖️"`, `subsetsDirName = "🪆️subsets"`, `subsetDirPrefix = "✳️"`, `subsetAnyDirName = "✳️any"`
- `subsetChildDirs = ["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"]`
- `subsetRequiredSurfaceDirs = ["👁️viewer", "✏️editor"]`
- `surfaceRequiredChildDirs = ["🎭️modes", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]`
- `modeRequiredChildDirs = ["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]`
- `windowRequiredChildDirs = ["🎬️actions", "🪛️utilities", "☑️options", "🎚️config", "👥️presence", "🫧️transient"]`
- `testsDirName = "🧪️tests"`, `testFixturesDirName = "🧫️fixtures"`, `testOraclesDirName = "🔮️oracles"`, `testBridgeDirName = "🏭️bridge"`, `testProbeDirName = "🔬️probes"`, `testGeneratorDirName = "🏭️generator"`
- `pathEmojiPolicy`: every git-visible file and folder needs exactly one emoji, unique among its siblings (`🔍️discovery`, root `📜️script.ts` ~line 18404).

Artifact tree (example `🗒️note/🗿️artifacts/🗒️note/`):

```
🗿️artifacts/<artifact>/
  🦀️.rs                                  # artifact lib root (cargo [lib] path)
  📦️packages/🦀️rust/                      # Cargo.toml, package.json, 📋️project.json, 📜️script.ts
  📦️packages/🟦️typescript/                # package.json, 📋️project.json, 📜️script.ts (optional)
  🔮️oracles/📦️packages/🦀️rust/Cargo.toml  # test oracle crate (role = "test"), 🔣️.json, 🦀️.rs
  🔨️modules/  🧪️tests/  🏅️standards/
  🏅️standards/🔖️1/🪆️subsets/✳️any/
    🦀️.rs
    🧬️schema/   🔣️.json 🔗️.graphql 🛰️.proto 🟦️.ts 🦀️.rs 📸️snapshot/ 🔺️diff/ 💡️inferences/ 🧬️mutations/ 🧪️tests/ 🧫️fixtures/
    🚪️io/       🦀️.rs 🟦️.ts 📝️text/ 💾️binary/ 🪶️sqlite/ 📥️import/ 📤️export/ 🧪️tests/ 🧫️fixtures/
    ✏️editor/   🦀️.rs 🟦️.ts 🎚️config/ 🎭️modes/ 🎮️commands/ 👥️presence/ 🪟️window/ 🫧️transient/ 🗣️terminology/ 📌️panels/ 🧵️retained/ 📚️examples/ 🧪️tests/
    👁️viewer/   (same shape, read-only)
    📚️examples/  🏭️generator/  🔬️probes/  🔮️oracles/  🖼️assets/  🧪️tests/  🧫️fixtures/
```

Subset-level `🔣️oracle.json` manifests (per-subset registration) are referenced by the plugin oracle comments; artifacts also carry `🧪️tests/⚡️quick`, `🧪️tests/🔬️unit`.

Plugin-level oracle comment (gis `🔮️oracles/🔣️.json`): "It carries NO oracle and NO catalog ... What lives here is the single thing a subset manifest cannot express — the crate this plugin's test adapters link."

## 2. Cargo

### 2.1 Workspaces (three independent roots)

1. `✏️s/Cargo.toml`: the workspace for all plugin artifact/extension crates. `[workspace.metadata.semio.repository]` and `resolver = "2"`. `cargo-features = ["trim-paths"]`. Root workspace `exclude`s `✏️s` and `🌎️hub`.
2. `🌎️hub/Cargo.toml`: the workspace for the WASM compositions (`semio-hub-*`), including the stdio extensions.
3. `🏭️bridge/Cargo.toml` (per plugin): each declares `[workspace]` itself, with the comment "Own workspace root: the repository root manifest is a shared leased file". Its own `Cargo.lock`.

Root `Cargo.toml`: `[workspace] members` (lines 5-126) contains framework crates only. `exclude` (lines 127-225) contains `"✏️s"`, `"🌎️hub"`, `"**/🏅️standards/**"`, `"**/🔮️oracles/**"` and about 90 explicit plugin-internal standard generator/probe and oracle crates (for example note's codec and oracle crates at lines 219-220).

Shared metadata: `[workspace.package] version = "0.1.0", edition = "2021", rust-version = "1.95"` (`✏️s/Cargo.toml` line 245); `[workspace.lints.rust]` / `[workspace.lints.clippy]` (line 589+). Each member sets `[lints] workspace = true`.

### 2.2 Member lines (copy the pattern)

`✏️s/Cargo.toml` members (sample):
```toml
    "🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust",
    "🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust",
    "🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust",
    "🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust",
    "🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust",
    "🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust",
```
Also add every artifact's `🏅️standards/.../🏭️generator/.../📦️packages/🦀️rust` and `🔬️probes/...` crate that exists (note line 236-237).

`✏️s/Cargo.toml` `[workspace.dependencies]` (lines ~250-402), one line per artifact and per extension:
```toml
semio-s-artifact-note-note = { path = "../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust" }
semio-s-artifact-dag-dag = { path = "../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust" }
semio-s-artifact-gis-gismap = { path = "../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust" }
semio-s-plugin-flow-extension-draw = { path = "../✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust" }
```

`🌎️hub/Cargo.toml`: members (`🧩️compositions/🗒️note/📦️packages/🦀️rust`, line 46) and `[workspace.dependencies]` (`semio-s-artifact-note-note = { path = "../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust" }`, line 99). Hub also has `[profile.wasm-dev.package.semio-hub-gis]` (line 434) but only for a few hubs (norm, stdio, gis, vcs, demonstrator); not required.

Dependency gating rule: `🕸️dag/🏭️bridge`, `🌍️gis/🏭️bridge` and `🖍️draw/🏭️bridge` each depend on their own artifact crate with `path`.

### 2.3 Crate naming and roles

| Role | Crate name pattern | Manifest marker | Example |
| --- | --- | --- | --- |
| Artifact | `semio-s-artifact-<plugin>-<artifact>` | `[package.metadata.semio] role = "artifact"` | `semio-s-artifact-note-note` |
| Artifact test oracle | `semio-s-artifact-<plugin>-<artifact>-test-oracle` | `role = "test"` | `semio-s-artifact-note-note-test-oracle` |
| Generator/codec and probe subcrates | own names, e.g. mathematical's `equation-1-any-json-engine`, `generate`, `reader`; probe `semio-drawing-oracle-probe` (draw) | `🏅️standards/.../🏭️generator/...`, `🔬️probes/...` | (note's codec crate name not read) |
| Extension | `semio-s-plugin-<plugin>-extension-<ext>` (sometimes just `semio-s-plugin-<plugin>-<ext>`, e.g. `semio-s-plugin-process-wood`) | `role = "extension"`, `extends = "<plugin>"` | `semio-s-plugin-flow-extension-bim` |
| Hub composition (WASM) | `semio-hub-<plugin>` | `[package.metadata.component] package = "semio:<plugin>"`, `role = "hub"`, `component-kind = "plugin"` | `semio-hub-note` |
| Mutation bridge | `semio-<plugin>-mutation-bridge` (bin) | own `[workspace]` | `semio-dag-mutation-bridge` |

### 2.4 Artifact Cargo.toml (note, trimmed; `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml`)

```toml
[package]
workspace = "../../../../../.."
name = "semio-s-artifact-note-note"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "note note artifact"

[package.metadata.semio]
role = "artifact"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-artifact-reference = { workspace = true }
semio-framework-dsl-record-derive = { path = "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust" }
semio-framework-dsl-record = { path = "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust" }
...
semio-framework-plugin = { workspace = true, features = ["component-guest"] }
...
[dev-dependencies]
semio-framework-plugin = { workspace = true, features = ["component-guest", "artifact-app-testing"] }
```

Notes:
- `workspace = "../../../../../.."` points to the `✏️s` root (6 levels up).
- No `crate-type` on artifacts (default rlib). Hub compositions use `crate-type = ["cdylib", "rlib"]`.
- Framework dependency paths are relative: `../../../../../../../🧰️framework/...` (7 levels from `📦️packages/🦀️rust` under an artifact).
- Framework crate names used by artifacts (verified): `semio-framework-plugin` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust`), `semio-framework-plugin-host` (`.../🔌️plugin/🖥️host/📦️packages/🦀️rust`), `semio-framework-plugin-describe` (`.../🔌️plugin/🖨️describe/📦️packages/🦀️rust`), `semio-framework-os-kernel` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`), `semio-framework` (`🧰️framework/📦️packages/🦀️rust`), `semio-framework-value`, `semio-framework-value-derive`, `semio-framework-dsl`, `semio-framework-dsl-record`, `semio-framework-schema`, `semio-framework-ui-contract`, `semio-framework-job`, `semio-framework-pack-json`, `semio-framework-dispatch-macros`, `semio-framework-async`, `semio-framework-2d`, `semio-framework-artifact-reference`, `semio-framework-tool-run`, `semio-framework-tool-machine`, `semio-framework-ui-scene` (dev). Feature flags seen: `component-guest`, `artifact-app-testing`, `protocol-laws`, `conversion-drawing`, `component-app-assembly`.

### 2.5 Composition (hub) Cargo.toml (note, `🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/Cargo.toml`)

```toml
[package]
workspace = "../../../.."
name = "semio-hub-note"
...
[package.metadata.component]
package = "semio:note"

[package.metadata.semio]
deployment-directory = "🗒️note"
component-kind = "plugin"
role = "hub"

[[package.metadata.semio.playground]]
variant = "note"
app = "s.note.note@1/*#editor"
ports = { react = 6080, wgpu = 6180 }

[lib]
crate-type = ["cdylib", "rlib"]
path = "🦀️.rs"

[dependencies]
semio-s-artifact-note-note = { workspace = true }
...
[package.metadata.semio.sources]
artifacts = ["../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts"]

[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]
semio-framework-os-run = { path = "...", package = "semio-framework-os-run" }
```

Playground fields that matter: `variant` (dev variant id), `app` (`s.<plugin>.<artifact>@1/*#editor`), `ports` (react and wgpu dev ports; mirrored into launch.json `S_OS_PORT`), optional `aliases`, optional `mcpHost` / `nativeHost` (only gis has them, pointing at `✏️s/🧑‍💻dev/💡️services`), `[package.metadata.semio.sources] artifacts = [...]`, `depends-on = [...]` for runtime actor dependencies.

Composition lib root (note `🌎️hub/🧩️compositions/🗒️note/🦀️.rs`): `Plugin::<NoteApps>::builder("note").label("Note").version("0.1.0").package_id("semio:note").declare_artifact(crate::artifacts::note::artifact()).activation(ActivationEvent::OnArtifactKind{...}).execution(ExecutionMode::Isolated).requests(CapabilityRequest{ id: CapabilityId("artifacts.write"), ... }).try_build()`. Apps are a closed `dyn_enum_close!` enum of `EditorApp`/`ViewerApp`. Test modules via `#[path = "🧪️tests/🔬️surface/🦀️.rs"]`.

Composition sidecar files: `🔣️.json` (generated plugin descriptor/manifest JSON, e.g. 211 KB for note) and `🛂️.descriptor.semio` (committed descriptor pack). `🧪️tests/🔬️surface/`, `🧪️tests/🛂️committed-descriptor/`, `🧫️fixtures/🛂️committed-descriptor/`. Both are produced by the describe step (see 5.4).

### 2.6 Artifact lib root with `#[path]` module tree (note `🗿️artifacts/🗒️note/🦀️.rs`, lines 424-536)

The lib uses emoji folder/file names via `#[path]`, one `pub mod` per directory level:
```rust
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
```
Each subset's `🦀️.rs` is reached this way; artifact-level types are re-exported (`pub use crate::schema::snapshot::NoteSnapshot;`). Tests: `#[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`.

### 2.7 Bridge crate (`🕸️dag/🏭️bridge/Cargo.toml`, `🦀️.rs`, `📜️script.ts`)

Cargo: `[workspace]` (own root), `[[bin]] name = "semio-dag-mutation-bridge" path = "🦀️.rs"`, deps on `semio-framework-pack-json`, `semio-framework-value`, `semio-framework-os-kernel`, `pack` (= `semio-framework-pack`) and the artifact crate.

`🦀️.rs` holds two hand-written tables that must be edited per plugin:
- `AGGREGATES: &[(&str, fn() -> &'static [MutationLeafDescriptor])]` naming `semio_s_artifact_dag_dag::...::DagConfigMutation`, `DagPresenceMutation`, `DagMutation`.
- `COORDINATES: &[(artifact, standard, subset, surface, owner-dir, prefix)]` for `s.dag.dag` standard `1` subset `any`.

`📜️script.ts` spawns `cargo run --quiet --offline --bin semio-dag-mutation-bridge -- list-mutations <artifact> <standard> <subset> [<surface>]` and prints RuntimeMutationInventory JSON. gis and note bridges are identical except the names (diffed).

## 3. How a plugin is discovered and registered

### 3.1 Chain (follow it in this order)

1. Cargo manifest with `[package.metadata.component] package = "semio:<name>"` and `[package.metadata.semio] component-kind/role` under `🌎️hub/🧩️compositions/<emoji><name>/📦️packages/🦀️rust/Cargo.toml` (plugins) or inside `✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust` (extensions, e.g. flow bim, with its `🔣️.json` + `🛂️.descriptor.semio` at extension root).
2. Discovery: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts` (`discoverComponentPackages`, `findPluginCargoFiles`; `parseComponentPackageId` requires the form `semio:<lowercase-alnum-or-hyphen>`) reads these manifests. Taxonomy gates directory names.
3. Generation: `bun nx run @semio-tech/plugin-registry:generate` (project `@semio-tech/plugin-registry`, target `generate`, defined in `🧰️framework/.../🔌️plugin/📇️registry/📋️project.json`, router `📜️script.ts`). It writes:
   - `🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`: one row per plugin: `pluginId`, `packageId` (`semio:draw`), `cratePath`, `packageName`, `wasmOut`, `directoryName`, `role`, `consumes`, `dependsOn`, `capabilities`, `contributes`, `activationEvents`, `extensionPoints`, `executionMode` (`isolated`), `hashes`. Current file has only `draw` and `puzzle` rows (generated 2026-10-06); do not treat it as complete.
   - `🤖️generated/🧩️plugins/🟦️.ts` (`@generated ... do not edit`): `PluginBuildTarget` rows and `COMPONENT_MODULE_DIRECTORIES`. Contains note, dag, gis, draw, architect, etc.
   - `🤖️generated/🧰️framework.json`, `🚀️playgrounds.json`, `🎠️playgrounds.json`, `🩺️diagnostics.json`, `🖥️hosts/`, `🗿️artifacts/`, `🏗️framework/`.
4. Taxonomy membership: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:
   - `semanticDirectoryMemberKinds["members-of-plugins"]` has `"memberNames": [..., "🕸️dag", "🖍️draw", "🌍️gis", "🗒️note", ...]` and `"source": "registry"`. The discovery code validates `source === "registry"` and checks directory names against `memberNames` (`🔍️discovery/🟦️.ts` ~lines 2271, 2432, 4207). Nothing in the library generates these arrays; a new plugin directory must be added to `members-of-plugins` by hand, then `bun nx run @semio-tech/plugin-registry:check` / `test` must pass.
   - Extension member lists sit around lines 8770-8810 (e.g. `🏘️flow-extension-bim`, `🏟️cad-extension-aec-building-structure`). `semanticDirectoryKinds` has 729 keys; none contains `bim`, `building` or `fem` (checked by key scan). See section 9 for emoji.
5. Hub: `🌎️hub/Cargo.toml` members and dependencies (section 2.2) and `🌎️hub/🧩️compositions/📜️script.ts` (`ownership` / `native` routes; `ownership` calls `assertConcreteCompositionOwnership(this.root)` from `🧪️tests/📇️ownership/🟦️.ts`, which was not read in full).
6. Plugin-specific hub code: `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (17,821 lines) imports `COMPONENT_MODULE_DIRECTORIES` from the generated registry and runs gates. Gis has many dedicated gates there (`trusted-stdio-gis-bundle-check`, `gis-map-proposal-check`, ...). Note, dag and draw have no dedicated hub gates (grep hits are only the generic imports and composition sources).
7. Root `📜️script.ts`: discovers every `🔒️policy-allowlist.json` by convention (`🔒️policy-allowlist.json` in `✏️s/🔌️plugins/` is one). Per-plugin tool-run lanes are a hand-maintained array (`📜️script.ts` ~lines 9776-9812, `INTERACTIVITY_TOOL_RUN_ANY`, entries like `{ toolId: "reorganize", root: "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/${INTERACTIVITY_TOOL_RUN_ANY}", ... }`). Add rows only for plugins that register tools.
8. Root `📜️script.ts` also hosts the per-plugin contract verifiers invoked from the root `📋️project.json` (e.g. `"note-document-contract": { "command": "bun ./📜️script.ts verify note-document-contract" }`, `"dag-demo-ownership"`, `"note-empty-config-ownership-oracle"`).
9. Root `🔒️dependencies.json`: inventory of every third-party dependency with its `users` (manifest paths, e.g. `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/package.json`). New third-party deps must be regenerated with root target `verify-dependencies-freeze-write-baseline` (`bun ./📜️script.ts verify dependencies write-baseline`). The test platform (`🧪️test/🕸️dependencies`) compares against it.
10. `nx.json` inference plugin `🧰️framework/.../📚️library/🔌️nx-plugin/🟨️.mjs`: includes `**/📋️project.json`, `**/Cargo.toml`, `bun.lock`, `**/*.patch`. Nx projects come from these files; no manual project registry.

### 3.2 Repo-level grep result for the four names (gis, note, dag, draw)

Searched with `rg` on `plugin-gis|plugins/🌍️gis|semio:gis|🌍️gis` (and the same for note/dag/draw) across the repo, excluding `.git`, `.venv`, `node_modules`, `target`, `*.lock`, tickets, fixtures and `.cursor`. Non-test, non-artifact registration sites found:

| File | Why it lists the plugin |
| --- | --- |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | dev/test/gate launch entries (section 6) |
| `✏️s/Cargo.toml` | workspace members + `workspace.dependencies` (section 2.2) |
| `🌎️hub/Cargo.toml` | hub members + `workspace.dependencies` |
| `Cargo.toml` (root) | `exclude` (lines 127-225) lists `✏️s`, `🌎️hub` and plugin-internal standard/oracle crates (note at lines 219-220). The framework `🕸️dag` artifact path (line 77) is a framework crate, unrelated |
| `🌎️hub/🧩️compositions/<plugin>/` | the WASM composition |
| `🌎️hub/🧩️compositions/📜️script.ts` | ownership/native gates |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | generic imports; gis gates |
| `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` | gis-only inference routing |
| `✏️s/🧑‍💻dev/...` (`🚀️entry`, `🧩️service-composition`) | gis-only inference worker |
| `📜️script.ts` (root) | verify contracts, tool-run lanes, policy checks |
| `📋️project.json` (root) | verify targets (note/dag contract verifiers) |
| `🔒️dependencies.json` (root) | dependency users (manifest paths) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `members-of-plugins`, extension memberships |
| `🧰️framework/.../🔌️plugin/📇️registry/🤖️generated/...` | generated registry (`🔌️plugins.json`, `🧩️plugins/🟦️.ts`) |
| `🧰️framework/.../🔌️plugin/📇️registry/...fixtures/...` | fixtures/tests (no action) |
| `🧰️framework/.../🧑‍💻dev/🧫️fixtures/🚀️local-hub.json` | fixture |
| `🌎️hub/🗿️artifact-authority/...`, `🌎️hub/📇️directory/...` | gis-specific fixtures/tests |
| `🧰️framework/.../🖨️describe/...`, `🌐️browser-bundle/...`, `📇️registry/...` | framework tests and fixtures that mention gis as an example plugin (no registration) |
| `🎛️dashboard` (framework) | README: dashboard offers launch entries from `.vscode/launch.json` |
| `🔒️policy-allowlist.json` (`✏️s/🔌️plugins/`) | policy allowlist keys |
| `.cursor/plans/*.plan.md`, `🎫️tickets/*` | history only, not registration |

Note and dag additionally appear in: root `Cargo.toml` (lines 219-220: note artifact codec and oracle crates), `✏️s/Cargo.toml` (lines 80, 121, 236-237, 272, 275), `🌎️hub/Cargo.toml` (lines 33, 46, 96, 99), `🌎️hub/🧩️compositions/{🕸️dag,🗒️note}/`, `🔒️dependencies.json` (`/entries[225]`, `/entries[226]`), root `📋️project.json` (note-/dag- verify targets), root `📜️script.ts` (line 7598 contract list, line 9806 tool-run lanes for dag and note), and `.vscode/launch.json`.

## 4. nx / bun / package.json

Per-plugin contract (matches AGENTS.md): `project.json` only calls `bun ./📜️script.ts <command>`; `package.json` only calls `bun nx run <project>:<target>`.

Examples (verified):
- Artifact Rust project `🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/📋️project.json`: name `@semio-tech/note-note-rs`, `tags: ["language:rust", "role:artifact", "family:note"]`, targets `build` (`bun ./📜️script.ts build`), `check`, `test` (`dependsOn` the hub `@semio-tech/note-plugin:component-dev`), `verify-note-empty-config-ownership`, `verify-sqlite-snapshot-guest`, `verify-snapshot-guest`, `verify-sqlite-snapshot-native`. Each target has `metadata.semio.workspaceCommand`.
- Artifact Rust `package.json`: `"name": "@semio-tech/note-note-rs"`, `"scripts": { "verify-note-empty-config-ownership": "bun nx run @semio-tech/note-note-rs:verify-note-empty-config-ownership" }`, `"bundleKind": "repo"`, `"nx": { "includedScripts": [] }`.
- Artifact TypeScript: `@semio-tech/note-note-js` (package.json `bundleKind: "repo"`, `"projectType": "library"`, dependencies `workspace:*` on `@semio-tech/framework`, `@semio-tech/ui-react`, `@semio-tech/ui-styling`, `@semio-tech/s-3d-js`, cad modules). Name inconsistency: dag's artifact TS package is `@semio-tech/dag-dag` (no `-js`), its plugin-level package `@semio-tech/dag-js`.
- Hub composition `🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/📋️project.json`: name `@semio-tech/note-plugin`, targets `test`, `test-quick`, `test-long`, `test-exhaustive` running `bun ./📜️script.ts test [quick|long|exhaustive]`. Its `📜️script.ts` routes `test` to `runRepositoryCargoTests(["semio-hub-note"], ...)` and calls `registerPlaygroundSiteBuildCommands(router)`.
- Plugin-level TS (gis) `📦️packages/🟦️typescript/package.json`: `"name": "@semio-tech/gis-js"`, `"bundleKind": "repo"`, `"scripts": { "test": "bun nx run @semio-tech/gis-js:test" }`. `📋️project.json` has `test` executor `nx:run-commands` with `bun ./📜️script.ts test`.
- Hub compositions root project `@semio-tech/hub-compositions` (`🌎️hub/🧩️compositions/📋️project.json`): `ownership` and `native` targets.
- Root Nx project `workspace` (root `📋️project.json`) has target `verify-dependencies-freeze-write-baseline` (verified). Root `package.json` scripts: `nx`, `setup`, `dashboard`, `dashboard:install`.
- Workspaces: root `package.json` `workspaces` covers `🧰️framework/**` only; `✏️s/package.json` (`semio-s-workspace`) has `"workspaces": ["**", "../🧰️framework/**", ...]`, so plugin JS packages resolve via `✏️s/package.json` and `bun.lock` at root and in `✏️s`.
- `📜️script.ts` contract: `BundleScript` + `ScriptRouter(import.meta.dir).register("<cmd>", Class)` + `runScriptMain(router, { defaultCommand })`. Plugin routers only register what they need (`test`, `verify`, `build`, `check`, `canonical-architecture`, `native`, ...).

## 5. Cargo-side plugin plumbing: descriptor, build, describe

5.1 `🖨️describe` (framework): `🧰️framework/.../🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` routes `build`, `test`, `describe`, `component`, `test-fresh-component`, `test-canonical-descriptor-pack`. `describe` / `component` emit `🛂️.descriptor.semio` + `🔣️.json` at the owner root (`🛂️descriptor-emission`, `🏭️fresh-component`).

5.2 `🏗️build` (framework): `🧰️framework/.../🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts` writes `🛂️.descriptor.semio` into the owner root (`writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), ...)`) and copies it to `dist` for the build. Plugin wasm target: `wasm32-wasip2` (`🏗️build/📋️plan/🟦️.ts` line 34).

5.3 Toolchain: `rust-toolchain.toml` `channel = "nightly-2026-07-20"`, targets `wasm32-unknown-unknown`, `wasm32-wasip2`. `.cargo/config.toml` `[target.wasm32-wasip2]` sets the plugin guest stack (`-zstack-size=8388608`) and a memory maximum. Build cache under `.🧬semio/🦑️repo/⚡️cache/cargo`.

5.4 Regenerate after changing a plugin's Rust surface: `bun nx run @semio-tech/plugin-registry:generate`, then the describe pipeline (`bun ./📜️script.ts describe` in the framework describe package; exact project name not checked), then `verify dependencies write-baseline` if deps changed. Verify the regenerated files with `bun nx run @semio-tech/plugin-registry:check` and the composition's `test` target.

5.5 `🔒️policy-allowlist.json`: a plugin does not need an entry unless a policy rule names a path (e.g. `semantic-vocabulary` entries list specific `🦀️.rs` paths of writer/mathematical). Deleting the plugin deletes its entries.

## 6. Launch configurations (`.vscode/launch.json`)

Facts:
- `.vscode/launch.json` is 3.3 MB, 4,667 configurations; `.vscode/🧩️launch.seed.jsonc` (2.7 MB) has 3,505 configurations, plus `devLaunchers`, `projectLaunchers`, `inputs`, and string tokens like `"@generated:dag:react"`. No code in the working tree consumes `devLaunchers` or the `@generated:` tokens (`rg` finds nothing outside the seed). Treat the seed as the authored source of the naming/order rule; keep `launch.json` in sync by hand.
- `launch.json` is tracked by git. The dashboard (`🎛️dashboard` README) offers every configuration of `launch.json` to devs.
- Groups: `0_dev`, `1_keyboard`, `2_mouse`, `2_build`, `3_dev` (the `🛠️dev` family), `4_build`, `4_gate` (the `⚖️gate`, `⚖️test`, `⚖️check`, `🔍️check` family), `4_test`, `9_gates`, `9_goal`, `9_clean_architecture`, `repo-gate`, `🧹clean🛡️gates`, `🪶️ Snapshot SQLite`, etc.
- Seed `projectLaunchers.classes`: `dev` → `3_dev` orderBase 900 (tokens dev/serve/start/watch/activate/open/launch/inspect/demo/playground/attach); `build` → `4_build` orderBase 900; `gate` → `4_gate` orderBase 900; `run` → `3_dev` orderBase 950.

Dev launch entry shape (dag, `.vscode/launch.json`):
```json
{
  "name": "🛠️dev🕸️dag⚛️react",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run workspace:dev -- dag",
  "cwd": "${workspaceFolder}",
  "env": { "S_OS_PORT": "6017", "SEMIO_PLUGIN": "dag", "SEMIO_RENDERER": "react" },
  "presentation": { "group": "3_dev", "order": 140 },
  "serverReadyAction": { "action": "openExternally", "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6017)", "uriFormat": "%s" }
}
```
Companions: `🛠️dev🕸️dag🧊️wgpu🌐️wasm` (order `140.1`, port 6117, `SEMIO_RENDERER: "wgpu"`), `🛠️dev🕸️dag🧊️wgpu🖥️native` (order `140.2`, `command: "bun nx run @semio-tech/framework-renderer-wgpu:native -- dag"`, no env). Note entries: `🛠️dev🗒️note⚛️react` (388, 6080, `SEMIO_APP: "s.note.note@1/*#editor"`), `🛠️dev🗒️note🧊️wgpu🌐️wasm` (388.1, 6180), `🛠️dev🗒️note🧊️wgpu🖥️native` (388.2). Gis2d/gis3d entries use 160/160.1/160.2 and 160.3/160.4/160.5.

Naming convention: `🛠️dev` + plugin emoji+name + artifact emoji+name (+ `🧊️wgpu` / `⚛️react` / `🌐️wasm` / `🖥️native`). Gates: `⚖️gate🕸️dag🪶️sqlite` (order 408.74x), `⚖️check🗒️note-snapshot-guest🦀️` (411.264x), `⚖️test-document-contract🗿️artifacts🗒️note🟦️` (900.048x). Test-run entries for draw: `🧪️test🖍️draw🦀️` (386.5), `🧪️test🖍️draw🟦️` (386.4), `🔍️check🖍️draw🦀️` (386.6), `🛠️activate🖍️draw⚛️react` / `🛠️serve🖍️draw⚛️react` (386.3, 386.31).

Order band observed in `devLaunchers` (seed): cad 10; dag 140; mathematical 141; architect 142; flow 150; imperative 155; sequence 156; lowpoly 157; layout 158; gis2d 160; gis3d 160.3; animate 170; generation2d 180; generation3d 190; process3d 195; sourcing 196; wfc 201-205; puzzle 220-250; block 261-263; reasoning 270; shooting 290; trinity 360/380; forms 385; vcs 385; raster 386; draw 387; writer 387; note 388; remodel 389; fem 392. Pick an unused integer (or `.n` suffix) in the band for the plugin.

Ports: `S_OS_PORT` and `playground.ports.react` / `.wgpu` must match. Used react ports in compositions include 6015-6032, 6040-6045, 6051-6064, 6075-6093, 6100-6110, 6211-6218, 6231-6250, 6266-6270 (the list was truncated in the grep, so check before choosing).

## 7. bridge (`🏭️bridge`) and commands (`🎮️commands`) wiring

- `🏭️bridge/` contents (present in all 34 plugins): `Cargo.toml` (own workspace), `Cargo.lock` (generated), `🦀️.rs` (mutation inventory printer), `📜️script.ts` (Bun wrapper running `cargo run --offline --bin <crate> -- list-mutations ...`).
- The test platform calls it: the `🦀️.rs` header names the caller `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts`. `testBridgeDirName = "🏭️bridge"` in taxonomy.
- The bridge's `COORDINATES` must list every (artifact, standard `1`, subset `any`, surface) the plugin owns, and `AGGREGATES` must list every `DESCRIPTORS` mutation aggregate the artifact mounts. A verb dispatched in production but absent from the manifest is a breach.
- `🎮️commands/` is a placeholder for plugin commands; real command surfaces live under `✏️editor/🎮️commands/` and `👁️viewer/🎮️commands/` of each subset (`surfaceChildDirs` includes `🎮️commands`).
- Interactivity tool-run lanes in root `📜️script.ts` (section 3.1 item 7) list commands per plugin.

## 8. Plugin-to-plugin dependencies

- Runtime actor deps: `[package.metadata.semio] depends-on = [...]` in the hub composition. Demonstrator: `depends-on = ["cad", "gis", "procedural", "process", "puzzle", "sourcing", "flow-extension-bim", "flow-extension-brep", "flow-extension-dictionary", ...]` (its comment says the ids must match the `.depends_on(...)` rows of its manifest). Procedural (`🌎️hub/🧩️compositions/🌀️procedural`): `consumes = ["forms.questionKind", "flow.extension"]` and `depends-on = ["flow-extension-bim", "flow-extension-brep", ...]`. Stdio extensions: `depends-on = ["stdio"]` (media, bim, office, pdf, binary, mesh, image, cad, semio).
- Extensions: `[package.metadata.semio] role = "extension"`, `extends = "flow"` (or cad/imperative/sourcing), `contributes = ["flow.extension"]`. Flow BIM example in section 0.
- Build-time Rust deps across plugins are normal for shared codecs: dag, gis, draw, note artifacts depend on `semio-s-artifact-stdio-*` crates (`semio-s-artifact-stdio-json`, `-svg`, `-png`, `-pdf`, `-dxf`, `-dwg`, `-semio`, ...). Note's oracle depends on `semio-s-plugin-stdio-document-test-oracle` and `semio-s-plugin-stdio-markup-test-oracle`.
- Taxonomy restrictions (`🔣️taxonomy.json` → `cargoDependencyDirections`): plugin roots that are not `🗿️artifacts/` or `🧩️extensions/` may not depend on artifact or extension crates (`cargo-plugins-no-artifacts`, `cargo-plugins-no-extensions`); s-modules may not depend on plugins/extensions (`cargo-modules-no-plugins`); framework may not depend on hubs/plugins/extensions (`cargo-framework-no-implementation-role`). The hub (`role: "hub"`) is not restricted by these.

## 9. Emoji for the new plugin (constraints observed)

Rule: every file and folder has exactly one emoji, unique among its siblings (taxonomy `pathEmojiPolicy`). The sibling set that matters for a plugin is `✏️s/🔌️plugins/` (34 names; `🏗️` is taken by `🏗️fem`, `🏛️` by `🏛️architect`, `🏭️` by `🏭️process`). The same emoji is reused in `🌎️hub/🧩️compositions/<e><name>`, in launch names and in `semio-hub-<name>`, so check the hub compositions directory too. Already-taken BIM-adjacent emoji in other sibling sets: `🏠️` (stdio extension `🏠️bim`, and the space artifact `🏠️home`), `🏘️` (flow extension `🏘️flow-extension-bim` deployment name), `🏢️` (cad extension `🏢️aec-building`), `🏗️` (stdio artifact `🏗️ifc`). Verify with `ls` on the exact sibling directory before picking.

## 10. Checklist: what to create or edit for a new `bim` plugin

Work in order. Names below use `<E>` for the chosen emoji and `bim` for the name; `<A>` for the artifact dir (emoji + name). Keep the same emoji for plugin, composition and launch names.

A. Decide (no file changes)
  1. Emoji `<E>` unique among `✏️s/🔌️plugins/` siblings and among the hub compositions dir (`🌎️hub/🧩️compositions/`).
  2. Artifact(s) `<A>`, standard `🔖️1`, subset `✳️any`. Decide whether the flow/stdio BIM overlap becomes an extension (`🌊️flow/🧩️extensions/🏗️bim` already exists) or a new plugin.

B. Plugin root `✏️s/🔌️plugins/<E>bim/`
  3. `AGENTS.md` (front matter `technology: bim`, `emoji: <E>`), `README.md` (front matter `name: bim`, `kind: user`).
  4. `🎮️commands/📌️.empty.md` (copy from note; text "This owner currently declares no commands.").
  5. `🏭️bridge/` : `Cargo.toml` (own `[workspace]`, `[[bin]] name = "semio-bim-mutation-bridge"`), `🦀️.rs` (copy dag's structure: AGGREGATES + COORDINATES), `📜️script.ts` (copy dag's; change the bridge name), `Cargo.lock` (generated by cargo).
  6. Optional: `📦️packages/🟦️typescript/{package.json (@semio-tech/bim-js), 📋️project.json, 📜️script.ts}`, `🧪️tests/`, `🧫️fixtures/`, `🔮️oracles/🔣️.json` (schemaVersion 2, copy gis/draw), `📖️stories/` if needed.

C. Artifact `🗿️artifacts/<A>/`
  7. `🦀️.rs` (artifact lib root; `#[path]` tree as in note).
  8. `📦️packages/🦀️rust/{Cargo.toml (package name semio-s-artifact-bim-<artifact>, [package.metadata.semio] role = "artifact", [lib] path = "../../🦀️.rs", [lints] workspace = true), package.json (@semio-tech/bim-<artifact>-rs), 📋️project.json (project name @semio-tech/bim-<artifact>-rs; targets call bun ./📜️script.ts ...), 📜️script.ts (BundleScript router with runArtifactRustPackageMain)}`.
  9. `📦️packages/🟦️typescript/` (optional, see note/gis).
  10. `🏅️standards/🔖️1/🪆️subsets/✳️any/` with `🦀️.rs`, `🧬️schema/` (`🔣️.json`, `🦀️.rs`, `🟦️.ts`, `📸️snapshot/`, `🔺️diff/`, `🧬️mutations/`, optional `💡️inferences/`), `🚪️io/` (`🦀️.rs`, `🟦️.ts`, `📝️text/`, `💾️binary/`, `📥️import/`, `📤️export/`, `🪶️sqlite/`), `✏️editor/` (`🦀️.rs`, `🎚️config/`, `🎭️modes/`, `🎮️commands/`, `👥️presence/`, `🫧️transient/`, `🪟️window/`, `🗣️terminology/` if needed), `👁️viewer/` (same, read-only), `📚️examples/`, `🧪️tests/`, `🧫️fixtures/`, `🔬️probes/` and `🏭️generator/` when needed. Use `bun ./📜️script.ts new artifact <owner-root> <new-dir>` (root `📜️script.ts`, router `"new"` → `CleanMechanismNewScript`) to scaffold the artifact tree; `new subset <standard-root> <dir>` for subsets; scaffolds are taxonomy-checked.
  11. `🔮️oracles/📦️packages/🦀️rust/Cargo.toml` (`role = "test"`, `[features] oracles = [...]`) and `🔮️oracles/🦀️.rs`, `🔣️.json` if the artifact has an oracle.
  12. `🧪️tests/⚡️quick/`, `🧪️tests/🔬️unit/`, and per-artifact `.feature` files (`🥒️.feature`, language-agnostic test; AGENTS requires at least one per feature).

D. Workspaces
  13. `✏️s/Cargo.toml`: add the artifact crate to `[workspace] members` and a line to `[workspace.dependencies]`; add the `🔮️oracles` crate to members if created.
  14. `🌎️hub/Cargo.toml`: add `🧩️compositions/<E>bim/📦️packages/🦀️rust` to members, add `semio-s-artifact-bim-<artifact>` to `[workspace.dependencies]`, add `semio-hub-bim` to `[workspace.dependencies]` only if another crate needs it.
  15. Root `Cargo.toml`: no workspace member is needed (`✏️s` and `🌎️hub` are excluded). If the new plugin adds standard generator/probe or oracle crates, mirror the existing explicit entries in the root `exclude` list (lines 127-225), as note's codec and oracle do at lines 219-220.

E. Hub composition `🌎️hub/🧩️compositions/<E>bim/`
  16. `📦️packages/🦀️rust/Cargo.toml` (name `semio-hub-bim`, `[package.metadata.component] package = "semio:bim"`, `[package.metadata.semio] deployment-directory = "<E>bim"`, `component-kind = "plugin"`, `role = "hub"`, `[[package.metadata.semio.playground]] variant, app = "s.bim.<artifact>@1/*#editor", ports = { react = …, wgpu = … }`, `[package.metadata.semio.sources] artifacts = ["../../../../../✏️s/🔌️plugins/<E>bim/🗿️artifacts"]`, `depends-on = [...]` if needed; `[lib] crate-type = ["cdylib","rlib"]`).
  17. `📦️packages/🦀️rust/📋️project.json` (`@semio-tech/bim-plugin`, targets test/test-quick/test-long/test-exhaustive), `📜️script.ts` (route `test` to `runRepositoryCargoTests(["semio-hub-bim"], ...)`; add `registerPlaygroundSiteBuildCommands`).
  18. `🦀️.rs` (`Plugin::<BimApps>::builder("bim").label(...).version("0.1.0").package_id("semio:bim").declare_artifact(...).activation(ActivationEvent::OnArtifactKind{...}).execution(ExecutionMode::Isolated).requests(...).try_build()`; `dyn_enum_close!` for editor/viewer apps).
  19. `🧪️tests/🔬️surface/🦀️.rs`, `🧪️tests/🛂️committed-descriptor/🦀️.rs`, `🧫️fixtures/🛂️committed-descriptor/🔣️.json`.
  20. Generated (do not hand-write): `🔣️.json`, `🛂️.descriptor.semio` (via describe), and `🌎️hub/🧩️compositions/📜️script.ts` gates must still pass.

F. Registry and generated files
  21. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: add `<E>bim` to `members-of-plugins.memberNames` (and to any `members-of-…` list for extensions/artifacts it owns). Membership is validated by discovery; the list is not generated.
  22. Run `bun nx run @semio-tech/plugin-registry:generate` and keep its output (`🤖️generated/🔌️plugins.json`, `🤖️generated/🧩️plugins/🟦️.ts`, `🤖️generated/🧰️framework.json`, `🚀️playgrounds.json`, `🎠️playgrounds.json`, `🩺️diagnostics.json`). Do not commit unless asked (AGENTS.md).
  23. If new third-party deps: `bun nx run workspace:verify-dependencies-freeze-write-baseline` → `🔒️dependencies.json`.
  24. If the plugin has interactive tools: add rows to the tool-run array in root `📜️script.ts` (~line 9776-9812).
  25. If a policy rule needs it: `✏️s/🔌️plugins/🔒️policy-allowlist.json` entries (keys follow the policy rule ids).

G. Launch (`.vscode/launch.json`, and mirror into `.vscode/🧩️launch.seed.jsonc` `devLaunchers`)
  26. Add `🛠️dev<E>bim<A>⚛️react` (group `3_dev`, order `N`, env `S_OS_PORT`, `SEMIO_PLUGIN: "bim"`, `SEMIO_RENDERER: "react"`, `SEMIO_APP: "s.bim.<artifact>@1/*#editor"`, `serverReadyAction` pattern with the same port), `🛠️dev<E>bim<A>🧊️wgpu🌐️wasm` (order `N.1`), `🛠️dev<E>bim<A>🧊️wgpu🖥️native` (`N.2`, `command: "bun nx run @semio-tech/framework-renderer-wgpu:native -- bim"`). Add a `devLaunchers` row with `namePrefix`, `order`, `wgpuOrder`.
  27. Add gate/test entries in `4_gate` (`⚖️gate<E>bim…`, `⚖️check…`, `⚖️test…`) pointing to the `bun nx run @semio-tech/bim-<artifact>-rs:<target>` names of step 8 and to the hub `@semio-tech/bim-plugin:test`.
  28. `compounds` only if the plugin needs companion processes (e.g. `🧭️compound🖥️s⚛️react🗄️os-hub`).

H. Verify (run them; do not claim pass without output)
  29. Target names follow the note/dag pattern (by analogy, not yet run): `bun nx run @semio-tech/bim-plugin:test-quick` (hub), `bun nx run @semio-tech/bim-<artifact>-rs:test` (artifact), `bun nx run @semio-tech/plugin-registry:check`, `bun nx run @semio-tech/plugin-registry:test`, `bun nx run workspace:verify-dependencies-freeze-write-baseline` (if deps), `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-<artifact>`, `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --target wasm32-wasip2`. Target names are taken from the project.json files cited above.
  30. Runtime confirmation (AGENTS: "confirm runtime behaviour with console logs"): dev entry from launch.json, `[DEBUG] ` prefixed temporary logs only.

## 11. Open points and caveats

- The exact `bun nx run` project name for the framework describe package was not checked; use `bun nx run` with the name from its `📋️project.json` (`🖨️describe/📋️project.json`, not read).
- `🔌️plugins.json` currently contains only two rows; the generator's `--exclude` / stale-channel logic was not read, so the reason is unknown.
- `🌊️flow` and `🗄️stdio` use non-standard children (`🕸️manifest.json`, `🦀️.rs`, `📇️registry`); these were not analyzed further.
- The scratchpad of this session already contained `r1-prose.md`, `r1-template.md`, `build-r1.sh` (timestamps before this run). They were not used as sources.
- Launch-entry order numbers and port numbers are per-plugin hand choices; the list above is a census, not a reservation.
