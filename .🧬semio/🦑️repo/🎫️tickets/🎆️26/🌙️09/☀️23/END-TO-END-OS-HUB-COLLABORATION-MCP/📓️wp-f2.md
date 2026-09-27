# F2 — frontend transport + runtime performance of the os `s` shell

Slice F2, session 13 (2026-09-26 19:0x). Coordinator = main Claude Code chat. Continues [F1](📓️wp-f1.md) (runtime performance) and
the U5 16:3x connection finding ([U5](📓️wp-u5.md) §12-6, `📓️work-packages.md` 16:3x). Ports 8080–8089 / 6580–6589. Builds/tests
`nice -n 10`. Captures `wp-f2/generated/` (expendable); durable data/logs `.🧬semio/🌐hub/s13-f2-*`.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Session 13

| # | item | state | evidence |
|---|---|---|---|
| F2-1 | per-origin connection budget: inventory per origin at install time → ONE multiplexed channel per origin (schema-first frame contract TS + Rust twin, progress/cancel, backpressure, reconnect+resume), fixture law + third-party oracle, live before/after + stress | **done, measured live + permanent gate**: long-lived HTTP/1.1 streams on the serve origin **4–5 of 6 → 0**; every watch/folder/job stream on ONE WebSocket per page (workers bridged); stress 66 streams + 300 fetches on one link: 300/300 fetches, 96/96 folder notices (per-stream SSE contrast: 0/300 fetches in 20 s). Law 81/81 (AJV oracle), framework 265/265, store 9/9. Gate `@semio-tech/framework-os-dev:connection-budget` (`verify connections`, launch row, acceptance check `connection-budget`) **PASS** on 6580; its NetLog oracle is red on the pre-mux log (18 idle holds). Rust twin: none — no Rust consumer (checked) | `f2-conn-{hub2,after2}.json`, `f2-stress-stress1.json`, `f2-sse-contrast.txt`, `connection-budget-2.txt` |
| F2-2 | runtime performance sweep: cold `dev s` boot timeline, first paint, hub document open latency, hub catalog plugin install, typing/paint latency in 3 editors → root-fix top offenders + laws | **in progress** — boot profiled (`f2-boot.mjs`); root fixes landed + laws: file validators/304 (warm reload 25.4 → 0.1–0.7 MB), pack key comparator (no per-compare encoder), manifest encode/decode round trip removed (boot main-thread busy 3.8 → 1.9 s, long tasks 12 → 1–3); **typing paints: the latency gate's paint count was vacuous (0 classes hooked: the page's 250-entry Resource Timing buffer was full) → fixed + a scenario that hooks nothing now fails; then measured writer 4.13 / jack 3.67 paints per key → 1.13 / 1.79–2.13** (TextEditor no longer paints on its own and skips byte-identical echo packs); draw drag now measured (1.06). **Cold boot: 909 → 453 modules, 62.4 → 40.4 MB JS, FCP 2.4 → 0.73–0.90 s, ready 3.5 → 1.66–1.81 s, all 60 plugins loaded 3.5 → 2.0–2.2 s** (drei member imports + prebundled three-stdlib; law: browser-graph `denySpecifiers`). Gate at load 7.8–10.7: writer/jack/draw **PASS** (p95 48.7 / 36.9 / 28.8 ms), dag BLOCKED by load only (2.06 paints, p95 18.4 ms). Open: hub doc open / hub install (7800 down) | `f2-boot-{b2,a1,c1,c2,c3,c4}.json`, `interaction-latency-s13-{b,g}.txt`, `f2-paint-stacks-*.json` |
| F2-3 | idle cost re-verify (F1 146/146) on the current tree; memory growth over 30 min with 10 windows; + Vite HMR under Bun (root fix + law + live proof) | **done** — **idle: 148/148 PASS** on the post-rebuild tree (77 editors + 71 viewers; max 0.1 main frames/s, 0 rAF/s, 0 draws/s, max main busy 1.88 %, nothing after any close); gates `:idle-budget` + `:memory-soak` landed (launch rows, acceptance checks), reducer law 3/3. **HMR: done, live-proven** — served-module edit → page hot update in 475 ms, no reload (6581, launch-row default HMR); law `♨️hot-update` 2/2: exactly 1 update per write (in-place 45 ms, atomic 24 ms), unwatched control 0; fixed real-path replay (symlinked roots delivered 0) + `add`+`change` double replay. **Found + fixed: Vite restart/close under Bun wedges under a connected page** (unreachable after restart, 0 `close` events; node 4 ms) → `semioServeCloseVitePlugin` in all 5 serve configs: restart 9 ms, 1 close event, 1 update/write after; red without it; live config-entry edit on 6581 → reconnected 3.0 s, HTTP 200. **Memory soak: PASS** — 10 editors open for 30 min on 6580: JS heap 101.28 → 101.31 MB (0.001 MB/min), workers 40.19 → 38.97 MB (0 MB/min), DOM nodes 1632 → 1632, listeners 1263 → 1263, 0 frames/s | `idle.json` (s13-all), `soak.json` (s13-soak), `memory-soak-s13.txt`, `test-hot-update-{5,6,red}.txt`, `hmr-live.txt`, `f2-hmr-live.json`, `f2-vite-close.ts` |

### Session 13 log

- 19:0x read AGENTS.md, preambles 13/12, handovers `📓️wp-f1.md`, `📓️wp-u5.md` (§12-6 connection budget), `📓️work-packages.md`
  16:3x, memory note "Per-Shell Watch EventSource Connection Budget". Load 40, 111 GiB free. 7800 = W3's (B2), nothing on 8080–8089 /
  6580–6589.
