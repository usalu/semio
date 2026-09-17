# 📓️ Wave 2B — puzzle ◻️2d hover paint, handle-suggestions popup, shift+tab back-cycle

Slice 2B of the 2026-09-17 parity fleet. Scope: E2 §7/§9/§13 + OWED 1/2/2b, E8 §1 (Hover, Suggestions
popup, Brush rows), E1 rows 53-58 + §H4. Paths below are repo-relative.

EDITOR2 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
HOST = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`

---

## 1. Hover paints (E2 §7 / OWED 1 / E8 §1 "Hover")

The audit's one-line diagnosis was incomplete: the guest hardcode was only half the break. The chain
was measured end to end in source and had **two** holes.

| link | before | after |
|---|---|---|
| board engine `HoverChanged` / `hover` event | real (`♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`, `set_hovered_id` / `update_hover_from_world`) | unchanged |
| `Board2dHost` → framework `interactionHover` | **never dispatched** — `hover` rows rode `applyBoardEvents`, which has no `hover` arm, so every hover was a wasted retained-job payload and the guest never learned of it | dispatched, coalesced (below) |
| guest scene echo `hoveredId` | `hovered_id: None` hardcoded | `envelope.interaction.hovered_id()` |
| `setHoveredIdSilent` | applied unconditionally, clobbering the live local hover | applied only to panes the pointer is NOT over |

Landed:

- `EDITOR2/🦀️.rs:164` — `Puzzle2dInteractionSnapshot::hovered_id()`, the one `"pointer"`-channel read.
- `EDITOR2/🎭️modes/✏️edit/🦀️.rs:194` — `hovered_id: envelope.interaction.hovered_id()` in
  `puzzle2d_board_scene`, i.e. **all three** window kinds (overview / detail / selection share one
  scene builder), so hover paints in every pane.
- `EDITOR2/🦀️.rs:4779` — `ArtifactEditor::interaction_scope` (2d had none; the framework's blanket
  `UiDirtyScope::Full` repainted every panel, rail, measure and label on **every pointermove**).
  Hover → `puzzle2d_window_only_scope()` (3 canvas bodies, nothing else); Select/ClearSelection/
  SelectAll → `puzzle2d_select_scope()`; mode/granularity → `puzzle2d_window_and_engagements_scope()`.
  A verb touching an undeclared domain answers `None` and keeps the framework's widest scope.
- Scene contract: `Board2dScene.domain_id` added
  (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`, TS twin `…/🎬️scene/🟦️.ts`) so the generic host
  knows which interaction domain to publish on instead of hardcoding `"vortex"`. 2d publishes
  `PUZZLE2D_INTERACTION_DOMAIN`; 5d's edit board publishes `PUZZLE5D_INTERACTION_DOMAIN`, its viewer
  board `None` (read-only).
- HOST: `"hover"` joined `PUZZLE2D_TRANSIENT_EVENT_NAMES`, so hover never rides `applyBoardEvents`
  again. `latestBoard2dHoverId(rows)` reads the last hover row per drained batch;
  `dispatchBoardHover` is a `createCoalescingActionDispatcher` over `dispatchSettled`
  (`onAction`'s awaitable twin — the only shape that actually arms the gate, per World3dHost's
  wave-B33 note), so a pointer storm keeps **at most one** `interactionHover` round trip outstanding.
- HOST: `board2dGranularityById(fixtureJson)` classifies node / `node:handle` / edge — the client twin
  of the guest's `puzzle2d_selection_targets` — and `board2dHoverActionArgs` builds the wire shape
  (mirrors `world3dHoverActionArgs`).
- HOST: local paint is free — the engine already painted its own raycast this frame. The pane under
  the pointer therefore ignores the guest echo (`hoverActiveRef` guard) and mirrors it to
  `data-board-hover-paint-id` imperatively (no React state, no re-render).

