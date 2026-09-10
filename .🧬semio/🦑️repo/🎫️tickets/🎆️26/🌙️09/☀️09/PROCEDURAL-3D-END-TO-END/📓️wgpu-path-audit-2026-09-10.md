# Wgpu path audit — Procedural 3D end-to-end (2026-09-10)

Read-only, no file outside this ticket touched, no `git` mutating command run, no cargo build/dev
server/browser started (per this audit's own constraints). All file:line evidence read against the
working tree at audit time (**2026-09-10 02:55 CEST**, `date` run against the live tree). Template:
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-wgpu-coverage-audit.md`.
Boot-path baseline: this ticket's `📓️boot-path-audit-2026-09-09.md` §2 (`renderer=wgpu` path,
port 6118) and §6 (recipe).

Scope: `s.procedural.generation3d@1` (variant `generation3d`, aliases `["procedural 3d"]`), the
**wgpu** renderer target only, in addition to the React target this ticket already confirmed booting.

## Verdict (lead)

**The wgpu path is not usable for generation3d today, and not because of a missing tool — because
its two `World3d` windows (`procedural-preview`, `generation3d-generate-preview`) publish geometry in
a shape the wgpu native scene layer has no code path to consume.** Every other surface generation3d
uses (`NodeGraph` flow window, the two `Canvas2d`-declared windows which actually carry plain
`UiNode` trees, panels, window measures, context menu) rides on framework-generic machinery already
proven native-wgpu by the puzzle3d audit, and should work once the plugin closure is rebuilt (§4 — it
is currently 1.5–23 days stale relative to source, predating both of today's `generation3d` fixes).

The headline, previously-unconfirmed finding (§1.1): `World3dScene::base()` — which every generation3d
preview window uses via the shared `world3d_scene()` helper — never sets `World3dScene.snapshot`, only
the plain-JSON `meshes_json`/`instances_json`/`selection_json` text fields. The wgpu-native
`sync_world3d_state` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9056`) requires
`scene.world_3d.snapshot` to already be `Some(lease)` — if it is `None` it sets
`World3dSnapshotFault::Unavailable` and returns immediately, **before touching `meshes_json` at all**.
No file anywhere in the repo (renderer, `ui_wgpu`, `infinite/world`, or their TypeScript twins) was
found constructing a `World3dSnapshotLease` from `meshes_json`/`instances_json` — `world3d_snapshot_begin`
and its sibling snapshot-builder functions are defined and re-exported but have **zero call sites**
repo-wide outside their own definition/tests. Puzzle3d's wgpu audit didn't hit this because puzzle3d's
3D content is GLB assets streamed by URL (`references_json` → `publish_world3d_asset_mesh_lease`), a
third, separate delivery path that never touches `meshes_json` or the snapshot gate. Generation3d has
no such asset — it is inherently procedural, tessellated live — so it has no way around this gate.
This is a static-read finding with strong, multi-file evidence (§1.1); it was **not run**, per this
audit's no-build/no-server constraint — flag for a runtime check before treating it as certain.

## 1. Surface-kind coverage for generation3d's five windows

Windows (`create_generation3d_app`, `✏️editor/🦀️.rs:241-291`, per this ticket's
`📓️feature-inventory-2026-09-09.md` §1) and the `SurfaceKind` each actually declares, grepped fresh
this pass (`grep -rn "SurfaceKind::" ✏️s/🔌️plugins/🌀️procedural`, excluding tests):

| window | file:line | `SurfaceKind` | wgpu-native path |
|---|---|---|---|
| `procedural-main` (Flow) | `🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:23` | `NodeGraph` | bespoke — real (§1.2) |
| `procedural-preview` (edit-mode 3D) | `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:25` | `World3d` | bespoke, but blocked (§1.1) |
| `generation3d-generations` (tree) | `🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:18` | `Canvas2d` | **not actually canvas draw records** — real UiNode tree (§1.3) |
| `generation3d-generate-form` (form) | `🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:21` | `Canvas2d` | same — real UiNode tree (§1.3) |
| `generation3d-generate-preview` | `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:26,67,74` | `World3d` (or `TextEditor` fallback before a generation is evaluated) | same blocker as above when `World3d` |

