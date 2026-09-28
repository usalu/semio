# F3 — runtime performance of the os `s` React shell (successor of [F2](📓️wp-f2.md), [F1](📓️wp-f1.md))

Slice F3, session 14 (2026-09-27 18:4x). Coordinator = main Claude Code chat. Rules: `📓️session-14-preamble.md` (+ 13/12).
Ports: hubs 8200–8209, serves 6700–6709. Scripts `wp-f3/`, expendable captures `wp-f3/generated/`, durable data/logs
`.🧬semio/🌐hub/s14-f3-*`. Hub 7800 (W4's, catalog B3) read-only live use.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Session 14

| # | item | state | evidence |
|---|---|---|---|
| F3-1 | runtime perf sweep: cold `dev s` boot timeline, first paint, TTI, hub document open (7800), hub catalog install, typing/paint latency (note, writer, draw, puzzle), hover latency → root-fix top offenders (host TS now, guest as prepared patches) + budget laws, before/after | **in progress** — offenders found + fixed: (a) **serve start: first HTTP 200 65 s → 34.6 s (load 80 → 116), freshness pass sync prefix 48 900 → 63 ms — LANDED** (one async bounded source walk; a receipt change no longer freezes Vite; laws 56/56, red share 1.000 → 0.000); (b) **writer typing regression 2.08 → 1.0 paints per key: LANDED** (echo projection drops the buffer's lane ref; law echo-pack 5/5, red 3 FAIL; live 12 keys → 12 paints + cadence). Cold browser boot at load 100: 453 modules / 40.2 MB JS (F2's win holds), warm reload 0.14 MB; timings not judgeable (load 88–113) | `f3-boot-s14-a1.json`, `f3-paint-stacks-writer.json`, `f3-echo-diff-writer*.{json,txt}`, `test-echo-pack-{2,red}.txt` |
| F3-2 | re-verify F2-1 (one stream-mux channel per origin, no socket-pool stalls) + F2-3 (idle 148/148) on the current tree; fix regressions | **F2-1 holds (measured)**: gate PASS on :6700 — 0 idle HTTP holds, 1 stream-mux link, 64 + 2 streams, 300/300 fetches ≤ 20 s, notices 64 + 32 exact; NetLog `SOCKET_POOL_STALLED_MAX_SOCKETS_PER_GROUP` 858 = queueing of the 300-fetch burst + 453 module requests through 6 HTTP/1.1 connections (queue p95 2.55 s at load 96), no request held. F2-3 idle: open | `connection-budget-a.txt`, `🧑‍💻dev/🤖️generated/🔀️connection-budget/s14-f3-a/` |
| F3-3 | permanent perf budget check (rule 17): verb in os-dev `📜️script.ts`, acceptance record, `RELAY R10:` target spec | open | — |

### Session 14 log

- 18:49 read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `📓️wp-f2.md`, `📓️wp-f1.md`, memory
  notes (hover anatomy, connection budget). Machine: load 82 / 95 / 69 (10 cores), swap 11.8/13.3 GB, 135 GiB free, 3 rustc,
  14 headless chromium processes (peers). 7800 = os-hub 4548. Nothing on 6700–6709 / 8200–8209.
- 18:5x serve **6700** started (`wp-f3/f3-serve.sh 6700 a` → w2-detach pid **61717**, `S_HUB_URL=http://127.0.0.1:7800 S_LOCAL_ONLY=1
  SEMIO_VITE_HMR=0`, log `.🧬semio/🌐hub/s14-f3-logs/serve-6700-a.txt`): first HTTP 200 after **65 s**; Vite itself "ready in 3522 ms",
  spawned ~60 s after the serve process. Phase probe (`wp-f3/f3-serve-phases.ts`, same steps as `ServeScript`, load 59–80): imports
  2.9 s, receipt 1 ms, **`reportServeStagedModuleFreshness` synchronous part 48.9 s**, `ensureDevLocalHub` 0.1 s. Per-plugin probe
  (`f3-freshness-probe.ts`): **57 of 60 staged module dirs have no `.source-stat-index.json`** (materialized 16:52 by the failed final
  chain, never activated after) → `resolveBootSourceContentHashesAsync` falls back to the fully synchronous
  `buildComponentSourceStatIndex` (readFileSync + sha256) over **54 131 source files** (stdio 10 937, norm 8 595, energy 6 780…) before
  its first `await`, i.e. before Vite is even spawned. Same sync walk in the Vite process: `reportActivationFreshness` runs
  `resolveBootSourceContentHashes` (sync) on every receipt change while serving → the serve's event loop (HTTP, HMR, stream-mux beats
  with a 25 s dead-link watchdog) freezes for the whole walk.
