# 🗺️ Verfolgen (`s.gis.gismap@1/*#editor`) — selection/hover round trip + real example catalogue

Date: 2026-09-16. Crate `semio-s-artifact-gis-gismap` (artifact `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap`),
host `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx`.

## 1. Findings — the audit's claim (a) was half stale, half real

`📓️app-verfolgen.md` §5 reports `TiledMapHost` dispatching the deleted `setFeatureSelection`/`setHover`
verbs at `:903-910,956`. **That half is already fixed and committed** (commit `b2064cc237`, 2026-09-13):

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx:1074`
  already dispatched `interactionSelect` (with a `clearSelection` miss arm), and `:1120`
  already dispatched `interactionHover` on the `"pointer"` channel.
- The guest side already declares the domain: `.interaction(InteractionDefinition { id: "features", … })`
  at `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1127`
  (granularities `layer`/`feature`, `HierarchyProvider::Flat`, modes Multiple/Single, methods
  Pick/Rectangle/Lasso), so the framework auto-injects
  `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/… — the app never declares them.

**The real, still-open half was the return path.** `World3dHost`'s contract is two-way:

| lane | World3d (cad) | gis map (before) |
|---|---|---|
| dispatch args | `world3dSelectionActionArgs` / `world3dHoverActionArgs` — `🌐️World3dHost/🟦️.tsx:5232` / `:5142` | inline object literals in `TiledMapHost` |
| domain id on the scene | `scene.domain_id = Some(CAD_INTERACTION_DOMAIN)` — `📐️cad/…/✏️editor/🎭️modes/✏️edit/🦀️.rs:281`; host reads `scene?.domainId` at `🌐️World3dHost/🟦️.tsx:5294` | `TiledMapScene` has no `domainId`; the host hardcodes `"features"` |
| **guest → scene read-back** | `render_with_request_context` → `CadInteractionSnapshot::from_interaction` → `world_selection_json(…)` → `World3dScene.selection_json`; host parses it at `🌐️World3dHost/🟦️.tsx:5430` | **missing** |

`map::render` built the scene with `TiledMapScene::base(…)` only, leaving `selection_json`/`hover_json`
at their `"{}"`/`"null"` defaults, with an explicit comment saying `ArtifactEditor::render` carries no
`InteractionView`. `Gis2dPlayApp` never implemented `render_with_request_context`, so the interaction
store was written by every pick and read by nobody. `TiledMapHost` feeds exactly those two fields into
the wasm session (`syncMapInteraction` → `session.syncInteraction`, `🧭️TiledMapHost/🟦️.tsx:212`) and into
the hover popup (`parseMapHoveredFeature(scene.hoverJson)`), so **the highlight and the popup could never
appear**, even though the dispatch was correct.

Audit claim (b) was accurate: `setActiveExample` branched only on empty-vs-non-empty, and the manifest
staged a hand-written `"reuse-map"` id that matched nothing in `📚️examples` (the declared facet is
`🎬️demo`, id `"demo"`, `📚️examples/🎬️demo/🦀️.rs:5`).

**Peer ticket** `.../🎆️26/🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/` was read first (`📓️status.md`,
`📓️verification.md`). Its work is envelope/store-ladder + build-gate, disjoint from this. Note that
the peer's "Xcode license blocks `cargo test`" blocker is **not** reproducible with the
`DEVELOPER_DIR=/Library/Developer/CommandLineTools` prefix — every gate below ran natively.

## 2. Concurrent peer on the same files (important)

A peer (not visible in `ListAgents`) was editing
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs` **live during this session**, implementing the
same `Gis2dInteractionSnapshot` idea. Edits were kept minimal and re-read before each write; the two
strands were merged rather than duplicated:

- Peer-owned: `Gis2dInteractionSnapshot` itself (`:55`), its `feature_selection_json` (`:83`) /
  `feature_hover_json` (`:90`) / `selected_layer`, the `GIS2D_*` granularity consts, and the inspection
  panel's new 4-arg `render`.
- This session: `Gis2dPlayApp::render_body` (`:758`), `render_with_request_context` (`:1056`), the map
  window's scene wiring, the example catalogue, and all the tests below.

## 3. Edits (file:line)

### Guest — selection/hover round trip

1. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:40,43`
   — added `GIS2D_INTERACTION_DOMAIN` / `GIS2D_FEATURE_GRANULARITY` consts and replaced the `"features"`/
   `"feature"` string literals in `select_feature_action_args` (`:723-726`) with them.
2. Same file `:757-770` — new `impl Gis2dPlayApp { fn render_body(…, interaction: &Gis2dInteractionSnapshot) }`,
   the one render implementation both trait entry points share (mirrors `CadPlayApp::render_body`).
