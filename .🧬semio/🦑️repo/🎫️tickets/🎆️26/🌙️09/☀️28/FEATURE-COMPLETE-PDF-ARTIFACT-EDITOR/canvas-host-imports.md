# React Canvas Host Imports

The PDF editor's mounted Vite run failed during import analysis. Paint2dHost referenced a missing Canvas2dHost backing-store module, while the resize implementation still lived in NodeGraph label overlays. The React renderer's scene and world host entry also imported the camera debounce constant through an alias that Vite resolved to the Canvas2dHost component module.

The resize implementation now lives in the Canvas2dHost backing-store module. Paint2dHost, NodeGraph's flow surface, and its label overlays resolve that one implementation. The React renderer and NodeGraph entry points still export it. The React renderer imports the camera debounce constant from its defining module and still exports it.

## Verification

- The mounted Vite rerun cleared the missing import and proceeded to dependency optimization. The PDF editor mount is being checked separately in the mounted browser session.
- The device-pixel backing-store assertions from the existing hidden-tab-surfaces test passed in isolation against a jsdom canvas using the relocated implementation. They cover the browser default, DPR scaling, idempotence, clamping to 1×1, and absent canvases.
- The full hidden-tab-surfaces file was also attempted through the React Vitest config. Collection stopped at the missing generated Flow binding, `🎁️browser-package/🕸️bindings/flow_core.js`, before running assertions.
- The repository diff whitespace check passed for the renderer engine.
