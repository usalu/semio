# W2f — CAD spatial editor: engine port + wgpu parity

Packet **W2f (XL)** of `26/09/17/WGPU-RENDERER-REACT-PARITY`. Reads:
`📓️audit-plugin-react-only-surfaces.md` §2 row 1 / §7 `P-cad`, `📓️audit-interpreter-elements.md` P0 #4.
Logs under `🗑️generated/w2f-*.txt`.

---

## 1. Census correction — what the CAD "React-only surface" actually is

Both audits state the CAD spatial-tree editor "has no wgpu implementation of any kind" because
`⚙️engine/` carries no `🎯️targets/🧊️wgpu` sibling. That premise is right and the conclusion is wrong,
and the difference decides the whole packet. Verified against disk:

| Claim | Verdict | Evidence |
|---|---|---|
| `📺️renderer/🟦️.tsx` (6 766 lines) is the CAD editor screen the OS shell renders | **No** | it is `@semio-tech/cad-js`'s R3F editor. Repo-wide, `InteractionRepl`/`InteractionSpatialView`/`InteractionCanvas` have **zero** consumers outside that file, its own vitest suite, and `📖️stories/🎭️renderer/🧪️.story.tsx`. `@semio-tech/cad-js` is imported only by the cad plugin's own TS, the spatial-kernel tests, three plugins' `package.json`, and the repo library taxonomy — never by `💻️os`'s renderer. |
| The CAD editor screen has no wgpu path | **No** | the shell-facing CAD editor is the Rust **play app** `crate::editor::cad` (`✏️editor/🦀️.rs`, 2 565 lines + `🎮️commands/*` 11 files + `📌️panels/*` 3 + `🎭️modes/✏️edit/**` + `👥️presence` + `🎚️config`). Each of its four panes is a `WindowKindDefinition` with `surface_kind: SurfaceKind::World3d` (`🎭️modes/✏️edit/🪟️windows/{📐️shape,🏢️building,🔥️energy,🏛️structure-classic}/🦀️.rs`) rendered by `edit::build_world_scene_for_pane` into a `World3dScene` wire. React paints that wire with `🌐️World3dHost`; wgpu paints the *same* wire with `render_world3d_surface_step` (Tier 1 of `render_component_scene_step`, closed by W1f). Panels/tree/measures go through `UiNode`, painted by both Interpreter targets. |
| Only 2 of 9 `⚙️engine/` submodules are Rust | **Yes**, and it understates the Rust that exists elsewhere | `⚙️engine/` holds one Rust file (`🕹️interaction/🦀️.rs`, 1 050 + 429 test lines) against seven TS ones. But `🎰️stately`+`🎬️actions`+`🗿️artifact`'s *statechart* duties are already re-implemented in Rust — as a generic interpreter over the same JSON assets — inside that one file (`start_session`/`apply_event`/`keyed_transitions`/`can_commit`/`parse_repl_line`/commit runner/`preview_display_items`), plus `🎬️interaction-spec/🦀️.rs` (the spec types) at artifact level. |

So W2f is **not** "build a wgpu paint layer for a React screen". It is:

> the React editor implements a large body of *spatial-editor engine* behaviour that the Rust play app
> never computes, and therefore never puts on the `World3d` wire — so **both** shell targets (React
> `World3dHost` and wgpu) paint object-level instances only: no face/edge/vertex/wire/shell/anchor pick
> targets, no typology colours, no primitive/typology/entity-flag visibility filtering, no hover-key
> aliasing, no modifier-mode multi-select merge. The parity gap is in the **plugin's engine and wire
> payload**, identical on both targets.

That reframing is what makes the packet tractable: one Rust engine port fixes React *and* wgpu at once,
which is exactly the multi-implementation rule (schema-first, one engine, N paint targets).

### 1.1 Residual true-wgpu gap

One thing the reframing does *not* cover: the `World3dScene` wire has no lane for sub-object pick
targets or per-typology styling today (`world_instances_json` hard-codes `#3b82f6`/`#64748b`;
`world_selection_json` carries `ids`/`hoveredId`/gumball/engagement only). Emitting new lanes needs the
framework `World3dScene` struct + both hosts to read them — a framework seam owned by other W2 packets.
This packet therefore lands the engine first (target-neutral, fully testable inside the plugin crate),
and reports the wire-lane extension as the hand-off. See §5.

