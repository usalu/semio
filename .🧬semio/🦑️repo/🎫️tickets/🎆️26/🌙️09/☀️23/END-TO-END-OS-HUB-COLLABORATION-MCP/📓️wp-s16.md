# S16 — os `s` frontend with all plugins and artifacts (session 13)

Slice S16, session 13, 2026-09-26 19:0x. Successor of S15 (`📓️wp-s15.md`) and U5 (`📓️wp-u5.md`); audit
`📓️audit-s12-os-frontend.md`. Ports: hubs 8040–8049, serves 6540–6549. Durable data/logs `.🧬semio/🌐hub/s13-s16-*`.
Captures (expendable) `wp-s16/generated/`. Harnesses: S15's (`wp-s15/s15-matrix.mjs`, `s15-toolrun-matrix.mjs`,
`s15-b2-sweep.sh`, `s15-hub-journey.mjs`) and U5's (`wp-u5/u5-*-probe.mjs`).

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Session 13

| # | item | state | evidence |
|---|---|---|---|
| 1 | S12-3g: saga-opened hub document replaced by the space index within 500–750 ms — verify, root-fix, law | **measured PASS on the current code** — gone. Root fix = C10's route ledger (09:07, `🏛️ShellHost/🧭️route-ledger`: an applied URI is never re-applied by the route effect; only a replaced session or an explicit navigation invalidates it). Live on hub 8040 / serve 6541: saga-opened note (en) and writer (de) keep ONE window state for 15 s hold + 15 s trace, verb/undo/redo `[0,1,0,1]`; Space-index ROW opens after a signed-out hard load of `/spaces/<id>` (route admission `await-sign-in` → sign in → space) keep one state for 20 s: note de ×2, writer en ×1. Laws `🧭️route-ledger` + `🔀️surface-switch` **9/9** | `journey-trace-{note-en,writer-de}.txt` (s13-s16-logs), `generated/s16-row-open-{note-de-r1,note-de-r2,writer-en-r1}.json`, `generated/s16-law-route-ledger.txt` |
| 2 | B2 kind sweep reds in `s` (de 9/16): shell-side vs hub-side; fix shell-side, route hub-side to H11 | **classified, no shell-side cause found.** On the current-tree hub 8041 (B2 clone, C11's 19:07 binary) the de sweep `c1` rows 0–11: block2d, draw, wfc2d, bitmap, grid2d, block3d, wfc3d, grid3d, block5d created AND opened by the saga (the 7800 `component` 503s are gone with the streamed route); puzzle2d/3d/5d stayed `accepted` past the journey's 180 s (5 min shown with the elapsed line + Cancel, en/de) = hub-side CPU-bound puzzle genesis, already root-caused by H11 (749 s / 579 s, all created; speed → H12, stage → LC). The 180 s bound was the HARNESS's, not the shell's (the shell has no fixed creation deadline since S12-3c): the item-5 sweep waits 900 s for puzzle kinds. Verb rows void under rule 23 (hub edits refused in React until the rebuild); re-measured in item 5 | `s13-s16-logs/c1-sweep-de.txt`, `journey-c1-de-*.txt`, `📓️wp-h11.md` item 4 |
| 3 | UX leftovers: keyboard/focus, WCAG, mobile/tablet, customization persistence, German completeness | **measured on the current code (serve 6540, 06:3x)**: 29 tab stops, every one with a visible focus ring, ⌘K palette focused, focus moves into the opened program's tabpanel; default theme chrome pairs **18/18 AA** both appearances (lowest 4.84); phone 375 / tablet 768 without horizontal scroll (tablet: composite + navigator side by side); Dark survives a reload (event-sourced `os.config.ui-preferences`); en/de chrome 92/91, identical only loan words/data; `check-chrome-i18n` 0 violations. Found + fixed: Space-index open button named "Open: artifact-<id>" in de (item 6 log). **Cross-DEVICE (11:1x, current-tree hub 8042): was FAIL for every preference — root-fixed:** the worker's session-route allowlist (`browserDirectoryRequest`) never admitted `/directory/preference-page/v1`, so every device's preference lane failed before the network ("browser directory operation denied" → a silent `preference-lane-failed transport`); a device only ever saw its OWN changes. After (fixture `💻️os/🧫️fixtures/📇️directory/🚦️browser-directory-routes.json` + law, path-to-regexp + WHATWG URL oracle, red→green; os tsc 0): device B (fresh context, same user) gets **Dark + Deutsch + the ⌃⌥K Undo override** at sign-in (+2 s); dock panels stay device-local by design. **Still open:** saved NAMED layouts do not travel — they live in `semio.os.config › namedLayouts`, outside the event-sourced `os.config.ui-preferences` log the lane carries (design gap; needs `saveNamedLayout`/`deleteNamedLayout` preference mutations, schema-first incl. the Rust twin → after PUBLISH DONE). Row-name fix live after the restage | `generated/s16-ux-*.json`, `s16-contrast-census.txt`, `s16-chrome-i18n-lint.txt`, `s16-preference-device-{b2,b3}.json`, `s16-customization-c{1,2}.log`, `s16-law-directory-routes-{1,red}.txt` | `generated/s16-ux-{keyboard,locale,devices,persist}.json`, `s16-contrast-census.txt`, `s16-chrome-i18n-lint.txt` |
| 4 | full matrix on W3's consolidated restage (editors en + de, viewers, tool-run) with V1's permanent `program-matrix` | waiting for W3's activate-s (per-extension rows moved to S17 by the coordinator, 06:0x) | |
| 5 | hub-document sweep for every kind of every package (`--packages all`), en + de | waiting for the all-package publish | |
| 6 | (coord 19:2x, audit-s13 §4 #16) Home viewer lists 0 hub rows — read-only projection feed; root-fix + law | **written + host law green (15/15, red→green); guest native/wasm32 checks in flight (05:0x)**; landed in HEAD by the auto-commit `40a2736e661` (22:00); live proof after W3's restage | log 19:5x, 21:1x, 05:0x |
| 7 | (coord, audit-s13 §4 #7) plugin absent from the local registry: installable from the hub? verify S12-6 on the current code, fix a gap | **source + laws verified (06:4x)**: S15's registry union (`hubCatalogOnlyPluginsV1` → Marketplace `Source: hub · <generation>` → verified install into the store) is still wired in ShellHost; resolution + store laws **43/43** (after fixing the store law's unhandled rejection). The audit's "unowned, not started" is stale: S15 measured it live en + de on 09-26 (S12-6). Live re-proof on the rebuilt hub (7800 on B3/all) pending | log 06:4x |
| 8 | (coord, P0-3) a live `s` serve on my port, kept up for the matrix re-run | serve **6540** local-only up again 06:2x (w2-detach pid 55663, log `s13-s16-logs/serve-6540-b.txt`); restarted after W3's activate-s | |

### Session 13 log

- 19:07 read AGENTS.md, preambles 13/12, `📓️wp-s15.md`, `📓️wp-u5.md`, audit §4. Nothing of S15/U5 listens on 6540–6549 /
  8040–8049; 7800 = os-hub pid 54029 (catalog B2). Load 41, disk 111 GiB free.
- 19:10 hub **8040** restarted on S15's data root (catalog B `e8167ce8…`, 12 kinds; S15's 09:03 binary with the streamed routes): hold
  **17048** → hub **17050**, state `.🧬semio/🌐hub/s13-s16-hub-8040-state`, ready 17:11:46Z. Serve **6541** → 8040 (dev lane, HMR off):
  pid **18320**, log `s13-s16-logs/serve-6541.txt` (`joined the hub at http://127.0.0.1:8040`). Harnesses copied to `wp-s16/s16-*`
  (outputs retargeted to `wp-s16/generated/`, logs `s13-s16-logs/`, profiles `s13-s16-profiles/`).
- 19:2x item 1 measured (table). Serve **6540** local-only (HMR off) for the matrix: pid **31119**, log `s13-s16-logs/serve-6540.txt`.
  New probe `s16-row-open.mjs` (hard load `/spaces/<id>` → route admission → sign in → open a named row by its own open button → trace).
  Found on the way (item 3): the Space-index row's open button is named `Open: artifact-<id>` in a German shell (English verb + wire
  id instead of the document name) — the staged space guest predates U5's localized cells (restage); re-check after W3's restage.
- 19:4x hub **8041** = C11's current-tree recipe: B2 catalog clone (`f485bf7e…`, 16 kinds) + a copy of C11's 19:07 `os-hub`
  (`.🧬semio/🌐hub/s13-s16-bin/os-hub-hub-8041`, source sha256 `44294438…`), fresh root `s13-s16-hub-8041`, user1/2/3; hold **38102**
  (`wp-s16/s16-hub.sh`). Booting.
- 19:5x item 6 (coordinator: land before REBUILD START ~23:00) — **Home viewer sealed-page feed, written**:
  guest (space home): the config lane's one-item preparation (`HomeConfigPreparationFactory`), the page budget
  (`HOME_DIRECTORY_PAGE_BYTES`), the retained contract and the page emission (`HomeConfig::directory_event_page_emit`, the old
  editor handle body) moved into the shared config module (`✏️editor/🎚️config/🦀️.rs`); the editor's `applyDirectoryEventPage` routes
  through it; the viewer gains `HomeViewCommand::ApplyDirectoryEventPage` + a retained config-only job factory
  (`HomeViewCommandJobFactory`, Migrated, bounded proof `s.space.home@1/*#viewer`) + manifest view action (Chrome audience);
  language-neutral fixture `👁️viewer/🧫️fixtures/📬️directory-feed/🔣️.json` + 2 viewer laws (editor/viewer parity per page, serde_json
  as the third-party reader of the projection; manifest/route shape). Host: `directoryHomeOwnerAppV1` (`📇️directory-bootstrap`) — the
  owner opens for the visible Home in EITHER role; ShellHost uses it. Law `📇️directory-home-bootstrap` **15/15** (new row set
  `ownerSurfaces` + AJV `contains` oracle; red with the old editor-only rule: `home-viewer: expected false to be true`). Renderer
  `tsc`: 7 errors, none in my files (peers: `🔲️pixels/✍️editing`, `♿️accessibility`, `Paint2dHost` test, `Interpreter` story).
  Originals kept in `wp-s16/patches/home-viewer-feed/orig/`.
- 20:0x rule 22 (memory): stopped hub 8040 (hold 17048 / hub 17050), serves 6540 (31119 → vite 31415) and 6541 (18320 → vite
  18731), all by pid. Hub **8041** ready 19:51 (hold 38102 → hub 38104) — kept for item 2.
- 20:1x item 2 on the current-tree hub **8041** (B2 clone): serve **6541 → 8041** (pid **65861**, HMR off, log
  `s13-s16-logs/serve-6541-8041.txt`), space `S16 B2 Sweep` (`01a0def3-6eb7-…`), de sweep `c1` 0–15 detached (pid **66173**, log
  `s13-s16-logs/c1-sweep-de.txt`). **Rule 23 (20:1x): hub edits are refused in React until the rebuild** (LD's envelope wire
  change in the TS twin vs the 19:07 hub) — so this sweep classifies only create → saga-open; every verb row reads
  `edits [0,0,0,0]` and is void.
- 20:2x–21:1x item 6 checks: my first two `cargo check -p semio-s-artifact-space-home --features component-app-assembly --lib
  --tests` died in the build-dir lock convoy (one killed by the coordinator, one stopped by me after 20 min with 0 rustc).
  Run 3 compiled and found 3 errors in MY new viewer test (i64 instants, `expect_err` on a non-Debug emit) + 5 unnecessary
  qualifications of mine → fixed. C11 (who landed a `apply_directory_event_page` hunk in the same config file, coordinated by
  message) ran native `--lib --tests` rc 0 (20:54) and **wasm32-wasip2 `--lib` rc 0 (~21:00)** over both our edits. Run 5 on
  `build-fleet-b` (rule 26, cold) in flight (pid 5956).
- 21:1x–21:30 (before the usage cut): Space index table now LEADS with the Name column (`SpaceIndexTableLabels::columns/row`,
  `semio-s-artifact-space-space`): the grid names each row and its open button by the first cell, and a leading id read
  "Open: artifact-5bd9…" (en) / the same English+id in de. Law updated (`table_row_projects_the_seven_worker_brief_columns…`).
  HTTP reproducer for the slow hub creation lane written: `wp-s16/s16-creation-timing.ts` (not run yet). Requests file
  `wp-w3/requests/s16.txt` (space-home + space-space). B2 de sweep `c1` on 8041 reached row 12 before the cut; interim
  classification (verbs void under rule 23): block2d, draw, wfc2d, bitmap, grid2d, block3d, wfc3d, grid3d, block5d **created +
  opened by the saga**; puzzle2d, puzzle3d, puzzle5d **stayed `accepted` 5 min** (hub-side, same as S15/H9 on 8040/7800) —
  no shell-side cause found in rows 0–11.
- 05:00 (09-27) resumed after the overnight cut (rule 28): every process of mine is gone (hub 8041, serve 6541, sweep chain).
  Reconciled: all my session-13 edits are in HEAD (`40a2736e661`, 22:00 auto-commit) together with C11's `after=0` hunk in the
  same config file; nothing half-applied. Native check of both space crates (`--lib --tests`, build-fleet-b, private target)
  relaunched detached (pid 91898, `generated/s16-check-home-6.txt`).
- 05:05–06:40 native re-check of space-home + space-space (`--lib --tests`, build-fleet-b): 3 attempts, each parked in the
  build-fleet-b lock convoy (0 rustc for 10–30 min; one stopped by the coordinator, two by me). Landing row written 06:3x
  (📓️landing.md) with C11's 20:54 native / ~21:00 wasm32 greens + host laws **24/24** re-run on HEAD 06:2x; per rule 29 the
  rest is REBUILD fast gate/components. Item 3 UX measured (table). Item 2 classified (table).
- 06:4x found on the way (item 7 verification, laws): `🧪️tests/🗄️plugin-module-store` (S15's) ended with a Vitest **unhandled
  rejection** (`PluginModuleUnavailableError … hub index is unreachable`) although 43/43 passed — the fake-timer `settle` helper
  returned `pending.finally(…)`, a promise that rejects while the helper still advances timers and gets its handler only when
  returned. Fix: `settle` observes `pending` with both handlers, awaits the observer, returns `pending`. After: **43/43, 0
  unhandled errors** (`generated/s16-law-module-store-3.txt`; runs 1–2 were killed by the 15 min test budget under load, no
  result). Registry-union resolution law (`🔍️plugin-module-resolution`) green in the same run; `hubCatalogOnlyPluginsV1` still
  wired in ShellHost (item 7 source check); live re-proof after the rebuild.
- 06:4x stopped serve 6540 (w2-detach 55663 + orphan vite 55676, by pid) — not needed until W3's activate-s.
- 10:55 (after a second cut 07:0x–10:5x) REBUILD START was 09:53 (chain b3, W3). V1 ports my tool-run and hub-document sweep
  harnesses into permanent verbs; I run the ticket scripts post-restage meanwhile.
- 11:0x W3's REBUILD run 1 compiled space-home + the space plugin component (wasm32): 0 errors, 0 space-home warnings (run 1
  failed on raster + forms, not mine; chain relaunched 10:55). Landing row updated.
- 11:0x hub **8042**: catalog-less, CURRENT-TREE `os-hub` (W3's 11:04 prewarm build, copied + signed to
  `.🧬semio/🌐hub/s13-s16-bin/os-hub-hub-8042`, sha256 `bf994e77…`), fresh root `s13-s16-hub-8042`, user1/user2; hold **27630** →
  hub 27632 (`wp-s16/s16-hub-nocatalog.sh`, U5's catalog-less hold). Serve **6541 → 8042** (own session, pid **33188**, log
  `s13-s16-logs/serve-6541-8042.txt`).
- 11:1x item 3 cross-device: `s16-customization-probe.mjs` (U5's) run c1 (user1): device A's Dark/Deutsch/⌃⌥K reached the HUB
  (`s16-preference-page.ts`: 3 `user.preference-recorded` events) but device B stayed light/en/⌘Z. `s16-preference-device-b.mjs`
  (new; traces the worker wire): B posts `preference-lane-open` and gets `preference-lane-failed transport` 17 ms later with no
  request — root cause and fix in the table (worker allowlist). After: B folds the page at +17 ms after the open (b3) and the
  full probe c2 (user2, fresh devices) PASSES appearance, language and keybinding; named layouts FAIL (design gap, table).
- 11:3x stopped serve 6541 (33188 → vite 33280) and hub 8042 (hold 27630 → hub 27632) by pid (swap 24.1/25.6 GB, rule 22);
  restart recipe: hold `python3 wp-s16/s16-detach.py <state>/hold.txt bun wp-s16/s16-hub-hold-nocatalog.ts 8042
  .🧬semio/🌐hub/s13-s16-hub-8042 .🧬semio/🌐hub/s13-s16-bin/os-hub-hub-8042 .🧬semio/🌐hub/s13-s16-hub-8042-state`.
  Named-layout sync (item 3 gap) design for after PUBLISH DONE: `NamedLayoutStore` (`🖥️platform`) is a save/remove map over
  `semio.os.config › namedLayouts` — CRUD outside the event log; it becomes a projection of a new `setNamedLayout { appId,
  layoutId, layout | null }` ui-preferences mutation (data class persistedShared) with its Rust twin in `semio-framework-os-config`
  (guest-linked via renderer-wgpu → frozen until PUBLISH DONE) and a row in the shared `updates-every-os-ui-preference` fixture.
- 11:4x **post-restage run book (items 4–7), prepared.** When W3 reports `activate-s` + `verify-s` green:
  1. serve 6540 local-only: `python3 wp-w2/w2-detach.py <log> env S_OS_PORT=6540 S_LOCAL_ONLY=1 SEMIO_VITE_HMR=0 bun <dev pkg>/📜️script.ts
     serve s react dev` (cwd-independent; own session), census via `program-matrix` itself (live `__semioOsCatalogProbe`).
  2. `bun nx run @semio-tech/framework-os-dev:program-matrix -- http://127.0.0.1:6540/ --tag s16-r1en` → `--locale de --tag s16-r1de`
     → `--roles viewer --tag s16-r1viewers`; then `tool-run-matrix -- http://127.0.0.1:6540/ --tag s16-t1en` (+ `--locale de`).
     Every red: host side root-fixed here, guest side → main with the row + fault.
  3. item 6 live: hub 8042 (current-tree, fresh root) + serve 6541 → 8042; sign in, switch Home to Viewer → the hub rows appear
     (was 0); Space-index row buttons read "Öffnen: <name>".
  4. items 5 + 7 need the rebuilt 7800 (B3, then all): `hub-document-sweep -- <serve → 7800> --tag s16-h1en --space "S16 Sweep"`
     (+ `--locale de`), `--puzzle-saga-ms 900000`; item 7 = the registry-union probe (`wp-s16/s16-unstaged-front.ts … unregister`
     + `s16-registry-union.mjs`) against 7800.

