# Actual Three and WGPU Shading Pixels

## Reference Receipt

The ticket `📜️script.ts shading-oracle` runner rendered the shared, schema-validated World3d scene-shading fixture through installed Three r182 in Chromium using ANGLE Metal on Apple M1 Max. The fourth run exited successfully and reproduced all 17 recorded RGBA8 rows byte for byte. The profile explicitly sets a 64 × 64 viewport, DPR 1, sample position (32, 32), transparent black clear color, no antialiasing, and `premultipliedAlpha: false`.

The fixture covers light/dark semantic neutral colors, authored/environment colors, six static style cases, indirect Lambert, two direct-material cases, three ACES inputs, and sun lighting. Actual WebGL `readPixels` values are persisted in the language-neutral fixture. The runner validates its schema with installed Ajv and rejects divergence from the recorded rows. It uses installed Three materials, lights, tone mapping, and color conversion; it does not compute substitute reference pixels.

The production fixture and schema are under `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading` and `🧬️schema/🎨️scene-shading`. Temporary browser bundles and raw readbacks remain under this ticket's `🗑️generated/astra-runtime/three-shading-oracle-4` until ticket cleanup. The reusable test input remains in this ticket's `📜️script.ts`.

## Scope and Pending Comparison

This reference receipt does not establish WGPU visual parity. A controlled WebGPU readback is being added using the actual production `WORLD3D_SHADER`, vertex and instance layouts, uniform packing, and opaque/translucent blending configuration. Full renderer/browser integration remains a separate gate.

One unresolved hypothesis is transparent color blending: Three's encoded default framebuffer output and WGPU's linear blending into an sRGB attachment may differ even when opaque BRDF and tone mapping match. Actual pixel comparisons must establish the behavior before changing production color handling. Shared UI compositing must not acquire World3d tone mapping.

## First Production WGSL WebGPU Receipt

The actual production WORLD3D_SHADER was compiled and rendered by Chromium WebGPU on the Apple Metal adapter, then copied to a mapped GPU readback buffer. Shader SHA256: `2a45089bceb4be7ea6e3f9b67536d4a52e59707474055d2ba4c0151322871663`. The harness uses the actual 40-byte vertex, 96-byte instance, 240-byte globals, sRGB attachment, opaque replacement, and translucent alpha blend configuration. Camera projection uses installed Three with WebGPU clip coordinates; the sample row is flipped to match WebGL framebuffer coordinates. This is a controlled shader/pipeline test, not evidence that the full application renderer is correct.

Thirteen opaque cases agree within one 8-bit color value: twelve are byte-exact and mixed-metal differs by +1 blue. Four transparent cases fail despite exact alpha. This confirms a color-blending discrepancy that must be repaired without changing the shared UI output transform.

| Case | WGPU RGBA8 | Three RGBA8 | WGPU Minus Three |
| --- | --- | --- | --- |
| semantic-light | 215, 215, 210, 255 | 215, 215, 210, 255 | 0, 0, 0, 0 |
| semantic-dark | 23, 37, 42, 255 | 23, 37, 42, 255 | 0, 0, 0, 0 |
| authored-light | 163, 189, 202, 204 | 144, 167, 179, 204 | 19, 22, 23, 0 |
| environment-dark | 193, 126, 76, 166 | 152, 100, 61, 166 | 41, 26, 15, 0 |
| neutral-vertex | 180, 208, 223, 255 | 180, 208, 223, 255 | 0, 0, 0, 0 |
| disabled-vertex | 125, 145, 156, 115 | 81, 94, 100, 115 | 44, 51, 56, 0 |
| selected-solid | 255, 110, 119, 255 | 255, 110, 119, 255 | 0, 0, 0, 0 |
| hovered-solid | 157, 165, 160, 255 | 157, 165, 160, 255 | 0, 0, 0, 0 |
| highlighted-solid | 155, 225, 216, 255 | 155, 225, 216, 255 | 0, 0, 0, 0 |
| provisional-solid | 125, 182, 175, 158 | 96, 139, 134, 158 | 29, 43, 41, 0 |
| indirect-lambert | 77, 118, 145, 255 | 77, 118, 145, 255 | 0, 0, 0, 0 |
| rough-dielectric | 128, 167, 191, 255 | 128, 167, 191, 255 | 0, 0, 0, 0 |
| mixed-metal | 203, 91, 34, 255 | 203, 91, 33, 255 | 0, 0, 1, 0 |
| aces-in-range | 164, 197, 215, 255 | 164, 197, 215, 255 | 0, 0, 0, 0 |
| aces-hdr-white | 250, 250, 250, 255 | 250, 250, 250, 255 | 0, 0, 0, 0 |
| aces-hdr-colored | 255, 240, 202, 255 | 255, 240, 202, 255 | 0, 0, 0, 0 |
| sun-neutral-light | 208, 197, 173, 255 | 208, 197, 173, 255 | 0, 0, 0, 0 |

