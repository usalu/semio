# 🛍️ Flow Catalogue / Node-Graph Surface Bound — 2026-09-09

Lane: framework flow catalogue · node-graph surface · renderer engine · `AppActionRegistry`.
Ticket: `2026/09/09/PROCEDURAL-3D-END-TO-END`. Origin: `📓️unit-suite-2026-09-09.md` §3.1 / §3.3.

Live symptom that opened the lane:

```
plugin-ui.intake-budget-exhausted:1:procedural-main:74736
```

The `procedural-main` surface published a **74 736 step** UI document in a single
intake turn. The intake budget is finite (see
`📓️…/hidden-browser-pane-throttles-plugin-boot`), so the flow window never
mounted: the whole editor UI was dead on arrival on every boot with the brep +
math kernels installed.

> Status: **done**. Four instances; the first three died mid-flight (one rate limit, two stream
> stalls), so §1 onwards was written by the fourth against the state the third had committed at
> `6ad7b0e7bc` (2026-09-10 01:31), which carries the whole redesign, plus the runs and repairs below.

## 0. Result in one line

`procedural.play.main` went from **111 031 B and unrenderable** to **15 447 B**, and is now flat in the
installed operator count (862 B with 500 operators registered). §2 for the measurements, §3 for the runs.

## 1. The design

### 1.1 What was wrong

`ui_scene::encode` admits a WHOLE surface payload against one preallocated per-surface capacity,
`ui_contract::UI_FIXED_BYTES = 32 * 1_024`
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:22`). Every flow-backed
node-graph window embedded the ENTIRE registered operator catalogue in its own scene — twice, once as
`operators: Vec<NodeGraphOperatorRecord>` and once as `catalogue_json` (the palette sections built over
the same records). With an EMPTY operator registry that was ~14 KB and fit; with the real `brep` + `math`
operator sets installed — the live configuration — the catalogue alone is ~100 KB and the surface
measured **111 031 B, 3.4× the capacity**. `encode` refused, the window rendered nothing, and the boot
symptom was the intake budget dying on the retry storm:

```
plugin-ui.intake-budget-exhausted:1:procedural-main:74736
```

The catalogue is not per-surface data. It does not change while an app instance lives, it is identical
across every node-graph window of that app, and it is identical across the catalogue panel, the side
palette and the canvas spotlight. It was being paid for once per surface per frame.

### 1.2 The shape it moved to

The catalogue became a RESERVED APP-STATIC SECTION, a fourth peer of the three sections the framework
already publishes outside the window/panel body path.

- `semio_framework::UiRefreshSection` gained `Catalogue`
  (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4432-4445`): `UI_REFRESH_SECTION_KEYS` went `[&str; 3]` →
  `[&str; 4]` (`…, "catalogue"`), `UI_REFRESH_SECTION_BODY_KEYS` gained
  `"framework.section.catalogue"`, and `UiRefreshSection::ALL` went `[Self; 3]` → `[Self; 4]`. The
  TypeScript twin is `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:1155`.
- `ArtifactApp::app_catalogue_json` is the app's override point
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11151-11160`, default `"{}"`), object-safe
  counterpart at `:11717-11721`, and the section resolver dispatches to it at `:30799`:
  `semio_framework::UiRefreshSection::Catalogue => resolve_ready(instance.app.app_catalogue_json())`.
  Every app wrapper (`EditorApp`, `ViewerApp`, `VcsArtifactApp`, the derive shims) forwards it.
- Flow builds the payload: `FlowAppCatalogue { operators, sections }` plus `flow_app_catalogue()` /
  `flow_app_catalogue_json()`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs:169-202`, the struct at `:183`, the builders at `:195`/`:200`).
