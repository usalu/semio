# Native Storybook proof: puzzle 5D

Graph service and capacity gate were congested, so everything ran natively.

## Commands
1. wasm: `cd 🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust && repo cargo --store=shared --configuration=prototype --lang=en -- build --lib --profile prototype --target wasm32-unknown-unknown` (95 min, 1092 crates), then `.cache/b/r/m/cargo/home/bin/wasm-bindgen <lane>/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm --out-dir 🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg --typescript --target web --out-name semio_puzzle` (40 min, 296 MB bg.wasm, prototype/no-opt); hand-wrote `pkg/package.json` like `buildWasmWebV1`.
2. Staged the package where the storybook managed-wasm plugin serves it: copied `pkg/semio_puzzle{.js,.d.ts,_bg.wasm,_bg.wasm.d.ts}` into `.cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c/`.
3. Storybook: `SEMIO_VITE_CACHE_DIRECTORY=$PWD/.cache/b/p/m/a/native-vite-5d SEMIO_BUILD_MODE=prototype STORYBOOK_SCOPE=puzzle/5d node node_modules/storybook/dist/bin/dispatcher.js dev -p 6010 --ci --no-open --disable-telemetry` (node, not bun: bun path for `bin/index.cjs` does not exist in storybook 10).
4. Playwright (bun, because the spec imports JSON without attributes): `bun node_modules/playwright/cli.js test -c <ticket>/native-pw.config.ts -g puzzle5d` with baseURL http://localhost:6010/, workers 1, swiftshader GL args.

index.json has `🧩️puzzle🕐️5d` Concrete Forest, Nakagin Capsule Tower, Capsule Dream (+ Docs).

## Result: 4/4 pass
- nakagin boots assembled (180/180), scrub to oldest reveals 1 part: PASS
- concrete-forest 1 part / 0 fasteners: PASS
- nakagin-capsule-tower 180 / 179: PASS
- capsule-dream 2880 / 2864: PASS

## Fixes
- `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-timeline/🧪️.story.tsx`: root wrapper `height:"100%"` -> `"100vh"`; the 180-row timeline stretched the world canvas to 4326 px so the background click point was off-screen.
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts`: `clickCanvasPoint` hit-test accepts the canvas event source (r3f `eventSource` sets canvas `pointer-events:none`, so `elementFromPoint` returns its parent); capsule-dream debug visibility timeout 5s -> 60s (software GL, 2880 parts).
- Fixture counts untouched.

## Gaps
- The `pkg/` is prototype (unoptimised, 296 MB); `repo run ...:wasm` would stage a managed pkg itself. The manual staging dir under `.cache/b/p/m/browser` is not hashed from content; a real run replaces it.
- Playwright run under bun natively, not through the registered runner; puzzle3d/infinite cases not run (scope inactive).
- Cargo lane dir was GC'd mid-bindgen; bindgen survived via open fd.
- Server PID stopped. Env needs `SEMIO_VITE_CACHE_DIRECTORY` (not set by plain node launch).