The test command exited nonzero because those transparent differences exceed the current one-byte test threshold. The threshold accounts for GPU floating point/quantization variation; it does not accept the transparent discrepancy. A general repair must preserve encoded scene blending over nonzero backgrounds and other geometry, not only make the transparent-black fixture pass. Raw artifacts remain under `🗑️generated/astra-runtime/wgpu-shading-oracle-1`.

## Native Gate

UI16 compiled the current changes, then stopped after 251 passing tests and one exact memory-accounting failure: UiSurfaceRegistry slot size measured 161904 bytes versus committed 161896 bytes, with capacity 64 and owner size 520 unchanged. The remaining 359 tests did not run. The owner of the new state must account for the measured eight bytes before a full no-fail-fast rerun. No compiler or full-suite success is claimed.

The shader/render native suite then passed all 136 tests with zero skips, including the current production/canonical shader validation. The Three reference also remained byte-exact after the harness moved to an isolated secure browser origin. The temporary Tree flow field responsible for the eight-byte UI size change was removed; UI17 is rerunning the full census without fail-fast.

## Encoded World Attachment Repair — 20 Exact Pixels

Three7 reproduced the expanded 20-row fixture byte-exact. Three new cases cover disabled vertex color over opaque light panel, provisional color over opaque dark panel, and ordered authored/provisional transparent overlap. Before repair, the dark case differed by 19/27/25 RGB values, confirming the defect on a nonzero destination.

WebGPU3 exited successfully: all 20 RGBA8 rows match actual Three byte for byte. Production shader SHA256 is `2c9da2f75944c09b23f99eef8572ed2a2cb782228d0fdfd1d29df1187162ab9f`, verified unchanged after the run and containing the exact installed Three output-transfer coefficients. The controlled target now uses compatible sRGB and UNORM views of the same eight-bit texture: background clears through the sRGB view; World shaders encode their ACES output and blend through the UNORM view; subsequent World layers preserve destination contents. The production packet uses the same separation for scene and foreground World mesh/line/textured draws while shared UI remains on the sRGB view.

This establishes the bounded pixel fixture, not full runtime or all backend parity. Standalone WebGPU, D3D12, and Metal retain their existing linear scene-target contract and require separate transparent-compositing review. Animated celebrated materials, GLB material provenance, textured lighting, camera framing, and live application output remain open.

World11 initially failed compilation at a fallible mesh-schema field access and malformed fixture literal; both were repaired. World12 then ran 226 selected tests: 225 passed and one failed at exact float equality (0.35999998 versus 0.36) in the new provenance/style law, with 243 unrelated tests filtered by the target. UI17 ran all 611 tests: 610 passed and the same eight-byte exact budget assertion failed; its owner subsequently removed the separate layout-change bool and used an existing flag. UI18 and canonical activation12 are now validating the final source boundary.

## Permanent Validation Command

The oracle now lives beside the existing scene-shading tests in the engine's `🧪️tests/🎨️world3d-scene-shading/📜️script.ts`. The React renderer package router registers `scene-shading-pixel-check`, and `.vscode/launch.json` exposes the matching `📦️test🧊️⚛️world3d🎨️pixels` command in the existing build/test order. Its Nx target acquires Chromium through the existing `workspace:deps-browsers` dependency and runs uncached because the result depends on a live GPU. It uses Metal ANGLE only on macOS and platform defaults elsewhere; other OS runs are not claimed here.

`bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache` passed: both actual producers rendered all 20 cases, and every RGBA8 channel was byte-exact on this Mac. The schema explicitly permits a maximum one-byte RGB hardware variation for WGPU; alpha and the installed Three reference remain exact. Case identity/order is checked, so extra/missing reference rows cannot silently pass. The successful invocation took 7.7 seconds after project-graph discovery. The first invocation used the deliberately reused old Nx graph and failed target discovery before running; removing graph reuse discovered the registered task without cache edits.

Artifacts are optional and go under `SEMIO_TEST_ARTIFACT_DIR` when supplied. This ticket's receipt is `🗑️generated/astra-runtime/pixel-gate-permanent-2/world3d-scene-shading`. The retained ticket `📜️script.ts` now calls the shared test implementation rather than duplicating it. No runtime dependency was added.