- `FlowBackedNodeGraphExtras` lost BOTH `operators` and `catalogue_json`
  (`…/🗂️catalogue/🦀️.rs:219-226`; its builder at `:312`), and `catalogueJson` left the wire record entirely —
  `NodeGraphScene` in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json:23`
  no longer declares it. `operators` stays declared (a non-flow node graph may still carry a small
  inline set) but every flow-backed scene now leaves it empty and names operators by KIND ID only,
  from inside `fixtureJson`.
- The renderer fetches it once and caches it: `uiRefreshWantsCatalogue`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:3995-4001`)
  returns true only for a FULL `UiDirtyScope` — the catalogue has no dirty flag of its own because it
  cannot go stale within an app instance — and `buildUiRefreshRequest` carries the cached hash, so the
  repeat costs one hash compare instead of a ~100 KB re-serialize. The value reaches every scene host
  through `AppCatalogueContext` / `useAppCatalogue`
  (`…/🧱️elements/🗣️Interpreter/🟦️.tsx:565-579`), fed from `Shell`'s `appCatalogue`
  (`…/🧱️elements/🐚️Shell/🟦️.tsx:437-441`). The TS type is `AppCatalogue`
  (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:502-527`), mirroring the Rust `FlowAppCatalogue`.

### 1.3 The second defect the same lane had to fix — `AppActionRegistry`

Fixing the surface exposed generation2d's 24 `interactive-job.catalog-authority … migrated={}`
rejections (unit-suite §3.3): `migrated_tool_ids()` reads `AppActionRegistry.actions`, and
`AppActionRegistry::from_definition`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11861-11868`) built that index ONLY from
`window_kinds[].actions`. An `AppDefinition` written as a plain struct literal — legal, every field is
`pub`, and what generation2d does — never went through `AppBuilder::try_build_definition`'s injection
pass, so its interaction verbs were absent and every bounded tool proof keyed on them failed closed
with an empty `migrated` set. `from_definition` now seeds `actions` from
`semio_framework::interaction_action_definitions(definition)` first and lets the per-window rosters
overwrite it, so an authored window declaration still wins over the injected twin.

### 1.4 Three collateral fixes the same symptom needed

- **The intake pump could spin.** `OwnedUiSurfacePatch`'s staging generator
  (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts:369-378`) held two
  `while (…) yield 16;` busy-waits on a cell that was active-but-uninitialised or out of lease capacity.
  Each spin burned intake budget without advancing the scan; that is what turned one refused surface into
  `intake-budget-exhausted:…:74736`. Both became `if (…) { yield 16; continue; }` — re-enter the scan loop
  instead of pinning it on one cell.
- **`UiText` had no lossy path.** Long operator summaries could refuse admission outright. `UiText::clipped`
  (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:132-152`) takes the longest
  char-boundary prefix leaving room for `UI_TEXT_CLIP_MARK` (`"…"`) and appends it, for descriptions and
  summaries only — never identifiers or values.
