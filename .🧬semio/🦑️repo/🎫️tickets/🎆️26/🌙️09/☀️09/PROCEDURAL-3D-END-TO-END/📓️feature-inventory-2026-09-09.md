# 🧊️ Procedural Generation3d — user-perspective feature inventory (2026-09-09)

Read-only audit of `s.procedural.generation3d@1` at
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer,🧬️schema,🧪️tests,🧫️fixtures,📚️examples,🚪️io,🔮️oracle}`,
root crate `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`. No build was run — every claim below is
grounded in an actual read of the file cited next to it. Structural template:
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-08-feature-inventory.md`.

## Facts already established by earlier auditors (carried forward, not re-derived)

All 14 mutations (`disconnect-synapse`, `delete-widget`, `create-generation`, `create-widget`,
`rename-generation`, `move-widget`, `update-camera`, `update-synapse`, `connect-synapse`,
`change-schema`, `change-generation-value`, `delete-generation`, `delete-widget-position`,
`update-widget`) have a thin `MutationKind` root delegating to real `🔺️diff/🦀️.rs` +
`↩️inverse/🦀️.rs` logic, one test scenario each with the identical 7-fn pattern, real assertions, all
five fixture files present. No `todo!`/`unimplemented!`/`FIXME` in the plugin. gen3d editor has 32
command dirs (gen2d 22, assembly 0) — this audit counted 31 command directories plus one bare `generation`
helper module and confirms the 27-command dispatch enum below. Modes `✏️edit`/`🧬️generate`; edit windows
`👁️preview`/`🕸️flow`. Catalogue panel has a `🪪️identity` facet. All 27 editor actions at
`✏️editor/🦀️.rs:1289-1315` are `InteractiveJobClassification::Migrated`. Only stubs previously recorded:
txt import/export `Err("… not yet implemented")` under `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/…/🦀️.rs:10,13`
and `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/…/🦀️.rs:10,13`. `preview_selection_json` hardcodes
marquee `"rectangle"` (`✏️editor/🦀️.rs:1534-1539`, confirmed at line 1542 in this pass with unchanged
content). Shared flow machinery is imported from `semio_framework_artifact_flow_flow` and
`semio_framework_os_flow`.

## TL;DR — new findings this pass, ranked by user impact

1. **Every non-JSON/txt export/import format silently produces wrong or empty data — 7 of 9 IO
   formats are non-functional placeholders, not stubs.** `serialize_bytes` for `las`, `png`, `stl`,
   `dwg`, `obj`, `gltf` and `ply` all do the exact same thing —
   `Ok(<Generation3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())` — i.e. every
   "export to STL/OBJ/glTF/…" silently returns the artifact's own native DSL *text*, not any real mesh
   bytes of the named format (`🚪️io/📤️export/🧵️serializers/🗿️artifacts/{☁️las,📷️png,🔺️stl,🖊️dwg,🗿️obj,🧊️gltf,🧱️ply}/…/🦀️.rs`,
   8 lines each, byte-identical bodies). The import side is worse: all 7 `deserialize`/`deserialize_bytes`
   discard the input (`let _ = bytes;` / `let _ = (SCHEMA, from);`) and return
   `Ok(Generation3dSnapshot::default())` — importing a real LAS/PNG/STL/DWG/OBJ/glTF/PLY file silently
   yields an *empty* document with no error at all (`🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{same 7}/…/🦀️.rs`).
   Only `json` (real, via `semio_s_artifact_stdio_json`) and `txt` (an honest `Err(...)` stub) behave as
   labeled. **Fix direction**: either wire these 7 to real per-format codecs or make them return an
   explicit "not implemented" error like `txt` does — silently emitting/discarding data is strictly worse
   than the already-known txt stub.
