# 🛠️ r5 exec `u-tools` — BIM authoring tools (utilities + retained gestures)

Scope: utilities and pointer gestures of the BIM editor (`✏️editor/🧵️gestures`, `🪛️utilities`, `🎮️commands`), each gesture emitting ONLY existing `ModelMutation`
kinds. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `E` = `S/✏️editor`.

## 1. Architecture (decisions)

| Concern | Decision |
|---|---|
| Gesture runtime | A gesture is a **retained tool statechart per window instance** (`E/🧵️gestures`). Every pointer/keyboard command is a *bounded first-step retained job* (the editor's existing `BimCommandJobFactory` / `ArtifactCommandWork`); the statechart lives in the app instance's **operation owner** (`GestureOwner: ArtifactInstanceOperationOwner`, keyed by window id, built by `BimModelApp::build_instance_operation_owner`), so state survives from one pointer command to the next, dies with the window's utility switch, with `Escape`, and with the instance (`close_step` clears it). The drawing editor's resumable `FixedOperationRegistry` machinery was deliberately NOT copied: BIM gestures are synchronous and cheap, one event = one bounded step. |
| Preview | The marks of the gesture (`Preview`, JSON) travel in the **window transient** (`BimWindowTransient.preview`, ephemeral local per window, new field + 5 schema facets); `render_with_request_context` reads it and paints it as overlay records after the plan/section linework (`plan::render_over`, `section::render_over`). A move command writes the transient lane only: proven by `a_move_shows_the_preview_in_the_window_transient_and_never_touches_the_document` (document pack byte-identical). |
| Pure core | `Tool::event(ctx, ToolEvent) -> Step { mutations, pick, arm, refused }` and `Tool::preview(ctx) -> Preview` are pure over `ToolContext` (snapshot, inference, surface, selections, authoring seed). Every proposed mutation is dry-run through the model's own `diff` (`ToolContext::accepts`); a refused proposal writes nothing and surfaces `bim.tool.rejected`. |
| Coordinates | Plan: `plan::pixel_to_model`; section: `(u, v)` along the line / up (`v = -canvas y`); world: the host's ground ray-cast point (`worldPointerDown/Move`, window engagement `session_active` while a drawing utility is armed). |
| History | One click that completes an element = one `Emit::mutations` = one history row (undo restores through the concrete inverse). A wall chain writes one `create-wall` per segment click. |

Nodes (all under `E/🧵️gestures/`, each with `🧪️tests/🔬️unit`): `📐️plane` (geometry), `🧲️snap`, `🧭️session` (vocabulary), `🧱️chain` (wall, arc wall, curtain wall, beam, railing, grid, measure),
`🧩️area` (slab polygon/rectangle/from walls, roof), `📍️point` (column, space, stair), `🪟️opening` (window, door, void + host search), `🎯️select` (pick, marquee, wall handles, opening slide, storey-height handle),
`🚚️transform` (move, rotate), `🫧️overlay` (marks → canvas records), root `🦀️.rs` (registry, owner, `run`, `canvas_pointer!` payload macro). Commands: `🎮️commands/{↔️canvas-pointer-move,⬆️canvas-pointer-up,👆️canvas-double-click,✅️canvas-commit-draft,🚪️canvas-escape,🌍️world-pointer-down,🌐️world-pointer-move,🛠️arm-utility}` + the extended `🖱️canvas-pointer-down`.

## 2. Tools → mutations

| Utility (hotkey) | Windows | Emits |
|---|---|---|
| select (V) | plan, section (world: host picking) | selection request only (`interactionSelect` effect), marquee (window = contained, crossing = touched); wall handles → `set-wall-axis`; opening drag → `move-opening` (`host` set when the wall changes — `rehost-opening` is removed upstream); section storey-top handle → `set-storey-height` |
| move / rotate | plan, world (click-click) | `move-elements`, `rotate-elements` (pivot, reference, target; shift quantises 15°) with plan-outline ghost |
| wall (W) / arc wall (A) | plan, world | `create-wall` per segment: active wall type (selected library entry else first), storey of the window, `StoreyTop{0}`; arc = third click sets the bulge through the circle; closes on its first point; Enter/double-click/Esc end the chain |
| curtain wall (U) | plan, world | `create-curtain-wall` (glass panels, metal mullions, 1.5 m grid) |
| column (C), beam (B) | plan, world | `create-column`, `create-beam` |
| slab (S), slab from walls, roof (R) | plan, world | `create-slab` (polygon / rectangle drag / closed wall loop via the `spaces` room finder), `create-roof` (gable preset, longest edge) — loops written counter-clockwise |
| window (N), door (D), opening (O) | plan, world | `create-opening` at the foot of the pointer on the hovered wall/curtain wall, kept inside the host, facing from the pointer side; warning ghost if the model would refuse |
| stair (T), railing (L), space (P), grid (G) | plan, world | `create-stair` (drag or click-click), `create-railing`, `create-space` (`Bounded{seed}`, next free number), `create-grid-line` (next free label, letters/numbers) |
| measure (M) | plan, world | nothing, ever |
| delete key | all | stays `deleteSelection` (per-kind `delete-*`): `delete-elements` has an unbounded inverse and the bounded retained route refuses it (`mutation.too-large`, 65537 staged rows > 65536), so it cannot back a key press today; it needs its own resumable job |

Snapping (`🧲️snap`): endpoint > intersection (grid×grid, grid×wall, wall×wall) > midpoint > grid line > edge > orthogonal (45° steps from the chain anchor, shift locks) > free; markers per kind in the overlay.

## 3. Registrations / shared-file edits (surgical)

* artifact root `🦀️.rs`: mount blocks for the 8 new command nodes.
* `E/🦀️.rs`: command-table rows (+23, `canvasPointerDown` moved to `[Artifact, WindowTransient]; Mutation`), bridge arms, `BimDispatchCtx` fields (`gestures`, `window_transient`, `transient_out`), `BimCommandWork` carries the owner handle and publishes the window transient, `build_instance_operation_owner`, `render_body` takes the preview, keybindings from `utilities::keybindings()` + `GESTURE_KEYBINDINGS`, pointer actions `in_palette: false` + `Input` audience, fault notices en/de.
* `🪛️utilities`: 19 rows (icon, en/de label, group, windows, hotkey, arming command) replacing the single select row; its test updated.
* `🫧️transient`: `preview: String` (+ `🔗️.graphql`, `🔣️.json`, `🛰️.proto`, `🟦️.ts`, test).
* `🎛️chrome`: world `session_active` while a drawing utility is armed. `🧰️kit`: `IdMint::seeded`.
* `🗺️plan`/`📐️section`: `render_over` (existing `render` unchanged). `🧪️tests` of the editor: `every_command` extended, `dispatch_in` merges the settled effects.
* Trivial blockers fixed in peers' files (only to compile the lib tests; u-editor's kit `window_config!` call into the shared `bim_window_config!` was left to u-editor): `🧬️schema/💡️inferences/🧪️tests/🔬️unit` include path `🏢️storey-levels` → `🪜️storey-levels`; `✏️editor/🧪️tests` `zz_debug_close` `&mut app` → `&mut *app`.