### 1.1 `World3d` — the snapshot-lease gate (headline finding)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:88-97` — builds the scene as
  `semio_framework_ui::wgpu::World3dScene { status_json, domain_id, domain_granularity_id, ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun) }`.
  `generation3d-generate-preview` (`🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`) follows the same shape.
- `world3d_scene()` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33594-33598`) is a thin
  wrapper over `World3dScene::base(camera_json, meshes_json, instances_json, selection_json)`
  (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs:371-380`) plus
  `environment_json` — it never touches `snapshot`, which stays at its struct default (`None`,
  `#[serde(skip_serializing_if = "Option::is_none")]`, `scenes.rs:209-210`).
- `sync_world3d_state` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9056-9074`):
  ```rust
  let Some(world) = scene.world_3d.as_ref() else { state.snapshot_fault = Some(World3dSnapshotFault::Unavailable); return; };
  let Some(lease) = world.snapshot else { state.snapshot_fault = Some(World3dSnapshotFault::Unavailable); return; };
  ```
  — reads `world.snapshot` only; `meshes_json`/`instances_json` are never dereferenced by this
  function. `render_world_3d` (`…♾️infinite/🌍️world/🦀️.rs:9106` onward) calls `sync_world3d_state`
  then continues drawing (clear color, camera, sun) regardless of the fault, so the window paints but
  with an empty `state.draws`/`meshes` map — no mesh geometry.
- No producer of `World3dSnapshotLease` from JSON was found: `world3d_snapshot_begin`
  (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌍️world3d_snapshot.rs:289`) and its siblings
  (`world3d_snapshot_admit_page`, `_close_step`, `_seal`, …) are imported/re-exported in
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:2870-2874` but have
  **no call site anywhere in the repo** outside their own definitions and unit tests (`grep -rln`
  across the whole tree). Neither React nor wgpu materializes a snapshot from `meshes_json` today.
- Puzzle3d's wgpu audit (template, §1) found real GLB-asset instancing on wgpu — that path is
  `references_json` (a URL) → `publish_world3d_asset_mesh_lease`
  (`…♾️infinite/🌍️world/🦀️.rs:11136`), a third mesh-delivery route that never touches `meshes_json` or
  the snapshot gate. Generation3d has no GLB asset — it renders live-tessellated procedural geometry —
  so it has no way around this gate today.
- **Not run**: this is a static-read conclusion. A runtime check (open `?plugin=generation3d` on
  `:6118` after a rebuild, watch for `World3dSnapshotFault::Unavailable`/an empty 3D viewport) would
  confirm or falsify it in minutes; recommended as the first verification step (§6).

### 1.2 `NodeGraph` (Flow window) — real, bespoke, generic

`node_graph_states: AdmittedSurfaceMap<…>` on `ShellState`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)
gets real wheel/pointer dispatch, distinct from the generic `handle_scene_*` handlers
(`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1262-1264`, the same bespoke-dispatch exclusion list
puzzle3d's audit cites for `World3d`/`TiledMap`/`Board2d`) —
`…🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11552-11558` (`node_graph_states.id_at`/`.get`) and three more
draw-time iterations at `:13491,13516,13587`. This is framework-generic (same code puzzle2d/procedural2d
exercise), not generation3d-specific, so it should carry over once the crate is rebuilt (§4).

### 1.3 `Canvas2d` windows — actually plain `UiNode` trees, already proven native

Despite declaring `SurfaceKind::Canvas2d`, both `generate-form` and `generations` windows return a
`BuiltNode` (a `UiNode` tree), not a `Canvas2dScene` draw-record payload:
- `🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:35-38` — `render()` returns
  `crate::generation_form(&spec, &current.values, …)`, or `built_text_node(labels.generate_hint)`
  when nothing is selected.
- `🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:32-34` — `render()` returns
  `crate::generation_tree(GENERATION_3D_PLAY_APP_ID, …)`.

Both resolve to ordinary `UiNode` variants (`Stack`/`Text`/`Tree`/`Field`/etc.), which the puzzle3d
audit already confirmed are rendered by a **pure, exhaustive native wgpu retained-widget system**
(`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1587-1607`, `ui_node_kind_tag`) — not the
`Canvas2dScene` fill/stroke/blend-mode pixel-drawing pipeline
(`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1599-2035`, the `Canvas2dPacketItem`/blend-mode machinery
that IS wgpu-tested at `…/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs` but is irrelevant here since it's never hit).
These two windows are therefore **low-risk** on wgpu — the same native path already proven for
puzzle3d's sliders/steppers/selects/trees carries the Form and Generations windows too.

### 1.4 Panels, window measures, utilities, context menu

- **Panels** (Document/Catalogue/Inspection) are `UiNode` trees the same way — same native path as
  §1.3. The Catalogue *panel* itself is fine (it was rebuilt as a bounded page,
  `📓️catalogue-surface-2026-09-09.md` §1.4); the app-static catalogue *section* used by the canvas
  spotlight is a separate, renderer-fetched concern — see §2.2, a real wgpu gap.
- **Window measures** — show-mode select (Shaded/Shaded+edges/Wireframe/Points), sun sliders
  (azimuth/elevation/intensity toggle), LOD select on the Flow window — are `WindowMeasure` values
  built host-side (`👁️preview/🦀️.rs:36-51`, `🕸️flow/🦀️.rs:38-51`) and rendered through
  `program.window_measures(…)` in `refresh_ui`
  (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3196`), which is renderer-agnostic host-side data — the
  same generic chrome path puzzle3d's fill slider used successfully. Low risk.
