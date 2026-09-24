# 📓️ Puzzle 5d — context-menu suggestions, vortex selection sync, transform gumball

Opened 2026-09-23 (session ⚪4905b6a0, Opus 5.5). Repo MCP `ticket_open` answered "invalid tool params" → bookkeeping on disk.
Predecessor: `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`.

## Root causes found (why 5d felt ad hoc)

| # | Symptom | Root cause |
|---|---|---|
| 1 | Transform showed Move/Rotate/Scale as tabs **and** repeated the Move/Rotate flags in Utility Options; no gumball | 5d declared three grouped utilities (`move`/`rotate`/`scale`, `group: "transform"`) instead of puzzle 3d's single `transform` utility, and emitted one flag group per handle. |
| 2 | Vortex (grip) selection not synchronized between 3D pane and board/inspector | The 5d World3d scene never set `domain_id`/`domain_granularity_id`, so `World3dHost` fell back to the domainless `worldPick`/`worldVortexSelect`/`setHover` verbs 5d has no handler for. Grip/fastener markers also reported the host's default granularities (`vortex`/`attraction`), which are not granularities of 5d's domain; `targetVolume` was not a declared granularity at all. |
| 3 | Grip context menu offered no working suggestions | The "suggest" row dispatched `targetBrushSuggestions`, which only acts while the brush is armed (a dead row from a context menu); 5d published no `suggestionMenu`/`hoveredVortexFullId`, had no open/close/hover/accept verbs, and a right-click on a grip of a selected part opened the PART's menu (`from_surface` let the selection win over the hit). |
| 4 | Misc. drift from 3d | No `lodJson` (LOD/grid options had no effect on the host), chunk size hard-coded to 256, `volumeBrush` presented as `select` to the host, no `voxelDims`/`gridFactor` on the interaction lane, per-window active utility looked up by window KIND instead of instance id in `render`/`context_menu`. |

## Design

**Suggestions submenu (World3dHost, shared by 3d and 5d)**
- On right-click the host inspects the resolved menu specs; a row dispatching `openVortexSuggestions` with a `fullId` makes the menu a *suggestion menu* (`world3dSuggestRowTarget`).
- An effect keyed on that target dispatches `openVortexSuggestions {submenu: true}` the moment the menu opens (search starts precomputing) and `closeVortexSuggestions {fullId}` when it closes or retargets (scoped close, so a late close never tears down a newer menu).
- The row becomes a live submenu (`world3dMenuWithSuggestionSubmenu`) whose children are the candidate rows built from `interaction.suggestionMenu` (pending → "Checking placement…", empty → "No placement", else one row per free candidate).
- Each candidate row carries its trace `key`; hovering it sets `suggestionFocus` and the tool-run trace layer draws **only that record** (`ToolRunTraceLayer focus`, `ToolRunTraceRecordStore.slot`); leaving releases it. `data-suggestion-focus` publishes it as a vital.
- The floating popup (Alt+right-click / armed brush) is unchanged and only renders for `submenu: false`.

