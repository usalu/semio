# W3b report — `🗺️surface/{🎨️paint,🕸️node-graph,🗺️tiled-map}`

Boundary: `🧰️framework/🔨️modules/🗺️surface/{🎨️paint,🕸️node-graph,🗺️tiled-map}/🦀️component.rs` (crate
`semio-framework-surface`). `🏔️terrain` untouched (already done, per `📌️important.md`). This report
follows the exemplar's **method**, not its outcome — every field below was traced to a real owner
before classification, not inferred from the struct's setter shape, and the recon's corrected
"real external consumers" finding (3/1/7/18 hits across `RasterHost`/`RasterSession`/`GraphHost`/
`MapHost`) was taken seriously rather than dismissed.

## Headline finding

**All three modules own no tier-(a) authoritative state, traced independently for each, not copied
from `🏔️terrain`.** Unlike terrain — where the owner was a demo-quality gis fixture DTO — all three
surface lanes mirror **real, shipped, event-sourced owners with existing mutation vocabulary**:

| Module | Mirrored field(s) | Real owner | Real triads already shipped there |
|---|---|---|---|
| `🎨️paint` | `RasterHost.document` | `✏️s/🔌️plugins/🖨️raster`'s `RasterSnapshot` (`crate::artifacts::raster::schema::{snapshot,diff,mutations}`) | 11: `create/delete/rename/move/resize/reorder-layers`, `change-layer-{opacity,blend-mode,visible,adjustment-kind}`, `add/remove-layer-asset` |
| `🎨️paint` | `RasterHost.{camera,brush_size,brush_opacity,active_utility,selected_ids,hovered_id}` | same plugin's `RasterConfig` (LOCAL_UI app state) | `RasterConfigMutation::{SetBrushSize,SetActiveUtility,…}` |
| `🗺️tiled-map` | `MapHost.features` | `✏️s/🔌️plugins/🌍️gis`'s `GismapSnapshot` (`crate::artifacts::gismap::schema::{snapshot,diff,mutations}`) | 12: `create/delete/reorder-{positions,routes,regions}`, `replace-{position,route,region}-data` |
| `🗺️tiled-map` | `MapHost.{render_mode,vector_style,forced_lod_id,layer_visibility,layer_stroke_scale,selected_positions,selected_routes,hovered_kind,hovered_id}` | same plugin's `Gis2dConfig` (LOCAL_UI app state) | `SetRenderMode`, `SetVectorStyle`, `ToggleLayerVisibility`, `SetLayerStrokeScale`, `set_lod_mode`, `set_hover`, `set_camera` |
| `🕸️node-graph` | `GraphHost.dag` (node positions, connections) | `💻️os/🔨️modules/🌊️flow`'s `FlowFixture` (`Widget`/`SynapseSpec` graph) | **none yet** — `🌊️flow/🌿️vcs` still dispatches through the banned `CollectionMutation<K,V,P>`/`Patch` shape at time of writing; W3c's flow lane owns the file (frozen, read-only, for this wave) and already has the target design (`move`/`connect`/`disconnect`) delivered to SMO |

**This is a different, and stronger, evidentiary basis than terrain's.** Terrain's owner was a demo
fixture DTO with a gap (`GisTerrainSnapshot` had no `project_origin` field at all). Here, every
mirrored field's real owner was independently confirmed to exist, be registered, and already carry
conforming mutation vocabulary (paint/tiled-map) or an explicit design target not yet landed
(node-graph). **No new `🧬️mutations` vocabulary is authored in this wave's boundary** — not because
the fields don't matter, but because authoring a second mutation set here would duplicate
authoritative state on the wrong owner, which is exactly the violation this ticket exists to remove.

## Why: the field-by-field trace, per module

### `🎨️paint/🦀️component.rs` — `RasterHost` (1838 → 1880 lines)