2. **`context_menu` hardcodes an empty selection, so every selection-dependent right-click item is
   permanently hidden.** `let selected: Vec<String> = Vec::new();` at `✏️editor/🦀️.rs:219` (own doc
   comment at 205-207 confirms: "carries no `InteractionView` either… so the selection-dependent rows
   below always take the nothing-selected branch"). `translateSelection`/`rotateSelection`/`scaleSelection`,
   the `targets` group (`removeWidget`/`removeGeneration`) and the delete-selection item never appear in
   the context menu regardless of actual selection state (`✏️editor/🦀️.rs:221-234`). Unlike puzzle3d's
   TL;DR #1, this does **not** affect `render`/`render_with_request_context` — those two do receive a
   real `InteractionView` (`✏️editor/🦀️.rs:179-189`) and hover/selection/gumball ARE live in gen3d (see
   §3). The gap is narrower: only the right-click menu's item visibility, not the whole render pipeline.
   All the actions themselves remain reachable via the palette/toolbar. **Fix direction**: thread the same
   `InteractionView` the framework already gives `render_with_request_context` into `context_menu`.
3. **Two fully-implemented commands (`flow-eval-resolve`, `flow-tessellate-resolve`) are dead code —
   declared, real, and never dispatched from anywhere.** Both modules are `pub mod`-declared at
   `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:679,683` with genuine bodies
   (`FlowEvalResolve::handle` calls `session.seed_node_cache(...)` and re-arms `flowEvalTick`;
   `FlowTessellateResolve::handle` calls `session.resolve_preview_tessellate(...)`), but neither struct
   appears in `Generation3dCommand`, `command_from_action`, or anywhere in the repo's `.ts`/`.tsx` tree — a
   repo-wide grep for `flowEvalResolve`/`flowTessellateResolve`/`FlowEvalResolve`/`FlowTessellateResolve`
   returns only the two files themselves and the one `mod` declaration. Neither can be triggered by any UI
   action today. A third command directory, `👥️set-contributions`, is worse: it is an **empty directory
   with no `🦀️.rs` at all** and no `pub mod` anywhere — pure scaffold, never compiled.
4. **A 9th example (`demo-session`) exists with a full `ExampleSource` but is excluded from the
   manifest.** `✏️editor/📚️examples/🎬️demo-session/🦀️.rs` declares `ID`/`label()`/`ICON`/
   `PRIMARY_TEXT`/`source()` identically to the 8 real examples, and its `.cmd.semio` asset
   (`action=demo`) is real content — but `examples()` at `✏️editor/🦀️.rs:1378-1387` lists exactly the 8
   bundled DSL examples and never calls `crate::examples::demo_session::source()`. It cannot appear in the
   navbar dropdown or `setActiveExample`'s arg picker. `examples_match_set_active_example_select_options`
   (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:917-936`) asserts exactly 8, which passes — meaning this exclusion is
   either deliberate or a forgotten wire-up, but either way `demo-session` is unreachable from the UI.
5. **No example has a test that proves it actually tessellates.** All 8 examples' tests
   (`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`) are the identical one-liner `primary_asset_is_nonempty`
   (`text.len() > 8`) — a text-length check on the raw DSL, not a parse/evaluate/tessellate assertion. Only
   `📐️box-fillet-preview` additionally has `inference_determinism_law`/`inference_default_law`
   (`📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🦀️.rs`), which test the schema's `Inference`
   type, not that the flow graph evaluates to non-empty preview geometry. Nothing in this artifact's test
   suite proves any of the 8 examples renders a visible 3D preview.

## 1. Windows, panels, layout

**Editor manifest** (`create_generation3d_app`, `✏️editor/🦀️.rs:241-291`): two modes, five window kinds,
three panel tabs.

- **Mode `edit`** (`GENERATION_3D_PLAY_MODE_EDIT = "edit"`, default mode,
  `🎭️modes/✏️edit/🦀️.rs:7-16`) — default row layout `[Flow 68% | Preview 32%]`
  (`create_default_layout(&[flow, preview], "row", [68.0, 32.0], ["Flow","Preview"])`,
  `🎭️modes/✏️edit/🦀️.rs:14-16`). Two windows:
  - `procedural-main` ("Flow"/"Workflow") — `🕸️flow` window,
    `🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:12-35`. `SurfaceKind::NodeGraph`. Renders a real
    `NodeGraphScene` off `fixture_to_workflow`, with live hover/selection (`NodeGraphHover`,
    `marks.graph_selection_ids()`/`graph_highlight_ids()`) and a real LOD select measure
    (Coarse/Medium/Fine → `setLodMode`, `🕸️flow/🦀️.rs:38-51`).
  - `procedural-preview` ("Preview"/"Vorschau") — `👁️preview` window,
    `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:11-33`. `SurfaceKind::World3d`. Renders a real
    `World3dScene` (`preview_payload`/`preview_selection_json`), chrome measures = Show-mode select
    (Shaded/Shaded+edges/Wireframe/Points → `setShowMode`) + sun group (→ `toggleSun`/`setSunAzimuth`/
    `setSunElevation`/`setSunIntensity`), `🪟️windows/👁️preview/🦀️.rs:36-58`. `utilities: Vec::new()` —
    **no gumball/brush/fill tool row exists for this app at all**; the only interaction surface is the
    node graph.
- **Mode `generate`** (`GENERATION_3D_PLAY_MODE_GENERATE = "generate"`, named layout
  `generation3d-generate`, `🎭️modes/🧬️generate/🦀️.rs:6-35`) — row layout `[Generations 22% | Form 43% |
  Preview 35%]`. Three windows:
  - `generation3d-generations` ("Generations"/"Generationen") — real tree of
    `GenerationPlayState` via shared `crate::generation_tree` (`🪟️windows/🗂️generations/🦀️.rs:34-36`).
  - `generation3d-generate-form` ("Form"/"Formular") — real editable form
    (`flow_fixture_to_form_spec` + `crate::generation_form`, bound to `updateGenerationValues`); shows
    `generate_hint` ("Add a generation to edit input values.") when no generation is selected
    (`🪟️windows/📝️form/🦀️.rs:36-43`).
  - `generation3d-generate-preview` ("Preview"/"Vorschau") — real `World3dScene` over
    `generation_fixture_for(fixture, generation)`; falls back to a `TextEditorScene` showing
    `preview_hint` ("(evaluate a generation to preview output)") when nothing has been generated yet
    (`🪟️windows/👁️preview/🦀️.rs:56-68`, `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`).

**Panel tabs** (all real, `panel_tab_def` at `✏️editor/🦀️.rs:257-259`):
- **Document** (framework "Artifact" tab, German "Dokument") — flat widget tree, item ids = raw widget
  ids matching the `graph` interaction domain 1:1, icon `cpu` per row, `interaction_domain("graph")`
  wired (`📌️panels/🗿️artifact/🦀️.rs:41-44`).
- **Catalogue** ("Katalog") — real, sourced from the shared `semio_framework_os_flow::flow_palette_catalogue_sections()`;
  each row dispatches `addWidget` with a `kind` arg; carries a `🪪️identity` facet
  (`📌️panels/🛍️catalogue/🪪️identity/🦀️.rs`) that derives a stable tree-item key from
  `(kind, neuron_kind, format, action)` (`📌️panels/🛍️catalogue/🦀️.rs:25-48`).
- **Inspection** ("Inspektion") — real, and unlike puzzle3d's dead inspector this one is genuinely
  selection-driven: `inspection_panel::render(&document.fixture, &marks.graph_selection_ids(), labels)`
  (`✏️editor/🦀️.rs:163`) receives live `graph` selection from `PreviewInteractionMarks`. Shows an editable
  numeric field bound to `patchFlowWidgets` for a selected `InputSlider`, plus read-only rows per widget
  kind (`InputNote`/`Neuron`/`Variable`/`OutputAction`/`OutputExport`)
  (`📌️panels/🔍️inspection/🦀️.rs:27-104`).

## 2. Every editor action — classification, Work type, real vs stub

`Generation3dCommand` (`✏️editor/🦀️.rs:59-91`) declares **27 rows**, all `Migrated`
(`✏️editor/🦀️.rs:1289-1315`), all proofed with a real `factory_type: Generation3dBoundedCommandJobFactory`
(not a bare/unowned factory — `✏️editor/🦀️.rs:902`, `bounded_first_step_tool_proofs!` block at 896-931).
`build_tool_job` (`✏️editor/🦀️.rs:859-895`) routes 6 of the 27
(`setActiveExample`/`addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues`/
`selectGeneration`, `GENERATION3D_PREVIEW_TOOL_IDS` at `🦀️.rs:204`) through `Generation3dPreviewCommandWork`
(a real chunked flow-eval preview job, `🦀️.rs:225-337`); the other 21 through `BoundedArtifactCommandWork`
wrapping `generation3d_retained_reduce` (`🦀️.rs:345-370`), which special-cases 5 selection-aware commands
(`nodeGraphEdit`/`deleteSelection`/`translateSelection`/`rotateSelection`/`scaleSelection`) to read the
real `graph` interaction-domain selection off `protocol::InteractionState`, and falls through to the
generic `command.dispatch(...)` for the rest.

| Action id | Real / stub | Notes |
|---|---|---|
| `setActiveExample` | Real | Swaps fixture; preview-tool-id chunked job |
| `nodeGraphEdit` | Real | `setFixture`/`deleteSelection`/`connect` sub-ops (`🎮️commands/✏️node-graph-edit/🦀️.rs:24-52`) |
| `deleteSelection` | Real | Selection-aware, reads live `graph` selection |
| `removeWidget` | Real | `host.remove_widget`, selection auto-pruned by framework |
| `moveMediaNode` | Real | `host.move_widget` |
| `addWidget` | Real | Builds a kind descriptor (`inputSlider`/`neuron|<kind>`/other), `host.add_widget` |
| `patchFlowWidgets` | Real | Patches `InputSlider.value` for matching widget ids |
| `reorganize` | Real | `host.reorganize({"orientation":"leftRight"})` |
| `translateSelection` | Real | Selection-aware |
| `rotateSelection` | Real | Selection-aware |
| `scaleSelection` | Real | Selection-aware |
| `addGeneration`/`removeGeneration`/`renameGeneration`/`updateGenerationValues`/`selectGeneration` | Real | Preview-tool-id chunked job, real generation mutations |
| `nodeGraphViewport` | Real | Config-only, sets flow-graph camera |
| `worldPointerDown` | No-op by design | `HostOnly` lane, `Ok(Emit::default())` (`🎮️commands/🌍️world-pointer-down/🦀️.rs:14-16`) — host-side pick passthrough, same pattern as puzzle3d's own `worldPointerDown` |
| `graphPointerDown` | No-op by design | Same pattern (`🎮️commands/🖱️graph-pointer-down/🦀️.rs:14-16`) |
| `setLodMode`/`setShowMode` | Real | Config-only display toggles |
| `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` | Real | `apply_world3d_sun_action` + `SetSun` config mutation |
| `setCamera` | Real | Sets `Generation3dPreviewCamera` |
| `flowEvalTick` | Real | Drives the incremental flow-eval session, chains itself while `session.tick()` reports more work, arms tessellate effects on completion (`🎮️commands/⏱️flow-eval-tick/🦀️.rs:14-32`) |

Genuinely dead: `flow-eval-resolve`, `flow-tessellate-resolve` (real bodies, never dispatched — TL;DR
#3), and `set-contributions` (empty directory, never compiled).

## 3. Viewer (`👁️viewer`)

`Generation3dViewer` implements `ArtifactViewer` only (never touches `✏️editor`,
`👁️viewer/🦀️.rs:1-5`). `Config`/`Presence`/`Transient` are all `NoConfig`/`NoPresence`/`NoTransient`
(`👁️viewer/🦀️.rs:43-48`) — **no persisted camera, no selection/hover, no presence, no commands** beyond
a single inert `Generation3dViewCommand::Noop` (`👁️viewer/🦀️.rs:20-33`); `handle` always returns
`ViewEmit::default()` (`👁️viewer/🦀️.rs:62-71`). One mode (`view`, single full-pane Preview window,
`🎭️modes/👁️view/🦀️.rs`). The Preview window (`MeshWindowKit`) re-evaluates the **whole** flow fixture
fresh on every render via `FlowHost`/`tessellate_geometry` directly — no session cache, no incremental
tick chain, by explicit design (`🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:8-13,162-167,224-227`). Camera
is a hardcoded default (`position=[4,-4,3] target=[0,0,0] fov=45`,
`🪟️windows/👁️preview/🦀️.rs:24-26`). Selection is always empty (`world3d_selection_json("rectangle", &[],
None)`, `🪟️windows/👁️preview/🦀️.rs:234`) — consistent with the viewer being read-only by construction.

## 4. Schema

`🧬️schema/🔣️.json` top-level: `Generation3dArtifact { fixture: FlowFixture, generation:
GenerationPlayState }`, both `required`. `FlowFixture = { schema, camera: CameraJson{x,y,zoom}, widgets:
Widget[] (opaque JSON string, "contentMediaType": "application/json"), synapses:
SynapseSpec{id,from,to,fromPort,toPort}[], layout: {[widgetId]: WidgetLayout{x,y}} }`.
`GenerationPlayState = { generations: FormGeneration{id,name,valuesJson}[], selectedGenerationId?,
previewText? }`. Two more `$defs` (`Generation3dPreviewCamera`, `Generation3dStringList`) are present in
the same generated JSON file but belong to `Generation3dConfig`
(`✏️editor/🎚️config/🦀️.rs:69-90`: `lod_mode`, `show_mode`, `camera: CameraJson`, `preview_camera:
Generation3dPreviewCamera`, `sun_json`, `selected_generation_id`, `preview_eval_text`), not the artifact
schema's own required fields — this is a shared-`$defs` JSON artifact of the schema generator, not a bug.

`💡️inferences` — present with the standard multi-representation set (`.json`/`.ts`/`.rs`/`.proto`/binary
grammar files) under `🧬️schema/💡️inferences/{💾️binary,📝️text}`.

`📸️snapshot/🦀️.rs` (26 lines) — `Generation3dSnapshot { fixture: FlowFixture, generation:
GenerationPlayRoot }`, `#[artifact_schema(id = "s.procedural.generation3d")]`, real `Default` impl.

`🔺️diff/🦀️.rs` (34 lines) — `Generation3dDiff { artifact: Option<Box<Generation3dArtifact>>, fixture:
Option<FlowFixture>, generation: Option<GenerationPlayRoot> }`, sparse field delta, real.

## 5. Examples (8 bundled, `examples()` at `✏️editor/🦀️.rs:1378-1387`)

All 8 share one shape: `🖼️assets/{name}/🗣️.dsl.semio` (primary text, real flow-graph DSL) +
`🖼️assets/🎒️.pack.semio` + `🖼️assets/📡️{name}.spr.semio` + `🖼️assets/🔧️{name}.op.semio` (a near-empty
op header, `semio procedural.generation3d.op v1 / edit reuse started="0" actor=example` — the real content
lives in the `.dsl.semio`). Op chains read directly from each DSL:

| Example | Op chain | Expected preview |
|---|---|---|
| 🍄 hexagonal-mushroom-column | 3 sliders (height/radius/sides) → `brep.curve.polygon` profile + `math.vector` axis → `brep.solid.extrude` → `output-preview` | Hexagonal column solid, default height 6 |
| 🍩 sphere-cut-with-torus | slider → `brep.prim3d.sphere` + `brep.prim3d.torus` → `brep.bool.cut` (preview=true) → `brep.measure.volume` → `output-preview` (scalar volume readout, default 37.73) | Sphere-minus-torus solid; the output-preview node shows a **number**, not geometry |
| 🐚 box-shell-preview | 2 sliders → `brep.prim3d.box` → `brep.solid.shell` (preview=true) | Hollow-shelled box |
| 📐 box-fillet-preview | 2 sliders → `brep.prim3d.box` → `brep.solid.fillet` (preview=true) → `output-preview` | Filleted box; only example with inference-law tests |
| 📦 rectangle-extrude-volume | 3 sliders → `brep.curve.rectangle` + `math.vector` → `brep.solid.extrude` (preview=true) → `brep.measure.volume` (not previewed) | Extruded rectangular volume |
| 🧲 sphere-box-fuse | 2 sliders → `brep.prim3d.sphere` + `brep.prim3d.box` → `brep.bool.fuse` (preview=true) → `output-preview` | Fused sphere+box solid |
| 🧹 face-sweep-extrude | 3 sliders → `brep.curve.rectangle` → `brep.surf.planarFaceWire` → `math.vector` → `brep.sweep.extrude` (preview=true, no separate output-preview node) | Swept face solid |
| 🪢 rectangle-wire-preview | 2 sliders → `brep.curve.rectangle` (preview=true) | Bare rectangle wire, no solid |

Tests: 7 of 8 examples have only `primary_asset_is_nonempty` (a `text.len() > 8` check on the raw DSL,
`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`); `📐️box-fillet-preview` additionally has
`inference_determinism_law`/`inference_default_law` (schema-`Inference` stability, not a render check). No
example test parses+evaluates+tessellates and asserts non-empty mesh output — see TL;DR #5.

A 9th example, `demo-session` (`✏️editor/📚️examples/🎬️demo-session/`), has an identical `ExampleSource`
shape but is excluded from `examples()` — see TL;DR #4.

## 6. Oracle

`🔮️oracle/🔣️.json` registers exactly one oracle, `procedural-3d-python-independent`
(`kind: "verified-native-second-implementation"`), NOT a third-party library: a hand-written Python
reimplementation of the document + all 14 mutations at `../../../../../🧪️tests/mutate-procedural-3d-1/🐍️component.py`,
replaying the 14 committed `(before, mutation, diff, outcome, after)` fixture quintets. Its own rationale
text states a real third-party survey was performed and declined (`ecosystemsSearched: ["python/pypi"]`,
reasoning: "no node-graph format models a graph whose layout is a SPARSE side table keyed by node id...").
The registration explicitly flags: (a) the Python probe "establishes that an independent implementation
of the specification computes the committed after-snapshots, not yet our codec against a second
producer" — it does not invoke this crate's actual Rust mutation code; (b) under "Protocol V2" this is
classified as not discharging the external-oracle requirement — "a qualifying third-party reference…is
still owed."

`🧫️fixtures` (73 files) holds the 14 committed mutation quintets under `🧬️mutations/<kind>/<scenario>/`
plus three architectural "phase8 law" fixtures (not runtime test data, compliance checklists for the
retained-command machinery): `👑️p8yz-b-owner-catalog-laws.json` (field-ownership map across
snapshot/mutation/nested-owner/auxiliary categories, names `delete-widget-position` as the
"generation3dOnlyMutation"), `🔬️p8yz-b-third-party-oracle-laws.json` (a small worked `move-widget`
example using `serde_json` as a test-only oracle for one feature), `🧷️p8yz-b-retained-mounted-laws.json`
(the bounded-job lifecycle/credit/publication contract this artifact must satisfy, discriminator `P3D3`).

## 7. IO — serializers/deserializers under `🚪️io`

9 formats each direction (`las`/`png`/`json`/`txt`/`stl`/`dwg`/`obj`/`gltf`/`ply`), all under
`🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/<format>/<version>/✳️any/🦀️.rs`:

- **`json`** — real both ways, via `semio_s_artifact_stdio_json::schema::snapshot::{write_json_pretty,
  parse_json_text, JsonSnapshot}` (`🚪️io/📤️export/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`,
  `🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`).
- **`txt`** — honest stub, `Err("txt export/import not yet implemented")` both ways, with a doc comment
  explaining the pre-migration code referenced nonexistent types
  (`🚪️io/📤️export/…/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`, `🚪️io/📥️import/…/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`).
- **`las`/`png`/`stl`/`dwg`/`obj`/`gltf`/`ply`** (7 formats) — non-functional placeholders, not stubs: see
  TL;DR #1. Export silently emits the artifact's native DSL text mislabeled as the target format; import
  silently discards the input bytes and returns a default empty document. No error is ever raised.

## 8. Gap list, ranked by user impact for "all windows work, all examples with 3D preview/hover"

1. **7 of 9 export/import formats (LAS/PNG/STL/DWG/OBJ/glTF/PLY) silently produce wrong-format or empty
   data with no error** — worse than the already-known txt stub because it looks like success
   (`🚪️io/{📤️export,📥️import}/…/{☁️las,📷️png,🔺️stl,🖊️dwg,🗿️obj,🧊️gltf,🧱️ply}/…/🦀️.rs`).
2. **No test proves any of the 8 examples actually renders 3D preview geometry** — only a DSL
   non-emptiness check exists; a broken op-chain would pass every test in this crate
   (`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`).
3. **Right-click context menu never shows transform/delete/remove items for a real selection** —
   hardcoded empty selection at `✏️editor/🦀️.rs:219` (the render pipeline itself is unaffected — hover,
   selection and the gumball ARE live there).
4. **`flow-eval-resolve`/`flow-tessellate-resolve`** are fully implemented but unreachable from any UI
   action (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:679,683`); `set-contributions`
   is an empty, never-compiled directory.
5. **`demo-session`** is a complete 9th example excluded from the manifest, so it cannot be selected from
   the UI (`✏️editor/📚️examples/🎬️demo-session/🦀️.rs` vs `✏️editor/🦀️.rs:1378-1387`).
6. txt import/export remain explicit `Err` stubs (already known, unchanged).
7. `preview_selection_json` still hardcodes marquee method `"rectangle"` (already known, unchanged,
   cosmetic — no persistent "last marquee method" outside a live gesture, same as puzzle3d).

Everything else this pass checked — all 27 editor actions' dispatch wiring, the flow/preview/generations/
form/generate-preview windows, all 3 panels (including a genuinely live selection-driven inspector, unlike
puzzle3d's), the viewer, the artifact/config/diff/snapshot schema, and the oracle/fixture registration —
is real and reachable from the UI.
