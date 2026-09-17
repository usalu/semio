# 📓️ Wave 2E — puzzle ◻️2d transform gumball (rotate ring) + `Board2dHost` probe vitals

Slice 2E of the 2026-09-17 parity fleet. Scope from `📋️master-plan.md` row 2E, `📓️E8` §OWED 1+3,
`📓️E1` rows 2–5 / 42 / §C row 2.

---

## 1. What landed

### 1.1 Board engine — generic graph core (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`)

New `#region 🔖️TransformGumball` (inserted between `SelectionMarquee` and `Engine`, `🦀️.rs:575-651`):

| symbol | what |
|---|---|
| `TransformGumballFlags { move_enabled, rotate_enabled }` | which handles the select utility composes; `Default` = both on. **No scale field** — deliberate, same law as puzzle3d (`📓️E1` row 42) |
| `transform_pivot_of(&[Point]) -> Option<Point>` | arithmetic mean of the selected node centres — byte-identical definition to the guest's `puzzle2d_selection_centroid`, so preview and commit agree |
| `rotate_point_about(pivot, point, radians)` | counter-clockwise world rotation |
| `transform_ring_radius_world(pivot, corners, zoom)` | farthest corner + a constant **screen** gap, floored at a grabbable minimum |
| `transform_ring_hit(pivot, radius, zoom, point)` | band test (not a disc) |
| `transform_ring_angle_delta(pivot, from, to)` | normalised to `(-π, π]` so a drag across the seam never jumps a turn |
| `snap_transform_angle(radians, snap)` | quantises to the 15° step under the grid-snap modifier |
| consts `TRANSFORM_RING_GAP_PX`, `TRANSFORM_RING_MIN_RADIUS_PX`, `TRANSFORM_RING_HIT_TOLERANCE_PX`, `TRANSFORM_ROTATE_SNAP_RADIANS` | re-exported from the styling tokens |

Re-exported through `🎲️board/🔌️ports/➡️directed/🦀️.rs:1192-1196` so the wgpu port can name them.

**Styling tokens are generated** — the four new board metrics were added to the SOURCE
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:262-266` *and* hand-applied to every generated twin so
the tree stays consistent until the generator next runs:
`🔤️tokens/🦀️.rs`, `🔤️tokens/🐍️.py`, `🤖️generated/🔤️tokens/🟦️.ts` (2 sites), `🌓️theme/🔣️.json`.
Values: `transformRingGapPx 26`, `transformRingMinRadiusPx 44`, `transformRingHitTolerancePx 10`,
`transformRotateSnapDegrees 15`.

### 1.2 Board engine — wgpu session / puzzle-2d port (`…/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`)

- **`BoardEventKind::NodeRotate` (`"nodeRotate"`) and `BoardEventKind::TransformPreview`
  (`"transformPreview"`)** added to the enum + `name()` (`🦀️.rs:1551-1596`).
- `BoardOwnedEvent::node_rotate(ids, radians, pivot)` — payload
  `{"ids":[…],"radians":f64,"pivot":{"x":f64,"y":f64}}`, capped at `BOARD_POINTER_ITEM_CAPACITY`,
  refuses an empty id list (`🦀️.rs:~1937`).
- `node_drag_end` refactored into a shared `moves_row(kind, moves)`; `transform_preview(moves)` reuses
  it, so a preview frame is the same `{"moves":[{id,x,y}…]}` shape a drag end already publishes.
- `BoardTransformDrag { pivot, grab, radius_world, radians, start_positions, start_handle_angles }` —
  a grab-time snapshot; every frame re-derives from it, so the preview never accumulates float drift
  and a cancel restores the exact pre-gesture geometry.
- `BoardHost` fields `transform_flags: TransformGumballFlags`, `transform_drag: Option<BoardTransformDrag>`
  (+ `Default` init, + the `BoardHostClosePhase::Previews` retirement arm that pops the two owned maps
  one entry per step, + `nonopaque_terminal_is_empty()` witness).
- Public surface: `set_transform_flags(move, rotate)`, `transform_flags()`,
  `transform_gumball_json()`, `interaction_json()`.
- Gesture wiring:
  - `pointer_down_screen` — `try_begin_transform_drag_at(world)` is asked **before**
    `try_begin_bounded_selection_drag_at` and before the node/handle/edge hit test, so the ring
    outranks whatever sits under it and never re-picks it.
  - `pointer_move_screen` — `update_transform_drag(world, self.grid_snap_enabled)` returns early,
    rotating node centres **and** handle angles and publishing one transient `transformPreview` row.
    A refused reservation drops the frame outright (no overflow claim, no fault).
  - `pointer_up_screen` — `commit_transform_drag()` publishes exactly ONE `nodeRotate`; a zero-angle
    release commits nothing.
  - `pointer_leave_screen` and `cancel_area_select()` (Escape) both `cancel_transform_drag()`.
  - `set_transform_flags(_, false)` cancels a live drag.
- `defers_descriptor_sync_from_js()` now also true while the ring drag is live, so a scene echo
  cannot clobber the preview mid-gesture.
- Paint: `append_transform_gumball_paint` appended to both the world-space and screen-space arms of
  `append_cached_world_content` — dashed ring when idle, solid + pivot dot + grab spoke while
  dragging; while dragging it paints the **grab-time** ring so the band does not breathe under the
  cursor for a non-circular selection.

### 1.3 `Board2dScene` contract (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/`)

