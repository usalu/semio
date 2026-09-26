# U5 — os `s` frontend: localized, accessible, interaction-friendly as a user experiences it

Slice U5, session 11, 2026-09-25. Continues U1 (progress/cancel/connection), U2 (touch/tablet/contrast/diagram a11y),
U3/U3b (`LocalizedLabel` mutation labels), U4 (pinch/diagram nav/contrast), AP1 (approval affordance). Sibling S15 owns the
per-plugin matrix, lazy install and the default-palette WCAG fixes. Ports: serves 6580–6589, hubs 8080–8089.
Captures: `wp-u5/generated/`.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Status

| # | task | state | evidence |
|---|---|---|---|
| 1 | mutation labels: German per plugin + live en/de history rows | **measured**: 1649 leaves / 105 crates handcrafted, native check sweep 14/14 green; live History rows en+de for dag, block, gis, note, layout, draw, forms, puzzle (post-restage) and writer, energy (pre-restage); raster one row "Hover" untranslated → fixed in source (framework label), pending describe | §1 |
| 2 | TaskManager real window + agent tool-call cancel | **measured**: Tasks window live en/de (sections, real actor row, named Suspend/Resume/Cancel); job ledger + guest cancel covered by an integration test (no live spawned job observed); agent Cancel on an approval-parked call now ends it **CANCELLED in 364 ms** (was: waited out the 120 s deadline) — gateway fix + gate `live-agent-loop` **22/22** | §2 |
| 3 | ShellSync localized + persistent hub indicator | **measured**: badge online → reconnecting 5.1 s after hub SIGSTOP, rAF max gap 33 ms (no freeze), back online 3.3 s after SIGCONT; de texts live | §3 |
| 4 | one approval affordance (modal retired) | **measured**: gate (e1) Approve Once in the conversation affordance (countdown 120), (e2) Deny → PERMISSION_DENIED channel=shell, (i18n) en — 22/22 on serve 6580; de proven by G10's run of the same gate (`wp-g10/generated/live-agent-loop-de-2.txt`) | §4 |
| 5 | tablet auto breakpoint, 3 viewports | **measured**: desktop/mobile/tablet detected, no horizontal overflow, every footer control reachable (tablet footer labels compacted) | §5 |
| 6a | dag horizontal two-finger pan | **measured**: two-point touch pan +90 px → camera x −82.6 (zoom 1.1), vertical +70 px → y −63.6 = −70/1.1 | §6 |
| 6b | Home table empty after sign-in | **two causes**: (i) C10 F3 — Home's unauthenticated `foldDirectoryEvents` retired, sealed pages are the only writer: remote create → row 244 ms after the hub answered, own create 513 ms, reload → rows in 4.3 s, 0 refusals (**measured**); (ii) ≥ 9 hub spaces faulted the window (`nodes 129 > 128`) → windowed table (UI contract `Table`/`TableRow`, SDK kit on the tree-window ledger, React grid on the tree-window observer) **landed + tested**, live 30-space proof pending W2's space rebuild | §6 |

## Session 12

Session 12 (2026-09-25 22:5x →), same slice. Serves 6580–6589, hubs 8080–8089, durable data `.🧬semio/🌐hub/s12-u5-*`,
captures `wp-u5/generated/` (expendable). Staged guests = W2's restage4 (18:55, space guest carries the windowed table).