- **Utilities (gumball move/rotate/scale)**: generation3d declares **none**. `procedural-preview`'s
  `WindowKindDefinition.utilities: Vec::new()`
  (`📓️feature-inventory-2026-09-09.md` §1: *"no gumball/brush/fill tool row exists for this app at
  all; the only interaction surface is the node graph"*). Puzzle3d's wgpu gap #1 (missing gumball) is
  therefore **not applicable** to generation3d — there is no gumball utility to be missing.
- **Context menu**: generic wgpu support is real
  (`shell_context_menu_item_from_spec`/`ContextMenuOrganizer`,
  `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:284-311`). The plugin-side blind-selection bug this
  ticket already fixed (`📓️editor-gaps-2026-09-09.md` §1(a) — `context_menu` hardcoded an empty
  selection; now `context_menu_with_request_context` reads the real `graph` selection) is
  renderer-agnostic host logic, so wgpu inherits the fix once rebuilt (§4).

## 2. New wire formats: does wgpu consume the pack mesh wire and the app-static catalogue?

### 2.1 Mesh transfer — `pack::encode_record_body` never reaches either renderer (non-issue for wgpu specifically)

`📓️tessellation-jobs-2026-09-09.md` §2.1 describes `MeshData` now crossing the **host↔wasm-extension**
boundary as a `pack` record body (chunked base64) instead of a JSON string. Tracing where that decoded
data goes next (`✏️editor/🦀️.rs`, generation3d's own editor crate):
- `mesh_data_for_preview_handle` (`✏️editor/🦀️.rs:2141-2159`) calls `decode_preview_mesh_pack`
  (`:2160`) to turn the pack body back into a `semio_framework_plugin::MeshData` struct.
- `preview_payload` (`:2263-2340`) then re-serializes that struct as plain JSON:
  `PreviewPayload { meshes_json: dsl::json::to_string(&dsl::json::Value::Array(meshes)), … }`.
- That `meshes_json` **plain JSON string** is exactly what `world3d_scene()`/`World3dScene::base()`
  carry (§1.1) — the same field shape as before the tessellation-jobs change.

**Conclusion: the new pack wire is purely an internal optimization between the flow extension and the
plugin host (shrinks a ~JSON-array-of-arrays transfer across the wasm-extension boundary); it never
crosses to either renderer.** Neither React nor wgpu needs to change to consume it — both still receive
`meshes_json` as JSON text, unchanged. This resolves the task's item (2) mesh-wire half in wgpu's favor:
there is no wgpu-side gap here. (`decodeMeshPackBody`/`decodeMeshPackChunks`,
`🧰️framework/🛍️products/💻️os/🟦️.ts:2336,2385`, are also not called from `World3dHost/🟦️.tsx` at all —
defined and cross-language-tested, `🧪️tests/🧊️mesh-pack-decode/🟦️.ts`, but currently dead code on
*both* renderers, for the same reason: the render-facing wire was never migrated off `meshes_json`.)

The real wgpu gap for `World3d` content is §1.1's snapshot-lease gate — unrelated to this pack wire.

### 2.2 App-static catalogue — wgpu has no fetch path for it (real gap)

`📓️catalogue-surface-2026-09-09.md` moved the operator catalogue out of the per-surface `NodeGraphScene`
payload into a 4th `UiRefreshSection::Catalogue`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4432-4445`), fetched via `ArtifactApp::app_catalogue_json()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11151-11160`) and cached client-side by
`uiRefreshWantsCatalogue`/`buildUiRefreshRequest`
(`🧱️elements/🛠️ShellHelpers/🟦️.tsx:3995-4001`) — **a React-only TSX file, no wgpu-target sibling** —
then threaded to scene hosts via `AppCatalogueContext`/`useAppCatalogue`
(`🧱️elements/🗣️Interpreter/🟦️.tsx:565-579`) fed from `Shell.tsx`'s `appCatalogue`
(`🧱️elements/🐚️Shell/🟦️.tsx:437-441`).

Grepped the wgpu-target Rust for any of `app_catalogue`/`AppCatalogue`/`UiRefreshSection`/`Catalogue`
in `Shell/🎯️targets/🧊️wgpu/🦀️.rs` and `Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`: **zero hits**. The wgpu
`refresh_ui` (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3159-3230`) calls
`program.render_with_document(…)` per window/panel body directly — it never calls
`app_catalogue_json()` and has no `UiDirtyScope`/refresh-request concept at all (grepped for
`UiDirtyScope`/`buildUiRefreshRequest`/`RefreshRequest`/`DirtyScope` in the wgpu renderer target and
`ui_wgpu`: zero hits). **The wgpu Flow window's node-graph palette/catalogue and the canvas spotlight
(the unbounded browse surface explicitly said to read the app-static catalogue,
`📓️catalogue-surface-2026-09-09.md` §1.4) will be empty on wgpu** — not because of the crate
staleness in §4, but because the wgpu host has no code path to ask for or receive
`UiRefreshSection::Catalogue` at all. This is a genuine, still-open gap for the Flow window's palette
UX on wgpu (the small, bounded Catalogue **panel** in §1.4 still works — it doesn't need the app-static
section, only the node-graph canvas spotlight/full palette does).

## 3. Hover / selection / picking on wgpu

Carried forward from this ticket's `📓️kernel-and-preview-audit-2026-09-09.md` §4 (predates this pass,
re-verified as still current — no picking code was touched by anything else read this session):

