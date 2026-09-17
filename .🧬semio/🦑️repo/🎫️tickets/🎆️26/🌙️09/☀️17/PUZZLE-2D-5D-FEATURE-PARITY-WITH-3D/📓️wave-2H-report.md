# 📓️ Wave 2H — target regions IN the board engine (paint, hit-test, gestures, `region*` events)

Slice 2H finishes what `📓️wave-2F-report.md` §4.2 left owed: the Rust board engine now ingests, paints,
hit-tests and drags puzzle ◻️2d target regions, and publishes three new board events. 2F's TS-side
alt+click stopgap is **gone** — one mechanism remains, and it lives in the engine.

Pattern followed: 2E's rotate gumball (generic geometry in the board core, gesture + paint + events in
the directed-normal port, generated styling tokens, laws in both test files).

---

## 1. What landed

### 1.1 Styling tokens (generated — JSON source + every twin, 2E's rule)

Added to the SOURCE `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:267-270` and hand-applied to all four
generated twins (`🔤️tokens/🦀️.rs:189-192`, `🔤️tokens/🐍️.py`, `🤖️generated/🔤️tokens/🟦️.ts` ×2 sites,
`🌓️theme/🔣️.json`):

| token | value | what |
|---|---|---|
| `regionGripPx` | 9 | screen size of a corner grip square |
| `regionHitTolerancePx` | 6 | screen width of the edge/corner grip band |
| `regionMinExtentWorld` | 4 | the floor neither a resize nor a paint may go under |
| `regionLabelInsetPx` | 6 | inset of the corner label tag |

### 1.2 Board core — generic geometry (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`)

New `#region 🔖️TargetRegions` (`:655-799`, between `TransformGumball` and `Engine`):

| symbol | what |
|---|---|
| `RegionData { id, x, y, width, height, label, hidden, locked, selected }` | the engine's copy of one `targetRegions` row |
| `region_bounds(x, y, w, h) -> [f64; 4]` | the ONE normalization of a corner-plus-extent rectangle (a negative extent reads identically) |
| `RegionGrip` (`Body`/4 edges/4 corners) + `name()` + `edges()` | which part of a rectangle a pointer grabbed; `edges()` says which of the four bounds it moves |
| `region_grip_at(bounds, zoom, point)` | grip under a point; corners > edges > body, band is a constant SCREEN width |
| `region_grip_drag(bounds, grip, dx, dy)` | applies a drag to the edges the grip owns, clamped at `REGION_MIN_EXTENT_WORLD` so no extent can collapse or invert |
| `snap_region_scalar(value, step)` | the one snap definition paint/move/resize share |
| consts `REGION_GRIP_PX`, `REGION_HIT_TOLERANCE_PX`, `REGION_MIN_EXTENT_WORLD`, `REGION_LABEL_INSET_PX` | re-exported from the tokens |

Re-exported through `🎲️board/🔌️ports/➡️directed/🦀️.rs:1203-1207`.

### 1.3 Descriptor + fixture contract (`…/➡️directed/🦀️.rs`)

- **`RegionDescJson`** (`:231-253`) — `{id, x, y, width, height, label?, hidden?, locked?, selected?}`,
  derived `ToValue`/`FromValue` (unlike its node/edge siblings it carries no free-form `userData`).
- **`SceneDescriptorJson.regions: Vec<RegionDescJson>`** (`#[serde(default)] #[value(default)]`).
- **`FixtureJson.target_regions: Vec<serde_json::Value>`** (`#[serde(default, rename = "targetRegions")]`)
  plus the two hand-written `dsl::ToValue`/`FromValue` arms — `FixtureJson` is hand-written, not derived.
- **`ActiveUtility::AreaBrush`** (`:433-437`) — a third utility next to `Select`/`Brush`.

> 🎯️ **Why the fixture and not a new scene lane.** The guest publishes the WHOLE document as
> `Board2dScene.fixture_json` (`cached_fixture_json(document_json, &envelope.fixture)`), and 2F's
> `targetRegions` is a member of that document. The regions therefore already ride
> `Board2dSceneLane::Fixture` — the lane the brief asks for — and a second carrier would have been a
> second mechanism. Only the brush SIZE needed a new scene field (§1.5).

### 1.4 Board host (`…/➡️directed/➕️normal/🦀️.rs`) — new `//#region 🎯️TargetRegions` (`:12820-13086`)