| Field | Apparent shape | Traced source | Tier | Disposition |
|---|---|---|---|---|
| `document: RasterDocument` | mutable struct field + `sync_document_json` | `RasterSnapshot.layers` in `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`, re-exported at `…/🖨️raster/🦀️component.rs:201` (`pub use crate::artifacts::raster::schema::snapshot::RasterSnapshot`). 11 real triads confirmed on disk under `…/🧬️schema/🧬️mutations/{🌱create-layer,🗑️delete-layer,✏️rename-layer,↔️move-layer,📐resize-layer,🔀reorder-layers,🌫️change-layer-opacity,🎨change-layer-blend-mode,👁️change-layer-visible,🎚️change-layer-adjustment-kind,🖇️add-layer-asset,🗂️remove-layer-asset}`, each with `mutation`/`diff`/`inverse` leaves. `add-layer-asset`'s own docstring: *"NOT one of the coordinator's ten mandated derivations; added so `image:in` media import … can stay a real, undoable operation now that whole-document replace is gone."* | (a) elsewhere, not here | Kept as-is; `sync_document_json` stays the passive mirror refresh |
| `camera`, `brush_size`, `brush_opacity`, `active_utility`, `selected_ids`, `hovered_id` | mutable fields + `set_*` methods | `RasterConfig` in `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config/🦀️component.rs:23-37` — same field names, `#[state(local_ui)]`-tagged in the config's own schema (`…/🎚️config/🧬️schema/🦀️component.rs:11,15`, `@state(class: LOCAL_UI)` in the GraphQL twin), with real `RasterConfigMutation::{SetBrushSize,SetActiveUtility,…}` variants at `…/🎚️config/🦀️component.rs:232,236` | (c) Preview/Effect, elsewhere | Kept as-is |
| `viewport` | mutable field + `set_size` | Render-session device size/DPR — not document content by any framing; no owner to trace to | (d) render-session wiring | Unchanged |
| `images: RasterImageCache`, `buffers: RasterLayerBuffers` | struct fields | `images` is a decoded-GPU-texture cache, wholly rebuildable from `buffers`. `buffers.paint` is the **active paint-gesture scratch buffer** — `paint_at`/`stroke_paint` write pixels directly during a brush stroke, never through a mutation. The eventual persisted commit is `image:in` asset import through the plugin's real `add-layer-asset` (see above); this host never calls that itself — it only holds the ephemeral scratch, analogous to `DraftEngineSession`'s "drop at any instant, nothing committed is lost" invariant | (d) ephemeral working representation | Not converted to `EngineRep<P>` — see "EngineRep applicability" below for why |
| `panning`, `painting`, `last_paint`, `pan_last`, `show_selection_chrome` | interaction flags | Pure interaction state, discarded on gesture release; no persisted counterpart anywhere | (c) Preview/Effect | Unchanged |
| `theme_clear`, `checkerboard_light_cell`, `checkerboard_dark_cell` | derived-from-theme fields | Recomputed from the app's UI theme via `set_canvas_theme_from_json`; same category as terrain's checkerboard cache | (d) runtime wiring | Unchanged |

Methods: `pick_targets_at_screen_json`, `marquee_hits_json`, `navigator_fit_camera_json`,
`navigator_viewport_overlay_json`, `build_*_scene`, `document_world_bounds` — all (e) pure compute
over the mirrored `document`/`camera`/`viewport`, already had zero mutable-beyond-struct state and
were left unchanged. `RasterSession` (wasm bridge, `#[cfg(target_arch = "wasm32")]`) is a thin
delegating wrapper; unchanged.

### `🕸️node-graph/🦀️component.rs` — `GraphHost` (1145 → 1169 lines)

The module's **own pre-existing docstring already stated the conclusion**: *"the OS infinite-board
projection remains authoritative — this host is a render-session cache."* Confirmed rather than
merely trusted, by tracing the actual consumer:

| Field | Apparent shape | Traced source | Tier | Disposition |
|---|---|---|---|---|
| `dag: DagHost` | pub struct field, mutated via `sync_from_payload`/`sync_from_scene_*` | `NodeGraphScenePayload` is built and fed by `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` (verified: `use flow::{dag::…, FlowFixture, FlowHost}`, `use framework_surface_node_graph::GraphHost`, `entry.node_graph = Some(NodeGraphEngine::Dag(GraphHost::default()))`, `engine.sync_from_scene_pack(&scene_pack)`) — the pack is derived from OS `flow`'s `FlowFixture` (`Widget`/`SynapseSpec` graph). **That owner is real but not yet event-sourced**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:141-143` still dispatches `FlowMutation::{Widgets,Synapses}(CollectionMutation<…>)` — the banned generic wrapper, confirmed by grep (`CollectionMutation` used directly, `Patch` variant present) — at time of writing. Per `📌️important.md`'s hot-file table this file is `🌊️flow` W3c's, frozen/read-only for this wave; W3c's design (`📓️wave3c-design/flow-target-shape.md`, already delivered to SMO) targets exactly `create`/`delete`-widget, `connect`/`disconnect`-synapse, `move`-widget — the verbs this ticket's brief anticipated | (a) elsewhere — real, owner traced, owner's own dissolution not yet landed | Kept as pure mirror; no vocabulary authored here or invented against an unshipped target |
| `catalogue_json`, `controls_json`, `capabilities_json` | pub `String` fields | `NodeGraphScenePayload`'s own field docs / usage: transient UI-panel content, never round-tripped back into `FlowFixture` or any snapshot found by grep | (c) Preview/Effect | Unchanged |
| `last_payload_signature: u64` | private field | Content-hash of the last-applied payload, purely a change-detection cache gating `sync_from_payload`'s rebuild — never read elsewhere | (d) runtime wiring | Unchanged |

`GraphSession` (wasm bridge) is a thin delegating wrapper; unchanged. No test changes were needed —
existing 20+ tests already cover `sync_from_payload`/`sync_from_scene_json`/`sync_from_scene_pack`
round-trips, camera, selection, hover, LOD, and picking.

### `🗺️tiled-map/🦀️component.rs` — `MapHost` (4236 → 4272 lines)

The exact field the coordinator's correction flagged (`MapHost.features`) **already carried its own
doctrine-correct docstring before this wave touched it** (`:1353`, pre-existing): *"GIS feature tables
mirrored from projection JSON — not authoritative document state (OS `ArtifactStore` owns packs)."*
This wave's contribution was **confirming that claim against the real owner on disk**, not merely
trusting the comment:

| Field | Apparent shape | Traced source | Tier | Disposition |
|---|---|---|---|---|
| `features: MapFeatureTables` (`positions`/`routes`/`regions`) | pub struct field + `sync_map_json` | `GismapSnapshot.{positions,routes,regions}: Vec<MapFeature>` in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:17-25`. Confirmed the mirror mechanics directly: `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🗺️maphost/🦀️component.rs:16-19` — `pub fn map_host_from(document: &GisMapSnapshot, cfg: &Gis2dConfig) -> MapHost { let mut host = MapHost::new(); … host.sync_map_json(&descriptor); … }` — a **fresh `MapHost` is rebuilt from the snapshot on every call**, exactly the mirror pattern, not a divergent second copy. 12 real triads confirmed on disk under `…/gismap/…/🧬️schema/🧬️mutations/{🆕create-position,🗑delete-position,🔀reorder-positions,🔁replace-position-data,🛣️create-route,✂️delete-route,🧭reorder-routes,♻️replace-route-data,🌐create-region,🧹delete-region,🔃reorder-regions,🔄replace-region-data}` | (a) elsewhere, not here — confirmed, not merely inherited from the pre-existing comment | Kept as-is; no per-feature mutation methods exist on `MapHost` to begin with (only wholesale `sync_map_json`), so there was nothing here that could have been mistaken for authoritative editing |
| `render_mode`, `vector_style`, `forced_lod_id`, `layer_visibility`, `layer_stroke_scale`, `selected_positions`, `selected_routes`, `hovered_kind`, `hovered_id` | mutable fields + `set_*` methods | `Gis2dConfig` in `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎚️config/🦀️component.rs` and its command dispatch in `…/◻2d/🦀️component.rs:139-165` — real `SetRenderMode`, `SetVectorStyle`, `ToggleLayerVisibility`, `SetLayerStrokeScale`, `set_lod_mode`, `set_hover`, `set_camera`, `fit_world` commands, confirmed present and dispatched | (c) Preview/Effect, elsewhere | Kept as-is |
| `camera`, `viewport` | pub fields | Same pattern as paint/terrain — live viewport camera, Preview/Effect | (c) | Unchanged |
| `tiles: MapTileLedger` (raster + vector tile cache) | private struct field | Network-fetched PNG/PBF tile bytes keyed by `(z,x,y)`, evictable LRU-shaped ledger — same category as `🏔️terrain`'s `elevation`: not derivable from any snapshot, fails `EngineRep::build(&P)`'s contract by construction | out-of-doctrine external-resource cache | Kept as-is, documented; not forced into `EngineRep` |
| `events: Vec<serde_json::Value>` | pub field | Interaction event log drained each frame by the render loop; no persisted reader found | (c) Preview/Effect | Unchanged |
| `interaction: MapInteraction` | private enum field | Current pan-gesture state (origin + start screen point), discarded on release | (c) Preview/Effect | Unchanged |
| `theme: MapPalette` | private field | Derived from UI theme via `set_map_theme_from_json`, same category as paint's `theme_clear` | (d) runtime wiring | Unchanged |

Methods: the large label-declutter, tile-visibility, LOD, and picking machinery (`visible_tiles_json`,
`weight_slider_keys_at_lod`, `position_screen_json`, `map_point_in_polygon`, etc.) are (e) pure compute
over `features`/`camera`/`viewport`/`tiles`; unchanged, already well covered by the file's existing
~130 tests (visible in the baseline test list below).

## `EngineRep<P>` applicability — investigated, not force-fit

Read `//#region 🔖️EngineRep` (`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:175-194`) in
full before deciding. `RasterHost.document` and `MapHost.features` are, unlike terrain's `elevation`,
**genuinely wholly derivable from a real snapshot** (`RasterSnapshot`/`GismapSnapshot`) — a better
structural fit for `EngineRep`'s contract than terrain's network-fetched cache was. They were **not**
converted, for a reason specific to `EngineRep`'s own docstring: *"Built ONLY inside a `🔺️diff`
constructor or an `InferredField::{plan,dep_input,compute}` body. Dropped when that function returns.
Never a durable struct field… never crossing a dispatch boundary."* `document`/`features` are read
across an entire render session — pointer picking, painting, multiple render frames — which is
precisely the multi-call, cross-dispatch lifetime `EngineRep` forbids; retrofitting it here would
either break the render use case or misuse the abstraction as a general-purpose cache with a stricter
name. This host-level mirror-refreshed-wholesale-on-sync shape is a **recognized third category**
alongside `EngineRep` and the out-of-doctrine external-resource cache — the same category terrain's
`origin_lon`/`origin_lat`/`exaggeration` mirror setters already established and were deliberately kept
in, not converted. Flagged for IIF/coordinator confirmation exactly as terrain flagged its own open
inference question, not asserted as settled doctrine.