- 19:0x cold browser boot (`wp-f3/f3-boot.mjs` = F2's probe, `f3-boot-s14-a1.json`, load 96–113): cold DCL 44.4 s / FCP 44.6 s /
  all 60 plugins 50.7 s — the page is idle 48.5 of 50.6 s, waiting on Vite's first transform of the module graph (slowest module
  responses 28 s, queue up to 26.5 s); **453 modules, 40.2 MB JS, 120 descriptor JSON 17.6 MB, font pack 8.4 MB** (F2's drei/three-stdlib
  win holds). Warm reload: FCP 3.0 s, all plugins 5.4 s, transfer 0.14 MB (304s). Timings are load-bound, counts are the verdict.
  The probe's `data-semio-os-ready` observer never fired (`ready: null`) → the permanent harness polls the dataset (program-matrix
  `awaitBeacon` semantics).
- 19:02 **F2-1 re-verify: PASS** (`bun ./📜️script.ts verify connections http://127.0.0.1:6700/ --tag s14-f3-a`, capture
  `wp-f3/generated/connection-budget-a.txt`).
- 19:04 latency gate (`verify latency http://127.0.0.1:6700/ --tag s14-f3-a`, `latency-a.txt`, load 88–98 → timings blocked):
  **writer-typing FAIL 2.08 paints/input** (F2 left it at 1.13), jack 2.17 (bound 2.5, by design), draw 1.06, dag 2.06.
