# Vite stale transform — deterministic repro, root cause, guard, laws (2026-09-15)

Lane `vite-stale-transform-guard`. Port owned and recycled: **:6029** (`📜️serve-generation3d-react-6029.sh`,
`screen -dmS g3dreact6029`). No peer serve was touched.

Probes: `🐍️source-watch-primitive-recon.mjs`, `🐍️stale-transform-probe.mjs`,
`🐍️stale-transform-burst-probe.mjs`, `🐍️freshness-law-counterproof.mjs`,
`🐍️stale-transform-browser-proof.mjs`.

---

## 1. Repro matrix — write style × module depth

### 1.1 The watcher primitive (`node:fs.watch` recursive, under `bun`, real repository tree)

`🐍️source-watch-primitive-recon.mjs`, 5 repeats per cell. **`namedTarget` is the whole question**: Vite keys
its module graph by the edited file's path, so an event that names anything else invalidates nothing.

| depth | write style | event named the EDITED path | event named its parent dir | events delivered |
|---|---|---|---|---|
| 8 (`🧱️elements/🕸️NodeGraph/`) | in-place append | **5/5** | 0/5 | 5 |
| 8 | `sed -i ''` (macOS temp+rename) | **0/5** | 0/5 | 6 |
| 8 | rename-into-place (editor save) | **0/5** | 0/5 | 5 |
| 8 | truncate + rewrite | **5/5** | 0/5 | 6 |
| 4 (`🖱️ui/🎨️styling/`) | in-place append | **5/5** | 0/5 | 5 |
| 4 | `sed -i ''` | **0/5** | 0/5 | 16 |
| 4 | rename-into-place | **0/5** | 0/5 | 7 |
| 4 | truncate + rewrite | **5/5** | 0/5 | 5 |
| 1 (`🌎️hub/`) | in-place append | **5/5** | 0/5 | 6 |
| 1 | `sed -i ''` | **0/5** | 0/5 | 7 |
| 1 | rename-into-place | **0/5** | 0/5 | 5 |
| 1 | truncate + rewrite | **5/5** | 0/5 | 5 |

Depth is irrelevant. The **write style** is everything, and events *were* delivered in every cell — they
named the temporary file (`.!77419!🟦️.ts`, `🟦️.ts.tmp-probe`) or occasionally the grandparent directory,
never the edited module.

### 1.2 The live serve (`:6029`, `SEMIO_VITE_HMR=0`, generation3d react dev)

`🐍️stale-transform-probe.mjs`: fetch `/@fs/<abs>`, edit, wait 2.5 s, fetch again.

| module | in-place append | `sed -i ''` | rename-into-place | `import type` only |
|---|---|---|---|---|
| `📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` (depth 8) | fresh | **STALE** | **STALE** | **STALE** |
| `🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts` (depth 6) | fresh | **STALE** | **STALE** | **STALE** |
| `🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts` (dependency of a dependency) | fresh | **STALE** | **STALE** | **STALE** |

**9 of 12 stale**, every time, for the life of the process. `mtimeMoved` was `true` in all 12 — the file on
disk was new; only the dev server disagreed.

### 1.3 The law that was already red

`🧪️tests/🧹️config/🟦️.ts > dev server watch policy > replays an atomic save over an existing source file as
a change` was **failing on `main`** before this lane touched anything:

```
AssertionError: one save of a modified module must reach Vite as a change:
  expected [ 'add:🧰️framework/🔎️arm.ts', …(2) ] to include 'change:🧰️framework/🟦️.ts'
```

The contract was written on 2026-09-13; the watcher has never met it.

---

## 2. Root cause

**macOS names a recursive `fs.watch` event after the directory entry that changed. An atomic save changes
the entry of the TEMPORARY file, so the edited module's own path is never named by any event.**

An atomic save is `write <target>.tmp` + `rename(<target>.tmp, <target>)`. FSEvents reports the temporary
file's creation and rename; the destination path's entry is reused rather than created, and libuv delivers
one coalesced event under the temporary name. `semioSourceWatchVitePlugin` tested `existsSync(temp)` —
false, because the rename consumed it — and emitted `unlink:<temp>`, a path no module graph entry is keyed
by. Nothing was invalidated, `moduleGraph.getModulesByFile(<target>)` was never called, the cached
`transformResult` survived, and `cachedTransformMiddleware` re-served it on every later request.