## 4. Tests

Commands (all through `🚦️gate.sh u-tools`, from `/c/git/semio`):

* `cargo check -p semio-s-artifact-bim-model --lib --target wasm32-wasip2` -> Finished (green).
* `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib bim::` -> 245 passed, 15 failed. Of the 15: **9 are peers' in-progress work**
  (3 window-config `the_inverse_sums_to_the_negative_diff...` of the shared `bim_window_config!` migration, `zz_debug_close*` x3, `two_plan_windows...`,
  `the_world_window_renders...`, `the_schedule_lists...`: all stall in the window-config store close, `[DEBUG] window-config registry ... stalled items=1 bytes=4096`),
  and **6 are my mounted-app tests (`gestures::app_tests::*`) which pass every body assertion and fail only in the fixture's `Drop` with the SAME close stall**
  (`registered fixture did not reach its exact terminal-empty witness` after 30 s, same `BimPlanWindowConfig` store cursor): re-run once u-editor's close fix lands.
  The instance owner is not the cause: for every peer window test `GestureOwner` has no session and behaves exactly like the framework's empty owner.
* `gestures` filter: 65/65 pure tests green (plus the node tests of the 8 command nodes and the utilities registry in `bim::`) (every tool, snapping, plane, overlay, registry, hotkeys, owner close, preview text, parametric e2e, oracle replay).
* Third-party oracle: `.venv/Scripts/python.exe -I S/🧪️tests/🛠️gestures-bim-1/🐍️.py check S/🧫️fixtures/🛠️gestures/🔣️.json` -> `ok: 23 cases agree with numpy 2.4.3 and shapely`
  (numpy circumcircle for arc bulges and angles; shapely/GEOS `project`, `box`, `orient` for foot offsets, rectangles, orientation; the fixture was WRITTEN by the oracle,
  the Rust subject replays the same JSON through its tools in `the_cases_the_third_party_oracle_wrote_replay_through_the_tools_to_the_same_numbers`).

What the tests pin (exact emitted mutations, per tool): wall chain (types, storey, `StoreyTop{0}`, closing snap), orthogonal/endpoint snapping, arc bulge 0.5 for a 4 m chord with a 1 m sagitta,
escape/finish, library type choice and `bim.tool.type-missing`, beam, curtain wall (glass/metal), grid labels A, B, 1, railing on double-click and escape, measure writes nothing; slab polygon
(CW input written CCW), close on first corner, rectangle drag, roof gable preset, slab from walls (area 7.7 x 5.7); column, space (`Bounded{seed}`, numbers 1, 2, 105), stair drag and click-click;
window/door/void at the foot of the pointer, facing from the side, clamped to 0.6, overlap -> `bim.tool.rejected` + warning ghost; select picks/merges/marquee (contained vs touching), end-handle
`set-wall-axis`, midpoint handle bulge 0.25 and back to a line, opening slide `move-opening` and onto another wall (`host: Some`), refused overlap, section storey-top handle -> `set-storey-height`
(snapped to 5 cm, floor 0.5 m); move/rotate (vector, click-click, shift lock, 15 degree quantising); every move/hover leaves the document byte-identical and every preview round-trips as transient text.

Parametric end-to-end (`drawing_four_walls_placing_a_window_and_dragging_the_storey_top_re_infers...`): five clicks draw four walls, one click places a window, a section drag of the storey top writes exactly one
`set-storey-height{st-ground, 3.5}`; afterwards `wall_layout[*].height == 3.5`, `opening_frames[window].host_height == 3.5` and still valid, the storey above is lifted, and the walls in the snapshot are untouched.


## 5. Open issues / follow-ups

* The 3D world window shows no gesture preview (its scene has no overlay seam in the editor yet); clicking works through the ground ray (`worldPointerDown/Move`).
* `slab-walls` resolves the room only on the click (region booleans per mouse move would be too expensive); there is no hover highlight of the loop.
* Tool options (arc/straight toggle, roof preset, wall location line) are separate utilities / fixed defaults; a measures-based option row is the follow-up.
* Hotkeys are bound as app keybindings; whether the shell routes bare letters to the owning window while a text field has focus is a shell concern (not verified in a browser).
* `GestureOwner` keeps a session per window until the instance closes (windows closed earlier leave a small idle session).