- **React picking is client-side, not host-Rust, not a wgpu compute pass.** `World3dHost/🟦️.tsx`
  (react-three-fiber) reads three.js's own raycaster (`event.faceIndex`/`event.face`/`event.point`
  from R3F `onClick`/`onPointerMove`) against `face_ids`/`edge_ids`/`vertex_ids` baked into `MeshData`
  at tessellation time, then dispatches domain-neutral `interactionHover`/`interactionSelect` —
  generation3d's own `world-pointer-down` command handler
  (`✏️editor/🎮️commands/🌍️world-pointer-down/🦀️.rs:14-16`) is a no-op stub (`Ok(Emit::default())`);
  the artifact itself never resolves a pick.
- **Open question, unresolved by a static read either pass**: `world3d_states: AdmittedSurfaceMap<World3dState>`
  in the wgpu render target is documented as "driven directly by the OS event loop," but no
  `Ray`/intersection routine was found in the searched wgpu-target files — whether this native path
  does its own CPU ray-triangle test or simply reuses React-resolved ids is **not settled by a static
  read** (needs a runtime check, not a guess).
- This question is now doubly moot for generation3d specifically until §1.1 is fixed: if the `World3d`
  window never gets past `World3dSnapshotFault::Unavailable`, there is no mesh geometry in `state.draws`
  for any native picking path to hit against in the first place. `NodeGraph`'s hover
  (`NodeGraphHover`, `marks.graph_selection_ids()`/`graph_highlight_ids()`) is a better near-term bet
  on wgpu — it rides the bespoke `node_graph_states` dispatch (§1.2), a 2D hit-test problem the same
  generic `hit_test_node`/`hit_test_subtree` machinery already handles for panel/chrome UI
  (`🎯️events.rs:114-246`, per the puzzle3d audit).

