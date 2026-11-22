# Native Dev Serve Recipe (Process 3D, react)

## Pipeline the launcher runs before vite
- `ensureCatalog` → `ensureBrowserSupport` → `ensureRendererEngines` → `ensureBootComponents` (generated inputs, descriptor emitter, cargo wasm32-wasip2 → jco transpile → publish) — 🎮️playground/🟦️.ts:50-156.
- Browser loads jco output from `/🔌️plugin-modules/🏭️process/` → `.cache/b/p/m/browser/prototype/modules/🏭️process` (♻️activation/📂️paths/🟦️.ts:7-8). Currently missing.
- Serve needs activation receipt `browser/prototype/runtime/react/process3d/activation/🔣️receipt.json` (ServeScript :47-58).

## Vite
```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
SEMIO_PLUGIN=process3d SEMIO_RENDERER=react SEMIO_BUILD_MODE=prototype VITE_SEMIO_PLUGIN=process3d VITE_SEMIO_RENDERER=react S_OS_PORT=6022 \
  bun node_modules/vite/bin/vite.js --mode prototype --config ✏️s/🧑‍💻dev/🎭️variants/🏭️process/🏗️builder/🌐️vite/🟦️.ts \
  --configLoader native --host 127.0.0.1 --port 6022 --strictPort
```
Env set in ♻️activation/🌐️serve/🟦️.ts:42-67; spawn ⚡️caching/🌐️vite/🟦️.ts:107. Hub session / locked prefs env not reproduced.
