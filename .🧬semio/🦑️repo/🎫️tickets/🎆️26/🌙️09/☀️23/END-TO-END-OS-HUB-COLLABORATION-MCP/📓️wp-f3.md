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
