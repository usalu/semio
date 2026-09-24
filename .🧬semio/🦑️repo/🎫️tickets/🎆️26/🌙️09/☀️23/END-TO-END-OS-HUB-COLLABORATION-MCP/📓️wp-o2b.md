# WP-O2b Warm Boot + Lazy Activate (Coordinator Review)

## Verdict (post-review)
- Warm direct ServeScript HTTP 200: **3.98 s** (stat-index freshness, non-blocking boot pass). Target under 10 s: **PASS**.
- Marker-trust `walkSources: false` removed. Boot uses schema-first `.source-stat-index.json` (stat walk; hash only size/mtime deltas).
- `serve-*-react-*` `dependsOn` restored to `@semio-tech/plugin-registry:session-${variant}` (zero-touch). Empty `dependsOn: []` rejected.
- Staging-root tests: **53 passed** (includes a–d boot freshness cases).
- Live stale proof (animate): edit one source byte → `source-changed` + serve `[stale] animate…` → restage markers → revert → clean. **PASS**.
- NX_DAEMON (isolated `NX_SOCKET_DIR` + `NX_WORKSPACE_DATA_DIRECTORY` under `.tmp-ticket/wp-o2b/generated/nx-iso`): cold graph prime **334.6 s**; first warm nx serve HTTP 200 **241.2 s** (dependsOn chain, 0/3 cache on that iso). Graph wait remains the dominant nx cost vs direct ServeScript.

## Stat-index freshness
- Schema: `SourceStatIndexV1` in activation schema (`semio.dev.source-stat-index/v1`) next to receipt schema.
- Staging writes `.source-content-sha256` + `.source-stat-index.json` via `writeStagedSourceFreshness`.
- Boot: parallel async per-plugin stat walk; reuse digests when `(size, mtimeNs)` match; rehash only changed/added; removed files change aggregate.
- Serve-start freshness runs non-blocking (`void reportServe…`); vite plugin skips duplicate sync walk on first listen.
- Tests: (a) unchanged → `hashedFileCount=0`; (b) touch identical → rehash 1, still fresh; (c) edit → stale; (d) add/remove → stale.

## Live (c) proof
- File: animate rust source (+1 newline). Probe: `hashed:1 reused:434 kind:source-changed`.
- Serve log: `[stale] animate: source-changed — source 89ad10b67f58… ≠ staged 40353db12bdb…`.
- Restaged with edited tree → fresh; reverted source; restaged original. Source bytes match pre-edit.

## dependsOn / zero-touch
- Restored: `dependsOn: [\`@semio-tech/plugin-registry:session-${playground.variant}\`]`.
- Empty staged `animate` module (backup under generated scratch, never peer cache wipe): ServeScript listen then `[lazy-activate] materialize animate via @semio-tech/animate-plugin:materialize-dev` — missing module materialized with progress path; module restored after.

## NX_DAEMON warm (isolated)
| Step | Seconds | Notes |
|---|---|---|
| `nx show projects` prime into iso dirs | 334.6 | cold graph into `generated/nx-iso` |
| First `serve-s-react-dev` → HTTP 200 | 241.2 | session/generate deps; Vite ready ~2.5 s after ServeScript starts |
| Direct ServeScript warm HTTP 200 | 3.98 | no nx graph |

Captures: `generated/nx-daemon-prime-meta.txt`, `warm-nx-daemon-serve2.txt`, `warm-final.txt`, `live-stale-*.txt`, `zero-touch-*.txt`.

## Prior (still valid)
- `test quick` 159 passed (pre-review).
- Lazy activate SSE + AbortSignal evidence retained from earlier O2b.
- Cold empty vite optimizeDeps: ~7.9 s (pre-review).

## Root files touched (this review)
- activation root + schema — stat index + `resolveBoot` rewrite; deleted `walkSources`
- freshness — async collect; no marker trust
- serve — non-blocking freshness start
- vite-plugins — `writeStagedSourceFreshness`; skip boot duplicate walk
- execution — write freshness index on activate
- library.mjs — restore session `dependsOn`
- staging-root tests — cases a–d
