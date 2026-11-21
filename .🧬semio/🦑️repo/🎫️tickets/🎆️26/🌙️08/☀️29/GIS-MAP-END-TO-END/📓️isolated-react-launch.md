# Isolated React GIS2D launch check

## Retained component

Prototype run `wUqI5O6` retained a linked 138,530,740-byte GIS component at `.cache/b/r/m/components/runs/wUqI5O6/w/7dcd44ff430d28916e89e450/component.wasm`. Independent native transpilation produced the JS binding and 138,445,673-byte main core under `🗑️generated/native-transpile-proof`. The run ended before a complete component provenance receipt or browser module was produced. [Prototype build log analysis](./prototype-browser-transpiler.md)

The subsequent scratch descriptor probe used the real Node JSPI probe, production host shim and rewrites, and a copy of the canonical prototype Preview 2 vendor. It reached the shared guest registry and failed at `semio-plugin-decl-registry` `🌳️tree/🦀️.rs:134`, reporting `app s.gis.gisterrain@1/*#editor: fault notice gis-terrain-brush-settings-invalid: syntax`. Node 24.15.0 reported a WebAssembly `unreachable` trap. The semantic output was empty. This is a GIS registration failure observable before React starts. [Probe assembly and reproduction command](./📓️guest-descriptor-probe.md)

The GIS component descriptor includes the terrain editor as well as the map editor. `configure_terrain_brush::fault_notices()` contributes both `gis-terrain-brush-settings-invalid` and `gis-terrain-window-context-required`. The validator in `🧰️framework/🔨️modules/🎠️kernel/📜️history/🦀️.rs:658` requires at least three dot-separated kebab segments. The terrain window configuration also emits `gis-terrain-window-context-required` directly. A corrected build needs matching codes at the declarations and emission sites.

The production browser materializer calls the linked guest’s `describe()` before writing its descriptor, bridge and component receipt. It compares the returned bytes with the receipt’s semantic SHA-256. The retained artifact therefore cannot supply the verified descriptor for a provenance-preserving GIS2D browser run. Reusing a descriptor from another component would break that binding.

## Isolated React host

The real React Vite config accepts a catalog-provided browser-host override for development serving. The catalog resolver consumes `SEMIO_TEST_ARTIFACT_DIR`, `SEMIO_TEST_BROWSER_MODULE_ROOT`, `SEMIO_TEST_BROWSER_ACTIVATION_ROOT` and `SEMIO_TEST_BROWSER_HOST_RECEIPT` together. It validates the fixed `browser-host/{modules,activation,🔣️receipt.json}` layout, hashes the module and activation trees, verifies the real Space component’s owner receipt, then matches the GIS descriptor, bridge and generated core digests to one selected GIS composition. Both the host receipt and its activation are fixed to variant `s` and prototype. The stand-alone `gis2d` variant has no arbitrary scratch-root override in the authored catalog/Vite path.

The existing Hub test `trusted-stdio-gis-bundle-check --two-author-shell` provisions that supported isolated host through `withTestBrowserHostV1`, starts the ordinary React shell via its real Vite config on allocated loopback ports and drives it through the workspace Playwright dependency. The test consumes a verified Hub-selected GIS component and descriptor, materializes Space plus GIS, and uses the host receipt while the shell runs. This test compiles supporting Hub and component inputs and publishes a test-owned diagnostics artifact. It cannot act as a small reader of the retained raw GIS Wasm or the standalone transpiler output.

## Browser verification after staging

For the managed `gis2d` server on 6041, the existing device harness requires `data-shell-ready="gis2d"` and exercises rendered pixels, tile delivery and map interactions. After the selected component stages successfully and the server is ready, the pinned Node tool can drive Playwright from the repository root. Run the following with each engine in turn to avoid contending for memory. The full matrix runs when the engine argument is omitted.

```zsh
gis_ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END'
repo tools exec node -- "$gis_ticket/📜️devices.mjs" http://127.0.0.1:6041/ --browser=chromium
```

No browser, compiler or publication was started for this investigation. The isolated probe result establishes the failure of the retained bytes. Browser input and rendering for the current managed build remain unverified.

## Trusted mobile WebKit input

The workspace lock and installed Playwright resolve version 1.64.1, with WebKit revision 2370. `playwright-core/types/types.d.ts` exposes only `Touchscreen.tap(x, y)`. The installed WebKit `RawTouchscreenImpl` sends `Input.dispatchTapEvent` to its page-proxy session. The public `newCDPSession` dispatcher rejects browser types other than Chromium. WebKit support for `connectOverCDP` in this pin attaches a browser and leaves that session restriction in place. Playwright's WebKit mouse implementation also rejects `wheel` in mobile contexts.

The iPhone 15 and iPad Mini rows in `📜️devices.mjs` therefore exercise a trusted single-finger tap and assert `pointerType === "touch"` and `isTrusted === true` on the canvas. Pixel 7 and Galaxy Tab S9 use Chromium CDP `Input.dispatchTouchEvent` for movement and multi-touch. JavaScript-dispatched DOM touch or pointer sequences are synthetic coverage. Trusted mobile WebKit pan and pinch require a real Safari device or simulator input path outside this harness.

The mounted React tiled-map suite covers touch-pan threshold, pinch handoff, terminal samples, pointer capture loss and mouse parity with neutral fixtures and Three.js camera math. The shared gesture fixture covers contact latching and pinch geometry. These tests remain the interaction fallback for the iPhone and iPad rows until trusted Apple-device gestures can be measured. [React map touch verification](./📓️react-map-touch.md)
