# Fix 2026-09-17 — page-shared plugin/extension watch `EventSource`

## Symptom and mechanism (already proven before this packet)

Headless measurement on the Entwerfen-mit-Bestand demonstrator (`http://127.0.0.1:6029`, probe
`🔍️landing-sse-probe.ts` in this ticket folder, read-only): every `FrameworkOsShell` mounted on a page
opened **two** permanent `EventSource` connections when its `pluginSource` memo subscribed —

- `createDevPluginSource(registry, "/🔌️plugin-modules/watch")`
- `createExtensionSource(PLUGIN_CATALOG, "/🧩️extension-modules/watch")`

multiplexed by `multiplexPluginSources`. The dev server is HTTP/1.1 and Chromium allows six connections
per origin, so shell 1 held 2, shell 2 held 4, and shell 3's two SSE exhausted the budget (they only
connected ~20 s later). Every later fetch of the page (plugin descriptor `🔣️.json`, `.core.wasm`)
queued behind six idle streams, so shell 3 and every later pane stayed in "booting" with no console
output; the 8-pane landing page never got past *koordinator* (the boot deadline is forgiven by other
shells' shard beats, so nothing was logged).

Cost was `2 × shells` streams per page. It is now `1 per distinct watch URL per page` (2 total),
independent of how many shells the page mounts.

## Design

New module-level registry in `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, region `🔌️PluginSource`:

```
type SharedWatchStream = { source: EventSource; listeners: Set<(data: string) => void>; lastSnapshotData: string | undefined }
const sharedWatchStreams = new Map<string, SharedWatchStream>()
function subscribeSharedWatchStream(watchUrl: string, onData: (data: string) => void): () => void
```

Rules implemented:

- **One stream per URL.** The first subscriber constructs the `EventSource`; later subscribers of the
  same URL only add a listener. Sharing is per JS module instance, i.e. per page bundle — the
  demonstrator's panes are components in one document, so all of them share it.
- **Refcount.** Unsubscribing removes the listener; when the set empties, the `EventSource` is closed
  and the map entry dropped, so a later subscription opens a fresh stream. The returned unsubscribe is
  idempotent (a double call cannot evict a newer stream on the same URL).
- **Snapshot cache + late replay.** The dev/extension endpoints only send `kind: "snapshot"` at connect
  time, and ShellHost's install pump depends on receiving one. The shared stream keeps the **raw `data`
  string of the last snapshot** and replays it to every LATE subscriber via `queueMicrotask` (not
  synchronously, so `subscribe` has returned before the caller is re-entered). A subscriber that
  unsubscribes before the microtask runs is not called.
- **Live fan-out.** `built`/`installed` (and any other) frames are delivered to every current listener,
  over a copy of the listener set so a listener that unsubscribes during dispatch is safe.
- **Per-listener parse and warn.** The shared handler fans out the raw `data` string; each source's
  `subscribe` callback does its own `JSON.parse` in `try/catch` and keeps its existing warning text
  (`plugin source "dev" malformed event…` / `…"extensions"…`). The shared handler parses once more only
  to decide whether the frame is a snapshot worth caching; a malformed frame is not cached and still
  reaches every listener, which each warn exactly as before.
- **Extension normalization stays per listener.** `createExtensionSource.subscribe` still runs
  `extensionSourceEventToPluginSourceEvent` in its own callback (so `uninstalled` is dropped per
  listener), including on a replayed cached snapshot.
- **`typeof EventSource === "undefined"` no-op preserved** (plain node/vitest without the fake).
- **`onerror` deliberately unhandled**, exactly as before: `EventSource` auto-reconnects and the server
  answers every reconnect with a full snapshot, which is fanned out like any other event; ShellHost
  drops replays through `pluginAvailabilityRouteV1`.

No behaviour change was made in ShellHost: a subscription-effect re-run used to open a fresh
`EventSource` whose connect-time snapshot replayed; it now re-subscribes and gets the cached snapshot
replayed. Same observable input, so the wave-B40 hot-swap guard still governs it.

## Files changed

| File | Change |
| --- | --- |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | New `SharedWatchStream` type, `sharedWatchStreams` map and `subscribeSharedWatchStream()` in region `🔌️PluginSource`; `createDevPluginSource.subscribe` and `createExtensionSource.subscribe` now go through it; doc updates on `PluginSource.subscribe`, `createDevPluginSource`, `createExtensionSource`; new `🧪️SharedWatchStreamTests` in-source registration block at the end of the file. |
| `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` | New `registerTests6` — "page-shared plugin watch streams" suite with a fake `EventSource`. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | Comments only (the two contract comments at ~3537 and ~4613 now say the stream is page-shared per URL and the snapshot is replayed from cache). No logic touched. |

## Tests

`registerTests6` installs a `FakeEventSource` class on `globalThis` per case (saved/restored around each
test) that records every construction, and pushes frames through `onmessage`. Nine cases:

1. two shells subscribing to the same watch url → exactly **one** `EventSource` constructed;
2. a late subscriber receives the cached snapshot (and only on a microtask, not re-entrantly);
3. a listener that unsubscribes before the microtask runs gets no stale replay;
4. live events fan out to every listener, and a departed listener stops receiving while the survivor's
   stream stays open;
5. the last unsubscribe closes the source; a later subscription opens a **new** one; double unsubscribe
   is a no-op;
6. different watch urls → different streams;
7. extension wire events are normalized per listener on one shared stream (`installed` → `built`,
   `uninstalled` dropped, cached snapshot replayed normalized);
8. a malformed frame warns once **per listener** and the stream keeps working;
9. no `EventSource` on `globalThis` → harmless no-op.

Command (from `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript`, project
`@semio-tech/framework-kernel`, target `test-quick`):

```
$ bun ./📜️script.ts test quick
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel
 ✓ 🟦️.ts > page-shared plugin watch streams > opens exactly ONE EventSource for two shells subscribing to the same watch url 1ms
 ✓ 🟦️.ts > page-shared plugin watch streams > replays the cached snapshot to a LATE subscriber (the endpoint only sends one at connect) 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > does not replay a stale snapshot to a listener that unsubscribed before the microtask ran 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > fans LIVE events out to every listener on the shared stream 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > closes the EventSource when the LAST listener leaves, and opens a new one for a later subscriber 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > keeps DIFFERENT watch urls on different streams 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > normalizes extension wire events PER listener on one shared extension stream 0ms
 ✓ 🟦️.ts > page-shared plugin watch streams > warns ONCE PER LISTENER on a malformed frame and keeps the stream alive 1ms
 ✓ 🟦️.ts > page-shared plugin watch streams > is a harmless no-op where EventSource does not exist (plain node) 0ms
 Test Files  1 failed | 2 passed (3)
      Tests  1 failed | 67 passed (68)
```

The single failure is **pre-existing and unrelated** —
`📤️return/📦️content/🟦️.ts > KernelReturnContentFraming matches the shared stream and independent frame
encoding at every split`, an ajv schema-resolution error:

```
Error: can't resolve reference https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/NonZeroU64
       from id https://json.schemas.assets.semio-tech.com/framework/actor/lifetime/schema.json
```

That file is untouched by this packet, does not import the kernel root module, and fails the same way in
isolation (`bun ./📜️script.ts test quick "📦️content"` → `Test Files 1 failed (1) | Tests 1 failed | 7 passed (8)`,
same ajv message).

## Type-check

There is no tsconfig under `🧰️framework`; the nearest is the repo root `/Users/ueli/Documents/semio/tsconfig.json`
(whose `include` is the whole repo). Checked the two edited kernel files with the root config's
compiler options plus `--allowImportingTsExtensions` (the root config's `include`-wide run is repo-scale
and full of peer churn):

```
$ bunx tsc --noEmit --allowImportingTsExtensions --target ESNext --module ESNext --lib DOM,ESNext \
    --moduleResolution bundler --strict --esModuleInterop --isolatedModules --resolveJsonModule \
    --skipLibCheck --jsx react-jsx \
    "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts" \
    "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts"
```

**Zero errors in either edited file** (`grep` for `🎠️kernel/🟦️.ts` and `createturnoutcomebroadcast` over
the output returns nothing; `--listFiles` confirms both files are in the program). The 73 reported errors
all come from OTHER files pulled in transitively and are pre-existing, caused by the standalone flags
lacking bun/vitest ambient types — e.g. `Property 'dir' does not exist on type 'ImportMeta'`
(`🎭️actor/📃️page/🟦️.ts`, `🎭️actor/📤️return/🟦️.ts`, `📤️return/📦️content/🟦️.ts`),
`Untyped function calls may not accept type arguments` in `🎭️actor/📬️mailbox/🧪️tests/…`, and
`Cannot find module '../🚪️lifetime/🟦️component.js'` in `🎭️actor/🤖️generated/…` (wasm-component
bindings that only exist after a component build).

## Other `new EventSource(` openers in the repo

Grep over `*.ts`/`*.tsx`/`*.rs`, excluding `node_modules`, ticket folders, `🗑️generated`, bundled
`🧶️bundles`/`📤️distribution` output:

1. `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2757` — the new shared one (this packet).
2. `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:3077` —
   `connectSseOnce`, `${FOLDER_ENDPOINT_PATH}/watch?uri=folder://<path>`. **Not changed** (report only).

   Scope of (2), traced:
   - The backbone store worker is created **once per `FrameworkOsShell`** (`ensureBackboneWorker`,
     `ShellHost/🟦️.tsx:2785`, memoized in `backboneWorkerRef`), so it is per shell, i.e. per pane, and
     N shells on a page run N workers. Workers share the page's per-origin HTTP connection pool.
   - Inside a worker, the stream is **per open document**, not per worker: `openArtifact` calls
     `watchFolder(state, folder)` (`👷️worker/🟦️.ts:5954`) once per opened document that has a
     `kind: "folder"` binding and `watchExternal !== false`, and each `watchFolder` keeps exactly one
     `EventSource` alive via `connectSseOnce`/`reconnectForever`. The watch URL is keyed by the folder
     path, so two documents in the same folder still open two streams.
   - Worst case is therefore `shells × folder-bound open documents` streams, and they are NOT
     deduplicated in any way.
   - It does **not** fire on the dev demonstrator page today: folder bindings are only produced when a
     document has a `spaceId` AND a `dataDir` is configured
     (`ShellHost/🧭️opening/🟦️.ts:44`, `resolveDocumentOpeningBindings`), and identity's own folder lane
     needs `S_DATA_DIR` (`👷️worker/🟦️.ts:2753`, `identityActorConfig`). A plain browser tab without
     `S_DATA_DIR` gets `bindings: []`, which matches the probe: only the plugin/extension watch streams
     were observed. On desktop/`S_DATA_DIR` runs with several panes over shared spaces this openers'
     count grows the same way the plugin watch used to, and would need the same treatment (a shared
     stream keyed by folder URI, ideally hoisted out of the per-shell worker).

No other SSE/long-lived stream openers exist in source (hub traffic uses `WebSocket` in the same worker,
`connectHubOnce`, which is per document as well but out of scope here).