Three things kept this invisible:

1. **`SEMIO_VITE_HMR=0`.** `onHMRUpdate` short-circuits on `hmr === false` (vite 7.3.6,
   `chunks/config.js:25633`), so there is no reload pass to paper over a missed invalidation.
2. **The 2026-09-12 correctness proof used `touch`.** `touch` is an in-place mtime write — the one style
   that *does* name the target (5/5 above). The proof exercised the single passing cell of the matrix.
3. **Every real writer uses the failing style.** Editors save atomically by design, `sed -i ''` on macOS
   writes `.!PID!name` and renames, and every agent file-writing tool does the same. A developer's normal
   save was the case that never worked.

This is the mechanism behind all four reported symptoms: :6027's Flow window booting the plugin's fallback
`Number → Math.add` graph while the preview was correct, :6028 serving `actor-ui-patch.pairing` from a
pre-batching transform, :6018 dying on an export that had already been added, and 13:03's "moving a slider
doesn't update the preview".

**Second, separate defect found:** the wgpu serve
(`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`) declares `server: { watch: null }` and mounts **no
replacement watcher at all**. Every source edit has been stale there since `watch: null` landed,
regardless of write style.

---

## 3. Fix

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, region
`🔖️SourceFreshnessVitePlugins`. Two mechanisms over one shared registry — push, then verify.

- **`createSourceFreshnessRegistry()`** — records `{mtimeMs, size}` for every file the dev server actually
  transforms, indexed by directory. Only transformed files are tracked, so "moved" is exactly "the cached
  transform is out of date"; a file the server never read has no transform to be stale.
- **`semioSourceWatchVitePlugin({ repoRoot, freshness })`** — unchanged event replay, plus: **any** event
  re-stats the tracked modules of its directory and emits `change` for each that moved. This is what turns
  a temporary-file event into the invalidation of the module it was renamed onto.
- **`semioTransformFreshnessVitePlugin({ freshness })`** — the guarantee that does not depend on any event
  arriving. `transform` records each module's stamp; a `configureServer` middleware (installed before
  Vite's `cachedTransformMiddleware`, because `configureServer` hooks run ahead of the internal middleware
  stack — `chunks/config.js:25682` vs `:25683`) re-stats the one file behind every module request, and
  the whole transformed set behind every document request, retiring stale modules synchronously through
  `moduleGraph.onFileChange` (which walks importers) before the request is answered.
- **`semioSourceFreshnessVitePlugins({ repoRoot })`** — mounts both over one registry. Mount both or
  neither.
- **Boot log line, `[DEBUG]`-free**, injected `order: "post"` so `semio-host-html`'s document rewrite
  cannot drop it:
  `semio dev · transform freshness: stat-guard (every module request + whole graph per document) · hmr off · 360 modules verified, 1 stale transforms retired · serve pid 12460 · document 2026-09-15T12:36:17.616Z`
  A stale serve is now one console line away: `retired` counts what that page load had to throw away.
- **Staged-guest verdict made visible.** `semioActivationVitePlugin` already computed which staged plugin
  modules are behind their own source and printed it to the *server* log only. It now also emits it into
  the page:
  `semio dev · 50 staged plugin module(s) are behind their source — the host in this page may speak a newer wire contract than the guest it is talking to: …`
  That is the `actor-ui-patch.pairing` axis: host TypeScript newer than the guest wasm it talks to.

### Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` | registry, directory rescan in the watcher, request-time stat guard, boot banner, activation verdict in the page |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` | mounts `...semioSourceFreshnessVitePlugins({ repoRoot })` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts` | same plugins on the wgpu serve, which had no watcher at all |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` | new `dev server transform freshness` laws; the red atomic-save law now carries a registry |

---

## 4. Laws, with output

`🧪️tests/🧹️config/🟦️.ts > dev server transform freshness` — spins up a **real** vite dev server
programmatically in a sandbox, mounts the plugins, and edits a fixture module by each write style with
**no settle window at all**:

- serves the current file on the FIRST request after *an in-place write* / *an atomic save (rename into
  place)* / *`sed -i ''` (macOS temporary + rename)*
- re-verifies the whole transformed set on a document request (leaf edited atomically, document requested,
  leaf request must carry the edit) and asserts the freshness banner is in the served document