3. Same file `:1049-1068` — `ArtifactEditor::render` now delegates to `render_body` with an empty
   snapshot; **new** `render_with_request_context` resolves the live domain via
   `Gis2dInteractionSnapshot::from_interaction(interaction)` and threads it in. This is the link that
   was missing.
4. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs:85-99`
   — `render` takes the interaction snapshot and sets
   `scene.selection_json = interaction.feature_selection_json(document)` and
   `scene.hover_json = interaction.feature_hover_json(document)`; the stale "no `InteractionView` here"
   comment is replaced by the round-trip contract. Import updated at `:3`.

### Host — `TiledMapHost` mirrors the World3dHost arg-builder contract

5. `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx:61-68`
   — `MAP_INTERACTION_DOMAIN` / `MAP_FEATURE_GRANULARITY` / `MAP_HOVER_CHANNEL` consts, documented as the
   wire twins of the guest consts.
6. Same file `:187-201` — exported `mapFeatureSelectionActionArgs(ids, merge, method)` and
   `mapFeatureHoverActionArgs(id)`, the tiled-map twins of `world3dSelectionActionArgs` /
   `world3dHoverActionArgs` (same de-duplication, same `targets` JSON shape, same empty-targets clear).
7. Same file `:1074` and `:1120` — the click/marquee and hover dispatches now call those builders
   instead of inline literals, so the wire shape is testable in isolation. Behaviour is unchanged for
   the already-correct verbs; the `clearSelection` miss arm is untouched.

### Example catalogue (audit gap (b))

8. `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🦀️.rs:10-32`
   — new `🔖️Catalogue` region: `example_catalogue() -> Vec<ExampleSource>` (reads the subset's own
   `📚️examples/🎬️demo` facet), `DEFAULT_EXAMPLE_ID`, `example_arg_options()` (catalogue → palette select
   options), and `example_document(id)` which resolves an id by parsing the facet's own DSL through
   `ArtifactDsl::parse_dsl` + `gis_map_snapshot_with_derived_children`. An empty id is the catalogue's
   "none" arm; **an unknown id now faults** instead of silently loading the bundled map.
9. Same file `:54` — `set_active_example::handle` is now `let next = example_document(&payload.example_id)?;`
   (was `if payload.example_id.is_empty() { default } else { default_document() }`). The diff-into-
   operations tail is unchanged, so undo semantics are identical.
10. `✏️editor/🦀️.rs:1185-1187` — the manifest's `setActiveExample` arg options are projected from
    `example::example_arg_options()` with `default_value(&example::DEFAULT_EXAMPLE_ID)`; the hand-written
    `"reuse-map"` option is gone. The stale SDK-GAP comment at `:1201-1204` was corrected.
11. `✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/⚙️config/🧪️tests/🔬️window-ownership/🦀️.rs:197` and
    `✏️editor/🧪️tests/🔬️unit/🦀️.rs:327` — the two existing `"reuse-map"` call sites now use
    `example::DEFAULT_EXAMPLE_ID`. (Without this the window-ownership law failed; see §5.)

### Tests

12. `✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs:1-56` — helper `interact(app, action, args)`
    that dispatches a framework-injected interaction verb with the exact `TiledMapHost` payload and
    settles its reserved job, plus two laws:
    - `an_interaction_select_marks_the_feature_selected_in_the_rendered_scene` — **the selection law
      the task asked for**: fresh scene is `{"positions":[],"routes":[]}`; after `interactionSelect` on
      a fixture position id the very next render carries `{"positions":["<id>"],"routes":[]}`.
    - `an_interaction_hover_reaches_the_scene_as_the_popup_record` — `interactionHover` on `"pointer"`
      reaches `scene.hover_json` as `{kind:"route", id:…}`.
13. `✏️editor/🎮️commands/🎨️example/🧪️tests/🔬️unit/🦀️.rs:47-73` —
    `the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest` (catalogue == declared facets;
    `""` → empty; `"demo"` → the bundled Liège reuse map with real routes; `"reuse-map"` → `Err`) and
    `the_manifest_stages_exactly_the_catalogue_ids` (every staged `exampleId` option is a catalogue id
    and resolves). The existing round-trip test now uses `DEFAULT_EXAMPLE_ID`.
14. `🧰️framework/…/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts:100-129` — new `feature interaction dispatch args`
    describe block: exact `interactionSelect` arg shape, id de-duplication + marquee method passthrough,
    `interactionHover` pointer-channel shape with the empty-targets clear, and `resolveMapInteractionSync`
    mapping the guest's published selection back onto the wasm session's granularity.

## 4. Commands run

All cargo commands prefixed `DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings
CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4` (the last per the coordinator's load-shedding note).
Logs under `🗑️generated/`.

| command | outcome |
|---|---|
| `cargo check -p semio-s-artifact-gis-gismap --features component-app-assembly --tests` | pass (`gis-check-native-2.txt`; first run caught one E0716 in a new test, fixed) |
| `cargo check --target wasm32-wasip2 -p semio-s-artifact-gis-gismap --features component-app-assembly -p semio-s-plugin-gis` | pass |
| `cargo test -p … --lib -- <7 targeted names>` | 7 run, 6 pass → hover assert was key-order-sensitive → fixed → 2/2 pass (`gis-test-hover.txt`) |
| `cargo test -p … --lib -- --skip live_envelope` | **244 passed, 0 failed, 2 filtered** (`gis-test-lib-skip-envelope-final.txt`) |
| `bun ./📜️script.ts test ../../../../🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts` (react renderer pkg, `SEMIO_TEST_LEVEL=long`) | **21/21 pass** (17 pre-existing + 4 new) (`tiledmaphost-ts-test.txt`) |
| `bunx tsc --noEmit -p …/⚛️react/📦️packages/🟦️typescript/tsconfig.json` | 846 pre-existing errors repo-wide, **zero mentioning `TiledMapHost`** |

### Before / after counts

- Peer baseline (`📓️verification.md`): gismap lib **239 / 240**, the one failure being
  `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`.
- After: **246 tests** (4 added here, 2 added by the concurrent peer). Excluding the two
  `live_envelope` gates: **244 pass / 0 fail**.

## 5. Two collateral failures, both fixed here

1. `gis_map_window_ownership_runtime_isolates_renders_and_reopens_two_map_windows` regressed on the
   catalogue change (it dispatched the now-rejected `"reuse-map"`); fixed by edit 11.
2. `editor::gis2d::panels::inspection::tests::the_inspector_detail_section_follows_the_features_selection`
   — a **peer-authored test added mid-session** — failed with
   `Error("BuiltChildren requires retained page transport")`: it `serde_json::to_string`d a `BuiltNode`
   directly. The panel's file had been quiet for 30+ min, so it was fixed here rather than left red:
   `✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:22-27` now routes the tree through
   `built_to_component_tree` + `artifact_app_laws::project_and_retire_fixture_tree`, the same route
   `unit_tests::context::render` takes. The panel itself is untouched.

## 6. What remains

1. **Browser check on :6029 (coordinator).** Click a marker / route in "verfolgen" → it should paint
   selected; hover → popup. The guest law is proven headlessly, but the wasm `syncInteraction` paint
   and the popup DOM have no headless coverage. Console signal to watch: an `interactionSelect:
   unknown merge` or a `clearSelection` fault would mean the domain declaration and the wire disagree.
2. **Generated plugin manifests still carry `reuse-map`.** `✏️s/🔌️plugins/🌍️gis/🔣️.json:916,943`,
   `✏️s/🔌️plugins/🎪️demonstrator/🔣️.json:51366,51393` and their `🧑‍💻dev/🔌️plugin-modules/` + `dist/dev/`
   copies are build products; they were deliberately **not** hand-edited. They will pick up `demo` on
   the next plugin-manifest regeneration. Until then the demonstrator's staged default arg is an id the
   guest now rejects — **regenerate the gis + demonstrator plugin manifests before the browser pass**,
   or drive `setActiveExample` with `exampleId: "demo"` explicitly.
3. **`TiledMapScene` has no `domain_id`/`domain_granularity_id`.** `World3dHost` reads the domain off
   the scene (`scene?.domainId`); `TiledMapHost` still hardcodes `"features"`. Fine while gis is the
   only tiled-map app, but a second one would need the scene-carried domain to match the World3d
   contract exactly. Not done here — it touches the shared scene schema.
4. **`ArtifactEditor::context_menu` still gets no `InteractionView`** (`✏️editor/🦀️.rs:751-756`), so
   `clearSelection` in the map right-click menu is always disabled. Pre-existing SDK gap, unchanged.
5. **`setActiveExample` does not prune a stale selection.** The `"features"` domain is
   `HierarchyProvider::Flat`, so `validate_state` does not auto-prune; a selection surviving a document
   swap now renders as `{"positions":[],"routes":[]}` (ids are filtered by document membership), which
   is a visible improvement over the old behaviour but still leaves the store holding dead ids.
6. **`live_envelope` gates were skipped**, not fixed — they are the GIS-2D-END-TO-END-BUILD peer's open
   gates. `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` was still
   running after 10+ minutes in a background run started here (left running, not killed).