New optional spine field `transform_flags` / `transformFlags` — a JSON string
`{"move":bool,"rotate":bool}`; absent leaves the engine default (both on).
Touched: `🎬️scenes/🦀️.rs` (struct :1950, `scene_pack_wire!` :1991, `base()` :2170, `ToValue` :2199,
`FromValue` :2230), `🎬️scene/🟦️.ts` (`Board2dScene.transformFlags?`),
`🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` (`spineFields`).
Deliberately **not** added to `🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json`: that catalog
already omits the two other optional carriers (`toolRunTrace`, `lanes`), so an optional field is
consistent with how the last two were handled.

Desktop path wired too: `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `BoardSyncCache.transform_flags`,
`sync_board_engine` arm, `board2d_transform_flags_from_json` (malformed ⇒ both on, never a silent
disarm), plus the `close_board_sync`/`board_sync_terminal` entries.

### 1.4 `Board2dHost` (`…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`)

- `transformPreview` added to `PUZZLE2D_TRANSIENT_EVENT_NAMES`; `nodeRotate` added to
  `PUZZLE2D_FLUSH_NOW_EVENT_NAMES`.
- `collectPuzzle2dLiveMirrorMutations` treats `transformPreview` exactly like `nodeDragEnd`, so the
  **existing `BoardPeerScope` peer bus** mirrors the live rotation into the sibling 2d panes with no
  new mechanism and no guest round trip.
- New effect syncs `scene.transformFlags` → `session.setTransformFlags(move, rotate)`.
- New exported helpers: `parseBoard2dTransformFlags`, `board2dStatusJson`, `type Board2dStatus`
  (re-exported from `🎯️targets/⚛️react/🟦️.tsx`).
- `recordFixtureVerdict` centralises the fixture apply verdict (parsed / chars / refusal reason /
  guest revision) and keeps writing the existing `data-board-fixture-parsed`.
- **Vitals** — every pre-existing `data-board-*` name is unchanged. New:

| attribute | value | published by |
|---|---|---|
| `data-window-instance-id` | `WindowInstanceIdContext` (3d parity) | React render |
| `data-board-guest-selection-json` | `scene.selectionJson` — guest-confirmed | React render |
| `data-board-selection-json` | **now the local/optimistic echo**, `localSelectionJson ?? scene.selectionJson` | React render |
| `data-board-status-json` | `{fixtureParsed,fixtureChars,refusalReason,pendingEvents,guestRevision}` | render + imperative |
| `data-board-interaction-json` | `{mode,utility,hoveredId,selectionCount,preselectCount,revision,deferringDescriptorSync}` from `BoardHost::interaction_json()` | imperative only |
| `data-board-transform-json` | `{move,rotate,ringVisible,dragging,radians,pivot,radius}` from `BoardHost::transform_gumball_json()` | imperative only |

`data-board-interaction-json` / `data-board-transform-json` are written ONLY from `publishBoardVitals`
(called from `scheduleRender`'s rAF and from a post-render effect), never from JSX, so a React
re-render cannot stamp a stale frame over the live one — the same pattern `data-board-fixture-parsed`
already used. ⚠️ **Battery note:** `data-board-selection-json` changed meaning from guest-confirmed to
local/optimistic (3d's `data-selection-json` semantics). Probes that want the committed set must read
`data-board-guest-selection-json`. Before any local gesture the two are identical.

### 1.5 Wasm bridge (`EDITOR2/🌉️wasm/🦀️.rs`) + session type

`setTransformFlags(move, rotate)`, `transformGumballJson()`, `interactionJson()` added to
`BoardSession`, and to `Board2dWasmSession` in `…/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` (all optional).

### 1.6 Guest — EDITOR2

- **New verb `setTransformGumballFlag {flag: move|rotate, pressed?}`** —
  `🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs`. Omitting `pressed` toggles (matching the framework
  toggle measure's echo); an unknown flag is a no-op, so "scale" stays deliberately absent.
- **`applyBoardEvents` handles `nodeRotate`** (`🎮️commands/🎲️apply-board-events/🦀️.rs:116-129`) —
  maps it onto the existing `puzzle2d_transform_selection(.., Puzzle2dTransform::Rotate{radians})`, i.e.
  the same reducer `rotateSelection` uses, as ONE history edit. Zero-angle / id-less rows commit
  nothing. Its `UiDirtyScope` arm is `window_bodies + panel_properties` (no catalogue/engagement churn).
- Runtime config `transform_move` / `transform_rotate` (`🎚️config/🦀️.rs`, default true).
- Window config `transform_move` / `transform_rotate` + the four schema twins
  (`🪟️window/🧬️schema/🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto` tags 13/14) + the
  `runtime()`/`split()` mapping, so the flags persist on the **WindowConfig** lane per pane.
- New utility-options measure `🎭️modes/✏️edit/☑️options/🔄️transform/🦀️.rs`, tagged
  `active_utility_id = select`, two toggles (`move` / `rotate-cw` icons) → `setTransformGumballFlag`.
  Added to `👁️overview::window_measures`.
- `puzzle2d_board_scene` publishes `transform_flags: {"move":…,"rotate":…}`.
- `apply_host_events`-side host sync: `host.set_transform_flags(runtime.transform_move, runtime.transform_rotate)`.
- **`transformBegin`/`transformEnd` deliberately NOT added.** 3d needs them because its gumball drag
  is a React/host session; the 2d ring lives entirely inside the wasm board engine, publishes one
  event on release and nothing during the drag, so empty bracket verbs would be dead registry rows.
  Flagged here rather than invented.

### 1.7 Registries touched (all in `EDITOR2/🦀️.rs` unless noted)

| registry | line |
|---|---|
| `use …commands::{…, set_transform_gumball_flag, …}` | :16 |
| `puzzle2d_command_variants!` → `SetTransformGumballFlag` | :1677 |
| `PUZZLE2D_RETAINED_TOOL_IDS` | :1944 |
| `PUZZLE2D_GENERIC_TOOL_IDS` | :1988 |
| `PUBLICATION_CONTRACTS` → `[WindowConfig]` | :2110 |
| `dispatch_puzzle2d_action` match arm | :2727 |
| `bounded_first_step_tool_proofs!` tools list | :4670 |
| `.action_with(puzzle2d_internal_action(…, ActionKind::View))` | :5195 |
| `.action_interactive_job(…, InteractiveJobClassification::Migrated)` | :5279 |
| terminology EN+DE `transform` / `move_flag` / `rotate_flag` | `🗣️terminology/🦀️.rs` |
| module decls `set_transform_gumball_flag`, `options::transform` | `🗿️artifacts/◻️2d/🦀️.rs:1793,1835` |
| fixture `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` `toolIds` | ✅ |
| fixture `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` Puzzle2dPlayApp window-config routes | ✅ |

---

## 2. Laws added

**Board core** — `♾️infinite/🎲️board/🧪️tests/🔬️unit/🦀️.rs` (6): `transform_pivot_is_the_selection_centroid`,
`rotate_point_about_turns_counter_clockwise`, `transform_ring_radius_keeps_a_constant_screen_gap`,
`transform_ring_hit_is_a_band_around_the_radius`, `transform_ring_angle_delta_takes_the_short_way_round`,
`snap_transform_angle_quantizes_only_under_the_snap_modifier`.

**Board host** — `…/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`, new `#region 🕹️TransformGumball` (8):
`a_rotate_ring_drag_commits_exactly_one_node_rotate_event`,
`the_rotate_preview_turns_positions_and_handle_angles_together`,
`the_rotate_ring_snaps_under_the_grid_snap_modifier`,
`locked_members_hold_the_pivot_but_never_turn`,
`the_rotate_ring_outranks_the_nodes_and_handles_under_it`,
`a_rotate_gesture_leaves_the_event_credits_untouched`,
`escape_cancels_the_rotate_ring_and_restores_the_geometry`,
`the_rotate_ring_is_armed_only_when_the_flags_allow_it`.