- **The catalogue PANEL was flat.** `flow_palette_catalogue_sections` offers hundreds of entries with the
  real kernels; the panel pushed all of them into one `UiFixedList` and refused at the 33rd with
  `ui.catalogue.items: fixed UI catalogue admission failed`. The build is now virtualised —
  ONE bounded page of `semio_framework_plugin::panel_page_rows` rows across every group, each truncated
  group closed by a continuation row naming the omitted count
  (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:1-12`).
  The unbounded browse surface is the canvas spotlight, which reads the app-static catalogue.

## 2. Bytes, before → after

Measured with the real `brep` + `math` operator sets installed (unit-suite recipe §2.7's
`🧪️tests/🔬️flow-operators/🦀️.rs` one-time `install_flow_extension`), `--nocapture` on the
surface-bound laws. Raw logs `🗑️generated/cat-b1-500ops.txt`, `cat-b2-3dbytes.txt`, `cat-b3-2dbytes.txt`.

| surface | before | after | instrument |
|---|---|---|---|
| **`procedural.play.main`** (the flow / node-graph window, `hexagonal-mushroom-column`) | **111 031 B** — REFUSED at `scene-surface.encode`, 3.4× the 32 768 B fixed admission | **15 447 B** | `ui.fixed-capacity` fault text (before) → `[STATS] surface example=hexagonal-mushroom-column body=procedural.play.main` (after) |
| `procedural.play.catalogue` (the catalogue PANEL) | refused — `ui.catalogue.items: fixed UI catalogue admission failed` at the 33rd row | 10 555 B | `[STATS] … body=procedural.play.catalogue` |
| node-graph surface with **500** registered operators | would have been ≫ capacity (the catalogue alone is 197 882 B at that size) | **862 B** | `[STATS] node-graph surface with 500 registered operators: surface=862 B of 32768 B, app catalogue=197882 B` |

The node-graph surface is now flat in the operator count — 862 B with 500 operators registered against
15 447 B for a real 30-node fixture — because what is left in the scene is the graph, not the vocabulary.
Every other generation3d body for every one of the eight bundled examples is unchanged and small
(`preview` 4 056 B, `generations` 762 B, `document` 574–1 042 B, `generate-preview` 1 222 B,
`inspection` 493 B, `generate-form` 117 B).

The 197 882 B app catalogue is paid ONCE per app instance on the reserved catalogue surface, and the
cached hash makes every later full scope free.

The generation2d twin (`cat-b3-2dbytes.txt`, small because its crate's own lib test links no geometry
kernel — see §3's note on that registry): `generation2d.play.main` 5 597 B, `catalogue` 3 282 B,
`inspection` 638 B, `document` 544 B, `preview` 348 B, `generate-preview` 120 B, `generate-form` 117 B,
`generations` 762 B. What its law pins is not the size, it is the SHAPE: `scene.operators` must be empty
and no body may grow with the installed operator set.

## 3. Runs

Private `CARGO_TARGET_DIR=$S/target-cat` (native) and `$S/target-cat-wasm`, `RUSTC_WRAPPER=""`,
`RUST_MIN_STACK=536870912`, `--test-threads=2` (unit-suite §0: `--test-threads=1` overruns the main
thread's 8 MiB macOS stack and aborts the whole binary). Raw logs in `🗑️generated/cat-*.txt`.
`cargo test` takes `--no-fail-fast`, NOT `--keep-going` — the latter is rejected outright, which is how
the first attempt of this batch produced five instantly-empty logs.

### 3.1 Compilation

| target | result | log |
|---|---|---|
| `cargo check --keep-going -p semio-framework-os-flow -p semio-framework-ui -p semio-s-plugin-procedural -p semio-s-plugin-flow` | **0 errors**, `Finished dev profile in 1m 38s`; warnings emitted by `semio-framework-ui-runtime`, `semio-framework-plugin`, `semio-framework-os-flow`, `semio-s-artifact-flow-flow` (11), `…-generation2d` (2), `…-generation3d` (1) — warning output is the proof the crates really expanded and type-checked rather than aborting early | `cat-c1.txt`, `cat-c3-final.txt` |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check --keep-going -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev` | **0 errors**, `Finished wasm-dev profile in 2m 56s`, same warning set | `cat-c2-wasm.txt` |

### 3.2 Rust tests

| suite | result | the lane's own tests | log |
|---|---|---|---|
| `-p semio-framework-os-flow --lib` | 79 passed / 125 failed — **identical to the pre-redesign baseline** (79/125 measured at 00:43 before any of this landed), so this lane introduced none of them: 81 are `final Dictionary ownership must be explicitly retired` inside `🧠️neural/⚙️engine`, 30 are `OrderedMap` root drops in `📡️replication` — the §2.3 owned-projection law, other lanes' | `host::tests::a_node_graph_surface_stays_under_the_fixed_admission_with_five_hundred_operators` **ok**, `host::tests::app_catalogue_carries_every_operator_and_palette_section` **ok** | `cat-t1-flowlib.txt` |
| `-p semio-framework-plugin --lib app_action_registry` | 1 passed / 0 failed | `component::app::app_builder_tests::app_action_registry_indexes_top_level_app_actions_and_their_migrated_disposition` **ok** | `cat-t2-registry.txt` |
| `-p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` | **287 passed / 16 failed** (was 244/51 at the unit-suite lane's close, 127/168 at the start of the ticket) | `component::tests::every_window_and_panel_surface_fits_the_resident_surface_bound` **ok**, `…::the_first_turn_sequence_retires_every_flow_host_it_builds` **ok**, `…::generation3d_labels_translate_catalogue_and_inspector_in_german` **ok**, `panels::catalogue::tests::generation3d_labels_resolve_native_english_by_default` **ok**, `modes::edit::windows::flow::tests::{renders_node_graph_scene, main_graph_scene_exports_flow_backed_node_graph_fields, the_app_catalogue_section_carries_the_registered_operators}` **ok** | `cat-t4-3d.txt` |
| `-p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` | **222 passed / 5 failed** (was 179/46) | the 2d twins of all of the above **ok**, plus `panels::inspection::tests::generation2d_labels_translate_catalogue_and_inspector_in_german` and `modes::edit::windows::flow::tests::definition_declares_the_node_graph_surface_and_body_key` | `cat-t5-2d.txt` |
| `-p semio-s-artifact-flow-flow --lib` | 122 passed / 90 failed — the crate did not COMPILE before this lane (§3.4); this is its first measurable state | `component::tests::interaction_topology_registers_every_widget_and_synapse_as_a_root` **ok**, `modes::edit::windows::main::component::tests::definition_declares_the_node_graph_surface_and_body_key` **ok**, `viewer::…::definition_declares_the_node_graph_surface_and_body_key` **ok** | `cat-t3-flowartifact.txt` |

generation3d's 16 remaining failures are all preview/tessellation/geometry
(`renders_world_preview_scene`, `switching_active_example_changes_preview_meshes`,
`preview_payload_has_meshes_and_instances`, `render_uses_the_configured_preview_camera`,
`patch_flow_widgets_recomputes_preview_geometry`, `translate/rotate/scale-selection` persistence),
store/vcs (`two_instances_converge_disjoint_widget_moves` fails at `attach b` with the DECLARED
`module.vcs` fail-closed remote-snapshot merge, `vcs_artifact_app_non_empty_retained_maintenance_swap`),
or the fold-contract envelope. None is a surface, catalogue, or registry failure.

generation2d's 5: `import_params_in_patches_matching_input_slider`,
`vcs_artifact_app_non_empty_retained_maintenance_swap…`, two retained/mounted binary laws, and
`two_instances_converge_disjoint_widget_moves` — see §3.3.

### 3.3 The 24 `catalog-authority … migrated={}` proofs

All 24 are gone. One catalog-authority failure remains in generation2d, and it has a DIFFERENT root
cause: `two_instances_converge_disjoint_widget_moves` goes through
`semio_framework_plugin::testkit::assert_two_instances_converge`, which builds both sides with
`paired_apps` → `new_app` — the deliberately REGISTRY-LESS wrapper
(`🔌️plugin/🦀️.rs:6596-6600`, "UI and typed command dispatch fail closed until a real registry and
exact factory are supplied"). An app carrying `bounded_first_step_tool_proofs!` cannot be built that
way at all, so no `from_definition` fix can reach it. The registered twin
(`assert_two_registered_instances_converge`, `:6853`) is the correct helper, but switching the call
site only moves the failure onto the same DECLARED `module.vcs` fail-closed merge its generation3d
twin already hits — so it is left as a named follow-up for the store/vcs lane rather than relabelled here.

### 3.4 Two compile errors this lane had to clear to run the flow artifact at all

`semio-s-artifact-flow-flow`'s lib test did not compile — nothing to do with the catalogue, both from
peer refactors landed earlier the same day, both blocking the flow node-graph tests this lane owes:

- `dsl::Fault` no longer implements `Display` (the `⚠️diagnostic` refactor), so
  `error.to_string()` / `"{error}"` no longer compile:
  `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🎮️commands/➕️add-widget/🧪️tests/🔬️unit/🦀️.rs:112` now reads
  `error.message` and formats `{error:?}`.
- `FlowPlayApp` declares `type Config = NoConfig` (the app/window config split), while
  `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:277` still built a `ConfigView` over `FlowMainWindowConfig`;
  it now builds `semio_framework_plugin::NoConfig::default()`.

### 3.5 Renderer TypeScript

`bun nx run @semio-tech/framework-renderer-react:test-long` — **20 files, 756 tests, all passed**
(`cat-ts1.txt`), including the two catalogue laws in `🧪️tests/🔬️engine-contract/🟦️.ts:2948-2965`:
`buildUiRefreshRequest asks for the app catalogue on a full scope only, carrying its cached hash` and
`applyUiRefreshResponseToCache caches the app catalogue and leaves it untouched on an unchanged hash`.
`test-long` is the level that restores the engine suite corpus — `fundamental`/`quick` select the single
bounded resident-composition file, so a green `test`/`test-quick` would NOT have exercised these.

### 3.6 A re-check at 02:15 caught two peer lanes mid-edit

The four-crate `cargo check` was clean at the start of this instance (`cat-c1.txt`, 0 errors,
`Finished dev profile in 1m 38s`). A confirming re-run 40 minutes later (`cat-c3-final.txt`) showed 6
errors in files this lane never touches, all from concurrent sessions editing the tree live:

- `✏️s/🔌️plugins/🌀️procedural/…/🧊️generation3d/…/✏️editor/🦀️.rs:1181` — `EXTENSION_RESPONSE_TOOL_CONTRACT`
  not in scope. It IS in scope now, declared at `:290`; the file's mtime was the same minute as the
  check. The peer added the use site before the const.
- `✏️s/🔌️plugins/🌀️procedural/…/🌀️generation2d/…/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:190,549,728` —
  `Generation2dMountedRecordOwner::NeuralValue`'s fields are being reshaped from `{table, row, value}`
  to `{owner}` and `Generation2dMountedContainerOwner` is gaining `DictionaryEntries`/`DictionaryEntry`.

Both are mid-refactor states of other lanes, not regressions from this one. `cargo check -p
semio-framework-os-flow -p semio-framework-ui` on the final tree is **0 errors, 21.77s**
(`cat-c4-lane.txt`), and both procedural suites in §3.2 compiled and ran to completion on this lane's
sources.

## 4. Follow-ups this lane deliberately did not take

1. **`semio-s-artifact-flow-flow`'s own tool-proof join.** With `migrated` no longer empty, the flow
   artifact's proof now names the real gap: 13 of its 34 generated declarations are classified
   `InteractiveJobClassification::BatchOnlyPendingRewrite`, not `Migrated`
   (`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:2300-2327` — `addGeneration`, `removeGeneration`,
   `selectGeneration`, `renameGeneration`, `updateGenerationValues`, `reorganize`, `renameFlowWidget`,
   `nodeGraphEdit`, `spotlightCommit`, `runExtensionAction`, `focusSelection`, `duplicateWidget`,
   `connectMediaPorts`), so `migrated_tool_ids()` legitimately omits them and the app cannot be
   constructed. Each needs a real owned reducer before it can be reclassified. Owner: flow editor lane.
2. **`assert_two_instances_converge` is registry-less by construction** — see §3.3. Owner: store/vcs lane.
3. **`NodeGraphScene.operators` still exists in the wire record** and is simply always empty for
   flow-backed scenes. It is kept for a non-flow node graph carrying a small inline set; if no such
   consumer appears, it should be deleted from
   `🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json` too.

## 5. Files changed

### Framework — UI contract, scene wire and intake
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json` — `catalogueJson` dropped from `NodeGraphScene`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs` + `🧪️tests/{🔬️scenes-unit,🔬️scenes-value-round-trip}/🦀️.rs` + `🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs` (`UI_TEXT_CLIP_MARK`, `UiText::clipped`), `🏗️builder.rs`, `🧪️tests/🔬️action-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts` — the two intake busy-waits
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs`, `🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/**` (`📤️output`, `♻️reconcile`, `🎭️present` + their tests)
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`

### Framework — reserved catalogue section
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` and `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — `UiRefreshSection::Catalogue`
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts` — the `AppCatalogue` TS record

### Framework — flow
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs` — `FlowAppCatalogue`, `flow_app_catalogue{,_json}`, `FlowBackedNodeGraphExtras` shrunk, `flow_backed_node_graph_extras` retires its status host
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` — the 500-operator bound law and the app-catalogue law

### Framework — plugin
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `ArtifactApp::app_catalogue_json` (+ every wrapper), the `UiRefreshSection::Catalogue` resolver, `AppActionRegistry::from_definition`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs` — the registry law

### Renderer (TypeScript)
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — `AppCatalogueContext`, `useAppCatalogue`
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx` — fetch, `preserveJsonIdentity` caching, the provider
- `…/🧱️elements/🐚️Shell/🟦️.tsx` — `appCatalogue` on the window-UI state
- `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `uiRefreshWantsCatalogue`, request/cache plumbing
- `…/🧱️elements/🕸️NodeGraph/🟦️.tsx` — `syncFlowSessionAppCatalogue`, operator infos from the context
- `…/🧪️tests/🔬️engine-contract/🟦️.ts`, `…/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts`, `🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`

### Plugins
- flow: `✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs` (`app_catalogue_json` override); `✏️editor/🧪️tests/🔬️unit/🦀️.rs` and `✏️editor/🎮️commands/➕️add-widget/🧪️tests/🔬️unit/🦀️.rs` (§3.4 compile repairs)
- generation3d: `✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs`, `✏️editor/📌️panels/🛍️catalogue/🦀️.rs` (virtualised page), `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- generation2d: `✏️editor/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (the surface-bound twin at `:441-455`)