## 4. `?plugin=generation3d` boot on the wgpu wasm shell — staging tree and staleness right now

`resolvePlaygroundBoot(PLUGIN_CATALOG, import.meta.env.VITE_SEMIO_PLUGIN || …)`
(`…🧑‍💻dev/🟦️.ts:12-14`, per `📓️boot-path-audit-2026-09-09.md` §6) is renderer-agnostic; both targets
set `VITE_SEMIO_PLUGIN=generation3d`. The two targets read from **different, independently-staged
directories**:

| target | staging root | staged? | freshest mtime (this audit, now = 2026-09-10 02:55 CEST) |
|---|---|---|---|
| React (port 6018) | `…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/generation3d/` | yes | **2026-09-10 02:41:24** — 14 min old at audit time |
| wgpu (port 6118) | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/` (unprofiled tree, `buildPlugins("generation3d")`'s `pluginOutRoot`) | yes, but stale | see table below |

wgpu plugin-modules staleness, per-crate (`stat -f "%Sm"` on each crate's `.core.wasm`, the 11-crate
closure §2 of `📓️boot-path-audit-2026-09-09.md` identified: `procedural` + `forms` + the 9
`flow-extension-*` crates):

| crate | dir | `.core.wasm` mtime | age vs. now |
|---|---|---|---|
| `🌀️procedural` | `🔌️plugin-modules/🌀️procedural/` | 2026-09-07 08:45:43 | ~2.75 days |
| `📋️forms` | `🔌️plugin-modules/📋️forms/` | 2026-08-17 21:20:22 | **~23 days** |
| `🏘️flow-extension-bim` | `🔌️plugin-modules/🏘️flow-extension-bim/` | 2026-09-08 17:34:28 | ~1.4 days |
| `🧊️flow-extension-brep` | `🔌️plugin-modules/🧊️flow-extension-brep/` | 2026-09-06 09:15:05 | ~4.7 days |
| `📚️flow-extension-dictionary` | same pattern | 2026-09-06 09:15:05 | ~4.7 days |
| `📝️flow-extension-text` | same pattern | 2026-09-06 09:15:07 | ~4.7 days |
| `🔤️flow-extension-primitive` | same pattern | 2026-09-06 09:15:07 | ~4.7 days |
| `🎨️flow-extension-draw` | same pattern | 2026-09-06 09:15:05 | ~4.7 days |
| `🔀️flow-extension-logic` | same pattern | 2026-09-06 09:15:06 | ~4.7 days |
| `🧮️flow-extension-math` | same pattern | 2026-09-06 09:15:07 | ~4.7 days |
| `📃️flow-extension-list` | same pattern | 2026-09-06 09:15:05 | ~4.7 days |

All 11 crates are present (no missing-crate 404 risk per se), but **every one predates both of this
ticket's `generation3d`-relevant fixes** — `📓️tessellation-jobs-2026-09-09.md`'s pack-wire/budget work
and `📓️catalogue-surface-2026-09-09.md`'s surface-bound fix (its own status line: done at commit
`6ad7b0e7bc`, **2026-09-10 01:31** — after every wasm above was built). Contrast with the react
staging tree's 14-minute freshness. **A `buildPlugins("generation3d")` rebuild (the wgpu dev path's own
step 2, `📓️boot-path-audit-2026-09-09.md` §2) is required before any wgpu boot reflects current
source** — otherwise a runtime check would be validating stale code, not the fixes this ticket landed.
`forms` at 23 days stale is the single oldest crate in the closure.

## 5. Gap list, ranked by user impact

1. **Highest — `World3d` preview windows render no geometry on wgpu (§1.1).** Both `procedural-preview`
   and `generation3d-generate-preview` publish `meshes_json`/`instances_json` only; `sync_world3d_state`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9056-9074`) requires
   `World3dScene.snapshot` and faults `Unavailable` otherwise, with no JSON→snapshot bridge found
   anywhere. **Fix lane files**: either (a) add a bridge that decodes `meshes_json`/`instances_json`
   into a `World3dSnapshotLease` before `render_world_3d` is called (new code, likely near
   `sync_world3d_state` in `…♾️infinite/🌍️world/🦀️.rs`, or in
   `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `refresh_ui`/frame-transaction path before the scene
   reaches this function), or (b) have `sync_world3d_state` fall back to parsing `meshes_json` directly
   the way `World3dHost/🟦️.tsx` does when `snapshot` is absent. **Needs a runtime check first** — this
   audit is static-read only.
2. **High — the Flow window's app-static catalogue (canvas spotlight / unbounded palette) is empty on
   wgpu (§2.2).** No wgpu-target file requests or receives `UiRefreshSection::Catalogue`. **Fix lane
   files**: `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`refresh_ui`, needs an `app_catalogue_json()`
   fetch + cache mirroring `Shell/🟦️.tsx:437-441`'s `appCatalogue`), and
   `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (needs the consuming side, mirroring
   `Interpreter/🟦️.tsx:565-579`'s `AppCatalogueContext`). The small, bounded Catalogue *panel* (§1.4)
   is unaffected — only the canvas spotlight/full palette needs this section.
3. **Medium, blocking any verification — the wgpu plugin closure is 1.4–23 days stale (§4)**, predating
   both of this ticket's `generation3d` fixes. Not a code gap, but gap #1/#2 (and everything else)
   cannot be runtime-verified until `buildPlugins("generation3d")` is re-run. **Action**: run the
   recipe in §6 before any other wgpu work on this app.
4. **Low, unconfirmed — 3D picking/hover on `World3d`, once §1 is fixed (§3).** Whether the native
   `world3d_states` path does its own ray-cast or needs one added is unresolved by static read; likely
   moot in practice until gap #1 puts real geometry into `state.draws` to pick against.
5. **Not a gap — no gumball utility exists for this app (§1.4).** Unlike puzzle3d, generation3d
   declares zero utilities on its `World3d` windows, so puzzle3d's "missing gumball" finding does not
   carry over; nothing to fix here.
6. **Not a gap — Canvas2d-declared Form/Generations windows, panels, window measures, context menu,
   NodeGraph dispatch, and the mesh pack wire (§1.2, §1.3, §1.4, §2.1) are all either already
   framework-generic and wgpu-native, or (mesh pack wire) never reach the renderer at all.** These
   should work once §3's staleness is cleared, pending a runtime check.

## 6. Verification recipe for a later pass

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=<private-scratchpad-target> \
RUSTC_WRAPPER="" \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6118 \
bun ./📜️script.ts dev procedural 3d
```
(or the `.vscode/launch.json` row `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm`, `.vscode/launch.json:2077-2094`
— `command: "bun nx run workspace:dev -- procedural 3d"`, env `S_OS_PORT=6118`/`SEMIO_RENDERER=wgpu`,
`serverReadyAction` pattern `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):6118)`). Leave
`SEMIO_PLUGIN_ONLY` unset (drops 10 of 11 needed crates, per `📓️boot-path-audit-2026-09-09.md` §2).

**Readiness markers** (from `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1464-1498`):
- `[dev] Port 6118 already serving wgpu trunk at <url>` — already up, reused.
- `[dev] wgpu trunk serving at <url>` — fresh boot succeeded.
- Then open `http://127.0.0.1:6118/?plugin=generation3d`.

**What to check, in order, once open**:
1. Does the `procedural-preview` (edit mode) or `generation3d-generate-preview` (generate mode, after
   evaluating a generation) window show geometry, or an empty/faulted viewport? This directly tests
   gap #1 (§1.1) — the single highest-value confirmation this ticket needs next.
2. Does the Flow window's canvas spotlight / full operator palette populate, or stay empty? Tests
   gap #2 (§2.2).
3. `strings <staged .core.wasm> | grep <a string unique to today's source>` before trusting any of the
   above — port-listening is not proof of a rebuild (`📓️boot-path-audit-2026-09-09.md` §6, "Known
   pitfalls," carried over verbatim from the puzzle3d docs).
