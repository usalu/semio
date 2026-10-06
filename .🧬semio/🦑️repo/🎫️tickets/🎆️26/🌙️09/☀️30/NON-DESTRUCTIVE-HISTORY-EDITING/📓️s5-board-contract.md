# 📓️ Board host contract — field list (design §22.31, the concrete form of §22.9)

Author: S5-PUZZLE, 2026-10-05 10:15. Status: FIELD LIST, no code. Implementers: S5-PUZZLE (schema, generated tables, board engine,
puzzle side), S5-UI (React hosts), S5-WGPU (wgpu hosts), S5-RUNTIME (runtime literal), S5-GATES (gate), S5-TOOLS (block 2d producer).

Paths: `OSM` = `🧰️framework/🛍️products/💻️os/🔨️modules`; `B2H` = `OSM/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost`;
`EC` = `OSM/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas`; `BOARD` = `OSM/♾️infinite/🎲️board`;
`P2` / `P5` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any`.

Rule of the contract: the framework hosts name no artifact and no artifact vocabulary. Every fact a host needs is either a
row of the ONE board event schema (generated into a Rust and a TypeScript table) or a value the app's scene / descriptor declares.

## 1. Board event kinds and their delivery

### 1.1 Where the declaration lives

- **Schema of record (new):** `BOARD/🧬️schema/🔣️board-event/🔣️.json` — owner = the board engine (the publisher of the rows).
  `definitions.Kind` is a `oneOf` of one `{ "const": "<name>", "description": …, "x-semio-board-event": Delivery }` per kind;
  `definitions.Row` = `{ name: Kind, payload: object }`; `definitions.GestureRecord` (+ `DragRecord`, `RotateRecord`, `ScaleRecord`,
  `ProximityPair`), `definitions.Camera` move here from the coalescing corpus schema, which `$ref`s them afterwards
  (`B2H/🧬️schema/🔣️board-event-coalescing/🔣️.json` keeps only `Case` and `expect`).
- **Annotation `x-semio-board-event` (type `Delivery`)** — registered in the strict vocabulary fixture beside `x-semio-ui`:

  | field | type | meaning |
  |---|---|---|
  | `delivery` | `"transient"` \| `"flushNow"` \| `"coalesced"` | `transient`: never enters a dispatched batch. `flushNow`: kept in order and the buffer flushes now. `coalesced`: kept, never flushes by itself (leaves with the next flush) |
  | `coalesce` | `"latest"` \| `"append"` (required unless `transient`) | repeated rows of the kind in one buffer: only the latest survives, or all stay in order |
  | `lane` | `"board"` \| `"view"` \| `"hover"` \| `"mirror"` \| `"local"` | where the row goes. `board`: a row of the app's board-events verb. `view`: the app's view verb (the camera; never a history row). `hover`: the framework hover lane (`interactionHover`). `mirror`: the host's live peer-pane mirror only. `local`: engine-local chrome, nowhere |
  | `awaits` | `{ "key": string, "kind": Kind }` (optional) | a `flushNow` row that carries `payload[key]` flushes only when the buffer holds a row of `kind` with the same `payload[key]` |
  | `mirror` | `"positions"` \| `"selection"` \| `"preselect"` (optional) | what the host's peer-pane mirror reads from the row (a `board` row may also feed the mirror) |

- **Generated tables (never hand-edited, `check-generated` law):** Rust `BOARD/🤖️generated/🔣️board-event/🦀️.rs`
  (`BoardEventKind` with `ALL`, `name`, `parse`, and `const fn delivery(self) -> BoardEventDelivery` returning
  `{ delivery, coalesce, lane, awaits, mirror }`), TypeScript `BOARD/🤖️generated/🔣️board-event/🟦️.ts`
  (`BOARD_EVENT_KINDS`, `BoardEventKind`, `BOARD_EVENT_DELIVERY: Readonly<Record<BoardEventKind, BoardEventDelivery>>`).
  Generator: one `generate board-events` command in the board module's `📜️script.ts`. The hand-written
  `BoardEventKind` enum in `BOARD/🔌️ports/➡️directed/➕️normal/🦀️.rs:1598-1666` is replaced by the generated one.
- **Consumers read the table, never a name:** React `coalesceBoard2dEvents` (`B2H/🟦️.tsx`), wgpu `coalesce_owned_board_events`
  (`EC/🎯️targets/🧊️wgpu/🦀️.rs`). Deleted: `PUZZLE2D_TRANSIENT_EVENT_NAMES`, `PUZZLE2D_FLUSH_NOW_EVENT_NAMES` (`B2H/🟦️.tsx:324-325`),
  `board_event_transient`, `board_event_flush_now` (`🧊️wgpu/🦀️.rs` ≈ :5127 / :5133), the wgpu law
  `transient_and_flush_now_tables_match_the_react_sets` (replaced by: the generated tables equal the schema).
- **Corpus:** `B2H/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` stays the behavioural proof for both hosts (it already states
  `expect.camera` beside `expect.events`); it gains one case per kind that has none today (`hover`, `brushPreview`,
  `linkCompatibleNodes`, `linkTargetRing`, `proximityConnect` alone) so every table row is exercised.

### 1.2 The table (all 20 kinds the engine publishes today)

"React" / "wgpu" = what the two hosts encode by name today. Consumers: which guest reads the row from `applyBoardEvents`.

| kind | React today | wgpu today | `delivery` | `coalesce` | `lane` | `awaits` | `mirror` | guest consumers |
|---|---|---|---|---|---|---|---|---|
| `camera` | special case (latest; view verb since F7) | special case (latest) | `coalesced` | `latest` | `view` | — | — | none (view verb `setCamera`) |
| `nodeMove` | TRANSIENT | transient | `transient` | — | `mirror` | — | `positions` | — |
| `transformPreview` | TRANSIENT | transient | `transient` | — | `mirror` | — | `positions` | — |
| `gesture` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d (drag, rotate, scale), 5d (drag) |
| `select` | FLUSH_NOW unless its gesture is open | same | `flushNow` | `append` | `board` | `{ key: "gestureId", kind: "gesture" }` | `selection` | 2d, 5d |
| `preselect` | TRANSIENT | transient | `transient` | — | `mirror` | — | `preselect` | — |
| `preselectCancel` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | `selection` | 2d (ignored row), 5d (ignored) |
| `hover` | TRANSIENT (own lane) | transient (own lane) | `transient` | — | `hover` | — | — | — |
| `brushPreview` | TRANSIENT | transient | `transient` | — | `local` | — | — | — |
| `brushCandidates` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d |
| `brushPlace` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d, 5d |
| `linkCompatibleNodes` | TRANSIENT | transient | `transient` | — | `local` | — | — | — |
| `linkTargetRing` | TRANSIENT | transient | `transient` | — | `local` | — | — | — |
| `edgeCreate` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d, 5d |
| `edgeDelete` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d, 5d |
| `nodeDelete` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d, 5d |
| `indirectConnect` | in neither set (kept, waits) | in neither | `coalesced` | `append` | `board` | — | — | none today |
| `proximityConnect` | in neither set (kept, waits) | in neither | `coalesced` | `append` | `board` | — | — | none today |
| `regionCreate` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d |
| `regionResize` | FLUSH_NOW | flush_now | `flushNow` | `append` | `board` | — | — | 2d |

Block 2d consumes none today; as the second producer (§22.31 c) it reads `gesture` rows from the same verb and table.

### 1.3 The verbs a board host dispatches (declared by the scene, not spelled by the host)

Today both hosts spell `"applyBoardEvents"`, `"setCamera"`, `"addNode"`, `"interactionHover"`. `interactionHover` is a framework verb
and stays. The other three become fields of `Board2dScene` (`FW/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:2523`, TS twin, UI contract schema):

| new `Board2dScene` field | type | today's literal | meaning |
|---|---|---|---|
| `eventsAction` | `string` (action id) | `"applyBoardEvents"` | receives `{ eventsJson: string }` = the JSON array of `lane: board` rows |
| `cameraAction` | `string` (action id) | `"setCamera"` | receives `{ camera: { x: number, y: number, zoom: number } }`; a `view` verb, never a history row |
| `dropPayload` | `DropPayload` \| absent | (hard-coded, § 2) | what a catalogue drop onto the board is and does |

## 2. Catalogue drop payload — `dropPayload { mime, schema, … }`

Replaces: React `Puzzle2dFixtureDropPayload` (`B2H/🟦️.tsx:47`), `parsePuzzle2dCatalogueDragPayload` (`:162`),
`puzzle2dFixtureDropPreviewJson` (`:448`), the hard-coded `dispatch("addNode", {…})` (`:1620`); wgpu
`puzzle3d_catalogue_drag_payload_json` / `puzzle3d_catalogue_drag_payload` (`EC/🎯️targets/🧊️wgpu/🦀️.rs:3606, :3614`) and their React
twin `parsePuzzle3dCatalogueDragPayload` (World3dHost). One generic parser per language (`parseCatalogueDragPayload` /
`catalogue_drag_payload`) validates the dragged JSON against the declared schema and maps it.

`DropPayload` (a contract value on the scene node; the same type on `Board2dScene` and `World3dScene`):

| field | type | meaning |
|---|---|---|
| `mime` | `string` | the drag mime the surface accepts (today the one constant `application/x-semio-catalogue-item`, `CATALOGUE_DRAG_MIME`) |
| `schema` | `string` (schema `$id` of a schema the app's descriptor publishes) | the JSON Schema of the dragged payload; a payload that fails it is not a drop |
| `action` | `string` (action id) | the verb a drop dispatches (2d: `addNode`) |
| `position` | `{ x: string, y: string, z?: string }` | the verb argument names that receive the drop's world position |
| `args` | `Record<string, string>` (verb argument name → RFC 6901 pointer into the payload) | how payload members become verb arguments; an absent member omits its argument |
| `ghost` | `Record<string, string>` \| absent (ghost descriptor member → RFC 6901 pointer) | what the host paints while dragging; member names are the surface's own ghost vocabulary (§ 2.2) |

### 2.1 The two payloads that exist today, as declared values

| app | payload schema members (name: type) | `action` | `position` | `args` |
|---|---|---|---|---|
| puzzle 2d board | `kindId: string` (required), `catalogSlice: string` (default `"nodes"`), `shape?: "circle" \| "rectangle"`, `radius?: number > 0`, `width?: number > 0`, `height?: number > 0`, `iconKind?: string` | `addNode` | `{ x: "x", y: "y" }` | `kind ← /kindId`, `shape ← /shape`, `radius ← /radius`, `width ← /width`, `height ← /height`, `iconKind ← /iconKind` |
| puzzle 3d world | `objectKind: string` (required, non-empty), `meshUrl?: string` (non-empty) | the app's existing drop verb | `{ x, y, z }` as today | `objectKind ← /objectKind`, `meshUrl ← /meshUrl` |

The payload schemas are app-owned: `P2/✏️editor/🧬️schema/🔣️catalogue-drop/🔣️.json` and the puzzle 3d twin; both are published in
the descriptor so a host resolves `dropPayload.schema` without reading plugin files.

### 2.2 Board ghost vocabulary (surface-owned, artifact-neutral)

`ghost` members a 2d board understands (today's preview JSON, renamed off puzzle words): `kind: string` (was `nodeKind`),
`shape: "circle" | "rectangle"`, `radius: number`, `width: number`, `height: number`, `iconKind: string`; `x`, `y` are the host's
(world position). The fallbacks stay host constants (`BOARD_CATALOGUE_DROP_FALLBACK_RADIUS`, `…_EXTENT`).

## 3. Selection domain and granularity classifier — declared, never `"vortex"`

Today: the runtime spells `"vortex"` and the default granularity `"object"` 22 times (`OSM/🔌️plugin/🦀️.rs:29694-29714,
29949-29953, 30061-30063, 30127-30132`: where leftover selection ids go when their own domain is gone); React `board2dGranularityById`
(`B2H/🟦️.tsx:203`) hard-codes "nested `handles[]` id → `handle`, `edges[]` id → `edge`, else `node`"; the wgpu twin
`board2d_granularity_by_id` (`🧊️wgpu/🦀️.rs` ≈ :5159) the same; the Interpreter keys a context-menu label by `vortex`
(`🗣️Interpreter/🟦️.tsx:821`) and switches an icon classifier on `classifierKind === "puzzle2d"` (`:1210`, `:1795`).

### 3.1 Descriptor: declared selection domains

The app descriptor already lists interaction domains by id (`.interaction_domain(controller, domain)`). Each gains:

| field | type | meaning |
|---|---|---|
| `id` | `string` | the domain id (puzzle: `"vortex"`; an app's own word, never read by name in the framework) |
| `primary` | `boolean` (exactly one per app) | the domain that receives leftover selection ids — replaces the runtime literal |
| `defaultGranularity` | `string` | the granularity a selection of this domain has when none is active — replaces the literal `"object"` |
| `granularities` | `string[]` (ordered) | the granularities the domain offers |
| `label` | `LocalizedLabel` | the domain's name in menus — replaces `ui.surfaceContextMenu.vortex` |

Runtime (S5-RUNTIME): `state.selection.get("vortex")` → `state.selection.get(primary.id)`, `"object"` → `primary.defaultGranularity`.

### 3.2 Scene: the board's granularity classifier

`Board2dScene` already carries `domainId`. It gains `granularityClassifier: GranularityRule[]`, evaluated in order over the fixture
the scene paints; the first rule that holds an id names its granularity, an id no rule holds takes `fallback`:

| field | type | meaning |
|---|---|---|
| `granularityClassifier[].granularity` | `string` | the granularity ids under this rule report |
| `granularityClassifier[].ids` | `string` (RFC 6901 pointer template with `*` for "every item") | where the ids are in the fixture |
| `granularityFallback` | `string` | the granularity of an id the fixture does not carry yet |

Puzzle 2d declares: `[{ granularity: "node", ids: "/nodes/*/id" }, { granularity: "handle", ids: "/nodes/*/handles/*/id" },
{ granularity: "edge", ids: "/edges/*/id" }]`, `granularityFallback: "node"` — exactly today's `board2dGranularityById`.
(Rule order: a later rule overwrites an earlier one for the same id, as the map does today.)

### 3.3 Icon classifier kind

`classifierKind` on an `iconSelect` control stays a string, but its vocabulary is the framework's: `"puzzle2d"` → `"shapeGlyph"`
(what `classifyIconSelectorMode` does: it sorts icons into shape glyphs and pictograms). No host compares it to an app name.

## 4. Rename table

### 4.1 React `B2H/🟦️.tsx` (all 52 hits; line numbers of today's file)

| old | new | lines |
|---|---|---|
| `type Puzzle2dFixtureDropPayload` | deleted → `CatalogueDropPayload` (generic, § 2) | 47, 165, 448, 1611 |
| doc: "puzzle 2d: `puzzle2d-play-kinds.nodes.beam`", "puzzle 3d's" | "e.g. `<sectionId>.<kindId>`", "a bare-kind row" | 107, 108 |
| `parsePuzzle2dCatalogueDragPayload` | `parseCatalogueDragPayload(encoded, dropPayload)` | 162, 1639, 1654, 1667, 1696 |
| doc: "`vortex`-domain granularity", "client twin of the guest's `puzzle2d_selection_targets`" | "the scene's declared granularity classifier" | 203, 204 |
| `PUZZLE2D_TRANSIENT_EVENT_NAMES` | deleted → `BOARD_EVENT_DELIVERY[kind].delivery === "transient"` | 324, 346 |
| `PUZZLE2D_FLUSH_NOW_EVENT_NAMES` | deleted → `… === "flushNow"` + `awaits` | 325, 352 |
| `Puzzle2dLiveMirrorMutations` | `BoardLiveMirrorMutations` | 359, 378, 609 |
| `collectPuzzle2dLiveMirrorMutations` | `collectBoardLiveMirrorMutations` (reads `mirror` from the table) | 378, 909 |
| `puzzle2dEntityFlag` | `boardEntityFlag` | 432 |
| `puzzle2dFixtureDropPreviewJson` | `boardCatalogueDropGhostJson(payload, dropPayload.ghost, x, y)` | 448, 1643 |
| `puzzle2dScreenToWorld` | `boardScreenToWorld` | 490, 501, 1327, 1614, 1641 |
| `puzzle2dWorldToScreen` | `boardWorldToScreen` | 503, 774 |
| `beginPuzzle2dPeerGesture` | `beginBoardPeerGesture` | 594, 1374 |
| `endPuzzle2dPeerGesture` | `endBoardPeerGesture` | 583, 598, 1409, 1422, 1445, 1501 |
| `puzzle2dPeerOwnsGesture` | `boardPeerOwnsGesture` | 604, 960, 972, 1155, 1184 |
| `pushPuzzle2dLiveMirrorMutations` | `pushBoardLiveMirrorMutations` | 609, 910 |
| `notifyPuzzle2dPeersGestureEnded` | `notifyBoardPeersGestureEnded` | 627, 1413, 1433, 1449, 1509 |
| `pushPuzzle2dFixtureDropPreview` | `pushBoardCatalogueDropGhost` | 638, 1607, 1643 |
| doc: "the 2d twin of puzzle 3d's `hoveredKindId`" | "the board twin of the 3d world's `hoveredKindId`" | 846 |
| `dispatch("applyBoardEvents", …)` | `dispatch(scene.eventsAction, …)` | 927, 1576 |
| `dispatch("setCamera", …)` | `dispatch(scene.cameraAction, …)` | 925 (F7), 1030 |
| `dispatch("addNode", {…})` | `dispatch(dropPayload.action, mapped)` | 1620 |
| `board2dGranularityById(fixtureJson)` | `board2dGranularityById(fixtureJson, classifier, fallback)` | 208, 840 |

Importers to follow: `OSM/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx:1196, 1229` (re-exports), `🧪️tests/🔬️engine-contract/🟦️.ts`
(test names "puzzle 2d …" → "board 2d …"; the `-t "puzzle 2d|board 2d|…"` filter of the reports changes with them),
`B2H/🧪️tests/**`.

### 4.2 React `🗣️Interpreter/🟦️.tsx`

| old | new | lines |
|---|---|---|
| `vortex: "ui.surfaceContextMenu.vortex"` | the domain's declared `label` (§ 3.1); the i18n key is deleted | 821 |
| `classifierKind === "puzzle2d"` | `classifierKind === "shapeGlyph"` | 1210, 1795 |
| doc examples `1:puzzle3d-main-perspective` | `1:<surfaceId>` | 520, 534 |

### 4.3 wgpu `EC/🎯️targets/🧊️wgpu/🦀️.rs` (callers in `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (10), `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (17),
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (3), tests `🧪️tests/{🔬️wgpu-board2d-engine (3), 🔬️wgpu-catalogue-workflow-drop (3), 🧊️wgpu-standalone (9),
🧩️wgpu-engine-surfaces (10)}`, `🎞️Scenes/🧪️tests/🧊️wgpu-standalone` (10), `🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts` (1))

| old | new |
|---|---|
| `puzzle3d_catalogue_drag_payload_json` | `catalogue_drag_payload_json(raw, drop_payload)` |
| `puzzle3d_catalogue_drag_payload` | `catalogue_drag_payload(drag_data, drop_payload)` |
| `puzzle_board_camera` | `board_camera` |
| `puzzle_board_set_camera_silent` | `board_set_camera_silent` |
| `puzzle_board_publish_camera_into` | `board_publish_camera_into` |
| `puzzle_board_yield_to_pinch` | `board_yield_to_pinch` |
| `puzzle_board_pointer_cancel_into` | `board_pointer_cancel_into` |
| `puzzle_board_flush_events_into` | `board_flush_events_into` |
| `puzzle_board_direct_pointer_into` | `board_direct_pointer_into` |
| `puzzle_board_hover_into` | `board_hover_into` |
| `puzzle_board_key_into` | `board_key_into` |
| `puzzle_board_pointer_down` | `board_pointer_down` |
| `puzzle_board_pointer_move_into` | `board_pointer_move_into` |
| `puzzle_board_pointer_up_into` | `board_pointer_up_into` |
| `puzzle_board_pointer_leave_into` | `board_pointer_leave_into` |
| `puzzle_board_wheel_into` | `board_wheel_into` |
| `board_event_transient`, `board_event_flush_now` | deleted → `kind.delivery()` (generated) |
| `board2d_granularity_by_id(fixture_json)` | `board2d_granularity_by_id(fixture_json, classifier, fallback)` |
| literals `"applyBoardEvents"`, `"setCamera"` in the board functions | the scene's `events_action`, `camera_action` (cached in `board_sync_cache`) |

### 4.4 Guests (S5-PUZZLE)

Puzzle 2d and 5d set `eventsAction`, `cameraAction`, `dropPayload`, `granularityClassifier`, `granularityFallback` on their
`Board2dScene`; their descriptors declare the `vortex` domain with `primary: true`, `defaultGranularity`, `granularities`, `label`;
the drop payload schemas are published. No guest behaviour changes: the declared values are today's literals.

## 5. Proof and gate (§22.31 c)

- Law (both hosts): every corpus case coalesces as the corpus states, reading only the generated table.
- Law (schema): the generated Rust and TS tables equal the schema's `x-semio-board-event` rows (`check-generated`).
- Law (second producer, S5-TOOLS): block 2d emits `gesture` rows through the same host and verb fields.
- Gate (S5-GATES): no `puzzle` / `vortex` in non-test, non-fixture source under `🧰️framework/**` outside doc comments.

## 6. Order of implementation (each step leaves both hosts on the same corpus)

1. S5-PUZZLE: schema + annotation vocabulary + generator + generated tables + engine enum swap (no behaviour change).
2. S5-UI and S5-WGPU: read the table; delete the four name sets / functions; rename per § 4 (pure renames, one wave each).
3. S5-PUZZLE + S5-UI + S5-WGPU: `Board2dScene` fields (`eventsAction`, `cameraAction`, `dropPayload`, classifier) — contract change:
   UI contract schema + Rust + TS twins + both hosts + both guests in ONE train wave (it changes the scene wire, so it rides a
   channel bump if the scene is part of the frame layout — S5-CHANNEL decides).
4. S5-RUNTIME: primary-domain declaration replaces the runtime literal. S5-GATES: the gate. S5-TOOLS: block 2d producer + law.