- 19:10 writer anatomy (`f3-paint-stacks.mjs`, `f3-echo-diff.ts`): every key paints twice — the local edit, then the guest echo
  140–210 ms later; the echo pack differs from the last synced pack only in **`lanes`** (`[{lane:"buffer", bytes, hash}]`, the paged
  text carrier's ref of the buffer, added since F2): `sceneWithoutEchoedText` dropped `buffer` + `selectionJson` but not the ref that
  describes the buffer. Fix (19:14) + law; my first version read `TEXT_EDITOR_SCENE_LANES` at module level → TDZ inside an import cycle →
  every `s` boot failed (C12 caught it, 19:2x); fixed to a call-time read; boot to Home `ready:s`, 0 pageerrors; tsc 0 errors; laws 34/34.
  **After: 12 keys → 12 key paints + 1 first-echo sync + 3 caret-cadence paints; 1 pack synced in 12 keys (before: 26 paints, 8 syncs).**
- 19:55–20:10 **serve-start fix landed** (landing row "serve start no longer blocks"): patches `wp-f3/patches/f3-freshness-async.py` +
  `f3-freshness-callers.py` (idempotent, applied). One async walk (`walkSourceStats`, 32 files / 4 components in flight), every
  sync twin deleted, callers await, the Vite plugin's receipt pass is cancellable (a newer receipt aborts the older). Laws: staging-root
  56/56 incl. 3 new (yielding share ≤ 0.1 + a ticking timer + WebCrypto SHA-256 oracle over the digests; cancellation; bound/order).
  Red (`f3-freshness-red.ts` over HEAD's activation module): share 1.000, 0 ticks → FAIL; fixed: 0.9 ms of 2 332 ms, 185 ticks → PASS.
  Live: serve 6700 restarted (`f3-serve.sh 6700 b`, pid **97743**, log `serve-6700-b.txt`): **first HTTP 200 after 34.6 s** (was 65 s,
  load now 116 vs 80), Vite ready 5.2 s; phase probe (`serve-phases-after.txt`): freshness sync part **63 ms** (was 48 900 ms), the pass
  settles in the background at 76 s and prints its 18 `[stale]` lines then; boot to Home `ready:s`, 0 pageerrors. Vite plugin live
  check (`f3-vite-freshness-live.ts`: real watcher, 3 000-file tree without index, 2 receipt changes 30 ms apart): 1 `[stale]` line
  (superseded pass silent), max event-loop gap 169 ms. Remaining serve-start cost: module imports 2.6 s (1.8 s of it the freshness
  module's registry/store imports) + Vite boot + the first request's transform.

### Session 14b

Successor F3 (2026-09-28 12:0x). Load 45 → 32 (10 cores), swap 4.5/6 GB, 3 rustc, chain running (guest freeze ON).

| # | item | state | evidence |
|---|---|---|---|
| F3-0 | reconcile predecessor's in-flight host TS (ShellHost spawned-window dispatch, Shell reducers) | **done — kept, verified**: hunks are in HEAD `5bcb2da` (21:54) and the live tree (💻️os == HEAD); tsc 619 files rc 0; boot to Home `ready:s`, 0 pageerrors | `generated/tsc-shell-1.txt`, `generated/home-boot-c1.txt` |
| F3-1b | world hover anatomy + root fix (host TS) | **LANDED ×2** (spawned bodies out of the shell reducer; local interaction out of `ShellState`): per puzzle3d hover transition shell renders **2 → 0**, react-dom fibers 862 → 102, React render **94.6 → 29.0 ms**, long tasks **16 × ~70 ms → 0**; next offender: guest echo = 3 surface patches through the retained intake (`acceptOwnedUiPatches` ~54 ms/transition; hover rides the selection lane) — frozen code, prepared-patch territory | `f3-render-census-puzzle-*.json`, `f3-hover-puzzle-{e,f}.json`, `latency-{b,c}.txt` |
| F3-1c | R3F per-hover re-renders (host TS) | **LANDED** (`WorldCanvas` Canvas memo + children slot): R3F commits 13 → 5, fibers 578 → 290, render 17.7 → 11.2 ms per transition; click-select + pan smoke clean. Total React render per hover transition now **22.5 ms (was 94.6)** | `f3-render-census-puzzle-after3.json`, `world-click-b.txt` |
| F3-3b | perf budget harness (rule 17) | **LANDED + run live**: `verify boot` (`🧑‍💻dev/🧪️tests/🥾️boot-budget/🟦️.ts`, fixture `🧫️fixtures/🥾️boot-budget.json`, law `🧮️reducers/🟦️.ts` over 5 oracle vectors); live :6700 BLOCKED (payload within budget, timings load-bound); target spec relayed to R10; law vitest queued in the native lane | `boot-budget-{a,b,c}.txt`, `tsc-boot-budget-5.txt` |
| F3-2b | F2-1 / F2-3 re-verify | **both PASS**: idle 148/148 (4 resumed runs), connection budget PASS (1 stream channel, 64 streams, 0 idle holds, 300/300) | `idle-s14b-{a..d}.txt`, `connection-budget-b.txt` |

- 12:10 reconcile: the predecessor's last edits (20:2x–20:42) = `interactionReducer` structural sharing via `mergeRecordPreservingIdentity`
  (a hover-only guest observation keeps every other field's identity), `spawnedWindowReducer` identity bail on `SET_SPAWNED_WINDOW_UI`, and
  the spawned-window dispatch sites in ShellHost (`refreshSpawnedUi`: UI / engagements / measures dispatched as functional
  `mergeRecordPreservingIdentity` updaters instead of fresh records) — all landed in the 21:54 auto-commit; its reducer test ran 33/33 at
  20:42 (`generated/test-shell-reducer-1.txt`). The 20:40 TDZ (`documentProgramFocused`) is not ours and is gone (declared l.4948, used
  l.9553). RELAY S18 sent: S18 owns ShellHost 🪟️TreeWindows (~l.2605–2665), refreshUi viewState/treeWindows (~l.5790–5805) and the sync-card
  URI adapter; F3 holds edits there.
- 12:12 rule 20 proof: `tsc -p wp-f3/tsc/tsconfig-shell.json` (Shell + ShellHost + ShellHelpers closure, 619 non-node_modules files) rc 0 in
  72 s; serve 6700 via S18's `ensureDevServe` (`wp-f3/f3-ensure-serve.ts 6700 c`, hub 7800, pid 51115, log
  `.🧬semio/🌐hub/s14-f3-logs/serve-6700-c.txt`): **first HTTP 200 after 3.8 s** (Vite ready 1.5 s; was 65 s before F3's freshness fix,
  34.6 s after it at load 116), boot to Home `ready:s`, 1 window, 0 pageerrors in 10 s.
- 12:14 latency gate on the current tree (`latency-b.txt`, load 41–45 → timings not judged): writer **1.13** paints/key (the 19:14 fix holds;
  was 2.08), jack 1.88, draw 1.06, dag 2.06 — all within bounds; p95 9.4 / 35.8 / 24.2 / 19 ms. Cold boot (fresh profile, Vite warm,
  `f3-boot-s14b-b1.json`, load 36): TTFB 0.39 s, FCP 6.2 s, ready 8.7 s, 465 modules 40.7 MB + 120 descriptor JSON 17.7 MB + font pack
  8.35 MB + CSS 1 MB; warm reload FCP 1.0 s, ready 2.7 s, 0.15 MB transferred.
- 12:2x hover anatomy. The predecessor's commit census counted a fiber's stale PerformedWork flag in bailed-out subtrees (identical
  numbers run after run); `wp-f3/f3-render-census.ts` uses React DevTools' own semantics (identical child pointer = bailed out),
  `actualDuration` and state-hook diffs. Idle 4 s: 0 commits. Per hover transition (`before1`, load 38): 20 commits (13 R3F + 7
  react-dom), 862 react-dom + 578 R3F fibers, React render 94.6 ms, of it FrameworkOsShellInner 2 renders / 69 ms; triggers: (a) every
  hover changes all three puzzle3d scene bodies — only the `selection` lane ref hash (`…lane selection … hash 68419a17… → 7b06a7a1…`, the
  hover lives in the selection lane), (b) `INTERACTION_STATE_OBSERVED` (hover). Whole-shell cascade because FrameworkOsShellInner's return
  tree is unmemoized (Icon 60, ChromeControlHint 50, LevelProvider 48 re-renders with equal props per transition). CPU profile
  (`f3-hover-puzzle-e.json`): one 65–74 ms long task per transition = retained UI intake of the 3 surfaces (`acceptOwnedUiPatches` 53 ms)
  + shell render; dev `jsx()` alone 63 ms/transition.
- 12:4x fix (a) landed (row in `📓️landing.md`): the reducer keeps only which spawned windows have a body + its activity, bodies are
  published into their `spawned:<windowId>` stores outside render. After (`after1`, `f3-hover-puzzle-f.json`, load 34): 1 shell render
  (26 ms), react-dom 862 → 478 fibers, React render 51.3 ms/transition, **0 long tasks** (was 16), hover → paint p50 3.6 ms; the guest echo
  still repaints (body updates reach World3dHost). Latency gate after: 4/4 editors open and paint (`latency-c.txt`). tsc rc 0, boot to Home
  clean. engine-contract vitest queued in the native lane (8 cargo jobs ahead).
- 13:0x fix (b) landed (row in `📓️landing.md`): local interaction is an ephemeral store, not a `ShellState` slice (nothing rendered
  read it). After (`after2`, `f3-hover-puzzle-g.json`, load 46–54): **0 shell renders** per hover transition, react-dom 102 fibers /
  11.3 ms, R3F 578 fibers / 17.7 ms, `performWorkOnRoot` 418 ms per 16 transitions (1 604 before both fixes), 0 long tasks, hover →
  paint p50 2.8 ms. tsc rc 0, boot to Home clean; test files' tsc + vitest queued in the native lane.
- 13:1x R3F anatomy (`r3f1`): the r3f store's `size` + `viewport` got new (value-equal) objects 2× per canvas per hover; cause =
  r3f 9.7 `configure` (runs on every `<Canvas>` render) compares react-use-measure's 8-key rect with the 4-key stored size via
  `is.equ(…, shallowLoose)` — `for (i in a) if (!(i in b))` fails on `bottom` — so `setSize` fires on every Canvas render, and
  `WorldCanvas` re-rendered `<Canvas>` on every World3dHost render (inline style/dpr/gl/handlers, fresh children). Fix landed (row in
  `📓️landing.md`). After (`after3`, load 29): R3F 5 commits / 290 fibers / 11.2 ms per transition; react-dom 90 fibers / 11.3 ms.
- 13:1x hub document open latency on 7800: NOT measurable now — coordinator broadcast 13:0x: current-tree serves cannot open documents
  on 7800/B3 (channel 18 vs 19) until 7800 is on ALL.
- 13:1x item 3: `verify boot` harness written (payload per kind always judged, timings under the load ceiling; ensureDevServe reuse or
  start → serve first-answer ms), tsc rc 0 (320 files), law registered in the os-dev vitest config; RELAY R10 sent (target
  `boot-budget`, new dirs `🥾️boot-budget/`, fixture). Still to run: live `verify boot` + the law (queued native lane ticket).
- 14:0x F2-3 idle census PASS 148/148 (runs a–d, interrupted twice for the browser slot, `--resume`); F2-1 connection budget PASS.
- 14:1x `verify boot` live: first run FAIL exposed calibration errors (19 code-imported JSON modules counted as descriptors; 4 css-pulled
  svg cursors as style; a non-persistent context has no HTTP cache → warm reload 67.7 MB) → classifier + persistent temp profile fixed,
  oracle updated, vectors regenerated (`--check` clean); runs b/c BLOCKED = payload within budget, timings not judged (load 37–40).
  Boot payload offenders for window 3 (frozen `🔌️plugin/` code): the guestslim typst font pack (8.35 MB, fetched on every boot before
  any typst consumer opens) and 120 plugin/extension descriptors (17.7 MB JSON at boot).
- Next hover offenders (not landed): (1) guest echo — every hover rides the `selection` lane whose hash sits in all three puzzle3d scene
  bodies → 3 surface patches through the retained intake (`acceptOwnedUiPatches` ~54 ms CPU/transition, `patch.install` p50 40 ms);
  guest/framework-ui side, frozen now; proposal: hover through the ephemeral interaction channel, not a document lane referenced by the
  body. (2) `publishLeftoverWorldSelectionV1` mints a new document overlay (fresh equal `ids` array) on every publication → every pane's
  World3dHost re-renders (uSES on the document overlay, 2/transition) and re-renders its canvas children; fix = structural sharing +
  a per-pane memoized overlay snapshot (World3dHost, host TS) — needs vortex-hover highlight coverage before landing.
- 14:2x serve 6700 (ensureDevServe pid 51115) stopped — idle (rule 21).
- 14:18 native-lane run (`wp-f3/f3-shell-tests.sh 3`): boot-budget law **6/6 PASS** (`test-boot-budget-3.txt`); tsc of the two shell
  test files: 8 errors, all pre-existing peer drift (5× TS2345 `projectionFrame` missing in `WorldParsedCameraState` fixtures, 3× TS2339
  `component.value` at l.4993–5008, present in HEAD) — none in this session's hunks (`tsc-shell-tests-3.txt`); both shell vitest runs
  found no files (invoked without the react package's `--config ../../🧪️tests/🎚️config/🟦️.ts`) → requeued as `f3-shell-vitest.sh 4`.

### Session 14c

Successor F3 (2026-09-28 16:5x). Chain relaunched 16:55 (guest freeze ON), hub 7800 DOWN, load 45 → 74 (10 cores).

| # | item | state | evidence |
|---|---|---|---|
| F3-0c | reconcile predecessor's last step (Python oracle ↔ TS twin on the same kind spec, script-imported JSON vector, regenerated fixture) | **done — nothing half-applied**: oracle `kind_of` == `bootResourceKindV1` (script-initiated `.json` = module, css-pulled svg = other), vector 0 carries `🔣️.json?import`/script; tree == HEAD `dfe2687`; oracle `--check` rc 0, tsc (harness closure) rc 0, law **6/6 PASS** (native lane) | `generated/{oracle-check,tsc-boot-budget,test-boot-budget}-14c-1.txt` |
| F3-1d | next hover / typing / paint offenders (host TS now, guest as prepared patches) | **LANDED (hover set) + budget laws**: leftover overlay structural sharing + per-pane snapshot + no doc seeding by a pane's first hover (World3dHost), ContextMenu closed-render fix; puzzle3d hover per transition react-dom commits 6 → **2**, R3F fibers 290 → **220**, React render 21.8 → **17.2 ms**; stale sibling-pane hover fixed; laws leftover 13/13 (red on HEAD 2 FAIL), ContextMenu 10/10 (red 12 vs 4 commits); latency fixture + `puzzle2d-drag` (1.13 paints/input) | `f3-render-census-puzzle-c14-*.json`, `world-hover-panes-c14-{head,2}.txt`, `test-*-c14-{6,7}.txt`, `latency-14c-{a,b}.txt` |
| F3-1e | writer typing offender (ShellHost, coordinator GO both regions 18:0x) | **LANDED ×2**: (1) per-program history → `createProgramHistoryStoreV1` + live History tree (`liveTreePanelDefinition`, new ui Panel live source) + `data-history-json`/auto check-in off the store; (2) panel bodies → `panelBodyStoreByKey` (one store per panel, loaded outside render). Writer per key: shell renders **3 → 0.04**, react-dom fibers **808 → 21.7**, React render **46.7 → 1.9 ms**; commits/key 5.04 → 4.04; History + Inspection smokes live, 0 pageerrors | `f3-render-census-type-writer-c14-{before1,after1,after2}.json`, `history-smoke-c14-1.txt`, `inspection-smoke-c14-1.txt`, `test-*-c14-{9..12}.txt` |
| F3-3d | budget laws for typing/hover (latency gate v2) | **LANDED**: `verify latency` judges React commits/input (DevTools hook) + `hover` gesture; rows `puzzle2d-drag`, `puzzle3d-hover`; commit bounds tightened after the fixes; schema defs for latency + boot-budget fixtures, jsonschema laws 11/11 | `latency-14c-{c..f}.txt`, `test-dev-laws-c14-13.txt` |
| F3-4 | hub document open latency (7800) | **BLOCKED** — 7800 is down until the chain moves it onto ALL (chain at rebuild-all after hub-prewarm rc 0 17:13) | — |
| F3-3c | one real `verify boot` run → acceptance record | **BLOCKED (measured, correct verdict)**: harness started its own serve via `ensureDevServe` (6700, local-only, stopped after); payload within budget (module 462/40.36 MB, json 120/17.83 MB, font 16/9.34 MB, style 3/0.36 MB, other 6/0 MB); timings not judged at load 68.7: serve 11.4 s, cold FCP 16.0 / ready 19.2 / plugins 19.4 s, warm ready 3.0 s | record `.🧬semio/🌐hub/s14-f3-acceptance/boot-budget-14c-b.json`, `generated/boot-budget-14c-{a,b}.txt`, `🧑‍💻dev/🤖️generated/🥾️boot-budget/s14c-f3-b/boot.json` |

- 17:00 reconcile: laws via `zsh 📜️fleet-mutex.sh native f3 -- zsh wp-f3/f3-boot-budget-laws.sh 14c-1` (new script: oracle drift + tsc + vitest).
- 17:01–17:03 `SEMIO_ACCEPTANCE_RESULT=… bun ./📜️script.ts verify boot --serve http://127.0.0.1:6700/ --tag s14c-f3-{a,b}` (os-dev
  `📦️packages/🟦️typescript`): both BLOCKED (load 73.8 / 68.7), rc 0, 59 s / 47 s wall; serve first answer 11.2 / 11.4 s (Vite cold
  after the app restart; 3.8 s warm at 12:12).
- 17:04 latency gate on the current tree (`latency-14c-a.txt`, load 58–63, timings not judged): writer 1.13, jack 2.08, draw 1.06, dag 2.09
  paints/input — all within bounds (the writer echo fix of 27 19:14 holds). Trial scenarios: `puzzle2d-drag` 1.13 paints/input (`BoardSession`)
  → kept in the fixture as a budget law; `note-ink` dropped — the note canvas is DOM/svg (`f3-window-body.ts`: no `<canvas>`, no `renderFrame`
  session), the paint hook cannot count its paints.
- 17:12 hover census baseline (`c14-before1`, load 42.5) = the 13:1x state: per transition react-dom 6 commits / 90 fibers / 10.8 ms, R3F 5 / 290 /
  11.0 ms; roots: `World3dHost` uSES@14 2× (the document slot as epoch — `{ids=json}`: every publication minted a fresh equal `ids` array),
  `ContextMenuController` state@2 4× (closed menu reset `setActivePath([])` on every `items` change, plus fused-panel memo churn → HEAD renders a
  closed menu **3×** per parent render), guest echo `UiNodeView` uSES@0 2× (frozen).
- 17:2x–17:4x fix set landed (landing row "hover re-render set"): after (`c14-after1/2`, load 40–49): react-dom 2 commits / 67 fibers / 8.2 ms, R3F 5 /
  220 / 9.0 ms, World3dHost re-renders only on its own hover change (1/transition, `{hoveredId≠}`), ContextMenuController gone from the roots.
  Found on the way (pane smoke `f3-world-hover-panes.ts`, same on HEAD): a pane's FIRST window-scoped leftover became the whole document slot, so the
  sibling pane (MainTop) painted that hover and kept it after the pointer left → fixed (neutral window fields), now `noStaleHoverAfterLeave: true`.
  Still by design of the guest: while hovering, MainTop also shows the hover — the guest's `selection` lane carries hover for every pane (offender 1).
  Rule 20: shell tsc rc 0 after every source edit, boot to Home `ready:s` 0 pageerrors (`home-boot-c14-{1,2}.txt`).
- 17:5x writer typing census (`f3-render-census-type.ts`, the hover census driven by keys, load 50): per key 4.96 commits, 808 fibers, 46.7 ms React
  render, **3 whole-shell renders** — 2 from the history projection (`useState` in ShellHost: entries/cursor change twice per key), 1 from
  `windowUi.panelUiByKey` (the writer Inspection body republished per key → the tab's config minted a NEW `UiDocumentStore`). RELAY S18/SH2 →
  coordinator GO for both regions (18:0x, keeping S18's TreeWindows/refreshUi/onAction-head lines and SH2's route-B anchors intact).
- 18:1x region (1) landed (landing row): after = 1.04 shell renders/key (the 0.04 = one `canUndo` flip). Found on the way: `applyHistoryPatch` read an
  `applied` flag set inside a lazy `useState` updater (not guaranteed to run synchronously) — the store's synchronous `update` fixes that.
- 18:2x region (2) landed (landing row): after = 0.04 shell renders/key, 21.7 fibers, 1.9 ms. Remaining 4 small commits/key (text editor surface,
  guest echo `UiNodeView`) cost ~0.5 ms each.
- 18:3x latency gate v2 landed (commits per input + hover; bounds writer 5 / jack 5 / draw 3 / dag 1.5 / puzzle2d 1 / puzzle3d-hover 8; measured 4.04 /
  4.0 / 2.22 / 1 / 0.22 / 7). Serve 6700 (pid 81340/81398, started 17:08) stopped 18:36.
- Guest part (prepared patch NOT written — design decision): the remaining puzzle3d hover cost is the guest echo — the guest writes the hover into
  every pane's `selection` lane BY DESIGN (5d/3d unit tests assert "the board pane paints the shared hover"), so each hover republishes 3 scene bodies
  (2 extra World3dHost renders + one redundant repaint ~150 ms later, `f3-hover-puzzle-c14-a.json`: hover → local paint p50 2.7 ms, echo paint p95
  158 ms). Options for window 3: (a) a hover lane outside the scene body's lane refs (host subscribes to the lane, bodies stay put), or (b) keep hover
  per window (B39) and drop the shared-hover feature. Needs the product call (linked-view hover yes/no) before a patch.

### Session 14c — hub latency (resumed 21:1x, 7800 READY 21:10 on `s14-w4-catalog-p24`; 7800 boot took **616 s**, H14 investigates)

| # | item | state | evidence |
|---|---|---|---|
| F3-5 | hub-document open latency per kind (first open vs warm) + two-client edit round trip, acceptance record | **running** — `verify two-human` (edit journey) now times each edit's arrival at the other human (40 ms poll, submit → seen) and the warm reopen (row click → mounted) besides create→mounted / open→mounted; probe on writer PASS 10/10: create→mounted 9.2 s, B open→mounted 8.6 s, edit seen by the other human 233–450 ms | `two-human-probe.txt`, run `two-human-hub-a.txt` |

- 21:1x serve 6700 via `ensureDevServe` joined to 7800 (pid 16595, ready 7.5 s). Two-human harness: + `awaitTextAt`, `KindTimings`
  (aEditLocalMs, aEditSeenByBMs, bEditLocalMs, bEditSeenByAMs, reopenAToMountedMs, reopenBToMountedMs), edits polled concurrently on
  both pages, acceptance measured + summary en/de gain edit-seen and warm-reopen p50/max; tsc rc 0. First probe timed the reopen
  from `openSpace` (≥ 12 s settle) → re-timed from the row click. 21:19 full run launched (pid 18888, tag `s14c-f3-hub-a`).
- 21:3x run a stopped after 2 kinds: the concurrent B poll timed PRESENCE changes (B's view moves before A submits → negative "seen" ms).
  Fixed: `watchText` records B's distinct texts with first-seen epoch ms; arrival = the first time B shows A's edited text after submit.
  Run b died at seeding: the fresh space never appeared on Home within 90 s (`no space row named …`, a hub/Home latency under load in
  itself); run c (21:50, pid 34743) reuses run a's shared space `01a0e976-2848-790e-95ed-749ce377b568`.
- 21:3x–21:4x **hub creation is the dominant open cost** (`f3-hub-creation-phases.ts`, HTTP only; `hub-creation-phases-*.{txt,jsonl}`,
  load 33–59): the `accepted` phase = `materialize_genesis` = the package's guest `codec.genesis` run by the hub's OWNED WASM INTERPRETER
  (`semio-framework-plugin-host`, `interpret_off_worker`, progress every 25 M fuel). Measured per creation, repeatable (not a cold
  cost): text.document 0.75–1.2 s (~5 M fuel), 2d.block 3.7–17 s (~15 M), s.stdio.md 26–72 s and s.stdio.csv 41 s (**~360 M fuel**,
  14–18 M fuel/s at load 37); open-plan answers in 1–63 ms. W4's cold open-plan probe (20:1x) had puzzle 2d/3d/5d 80/129/184 s, vcs
  118 s, gis 50 s, stdio 100–143 s. The same interpreter runs every `codec.pack-schema-hash` verification at hub boot (Cargo.toml's
  own note) → very likely the 616 s 7800 boot too (relayed to H14).
