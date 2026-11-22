# Native Browser E2E (Process 3D, react, prototype)

## Result
Vite serves http://127.0.0.1:6022/ natively (PID in `🗑️generated/vite.pid`, log `🗑️generated/vite.log`). HTTP-level checks pass (index 200, entry transform 200, bridge.js 200, 400 MB core wasm 200). **Browser verification NOT done**: no Chrome/Chromium on this machine (`/Applications/Google Chrome.app` absent, no ms-playwright cache) and the Claude Browser MCP pre-tool hook timed out. `verify-process-e2e.ts` is adapted (outDir = `🗑️generated`, logs plugin-module responses) but not yet run; it needs `playwright install chromium` (download, needs user approval) or a Chrome path.

## Commands
- bun: `~/Library/Caches/semio/runtimes/bun/*/bun` (not on PATH).
- Stale workspace links: 180 `@semio-tech/*` packages (e.g. `ui-layout`, `process-js`) were missing from `node_modules/@semio-tech` (cache dir `.cache/b/d/m/j/<id>/node_modules`). Symlinked them (what `repo dependencies sync` does; the repo MCP hook timed out).
- Extension components (same target dir `🗑️generated/native`): `🗑️generated/build-ext.sh` (wasm-prototype profile flags) for metal, wood, robotic, concrete (9-12 min each).
- `🗑️generated/publish-modules.ts <G> <closure.json>`: calls the real `transpilePluginComponentModulesAsync` (native transpile binary), `hostShimSource`, `rewritePreview2ShimImports`, `pluginComponentBridgeSource`, `finalizePluginDescriptor`, the real descriptor probe source under node JSPI, `encodeComponentReceiptV1`. Output: `.cache/b/p/m/browser/prototype/modules/<dir>` for process + 4 extensions.
- `🗑️generated/activate-native.ts <closure.json>`: real `collectActivationArtifactsV1`, `nextActivationReceipt`, `publishActivationReceipt`; installs extensions into `runtime/react/process3d/extensions`. Receipt: `.cache/b/p/m/browser/prototype/runtime/react/process3d/activation/🔣️receipt.json` (5 plugins).
- Vite (cwd dev package dir): env `SEMIO_VITE_CACHE_DIRECTORY=.cache/b/p/m/vite/state/process3d-react-prototype` (required, absolute, inside products partition) + recipe env; `bun <root>/node_modules/vite/bin/vite.js --mode prototype --config ✏️s/🧑‍💻dev/🎭️variants/🏭️process/🏗️builder/🌐️vite/🟦️.ts --configLoader native --host 127.0.0.1 --port 6022 --strictPort`.

## Why not the real stage code
`publishComponentBrowserV1` / `ActivationScript` need a repo-issued `ScriptInvocation` (leases, stores) received from the launcher via env; it cannot be created natively. Lower-level real functions were used instead. Deviations: receipt `sourceTreeDigest`/`provenanceSha256` are not cargo-observed (derived hashes); `.semio-artifact.json` written by the script; the activation support digest excludes the infinite fonts artifact; optimize stage skipped (prototype is unshipped, as in real code).

## Root causes found
1. Workspace node_modules link set stale (new packages not linked) -> `Cannot find module '@semio-tech/ui-layout'`.
2. Vite config demands `SEMIO_VITE_CACHE_DIRECTORY` (not in recipe).
3. Unfixed source inconsistency: `host-capacity` policy JSON has `estimates.maximumPhaseMegabytes` that the schema forbids (`🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision`); breaks any bun import of repo-lib control modules (catalog-view). Another agent's area; left alone.

## Files changed in source
None. Generated: node_modules symlinks, `.cache/b/p/m/browser/prototype/{modules,runtime}`, `.cache/b/p/m/vite/state/...`. Ticket: `verify-process-e2e.ts` edited; scripts under `🗑️generated`.