---

## 2. Design — module map

React module → Rust destination. `⚙️engine/<name>/🦀️.rs` is the language twin beside `🟦️.ts`
(that is how `🕹️interaction` already sits); target-specific paint lives under
`🎯️targets/🧊️wgpu/🦀️.rs` per the repo taxonomy.

| React source (region / file) | Concern | Rust destination | Status |
|---|---|---|---|
| `⚙️engine/🧬️typology/🟦️.ts` (167) + kernel `📐️geometry/🟦️.ts` typology/style/model-definition catalogue | typology + model-definition registry, authored/auto styles, entity kinds, construct kits | `⚙️engine/🧬️typology/🦀️.rs` (new) | **landed** — §3 I1 |
| `📺️renderer` `🧲️GeometryTargets` (647-1380) + `🧲️GeometryInteraction` (2277-2888) pure logic | pick kinds/toggles, visibility+typology+primitive+entity-flag filters, pick-target keys, hover-key aliases, reveal index, selection merge/prune by modifier mode, marquee coverage, per-target style | `⚙️engine/🧲️picking/🦀️.rs` (new) | **landed** — §3 I2 |
| `📺️renderer` `🎨️SpatialSceneColors` (1944-2084) | scene palette (selected/hovered/vertex/edge/face/object) | `⚙️engine/🧲️picking/🦀️.rs` `SpatialScenePalette` | **landed** (token ids, not CSS reads) |
| `📺️renderer` `🪩️Repl` pure helpers (4047-4550) | REPL line/palette/suggestion ranking, escape action, selection-by-model/by-state books | already Rust: `🕹️interaction/🦀️.rs::parse_repl_line{,_for}`, `numeric_entry_event`, `state_prompt`, `keyed_transitions`; per-pane selection lives in the framework `"cad"` interaction domain | **pre-existing** |
| `📺️renderer` `🎬️WorkerClient` / `🧊️CommittedMesh` / `🖼️DisplayPrimitives` | tessellation, mesh transfer, display-item paint | already Rust: `edit::world_meshes_json`(+cache), `object_mesh_data`, `interaction::preview_display_items`, framework `World3d` mesh lane | **pre-existing** |
| `📺️renderer` `🪩️Canvas` camera/auto-fit | camera + auto-fit | already Rust: `🎮️commands/🎥️camera`, `world3d_fit_json`, `CAD_FIT_PADDING` | **pre-existing** |
| `⚙️engine/📔️registry/🟦️.ts`, `⚙️engine/🏃️runtime/🟦️.ts` | JS asset glob + module registration + kernel selection | **not portable**: the Rust side has no glob/registrar concept — assets are `include_str!`-embedded and extensions are real wasm components declaring `contributes = ["cad.computer"]` (`🧩️extensions/*`, `🎮️commands/🧩️contribution`). Equivalent already exists by construction. | **n/a by design** |
| `⚙️engine/🎰️stately/🟦️.ts` (xstate/`@semio-tech/machine` adapter) | statechart host | **not portable**: `🕹️interaction/🦀️.rs` is a direct interpreter, no external machine kernel | **n/a by design** |
| `📺️renderer` R3F components (`SpatialPickGeometryLayer`, `SpatialPickTargetNode`, gumball, `GroundPickPlane`, panes) | paint + DOM interaction | wgpu side is the framework's `World3d` renderer + `UiNode` Interpreter, fed by the wire | **framework-owned**, §5 |

Reused Rust engines (no duplication): `semio_s_artifact_stdio_semio::…::brep::schema::engine::{Brep, BrepKernel}` for geometry/tessellation, `semio_framework_plugin`'s `World3dScene`/`scene_surface`/`MeshData`, the framework `"cad"` interaction domain for selection/hover, `🌳️Tree`/`PanelTreeBuilder` for the artifact/inspection/catalogue panels.

---

## 3. Per-module status

### I1 — `⚙️engine/🧬️typology/🦀️.rs` (609 lines) + `🧪️tests/🔬️unit/🦀️.rs` (22 tests) — **landed, green**