## Framework-module schema placement — still unpiloted, through no fault of this wave

Per the brief, this wave was told it might be the first to exercise
`<module>/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs` directly under a framework module.
**It is not, because no vocabulary was authored here to place.** Checked before concluding this is a
non-issue rather than an oversight: brep/drawing/mesh (W4) all landed their schemas under
`✏️s/🔌️plugins/🗄️stdio/**`, not under a framework module (W3a-0's design explicitly kept
`semio-framework-3d` as pure compute with the authoritative snapshot staying in stdio). So the
placement question remains genuinely unpiloted repo-wide after this wave too — recorded here rather
than silently dropped, since two consecutive surface-family waves (`🏔️terrain`, now this one) reaching
the same "no vocabulary" outcome for different, independently-traced reasons is itself worth the
coordinator knowing before assuming a third surface-adjacent lane will behave differently.

## Verbs

None derived, none sent to SMO. Both `📓️taxonomy.md` and `📓️derivation-rules.md` were read in full
before reaching that conclusion (not skipped as unnecessary) — same discipline the exemplar used.
`GraphHost`'s real document owner (`flow`'s `Widget`/`SynapseSpec` graph) does have anticipated verbs
(`move`, `connect`, `disconnect`) per this ticket's own W3c design docs, already delivered to SMO —
but that dispatch enum doesn't conformingly exist yet (still the banned `CollectionMutation`/`Patch`
shape), so binding a verb to it here would be inventing vocabulary against a target that hasn't
landed. Left unauthored and flagged, per the sanctioned "leave the enum EMPTY and flag it" outcome —
not applicable narrowly to two verbs here (unlike brep's `create-loop`/`delete-loop` gap), but to the
whole lane, because there is no dispatch anywhere in this boundary to bind verbs to.

## What changed

- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs`: module docstring (doctrine classification
  + citations to `RasterSnapshot`/`RasterConfig`), `RasterHost` struct field docstrings (every field,
  tier + reasoning). Zero public API change. 1838 → 1880 lines.
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs`: module docstring extension, `GraphHost`
  struct field docstrings. Zero public API change. 1145 → 1169 lines.
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`: module docstring extension, `MapHost`
  struct field docstrings. Zero public API change. 4236 → 4272 lines.
- No `🧬️mutations`/`🧬️schema` directories created anywhere in this boundary (see "Headline finding").
- No test files created or modified — existing suites (214 tests total across the crate) already
  cover the pure-compute surface of all three modules; no gap comparable to terrain's missing
  determinism assertions was found on inspection of `pick_targets_at_screen_json`,
  `navigator_fit_camera_json`, `sync_from_payload`, `visible_tiles_json`, `weight_slider_keys_at_lod`.

## Verification commands run, with real output pasted

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="<ticket>/🎯️target" cargo test -p semio-framework-surface --lib
```
```
test result: ok. 214 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.04s
```
Identical to the baseline recorded in `📓️wave2-reports/terrain-report.md`'s coordinator-verified gate
(214/214) — **zero regression, zero new tests needed** (docstring-only diff).

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="<ticket>/🎯️target" cargo check -p semio-framework-surface --all-targets
```
0 errors. `semio-framework-surface (lib) generated 69 warnings`, `(lib test) generated 80 warnings` —
same pre-existing warning population the exemplar recorded (dead-code/unused-field debt in
paint/node-graph/tiled-map predating this wave); none newly introduced by this diff (docstrings only,
verified by re-reading the diff — no field visibility, type, or signature was touched).

### Consumer crates — checked, not assumed clean

Docstring-only changes are behavior-preserving by construction, but the ticket's own rule ("if your
change breaks them, that is your change breaking them") was treated as requiring a real check, not an
argument from principle:

```
cargo check -p semio-s-plugin-raster   → 0 errors, 36 warnings, Finished
cargo check -p semio-framework-os      → 0 errors, 10 warnings, Finished   (owns EngineCanvas, the real GraphHost/RasterHost/MapHost render-loop consumer)
cargo check -p semio-s-plugin-puzzle   → 0 errors, 67 warnings, Finished
cargo check -p semio-s-plugin-gis      → 3 errors — see below
```

**The gis errors are pre-existing, unrelated churn, not this wave's.** All three are `E0433`/`E0432`
in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
and a sibling terrain-window file, both reaching a missing `crate::modules::terrain` — **note the
glyph trap `📌️important.md` warns about**: this is gis's own `🏔️gisterrain` artifact subset, a
completely different thing from this ticket's `🗺️surface/🏔️terrain` (already-done exemplar). Verified
not mine: `stat -f '%Sm'` on the error-source file → **Aug 13 00:13**, committed (`git log -3` →
flags 499/490), predating and unrelated to any file this wave touched (nothing under
`✏️s/🔌️plugins/🌍️gis/**` was edited). Not fixed — out of boundary, another session's in-flight or
already-landed defect in a different subset than the one this wave read from (`🗺️gismap`, which
checks fine as part of the same crate's compile — the errors are isolated to `🏔️gisterrain`).

## sharedFileRequests

None. No consumer file needed a patch — every field this wave classified stays exactly as it was
structurally; only docstrings were added.

## Concurrent-churn observations

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/…` — pre-existing `E0433`/`E0432` (missing
`crate::modules::terrain`), mtime Aug 13 00:13, already committed at flag 499. Not this wave's subset
(`🗺️gismap`, read-only, traced but not touched), not this wave's file, not caused by this wave's
docstring-only diff. Recorded, not fixed.

## Honest pass/fail

**Pass.** Boundary green at 214/214 (unchanged from baseline), zero new errors in any of the four real
external-consumer crates checked, zero regressions. The finding is negative in the same sense
terrain's was — no `🧬️mutations` vocabulary authored — but reached independently for three modules via
real per-field tracing (confirmed owners, confirmed shipped triads, confirmed unshipped-but-designed
target for node-graph) rather than by generalizing terrain's result, which the brief explicitly
forbade. The schema-placement pilot remains genuinely unexercised repo-wide, reported rather than
silently left implicit. `create-loop`/`delete-loop`-style narrow gaps do not apply here; instead the
one open item is node-graph's verbs, which are designed but wait on `🌊️flow/🌿️vcs`'s own dissolution
(W3c's boundary, not this wave's) landing before they can be authored conformingly anywhere.