| # | item | state | evidence |
|---|---|---|---|
| 12-1 | Home with ≥ 30 hub spaces: windowed table, no 128-node fault, keyboard + SR grid, en + de, sign-in → rows ≤ 1 s | **33 spaces measured PASS** (0 faults, grid/rows/columnheaders/status en+de, End/Home keys); **138 spaces: new ITEM fault found** (`items 4098 > 4097` at 53 nodes) → fixed in source (census + SDK item ledger, laws green), + cells localized; live re-proof waits on W2's space guest restage | §12-1 |
| 12-1b | C10 relays: Home after reload 1 of N; `/spaces/<id>` lands on Home; `tree window refresh failed … actor-activation.revoked` | **fixed + measured**: reload 4/4 → 6/6 rows (store revision); `/spaces/<id>` **5/5** in the space (was 1/3 — identity restore re-established Home and the route re-apply was dropped by a reentrancy guard → one serial session lane); revoked activation = typed retirement → **0 JS console errors** on both probes | §12-1 |
| 12-1c | canonical zero-touch serve on 7800: 0 network errors; ≥ 30 spaces live | **4 product defects fixed + measured** (agent-bridge / local-session typed "not offered", extension watch route, backbone 204): 0 HTTP 4xx/5xx, 0 JS errors; only the stale MCP gateway's refused bridge websocket remains (G10 / restart). Home 36–38 hub rows on 7800, 0 faults, keyboard + de, rows 661 ms after the hub answered | §12-1 |
| 12-2 | Tasks window with a real spawned job: progress, Suspend/Resume/Cancel, en + de | **measured PASS** (remodel reconstruction, a ≥ 60 s spawned run): live progress (36 refreshes / 12 s), Suspend → "suspended" 1.06 s, Resume → running again 1.5 s, Suspend again 1.5 s, **Cancel → row gone 0.54 s**, en + de. Two root fixes: spawned programs' own progress/completion passes were owned by the PROGRAM as presenter (never current → never painted); Cancel sat outside the narrow Tasks window. Also RTL-mangled right panels fixed; focused-program tools/ToolRun/examples in `s` | §12-2 |
| 12-3 | Puzzle rows en ↔ de + History sweep of all staged plugins for missing German | **measured PASS**: puzzle History 11/11 rows switch en ↔ de live incl. "Add Node↶ → Knoten hinzufügen↶"; static sweep 2801 mutation-label sites, **0 identical, 0 English leaks**; raster's `Hover` ships as "Darüberfahren" in the staged descriptor; per-kind live rows are S15's matrix (de 75/75) — not duplicated | §12-3 |
| 12-4 | Customization persists across reload and devices (hub-backed shared / local-only) | **reload: measured PASS** (appearance, language, keybinding override, saved named layout, dock panels + tab); **keybinding capture was broken** (a modifier's keydown ended the capture — no chord with a modifier could be recorded) → **fixed** + laws; **second device: FAIL for all** — every preference is browser-local; the hub has no per-user preference store → design + request filed (hub route = H9 / freeze) | §12-4 |
| 12-5 | Remaining unowned UX items of the os-frontend audit | **taken + closed:** audit §4 #10 (Home ≥ 9 live), #13 (layout persistence measured), #14 (live job row in Tasks), #15 (puzzle en/de, raster Hover); #11 verified fixed in source (gateway `ApprovalWithdrawn`); plus found + fixed: canonical-serve 4xx/5xx ×4, keybinding capture, Hotkeys German names, Tasks overflow, RTL panels, hub overlay, spawned progress owner. **Left:** #6 (H9), #7 (hub install of an unregistered plugin — unowned, large), #12 (WG7), Home viewer lists no hub rows (needs a guest projection feed) | §12-5 |

### Session 12 log

- 23:00 read preambles 11/12, this report, the S11 os-frontend audit. Nothing of mine running; `wp-u5/generated/` was swept
  (all session-11 captures gone; the report keeps the measured numbers). W2 is releasing catalog B2 (chain 83333).
- 23:01 hub **8080** up: hold `u5-hub-hold.ts` pid **90570**, os-hub **90572** (binary = copy of H9's 17:37 current-tree build
  → `.🧬semio/🌐hub/s12-u5-bin/os-hub`, data `.🧬semio/🌐hub/s12-u5-hub-8080`, state `…/s12-u5-state-8080`). ada + bo provisioned
  with `OS_HUB_DATA=… os-hub credential set --email … --display-name …` (password on stdin). Seeded 32 spaces (`u5-hub-seed.ts`).
- 23:03 serve **6580** (`S_OS_PORT=6580 S_HUB_URL=http://127.0.0.1:8080 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev` in
  `🧑‍💻dev/📦️packages/🟦️typescript`; the root `📜️script.ts` has no `serve`): script pid **92892** → vite **93215**, log
  `.🧬semio/🌐hub/s12-u5-logs/serve-6580.txt`, "[fresh] 60 staged components match their sources and the activation receipt".
- 23:05 run A (33 rows) PASS; 23:07 run B (138 spaces) ITEM fault → 23:10–23:50 fix + laws (§12-1); 23:5x W2 request.
- 00:xx hub/serve restarted with `setopt no_bg_nice` (rule 17): hold **64020** → os-hub **64023** (8080), serve script
  **64021** → vite **64063** (6580). Focused-program chrome + Tasks tool-run lane (§12-2); C10 relay → Home reload fix
  (§12-1).
- 01:40 C10 relay (revoked refresh) → retirement law; 01:48 found `/spaces/<id>` regressed to 1/3 → session lane;
  01:56 laws green; 03:4x (after the usage-limit pause) route 5/5, reload 6/6, 0 JS console errors.
- 04:0x canonical serve 6581 on 7800 (user1 + 32 seeded spaces) → 4 optional-endpoint defects fixed, 0 HTTP 4xx/5xx; Home
  ≥ 30 live on 7800. 04:3x G10 S4 relay (hub overlay / lost space) fixed + live. 05:xx Tasks: spawned progress owner +
  row overflow fixed, Suspend/Resume/Cancel live en + de. Item 3 (puzzle en/de, German sweep), item 4 (customization +
  keybinding capture), Hotkeys German names. Coordinator decision → per-user preference lane: TS half landed + laws; Rust
  half prepared (`patches/u5-preference-lane-apply.py`), held by the hard guest freeze (rule 20).
- Processes now: hub hold **64020** → os-hub **64023** (8080), serve **64021** → vite **64063** (6580) — kept for the
  preference-lane proof; standard serve 6581 stopped.

### 12-1 Home with many hub spaces

**Run A, 33 rows (32 hub + demo), restage4 guests** (`u5-home-table-probe.mjs … s12a 32`, `generated/u5-home-s12a.json`):
`role=grid` "Studios", `aria-rowcount` 34, 7 `columnheader`s, window `total 33 offset 0 length 33`, **0 faults** (no 128-node
fault any more), End → focus on row 32 in 82 ms, Home → row 0, status "Rows 1–33 of 33"; German: headers
"Name/Art/Sichtbarkeit/Mitglieder/Aktualisiert/Herkunft/Aktionen", status "Zeilen 1–33 von 33". Grid 1.6 s after the sign-in
click (includes the auth mint; the ≤ 1 s-after-hub-answer measurement is now in the probe — `mintToRowsMs`,
`lastDirectoryAnswerToRowsMs` — and runs with the re-proof).

**Run B, 138 hub spaces** (32 + 106; 63 creations of the 128-burst were refused by the hub's own `server.rate-limit
directory-command` — expected, the seed ignored the 429s): **fault** `s-home-main [reconciler g13]: Credits { nodes: 53,
items: 4098 }, max_items 4097` → the surface kept its previous paint (demo row only). The windowed table fixed the NODE bound;
a Home author row (6 cells, 5 row actions + activation, each binding with its own `spaceId` map) costs ~102 reconciler items,
so a first paint of 48 default rows overdraws the per-surface ITEM ceiling long before 128 nodes. The ARIA snapshot
(Playwright `ariaSnapshot`, third-party oracle) of the painted grid: `grid "Studios"` → `rowgroup` → header `row` with 7
`columnheader`s → `row "Demo Studio atelier private 1 0 local open: Demo Studio"` with `gridcell`s and a `button "open: Demo
Studio"` → `status: Rows 1–1 of 1`; ArrowRight moved focus onto that button, Escape back to the row.

Also seen: row cells were wire ids, not words — Kind `atelier`, Visibility `private`, Updated = raw epoch ms for hub rows
(`1790370316130`) / `0` for local drafts; the Space index table was English-only (columns, "Artifacts", "Open", "Create Artifact").

**Fixes (landed, compile-atomic, laws green):**

* `semio-framework-ui-runtime` `♻️reconcile`: `pub fn surface_subtree_items(&TreeNode)` — the reconciler's own per-record
  semantic census + `SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM` (1/2, the allocation items the copy/bindings/assembly
  stages charge on top) + one per extra page of an oversized owner; `SURFACE_RECONCILE_MAX_ITEMS` public. Law
  `subtree_item_census_bounds_the_items_a_fresh_reconciliation_consumes`: tree pick rows, bound tree rows (4-entry maps),
  light and Home-shaped table rows and their containers — priced ≥ consumed and ≤ 2× consumed against the REAL reconciler
  (measured consumed/census 1.17–1.35). One census cursor reused per subtree. `generated/u5-s12-test-runtime-census.txt` 2/2
  (with the four-section size law); full runtime lib 122/125 — reds: the two known `runtime_tree_retirement_*` (§7) and
  `every_large_tree_cursor_slice_stays_below_eight_milliseconds` (timing under load 26; passes alone).
* SDK `semio-framework-plugin` `TreeWindows`: an ITEM ledger beside the node ledger — `TREE_WINDOW_BODY_ITEM_BUDGET =
  SURFACE_RECONCILE_MAX_ITEMS − 1 − TREE_WINDOW_FIXED_NODE_HEADROOM × TREE_WINDOW_FIXED_NODE_ITEMS(32)` = 3328; every
  materialised row is priced by its subtree (nested windows replaced by their parent row's price); a row that does not fit
  ends the window early and gives back its and the unbuilt rows' records/items (also on `ui.fixed-capacity` / full children,
  which used to leak the granted records). Laws `table_kit_ends_a_window_of_heavy_rows_inside_the_body_item_budget` (100
  requested Home-shaped rows → shortened, stamp still `total 500`, records refunded), `table_kit_serves_a_whole_window_of_light_rows`
  (60/60); window-kit suite 19/19; full SDK lib 837/839 — reds `tool_run_*` (2 ms timing law under load; an arena-contention
  `ui.fixed-capacity` that passes alone and in its group) — `generated/u5-s12-test-sdk-*.txt`.
* Space plugin: `HomeSpaceRow { kind: SpaceKind, visibility: SpaceVisibility, updated_ms: Option<u64> }` (were strings);
  `HomeSpaceRow::cells(&HomeTableLabels)` shared by the Home editor and viewer; `HomeTableLabels` + `kind_*`, `visibility_*`,
  `updated_never` (en/de). Space index: `SpaceIndexTableLabels` (app_labels en/de) replaces the English
  `SPACE_INDEX_TABLE_COLUMNS` (columns, table name, actions header, "Open"/"Öffnen", "Create Artifact"/"Artefakt erstellen").
  `utc_minute_text(ms, Locale)` ("2026-09-25 21:05 UTC" / "25.09.2026, 21:05 UTC"; the guest has no time zone, so it says UTC)
  and `rfc3339_utc_epoch_ms` (local `saved_at`) in the neutral snapshot schema; language-agnostic fixture
  `📸️snapshot/🧫️fixtures/🕰️utc-minute/🔣️.json` (7 instants incl. leap days, 2100, 9999; accepted/refused RFC 3339) read by
  the Rust law and by the TS oracle `📸️snapshot/🧪️tests/🔬️unit/🟦️.ts` (`Intl.DateTimeFormat` UTC + `Date.parse`; run: ok).
  Tests: home 93/95, space 89/106 — every new/changed law green; reds are peer state (`retained_config_cancel…`,
  `direct_owner_descriptor…`, 17 space-index command relays on the registry-less `context::new_app()` — "registry declares
  createArtifact", effects 0), none in a touched path. `cargo check -p semio-s-plugin-space --lib --tests` green.
* W2 request filed (`wp-w1/requests/u5.txt`): space guest rebuild + restage.

**Home after a reload listed 1 of N spaces (C10 relay, found + root-fixed, measured).** Reproduced on hub 8080 with bo (5
spaces, `u5-hub-seed.ts --email=bo@example.org`, probe `u5-home-reload-probe.mjs`, `U5_EMAIL=bo@example.org`): fresh sign-in
"Rows 1–6 of 6"; after `reload` the grid said `aria-rowcount 7` but **"Rows 1–1 of 6"** for good, 2 of 3 runs. Not the fold
(the guest had all 6 entries — `total 6`) and not the stale guest: the host asked for `rows 1`. Traced with temporary logs
(removed): the table's tree-window observer measured once (total 1, before the persisted projection landed), reported
`{offset 0, rows 1}`, and never measured again — its effect is keyed on the store revision, and a window body's
`UiDocumentStore` comes from `createBuiltNodeStoreCacheV1`, which reloaded every new BuiltNode **at revision 0**, so
`useUiDocumentRevision` stayed 0 for the store's life. Fix (`🏛️ShellHost` `createBuiltNodeStoreCacheV1`): every reload loads
the next revision of that store; law in `🧪️tests/🔬️engine-contract` (the existing store-cache law now also asserts the
revision grows on reload). After: **4/4 reloads → "Rows 1–6 of 6"** at +2 s (`generated/u5-home-bo-reload-fix*.json`).
Found in the same probe, still open: a Viewer → Editor role round trip leaves Home with 0 hub rows (the viewer lists only
local rows; the re-created editor never re-reads the directory) — next.

**`/spaces/<id>` hard load ended on Home (C10 relay; found + root-fixed, measured).** Re-run at 01:4x (after my 01:10
11/11): `u5-space-route-probe.mjs … revoked-b 3` → attempt 1 never mounted the space (90 s), attempt 2 mounted it and was
back on Home 1.5 s later, attempt 3 stayed; the URL named the space every time (`generated/u5-space-route-revoked-b.json`,
a third instance created in the failing loads). Cause: the hub identity is restored mid-load, `ShellHost`'s identity
effect re-establishes the session (always the LANDING app in `s`), and the route effect's re-application was thrown away
by `applyShellUriDepthRef` — a reentrancy guard that DROPPED every route requested while another application was in
flight (the first route was still awaiting its document open). Fix: `createShellSessionLaneV1` (`🔀️surface-switch`): one
serial lane for route applications and identity re-establishments, a route waiting at the end of the lane absorbs later
ones (latest wins), nothing dropped, failures settle only their own requests, always the newest `applyShellUri`; the
identity effect runs on the lane; `establishPrimarySession` clears the open space/instance (Home has none) so the route
re-opens behind it. Law `shell session lane` over the language-neutral fixture `🏛️ShellHost/🧫️fixtures/🧭️session-lane/🔣️.json`
(6 scenarios) with a `p-limit` (concurrency 1) oracle — surface-switch file 2/2 (`generated/u5-s12-vitest-surface-switch.txt`).
After: **5/5 loads in the space**, 7.8–11.0 s, 0 faults (`generated/u5-space-route-lane-a.json`).

**`tree window refresh failed … actor-activation.revoked` (C10 relay; root-fixed, measured).** A turn whose captured
activation was revoked under it (session switch sealing Home, hot swap) is cancelled WITH that activation:
`isPluginInstanceRetiredV1` now classifies the shard client's `actor-activation.revoked` as a typed retirement (other
activation faults stay loud), and the tree-window scheduler drops a retired refresh like every other lane. Law in
`🧪️tests/🔌️plugin-runtime` ("cancels a turn whose actor activation was revoked under it…") — 11/11 retirement laws
(`generated/u5-s12-vitest-plugin-runtime.txt`). Also: a sealed-instance effect pass that carries **zero** effects no longer
logs a "dropped host effects … (0 effect(s))" warning (30 per 5 loads). After, both probes: **0 JS console errors, 0
warnings** (`u5-home-lane-a.json`, `u5-space-route-lane-a.json`). Remaining console lines are browser resource errors of
my serve/hub, not app errors: `/__semio/agent-bridge` 404 (the documented "no gateway" answer, polled 4/8/16/30 s — a
204 would keep it out of the console; G10's area), `/🧩️extension-modules/watch` 404, `/_semio/dev/local-session` 404,
`/_semio/hub/spaces/<id>/artifact-creations` 503 (my hub 8080 holds no creation catalog).
Engine suites after the ShellHost changes: surface-switch, directory-home-bootstrap, session-authority-notice,
engine-contract 734 tests — 2 source-text laws in `🪟️spawned-program-session` followed my `programOfWindow` /
`refreshSpawnedUiPass` refactors and were updated to assert the one resolution path; that file 45/45; renderer `tsc` clean.

**Canonical zero-touch serve on hub 7800 (B2) — network errors (coordinator follow-up; measured).** Standard `s` serve
(`S_OS_PORT=6581 SEMIO_PLUGIN=s SEMIO_RENDERER=react S_HUB_URL=http://127.0.0.1:7800 S_DATA_DIR=…/s12-u5-serve-6581`, no
private rendezvous dir, serve-only — no activation), user1 with 32 seeded spaces (`U5 Home 01…32`, 36–38 hub rows).
Four product defects answered 4xx/5xx on every boot and are fixed (each with a language-neutral law + third-party oracle):

| request | was | now | law |
|---|---|---|---|
| `GET /__semio/agent-bridge` (both shells poll it from boot) | `404` "no live gateway" | `200` `AgentBridgeOfferAnswerV1` — `{schema:"semio.os.agent-bridge-offer/v1", offered:false}` or the offer; parser accepts exactly that schema | `🔗️AgentBridge/🧫️fixtures/🛰️offer-answers/🔣️.json` + `🧬️schema.json`, Ajv oracle; AgentBridge suite 69/69 |
| `GET /_semio/dev/local-session` | `404` "local-session unavailable" | `200` `LocalHubSessionAnswerV1` (`📇️directory/🎫️local-session`) — typed "not offered" or a session; ShellHost decodes with the shared parser | `📇️directory/🎫️local-session/🔣️.json` + `🧬️.schema.json`, Ajv + fast-deep-equal oracles (HubSignIn file 38/38) |
| `GET /🧩️extension-modules/watch` | `404` — the `pre` static mount of the same route shadowed the (unmarked) extension store | store is `pre` and listed first → `200` SSE snapshot | "extension store route precedence" (real plugins, vite's `sortUserPlugins` order; also pins that an unmarked store 404s) — store suite: new law ✓, 2 pre-existing reds (`🔌️plugin/🧩️extension/*.json` fixtures no longer exist, not touched) |
| `GET /semio-backbone?…documentId=os.config.identity` (fresh data root) | `404` | `204 No Content` = nothing written yet; readers take 204 (and a host without the route, 404) as absent | `🧪️backbone-envelope-io` "reads 204 No Content … as null without retrying" (5/5) |

After (restarted 6581): reload probe 36 hub rows at +5 s, route probe **3/3** in the space — **0 HTTP 4xx/5xx, 0 JS errors**
(`generated/u5-home-std-7800-b.json`, `u5-space-route-std-7800-b.json`). **One source remains, not in the shell:** the
semio MCP gateway of the Claude Code session (`dev mcp stdio os`, pid 88810, started 19:36, 8.6 h old) publishes a live
offer `ws://127.0.0.1:59773/bridge`, accepts the TCP connection and never answers the websocket handshake (`curl` upgrade:
no response; `lsof`: accepted sockets left `CLOSED`) — every shell redial is a browser `WebSocket … failed` console line
(11 per probe). Owner: the gateway (G10); a restart of that MCP server or a gateway fix removes it — the shell cannot hide
a browser-level socket error.
**Home ≥ 30 on 7800** (`u5-home-table-probe.mjs … std-7800-b 36`): `grid "Studios"`, `aria-rowcount 39`, window total 38,
**0 faults**, End → last row in 118 ms, ArrowRight → "open: U5 Home 03", German headers/actions; **rows 661 ms after the
hub's directory event page answered** (the hub itself was slow: session mint 8.5 s, event page 3.9 s under load). The
cells are still wire ids + epoch ms on 7800: the staged space guest predates the localized cells (serve log
`[stale] space: source-changed`; W2 request filed).

**G10 S4 relay — hub pane lost its space; "Open Hub and Spaces" set `/hub` with no overlay (root-fixed, measured).**
(2) `/hub` is an overlay, but after the session lane landed it was queued like a session route and waited behind the
in-flight space route, whose job awaited the space index's document opening (bounded only by the opening deadline).
Fix: `shellRouteIsOverlayV1` (`🔀️surface-switch`, with `SHELL_HUB_ROUTE` moved there): the route effect opens/closes
the overlay on the spot and hands only session routes to the lane; a route's document opening is started in lane order
but no longer awaited inside the lane. (1) The hub pane read `openSpaceIdRef.current` during render, so a space
re-opened after an identity re-establishment (which briefly returns to Home) never reached an already-open pane.
Fix: `openSpaceId` state written with the ref by one `rememberOpenSpaceId` (all six writers); the pane renders the
state. Laws: `shell session lane` + "opens the hub overlay on the spot and queues only session routes — equal to a
WHATWG URL oracle" (9 route rows in `🧭️session-lane/🔣️.json`), source law "renders the hub workspace's active space
from state…" (`🪟️spawned-program-session` 46/46). Live on 7800 (`u5-hub-overlay-probe.mjs … a`, user1, standard serve
6581): in space A → palette `command.os.os.openHub` → `/hub` overlay shown, agent pane names the space; close → back in
space A; palette hub 150 ms after a route change to space B → overlay shown at once and still after 8 s, never "Open a
space to manage…" (`generated/u5-hub-overlay-a.json`).

### 12-2 Tasks window and a real long-running job

**Right-edge panels mangled prose (found + fixed, measured).** `Panel` sets `dir="rtl"` on a right-anchored panel to
mirror its chrome, and every custom window body inherited it: Tasks read ".No task is running", its actor table ran
right-to-left and clipped its first columns; Chat read ".along with the agent's replies…", ".Not connected to an agent —
messages cannot be sent", placeholder "...Tell the agent what to do" (`wp-u5/u5-panel-dir-probe.mjs`,
`generated/u5-panel-dir-before*.png/json`: text direction `rtl`). Fix: `🌳️Tree` renders `emptyState` — the slot every
custom panel body (Tasks, Chat, Marketplace, Sync…) is mounted through — in a `data-slot="tree-empty-state"` wrapper with
`dir="auto"` (content reads in its own direction; tree rows and chrome stay mirrored as designed on 09-22). After
(`generated/u5-panel-dir-after*.png/json`): Tasks "Running tasks" / "No task is running." `ltr`, Chat prose and placeholder
`ltr`, History tree rows still mirrored (`rtl`, unchanged). Law: `🌳️Tree/🧪️tests/🧩️component` "Tree empty state reading
direction" (Testing Library role/text queries) — file 36/36 (`generated/u5-s12-vitest-tree-only.txt`). Note: the big
`🧪️owned-locale-detector-retirement` suite has 11 pre-existing reds, two of them stale expectations that tree roots carry
`dir="auto"` in mirrored panels (the 09-22 commit `f2585a4fdb` switched tree roots to the flow direction without updating
them) — not touched here.

**No reachable long spawned job (measured).** Probes `u5-job-ledger-probe.mjs` (hooks the page's own job-ledger module
instance — the exact vite URL from the resource timings — and timestamps every row) and `u5-live-probe.mjs jobs`:
spawning fem2d / fem3d editors drove **zero** ledger rows (`generated/u5-job-fem2d-a.json`, `u5-live-jobs-fem.json`); the
fem mounted-analysis `SpawnJob` is emitted from `pending_effects`, which the guest drains only after a mutation, and a demo
model's revision completes inside one step slice. Framework reserved verbs (undo, select…) spawn jobs the host drives
outside the ledger by design (two dummy steps). The real long-running work — tool runs (`s.wfc.*.fill.run`,
`s.puzzle.*.fill.run`, `generation*.previewEval.run`, `remodeling.reconstruction.run`) — is driven in-guest
(`ToolRunEffect::Schedule/DriveOneUnit`), and in `s` it is **unreachable**: `ShellHost` resolves mode tools from
`session.app` only (`resolveModeTools(session?.app, activeModeId)`; the footer has no Tool category for a spawned program)
and `refreshSpawnedUi` requests "no panels" for spawned programs, so the ToolRun panel never exists for them; the palette's
`toolRunStart` is refused "needs a toolId argument or an active tool" (`generated/u5-toolrun-wfc2d-a.json`).

**Focused-program chrome in `s` (coordinator-approved; landed, measured).** One resolution path for the session app and
spawned programs, no special case: `ShellHost` resolves mode tools, the Tool category, the ToolRun panel, the footer
panels, the example picker and tool arming from the FOCUSED program (`focusedApp`, `focusedModeId`, `focusedPanelUiByKey`,
`focusedToolMeasuresByToolId` via `🪟️spawned-program` `programEntriesV1`/`withProgramEntriesV1`/`programKeyedEntriesV1`);
`onAction`'s `programOfWindow` routes utility activation and tool arming to the program owning the window;
`refreshSpawnedUi` now requests the program's panels and tool measures (one pass in flight per program, coalesced), and
slices of other programs are preserved instead of being replaced. Fixes found on the way: `PluginRuntime.refreshUi` computes
the missing surfaces at SUBMISSION (overlapping refreshes answered "stopped without publishing requested UI surfaces"),
and the focus-change panel refresh no longer races the spawned refresh.
Tools proven live in `s` (probe `u5-toolrun-probe.mjs`, spawn → arm tool → Start → samples):

| plugin | app | tool | result |
|---|---|---|---|
| wfc | `s.wfc.wfc2d@1/*#editor` | fill | ToolRun "Finalized · Complete (10/10)", 3 of 3 steps (`u5-toolrun-wfc2d-g.json`) |
| wfc | `s.wfc.wfc3d@1/*#editor` | fill | "Finalized · Complete (10/10)", 3 of 3 (`u5-toolrun-wfc3d-a.json`) |
| puzzle | `s.puzzle.puzzle3d@1/*#editor` | fill | "Complete, ready to finalize", 0 of 100 objects at the sample (`u5-toolrun-puzzle3d-a.json`) |
| remodel | reconstruction editor, example "Synthetic Orbit" | reconstruction | runs ≥ 60 s: "Ingesting frames (1/10): 3 → 31 of 36 decisions" (`u5-toolrun-remodel-f/g.json`) |

**Tasks window, tool-run lane (landed; partly measured).** `🧵️TaskManager` lane "toolRun" ("Tool run"/"Werkzeuglauf"):
each live ToolRun group of the focused program becomes a task (`toolRunPanelTasksV1` reads the ToolRun panel body;
`toolRunTasksV1`, id `toolRun:<spawnedId>#<run>`) with a determinate bar (`aria-valuenow/max`), elapsed time, and named
Suspend/Resume/Cancel buttons that fire the run's own Pause/Resume/Abort bindings; state "suspended"/"Angehalten".
Laws: TaskManager + tool-run-panel 27/27 (`generated/u5-s12-vitest-tasks.txt`, fixture `🏃️running-tasks.json` +
`⏯️tool-run/🪧️panel-running.json`). Live (remodel, `u5-toolrun-remodel-g.json`): row "running · Ingesting frames (1/10):
3 of 36 decisions (8 %) · 1 s" with "Suspend Reconstruction"/"Cancel Reconstruction" → **Suspend: row "suspended" in
608 ms**, buttons become Resume/Cancel. **Open:** Resume is applied in the guest but the row stays "suspended" and the
ToolRun panel stays at 31/36 while the guest advances — the spawned program's refreshes do not land while its run is
live; the Cancel click did not reach an abort dispatch.

**Spawned progress never painted (found + root-fixed, measured).** Hop traces read from the page (`performance`
measures `semio.hop.*`, added to the probe per control step): after Resume the guest ran **618** operation-drain turns in
12 s and the shell asked for **one** refresh (`generated/u5-toolrun-remodel-h.json`). Cause: every pass a PROGRAM runs on
its own — its operation progress, operation completions, spawned-job progress, deferred effects — captured the program
itself as the effect's PRESENTATION origin (`captureEffectOwner(target, captureDialogOrigin(target))`), and
`isCurrentEffectOwner` requires the presentation to be the primary session's current origin, so for every spawned
program the pass returned before its refresh. Only an unrelated action's own completion ever repainted a spawned program
(which is why Suspend "worked" and Resume did not). Fix: `shellEffectOwnerIsCurrentV1` (`🗨️dialog-origin`, pure):
presentation = the primary session, source = the primary app or an exact live spawned app; ShellHost
`captureProgramEffectOwner` for all seven program-driven lanes. Law in `🔬️engine-contract` ("admits a spawned program's own
progress and completion passes only when the primary session presents them": program-presented = the defect → refused,
retired program → refused, replaced primary → refused) — 15/15 dialog-origin laws.
**Cancel outside the window (found + fixed, measured).** Playwright: `click: … element is outside of the viewport`
(`u5-toolrun-remodel-k.json`, screenshot `u5-toolrun-remodel-k.png`): the task row put bar + long progress text
(`shrink-0`) + Suspend + Cancel on one unwrapping line, so in the ~300 px Tasks window Cancel was clipped; the actor
table (12 columns) was clipped too. Fix: bar spans the row; a wrapping footer holds the progress text (`flex-1`, wraps)
and an actions group (right-aligned, wraps under the text when narrow); the actor table is a named, focusable
`overflow-x-auto` region. Laws (Testing Library): "keeps every task control in one group after a wrapping progress
line…", "scrolls the wide actor table inside a named, keyboard-focusable region…" — TaskManager + tool-run panel 29/29.
Live row geometry (de): row 1301–1593, "Rekonstruktion abbrechen" 1503–1592 inside the clip 1298–1596
(`u5-tasks-row-remodel-de-b.png`).
**After, live (6580, remodel "Synthetic Orbit", `u5-toolrun-remodel-l.json`):** Suspend → suspended in 1.06 s (25/36);
Resume → running 1.5 s later, then 28 → 32 → 35 → "Extracting features 3 … 35" → "Matching features 22 … 59 of 374"
(36 `refresh.turn` in 12 s); Suspend → 1.5 s; **Cancel → task gone in 0.54 s**; 0 faults. German
(`u5-toolrun-remodel-de-a.json`): "Rekonstruktion anhalten"/"Rekonstruktion abbrechen", "Bilder werden eingelesen (1/10): 11
von 36 Entscheidungen (30 %) · 1 s", suspend 1.07 s, resume 1.5 s, cancel 0.55 s.

### 12-3 Puzzle rows en ↔ de and the German sweep

**Live (standard serve 6581, `u5-history-locale-probe.mjs … s12-d puzzle`):** spawn puzzle, drive `addNode` + undo/redo,
read every History row, switch the shell to German through Settings, read the same rows: **11/11 re-rendered in German, 0
untranslated** — Toggle Panel → Panel umschalten, Activate Window → Fenster aktivieren, Switch Panel Tab → Panel-Tab
wechseln, **Add Node↶ → Knoten hinzufügen↶**, Clear Selection → Auswahl aufheben, Undo → Rückgängig, Redo → Wiederholen
(`generated/u5-history-s12-d.json`). The session-11 "puzzle not measured" gap was the probe: while a spawned program is
focused, the footer Settings panel opens on the PROGRAM's own settings sub-tab (puzzle: Fill · Count, Suggestion Offset, …)
and the language lives under the shell's `framework.settings.general` sub-tab (`u5-settings-focus-probe.mjs`,
`u5-settings-puzzle-c-*.png`) — by design, not a defect; the probe now selects General.
**Raster "Hover":** the staged raster and puzzle descriptors carry the framework hover verb as "Hover"/"Darüberfahren" (4 and
12 pairs, served `🔣️.json`).
**Static German sweep** (`u5-label-audit.py`, every `MutationKind`/`SemanticMutation`/`CompositeMutationKind::label` in
`✏️s` + `🧰️framework`, `generated/u5-s12-label-audit.json`): **2801 sites — 2699 ok, 0 identical en = de, 0 English leaks**;
76 flagged "camel" are unit/abbreviation English sides (`M_Ed [kNm]`, `SfM`) with proper German, 26 delegate to helpers.
Per-kind live History rows are S15's matrix (editors de 75/75, "every History row German") — not duplicated here.

### 12-4 Customization across reload and devices

Probe `u5-customization-probe.mjs` (device A = one browser context: sign in, set Dark, Deutsch, record a keybinding
override on the first Hotkeys row, save a named window layout "U5 saved layout", open panels; reload; device B = a fresh
browser context signing in as the same hub user). Captures `generated/u5-customization-{c,d,e}.json` + screenshots.

| customization | persisted in | A after reload | B (second device, same user) |
|---|---|---|---|
| appearance Dark | `semio.os.config` › `os.config.ui-preferences` event log | **dark ✓** | light ✗ |
| language Deutsch | same log | **de ✓** | en ✗ |
| keybinding override (Undo → ⌃⌥K) | same log | **⌃⌥K ✓** (after the fix below) | ⌘Z ✗ |
| saved named layout | `semio.os.config` › `namedLayouts` | **listed ✓** | absent ✗ |
| dock panels (visible anchor + tab path) | `semio.os.config` › `dockUi.apps[appId]` | **restored ✓** | — (device-local by nature) |

**Keybinding capture could not record any chord with a modifier (found + fixed).** Settings ▸ Hotkeys ▸ Record ended the
capture on the first keydown that was not a complete chord — and every chord a hand types starts with the modifier's own
keydown (`Control`), so pressing ⌃⌥K left Undo at ⌘Z (`u5-customization-b.json`: capture state "Press keys…", override
unchanged). Fix (`📌️ChromePanels`): `keybindingCaptureStepV1` — Escape cancels, a lone modifier keeps waiting, anything
else records `modifiers+key` in the dispatcher's spelling. Laws: 7 rows in `📌️ChromePanels/🧫️fixtures/⌨️keybinding-capture/🔣️.json`
and a `@testing-library/user-event` oracle typing ⌃⌥K / ⌘⇧Z modifiers-first on a focused button (ChromePanels 9/9). Live
after: Undo reads "⌃⌥K" and survives the reload.
**Hotkeys rows stayed English in German (found + fixed, measured).** Rows were `uiDataLabel(humanizeControlId(id))`
("Undo" in a German shell). Now: an app action / OS command is named by its manifest label in the live language
(`controlKeybindingLabel`, ShellHost), a shell control by its chrome bundle entry (`shellLabelIfDefined`, via a new
`UiI18nPort.exists`), and only an id nothing names is humanized. Law "keybinding row names" (ChromePanels 10/10); live
(`u5-customization-f.json`): "Rückgängig ⌃⌥K Aufnehmen Zurücksetzen".

**Per-user preference lane (coordinator decision 04:xx: U5 builds it end to end on the directory lane).** Status:
TypeScript half **landed + laws green**; Rust half **prepared, held by the hard guest freeze** (rule 20: the directory
schema Rust twin is guest-linked, and every hub edit names its new variants) —
`.tmp-ticket/wp-u5/patches/u5-preference-lane-apply.py` (`--check`: all anchors present in 12 files), to apply the moment
W2 announces the `--packages all` publish, then cargo check + hub tests + live proof.
* **Schema-first data classes:** `🎚️config/🧬️schema/🎨️ui-preferences/🔣️.json` `$defs.DataClasses` (every key:
  `persistedShared`, except `layout` = `persistedLocalOnly`, the device class) and the mutation schema's
  `$defs.PreferenceKeys` (mutation → key); TS readers `UI_PREFERENCE_DATA_CLASSES`, `UI_PREFERENCE_MUTATION_KEYS`,
  `uiPreferenceMutationDataClassV1`; fixture `🎨️ui-preferences/🧫️fixtures/🗂️data-classes/🔣️.json` (Rust twin law in the
  prepared patch reads the same files).
* **Directory contract:** event `user.preference-recorded { userId, schema, mutation }` (principal-only, no space) and
  command `record-user-preference { schema, mutation }` in the JSON schema (`📇️directory/🧬️schema/🔣️.json`), the TS twin
  (`validUserPreferenceRecordV1`, page parser, command canonicalizer) and — prepared — the Rust twin; admission fixture
  `📇️directory/🧬️schema/🧫️fixtures/🎚️user-preference-record/🔣️.json` (15 rows, both twins). The hub stays domain-neutral:
  `schema` names the vocabulary, `mutation` is an opaque bounded JSON object.
* **Lane separation (why):** the Home guest folds `/directory/event-page/v1` with its compiled Rust twin; an unknown event
  kind there would break Home until a restage. So the default page and socket fold skip preference events, and
  `GET /directory/preference-page/v1?after=` (same page machinery, receipts and seq) carries only the caller's own
  (prepared hub patch: `DirectoryEventLaneV1`, `preference.record` / `preference.read` in `HubAccessPolicyV1` for
  `authenticated`, 4 policy vectors).
* **Client:** `DirectoryClient.preferencePage(after, userId)`; worker lane (`preference-lane-open/record/close`, one page
  in flight, wakes when the directory head passes the frontier, own socket); `🎚️UiPreferences` engine:
  `commitUiPreferenceOnLaneV1` (local-first; a shared change is queued with its own id + request id, a newer change of
  the same slot supersedes a pending one), `foldUiPreferenceLanePageV1` (hub order; own changes acknowledged; pending
  re-applied on top — converges on the hub's last write per slot); ShellHost opens the lane while a verified hub session
  stands and re-sends the outbox after every page (offline edits reach the hub on reconnect).
* **Laws (renderer long suite, `🎚️UiPreferences` 4/4):** data classes vs fixture with an Ajv oracle over the mutation schema;
  6 two-device scenarios (`🧫️fixtures/🌐️preference-lane/🔣️.json`: live share, device-local layout stays, offline
  reconcile, both-offline convergence, idempotent resend, superseded pending) — each device equals the fixture AND an
  independent hub-order replay oracle, every recorded event valid under Ajv against the directory JSON schema; refusal
  rows + the admission fixture checked against Ajv. Renderer `tsc` clean; worker/client/schema `tsc` clean.

**Devices — before the lane.** Every preference lives in the browser's `semio.os.config` (`OsShellConfig` over the
StoragePort; the wgpu shell reads the same key); nothing is written to the hub, and the hub has no per-user store (routes
`/auth/*`, `/directory/*`, `/spaces/{id}/…` only). Proposed split and lane (not built — it needs a hub route, H9's area,
and lands after W2's `--packages all` publish because of the ABI freeze):
* **persisted shared (per user, hub-backed):** appearance, theme id + custom themes, language, terminology, driver +
  custom drivers, keybinding overrides, saved named layouts.
* **persisted local-only (per device):** chrome layout (desktop/tablet), dock panels and sizes, window panes.
* **Lane (CQRS/event-sourced, local-first):** the existing `UiPreferencesConfigMutation` log stays the write model; each
  local commit is appended to a per-user hub event log (`POST /auth/sessions/me/preferences/events`, idempotent by event
  id) and devices fold `GET …/events?since=<seq>` into their local log (offline edits queue and replay; last event wins per
  field, which is what the fold already does). No CRDT, no CRUD.

### 12-5 Remaining unowned items of the os-frontend audit (`📓️audit-s12-os-frontend.md` §4)

| # | item | taken? | result |
|---|---|---|---|
| 10 | Home ≥ 9 spaces live proof | yes | 36–38 hub rows on 7800, 0 faults, keyboard + de (§12-1) |
| 11 | approval affordance not cleared after a cancel | checked | fixed in source by its owners: the gateway sends `ApprovalWithdrawn` (`🌉️mcp/🛡️policy/🦀️.rs`), the shell drops the pending approval on `approvalWithdrawn` (`🔗️AgentBridge`); not re-measured live (needs an agent run) |
| 13 | layout persistence unknown | yes | measured: dock panels + tab, saved named layouts, appearance, language, keybindings survive a reload; cross-device via the preference lane (§12-4) |
| 14 | no live spawned-job row in Tasks | yes | tool-run rows live with Suspend / Resume / Cancel, en + de (§12-2) |
| 15 | puzzle en/de rows, raster Hover | yes | puzzle 11/11 rows en ↔ de; Hover ships as "Darüberfahren" (§12-3) |
| 6 | creation kind picker labels every kind "Editor" | no | H9 owns it |
| 7 | a plugin absent from the local registry cannot be installed from the hub | no | unowned and large (registry + hub install flow); not started |
| 12 | native wgpu accessibility | no | WG7 |
| — | Home viewer lists no hub rows | found | the viewer is read-only by construction and the directory projection is fed only through the editor's sealed-page lane; a fresh viewer instance has none. Needs a read-only projection feed for the viewer (guest + host) — after the guest freeze |

## 0. Inherited state (truth vs audit)

The S11 audit (`📓️audit-s11-os-frontend.md`) repeats G5's 2026-09-2x findings; several are stale against the tree:

| audit claim | truth in the tree (read 2026-09-25 00:5x) |
|---|---|
| `MutationKind::label()` is 2690 hard-coded English strings; history panel English-only | **stale**: U3/U3b moved all 2795 sites to `LocalizedLabel::native(en, de)`; React `🏛️ShellHost` resolves `historyEntryLabelText(entry.label, uiTerminology, uiLocale)` at render time. The real gap is the *quality* of the German (generated from a term glossary) — §1 |
| TaskManager built but never mounted | **stale**: U1 mounted it as `os.task-manager` (bottom-right dock + palette `os.openTaskManager`) — but it only showed actors and said "no runtime attached" because no host passed a registry. The React plugin runtime **does** own an `ActivationRegistry` (module-private `getActivationRegistry`) — U1's "no ActivationRegistry" note was wrong. Jobs/installs/tool calls were not listed anywhere — §2 |
| no cancel on agent tool calls | **stale**: U1 added `cancelToolCall` (bridge frame tag 10 → gateway job registry) |
| `ShellSync` status text English; no persistent hub indicator | **mostly stale**: U1 localized the status words and mounted `HubConnectionIndicator` in the footer. Residual: the popover title `{cardKind} backbone` was an English literal, and the badge ignored the shell's own session link (a signed-in shell with no attached document read `offline`; a hub shortage was invisible until a document socket dropped) — §3 |
| tablet is a manual override only | **stale**: U2 added `UI_TABLET_MAX_WIDTH_PX = 1023`, `useUiDevice()`, `selectUiDevice(state, measured)` — needs a live 3-viewport check — §5 |
| two approval surfaces | **true**: AP1 §6.5 — the modal `AgentApprovals` veil blocked the inline chat decision — §4 |

## 1. Mutation label localization

### 1.1 Measured truth (`🐍️` → `wp-u5/u5-label-audit.py`, capture `generated/u5-label-audit.json`)

2800 trait-anchored `fn label(&self) -> …LocalizedLabel` sites in `✏️s` + `🧰️framework`; 2659 distinct (crate, en, de)
templates (`generated/u5-label-pairs.txt`). Every site carries a German cell — no `String` path, no English fallback exists.
But the German was produced by U3's term glossary + compounding rules, and reading all 2659 pairs found it is frequently
**not German a domain expert would write**:

| defect class | examples (before) |
|---|---|
| symbols glued into compounds (58) | `Bemessungsaxialkraftn_Ed [kN] auf {} ändern`, `Referenzprimärenergienbedarfq_p`, `NetzuRL`, `DokumentuRI` |
| wrong domain term | Eurocode `action` → `Aktion` (is *Einwirkung*), `shear force V_Ed` → `Schubkraft` (is *Querkraft*), pile `shaft/base resistance` → `Schacht-/Basiswiderstand` (is *Mantelreibung/Spitzenwiderstand*), table `column` → `Stütze` (is *Spalte*), DTP `story` → `Geschoss` (is *Textfluss*), machining `stock` → `Bestand` (is *Rohteil*), mp4 `sample` → `Probe`, JSON `member` → `Glied`, LiDAR `points by return` → `Punkte durch Rückluft`, SHGC → not `g-Wert` |
| verbs parsed as nouns | `Seal run` → `Siegellauf`, `Switch space alternative` → `Schalterraumalternative`, `Touch artifact` → `Berührungsartefakt`, `collapse page size` → `Einklappenseitengröße`, `Sign out` → `aus signieren` |
| missing genitive / word order | `Griff "{}" Griffart auf "{}" ändern`, `Ebene "{}" sichtbar auf {} setzen`, `Kante "{}" gesperrt ändern` |
| `(s)` plural hack (10) | `{} Objekt(s) verschieben`, `Gruppe {} Knoten(s) in Ebene #{}` |
| English side not prose | 484 stdio sites were kebab op ids (`set-snapshot`, `remove-topic`); en1992/en1995/din4108 English was a field-name dump (`Change a c mm2 to {:?}`) |
| wrapper labels | 18 `stdio.semio` envelope leaves (`apply-cad` …) labelled every edit with the bare subset name (`cad`/`cad`, `graph`/`Graph`) |

### 1.2 What landed (01:15)

* **Handcrafted table** `wp-u5/u5-de-corrections.txt` — ~930 template rewrites across 60 crates, written by reading every
  pair (energy/EnergyPlus, Eurocode/DIN norms, OOXML/PDF/glTF/media codecs, architecture programming, DTP layout, FEM, …).
  Where the English was a field dump it carries the English too (`⟸`).
* **Codemod** `wp-u5/u5-apply-de-corrections.py` (one-off, ticket folder): region-anchored exactly like U3, span-keyed,
  literal-only; asserts every rewritten literal keeps its placeholder sequence and is a valid Rust string body; a
  correction matching no site aborts the run. Kebab English → sentence case (`set-snapshot` → `Set snapshot`, acronym
  table). Run: `sites 2774, corrected 1313, kebabEnglish 523, changedFiles 1649, errors 0` (`generated/u5-apply-run.json`).
  One stale row (`splice`) was dropped because a peer renamed that leaf to `replace-byte-range` minutes earlier.
* **Manual edits** `wp-u5/u5-manual-label-edits.py`: the 18 `apply-*` envelope leaves now delegate to the wrapped
  subset mutation's own localized label (`protocol::SemanticMutation::label(&self.mutation)`); the 10 `(s)` labels pick
  singular/plural per count (`1 Objekt verschieben` / `{count} Objekte verschieben`).
* **Rebuild request** appended to `wp-w1/requests/u5.txt` (W2's post-H9 full rebuild picks the literals up for every crate
  it compiles after 01:15).

### 1.3 Compile-atomic proof

`wp-u5/u5-check-sweep.sh` — native `cargo check -p … --keep-going` over all 105 touched crates in 14 batches of 8, one
cargo at a time (`generated/u5-check-batch<N>.txt`, summary `generated/u5-check-sweep.txt`): **14/14 EXIT 0, 0 errors**
(02:32). Post-sweep audit (`generated/u5-label-audit-after.json`): 2698 ok; the 76 "camel" hits are symbols/proper names
kept on purpose (`V_Ed`, `glTF`), the 26 "other" are the envelope delegations.

### 1.4 Live en/de history rows (`wp-u5/u5-history-locale-probe.mjs`)

Per plugin: spawn from Home, drive one real mutation + undo/redo through the Actions rail, read every History row, switch
the SHELL locale to German in Settings, read the same rows again without dispatching.

| plugin | row (en → de) | run |
|---|---|---|
| dag | Add Node → Knoten hinzufügen | r3 (restaged guests, `generated/u5-history-r3.json`) |
| block | Add Handle Kind → Griffart hinzufügen | r3 |
| gis | Add Feature → Objekt hinzufügen | r3 |
| note | Add Block → Block hinzufügen | r3 |
| layout | Add Page → Seite hinzufügen | r3 |
| draw | Add Layer → Ebene hinzufügen | r3 |
| forms | Add Step → Schritt hinzufügen | r3 |
| writer | Set Text → Text festlegen | r2 (`generated/u5-history-r2.json`; not spawned in r3) |
| energy | Create zone → Zone anlegen | r2 (not spawned in r3) |
| raster | Add Layer → Ebene hinzufügen, Set Composite Viewport → Komposit-Ansichtsfenster festlegen, **Hover → Hover** | r3 |
| puzzle | Add Node → Knoten hinzufügen (+ Undo/Redo, panel and window rows) | `wp-u5/u5-puzzle-history-probe.mjs`, `generated/u5-puzzle1.json` (07:3x; r3's miss was a Settings boot without a language control) |

Every other row (Toggle Panel, Activate Window, Clear Selection, Undo/Redo) re-rendered in German. The one untranslated
row is the framework's own hover interaction (`🛂️manifest/🦀️.rs` `INTERACTION_HOVER_ACTION_ID`): German "Hover" →
"Darüberfahren" (`cargo check -p semio-framework` green); descriptors carry it after W2's next describe (request filed).

## 2. Progress + cancellation UI

**Landed** (React host):

* `🔌️PluginRuntime/💼️job-ledger/🟦️.ts` (new, React-free) — the live spawned-job ledger `driveSpawnedJob` keeps: one row per
  job it drives (plugin, actor, kind, admitted steps, whether the guest reported progress, start time, `cancelling`),
  step updates coalesced to ≤ 1 publish / 250 ms, open/cancel/close immediate. `cancelSpawnedJobV1` sends `cancelJob`
  through the job's own actor ingress (cooperative: the guest frees the id, its next `step-job` answers `job.unknown`, the
  drive delivers that as the job's `job-completed` failure — the ledger only records that a human asked).
* `pluginRuntimeActivationRegistryV1()` exposes the runtime's real `ActivationRegistry` (never creates one).
* `🧵️TaskManager`: `TaskManagerWindow({ sources })` — **Running tasks** (spawned jobs, plugin installations, agent tool
  calls; each a `role=progressbar` with localized `aria-valuetext` "128 steps · 12 s" and a named Cancel button, disabled
  while cancelling) above the **Actors** table (suspend/resume/cancel). The window starts the registry's metrics publisher
  while mounted and stops it when closed. en + de labels (`os.taskManager.tasks.*`).
* `🏛️ShellHost`: stable `TaskManagerSourcesV1` (ledger + installs from `pluginStatusById` + running tool calls from the
  agent conversation); cancel routes job → `cancelSpawnedJobV1`, install → that plugin's `AbortController`, tool call →
  `agentBridge.cancelToolCall`. The unused `activationRegistry` prop (U1 placeholder nobody passed) is removed.

**Tests**: `🧵️TaskManager` 22 + `💼️job-ledger` 4 (new suite) — fixture `🧵️TaskManager/🧫️fixtures/🏃️running-tasks.json`
replayed, ARIA read back through Testing Library role queries (third-party oracle); `🔌️plugin-runtime` integration test
"lists a driven job in the task-manager ledger and routes a cancel to the guest's own cancel-job" (fake shard, 3/3 green,
`generated/u5-vitest-plugin-runtime-jobs.txt`). Renderer typecheck 0 errors (`generated/u5-typecheck-renderer-4.txt`).

**Live** (`generated/u5-live-tasks.json`, `u5-tasks-en.png`, `u5-tasks-de.png`): Tasks window sections
"Running tasks / Actors" ↔ "Laufende Aufgaben / Akteure", empty state "No task is running." ↔ "Es läuft keine Aufgabe.",
real actor row `space#1 | space | Maintenance | Active` ↔ `Wartung | Aktiv`, buttons "Suspend/Resume/Cancel: space#1" ↔
"Anhalten/Fortsetzen/Abbrechen: space#1". A spawned fem model listed its actor (`fem#3`) but ran no retained job during
the probe, so no live job row was observed (`generated/u5-live-jobs.json`).

**Agent tool-call cancel — defect found and fixed.** Live gate run 1 (`generated/u5-live-agent-loop-en.txt`): Cancel on the
running `action_invoke` row turned it `cancelling`, but the call — parked on a destructive-verb approval — ended only at the
120 s approval deadline as `APPROVAL_REQUIRED` (the gate passed it anyway). Cause: `ApprovalCoordinator::resolve_by_shell`
polled for a decision until its deadline and never looked at the tool call's own job, which `AgentCancel` flips.
Fix (gateway, `semio-framework-os-mcp`):

* `📣️notify` `active_request_cancel_requested()` — whether the `tools/call` running on this thread was asked to stop
  (`notifications/cancelled` for its request, or a cancel of any job minted under it — the chat Cancel);
* `🛡️policy` `ApprovalResolution::Cancelled`; the shell-lane wait checks it every poll;
* `🔀️dispatch` `SettledApproval::Cancelled` → decides the parked handle `false`, answers `CANCELLED` with the approval handle;
* tests: `a_cancelled_tool_call_ends_its_parked_shell_approval_instead_of_waiting_out_the_deadline`,
  `outside_a_request_scope_no_call_reads_as_cancelled` — policy suite 28/28 (`generated/u5-test-os-mcp-policy.txt`);
* gate (c) tightened: clicks only the `action_invoke` row (`data-semio-agent-chat-tool`, new attribute on the chat Cancel
  button — run 2 had clicked an already-answered search row) and requires `CANCELLED` within 30 s.

Gate run 3 (`generated/u5-live-agent-loop-en-3.txt`): **22 passed, 0 failed** — (c) `clicked=inv_8 state=cancelling
settledMs=364 code=CANCELLED`.

## 3. Connection status

* `🔄️ShellSync`: popover title `{cardKind} backbone` (English literal) → `ui.sync.backboneFile|Folder|Remote` (en + de).
* `hubConnectionSummaryV1(statuses, session, link)` now folds the shell's own session link (`verifying | reachable |
  unreachable` from the directory session refresh loop's `onDegraded`) with the documents: `local` (no hub configured),
  `signedOut`, `live · N peers`, `reconnecting` (a verified session that stops answering — a short shortage the loop rides
  out on bounded backoff, nothing else stops), `connecting`, `online` (signed in, hub reachable, nothing attached — used to
  read a false `offline`), `offline`. `ShellHost` tracks `hubSessionRefused` (hub pronounced the session over / human
  cancelled) so that case offers sign-in instead of a never-ending `reconnecting`.
* Law fixture `🔄️ShellSync/🧫️fixtures/📶️hub-connection-summary.json` (10 cases) replayed; `ShellSync` suite green.

**Live** (`generated/u5-live-hub.json`, `u5-hub-reconnecting.png`, `u5-hub-reconnecting-de.png`): signed in → "Hub
connection: online"; hub process SIGSTOP → "reconnecting…" after 5078 ms while the page kept painting (967 rAF frames, max
gap 33 ms); SIGCONT → "online" after 3313 ms; German: "Hub-Verbindung: online" / "verbinde erneut…" (4820 ms).

## 4. Agent approvals

* `🤖️AgentApprovals`: the modal `Dialog` is **retired**. `AgentApprovalAffordance` is the ONE affordance per approval,
  rendered as the approval's row in the agent conversation: verb, capability, description, target, change summary, who
  asked, risk, polite live countdown, three decisions with the shared `framework.approvals.<decision>.<id>` ids (the wgpu
  twin's scheme), accessible names carrying the capability; a resolved row keeps its record and says how it was decided
  (`decidedDeny|Once|Session`, en + de). `section tabIndex=-1 aria-labelledby` so focus can land on the group, never on a
  decision.
* `AgentApprovalsNotice` (footer, every device): "N agent approvals are waiting" + Review — decides nothing, reveals the
  chat panel and moves focus onto the oldest affordance; announces a new request in its live region.
* `ShellHost`: a newly arrived approval reveals the chat panel (without stealing focus); `revealAgentApproval` for Review.
* `💬️AgentChatPanel` renders the affordance instead of its own copy (duplicate countdown/decision code deleted).
* Gate `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` reads the unified affordance (`surface=conversation`).

**Live** (gate run 3 on serve 6580, note spawned in the real `s` host): (e1) `affordance=conversation countdown=120`,
Approve Once → the destructive `deleteSelection` commits; (i18n) en "Approve Once"/"Deny"; (e2) Deny →
`PERMISSION_DENIED channel=shell`; (e3) silent client → typed `APPROVAL_REQUIRED`. German: G10's 05:48 run of the same gate
on the same affordance, (i18n) de "Einmal genehmigen"/"Ablehnen" PASS.

## 5. Devices (`wp-u5/u5-live-probe.mjs devices`, `generated/u5-live-devices.json`, `u5-devices-*.png`)

| viewport | detected | touch | horizontal overflow | footer controls clipped / unreachable |
|---|---|---|---|---|
| 1440×900 | desktop | no | none (scrollWidth 1440) | 0 / 0 of 9 |
| 375×812 | mobile | yes | none (375) | 0 / 0 of 2 (mobile chrome) |
| 768×1024 | tablet | yes | none (768) | 0 / 0 of 9 |

Defect found at 768: the footer's panel-tab labels pushed Tasks/History off the bar. Fix: `PanelTabBar` gained
`compactLabels` (label becomes `sr-only`, the icon stays, the accessible name is unchanged), `ShellHost` sets it on tablet
and omits the empty presence bar there; measured after the fix as above.

## 6. Small defects

### 6a. dag horizontal two-finger pan

`🗺️surface/🕸️node-graph/🦀️.rs`: `GraphHost::wheel_screen/plan_wheel` take `delta_x` and pan `x − delta_x / zoom` (was
dropped); the wasm binding passes it; the wgpu renderer's call site updated. Native check + unit tests green
(`generated/u5-test-surface-wheel.txt`, new `graph_host_wheel_screen_pans_horizontally_by_delta_x_over_zoom`); wasm32
check green; `@semio-tech/framework-surface-rs:wasm` rebuilt through the wasm mutex (06:10–06:12, EXIT 0). Stale docstring
in `🕸️NodeGraph/🟦️.tsx` ("the dag binding currently drops deltaX") corrected.
**Live** (`wp-u5/u5-dag-pan-probe.mjs`, `generated/u5-dag-pan1.json`): a CDP two-point touch pan (constant finger distance)
over the dag canvas: +90 px horizontal → camera x 0 → −82.6 (one pinch notch to zoom 1.1 was recognised at touch start),
y unchanged; +70 px vertical → y −63.6 = −70/1.1, x unchanged.

### 6b. Home's own space table empty after sign-in

Probes: `wp-u5/u5-home-live-probe.mjs` (sign in, remote create by a second client of the same human, own create through the
Create Space dialog, reload), `wp-u5/u5-home-reload-probe.mjs`.

**Cause 1 — C10 F3, fixed.** Live directory events reached Home as `foldDirectoryEvents`, classified
`BatchOnlyPendingRewrite`, so every live fold was refused (`generated/u5-home-before2.json`: `input #3 foldDirectoryEvents
refused: dispatch-failed … BatchOnlyPendingRewrite`). Migrating it would have been wrong: the fold advances the projection
cursor past the sealed receipt, so the next authenticated page (`after == receipt cursor`) answers the non-retryable
`s.home.directory-event-page-frontier-race`, and the raw fold bypasses the hub's per-user filtering. The worker's
acknowledged stream already turns every live event into a wake → next sealed page → `applyDirectoryEventPage`. So the raw
writer is **retired** (one-off codemod `wp-u5/u5-retire-home-fold.py`): Home's `foldDirectoryEvents` action + command
module, `HomeConfigMutation::FoldDirectoryEvent`, `HomeViewCommand::FoldDirectoryEvents`, the retained-limits fixture row
and schema route (18 → 17), the taxonomy member; `ShellHost` and the wgpu shell relay raw events only to the Space index
(the one app that still folds them); the `home-host-panel-owner` gate in `📜️script.ts` asserts the action is gone (its stale
batch-only list — `applyDirectoryEventPage`/`createStudio` are migrated — corrected). Checks: `cargo check -p
semio-s-artifact-space-home --lib --tests` and `-p semio-s-plugin-space --lib --tests` green; `cargo test -p
semio-s-artifact-space-home --lib --features component-app-assembly` 92/94 — the 2 reds are peers' (below); wgpu renderer
check green. W2's 05:58 restage carries it (descriptor regenerated 05:15 without the Home action).
**Live on the restaged guests** (`generated/u5-home-restaged.json`): signed in → hub rows at once; a space created by a
second client → its row 244 ms after the hub answered; own Create Space → row in 513 ms; reload → rows back in 4.3 s; zero
fold refusals, frontier races or faults.

**Cause 2 — a table could not hold more than 8 author rows.** With ≥ 9 hub spaces the Home surface faults:
`retained surface render fault … s-home-main … nodes: 129, max_nodes: 128` and the window keeps its previous paint (only the
local demo row) — "empty table while the overlay lists spaces". Re-measured on a fresh hub with 30 spaces and the old guest
(`generated/u5-home-before.json`): same fault, 1 row. An author row cost 12 nodes (row + 6 cells + 5 action buttons), the
header 8, the chrome 5; `UI_DOCUMENT_NODES = 128`. The SDK kit also capped a table at 32 rows (`TABLE_WINDOW_ROWS`).

**Fix — one windowing mechanism for trees and tables** (landed; live proof waits on W2's space rebuild):

* **UI contract** (additive, both languages): `Component::Table(TableProps { label, columns, actions_label, window:
  Option<TreeWindow> })` and `Component::TableRow(TableRowProps { cells, row_actions })` — the header lives on the table,
  cells and row actions are PROPS, so a table costs `1 + materialised rows` records, the tree cost model exactly; the table
  carries the same `TreeWindow` stamp a tree section does. Rust: builders `table`/`table_row`, limits, accessibility (`grid`,
  `row`, rows focusable, table named), typed copy/compare/retirement rosters, reconciler census, wgpu mapping (a table is a
  stack, a row a button reading its cells joined, primary action = row activation). TS: generated projection
  (`@semio-tech/ui-contract-rs:generate` law 86 types, `typegen_export` green), retained wire decoder, text accounting,
  accessibility twin, `UiDocumentStore` text bytes.
* **Language-neutral fixtures**: conformance case `🧩️component/📊️table` (a window rows 12–13 of 40) read by the Rust
  conformance laws (26/26 with accessibility + limits) and by the Interpreter's corpus law (64/64); the typed-retirement
  roster (`🧩️components.json`, `componentVariants`) carries both new variants; Ajv validates the corpus catalog (64 cases).
* **Plugin SDK**: `TableWindowKit::render_rows(windows, label, columns, actions_label, entries, row)` builds only the host's
  slice of `entries` on the SAME `TreeWindows` ledger the tree panels spend (`claim_window`/`sliced`/`stamp`/
  `tree_window_rows`), `table_window_row(key, cells, actions, activate)` and `table_row_action(icon, label, action)` build one
  row record. The fixed 32-row `TableRowsView`, its retire arena and the reactor's arena ladder are gone (rows are ordinary
  built nodes retired by the built-node ladder). Laws: one node per row, row actions/activation as props, exactly the host's
  window (`offset 200 rows 20` of 500 → 20 rows, ledger debited 21), a 10 000-row first paint stays at one default viewport
  (5/5, `generated/u5-test-sdk-table.txt`).
* **Apps**: Home editor + viewer and the Space index editor + viewer render through the windowed kit (`HomeTableLabels.table_name`
  en/de); Home rows open on Enter/double-click (`openSpace` activation) and keep their role-gated actions. Home window tests
  21/21, Space window tests 7/7.
* **React host**: `TableView` (Interpreter) renders `role=grid` with `aria-rowcount`/`aria-colcount`, a header row of
  `columnheader`s, `row`s with `aria-rowindex` at their LOGICAL index behind the tree windows' own spacers
  (`treeWindowDomAttributes`, `treeWindowSpacerRows`), and reports through the SAME `useTreeWindowObserver` — so the body's one
  scheduler asks the guest for the rows the viewport shows. Keyboard (ARIA grid, one tab stop): Up/Down, PageUp/PageDown,
  Home/End over the whole logical extent (scrolls unstreamed rows into view and focuses them when they arrive), Enter/Space =
  row activation, Right/Left into the row's actions (named "open: Studio 07"), Escape back. A polite status reads "Rows 13–14
  of 40" / "Zeilen 13–14 von 40" (`ui.host.tableRowRange`, en + de). `ShellHost` now gives WINDOW bodies the tree-window
  channel too (`TreeWindowContext` per window `bodyKey`; the scheduler refreshes `windowBodies` for them). Interpreter suite
  125/125 incl. 4 table laws (Testing Library role queries as the oracle); renderer typecheck 0 errors.
* Native checks green: `semio-framework-ui-contract`, `-ui-runtime`, `-ui` (wgpu-engine), `semio-framework-plugin`,
  `semio-s-artifact-space-home`, `-space-space`, `semio-s-plugin-space`, `semio-framework-os-renderer-wgpu`. W2 request filed
  (space guest rebuild + describe); no WIT/owned-ABI, pack-schema or codec-hash change — but the UI contract crate is linked
  into every guest, so any guest compiled after ~07:10 has new wasm bytes.

## 7. Honest gaps

* **Windowed table live proof** (6b cause 2): implemented and tested; the ≥ 25-space scroll-to-last proof runs once W2 restages the space guest (baseline captured: 30 spaces fault with the old guest).
* **wgpu table**: renders the materialised rows as activatable rows (cells joined, primary action) — no column layout, no row-action strip, no window requests yet (WG lanes).
* **Pre-existing reds, not mine** (same set on the baseline with my contract edits reverted, `u5-baseline-toggle.py` in the
  scratchpad): `semio-framework-ui-contract --lib` binding-copy / component copy+compare / typed + built retirement / document
  assembly laws ("did not retire exact owners", arena poisoned) and the runtime's `runtime_tree_retirement_*` twins — the
  close ladders no longer finish under the laws' byte grants. The Rust conformance, accessibility, limits and typegen laws are
  green; the new fixture rows ride the same (currently red) retirement laws.
* **Approval affordance after a cancel**: the gateway ends the call `CANCELLED` but sends no `ApprovalResolved`, so the
  shell's affordance stays until its own countdown and a late decision is ignored. The bridge has no "withdrawn" outcome;
  adding one touches the gateway⇄shell wire and both shells (G10/WG lanes).
* **No live spawned-job row** in the Tasks window (no retained job ran during the probe); covered by the integration test.
* **History en/de not read live** for puzzle; raster "Hover" fixed in source, visible after W2's next describe.
* **MutationKind labels** reach History rows only for contributed mutations; ordinary rows show the action definition label
  (both localized).
* C10 F1 (Create Space under the Actions chip) left to S15 (chip hit-testing is theirs).
* Peer reds seen, not mine: `retained_config_cancel_and_cleanup_respect_the_production_grant` (test expects `Blocked` for an
  undersized close; S8 made `close_step` release per page), `direct_owner_descriptor_surfaces_and_catalog_correspond`
  (in-flight `change-catalog-generation` outcome classes).
* HubSignIn buttons read "Sign in Sign in" in `innerText` dumps — not investigated (likely visible text + a visually
  hidden label).

## 8. Files changed

* Labels: 1649 `fn label` sites in 105 crates (`generated/u5-apply-run.json`, `generated/u5-check-batches.json`), 18
  `✉️base/🧬️schema/🧬️mutations/*apply-*` delegations, 10 plural labels (`wp-u5/u5-manual-label-edits.py`);
  `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (Hover).
* Tasks/jobs: `🔌️PluginRuntime/💼️job-ledger/🟦️.ts` (new), `🔌️PluginRuntime/🟦️.tsx`, `🧵️TaskManager/🟦️.tsx`,
  `🧵️TaskManager/🧫️fixtures/🏃️running-tasks.json` (new), `🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx`,
  `🧑‍🎨engine/🧪️tests/💼️job-ledger/🟦️.ts` (new), `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`,
  `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`.
* Shell: `🏛️ShellHost/🟦️.tsx`, `🔄️ShellSync/🟦️.tsx` + `🧫️fixtures/📶️hub-connection-summary.json` + tests,
  `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, `🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx`,
  `🔗️AgentBridge/🟦️.tsx`, `🤖️AgentApprovals/🟦️.tsx` + tests, `💬️AgentChatPanel/🟦️.tsx` + tests, `🕸️NodeGraph/🟦️.tsx`.
* Gateway: `🌉️mcp/📣️notify/🦀️.rs`, `🌉️mcp/🛡️policy/🦀️.rs`, `🌉️mcp/🛡️policy/🧪️tests/🔬️quick/🦀️.rs`,
  `🌉️mcp/🔀️dispatch/🦀️.rs`, `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`.
* dag: `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`, `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`.
* Home: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️.rs`, `…/✳️any/✏️editor/🦀️.rs`, `…/✏️editor/🎚️config/🦀️.rs` + tests,
  `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `…/🎮️commands/🏷️rename-space/🧪️tests/🔬️unit/🦀️.rs`, `…/🎮️commands/🌱create-space/🦀️.rs`,
  `…/👁️viewer/🦀️.rs` + tests, `…/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`, `…/✳️any/🧬️schema/🔣️.json`,
  removed `…/🎮️commands/📇️fold-directory-events/`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`; `📜️script.ts`;
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
* Windowed table: `🖱️ui/🧬️contract/🧩️component/🦀️.rs`, `🏗️builder/🦀️.rs`, `🛡️limits/🦀️.rs`, `♿️accessibility/🦀️.rs` + `🟦️.ts`,
  `🧾️typed/🦀️.rs`, `🪞️copy/🦀️.rs`, `⚖️compare/🦀️.rs`, `♻️retirement/🌳️typed/🧱️component/🦀️.rs` + `🧩️components.json` + `🧫️fixtures/🔣️.json` +
  `🧬️schema/🔣️.json`, `🧬️schema/🦀️.rs`, `🧪️tests/🧬️typegen-export/🦀️.rs`, `🧪️tests/🔬️conformance-unit/🦀️.rs`,
  `🧪️tests/🔬️conformance-corpus/🟦️.ts`, `🧫️fixtures/🧪️conformance/📇️catalog.json` + new `🧩️component/📊️table/`,
  `🧵️retained/🟦️.ts`, `🧵️retained/📦️wire/🧾️typed/🟦️.ts`; `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, `🛂️manifest/🟦️.ts`;
  `🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs`; `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`; `🔌️plugin/🦀️.rs` (table kit),
  `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, `🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`, `🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs`;
  space: `🫀️core/🦀️.rs`, Home editor + viewer main windows and tests, Space index editor + viewer main windows, roots and
  tests; React: `🗣️Interpreter/🟦️.tsx` + new `🧪️tests/📊️table/🟦️.tsx` + corpus law count, `📃️UiDocumentStore/🟦️.tsx`,
  `🏛️ShellHost/🟦️.tsx` (window-body tree-window channel), `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`.
  One-off codemods: `wp-u5/u5-table-window-{contract,sdk,apps,react}.py`; probes `u5-home-table-probe.mjs`,
  `u5-puzzle-history-probe.mjs`.
* Requests: `wp-w1/requests/u5.txt` (label pass, surface wasm, space guest + describe, Hover label, windowed table → space guest).

## 9. Processes started

| what | pid | port | capture |
|---|---|---|---|
| hub hold (`wp-u5/u5-hub-hold.ts`, catalog-less, credential sign-in on) | hold 67070, os-hub 67072 | 8080 | `wp-u5/state-8080/` |
| `dev s` serve, first (`S_OS_PORT=6580 S_HUB_URL=…8080`) | 67409 / 67450 | 6580 | `generated/u5-serve.txt` |
| `dev s` serve, restarted for W2's restage (+ `S_AGENT_BRIDGE_DIR=wp-u5/rendezvous`) | 19731 / 19749 | 6580 | `generated/u5-serve-2.txt` |
| local-only serve (stopped at once: it began building its own `os-hub`) | 31222 tree | 6581 | `generated/u5-serve-6581.txt` |
| cargo check sweep | 84658 → 24163 | — | `generated/u5-check-sweep.txt` |
| surface wasm build (wasm mutex) | — | — | `generated/u5-surface-wasm-build.txt` |

`wp-u5/target`, the copied hub binary and the hub data dir are deleted.
| hub hold, second run (fresh data, ada/bo provisioned with `os-hub credential set`, 30 spaces "Studio 01…30") | hold 71359, os-hub 71366 | 8080 | `wp-u5/state-8080/` |
| `dev s` serve, third run | 71361 tree | 6580 | `generated/u5-serve-3.txt` |

All stopped at the end of the turn; the hub data dir is deleted again (re-seed: `os-hub credential set` + `u5-hub-seed.ts`).
