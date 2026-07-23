# Get Procedural 3D Working End to End

## Root causes

1. **Radix Slider React 19 composed-ref loop** (`@radix-ui/react-slider@1.3.6`): `useComposedRefs(forwardedRef, (node) => setThumb(node))` created a new ref identity every render → Maximum update depth → `ShellRenderErrorBoundary` ("Renderfehler"). Fixed by bumping to `1.4.5` (passes state setters directly) + root override.

2. **`useCanvasAppearanceSync` unstable `sync` dependency**: Flow/preview hosts pass inline `() => { paintOverlays(); … }` every render. Effect depended on `sync` → called `paintOverlays` → `setContainerSize`/`setSliderStateJson` → re-render → loop (console errors after slider fix). Fixed by holding `sync` in a ref; effect only depends on `enabled`.

## Verification

- Headless Playwright on `http://localhost:6018/` (`SEMIO_RENDERER=react`, `SKIP_PLUGIN_BUILD=1`).
- Before: body showed "Render error: Maximum update depth exceeded".
- After: Flow + Preview windows, 3 canvases sized, 3 sliders, slider drag ok, `depthErrorCount: 0`, screenshot `e2e-after-fix.png` shows hexagonal column preview + flow graph.

## Note

Restarting `dev:procedural:3d` without `SEMIO_RENDERER=react` defaults to wgpu. Launch entry `procedural3d-react-dev` sets react. Concurrent UI-presence migration may break plugin wasm rebuild; use `SKIP_PLUGIN_BUILD=1` when only verifying renderer JS.