**React** — `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (4): the rotate commit flushes at once while
preview frames never reach the guest; `transformPreview` mirrors like a drag's final moves;
`parseBoard2dTransformFlags` never disarms on malformed input; `board2dStatusJson` shape.

**Guest EDITOR2** — `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, new `#region 🕹️TransformGumball` (3):
`set_transform_gumball_flag_composes_the_handles_without_touching_the_document`,
`a_node_rotate_board_event_commits_one_rotate_selection_edit`,
`rotating_a_two_node_selection_orbits_both_about_the_centroid`.

---

## 3. Commands run — verdicts

| command | verdict |
|---|---|
| `CARGO_INCREMENTAL=0 cargo check -p semio-framework-os-infinite --message-format=short` | ✅ **exit 0**, `Finished dev profile in 1m 07s`, 22 warning lines / 5 crate-level warnings, **0 errors**. One warning was mine (`field radius_world is never read`) and is now consumed by the paint + vitals path. → `🗑️generated/2E/infinite-check.txt` |
| `CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib -- transform` | ✅ **6 passed, 0 failed** (5 new core laws + 1 pre-existing 3d surface-verb law). → `infinite-transform-laws.txt` |
| `CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib -- rotate` | ✅ **8 passed, 0 failed** (7 new board-host laws + `rotate_point_about_turns_counter_clockwise`). → `infinite-rotate-laws.txt` |
| `CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib -- locked_members` | ✅ **1 passed, 0 failed**. → `infinite-locked-law.txt` |
| `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | ❌ **NEVER STARTED** — see §3.1 |

**All 14 new board-engine laws are green on a real native run.** The engine half of this slice —
the part the coordinator cannot exercise because wasm-pack is not mine to run — is proven.

### 3.1 2d crate native check — NOT OBTAINED, honest verdict

The addendum-19:55 gate (`until [ "$(pgrep -f 'cargo (check|test).*semio-s-artifact-puzzle-[2]d' | wc -l)" -lt 3 ]`)
**never opened**: 8 sibling `cargo check -p semio-s-artifact-puzzle-2d` invocations were alive
continuously (26 cargo processes overall), across two attempts and ~5 minutes of bounded polling.
`🗑️generated/2E/puzzle2d-native-check.txt` was therefore never created — the command did not run, so
there is **no** 2d verdict of any kind. Per addendum rule 4 the slice stops here rather than loop.

**Therefore the guest half of 2E (§1.3 wgpu sync, §1.5 wasm bridge, §1.6 EDITOR2, §2 guest/React laws)
is WRITTEN BUT NEVER COMPILED.** Treat every EDITOR2 claim in §1.6/§1.7 as source-verified only
(every registry row was grepped back after writing, and every helper signature — `puzzle2d_action`,
`WindowMeasure::Toggle`/`Group` field lists, `ctx.host.borrow_mut()`, `fixture_nodes -> &[Value]`,
`LabelText::into` — was checked against its sibling call site by hand), not compiler-verified.

Files an integrator should grep the first 2d error list for, since only these are mine:
- `…/◻️2d/🦀️.rs` (module decls for `set_transform_gumball_flag`, `options::transform`)
- `…/✏️editor/🦀️.rs` (9 registry rows + `host.set_transform_flags(…)` in the host sync)
- `…/✏️editor/🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs` (new)
- `…/✏️editor/🎮️commands/🎲️apply-board-events/🦀️.rs` (`nodeRotate` arm + its ui-scope arm)
- `…/✏️editor/🎭️modes/✏️edit/☑️options/🔄️transform/🦀️.rs` (new)
- `…/✏️editor/🎭️modes/✏️edit/🦀️.rs` (`transform_flags:` in `puzzle2d_board_scene`)
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️.rs` (`options::transform::measure`)
- `…/✏️editor/🎚️config/🦀️.rs`, `…/✏️editor/🪟️window/🦀️.rs` (+ its 4 schema twins)
- `…/✏️editor/🗣️terminology/🦀️.rs` (`transform` / `move_flag` / `rotate_flag`)
- `…/✏️editor/🌉️wasm/🦀️.rs` (3 `#[wasm_bindgen]` methods — invisible to a native check)
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (3 laws + 2 helpers)
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` (`transform_flags`, 5 sites)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`

