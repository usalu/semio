# Desktop Input Audit

The device acceptance script samples 121 fixed canvas points, quantizes each RGB channel by 8, and requires more than 12 sampled colors to change. A real pan on low-contrast or uniform tiles can be missed, while delayed tile delivery can produce a change without camera movement. Initial settlement requires two equal sampled frames and 200 ms without tracked tile completions; the first post-pan comparison does not require tile settlement. Report full-pixel difference and sampled change count together with session camera and tile request state to distinguish these possibilities.

The selector targets the sole canvas under the tiled map host. The canvas fills its host. The normal hover popup has pointer-events disabled; the renderer-unavailable overlay intercepts input only on failure. Selection marquee and open context menu are conditional. Verify document.elementFromPoint at the drag start and pointer target/capture to rule out outside overlays. The wheel listener is attached to the container; keyboard handling is attached to the focusable canvas and requires rendererReady. The same host and input handlers serve WebGPU and Canvas2D.

The desktop script drags with the middle mouse button and then checks wheel and ArrowRight only after pan succeeds. Chromium touch uses trusted CDP gestures; WebKit mobile profiles check a trusted tap only, not map movement.

Sources: 📜️devices.mjs lines 27–48, 62–98, 200–263; TiledMapHost/🟦️.tsx lines 32–45; TiledMapHost/🎞️map-render-effects/🟦️.ts lines 285–332.
