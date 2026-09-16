# 🧩️ cad.computer / sourcing.module topic wiring — 2026-09-16

Goal: make the seven `cad-extension-*` / `sourcing-module-*` components reachable from the
demonstrator's runtime component closure so the koordinator (cad) and aussuchen (sourcing) panes
actually receive their extensions.

## 1. How a host plugin consumes an extension topic (investigation)

The mechanism is uniform across `flow.extension`, `process.machines`, `cad.computer` and
`sourcing.module`; nothing is topic-specific in the framework.

**Metadata / selection (build + boot time)**

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs`
  `runtimeComponentClosure` — closes roots over `dependsOn` and, per selected component, over every
  `consumes` topic → every component whose `contributes` names that topic.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:929` — the Nx plugin runs that closure
  per playground variant; `:962` (react) / `:978` (wgpu) / `:941` (native) turn the selected ids into
  `…:materialize-<profile>` `dependsOn` rows of `prepare-<variant>-<target>-<profile>`.
  This is why the `prepare-*` targets were missing the extension builds.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:381` `expandPluginRegistry` — the same rule at boot: the
  primary plugin's `consumes` selects contributor entries, then the `dependencies` closure.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts:295` — `consumes` is
  read ONLY from Cargo `[package.metadata.semio]`. `:271`–`:284` (doc) + `:310`–`:333`: `contributes`
  comes from the crate's descriptor (`<cratePath>/../../🔣️.json`, see
  `🔣️taxonomy.json` → `generatorContracts["plugin-registry"].inputDiscovery.descriptorRelativePath`,
  resolved at `🔎️discovery/🟦️.ts:218`/`:256`) **when a descriptor exists**, else from Cargo
  `contributes`.