- `requestedTransformFile` resolves `/@fs/…`, `?import&t=`, `/@vite/client` and `/`
- the wgpu serve mounts the same plugins

```
bun ./📜️script.ts test quick --testNamePattern='dev server transform freshness|dev server watch policy'
 Test Files  1 passed | 2 skipped (3)
      Tests  32 passed | 141 skipped (173)
```

That includes the previously-failing `replays an atomic save … as a change`, now green.

**The laws are counterproofed, not assumed.** `🐍️freshness-law-counterproof.mjs` runs the identical sandbox
server with the plugins removed:

| mounted | in-place write | atomic save | `sed -i ''` |
|---|---|---|---|
| no watcher at all | **STALE** | **STALE** | **STALE** |
| watcher only (pre-fix shape) | **STALE** | **STALE** | **STALE** |
| watcher + stat guard (fix) | fresh | fresh | fresh |

A first draft of these laws was **vacuous** and is recorded here so it is not rewritten: fetching a
root-relative `/🧰️framework/🟦️.ts` is answered by Vite's *static* middleware from disk, never from a cached
transform, so every cell read "fresh" even with no watcher mounted. The laws now fetch `/@fs/<abs>` and
assert the response has had its TypeScript type annotation stripped, which proves the measured bytes really
are a cached transform.

### Verification on :6029, end to end

Serve recycled onto the fix (`ready in 6864 ms`), then:

- `🐍️stale-transform-probe.mjs` — **12/12 fresh** (was 9/12 stale).
- `🐍️stale-transform-burst-probe.mjs` — zero-delay atomic save, next request with no sleep: **fresh**.
  20 `🧱️elements/*/🟦️.tsx` host modules saved atomically at once: **0 stale**. Marker residue: 0.
- `🐍️stale-transform-browser-proof.mjs` — real chromium, real playground:
  ```
  boot: 93 console lines, 0 page errors
  banner: … 353 modules verified, 0 stale transforms retired · serve pid 12460
  after atomic save + reload: edited module executed = YES (fresh)
  banner: … 360 modules verified, 1 stale transforms retired
  after removing the debug line: still executed = NO (fresh)
  final page errors: 0 · marker residue: false
  ```

All `[DEBUG] stale-guard` lines were stripped; `grep -ra "stale-guard" 🧰️framework ✏️s 🌎️hub` is empty and no
`*.stale-guard-tmp` / `*.tmp-probe` files remain.

---

## 5. Not claimed

- **The wgpu serve was not booted.** Its config now mounts the same plugins and a law asserts the mount,
  but no wgpu browser session was run this lane. Its `🎚️config/🟦️.ts` also reaches into
  `🧑‍💻dev/🔌️vite-plugins/🟦️.ts` for the first time; that import resolves and the module loads under `bun`,
  but the wgpu serve's own start was not exercised.
- **Every peer serve is still stale.** Vite reads its config once. :6013, :6018, :6021, :6022, :6023,
  :6024, :6025, :6026, :6027, :6028 are all running the pre-fix config and will keep serving pre-edit
  transforms until their owners recycle them. Only :6029 was recycled.
- **Windows and Linux are reasoned, not measured.** The stat guard is platform-independent by
  construction; the watcher's directory rescan is a superset of what it did before on every platform. Only
  macOS/APFS was exercised.
- **Two suites are red and were red before this lane**, both untouched by these edits:
  `🧪️tests/🧹️config/🟦️.ts > vite config module graph` (2 cases — the declared module/source-byte bounds and
  the Bun-bundler agreement; already recorded as pre-existing in `📓️vite-serve-wedge-2026-09-12.md` §6) and
  `🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` (3 cases in `rewriteJcoComponentAssetUrls` and
  `buildPluginCatalog`).
- **A peer's in-flight refactor of `🎭️actor/📥️cold-pair` breaks the dependency pre-scan on every serve**
  (`No matching export … coldDocumentPairCursorEquals`). It is logged at startup, is not caused here, and
  did not affect any measurement above.
- **The guard bounds staleness of the served module against its own source. It does not verify a served
  host module against a staged guest's wire contract** — that comparison is the activation receipt's, and
  this lane only made its existing verdict visible in the page rather than turning it into a boot refusal.
