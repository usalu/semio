# F2 — frontend transport + runtime performance of the os `s` shell

Slice F2, session 13 (2026-09-26 19:0x). Coordinator = main Claude Code chat. Continues [F1](📓️wp-f1.md) (runtime performance) and
the U5 16:3x connection finding ([U5](📓️wp-u5.md) §12-6, `📓️work-packages.md` 16:3x). Ports 8080–8089 / 6580–6589. Builds/tests
`nice -n 10`. Captures `wp-f2/generated/` (expendable); durable data/logs `.🧬semio/🌐hub/s13-f2-*`.

Status legend: **measured** = ran here, capture named; **unverified** = read from source only; **written, not run**.

## Session 13

| # | item | state | evidence |
|---|---|---|---|
| F2-1 | per-origin connection budget: inventory per origin at install time → ONE multiplexed channel per origin (schema-first frame contract TS + Rust twin, progress/cancel, backpressure, reconnect+resume), fixture law + third-party oracle, live before/after + stress | **done, measured live**: long-lived HTTP/1.1 streams on the serve origin **4–5 of 6 → 0**; every watch/folder/job stream on ONE WebSocket per page (workers bridged); stress 66 streams + 300 fetches on one link: 300/300 fetches done, 96/96 folder notices delivered (same shape through per-stream SSE: 6 streams → **0/300** fetches in 20 s). Law 81/81 (AJV oracle), framework 265/265, store 9/9. Rust twin: none — no Rust consumer (checked) | `f2-conn-{hub2,after2}.json`, `f2-stress-stress1.json`, `f2-sse-contrast.txt` |
| F2-2 | runtime performance sweep: cold `dev s` boot timeline, first paint, hub document open latency, hub catalog plugin install, typing/paint latency in 3 editors → root-fix top offenders + laws | pending | — |
| F2-3 | idle cost re-verify (F1 146/146) on the current tree; memory growth over 30 min with 10 windows | pending | — |

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