- 19:1x serve **6580** (`S_OS_PORT=6580 S_HUB_URL=http://127.0.0.1:7800 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev` in
  `🧑‍💻dev/📦️packages/🟦️typescript`, pid 20133), then restarted with `S_DATA_DIR=.🧬semio/🌐hub/s13-f2-space` (the launch row's local data
  root, pid 30492; log `.🧬semio/🌐hub/s13-f2-logs/serve-6580-b.txt`). Probe `wp-f2/f2-conn-probe.mjs` (new: Chromium with a NetLog; after
  every step the requests still open per origin, the client-side ESTABLISHED sockets to the serve port, a same-origin probe fetch) +
  `f2-netlog-inventory.py` / `f2-netlog-timeline.py` (new: per-origin queue time, peak on the wire, long-lived streams, socket-pool stalls).
- 19:2x–20:05 **F2-1 before (measured; load 25–40, so counts, not timings, are the result):**

| run | long-lived HTTP/1.1 streams held on the serve origin | serve-origin requests | peak on wire | socket-pool stalls (`SOCKET_POOL_STALLED_MAX_SOCKETS_PER_GROUP`) |
|---|---|---|---|---|
| `base1` local-only, no data root, 8 programs | **2** (`/🔌️plugin-modules/watch`, `/🧩️extension-modules/watch`, whole session) | 1171 | 6 | 927 |
| `signin1` + hub sign-in (7800) | 2 + a `/_semio/hub/directory/event-page/v1` held 10 s each poll | 1274 | 8 | 1027 |
| `hub1` + `S_DATA_DIR` (launch-row default) + one space mounted | **4–5 of 6, permanently** (2 module watches + `/semio-backbone/watch` for the identity folder + one per open hub document (space index) + the 10 s event-page polls) | 2489 | 9 | 2116 |
| `hub2` same, space with a document + 3 programs | 4–5 of 6 permanently (timeline: "long-lived holding=5" from t=140 s to the end) | 2533 | 9 | 2135 |

  Source of each stream: kernel `subscribeSharedWatchStream` (1 per watch url per page, S15 17-09 fix), the store worker's `connectSseOnce`
  (ONE `EventSource` per open folder-bound document: identity facet `${dataDir}/os` and every hub document in a space `${dataDir}/spaces/<id>`,
  `resolveDocumentOpeningBindings`), the dev activation middleware (a module GET of an unmaterialized plugin BLOCKS until `nx materialize`
  finishes — minutes — and its progress goes out as SSE comments on the watch stream). Hub document sockets and the directory stream are
  WebSockets (Chromium pools them apart from the 6-per-origin HTTP/1.1 group: not part of the budget). So every additional open hub
  document removes one of the remaining 1–2 connections; the 3rd one makes every later fetch (module, command, install POST) queue
  until a stream closes. Boot itself is a separate cost: 960–1150 unbundled dev-module requests through 6 connections (queue p50 0.8–1.4 s
  under load) → item F2-2.
