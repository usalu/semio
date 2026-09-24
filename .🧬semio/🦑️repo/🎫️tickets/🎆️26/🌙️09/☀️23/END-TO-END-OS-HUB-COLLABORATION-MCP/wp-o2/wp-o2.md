# WP-O2 Fast OS Boot

## Design

1. **Boot closure**: `prepare-*-react|wgpu-*` depends only on host `appScoped` closure (space alone for `s`). Production `build-*-react-release` keeps full `selected` closure.
2. **`serve-*`**: dependsOn session only. ServeScript activates (publish-only) only when receipt missing or host unnamed.
3. **Content-hash freshness**: plugin trees only; served freshness trusts staged/receipt hash when plugin mtime is not newer than staged (no full rehash every boot).
4. **Lazy activation**: inline in `semioActivationVitePlugin` (SSE + abort + prefetch). No new vite-config imports.
5. **Vite configLoader**: Bun → `native`; Node → `bundle` (`bundle` under Bun failed with opaque `undefined`).

## Status

- [x] Boot-closure + serve/dev split
- [x] Content-hash + conditional ServeScript publish
- [x] Freshness mtime-gate
- [x] Bun-native configLoader (`.ts` + `.js`)
- [x] Warm ServeScript HTTP 200 + staged module GET
- [~] `framework-os-dev:test -- quick`: **155 passed**, 4 failed (transform-freshness timeouts 60s in `tests/config` — mkdir/rmSync; not activation/hash assertions)

## Timings

| probe | result |
|---|---|
| `prepare-s-react-dev` materialize deps | **1** (space) |
| `serve-s-react-dev` materialize deps | **0** |
| `build-s-react-release` materialize deps | 60 |
| warm ServeScript → HTTP 200 (:6221) | **35.55 s** (Vite ready 23.8 s) |
| always-activate + full rehash (before fix) | ~80–86 s |
| Bun+`bundle` configLoader | fail (`undefined`) |
| GET staged space bridge | HTTP **200**, 10996 B |

Captures under `.tmp-ticket/wp-o2/generated/`.

## Files

- `library/🟦️.ts` + `.js` — runtime-aware configLoader
- `library-mjs` — bootSelected / serve deps
- activation / freshness / serve / vite-plugins — hash + lazy activate
- staging-root fixtures

## Gap

- On-demand **materialize** of a missing wasm module not runtime-probed (fleet `wasm` mutex held by s14 restage). Middleware + hash-on-materialize are in code; staged module serve verified.
- Transform-freshness quick-test timeouts look load/flake; re-run when quieter or raise timeout — owned adjacent to WP-O1 vite graph.