Everything under `♾️infinite/🎲️board/**` and `🎨️styling/**` IS compiler-verified and law-green (rows 1–4
of the table above).

---

## 4. NOT verified / owed to integration

0. **The whole 2d crate — native AND `wasm32-wasip2`.** See §3.1: the build gate never opened, so no
   2d check ran at all. This is the single biggest gap of the slice.
1. **`cargo check --target wasm32-wasip2 -p semio-s-artifact-puzzle-2d`** — not run (addendum 19:55
   rule 3: only after a green native check). Nothing in this slice is `cfg(wasm32)`-gated except the
   three `🌉️wasm/🦀️.rs` bridge methods, which are `wasm32 && !p2` and therefore invisible to wasip2
   anyway; the added wasip2 risk over the native check is nil for 2E.
2. **`cargo check -p semio-s-plugin-puzzle --target wasm32-unknown-unknown --no-default-features`** —
   the exact form `wasm-pack` wraps (read from `📦️packages/🦀️rust/📜️script.ts` →
   `runWasmPackWebBuild{ noDefaultFeatures: true }`, `--target web`). Not run. This is the ONLY check
   that compiles `🌉️wasm/🦀️.rs`'s three new `#[wasm_bindgen]` methods — **integration must run it**
   before trusting `session.setTransformFlags` / `transformGumballJson` / `interactionJson` exist on
   the JS side.