- 20:07 rule 22 (memory budget): serve 6580 stopped (30492 → vite 30591) while I implement F2-1; restart from the recipe above.
- 20:1x–20:55 **F2-1 implementation (TS only — no Rust peer consumes these streams: the native wgpu shell reads plugin modules
  from disk and the hub/MCP servers never serve them; checked by grep over every tracked `.rs`).**
  - Contract `🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🧬️schema/🔣️.json` (`semio.io.stream-mux/v1`, subprotocol `semio.stream-mux.v1`):
    reader frames `open{stream,route,key,resume,credit}` / `grant` / `cancel`; server frames `hello{epoch,beatMs}` /
    `opened{mode: fresh|resumed, epoch, seq}` / `data{seq}` / `progress{done,total,note}` / `end{done|cancelled|refused|failed}` / `beat`;
    bounds as schema consts (frame ≤ 256 KiB, ≤ 1024 streams, credit ≤ 256 (default 32), ≤ 64 queued frames per stream, 128 retained
    events per route instance, resume grace 60 s, beat 10 s, dead after 25 s, reconnect 250 ms…10 s jittered, socket high water 1 MiB,
    idle linger 5 s).
  - `🔀️stream-mux/🟦️.ts`: canonical codec (exact fields, ranges, byte bound, refusal codes); `StreamMuxServerV1` (routes with
    `snapshot` / `coalesce` / live `source` / job `run`; one epoch-wide sequence so a resume across an unobserved event starts fresh;
    per-stream credit, coalescing queue, overflow → fresh snapshot, congested socket holds queues; jobs shared by key, progress
    latest-wins, cancelled by their last reader's cancel or after the grace of a lost link); `StreamMuxChannelV1` (ONE reconnecting link
    per page and path, resume from `{epoch, seq}` per stream, watchdog, linger, `attachPort` bridge); `StreamMuxPortEndpointV1`
    (worker side); `pageStreamMuxChannelV1`, `streamMuxWatchV1`. Erasable-only TS (strip-only law).
  - Law `🔀️stream-mux/🧪️tests/🧫️contract/🟦️.ts` over the language-agnostic fixture `🔀️stream-mux/🧫️fixtures/🔣️.json` (written by
    `wp-f2/f2-stream-mux-fixture.py`; canonical texts from Python's `json`): 21 vectors (decode + byte-exact re-encode), 35 hostiles
    (refusal + field), 18 server scenarios (snapshot/live order, credit, coalescing, overflow resync, resume, epoch change, grace expiry,
    ring eviction, refusals, malformed frame, job done/failed/cancel/lost-link, live source start/stop, folder notice collapse, beats,
    congestion), 5 channel scenarios (one link for all streams + resume after loss, dead-link watchdog, credit held by an unsettled
    handler, server end + refused frame, linger) + a MessageChannel port-bridge case. **Third-party oracle: AJV (draft 2020-12)** judges
    every vector, hostile and every emitted frame against the schema. Run in `@semio-tech/framework` test: **81/81**; red run with 3
    corrupted fixture rows → exactly 3 FAIL.
  - Dev serve: `devStreamMuxServer(httpServer)` (vite-plugins) accepts same-origin upgrades at `STREAM_MUX_PATH` (`/semio-stream-mux`) via
    the runtime's `ws` behind `RuntimeWebSocket*` types — measured: Bun 1.3.14's `node:http` upgrade socket transmits nothing written to it
    (curl/`ws` client: no 101), while Bun's built-in `ws` `handleUpgrade` works. Routes (`DEV_STREAM_ROUTES`, os entry): `plugin-modules.watch`
    (activation receipts), `extension-modules.watch` (extension store), `backbone.folder` (one `fs.watch` per watched uri while its
    instance lives, notices coalesced), `plugin-modules.activation` (the restage job with line progress + cancellation; the blocking
    lazy-activate GET is gone — a missing module now answers 404 at once, like its HEAD). Removed: every SSE endpoint + keepalive, the
    unregistered `semioPluginHotSwapVitePlugin` / `scanBuiltPluginModules` (dead) and their tests.
  - Kernel: `createDevPluginSource` / `createExtensionSource` take the owner's `PluginSourceWatch` (no URL, no `EventSource`, no
    page-shared cache — each subscription is one mux stream, the server sends each its own snapshot); fixture `📡️source-watch.json` v2.
  - ShellHost: both sources on `streamMuxWatchV1(pageStreamMuxChannelV1(STREAM_MUX_PATH), …)`; the backbone worker gets a
    `MessagePort` onto the page channel (`semio-stream-mux-port`, detached on unmount). Store worker: `watchFolder` opens
    `backbone.folder` on it (fresh open or notice → revalidate, notice credit returns when the read settles), `sseHealthy` →
    `watchHealthy`, SSE reconnect constants gone (the channel reconnects).
  - Tests updated: kernel watch cases (4 new, EventSource fakes gone), docklayoutstore PluginSource cases, worker offline-resilience
    (FakeStreamEndpoint; 2 new cases, SSE reconnect/backoff cases removed with the code), extension-store precedence law (install
    endpoint) + new stream law, dev adapter handler test + `🧪️cases.json` (plugin rows removed; the stale `GET install → next` row
    corrected to `listed`), backbone-parity state literal. Taxonomy: `framework-io-stream-mux` member registered.
  - Results: framework **265/265**; plugin store **9/9**; os worker/installation/parity run: my 4 folder-watch cases pass, 12 other
    failures are a peer's in-flight `WireMutationEnvelope.target` change (`envelope.target.length`, `wire str: truncated`), not F2;
    dev adapter test **1/1** (`SEMIO_TEST_LEVEL=long`; its neighbour "finalizes genuine descriptor bytes…" fails independently);
    renderer quick "stamps every boot install" 1/1. `tsc`: framework package → only 7 pre-existing errors (bun:sqlite `transaction`,
    accessibility); os-config scoped check over all 16 touched files → 0 errors in any touched region (remaining ones are the peer's
    envelope type, `whatwg-url` types, docklayoutstore's injected-any pattern).
- 20:56 serve 6580 restarted for the live proof → hub **8021** (C11's current-tree hub, client use only; 7800's old binary needs 84 s
  for user1's directory, rule 20), pid 99133, log `s13-f2-logs/serve-6580-c.txt`. Swap 16.1/17.4 GB, load 80–90 → counts, not timings.
- 21:0x **F2-1 live proof (serve 6580 → hub 8021, `S_DATA_DIR` set, load 80–90):**

| run | long-lived HTTP streams on 6580 | stream-mux sockets | streams on the channel | notes |
|---|---|---|---|---|
| before `hub2` (SSE) | 4–5 of 6 held for the whole session (2 module watches + identity folder + 1 per open hub doc + event-page retries) | — | — | 2135 socket-pool stalls |
| after `after1` local + space route | **0** | 1 per page load | — | `/semio-stream-mux` answered `101`, subprotocol `semio.stream-mux.v1` |
| after `after2` signed in, space index + 1 hub doc open | **0** (the only ≥ 10 s request is a 16 MB `.core.wasm` transfer) | 1 per page load | plugin + extension watch, `backbone.folder` × 3 (identity, space index, the opened document) | folder watches of the worker ride the page's socket through the port |
| stress `stress1` (`f2-stress.mjs`) | **0** | 1 | 66 open at once (64 `backbone.folder` + 2 shell watches) + 8 `plugin-modules.activation` jobs (each `opened fresh` → `end done`, modules already staged) | 300 concurrent fetches during it: **300/300 ok** (p50 3.0 s, max 6.7 s: 300 requests through 6 connections at load 80 — pool throughput, nothing blocked); touching every watched folder → **64/64** streams got a notice, touching half again → **32** more (96 total, exact); after closing: 2 streams left (the shell's own), 1 link |
| contrast `f2-sse-contrast.mjs` (plain Bun page, per-stream `EventSource`s, idle timeout off) | 64 EventSources → 6 open | — | — | **0/300** fetches finished within 20 s; 6 EventSources → **0/300**; 5 → 300/300 |

  Server endpoint checked with a `ws` client: `hello` → `opened fresh` + snapshot per route, unknown route → `end refused unknown-route`;
  a foreign `Origin` is refused (socket closed before `101`). Directory event pages (coordinator ask): they are the space index's
  paged catch-up through `/_semio/hub` (then the directory WebSocket pushes), not a poll; on 7800 every attempt hit the client's 10 s
  timeout (no response headers in the NetLog, retried with backoff) because 7800's old binary is slow (rule 20) — on 8021 all 5 pages
  answered `200` in < 1 s. Hub-origin HTTP (sign-in, `/directory/spaces`): ≤ 2 on the wire, nothing long-lived; the directory
  stream is a WebSocket. **No hub-side mux needed** for the budget; slow catch-up is hub latency (H9/H11 Qd, landed in the tree).
- 21:30 cut by the account usage limit (while writing the permanent harness); every process of ours died overnight (rule 28).
- 04:58 resumed. Reconciled: every F2 edit was already in auto-commit 653 (stream-mux module + fixture + law, kernel, ShellHost,
  worker, vite-plugins, harness); the only uncommitted diff in a file I touched (worker: 8 `[DEBUG] c11 …` lines removed) is C11's.
  framework suite re-run **265/265**. Serve 6580 restarted detached (`w2-detach.py`, pid **95457**, `S_LOCAL_ONLY=1`, data root
  `s13-f2-space`, log `s13-f2-logs/serve-6580-d.txt`).
- 05:0x **permanent gate** `🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts` (promoted from `f2-conn-probe.mjs` + `f2-stress.mjs` +
  `f2-netlog-inventory.py`): verb `verify connections <serveUrl> [--tag] [--streams] [--fetches] [--out] [--headed]`, nx target
  `@semio-tech/framework-os-dev:connection-budget`, launch rows `⚖️gate🔀️connection-budget⚛️react` (launch.json + seed, gate group
  order 411.275), acceptance check `connection-budget` (en + de summary and violations), SIGINT/SIGTERM cancel, output under the
  gitignored `🧑‍💻dev/🤖️generated/🔀️connection-budget/<tag>/` (report + NetLog). Law: no request idles ≥ 10 s on a serve-origin
  connection (< 64 KiB read), exactly one stream-mux WebSocket, every fetch of the burst within 20 s, every folder stream opens,
  every touched folder gets exactly its notice, channel census back to baseline. First run FAILED on my own beacon read (the
  attribute's value is `s`, not `ready:s`) → fixed; **run 2 PASS** (`connection-budget-2.txt`: 0 idle holds, 1 channel, 64
  streams, 300/300 fetches p50 569 ms, notices 64 + 32 exact, census 2 → 66 → 2). Oracle red check (`wp-f2/f2-inventory-red.ts`):
  the same `connectionInventory` over the pre-mux NetLog `hub2` names **18 idle holds** (module watches 83–306 s, folder watches
  284–304 s, event-page 10 s timeouts), over `after2` **0**. Scoped `tsc` of harness + verification router: 0 errors in them.
  Main asked to relay the plan row to V1 (agent ids resolve per coordinator session).
- 05:1x found on the way (not F2's, not fixed): `os:check-wgpu` reports the frame-worker bundle stale right after
  `os:generate-wgpu` wrote "0 changed" — check and generate disagree on the gitignored output (`registryCatalogInputView.kind`
  → null in check mode); the dev `space` component core wasm is **87 MB** (13 s transfer on every cold boot) → F2-2.
- 05:1x–05:40 **F2-2 boot anatomy** (`wp-f2/f2-boot.mjs`, new: fresh Chromium profile, cold load then reloads in the same profile;
  navigation/paint/LCP/beacon/all-plugins-loaded marks, long tasks, a 1 ms sampled CPU profile of the whole boot, resource timing by
  kind). Before (`f2-boot-b2.json`, load 57): 1051 requests (909 dev modules 62 MB decoded, 120 plugin descriptors **15.7 MB**, the guest
  font pack **8.4 MB**); a warm reload re-transferred **25.4 MB** (every descriptor + the font pack: the dev static mount sends no
  `ETag`/`Last-Modified`, so nothing is ever revalidated); main thread busy **3.8 s** of the boot, 12 long tasks = 1.2 s; top JS
  cost `adaptPluginHandle` **925–951 ms**: `loadPluginModule` hands its handle the manifest as pack BYTES (`encodePackValue(manifest)`)
  and `adaptPluginHandle` immediately decodes them again (`decodePackValue`) — 60 manifests encoded + decoded per boot, 790 ms of it
  in `encodePackValue`, **506 ms in `packByteCompare`** (a new `TextEncoder` + two encodes per key comparison).
  Root fixes (TS, compile-atomic, scoped `tsc` clean in the touched regions):
  1. `serveFileWithValidatorsV1` (`🖱️ui/🎨️styling/🏗️builder/🌐️vite`): every dev-served file (static-dir mounts incl. plugin modules,
     descriptors, vendor, UI assets, meshes, favicon ICO, map tiles) answers a weak `ETag` (size + ns mtime), `Last-Modified`,
     `Content-Length`, `Cache-Control: no-cache`, `304` for a current `If-None-Match` (weak comparison) / `If-Modified-Since`, and
     headers only for `HEAD`. Law "conditional delivery of dev-served files" (fixture `🎨️styling/🧫️fixtures/🏷️conditional-delivery`,
     13 cases + schema; **oracle: the `fresh` package**, express/koa's freshness judge, must agree on every GET) → **pass (36
     expects)**, red with 304 disabled → FAIL. Styling suite 67/68 (the 1 is a peer's panel-tab CSS assertion). Staging test updated
     (extracts the helper too; request double gains `method`/`headers`).
  2. `packByteCompare` (`💻️os/🟦️.ts`, exported): UTF-16 units compared directly; only a surrogate at the first difference takes the
     exact encode path. Law `💻️os/🧪️tests/🔤️pack-key-order` (fixture: 30 edge pairs incl. astral vs U+E000–U+FFFF and lone
     surrogates + a seeded 20 000-pair fuzz; **oracle: Node's `Buffer.compare` over UTF-8**) → **3/3**, and a bound: sorting
     surrogate-free keys constructs **0** `TextEncoder`s; red: a plain UTF-16 comparator disagrees on 8 of 30 pairs.
  3. Kernel `PluginWasmHandle.manifest` is the admitted manifest VALUE (was `() => Promise<Uint8Array>`); `adaptPluginHandle` reads it
     directly; ShellHost's slot adapter passes the real manifest; 15 test doubles updated. Renderer engine-contract + PluginRuntime
     **820/837** — the 17 failures are peers' in-flight TextEditor host tests (7500+ region) and one CPU-bound 100 000-expect ladder test
     timing out at load 119; every handle-manifest double passes.
  **After (`f2-boot-a1.json`, load 106 — wall times not comparable, counts are):** main-thread busy **3.8 → 1.9 s**; long tasks **12–13
  → 1–3** (1.2 s → 0.08–0.35 s); `encodePackValue`/`adaptPluginHandle`/`packByteCompare` no longer in the top 30; warm reload transfer
  **25.4 → 0.1–0.7 MB** (descriptors 15.77 → 0.03 MB, font pack 8.35 → 0 — all `304`). Serve 6580 restarted for the middleware change
  (pid **22452**, log `serve-6580-e.txt`).
- 05:4x **F2-3 permanent gates** `🧑‍💻dev/🧪️tests/💤️idle-budget/🟦️.ts` (promoted from `wp-f1/f1-idle-census.mjs` + `f1-lib.mjs`; rule 21):
  `verify idle <serveUrl> [--roles] [--only] [--seconds]` — per program (catalog probe, palette chord, bodies rendered, main thread
  settled) a Chromium trace of 10 s without input (oracle: Chromium tracing): main frames/s, rAF/s, compositor + viz draws/s, busy %
  (reported, not judged — load-sensitive), then closed and 4 s re-traced; law < 1/s for all three while open and after closing; a
  240 s per-row watchdog + browser restart on failure (the F1 census died on hung browsers). `--soak-minutes 30 --windows 10`: first 10
  editors open, every minute a forced GC and a sample of JS heap, worker heaps (CDP `Runtime.getHeapUsage` per worker), DOM nodes,
  listeners, 5 s idle trace; law after a fifth of warm-up: heap and worker slope ≤ 0.5 MB/min (least squares), nodes/listeners growth
  ≤ 1 %, frames < 1/s. nx targets `idle-budget`, `memory-soak`; launch rows `⚖️gate💤️idle-budget⚛️react`, `⚖️gate🫧️memory-soak⚛️react`;
  acceptance checks `idle-budget`, `memory-soak` (en + de). Reducer law `💤️idle-budget/🧮️reducers` over fixture
  `🧑‍💻dev/🧫️fixtures/💤️idle-budget.json` (synthetic trace with a decoy renderer, a tile worker whose draws must not count, overlapping
  tasks; two soaks; expected values from `wp-f2/f2-idle-fixture.py`, an independent Python least squares) → **3/3**, red 2 FAIL.
  Shared helpers exported from V1's program-matrix (`awaitBeacon`, `dismissIntroduction`, `windowIds`, `kindOf`, `roleOf`, `keyOf`,
  `MatrixProgram`) instead of copying them. Scoped `tsc`: 0 errors in the dev harness files. The ticket-local census copy
  (`f2-idle-census.mjs`) failed on `--resume` without a prior file → superseded by the gate, launched detached 05:45
  (`w2-detach.py`, pid **33875**, log `s13-f2-logs/idle-budget-s13.txt`); first rows space/home editor + viewer, space/space: 0/0/0.
- 06:0x **more boot anatomy (NetLog of one local-only cold boot, `f2-netlog-stress1.json`, 1391 requests / 171 MB):** the space
  component `.core.wasm` **83 MB** (43.7 MB of it the `name` custom section, 35 MB code — dev component, all 60 dev cores = 4.07 GB) is
  fetched by the shard worker on every boot — invisible to the page's resource timing; before the validator fix every reload
  re-transferred it and V8 had no cached response to reuse; the 3D stack is loaded eagerly even with no 3D window: three-stdlib 282
  modules 15.7 MB, drei 157 modules 4 MB, hls.js 5 MB, troika 1 MB, camera-controls, maath (~460 requests, ~27 MB) because the
  ui-react target barrel imports drei at top level and drei/fiber are deliberately excluded from Vite's optimizer (one Canvas store,
  memory note "React Serve Black Boot"). Not changed (a lazy 3D stack is a ui-react restructure; the dev component size is W3's
  build profile) — recorded as the next boot offenders. Descriptors: each fetched once (GET) + one `HEAD` availability probe
  (`requireServedPluginModule`, `no-store`) — no duplicate GETs.
- 06:1x **permanent latency gate** `🧑‍💻dev/🧪️tests/⏱️interaction-latency/🟦️.ts` (promoted from `wp-f1/f1-latency.mjs`):
  `verify latency <serveUrl> [--only]`, nx `interaction-latency`, launch row `⚖️gate⏱️interaction-latency⚛️react`, acceptance
  `interaction-latency`; scenarios in the fixture `🧑‍💻dev/🧫️fixtures/⏱️interaction-latency.json` (writer typing, trinity jack
  typing, draw drag, dag drag; bounds p95 input → frame and paints per input). Paints per input are always judged; the p95 only
  while the 1-min load ≤ 1 × cores, else the check is `blocked` with the measured numbers (timings on a saturated machine are not
  verdicts). Oracle: Chromium Event Timing entries recorded beside. Written + `tsc` clean; runs after the census frees the browser.
- 06:1x **hub document open latency in V1's two-human gate:** each kind row now carries `timings.createToMountedMs` (A: create →
  document mounted) and `timings.openRowToMountedMs` (B: the Open press → mounted, measured after the row is visible, so directory
  propagation is not mixed in). Needs a hub → first run on the post-rebuild 7800.
- 06:2x **HMR-on check of the stream channel** (temporary serve 6581 with the launch-row default HMR, stopped after): the mux
  answers `101` + hello + both snapshots + `refused unknown-route` exactly as with HMR off. Found on the way (pre-existing, not
  F2's, not fixed): **Vite's HMR WebSocket never connects under the bun-run dev serve** — Vite bundles the npm `ws` server, and under
  Bun 1.3.14's `node:http` an npm-`ws` `handleUpgrade` "connects" server-side while the client never receives the `101`
  (reproduced standalone: `wsprobe/s4.mjs`, npm ws server under bun → client silent; Bun's built-in `ws` → works). The stream
  channel is unaffected because a bare `import("ws")` resolves to Bun's built-in implementation under Bun and to npm `ws` under
  Node (both measured to work). Routed to main.
- 06:35 cut (usage limit). 09:55 resumed (REBUILD START 09:5x, rule 30). My serve 6580 (22452) and the census (33875) survived the
  cut; the census had hung since 06:54 on a dead Chromium (no child process, bun awaiting a protocol promise that never settles).
  Result so far (`🤖️generated/💤️idle-budget/s13-all/idle.json`): **119/121 rows within the idle budget — max 0.1 main frames/s,
  0 rAF/s, 0 compositor draws/s, max 1.88 % main busy (demonstrator/gismap editor), no frame after any close**; the 2 failures are
  OPEN failures, not idle offenders: `wfc/bitmap` editor did not open, `wfc/grid2d` editor gave no verdict in 240 s. Stopped the
  hung tree (my pids), hardened the gate (`withDeadline` for every shell boot, 3 bounded boot attempts, bounded health probe and
  leftover close; `--resume` keeps measured rows of the same tag) and resumed detached (pid **72102**, log
  `s13-f2-logs/idle-budget-s13-resume.txt`).
- 10:0x **correction of the 06:2x HMR finding:** Vite 7.3.6 already special-cases Bun (`process.versions.bun ?
  import.meta.require("ws")`, Bun's built-in ws), and the Chromium NetLog of the 6580 runs shows every Vite HMR socket
  (`ws://127.0.0.1:6580/?token=…`) answered `101` with frames both ways. The hang I saw came from the npm-`ws` CLIENT probe against
  Bun's server, and the standalone repro used the npm-`ws` SERVER, which Vite does not use under Bun. Told main. Still to prove
  (F2-3): a served-module edit reaches the page as an HMR update (dev config `watch: null`).
- 10:1x **idle census finished: 148/148 PASS** (resume run, `🤖️generated/💤️idle-budget/s13-all/idle.json`): 77 editor + 71 viewer rows,
  max 0.1 main frames/s, 0 rAF/s, 0 compositor draws/s, max main busy 1.88 %, 0 frames after every close. The two 06:5x open
  failures (`wfc/bitmap`, `wfc/grid2d` editors) opened and passed on the resume — see **For S16** below.
- 10:1x **R9's report (jsdom engine-contract: unhandled WS errors from the stream channel):** the page channel is now opened only when
  the serve announces it — `semioBackboneVitePlugin` defines `import.meta.env.VITE_SEMIO_STREAM_MUX_PATH` for `serve`
  (`STREAM_MUX_ANNOUNCEMENT` in framework-os); `ShellHost` reads it and falls back to the silent endpoint. engine-contract 704/704;
  replied to R9.
- 10:2x **extension store no longer imports dev vite-plugins:** `semioExtensionStoreVitePlugin` takes the stream-channel factory as
  `streams` (generic over the serve's HTTP server, type-only mux import); dev, play and demonstrator configs pass `devStreamMuxServer`.
  Store package 9/9 (`test-installation.txt`), `tsc` clean for the store, its law and the three configs (the play/demonstrator
  `OwnedBuildPlugin`↔Vite `config` hook mismatch is older and not F2's).
- 10:2x–10:4x **F2-3 HMR, root cause of the law's red (4 updates for 2 writes, then 0 with a symlinked root):** (1) Vite keys its
  module graph by real path, the replayed events named the symlinked path → `semioSourceWatchVitePlugin` watches
  `realpathSync(repoRoot)` and the transform guard resolves `server.config.root` the same way; (2) a rename event for an
  already-tracked module was replayed as `add` + `change` → only `change` now (`tracks()` on the freshness registry); (3) the
  remaining extra updates were the filesystem's late report of the sandbox's own creation (traced: `f2-hot-debug2.ts`,
  `hot-debug{2,3,4}.txt` — 1 `change` → 1 `hmr send` per write in every ordering). The law now counts per write over a quiet window.
  Watch-policy tests updated to the real-path contract (+ "a tracked module is never answered with `add`").
- 10:3x **new finding — a Vite dev serve under Bun never finishes `close`/`restart` while a browser is connected** (`f2-vite-close.ts`):
  bun 1.3.14 `server.close` still open after 15 s, `server.restart` → serve unreachable, 0 `close` events; node 24: 2 ms / restart 4 ms,
  HTTP 200. Split: the hot channel closes in 0–1 ms; `httpServer.close` hangs even after destroying every tracked socket or
  `closeIdleConnections` (Bun ignores `socket.destroy()` on served connections; its graceful `close` waits for the browser's
  keep-alive connections), and HMR off hangs too (it is the HTTP server, not the WebSocket). Impact: every edit of a serve's config
  entry restarts HMR-on serves (Vite skips config restarts only with `hmr: false`) → the serve stops listening forever; SIGTERM
  shutdown hangs; every resource released on the server's `close` (source watchers, stream channel, rendezvous records) stays live.
  Field sign: Vite pid 40403 (`:6060`, started 09-26 21:40, orphaned) is alive and listening on nothing (not mine, left alone).
  **Fix:** `semioServeCloseVitePlugin` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`): `close` first runs the
  runtime's own `closeAllConnections` — Node destroys the connections and `close` proceeds as usual; Bun stops the server outright,
  so the plugin completes the close (emits `close` once, calls back). Wired first in all five serve configs (dev, play, demonstrator,
  präsentation, wgpu). Measured with it: bun close 2 ms + 1 close event, restart 9–25 ms + HTTP 200; node (same logic) close 2 ms,
  restart 8 ms.
- 10:4x **law `🧑‍💻dev/🧪️tests/♨️hot-update` 2/2 PASS under bun 1.3.14** (`test-hot-update-5.txt`, expectations in the new fixture
  `🧑‍💻dev/🧫️fixtures/♨️hot-update.json`): per write style exactly 1 update (in-place 45 ms, atomic rename 24 ms), unwatched control 0;
  restart under a connected page 9 ms, 1 `close` event, HTTP 200, then 1 update per write (22 / 25 ms). **Red** without the close
  plugin: `{ restarted: "timeout", status: 0, closeEvents: 0 }` (`test-hot-update-red.txt`). dev `hot-update` + `🧹️config`: 48/52 —
  the 4 failures are older than this work: 3 config-graph checks (`🚀️local-hub/🏃️execution` imports `📚️library/📦️packages/🟦️typescript`
  since 82c0bdf59a9, 09-24 → 66 modules > 40, library denied) and the browser-graph check (ShellHost's
  `service-worker/🟦️.ts?worker&url` default import, since 56b837a6770, 09-25). Styling suite 67/68 (the failing CSS panel-tab check is
  not F2's).
- 10:48 **F2-3 live proof** (`f2-hmr-live.mjs` → `f2-hmr-live.json`) on an HMR-on `serve s react dev` at **6581** (launch-row default
  HMR, own data root `s13-f2-space-hmr`, stopped after): rewrote `🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx` with its own bytes →
  `[vite] hot updated` for the module (+ `/🎨️.css`) **475 ms** after the write, **no navigation**; rewrote the config entry with its
  own bytes → serve log "changed, restarting server..." + "server restarted." in the same second, page "connection lost" at 30 ms,
  reconnected + reloaded at **3.0 s** (Vite client polls every 1 s), HTTP **200**. The only other HMR-on serve (6064) had no
  connections; all peer serves (6064/6060/6013) answer 200 after. Told main.
- 10:54–11:26 **memory soak PASS** (`bun nx run @semio-tech/framework-os-dev:memory-soak -- http://127.0.0.1:6580/ --tag s13-soak`,
  detached, log `s13-f2-logs/memory-soak-s13.txt`, report `🤖️generated/💤️idle-budget/s13-soak/soak.json`; 2-min smoke first,
  `memory-soak-smoke.txt`, PASS): 10/10 editors (space/home, architect/program, animate/presentation, block/block2d, cad/cad,
  dag/dag, demonstrator/playground, draw/drawing, energy/model, fem/fem2d) for 30 min, 31 post-GC samples: JS heap 101.28 →
  101.31 MB (slope 0.001 MB/min, bound 0.5), workers 40.19 → 38.97 MB (0 MB/min), DOM nodes 1632 → 1632, listeners 1263 → 1263,
  0 frames/s / 0 rAF/s / 0 draws/s in every sample. Load 7–42 during the run (counts and heap, not timings, are the verdict).
- 11:26–11:52 **F2-2 typing paints (latency gate):** the first gate run (`interaction-latency-s13.txt`) reported `0 paints/input` for
  every scenario with `classes: []` — the paint half of the gate was vacuous. Cause: the hook finds the canvas session modules
  through Resource Timing entries, and the page's default 250-entry buffer is full long before a program opens (the `s` boot loads
  ~515 scripts; probe `f2-latency-modules.mjs`: default buffer → 0 session modules, 100 000 → `framework_editor.js` with
  `EditorSession`/`DagSession`). Fixed in `🧑‍💻dev/🧪️tests/⏱️interaction-latency/🟦️.ts`: the init script sizes the buffer, host-element
  session classes count too (the draw canvas paints in JS: `JsonLayersCanvasSession`), and a scenario that hooks no class FAILS.
  **Red with the real count** (`-b`): writer **4.13**, jack **3.67** paints per key (bound 2), draw unmeasured, dag 2.06.
  Stacks (`f2-paint-stacks.mjs` → `f2-paint-stacks-{writer,trinity}.json`): every paint goes through `GraphWasmCanvas`'s
  demand scheduler; per key one paint right after the key, then one or two more 50–190 ms later. Root cause in
  `✏️TextEditor/🟦️.tsx`: (1) 33 handler call sites called `renderFrame` on the frame-demanding handle, a SYNCHRONOUS paint on top of
  the invalidation every session call already makes; (2) every echo of the editor's own edit was synced again, even when the
  reconciled pack was byte-identical (the selection round trip). A first attempt that coalesced the calls into an own rAF made it
  worse (5.75 / 4.83: two schedulers painting one frame) and was removed. Fix: the editor never paints on its own (the handle
  invalidates, the canvas paints once per frame), and a pack is synced once per session (`sameScenePackV1`). **After:** writer
  **1.13**, jack **1.79–2.13** (the key + the guest's re-highlighted echo + the caret cadence inside the window; bound set to 2.5
  from this measurement), draw **1.06**, dag 2.06. Full gate at load 7.8–9.7 (`-g`): writer PASS p95 48.7 ms, jack PASS 36.9 ms,
  draw PASS 28.8 ms, dag BLOCKED (load 10.7 > 10 cores; 2.06 paints, p95 18.4 ms). Engine suites (engine-contract,
  surface-idle-frames, caret-cadence) **708/708**, engine quick 17/17, `tsc` clean.
- 11:55–12:08 **F2-2 cold boot: the drei barrel.** `f2-boot.mjs` now also sums the boot's bytes per node_modules package. Cold boot
  of `s` on :6580 at load 8 (`c1`): FCP 2.4 s, ready 3.5 s, all 60 plugins 3.5 s, **909 modules / 62.4 MB of JS**, of which
  three-stdlib 282 files 15.7 MB, hls.js 5.05 MB, @react-three/drei 157 files 4.05 MB, camera-controls, maath, gainmap, detect-gpu,
  meshline, @use-gesture, stats-gl… Cause: drei and fiber are excluded from prebundling (one Canvas store), and
  `@react-three/drei` is aliased to drei's `index.js`, so the three ui files importing the barrel (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`,
  `🧱️elements/🎬️Scene`, `🧱️elements/🔌️Ports`) made the browser fetch all of drei core + web raw, and every drei control imports
  `three-stdlib`'s index, which nothing ever scanned (an excluded package's imports are not discovered). Fixes: (1) every drei
  member is imported from its own module (`@react-three/drei/core/<Member>.js`, `web/Select.js`; exports checked per file) →
  `c2`: 735 modules / 51.6 MB, drei 20 files 0.32 MB, no hls/mediapipe/camera-controls; (2) `three-stdlib` joins the scene-host
  optimizer includes (`PLAYGROUND_SCENE_HOST_ESM_INCLUDE` in the styling builder, law case in `playgroundSceneHostOptimizeDeps`,
  3/3) → restarted 6580 (`serve-6580-g.txt`, re-optimized in < 1 s) → `c3`/`c4` at load 7: **453 modules / 40.4 MB, DCL 0.54–0.65 s,
  FCP 0.73–0.90 s, ready 1.66–1.81 s, all 60 plugins 1.98–2.24 s**. Runtime check (`f2-open-drei3d.json`): cad, puzzle3d,
  block3d and architect editors open with 0 errors and no "multiple instances of three" warning; the puzzle3d perspective view
  renders with the drei gizmo (screenshot). **Law:** browser-graph contract `denySpecifiers` (`🧑‍💻dev/🧫️fixtures/🌐️browser-graph.json`)
  + test "imports no denied barrel specifier from any browser-served module" — PASS, **red** with one barrel import put back into
  Scene (`test-browser-graph-red.txt`); the esbuild oracle test also asserts it (that test is red for the older ShellHost
  `?worker&url` reason). `tsc` clean for the four files.
- 12:0x **field sign of the close hang:** Vite pid 50336 (raster serve, `:6060`, started 09-26 21:5x, orphaned) is alive at 0 % CPU
  with no socket at all and `:6060` refuses — a Vite that stopped listening and never finished closing (pre-plugin code). Not mine,
  left alone; told main.
- 12:1x **paused by the coordinator** (memory cleanup): main stopped my 6580 serve and the orphaned 50336; no servers, browsers or
  builds of mine are running. Open for the resume once 7800 is on B3: (1) hub document open (click → editable) and hub catalog
  plugin install timings through V1's two-human gate (`timings.createToMountedMs` / `openRowToMountedMs`), then set and send V1
  (`a919ab599c212eb8b`) the final `--max-create-to-mounted-ms` / `--max-open-to-mounted-ms` from those measurements;
  (2) the remaining cold-boot bytes, for the record: the 8.4 MB guest font pack (`🪞️vendor/🔤️guestslim-typst-fonts.bin`,
  `🎠️kernel/🟦️.ts` `defaultGuestSlimAssetFetcher`) is required before the first guest surface, so it stays; it goes over the wire
  uncompressed in dev. Next biggest: prebundled three-stdlib 4.5 MB, fiber 2.1 MB, ShellHost 2.1 MB, icons 1.8 MB, and 17.3 MB of
  plugin descriptor JSON (all revalidated by 304 on warm reloads since the validators fix).
- 14:0x **resume: hub timings on 7800 (catalog B3), then HELD by the coordinator (memory warning, 14:0x).** Serve recipe
  (C10's `serve.sh`: `S_HUB_URL=http://127.0.0.1:7800 S_LOCAL_ONLY=1 SEMIO_VITE_HMR=0 serve s react dev` on 6580) booted in
  < 1 min; the gate (`verify two-human`, default development users) created space `01a0e2bd-fba0-7ed9-9ef1-413c8b5bc50f` and
  discovered the six requested kinds (2d.block, 2d.drawing, 2d.puzzle, 3d.puzzle, animate.presentation, text.document), then I
  stopped it on the hold before any kind ran (verify pid 6799 + its Chromium 6823: the nx wrapper's group kill does NOT reach
  them — nx starts the verify in its own process group; the launcher below starts the verify directly) and stopped the serve.
  Ready to go: `wp-f2/f2-hub-timings.sh <tag>` (serve + gate detached, one browser; puzzle 2d then 3d separates the catalog
  install from the open itself). No timings measured yet.

## For S16

wfc `bitmap` + `grid2d` editor "failed to open" in the s13 idle census:
- **Where:** serve **6580** (`S_LOCAL_ONLY=1`, `S_DATA_DIR=.🧬semio/🌐hub/s13-f2-space`, `SEMIO_VITE_HMR=0`, pid 22452, log
  `s13-f2-logs/serve-6580-e.txt`), dev lane staged by restage4 before the rebuild: the serve reported `[stale] wfc: source-changed —
  source 06c7a861db9c… ≠ staged 85c59065f48a…` (the page ran the staged wasm, not the current source). Time ~06:5x, 1-min load
  ~60–100 (fleet builds).
- **What:** census log (`s13-f2-logs/idle-budget-s13.txt`): `FAIL wfc/bitmap#editor — did not open` (the new window never appeared
  within the open bound), `PASS wfc/bitmap#viewer`, then `FAIL wfc/grid2d#editor — no verdict within 240 s` — after which the census
  hung: its Chromium had died (no child process left, bun awaiting a CDP promise). So the grid2d row is a dead-browser row, not a
  plugin verdict; the bitmap row is an open timeout under load. No page errors were captured by the census for these rows (it records
  verdicts, not console).
- **Re-check:** the resume on the same serve and the SAME staged wfc (`85c59065f48a`, `serve-6580-f.txt`) at load ~20 passed all four
  wfc bitmap/grid2d rows at 0 frames/s. A targeted open diagnosis at 10:49 (`f2-open-diagnose.mjs` → `f2-open-s16.json`, load 13):
  both editors open (plugin `loaded`), 2 windows each (`wfc-bitmap-input`/`-output`, `wfc-grid2d-grid`/`-preview`), **0 console
  errors, 0 page errors, 0 failed requests, 0 notices**. Note for the matrix: the palette has no `spawn.wfc.s.wfc.bitmap@1/*#editor`
  row — the bitmap editor is the plugin row `spawn.wfc` (its default program), so a matrix that presses only per-program rows will
  miss it.
- **Verdict:** not reproducible on the current tree; most likely load + a dying Chromium at 06:5x. Re-check in the post-restage
  matrix with console capture.