- **State:** `BoardHost.regions: BTreeMap<String, RegionData>`, `region_drag: Option<BoardRegionDrag>`,
  `region_paint: Option<BoardRegionPaint>`, `area_brush_extent: (f64, f64)` (default one large grid cell).
  `BoardRegionDrag` holds a grab-time `start_bounds` and re-derives every frame, so a drag accumulates
  no float drift and a cancel restores the exact rectangle — 2E's `BoardTransformDrag` law.
- **Ingest:** `fixture_scene_descriptor` reads `f.target_regions` (`:10100-10122`); a malformed ROW is
  skipped rather than refusing the whole document (regions are optional by construction, a board that
  cannot paint its constraints must still paint its graph). `sync_descriptor_with` retains/inserts them
  and folds `selected` into the selection set; `clear_scene` clears them.
- **Census:** `descriptor_admission` counts regions in the entity total against
  `BOARD_DESCRIPTOR_ITEM_CAPACITY` and their ids against `BOARD_DESCRIPTOR_BYTE_CAPACITY`. A gesture
  spends **no** pointer credits — the drag emits nothing until release.
- **Paint:** `append_target_region_paint` is appended at the TOP of `append_cached_world_content`, in
  world space, before the node fill layer — regions are the backdrop of the graph, never a lid.
  Translucent fill + outline (dashed when locked), selected/hovered stroke colours, four corner grips on
  the selected unlocked one, and a dashed live paint rectangle.
- **Hit test:** `region_grip_hit_world` is asked in `pointer_down_screen` **only** after
  `resolve_hit_world` (handles → nodes → edges), the rotate ring and the bounded-selection drag have all
  missed, and immediately before the background marquee. Hidden regions are neither painted nor picked.
  A real grip outranks a body, so an enclosing rectangle cannot swallow the one inside it.
  `resolve_pick_targets_world` publishes regions LAST, domain `"region"`, generality 3.
- **Gestures:** body drag = move, corner/edge grips = resize, both with a live preview and grid snap
  (`region_snap_step` → `lod_visible_grid_snap_step_world`, the same step the node drag uses). A LOCKED
  region still paints and still SELECTS (so the inspector can unlock it) but refuses every grab.
  `areaBrush` armed: a press anchors a rectangle, a drag sizes it, a release commits; a click that never
  left its anchor paints the configured brush extent.
- **Events — ONE per gesture, on release:** `regionCreate {x,y,width,height}`,
  `regionMove {id,x,y}` (corner only — a move never re-states an extent it did not touch),
  `regionResize {id,x,y,width,height}` (corner AND extent, because a west/north grip moves both).
  A release that moved nothing, or a rectangle under the extent floor, commits nothing.
- **Retirement:** new `BoardHostClosePhase::Regions` between `Wires` and `Selection` (pops one region
  and its two strings per step, then the drag); `nonopaque_terminal_is_empty` witnesses
  `regions.is_empty() && region_drag.is_none() && region_paint.is_none()`.
- **Vitals:** `target_regions_json()` — id-ordered rows with normalized bounds that reflect a live drag.
  `interaction_json` gained modes `regionPaint`/`regionDrag` and utility `areaBrush`;
  `defers_descriptor_sync_from_js()` is true while either gesture is live, so a scene echo cannot clobber
  the preview mid-gesture.
- `set_active_utility` accepts `"areaBrush"` and cancels any live region gesture on a switch;
  `set_area_brush_extent(w, h)` refuses a non-finite or non-positive axis rather than collapsing.

### 1.5 Scene contract + hosts

- `Board2dScene.area_brush_size: Option<String>` — `{"width":f64,"height":f64}` in WORLD units.
  Touched: `🎬️scenes/🦀️.rs` (struct, `scene_pack_wire!`, `base()`, `ToValue`, `FromValue`),
  `🎬️scene/🟦️.ts`, `🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` `spineFields`. The two 5d board
  literals pass `None`.
