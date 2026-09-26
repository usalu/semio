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
| 2 | B2 kind sweep reds in `s` (de 9/16): shell-side vs hub-side; fix shell-side, route hub-side to H11 | pending | |
| 3 | UX leftovers: keyboard/focus, WCAG, mobile/tablet, customization persistence, German completeness | pending | |
| 4 | full matrix on W3's consolidated restage (editors en + de, viewers, tool-run) + (coord 19:4x, `📓️audit-s13-plugins.md`) one row per EXTENSION component (loads with its parent, apps/kinds open, edit/undo/redo, en + de) | waiting for W3 | |
| 5 | hub-document sweep for every kind of every package (`--packages all`), en + de | waiting for the all-package publish | |
| 6 | (coord 19:2x, audit-s13 §4 #16) Home viewer lists 0 hub rows — read-only projection feed; root-fix + law | pending | |
| 7 | (coord, audit-s13 §4 #7) plugin absent from the local registry: installable from the hub? verify S12-6 on the current code, fix a gap | pending | |
| 8 | (coord, P0-3) a live `s` serve on my port, kept up for the matrix re-run | in progress | |

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