- The plugin builder has **no** `.consumes(…)`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:574`
  documents `.extension_point(…)` as the replacement for the Cargo tag, and no `🪪️manifest/*/🦀️.rs`
  in the repo calls `.consumes(`. No `🔣️.json` / `🛂️.descriptor.semio` carries `consumes` either
  (the descriptor only models what a package PUBLISHES). **So the wiring is Cargo-only** — no
  manifest or descriptor mirror was needed.

**Runtime push (host → guest)**

- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:239` `buildContributionsJson` and `:301`
  `scopeContributionsJson` fold every loaded plugin's `manifest.topicContributions` into a
  `ProgramContributionEntry[]` JSON.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4615`
  builds the scoped pack; the `install` unit right below it invokes the **`setContributions`** app
  command on every loaded plugin whose app owns it.

**Guest side — what exists per topic**

| topic | guest decode | host command | verdict |
|---|---|---|---|
| `process.machines` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1890`, `:1920`, `:2020`; config `✏️editor/🎚️config/🦀️.rs:41` | yes | complete (template) |
| `cad.computer` | TS: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts:43`, `:127` `syncCadComputerContributions`, `:196` `bootstrapCadModules` (registers each `moduleId` through the cad-js `register()` hooks). Rust validation: `…/🧬️schema/💡️inferences/🦀️.rs:985`–`:1007` `validate_cad_computer_contributions`; config field `…/✏️editor/🎚️config/🦀️.rs:112` | `…/✏️editor/🦀️.rs:946`, `:1047`, `:1941`; handler `…/✏️editor/🎮️commands/🧩️contribution/🦀️.rs:10`–`:23` | **complete** |
| `sourcing.module` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:556` `ContributedSourcingModule`, `:600` payload, `:682` `contributed_sourcing_modules`, `:713` `sourcing_modules`, `:720` `module_for`, `:735` `available_modules`; config `…/✏️editor/🎚️config/🦀️.rs:26` | `…/✏️editor/🦀️.rs:185`, `:245`, `:455`, `:1011` | **complete** |

**Conclusion for step 3: no runtime feature was missing.** Both cad and sourcing already discover,
decode and route their contributed modules and both own a `setContributions` command. The only gaps
were metadata (§2) and one stale descriptor (§3).

## 2. Edits

| file:line | change |
|---|---|
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:16` | added `consumes = ["cad.computer"]` to `[package.metadata.semio]` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml:16` | added `consumes = ["sourcing.module"]` |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:26` | `consumes` extended to `["forms.questionKind", "flow.extension", "process.machines", "cad.computer", "sourcing.module"]` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🔣️.json` + `🛂️.descriptor.semio` | replaced the committed placeholder descriptor (see §3) with the real one |

No Rust manifest or `🔣️.json`/`🛂️.descriptor.semio` `consumes` mirror exists, so nothing else needed
to change; no manifest test was touched (none asserts `consumes`).

## 3. Second defect found: `cad-extension-aec-building`'s committed descriptor was a placeholder

`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🔣️.json` (committed in `21fbcd3538`) held

```json
{ "role": "plugin", "manifest": { "pluginId": "empty", … "topicContributions": [] },
  "execution": "isolated", "contributions": {} }
```

Because a descriptor *exists*, `parsePluginCargo` prefers it over Cargo, so the registry row read
`contributes: []` — the crate's `contributes = ["cad.computer"]` (`📦️packages/🦀️rust/Cargo.toml:17`)
and its `bundle()` topic (`🧩️extensions/🏢️aec-building/🦀️.rs:100`) were both invisible. With the
`consumes` wiring in place the closure still came out at **27**, missing exactly this component.

The `describe` target cannot regenerate it — it fails:

```
bun nx run @semio-tech/cad-extension-aec-building-rust:describe
semio-framework-plugin-describe describe: compiling …/semio_s_plugin_cad_aec_building.wasm with the
owned interpreter: plugin: wasm validation: component has no core module implementing the owned
Semio actor ABI
describe semio-s-plugin-cad-aec-building failed: descriptor emitter exited with 1
```

(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` — its owned
interpreter refuses a handler-less `ExecutionMode::Declarative` extension.)

The **materialize** path uses a different, working emitter
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:61`–`:70`,
`finalizePluginDescriptor`) and produced a correct descriptor
(`pluginId: cad-extension-aec-building`, `execution: declarative`, topic `cad.computer`, plus the
`s.cad.cad` artifact contributions). I copied that pair from
`…/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🏢️cad-extension-aec-building/` into the
extension owner root — the exact destination `describe` documents
(`🖨️describe/🏭️fresh-component/🟦️.ts:315`).

**Remaining**: `@semio-tech/cad-extension-aec-building-rust:describe` is still broken. It needs the
owned-interpreter describe path to accept declarative extensions (or the target to delegate to
`finalizePluginDescriptor`). Not fixed here.

## 4. Commands and outcomes

All under `DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings
CARGO_TERM_QUIET=true CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 NX_TUI=false
NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false`, foreground, one at a time. Logs in `🗑️generated/build-*.txt`.

| target | result |
|---|---|
| `@semio-tech/cad-extension-aec-building-rust:materialize-dev` | ok (3m31s, `component-dev` 2m26s) |
| `@semio-tech/cad-extension-aec-building-energy-rust:materialize-dev` | ok (7.8s) |
| `@semio-tech/cad-extension-aec-building-structure-rust:materialize-dev` | ok (5.6s) |
| `@semio-tech/cad-extension-spatial-shape-rust:materialize-dev` | ok (3.9s) |
| `@semio-tech/sourcing-extension-beams-rust:materialize-dev` | ok (`component-dev` 1m2s) |
| `@semio-tech/sourcing-extension-slabs-rust:materialize-dev` | ok (4.6s) |
| `@semio-tech/sourcing-extension-windows-rust:materialize-dev` | ok (4.8s) |

Nx project names are `…-extension-…-rust`; the **plugin ids** are `sourcing-module-*` (catalog
directory names differ again: `🏟️cad-extension-aec-building-structure`,
`🔷️cad-extension-spatial-shape`).

All seven dirs under
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/`
carry `.nx-artifact.json`, `🌉️bridge.js`, `🔣️.json` — verified.

Then, last:

- `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` →
  `plugin registry catalog refreshed (59 plugin crates, 61 playgrounds, 54 framework packages)`.
- `bun nx run @semio-tech/plugin-registry:check-generated` → **passed**.
  (`catalog-complete` is not runnable here: it requires `--build-root <absolute fresh build root>`.)
- Generated registry now shows `cad consumes ["cad.computer"]`, `sourcing consumes
  ["sourcing.module"]`, `demonstrator consumes ["forms.questionKind","flow.extension",
  "process.machines","cad.computer","sourcing.module"]`, and `cad-extension-aec-building
  contributes ["cad.computer"]`.
- `runtimeComponentClosure(registry, ["demonstrator","procedural"])` → **28 components**:
  cad, cad-extension-aec-building, cad-extension-aec-building-energy,
  cad-extension-aec-building-structure, cad-extension-spatial-shape, demonstrator, flow,
  flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}, gis, procedural, process,
  process-extension-{concrete,metal,robotic,wood}, puzzle, sourcing,
  sourcing-module-{beams,slabs,windows}.
- `bun nx show project @semio-tech/framework-os-dev --json` —
  `prepare-koordinator-react-dev` and `prepare-aussuchen-react-dev` now each depend on all 4
  `cad-extension-*`, all 4 `process-extension-*` and all 3 `sourcing-extension-*` `materialize-dev`;
  `prepare-cad-react-dev` on the 4 cad ones; `prepare-sourcing-react-dev` on the 3 sourcing ones.
- `cd ♻️mit-bestand/🧺️demonstrator && bun ./📜️script.ts test`:

```
 RUN  v4.1.10 /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator
 Test Files  2 passed (2)
      Tests  4 passed (4)
   Duration  2.39s
```

## 5. What remains

1. **Reachability cut on the runtime push.** `scopeContributionsJson`
   (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:301`) only forwards a *foreign* plugin's contribution when
   `contributionReachesKinds` (`:290`) finds one of the open document's dotted operator kinds inside
   the contribution payload. Selecting the components is therefore necessary but may not be
   sufficient for the koordinator/aussuchen panes: if a cad/sourcing document yields no matching
   dotted kinds, the pack is cut to `[]` and the guest falls back to its shipped defaults
   (`shippedCadComputerContributionsJson`, `🏃️runtime/🟦️.ts:51`; `default_contributions_json`,
   sourcing `✏️editor/🎚️config/🦀️.rs:75`). Needs a live boot with `[DEBUG] contributions …` console
   lines to confirm; not verified here.
2. **`describe` broken for declarative cad extensions** (§3) — the owner-root descriptor is only
   refreshable via the materialize emitter today.
3. No live demonstrator boot was performed; run2 (`mit-bestand-demonstrator:activate-dev`) was
   running concurrently and was not disturbed.