- **wgpu desktop** `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `BoardSyncCache.area_brush_size`,
  the `sync_board_engine` arm, `board2d_area_brush_size_from_json` (malformed ⇒ `(0,0)`, which the engine
  reads as "keep the extent you have"), plus the `close_board_sync` / `board_sync_terminal` entries.
  `active_utility` already forwarded, and the fixture already carries the regions.
- **Wasm bridge** `EDITOR2/🌉️wasm/🦀️.rs` — `setAreaBrushExtent(w, h)` and `targetRegionsJson()`;
  both declared on `Board2dWasmSession` (`🪪️WasmSessionLoader/🟦️.tsx`).
- **`Board2dHost/🟦️.tsx`**
  - **REMOVED** 2F's stopgap: `board2dAreaBrushCommits` and the `onPointerDown` gate that dispatched
    `addTargetRegion` instead of letting the gesture reach the engine. Nothing else referenced them.
  - `regionCreate`/`regionMove`/`regionResize` joined `PUZZLE2D_FLUSH_NOW_EVENT_NAMES` (they commit on
    release, so they must not wait for the next batch).
  - New effect syncs `scene.areaBrushSize` → `session.setAreaBrushExtent`, via the new exported
    `parseBoard2dAreaBrushSize` (re-exported from `🎯️targets/⚛️react/🟦️.tsx`).
  - `publishBoardVitals` now also writes **`data-board-target-regions-json`** — the exact attribute
    `🔍️browser-probe.ts:156,2116` reads (2G's `target-regions` lane). Imperative only, like the other
    live vitals, so a React re-render cannot stamp a stale frame over a live drag.

### 1.6 Guest — EDITOR2

- `🎮️commands/🎲️apply-board-events/🦀️.rs`
  - `regionCreate` → `puzzle2d_push_target_region` (the document push `addTargetRegion` itself ends in);
  - `regionMove` / `regionResize` → `puzzle2d_relocate_target_region` with
    `{"position":[x,y]}` (+ `"size"` for a resize) — 2F's own reducer, which refuses a locked region, so
    the engine's refusal and the document's agree;
  - `nodeDelete` now also asks `delete_target_regions_from_fixture`, so **one** delete path covers every
    granularity (`deleteSelection` already removed selected regions — 2F);
  - `puzzle2d_board_events_scope`: `regionCreate` = window bodies + layers + properties,
    `regionMove`/`regionResize` = window bodies + properties (no catalogue/engagement churn).
- `✏️editor/🦀️.rs` — new `puzzle2d_area_brush_extent_world(runtime)` (the steppers count grid cells;
  world extent = `cells × grid_factor`, the identical product `puzzle2d_paint_target_region` already
  used, so a canvas click and a dispatched `addTargetRegion` paint the same rectangle), and
  `puzzle2d_push_target_region(fixture, x, y, w, h)` — now the ONE place a region row is born;
  `puzzle2d_paint_target_region` delegates to it. `sync_host_runtime_state` forwards
  `host.set_area_brush_extent(...)`.
- `🎭️modes/✏️edit/🦀️.rs` — `puzzle2d_board_scene` publishes `area_brush_size`.

### 1.7 Registries

**No new verb, so no registry row.** `applyBoardEvents` already declares
`[Artifact, WindowConfig, WindowTransient, Interaction]` (`✏️editor/🦀️.rs:2243`) — the `Artifact` lane
the three new arms write is covered, so the publication contract needed no change (verified by reading
it, not assumed). `puzzle2d_board_events_extent` is row-count based and stays honest at one unit per
region row. Twelve `SceneDescriptorJson` literals in three 2d engine test files gained `regions: vec![]`
(mechanical, one line each).

---

## 2. Laws added — 14, all executed green

**Board core** (`♾️infinite/🎲️board/🧪️tests/🔬️unit/🦀️.rs`, `//#region 🎯️TargetRegions`) — 4:
`region_bounds_normalize_a_negative_extent`,
`region_grip_prefers_corners_then_edges_then_the_body` (including: the band is a constant screen width,
so zoom 4 puts the same world offset outside it),
`region_grip_drag_moves_only_its_own_edges_and_never_collapses`,
`snap_region_scalar_quantizes_only_under_a_real_step`.

**Board host** (`…/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`, `//#region 🎯️TargetRegions`) — 8:
`target_regions_are_ingested_and_hit_tested_after_every_entity` (a node INSIDE a region wins the press),
`an_area_brush_drag_commits_exactly_one_region_create` (drag frames announce nothing),
`an_area_brush_click_paints_the_configured_extent_and_a_collapsed_one_paints_nothing`,
`region_body_and_grip_drags_commit_exactly_one_event_each` (+ a release that moved nothing commits nothing),
`region_gestures_snap_only_under_the_grid_snap_modifier`,
`a_locked_region_refuses_every_drag` (but still selects),
`a_hidden_region_is_neither_hit_tested_nor_pickable`,
`regions_count_against_the_descriptor_census_and_never_the_pointer_credits`.