Mounted at `crate::editor::cad::engine::typology` (`🗿️artifacts/📐️cad/🦀️.rs` engine block).
Embeds 9 `🔣️modelDefinition.json`, 41 `🗂️typologies/*/🔣️typology*.json`, 10
`🏷️attributeDefinitions/*.json` with `include_str!` and parses them through
`protocol::json::from_json_str` into `ToValue`/`FromValue` asset structs — the same mechanism
`🕹️interaction/🦀️.rs` already uses for the 60 interaction specs, so no new dependency and no glob.

| React/TS symbol | Rust twin |
|---|---|
| `ModelEntityKind`, `PRIMITIVE_MODEL_ENTITY_KINDS`, `TypologyPrimitiveKind` | `ModelEntityKind` enum (`as_str`/`parse`), `PRIMITIVE_MODEL_ENTITY_KINDS`, `TypologyPrimitiveKind` |
| `listModelDefinitionManifests`, `defaultModelDefinitionId`, `kernelTypologyIds`, `isShapeModelDefinition`, `modelDefinitionUsesGeometryPicking` | same names, snake_case |
| `listModelDefinitionTypologies`, `loadTypology`, `typologyForInteraction`, `listTypologiesForModelDefinition`, `modelDefinitionIdForTypology`, `listAttributeDefinitionsForModelDefinition`, `modelDefinitionSelectionEntityKinds`, `modelDefinitionTypologyIds` | same |
| `resolveTypologyStyle`, `typologyStyleCacheKey`, `autoTypologyStyle`/`mergeTypologyStyle`, `hashTypologyId`/`hslToHex`/`darkenHexColor` | `resolve_typology_style`, `typology_style_cache_key`, private `auto_typology_style`/`merge_typology_style`/`hash_typology_id`/`hsl_to_hex`/`darken_hex_color`, plus `js_number` for `String(number)` parity in the cache key |
| `typologyObjectPascalFromLabel`, `spatialTypologyToggleLabel` | `typology_object_pascal_from_label`, `spatial_typology_toggle_label` |
| `typologyConstructAssetIds`, `typologyConstructModeActionIds`, `typologyHasNativeConstructKit`, `listConstructableTypologiesForModelDefinition`, `typologyConstructKitByInteraction` | same |
| `capabilityActionSpecJson`, `ensureTypologyObjectFromCreateDiff`, `typologyIdForInteractionCommit` | **not ported**: they mutate the TS `Model`/emit a TS `ActionSpec`; the Rust commit path already binds objects in `🕹️interaction/🦀️.rs`'s commit runner. |

**Style parity is pinned bit-for-bit**, not approximated: `auto_style_reproduces_the_typescript_derivation`
asserts `spatial.shape.primitive.box → #3eaecc / edge #2a768b / pattern #338fa7` and
`energy.energy.hull → #3ecca7 / #2a8b72`, values computed independently from the TS algorithm
(FNV-1a over UTF-16 code units, `hash * 137.508 % 360`, HSL(0.58, 0.52), `Math.round` channel bytes).
`authored_style_survives_resolution_verbatim` asserts the exact React expectation
(`#8B7355` / `0.78` / cache key contains `hatch`) plus the whole cache-key string.

---

### I2 — `⚙️engine/🧲️picking/🦀️.rs` (~950 lines) + `🧪️tests/🔬️unit/🦀️.rs` (42 tests) — **landed, green**

Mounted `pub(crate) mod picking` under `crate::editor::cad::engine` (it reads the `pub(crate)`
ephemeral `CadGeometry`/`CadObject` import types, so its own visibility must not exceed theirs).
Input signature is the pane's own `(&[CadObject], Option<&CadGeometry>)` pair — exactly what
`modes::edit::cad_pane_working_objects` already returns.