**Guest**
- 3d: `Puzzle3dSuggestionMenu.submenu`, candidate `key` + catalog label, scoped close, `BrushSuggestionsFound::{free_keyed,target,done}`, context-menu hit-subject rule.
- 5d: single `transform` utility; domain binding + marker granularities; `suggestionMenu`, `hoveredVortexFullId`, `voxelDims`, `gridFactor`, `lodJson`, runtime chunk size; `targetVolume` granularity; four verbs `openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`acceptSuggestion` (all registries + fixtures + schemas); suggestion menu in the window transient (schema in json/graphql/ts/proto/rust); context menu hit-subject rule + `fill_from_interaction`; open/close emit the brush run's start/abort themselves (see below).

**Why 5d's open/close emit the run effect themselves** — live probe: the menu opened, but no tool run ever started; `pending_effects` is only drained by refresh passes the host runs after certain invocations, not after this window-transient-only verb (the armed-brush path does get one). The command now runs the same `run_effects` ladder the poll uses and returns its `toolRunStart`/`toolRunAbort`; the link's outstanding-request latch keeps the poll from repeating it. Puzzle 3d keeps the poll-only design (its laws are built on it); see Open items.

## Verification
See the Log below.

## Open items
- `kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode` (5d) fails at HEAD independent of this ticket: the framework route now wraps retained faults as "framework route 'import-media' returned a retained fault", the law expects "predecode cap".
- `brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin` (3d) is a wall-clock law; it failed only under load average 70–110 from peer builds.
- `the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement` (3d) fails at HEAD: framework logs a lane-less (View) typed op's history row at RETIREMENT, after the completion's `history_patch` was taken, so the row lags one command (peer commit `48b9d63cf6`). Recording it at completion fixes the lag but breaks laws that assert refused/no-op mutations log no row (they only passed through the lag) → semantics decision spun off as its own task; my trial edit reverted.
- Also failing at HEAD, unrelated to this ticket: 3d `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`, `a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle`, `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`, `retained_publication_contracts_are_an_exact_nonempty_tool_bijection` + `transform_brackets_are_migrated_host_only_routes_that_complete_empty` ("requires exactly one manifest declaration: 2" — the peer's plugin-builder action fan-out), wall-clock `*_stays_below_the_interactive_ceiling_*` laws under load; `semio-framework-plugin` lib tests do not compile (peer test: `active_tool` specified twice).
- Live boot: a click during the boot-time "Verifying shared access…" window can leave the app in `actor-activation.revoked` / "Shared access is unavailable" (peer hub-collaboration work); the probe waits it out.

## Log
- 11:3x Ticket opened on disk; baseline activation from cache; supervised serve :6014 (`…/☀️17/…/🔁️serve-supervisor.sh puzzle5d 6014`).
- 11:4x–12:0x Transform utility, domain binding, lanes, suggestion verbs, host submenu, trace focus implemented. `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --all-targets` EXIT=0 (5d lib 4 warnings, all pre-existing lines). Renderer `tsc`: my files clean (verified they are in the program via `--listFilesOnly`).
- 12:0x Live (built-in browser): one Transform toggle, its Move/Rotate flags shown directly; 3D pick selects `cb832a2c…` and the board pane's `data-board-selection-json` holds the same id; gumball rendered (screenshot).
- 12:1x Live: right-click on a grip of a SELECTED part opened the part menu → hit-subject rule added to 5d and 3d `from_surface` (+ law in 5d).
- 12:4x Live: menu opened and `suggestionMenu.submenu` published, but no tool run ever started (`data-tool-run-run=""`); armed brush does start runs → 5d open/close now emit the run effect themselves; native law `the_grip_suggestion_submenu_searches_without_the_brush_and_accepts_a_candidate` added.
- 13:1x–14:2x Live boot blocked by a peer's new `👕️canvas-presence` (`artifactPresenceRosterV1` returned a fresh `[]` → `useSyncExternalStore` "Maximum update depth exceeded"); fixed with one shared frozen empty roster. Committed plugin descriptor was stale (still `move/rotate/scale`, no suggestion verbs) → regenerated with `bun ./📜️script.ts describe` in `🧩️puzzle/📦️packages/🦀️rust` (32 min).
- 14:3x Headless probe `🔍️suggestion-probe.ts`: **PASS=19 FAIL=0** — boot, Concrete Forest, one transform utility, transform arms, 3D pick selects the part, pick reaches the board, gumball armed, grip hover publishes `hoveredVortexFullId`, grip menu offers the suggest SUBMENU (no part rows), search starts when the menu opens (pending, submenu:true), 7 candidates found before the row is hovered, row hover lists 7, hovering candidate 0/1 focuses trace key 0/1 (only that ghost drawn — screenshots), accept places part-1 (instances 1→2), selects it and closes the menu, Escape releases the search, no page errors.
- Final gates: 5d lib 588 passed / 1 failed (pre-existing `kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode`), incl. `retained_command` fixture oracle laws; 3d lib (suggestion/context_menu/vortex/window filter) 144 passed / 1 failed (`brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin`, wall-clock under load 40–110); vitest `🎣️suggestion-submenu` + `⏯️tool-run-trace` 13/13; `publication-authority-audit` Puzzle5dPlayApp + Puzzle3dPlayApp valid (Puzzle2dPlayApp diverges from peers' in-flight 2d edits); renderer `tsc` 0 errors.

## Round 2 (stop-hook continuation, 15:0x–15:4x) — the board pane gets the same menu
- Shared host module `🧰️framework/…/🧱️elements/🎣️suggestion-submenu/🟦️.ts` (row detection, submenu splice, focus binding, open/close lifecycle hook, ownership predicates); World3dHost and Board2dHost both use it (World3dHost's private copies and `worldSuggestionMenuOwnsWindow` removed; React target re-export + engine-contract law moved to `suggestionMenuOwnsWindow`). Law suite moved to `🎣️suggestion-submenu/🧪️tests/🧩️component`.
- Board2dHost: grip right-click → same "Suggest parts" submenu (search starts on open, candidates stream, hover focuses the candidate's BOARD twin on `ToolRunTrace2dLayer` via new `focus` prop, popup only for non-submenu menus, local brush-slot preview only for popups).
- 5d board scene publishes `suggestionMenuJson` (board shape: `handleId`, `hoveredIndex`, `submenu`, twin keys as DECIMAL TEXT — `key | 2^62` does not survive a JS double; found live: every twin collapsed onto 2^62).
- Menu subject: only the MOST SPECIFIC hit (hosts list handle before its node) — fixed in 5d and 3d `from_surface`; 5d buckets the board's `handle`/`edge` domains.
- Board engine (`♾️infinite/🎲️board/…/➕️normal`): `resolve_pick_targets_world` now leads with the handle `resolve_hit_world` resolves (indirect-ring handles at Overview/Compact LOD were hoverable but never reached a context menu); `@semio-tech/framework-surface-rs:wasm` rebuilt.
- UI `ContextMenuController`: the active path is seeded on open and kept across live `items` republishes while it still names the same row (re-seeding on every republish undid Escape's submenu collapse, so a live menu could never be closed by keyboard); law added to `🖱️ContextMenu/🧪️tests/🧩️component` (9/9).
- Probe `🔍️suggestion-probe.ts` extended with the board scenario: **PASS=26 FAIL=0** (board: seed handles published, grip menu offers the suggest submenu, search starts + 6 candidates, row lists them, candidate 0/1 focus their OWN twin keys `…904`/`…905`, Escape×2 releases the search).
- Gates: 5d lib 589 passed / 2 failed — pre-existing import-media law + `window_kind_actions_scope_transform_to_3d_only`, caused by a PEER's uncommitted `🔌️plugin/🦀️.rs` change (15:16, `build_definition` now fans every top-level action onto every window kind; blame = uncommitted). 3d lib (filter) 144/1 (timing law). vitest suites green; renderer tsc 0 errors; publication audit 5d + 3d valid.

## Follow-ups
- Menu title reads "Vortex Menu"/"Handle Menu" in 5d (host title keyed by hit domain).
- Puzzle 2d keeps its own `openHandleSuggestions` popup; it could adopt the shared submenu module.

## Round 3 (15:4x–17:1x) — puzzle 3d verified live, accept reselect fixed
- Probe generalized to `--plugin=puzzle3d --port=6013`: reads the PERSPECTIVE pane (3d's first pane is the orthographic Top), dismisses the welcome tour, waits out shared-access verification before choosing the example, picks the 3d object at the camera target (its origin is a corner), polls the post-accept selection.
- Found live: 3d accept placed the object but never selected it — the retained `Puzzle3dAcceptSuggestionWork` final `Emit` carried no `interaction_writes` (the in-memory `accept_suggestion` path does). Fixed (keeps `object_id` to `PublishResult`, emits `InteractionWrite::replace(object, [placed])`, same as `addObjectKind`); law `accept_suggestion_appends_an_object_and_closes_the_menu` now asserts the selection and was shown to FAIL without the write.
- 3d open/close stay poll-only: live, the search starts on menu open without command-emitted effects.
- Final live probes: **5d PASS=26/0**, **3d PASS=14/0** (boot, pick, grip hover, suggest submenu, search starts on open, candidates before hover, row lists 8, candidate focus 0/1, accept places + selects, Escape releases). 3d `accept_suggestion*` laws 6/6.