**Guest EDITOR2** (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `//#region 🎯️BoardRegionEvents`) — 2:
`board_region_events_commit_one_edit_each_through_the_target_region_verbs`,
`the_board_scene_carries_the_area_brush_extent_and_the_regions_ride_the_fixture`.

---

## 3. Commands run — verdicts

Evidence under `TICKET/🗑️generated/2H/`.

| # | command | verdict |
|---|---|---|
| 1 | `CARGO_INCREMENTAL=0 cargo check -p semio-framework-os-infinite --message-format=short` | ✅ **exit 0**, `Finished dev profile in 42.01s`, **0 errors**, 4 crate warnings (all pre-existing, none in my files). `infinite-check-1.txt` |
| 2 | `CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib -- region` | ✅ **11 passed, 0 failed** (4 core + 7 host). `infinite-region-laws-2.txt` |
| 3 | `… --lib -- area_brush` | ✅ **2 passed, 0 failed** (the click/collapse law + the drag law) |
| 4 | `… --lib -- board_host` (regression sweep) | ✅ **44 passed, 0 failed** — every pre-existing board-host law, including 2E's eight gumball laws, still green |
| 5 | gated `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | ✅ **exit 0**, `Finished dev profile`, **0 errors**, 5 warnings in the 2d lib — all pre-existing (4 `unnecessary qualification` in `🪟️window/🦀️.rs`, 1 unused import in `🎭️modes/✏️edit/🦀️.rs`), none mine. `puzzle2d-check-1.txt` |
| 6 | gated `… --target wasm32-wasip2 …` | ✅ **exit 0**, `Finished dev profile in 1m 25s`, **0 errors**, 5 pre-existing 2d warnings. `puzzle2d-wasip2-check.txt` |
| 7 | gated `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- board_region_events the_board_scene_carries` | ✅ **2 passed, 0 failed**. `puzzle2d-region-laws-3.txt` |
| 8 | `CARGO_INCREMENTAL=0 cargo check -p semio-framework-os-renderer-wgpu --message-format=short` | ❌ **11 errors, none in my file** — see §4.2. `renderer-wgpu-check.txt` |

Build gate used throughout: the **20:10 corrected** one
(`ps -axo args= | awk '$1 ~ /\/cargo$/ && /semio-s-artifact-puzzle-2d/'`). It opened immediately every
time (2 live cargos); total wait across the slice was under one minute.

---

## 4. Not mine — findings for the coordinator / other slices

### 4.1 🚨 `cargo test -p semio-s-artifact-puzzle-2d --lib -- region` has 16 pre-existing failures (2F)

Run #7 above is filtered to my two laws. The wider `-- region` filter reports **17 failures, 16 of them
2F's**, captured in `puzzle2d-region-laws-1.txt`:

- **15 × `…::committed_json_is_canonical`** across all seven region mutation leaves. This is exactly the
  float-canonical-form problem 2F itself flagged in its §4.4 — `serde_json::Number::eq` treats
  `PosInt(20)` and `Float(20.0)` as unequal. Owner: 2F / the 2d test-suite integrator.
- **1 × `fill_run_job_places_only_inside_visible_target_regions`** — panics with
  *"a region-constrained run must still place something, or this vector proves nothing"*
  (`⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:275`). 2F's own §4.3 predicted this: the
  `targetRegion` vector in `🎞️fill-run.json` states a law it had never actually run. Owner: 2F.

Neither is reachable from this slice's code — both were red before 2H and are red in the same way after.

### 4.2 `semio-framework-os-renderer-wgpu` is red at HEAD, in two other slices' files

11 errors, **zero** of them in `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (my only file in that
crate):
- `🧊️renderer/🦀️.rs:5816,6935,7041` — `MountedReplayRouteSeed` / `JobReplayRoute` have no field
  `document` (4 errors);
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3981,4023,4581,6841,6902,6974` — `semio_framework::ViewModel`
  initializers missing `tree_viewport_rows` and `tree_windows` (6 errors).

Both look like a peer's half-landed struct change. Left alone per hard rule 3. **Consequence for 2H:**
the wgpu `sync_board_engine` arm is compiler-checked only as far as the crate got — it produced no error
of its own, but the crate never reached `Finished`.

### 4.3 Region mutations do not reach `InvocationResult.mutations` — 2F's path, not the board's

My first draft of `board_region_events_commit_one_edit_each…` asserted
`!result.mutations.is_empty()` and failed. Measured with a probe dispatch in the same harness:

```
[DEBUG] 2H board mutations=0 verb mutations=0
```

i.e. `addTargetRegion` — **2F's own verb, dispatched directly** — also reports zero mutation rows, while
`nodeRotate` in the same app reports a non-empty set. The document fold is correct either way (the law
asserts the fixture, and the printed `targetRegions` array carried the exact painted geometry), so this
is a gap in the region differ's reach through `Puzzle2dPlayApp`'s preparation, not in the board-event
arms. The law therefore pins the document fold and says so in a comment. **Owner: 2F / the 2d
integrator** — worth checking before the battery, because an edit that journals no mutation also writes
no history row.

---

## 5. What is NOT verified

1. **No pixel has been looked at.** The region paint (fill/outline/grip squares/label tag) is
   law-covered for geometry and event emission only.
2. **The board engine owns no text primitive.** There is no `Scene::draw_text`/glyph API anywhere under
   `♾️infinite/` (grepped), so a region's `label` is painted as a small corner **tag**, not as glyphs. The
   string itself reaches the reader through `target_regions_json()`, the outliner row and the inspector
   (2F). If real in-canvas labels are wanted that is a board-engine text feature of its own, not a
   region feature — flagged rather than faked.
3. **`bun ./📜️script.ts typecheck` / the renderer's `bun test` were not run.** 2B measured the repo-wide
   TS typecheck as already red (858 errors) before its own change; I added one exported helper
   (`parseBoard2dAreaBrushSize`), one optional scene field and two optional session methods, and removed
   one exported helper that nothing else referenced (grepped). Owed to integration.
4. **`cargo check -p semio-s-plugin-puzzle --target wasm32-unknown-unknown --no-default-features`** — the
   only check that compiles the two new `#[wasm_bindgen]` bridge methods. Not run (2E's §4 note; the same
   gate is owed for its three methods).
