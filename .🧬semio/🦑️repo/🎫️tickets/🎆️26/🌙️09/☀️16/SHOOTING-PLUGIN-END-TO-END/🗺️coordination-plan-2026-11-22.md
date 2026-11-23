# 🗺️ Shooting Plugin End-to-End Coordination Plan (2026-11-22)

## Objective & Required End-State Evidence
Get the `🎥️shooting` plugin (`✏️s/🔌️plugins/🎥️shooting`, `🌎️hub/🧩️compositions/🎥️shooting`, `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting`, and renderer/catalog integration) running completely end to end with native-tool fallback whenever the `repo` graph/fleet service stalls.

### Explicit Deliverables & Verification Gates
1. **TypeScript, Schema, SQLite IO, Python Oracles & Source Admission Laws**:
   - Registered `@semio-tech/shooting-js` 11-entry suite green (route, export, icon-transform, parsing, sun, shot-camera, demo example, forest example, line-visibility, sqlite snapshot IO, demo-session).
   - Python neutral mutation oracle (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎥️mutate-shooting-1/🐍️.py`) green across all forward & inverse mutation fixtures.
   - Cross-platform source-closure boundary audit (`source-boundary-audit.mjs`) green across Linux, macOS, Windows, `wasm32-unknown-unknown`, and `wasm32-wasip2` targets (<= 32 files, <= 4000 active lines, <= 256 KiB).
2. **Rust Native Crates, Bridge, Hub Composition & Renderer Integration**:
   - All 13 crates in `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml` workspace compile & pass `cargo test --workspace --lib` + integration tests (`mutate-shooting-1`, `unit`, `semantic-contract`, `preparation-law`, `authoring-law`).
   - `✏️s/🔌️plugins/🎥️shooting/🏭️bridge` (`semio-shooting-mutation-bridge`) compiles & answers `listMutations` cleanly.
   - `🌎️hub/🧩️compositions/🎥️shooting` (`semio-hub-shooting` lib + surface tests) and `semio-hub-shooting-component` compile cleanly on native and `wasm32-wasip2`.
   - Framework icon render host & projected linework crates compile & pass their unit/export/retry tests.
3. **Component Staging (`wasm32-wasip2`), Renderer WASM Prerequisites & Dev Activation**:
   - Playground session staged (`registry ./📜️script.ts session shooting`), web support staged (`plugin-web ./📜️script.ts support`), renderer WASM & infinite fonts ready.
   - `semio-hub-shooting-component` built, described (`.descriptor.semio` verified), transpiled, and published under `.cache/b/p/m/browser/prototype/modules/🎥️shooting`.
   - `prepare` and `activate` green for `shooting` on `react` (`prototype`) and `wgpu` (`prototype`), verified across physical re-activation.
4. **Live Browser End-to-End Verification (React `6019` & Browser WGPU `6119` + `en`/`de`)**:
   - Standalone Chromium probes (`drawing-browser-check.mjs`, `wgpu-world-transport-probe.mjs`) green with third-party Sharp / Three.js verification.
   - Live React dev server on `http://127.0.0.1:6019/?plugin=shooting`:
     - `browser-export-probe.mjs` green: boot `data-semio-os-ready="shooting"`, WGPU/3D canvas rendered, Add Shot + captured camera persisted, Undo restores exact initial document, Export All Shots produces nonblank 256x256 PNG + SVG verified by Sharp, Inspector camera/framing/drawing controls persist & match live SVG preview + styled download + hidden-lines-only download, Viewer role switches cleanly and renders fault-free meshes, zero console/page faults.
     - `browser-cancel-probe.mjs` green: isolated session, 50-shot batch export shows intermediate Task Manager progress, real UI Cancel button stops batch mid-flight with `icon-render-export.cancelled`, strict prefix delivered, no late deliveries, document preserved intact.
     - Locale `de` boot & inspector/shell verification green.
   - Live Browser WGPU dev server on `http://127.0.0.1:6119/?plugin=shooting&example=base-icon`:
     - `browser-wgpu-probe.mjs` green: boot `data-semio-os-ready="shooting"`, accessibility bridge inspector edits projection/framing/hidden-lines/merge/fillColor, Save Download verifies persisted camera & drawing settings, styled SVG ink appears on WGPU canvas, Export Active Shot matches Sharp pixel oracle, zero page errors.

## Parallel Execution Lanes (`Grok 4.7 256k Medium Non-Fast`)
- **Lane A (`ts-oracles-and-laws`)**: Run & fix all 11 TypeScript tests, SQLite snapshot tests, Python mutation oracles, standalone Chromium GLB/SVG/transport probes (`drawing-browser-check.mjs`, `wgpu-world-transport-probe.mjs`), and cross-platform `source-boundary-audit.mjs`.
- **Lane B (`rust-shooting-workspace`)**: Compile & run all tests in `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml` (all 13 workspace crates + integration tests) and `✏️s/🔌️plugins/🎥️shooting/🏭️bridge` via `🦀️native-shooting-cargo.sh`, fixing any compiler/test regressions from recent framework/VCS changes.
- **Lane C (`rust-hub-and-renderer`)**: Compile & test `semio-hub-shooting`, `semio-hub-shooting-component` (native + `wasm32-wasip2` check), and framework `IconRenderHost` / projected linework crates, fixing any upstream/unrelated blockers (e.g. PDF/VCS/plugin-app/kernel-protocol).
- **Lane D (`wasm-stage-and-activate`)**: Stage `shooting` playground session, browser support, renderer WASM prerequisites (`framework-surface-rs`, `framework-editor-rs`, `framework-os-node-graph-rs`, WGPU wasm/fonts), build & materialize `semio-hub-shooting-component` via `driver-stages` (or native fallback), and run `prepare` + `activate` for `shooting` `react` and `wgpu` `prototype`.
- **Wave 2 (`live-servers-and-browser-e2e`)**: Start React (`6019`) and Browser WGPU (`6119`) servers via `🚀️launch-shooting-script.ts`, execute `browser-export-probe.mjs`, `browser-cancel-probe.mjs`, `browser-wgpu-probe.mjs`, and `de` locale verification, fix any runtime/UI defects end to end, re-verify until 100% green.