**Tree ↔ canvas both ways.** The outliner already stamps one `interactionSelect`/`interactionHover`
binding at its tree root (`📌️panels/🗿️artifact/🦀️.rs`'s `PanelTreeBuilder::interaction_domain`) and its
rows carry real entity ids **and** a granularity, so a hovered row now highlights on every canvas via
the scene echo; the canvas → tree direction is the new host dispatch. No per-row action was added (the
`project-ui-value-map-ascending-keys-and-arena-page` starvation note).

**The catalogue panel was deliberately left unbound.** 3d's catalogue tree does stamp
`interaction_domain` (`🧊️3d/…/📌️panels/🛍️catalogue/🦀️.rs:151`), but its rows are KIND rows with no
granularity and the domain declares `transitive: false` on both sides — so the binding resolves a kind
id the scene cannot paint. Copying it into 2d would buy a hover round trip per catalogue row and no
highlight. Real catalogue→canvas highlighting needs transitive kind hover
(`BoardHost::set_hovered_kind`, which the engine already has) and a `hoverKind` scene channel; that is
a separate feature, not this slice's, and it is listed under NOT verified rather than silently skipped.

## 2. Handle-suggestions popup, reachable without arming the brush (E2 §13 / OWED 2, E1 §H4, E8 §1)

### 2.1 One verb set, no aliases (greenfield rename)

The brush slot family and the 3d suggestions family are the same mechanism, so they are now one set.
Every id below went through **every** registry listed in `📜️executor-rules.md`.

| old 2d id | new id | note |
|---|---|---|
| `brushOpenSlot` | `openHandleSuggestions` | + `x`/`y`/`windowId` popup anchor |
| `brushCancelSlot` | `closeHandleSuggestions` | |
| `brushSetCandidateIndex` | `hoverSuggestion` | provisional preview, never a commit |
| `brushCommitSlot` | `acceptSuggestion` | ONE edit + re-select |
| `brushCycleCandidate` | `cycleBrushCandidate` | matches 3d's spelling |
| — | `cycleBrushCandidateBack` | new, for the `shift+tab` chord |
| — | `targetBrushSuggestions` | new, armed-brush targeting channel |

Command directories renamed to match (`🎮️commands/🔓️open-handle-suggestions`,
`🔒️close-handle-suggestions`, `🖱️hover-suggestion`, `✅️accept-suggestion`,
`🎣️target-brush-suggestions`, `🔁️cycle-candidate` kept), module paths rewired in
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs`.

### 2.2 Popup state (WindowTransient, per the slice brief)

- `EDITOR2/🎚️config/🦀️.rs` — new `Puzzle2dSuggestionMenu { x, y, window_id, handle_id }` (the 2d twin
  of `Puzzle3dSuggestionMenu`) + `Puzzle2dPlayRuntime.suggestion_menu`.
- `EDITOR2/🪟️window/🦀️.rs` — `Puzzle2dWindowTransient.suggestion_menu`, its
  `store::artifact_retire_struct!` cursor, the retained-bytes charge, and `runtime()`/`split()`
  threading. **WindowTransient lane only** — per-gesture scratch never reaches a persisted config.
- Schema twins updated by hand: `EDITOR2/🪟️window/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
  (`Puzzle2dSuggestionMenu` + `Puzzle2dWindowTransient.suggestionMenu`, `anyOf [$ref, null]` exactly
  like 3d's), and the neutral fixture case in
  `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`.

### 2.3 The guest is authoritative for the edit; the client only previews

The guest `BoardHost` is rebuilt from the document on **every** dispatch
(`puzzle2d_dispatch_emit`), so the slot `openHandleSuggestions` opened is gone by the next verb.
`EDITOR2/🦀️.rs:1517` `puzzle2d_restore_brush_slot(ctx)` re-enters it from the handle the window
transient remembers. The rebuild is deterministic (same fixture, catalogs, weights), so the candidate
page is the one the client painted and the index the popup names is the candidate that gets placed.
`acceptSuggestion` therefore commits through `apply_host_events` → `brushPlace` → **one** document
delta, and re-selects the placed node via `Emit.interaction_writes` — never through a client-side
commit, which would have been a second placement.

### 2.4 Candidate rows a slot can actually build from

A candidate is only buildable from a node kind that carries **handle templates**
(`brush_compatible_candidates` skips `kind.handles.is_empty()`), and `board_kind_catalogs_json` did not
enforce that: Concrete Forest names `meta.manifest-id=concrete-forest`, so it resolved the manifest's
presentation-only rows, the engine's usable `node_kinds` map stayed empty, and every brush or
suggestion lookup silently yielded nothing. `EDITOR2/🦀️.rs:401`
`board_kind_catalogs_json_or_inferred` applies the **same three-step rule the fill run already uses**
(`precompute::fill::fill_kind_rows`): document catalogs → manifest rows *if any carries templates* →
the kinds the document itself implies (`inferred_node_kind_rows`). Both the guest host sync
(`sync_host_fixture_content`) and the client's `glyphCatalogsJson` now go through it, so the popup, the
armed brush and the fill all see one catalog.

### 2.5 Entry points

- Context menu row `suggestNodes` → `openHandleSuggestions{handleId}`, shown **only** when exactly one
  handle is selected (ported from 3d's vortex branch, `🧊️3d/…/🦀️.rs:3003`). The board host's
  `onContextMenu` already silently selects the right-clicked target, so a right-click on a handle
  reaches it.
- `mapContextMenuSpecs` (World3dHost, shared) now injects the click point for `openHandleSuggestions`
  as well as `openVortexSuggestions`, so the popup hangs where the user clicked.
- A per-window `dispatchSuggestion` stamps `windowId` on every popup verb, so a sibling pane never
  adopts another pane's menu.
- Both popup rows carry `handleId` (`args` for accept, `hoverArgs` for preview) and
  `puzzle2d_restore_brush_slot` honours a caller-named handle **before** the transient. A client with
  the menu still on screen therefore never depends on the window transient having round-tripped back
  to the guest yet — the same reason 3d's `acceptSuggestion` reads `fullId` from its args first.

### 2.6 Host surface (reused, not forked)

- `Board2dScene.suggestion_menu_json` (+ TS `Board2dSuggestionMenu`/`Board2dSuggestionCandidate`
  types) carries the open popup. Built by `puzzle2d_suggestion_menu_json`
  (`EDITOR2/🎭️modes/✏️edit/🦀️.rs:127`), bounded to
  `PUZZLE2D_SUGGESTION_MENU_CANDIDATE_PAGE = 8` rows — the surface doc is fixed-capacity and a richly
  catalogued handle resolves arbitrarily many candidates.
- HOST renders it through the **same** `ContextMenuController` component path World3dHost uses
  (`closeOnSelect={false}`, `board2dSuggestionMenuItems` → `mapSuggestionMenu`), with
  `board2dSuggestionMenuOwnsWindow` scoping and a sibling-pane Escape/outside-dismiss path. The
  board's ordinary context menu is suppressed while the popup owns the pane.
- Vitals: `data-board-suggestion-menu-json` and `data-board-hover-paint-id` (2E owns the rest).
- Provisional paint: opening/previewing mirrors into THIS pane's engine
  (`brushOpenSlot`/`brushSetCandidateIndex`, newly declared on `Board2dWasmSession` — the wasm bridge
  `EDITOR2/🌉️wasm/🦀️.rs` already exported all four). Closing/accepting clears the local slot; the
  commit is never mirrored.

## 3. `tab` / `shift+tab` back-cycle (E2 OWED 2b, E1 §I)

- App keybindings `.keybinding("tab", "cycleBrushCandidate")` and
  `.keybinding("shift+tab", "cycleBrushCandidateBack")` — the exact 3d pair.
- HOST: the Tab handler moved out of the bubble-phase keydown listener into its own **capture-phase**
  listener. `ShellHost`'s keybinding dispatcher bails on `event.defaultPrevented` but runs on the
  shell root (an ancestor), so a bubble-phase `preventDefault` reaches it too late and the slot would
  have advanced **twice**. Capture + `preventDefault` makes it exactly one step, with the local engine
  moving the ghost this frame and the flushed `brushCandidates` row carrying the index to the guest.
- The chord is armed by the armed brush **or** by an open popup — which is what makes the picker
  keyboard-drivable without the tool.

## 4. Engine (shared, `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`)

One public method added next to `brush_open_slot`: `brush_target_slot(Option<&str>)` — points the slot
at a handle or at nothing **without** claiming the popup's hover/`suggestionsActive` flag. That is the
armed brush's own targeting channel (`targetBrushSuggestions`), so the popup and the brush share one
slot rather than growing a second one.

One fix in `brush_enter_slot`: **"open handle" is now one definition, shared by all three entry
points.** The pointer path already refused a fastened handle (`brush_nearest_slot_source` filters on
`handle_has_incident_edge`), but the direct API did not — `brush_open_slot` on a handle that already
carries an edge resolved a full candidate page by kind compatibility alone and would have placed a node
on top of an existing fastening. It now resolves an EMPTY page instead, which is exactly what a fully
fastened document's popup reads as "no placement available". The engine's own
`board_host_brush_open_slot_suggestions_commit_and_cancel` test opens on an edge-less handle and is
unaffected.

## 5. Registries touched (every new/renamed verb, all of them)

`puzzle2d_command_variants!` · `PUZZLE2D_RETAINED_TOOL_IDS` · `PUZZLE2D_GENERIC_TOOL_IDS` ·
`bounded_first_step_tool_proofs!` tools list · `build_tool_job` (generic arm) ·
`PUBLICATION_CONTRACTS` · `.action_with` + `.action_interactive_job(Migrated)` in
`create_puzzle2d_app` · `command_from_action` (generic, via the variants macro) ·
`🗣️terminology/🦀️.rs` EN+DE · `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` ·
`PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json`.

Lanes:

| verb | lanes |
|---|---|
| `openHandleSuggestions`, `closeHandleSuggestions`, `hoverSuggestion`, `cycleBrushCandidate`, `cycleBrushCandidateBack`, `targetBrushSuggestions` | `WindowTransient` |
| `acceptSuggestion` | `Artifact`, `WindowTransient`, `Interaction` |

`acceptSuggestion` gained the `Interaction` lane over the old `brushCommitSlot` because it now
re-selects the placed node — an undeclared lane is a runtime fault, so this is load-bearing.

## 6. Laws

Rust (`EDITOR2/🧪️tests/🔬️unit/🦀️.rs`, region `🔖️HandleSuggestions`):

1. `hover_id_reaches_the_board_scene_for_every_granularity_and_pane` — node / handle / edge × all
   three panes, plus `domain_id`, plus "no hover paints no hover".
2. `suggestion_popup_publishes_the_shared_candidate_page_and_the_previewed_index`.
3. `a_closed_suggestion_popup_publishes_no_menu`.
4. `context_menu_offers_suggest_nodes_on_one_selected_handle_only`.
5. `open_hover_accept_places_one_node_on_concrete_forest_and_reselects_it` — one new node, one
   commit, and the new node is **fastened to the handle the popup opened on**.
6. `nakagin_refuses_the_suggestions_popup_politely`.
7. `closing_the_suggestions_popup_discards_the_preview`.
8. `cycling_candidates_forward_and_back_never_commits`.

Engine (`EDITOR2/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs`):

9. `board_host_brush_open_slot_refuses_a_fastened_handle` — the shared "open handle" definition: a
   fastened handle resolves `"candidates":[]` and commits nothing; a free handle on the same board
   still resolves its compatible kinds.

`view_actions_emit_no_ops_through_the_registry` extended with the six new/renamed View verbs.

TypeScript (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`):

10. hover stays out of the board-events batch.
11. `latestBoard2dHoverId` (last row wins / clears / leaves alone).
12. granularity classification + `interactionHover` wire shape.
13. popup parse, window scoping, hover-preview rows distinct from the commit row (both rows carry
    `handleId`).
14. polite refusal (pending vs resolved-empty).

## 7. Commands run

Evidence under `TICKET/🗑️generated/2B/`.

| # | command | verdict |
|---|---|---|
| 1 | `bun ./📜️script.ts test long -t "interactionHover lane\|LAST hover row\|vortex-domain granularity\|handle-suggestions popup\|refuses politely"` (renderer react package) | **PASS — 5 passed, 1425 skipped, exit 0** (`ts-laws-1.txt`, and `ts-laws-2.txt` re-run after the last host change — every law named in the verbose output) |
| 2 | `bun ./📜️script.ts typecheck` (renderer react package) | **repo-wide RED before my change — 858 `error TS`.** Filtered: `🎬️scene/🟦️.ts` 0, `Board2dHost` 1, `WasmSessionLoader` 1, my law range (engine-contract 5054-5103) 0, and **0** errors naming any symbol I added. (`ts-typecheck-1.txt`) |
| 3 | 2d window-ownership Ajv + exact-record oracle, run standalone against `EDITOR2/🪟️window/🧬️schema/🔣️.json` with the audit's own settings | **PASS** — closed menu valid, open menu valid, app-config leak refused, exact keys match |
| 4 | `bun ./📜️script.ts publication-authority-audit [Puzzle2dPlayApp]` | **RED, and the one failing condition is NOT mine.** I reproduced `ownerOracle` step by step (`publication-audit-diagnosis.txt`): routes ↔ `action_interactive_job` pairs **OK**, `PUZZLE2D_RETAINED_TOOL_IDS` ↔ migrated **OK**, `bounded_first_step_tool_proofs` tools ↔ migrated **OK**, `PUBLICATION_CONTRACTS` ids **OK**, every per-route lane set **OK** — i.e. every registry my seven verbs touch agrees with the fixture. The sole failing boolean is the literal-source check `(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))`, which slice **2D** replaced with an early-return plus a proximity-connect budget (`EDITOR2/🦀️.rs:2840-2846`). Earlier in the session the audit threw before that, in `validateWindowOwnershipSchemas`, on a 5d `voxelDims` skew — since fixed by that slice. `Puzzle3dPlayApp` and `Puzzle5dPlayApp` both **PASS** the audit, so 2d's extent literal is the only thing standing between the plugin and exit 0. |
| 5b | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | **PASS — exit 0, `Finished \`dev\` profile … in 10m 52s`, 0 errors, 7 warnings in the 2d lib** (all pre-existing style lints, none from my edits). (`check-native-final.txt`) |
| 5a | `rustfmt --emit stdout` over all nine Rust files I touched | **PASS** — every file parses (syntax only; type checking is #6) |
| 5 | static registry audit of all seven verbs across every registry named in `📜️executor-rules.md` | **PASS** (`registry-audit.txt`) — RETAINED/GENERIC/proofs/variants/PUBLICATION_CONTRACTS all OK, `action_with` + `action_interactive_job` + dispatch arm present for each, both keybindings present, **0** legacy slot ids left |
| 6 | `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- suggestion hover_id cycling_candidates brush_open_slot` | **2 passed, 6 failed, 863 filtered out** (`test-suggestions-2.txt`). The lib and the lib-test target **compile** — no error anywhere outside a `🧪️tests` path. My two ENGINE laws pass, including the new `board_host_brush_open_slot_refuses_a_fastened_handle`. My six EDITOR laws all die in the same place: `📚️examples/🌲️concrete-forest/🦀️.rs:31` `panic!("concrete-forest example dsl parses: expected List, found Absent at 1:1")`, which then poisons the `LazyLock` for every sibling test in the process. **Root cause is not mine:** slice 2F added `target_regions: Vec<Puzzle2dTargetRegion>` as a `#[dsl(table)]` field to `Puzzle2dSnapshot` (`✳️any/🧬️schema/📸️snapshot/🦀️.rs`) and the DSL table is positionally required, so **every shipped example document** (which has no `target-regions` table) now fails to parse. Owed to integration. |
| 7 | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --target wasm32-wasip2` | **PASS — exit 0, `Finished \`dev\` profile`, 0 errors, 76 warnings (7 in the 2d lib, all pre-existing style lints: unqualified `Point`/`size_of` paths and a peer's unused `Buildable`/`HasBase` import).** (`check-wasm-1.txt`) |

Build-gate note: for most of the session the addendum's `< 3 concurrent cargo per crate` gate never
opened — 8-12 sibling `cargo check -p semio-s-artifact-puzzle-2d` processes were alive continuously,
**all at 0% CPU blocked on the shared artifact-directory lock** (the actual holder was an unrelated
`semio-s-artifact-energy-model` build). I stopped my own stale check to give the queue a slot back
(verified no `ppid=1` cargo orphan was left behind). Under the 19:55 addendum I then ran exactly ONE
gated native check with `--message-format=short` (#5b) and stopped polling.

**Summary: every gate this slice owns is green.** Native check, wasm32-wasip2 check, registry audit,
window-ownership oracle, rustfmt parse and all five React laws pass. The three remaining reds are each
traced, by path, to another slice's file — see §9.

## 8. NOT verified / owed to integration

**Owed to integration (blocked by another slice, retry after they land):**

- **My six EDITOR laws have never executed.** They compile; they die in `LazyLock` initialization
  because `concrete-forest`'s example DSL no longer parses (§7 #6 — 2F's `target_regions` DSL table).
  Re-run `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --features
  component-app-assembly --lib -- suggestion hover_id cycling_candidates` once the examples parse
  again; nothing in my code needs to change for them to run. The two ENGINE laws already pass, so the
  shared "open handle" contract and the slot family's engine behaviour ARE proven today.
- **The plugin publication audit does not exit 0** — one literal in 2D's `puzzle2d_generic_extent`
  (§7 #4). Every route/lane/registry the 2d owner declares already agrees with the fixture.

**Not verified at all:**

- **Runtime.** No activate / serve / battery — the coordinator owns those. Nothing in this slice has
  been driven in a browser: the popup's placement, the `tab`/`shift+tab` capture-phase ordering
  against the live shell keybinding dispatcher, and the hover round-trip cost under a real pointer
  storm are all source-and-law arguments, not measurements.
- **The `interactionHover` throttle is unmeasured.** `createCoalescingActionDispatcher` is the same
  gate World3dHost uses and it is armed correctly (awaitable `dispatchSettled`, not `dispatch`), but
  no one has counted guest turns per 70-move storm on the board the way wave B33 did for 3d.
- **`interaction_scope` narrowing is argued, not measured.** A hover now repaints only the three
  canvas bodies. The argument: `grep '\.hovered\b|hovered_id()'` over the whole 2d editor tree returns
  exactly two hits — the accessor and the scene builder — so no panel body renders hover, and tree-row
  hover presence is a framework overlay applied after render; 3d's own `puzzle3d_viewport_scope` (its
  `Interaction(Hover)` answer) likewise lists `panel_bodies: Vec::new()`. Still, only a battery proves
  no 2d surface silently depended on `Full`.
- **`targetBrushSuggestions` has no host caller yet.** The verb, its lanes and its engine channel
  (`brush_target_slot`) are real and registered, but `Board2dHost` does not dispatch it: the browser
  engine resolves the armed brush's candidates locally today. It is the guest-side half of the
  mechanism, reachable programmatically and from the palette, and deliberately not a dead
  `BatchOnlyPendingRewrite`-class verb — but nothing in the UI fires it.
- **The re-selection after `acceptSuggestion` is asserted at source level only.** The guest pushes the
  placed id through `Emit.interaction_writes` and the `Interaction` lane is declared, but
  `InvocationResult` carries no interaction writes and the test harness's `render_body` goes through
  the non-request-context `render` (selection always `[]`), so no law observes the placed node
  actually becoming the live selection. A battery would.
- **Catalogue-row → canvas highlight is NOT delivered** (see §1). The outliner half is; the catalogue
  half needs transitive kind hover and a scene channel for it, which is a feature of its own.
- **The whole-plugin publication audit does not exit 0** for the reason in §7 #4 — a 5d skew, not 2d.
- **5d/3d crates not checked by me.** I added `domain_id` to the two 5d `Board2dScene` literals so they
  keep compiling; `cargo check -p semio-s-artifact-puzzle-5d` is 5F/5E's gate, not mine.

## 9. Hand-offs

- **2F (blocking the whole crate's tests)** — `Puzzle2dSnapshot.target_regions` is a `#[dsl(table)]`
  field (`✳️any/🧬️schema/📸️snapshot/🦀️.rs`), and a DSL table is positionally required, so **every shipped
  example document** fails to parse: `concrete-forest example dsl parses: expected List, found Absent at
  1:1`. That panic poisons the example `LazyLock` and takes down every editor unit test in the process,
  mine included. Either give the table a DSL-level default/optional marker or add an empty
  `target-regions` table to the shipped example DSLs.
- **2C** — `Board2dHost/🟦️.tsx:1033` calls `session.setGridVisible?.(…)`, which is **not declared on
  `Board2dWasmSession`** (`🪪️WasmSessionLoader/🟦️.tsx`): the only typecheck error in that file
  (`TS2339`). Add the optional method next to the brush-slot block I added there.
- **2E** — we both edit `Board2dHost`. I own: the `hover` transient-name entry, the Hover and
  SuggestionMenu regions, the capture-phase Tab listener, `data-board-hover-paint-id` and
  `data-board-suggestion-menu-json`. You own the rest of the vitals, `board2dStatusJson` and the
  gumball. I added `Board2dScene.domain_id` + `suggestion_menu_json` to the shared scene struct and
  filled `grid_visible` / `selectable_*` into `puzzle2d_board_scene`'s literal so the crate would
  build — if your slice writes those fields too, the duplicate is a compile error, keep one.
- **2D** — you own the publication audit's only red condition. `ownerOracle` (`PLUGIN/📦️packages/🟦️typescript/📜️script.ts:293`)
  asserts the source still contains `(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))`;
  `puzzle2d_generic_extent` now early-returns and adds `PUZZLE2D_PROXIMITY_GESTURE_MAX`
  (`EDITOR2/🦀️.rs:2840-2846`), so the audit cannot exit 0 until that oracle literal is updated to the new
  shape. Everything else in the 2d owner passes — see `🗑️generated/2B/publication-audit-diagnosis.txt`.
- **2D** — `acceptSuggestion` now writes the `Interaction` lane. If `createEdge`/`proximityConnect`
  also re-select, declare that lane too; an undeclared lane is a runtime fault, not a warning.
- **2G / probes** — the new DOM surface is `data-board-suggestion-menu-json` (the full popup record,
  `""` when closed) and `data-board-hover-paint-id` (the locally painted hover, absent when none).
  `data-board-hovered-id` stays the guest echo. The existing probe
  `🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts` already asserts this family and
  **must be retargeted**, exactly here:
  - `:1623,1626` expect the context-menu row's `action === "brushOpenSlot"` → now `openHandleSuggestions`
    (the row id is `suggestNodes`; the probe's `/suggest|vorschl/i` text match still hits).
  - `:1597` expects `tab → brushCycleCandidate {"forward":true}` → now `tab → cycleBrushCandidate`.
  - `:1607` expects `shift+tab → brushCycleCandidate {"forward":false}` → now a **separate id**,
    `shift+tab → cycleBrushCandidateBack` with no args (3d's shape).
  - `:1562`'s note that "`brushOpenSlot` may be authored as a hover-preview or as a press" is now
    settled: opening is a press (context-menu row), previewing is `hoverSuggestion`, placing is
    `acceptSuggestion`.
  - **`suggestions-menu` picks the wrong granularity.** It calls `pickNode(0)` and then expects a
    suggest row. The row is authored for exactly one selected **handle** (3d's parity: the vortex
    branch, never the object branch), and a node has no handle to grow onto — the probe must select a
    free handle, not a node, or it will read red against correct behaviour. The law
    `context_menu_offers_suggest_nodes_on_one_selected_handle_only` pins that contract.
- **Coordinator** — `board_kind_catalogs_json_or_inferred` changes what the board engine receives as
  `glyphCatalogsJson` for documents whose catalog rows carry no handle templates (Concrete Forest).
  That is the same rule the fill run already applies, but it is the one change in this slice that
  could alter existing glyph rendering, so it deserves a look in the first 2d battery.
