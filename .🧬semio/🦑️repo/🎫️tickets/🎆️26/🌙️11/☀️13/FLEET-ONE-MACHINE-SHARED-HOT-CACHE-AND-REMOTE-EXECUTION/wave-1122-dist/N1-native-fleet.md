# N1 native-build driver lane g never engaged the fleet tier

Cause: `compilerWrapperEnvironmentV1` (native-build/🎟️admission/🟦️.ts:~412) hand-assembled RUSTC_WRAPPER/SEMIO_UNIT_CACHE_* for lane `g` from the driver environment and never merged `fleetRemoteEnvironmentV1`; `readFleetSettingsV1` therefore saw no SEMIO_REMOTE_CACHE_ENDPOINT. Only `unitCacheEnvironmentV1` (repo cargo lanes q/p) included it. Other cargo entry points: wasm-pack and component passes go through the same driver function (component-pass/🟦️.ts:503); the bench fleet runner sets its own test env (not a launch path). No other hand-built wrapper env exists. (Stage-fleet/component-final tier is separate; D1 noted it reads SEMIO_REMOTE_CACHE_ENDPOINT from env, untouched here.)

## Change
- unit-cache/🚪️launch/🟦️.ts: new `unitCacheChainEnvironmentV1` is the single assembly (wrapper, root, lane, inner chain, slots, focus, features, workspace root, fleet env). `unitCacheEnvironmentV1` = ensure shim + chain.
- native-build/🎟️admission/🟦️.ts: `compilerWrapperEnvironmentV1` delegates to the chain (lane g, inner [scheduler, RUSTC], build-scripts+broker-replay on); now also sets SEMIO_UNIT_CACHE_WORKSPACE_ROOT and the fleet variables (empty when SEMIO_UNIT_CACHE_REMOTE=off or no cache named).
- Test (admission/🧪️tests/♻️replay/🟦️.ts): driver chain equals `fleetRemoteEnvironmentV1(env)` field by field; `SEMIO_UNIT_CACHE_REMOTE=off` leaves no SEMIO_REMOTE_CACHE_* key.

## Tests
- replay test file: 5 pass, 0 fail.
- unit-cache tests file: not completed (host queue 1060 s then "Original Script deadline refused"); instead a direct probe showed unitCacheEnvironmentV1 key set unchanged and remote=off drops all SEMIO_REMOTE_CACHE_*.

## Live check (main checkout, scratch cache, upload disabled)
`N1-live-probe.ts`: tiny crate through the real sealed wrapper with the driver chain (lane g). Chain keys include all SEMIO_REMOTE_CACHE_* + WORKSPACE_ROOT. Events: lane g `remote-miss` then `miss`, `publish`; `y/lookups.jsonl` row lane "g" has `"remote":"c0f681e5…"` and slot (previously never present). Earlier run with a broken scheduler stub also showed `remote-miss` before the failure.

## Pinned clone sync list (/Users/ueli/.sinc/play; do not write from here)
1. 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts
2. 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts
(3. test only: …/🎟️admission/🧪️tests/♻️replay/🟦️.ts). Effective only for NEW builds: wrappers are sealed per session; a running play build keeps its old env.

Open: stage-fleet tier for component finals not re-audited here; unit-cache tests file needs a quieter host.