5. **No runtime / battery.** The area-brush drag, the region grips, the `data-board-target-regions-json`
   attribute and the flush timing are source-and-law arguments, not measurements.
6. **`Board2dSceneLane` spine-field fixture.** I appended `"areaBrushSize"` to `spineFields` the way 2E
   appended `"transformFlags"`; that list is already missing several live fields (`gridVisible`,
   `domainId`, `suggestionMenuJson`), so if any oracle asserts exact set equality it was red before this
   slice for other reasons.

---

## 6. Hand-offs

- **Slice 2G / battery.** `data-board-target-regions-json` now exists and is published imperatively from
  `publishBoardVitals`; rows are `{id,x,y,width,height,label?,hidden,locked,selected}` with **normalized**
  bounds. The probe's `target-regions/area-brush-paints-a-region` gesture (arm `areaBrush`, then a plain
  click-**drag** rectangle, no alt) is exactly what the engine now implements — the alt modifier is no
  longer required and no longer special. Three new board event names to expect: `regionCreate`,
  `regionMove`, `regionResize`, all flush-now.
- **Slice 2F.** Your §4.2 is closed. Your `board2dAreaBrushCommits` predicate and the `onPointerDown`
  gate are deleted (the engine owns the gesture); `puzzle2d_paint_target_region` now delegates to the new
  `puzzle2d_push_target_region` and is otherwise unchanged. §4.1 and §4.3 above are yours.
- **Slice 2C (panels).** `resolve_pick_targets_world` now offers a `"region"` domain at generality 3, so a
  right-click on empty board inside a region reaches a region target. No panel file was touched.
- **Slices 2B / 2E (`Board2dHost`).** My edits there are four small anchored ones: the flush-now set, the
  new `data-board-target-regions-json` line inside `publishBoardVitals`, one new scene-sync effect, and
  the deletion of 2F's alt+click gate. `parseBoard2dAreaBrushSize` sits next to `parseBoard2dTransformFlags`.
- **Slice I2 (2d integrator).** My 2d-crate footprint is: `✏️editor/🦀️.rs` (two helpers + one host-sync
  line), `✏️editor/🎮️commands/🎲️apply-board-events/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🦀️.rs` (one scene
  field), `✏️editor/🌉️wasm/🦀️.rs` (two methods), `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (two laws), and
  `regions: vec![]` in three `⚙️engine/**/🧪️tests` files. The crate checks green natively and on
  `wasm32-wasip2` with those in it.
- **5d.** Both 5d `Board2dScene` literals pass `area_brush_size: None`; a 5d ◻️2d pane inherits the whole
  region mechanism for free the moment its board scene publishes regions in its fixture.