3. **`bun test` of `🔬️engine-contract/🟦️.ts`** — not runnable without nx from this slice; the four
   React laws are written but unexecuted.
4. **`bun ./📜️script.ts publication-authority-audit Puzzle2dPlayApp`** — not run.
5. **No visual verification** of the ring paint (colours, stroke weights, spoke) — engine builds and
   the geometry is law-covered, but nobody has looked at a pixel.
6. **Peer half-landed breakage seen while working, NOT mine:** `Board2dScene::base()` in
   `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:2150` was missing `domain_id` after a sibling
   added that field to the struct. Left alone per rule 3 / addendum rule 2.

---

## 5. Hand-offs

- **Slice 2G / battery:** two NEW board event names to expect — `nodeRotate` (committed, reaches the
  guest) and `transformPreview` (transient, never reaches the guest). Six new DOM attributes, listed
  in §1.4. **`data-board-selection-json` now carries the LOCAL selection**; assert the committed one
  against `data-board-guest-selection-json`.
- **Slices 2B (suggestions popup) and 2F (target regions):** `Board2dHost/🟦️.tsx` was edited with
  small anchored edits only — the new `publishBoardVitals` helper is the place to add any further
  imperative vital, and `Board2dScene` gained one optional field so their own scene fields are
  unaffected.
- **Slice 2C:** the new `☑️options/🔄️transform` measure is registered in
  `👁️overview::window_measures` between `options::select` and `options::brush`.
- **Slice 2A:** `cancel_area_select()` now claims a live ring drag first and returns `true` for it;
  the Escape path in `Board2dHost` needs no change but the semantics widened.
- **5d:** every gumball symbol is in the generic board core / directed-normal port, so a 5d ◻️2d pane
  inherits the ring for free once it publishes `transformFlags` in its own board scene.