| React region / symbol | Rust twin |
|---|---|
| `SpatialPickKind`, `SpatialPickTargetKind`, `SPATIAL_PICK_TARGET_KINDS`, `CAD_PICK_GENERALITY` | same, plus `SpatialPickTargetKind::generality()` |
| `SpatialPickTarget`, `SelectionTarget`, `SpatialEntityFlags`, `SpatialPickKindToggles`, `SpatialTypologyToggles`, `SpatialPrimitiveToggles`, `SpatialToggleGroupState`, `CheckboxState` | same; the three `Partial<Record<…>>` toggle maps become small structs with an `enabled()` that reproduces React's `!== false`, so "unset" and "off" stay distinct |
| `GEOMETRY_KIND_TO_OBJECT_PICK`, `kernelGeometryKindForObjectPick`, `spatialPickKindsForSelectionAccept`, `pickTargetPrimitiveKind` | `geometry_kind_to_object_pick`, `kernel_geometry_kind_for_object_pick`, `spatial_pick_kinds_for_selection_accept`, `pick_target_primitive_kind` |
| `geometryBuckets` + `geometryRecords`/`geometryPointCentroid`/`geometryEdgePoints`/`geometryWirePoints`/`geometryFacePoints`/`geometryShellPoints`/`geometrySolidPoints`/`geometryAllVertexPoints`/`geometryEntityPoints`/`geometryEntityWireSegments`/`polylineWireSegments`/`collectGeometryEdgeSegments`/`collectGeometryEdgeSegmentsForMembers`/`collectSolidPrimitiveMemberIds` | `GeometryBuckets` with `edge_points`/`wire_points`/`face_points`/`shell_points`/`solid_points`/`all_vertex_points`/`entity_points`/`entity_wire_segments`/`all_edge_segments`/`edge_segments_for_members`/`solid_primitive_member_ids`. Authored asset order is preserved so target lists are deterministic. |
| `buildGeometryTypologyIndex`, `buildGeometryObjectIndex`, `listModelObjectsForModelDefinition`, `objectPrimitiveEntries`, `objectPrimaryPrimitiveRef` | same names (the last two private) |
| `createSpatialPickTargets`, `createModelObjectSpatialPickTargets`, `appendPrimitiveSpatialPickTargets`, `filterSpatialPickTargets`, `createSpatialPickEvent` | same; the pick event is built as a `DslValue` the statechart's `apply_event` consumes directly |
| every filter/toggle: `defaultSpatialPickKindToggles`, `filterSpatialPickTargetsForVisibility`, `spatialEntityFlagsForModelEntity`, `resolveSpatialEntityFlags`, `filterSpatialPickTargetsForEntityFlags`, `pruneSelectionTargetsForEntityFlags`, `intersectSpatialPickKindToggles`, `modelDefinitionPickTargetKinds`, `defaultSpatialPickKindTogglesForModelDefinition`, `defaultSpatialTypologyTogglesForModelDefinition`, `filterSpatialPickTargetsForTypologyToggles`, `spatialPickKindTogglesFromTypologyFilteredTargets`, `spatialSceneKindTogglesForModelDefinition`, `defaultSpatialPrimitiveToggles`, `filterSpatialPickTargetsForPrimitiveToggles`, `spatialToggleGroupState`/`Fill`/`CheckboxState`, `resolveSpatialSceneVisibility`, `filterSpatialPickTargetsForActiveView` | same names, snake_case |
| `spatialPickTargetKey`, `selectionTargetHoverKey`, `spatialPickTargetMemberKey`, `pinnedPickTargetKeys`, `spatialHoverKeyAliases`, `spatialHoverKeysMatch`, `revealedObjectIdsFromPickKeys`, `spatialPickTargetObjectRevealed`, `resolveSpatialPickTargetsToRender`, `canvasHoverKeyForSelectionTarget`, `spatialSelectionTarget` | same |
| `spatialSelectionModeFromModifiers`, `uniqueSelectionTargets`, `mergeSelectionTargets` | `spatial_merge_mode_from_modifiers`, `unique_selection_targets`, `merge_selection_targets` — **over `protocol::MergeMode`**, the one wire merge vocabulary, instead of a bespoke `SpatialSelectionMode`. `Range` behaves as `Replace` (a spatial canvas has no ordered topology, the same reason `marqueeModeFromModifiers` never emits it). |
| `spatialSceneColors`/`SPATIAL_SCENE_COLOR_FALLBACK`, `targetStyle`, `typologyStyleToMaterialProps`, `createSolidTypologyStyleResolver`, `WORLD_LOCKED_OPACITY_SCALE` | `SpatialScenePalette` (CSS-variable **token ids**, not resolved hex — the plugin never reads a theme, W1k's rule), `SpatialColorRef::{Token,Hex}`, `target_style`, `typology_style_to_material_props`, `solid_typology_style`, `WORLD_LOCKED_OPACITY_SCALE = 0.35` |
| `anchors` bucket (`AnchorRecord`) | `AnchorRow` + `read_anchor_rows`, read out of `CadGeometry::anchors` (untyped `DslValue` on the import struct, so the shape is read here rather than by widening a `pub(crate)` struct other lanes are mid-editing). Anchors get vertex pick targets and inherit their attachment's typology/owner, as React does. |
| screen-space half: `spatialPickTargetsFromRay`/`FromClientPoint`/`FromScreenSelection`, `targetRayScore`, `projectPointToClient`, `pointInRectangle`/`pointInPolygon`, `spatialSelectionCoverageFromGesture`, `dragDistance` | **deliberately not ported.** Host geometry, already Rust in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` (`marquee_is_crossing{,_from_path}`, `point_in_polygon`) and consumed by the renderer; a guest never sees client pixels. |

**Performance note found while wiring it:** React's point dedup keys on `p.join(",")`, and the first
port reproduced that with a `Vec<String>` scan — O(n²) with a `format!` per probe, on a pane with
thousands of kernel members. It is now `point_key` (the three `f64::to_bits`, with `-0.0` folded onto
`0.0` so JS's `"0"`/`"0"` collapse is preserved) into a `HashSet`, i.e. linear.

### I3 — `engagementPreview` had no wgpu reader at all — **landed, green** (framework seam)

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`. **New defect, not in any audit:**
`World3dScene::engagement_preview_json` was written by producers and read by React
(`🌐️World3dHost/🟦️.tsx:4027 EngagementPreviewLayer`) but had **zero** consumers on wgpu — the only
wgpu mention anywhere was the Interpreter's payload-size validator. Every rubber-band point, segment,
footprint box and height handle of every plugin construction interaction (all 60 CAD interactions,
and the same lane in generation3d/process3d) was invisible on that target while React drew it.

Landed: `WorldEngagementPreviewRecord` (the four React wire kinds `point`/`segment`/`box-preview`/
`linear-handle`), `World3dState::engagement_preview` synced in `sync_world3d_scene_document_lanes`
(and added to that function's change digest), and `append_engagement_preview_lines` called from
`render_world_3d` beside `append_component_overlays`, painting into the existing `LineDraw3d` lane
with `theme.row_hover` — which is `hover_interactive_fill`, the exact token React's `colors.hover`
resolves. A `point` draws as a three-axis cross and a `box-preview` as a 12-edge wireframe
(`append_box_wireframe_lines`): this target has one line pipeline and no per-item mesh, and React
already draws the box `wireframe`, so the silhouette matches. React's `0.05` extent floors and its
"height extrudes up from `cornerA.z`, never `cornerB.z`" rule are reproduced and pinned.

5 laws in `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`: lane parsing, absent/malformed lane, the
34-vertex paint of the four-kind fixture with the box top at `cornerA.z + height`, the degenerate-box
minimum extent, and "an item missing its geometry is skipped, never faulted".

### I4 — CAD wire payload now carries the engine's output — **landed, green**

1. **`world_instances_json` colour** (`🎭️modes/✏️edit/🦀️.rs`): was a hardcoded
   `selected ? "#3b82f6" : "#64748b"` premix; now `resolve_typology_style(&object.typology).color`.
   Both hosts layer selection/hover as separate booleans on top of the instance colour
   (`WorldEnvironmentMaterialRecord`'s own doc says so), so the premix both double-painted selection
   on one target and threw the typology away on both. Every CAD object now paints its authored (or
   deterministically auto-derived) typology colour on React **and** wgpu. Pinned by
   `world_instances_carry_their_typology_colour_not_a_selection_premix`.
2. **Geometry pick overlay** (`🎭️modes/✏️edit/🦀️.rs::pick_target_preview_items`): while the live
   session's state accepts a `selection.changed` (React's `replHostGeometryPickingEnabled` rule), the
   pane publishes its visible, unhidden, unlocked pick targets as `engagementPreview` `point`s
   (vertex/anchor kinds) and `segment`s (every other kind draws its own straight-edge wireframe, as
   React's `SpatialPickGeometryLayer` does). Coarsest kinds first (the pick generality order), capped
   at `CAD_PICK_OVERLAY_ITEM_BUDGET = 192` so a Concrete-Forest pane cannot overrun the bounded lane.
   Each item carries its `kind:id` pick key as `role`. Pinned by
   `the_pick_overlay_is_bounded_and_only_published_while_a_selection_is_accepted`.
   This is the OS-shell path's **first** sub-object pick affordance on either target.
3. **Inspection panel census** (`📌️panels/🔍️inspection/🦀️.rs::pane_geometry_census`): the summary
   branch now also lists, per pane, the typology-object and kernel-primitive pick-target counts and
   the per-typology object counts — React's `ModelStatsPane`/`SelectionPropertiesPane` summary,
   derived from the shared engine so both renderers report the same numbers. The three pre-existing
   summary rows are unchanged (their laws still pass).

---

## 4. Per-module status table

| Module | React source (lines) | Rust | Tests | Status |
|---|---|---|---|---|
| `⚙️engine/🧬️typology` | `🟦️.ts` 167 + kernel `📐️geometry/🟦️.ts` typology/manifest/style/attribute half | `🦀️.rs` 609 | 22 | ✅ ported, asset-embedded, style parity bit-for-bit |
| `⚙️engine/🧲️picking` (new; from `📺️renderer` regions 647-1380 + 2277-2888) | ~1 000 of the 6 766 | `🦀️.rs` ~950 | 42 | ✅ ported (screen-space half deliberately left to the framework) |
| `⚙️engine/🕹️interaction` | `🎰️stately/🟦️.ts` 309 + `🎬️actions/🟦️.ts` 1 886 + `🗿️artifact/🟦️.ts` 1 081 | `🦀️.rs` 1 050 (pre-existing) | 429 lines of laws (pre-existing) | ✅ already a generic interpreter over the same JSON assets |
| `⚙️engine/📔️registry`, `⚙️engine/🏃️runtime` | 186 + 210 | — | — | n/a by design: assets are `include_str!`-embedded and extensions are real wasm components declaring `contributes = ["cad.computer"]`; there is no glob/registrar step to port |
| `⚙️engine/📺️renderer` paint/interaction | 6 766 | the wire the Rust play app publishes (`modes::edit`) + the framework's own `World3d`/`UiNode` renderers | see I3/I4 | ◑ engagement preview + pick overlay + typology colour land here; per-target paint stays framework-owned (§1) |
| `♾️infinite/🌍️world` (wgpu `World3d`) | `🌐️World3dHost/🟦️.tsx` 8 071 | +`engagement_preview` lane | 5 | ✅ lane closed |

## 5. Remaining gaps / hand-offs

1. **`World3dScene` has no pick-target lane.** The instance lane is object-level only, so sub-object
   picking can be *shown* (I4.2) but not *hit-tested*: a click still resolves to the object through
   the `"cad"` domain's `object` granularity. Closing it needs a new `World3dScene` lane (id, kind,
   point, points, typology, style) plus readers in `🌐️World3dHost` and `♾️infinite/🌍️world`, then
   `merge_selection_targets`/`spatial_hover_key_aliases`/`resolve_spatial_pick_targets_to_render`/
   `target_style` wire up as the producers of that lane. That is the one remaining consumer for the
   half of `🧲️picking` that has no production caller today (the module mount documents this).
2. **Curved edges degrade to their endpoints.** `CadEdgeCurve` on the ephemeral import struct carries
   only `kind`, no centre/poles, so `arc`/`circle`/`ellipse`/`nurbs` edges sample as two points where
   React's `edgeSamplePoints` tessellates 32-64. Fixing it means widening the import struct (or
   reading the real `Brep` handle) — a schema change beyond this packet.
3. **`ensureTypologyObjectFromCreateDiff` / `capabilityActionSpecJson` / `typologyIdForInteractionCommit`
   not ported** — they mutate the TS `Model` or emit a TS `ActionSpec`; the Rust commit runner in
   `🕹️interaction/🦀️.rs` already binds object rows itself.
4. **The React editor package itself is untouched and still React-only**, by design: `@semio-tech/cad-js`'s
   `InteractionRepl`/`InteractionCanvas`/`InteractionSpatialView` are reachable only from
   `📖️stories/🎭️renderer/🧪️.story.tsx` and their own vitest suite, never from `💻️os`. If it is meant to
   stay, the parity laws now live on the Rust side and the two can be diffed; if it is meant to go,
   §1 says what would be lost (nothing the shell path uses).
5. **Locked objects are not dimmed on the wire.** `target_style`'s locked arm exists and is tested,
   but the instance lane has no per-instance opacity field, so `WORLD_LOCKED_OPACITY_SCALE` cannot be
   applied to a committed mesh from the plugin yet.

## 6. Verification

Every command run in the foreground, `-j 4`, plain env, one cargo at a time. Logs in `🗑️generated/w2f-*.txt`.

| Gate | Result |
|---|---|
| `cargo check -p semio-s-artifact-cad-cad -j 4` (baseline, before any edit) | ✅ 0 errors, 1 pre-existing warning (`CadObject` private-interface on `create_object_mutations`) |
| `cargo check -p semio-s-artifact-cad-cad --all-targets -j 4` (final) | ✅ 0 errors; lib warnings back to the same pre-existing 1 |
| `cargo test -p semio-s-artifact-cad-cad --lib -j 4 -- editor::cad::engine:: editor::cad::panels::inspection` | ✅ **100 passed, 0 failed** |
| `cargo test … -- the_pick_overlay… world_instances_carry…` | ✅ 2 passed |
| `cargo test -p semio-s-artifact-cad-cad --lib -j 4` (whole crate) | ⚠️ **406 passed / 18 failed** — see below |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-cad --lib -j 4` | ✅ 0 errors |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ 0 errors (54 pre-existing warnings, none in the world file's new code) |
| `cargo test -p semio-framework-os-infinite --lib -j 4 -- engagement_preview` / `-- degenerate_box_preview` | ✅ 5 passed |
| `bun nx run @semio-tech/cad-plugin:describe` | ✅ descriptor regenerated, `semio:cad@0.1.0`, no unclassified-verb abort (wasm `ebe30841…`, descriptor `4b8b2390…`) |

**The 18 whole-crate failures are peer-owned framework churn, not this packet's.** Every one panics
inside `🧰️framework/🛍️products/💻️os/🔨️modules/{🏪️store,🔌️plugin}/🦀️.rs` on one of four laws —
`tool proof catalog must exactly join migrated generated declarations to live concrete factories`,
`edit history insertion requires its exact mutation retirement factory`,
`artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority
detached every nested owner`, `artifact store reached Drop without its exact terminal-empty shallow-
shell witness`. Evidence they are not mine:

- `🔌️plugin/🦀️.rs` is `MM` in `git status` (staged **and** unstaged peer edits, mtime 01:19 today);
  its uncommitted diff rewrites `drive_artifact_envelope_decode_worker`'s close-page grant and the
  faulted-decode outcome path, citing ticket `26/09/17/TRINITY-PLUGIN-END-TO-END` — exactly the
  envelope-decode/retirement paths three of the four laws guard.
- The tool-proof fault's own detail reports `owner_eq=true controller_eq=true schema_eq=true
  unique=true typed_join=true` and rejects anyway: the law's predicate is internally inconsistent,
  which is a framework-side defect.
- None of the failing stacks enter any file this packet touched, and the same 18 fail with the
  identical list before and after the last CAD edit.
- The only two failures this packet *did* cause (`panels::inspection::tests::{summary_counts_every_
  pane_object_without_a_selection, unknown_selection_ids_fall_back_to_the_summary}`, from replacing
  the summary section instead of extending it) were found and fixed: the census now appends to the
  existing summary rows.

## 7. Files

Created
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧬️typology/🦀️.rs`
- `…/✏️editor/⚙️engine/🧬️typology/🧪️tests/🔬️unit/🦀️.rs`
- `…/✏️editor/⚙️engine/🧲️picking/🦀️.rs`
- `…/✏️editor/⚙️engine/🧲️picking/🧪️tests/🔬️unit/🦀️.rs`

Modified
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs` (engine module mounts)
- `…/✏️editor/🎭️modes/✏️edit/🦀️.rs` (typology colour, pick overlay, budget)
- `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` (pane geometry census)
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (2 wire parity laws)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` (engagement-preview lane + paint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` (5 laws)

Untouched on purpose: every `.tsx`/`.ts` in the CAD plugin (the React target still builds and its
own suite is unchanged), and every framework file other than the two `♾️infinite/🌍️world` ones.

## 8. Housekeeping

The machine hit `No space left on device` mid-run (893/926 GiB, 199 MiB free). Freed ~900 MiB safely
by deleting two abandoned `⚡️cache/cargo/target/.semio-describe-core-*` temp dirs from 2026-09-17
02:2x and trimming this packet's own logs; the incremental cache had **no** session older than
120 minutes, so the memory note's prune found nothing to take. A peer freed the rest shortly after.
