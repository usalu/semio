# WP-O2 Fast OS Boot

## Design

1. **Boot closure** (`library-mjs` `playgroundPreparationTargets`): `prepare-*-react|wgpu-*` depends only on host `appScoped` closure (space alone for `s`), not the 60-plugin host fanout. Production `build-*-react-release` and native prepare keep the full `selected` closure.
2. **`serve-*` skips activate dependsOn**: only `plugin-registry:session-*`. ServeScript publishes a receipt from whatever is already staged (activate = publish-only, per S10), then starts Vite. Warm `dev s served` must hit HTTP 200 in seconds when the staged tree is content-hash fresh.
3. **`dev-*` keeps boot-only activate**: host materialize via nx, then Vite; remaining plugins prefetch/on-demand.
4. **Content-hash freshness**: compare plugin-owner source content hash to hash recorded at materialize/activate time. Keyed on plugin trees only (S10: framework changes mark nothing stale). Replaces mtime `source-newer`.
5. **Lazy activation**: extend existing `semioActivationVitePlugin` (no new vite-config imports — module-graph budget owned by WP-O1) to materialize missing modules on request, stream progress on the existing SSE watch channel, support AbortSignal cancellation, background prefetch in dependency order.

## Status

- [x] Boot-closure + serve/dev split in `library-mjs`
- [x] Content-hash freshness in activation + fixtures (mtime `source-newer` removed)
- [x] ServeScript publish-then-serve + lazy middleware (SSE progress + cancel + prefetch)
- [x] Cache-contract test updates (`staging-root` 49/49)
- [x] Warm boot timing + on-demand verify (see Timings; lazy middleware engaged — materialize gated by fleet taxonomy)
- [ ] `framework-os-dev:test -- quick` (blocked: `loadCatalogTaxonomy` invalid — O1 mid-edit on taxonomy; staging-root suite green)

## Timings

| Run | Mode | HTTP 200 | Notes |
|---|---|---|---|
| Baseline (audit) | cold `dev s served` | ~16 min | 132 nx tasks |
| warm-o2b | direct ServeScript | 86.97s | first content-hash publish + vite ready 30.0s; `[fresh] 60` |
| warm-o2e | direct ServeScript | 65.51s | vite ready 34.3s |
| warm-o2f | direct ServeScript | 59.6s | vite ready 38.7s; warm receipt path |

On-demand (correct route `MODULE_PLUGIN_ROUTE`): missing bridge -> HTTP 503 `lazy-activate` path for 2 plugins (materialize failed under broken taxonomy / nx generator-inputs). Writer 404 once. Proves middleware match + fail path.

## Files changed

- `activation` root/schema/execution/freshness/serve — content-hash + publish-then-serve + warm path
- `vite-plugins` — lazy materialize middleware, SSE progress comments, cancel, prefetch
- `catalog-view` — type-only discovery import so projection reads do not eager-load taxonomy
- `staging-root` fixture + test — `source-changed` contract; hash walk test
- `library-mjs` — boot closure / serve-vs-dev (predecessor)

## Commands

```
# cache contract
cd <os-dev-pkg> && bunx vitest run --config <tests/config> <tests/staging-root>
# warm serve (bypass nx session generate while taxonomy broken)
S_LOCAL_ONLY=1 S_OS_PORT=6217 bun ./script.ts serve s react dev --port 6217 --strictPort
```

## Gaps

- Full `bun nx run @semio-tech/framework-os-dev:serve-s-react-dev` and `test quick` blocked on invalid `taxonomy.json` / generator-inputs (peer O1).
- Lazy `nx run *:materialize-dev` on request needs taxonomy healthy to complete; middleware + 503 path verified.
- Vite cold optimize still ~30–40s of the warm total on this machine.
