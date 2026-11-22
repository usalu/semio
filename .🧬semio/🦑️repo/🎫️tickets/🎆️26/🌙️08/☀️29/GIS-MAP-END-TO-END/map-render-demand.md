# GIS Map Frame Scheduling

The live Chromium Pixel 7 Canvas2D trace attributed three long pinch frames to the map's animation-frame callback. Each callback took 31–61 ms. The Rust map applies pointer motion, pinch and wheel changes synchronously. The React scheduler still drew on every frame while a pointer gesture was held and for 250 ms after any invalidation.

The map now schedules one paint for each batch of camera, tile, appearance, or interaction changes. Middle-button and single-touch pan request a paint on pointer motion. Pinch, wheel and keyboard zoom use their existing local camera invalidation. A level-of-detail update also invalidates immediately so it can repaint when the new view is already cached. Camera commands still publish through the existing gesture and action paths.

Ten assertions failed on the previous implementation. An isolated wheel or keyboard zoom produced 15 paints after the input. An unmoving pinch or pan could produce another 63 paints in a synthetic second. Two same-frame appearance updates also produced 15 paints, while a level-of-detail update without new tiles produced zero. The language-neutral repaint fixture specifies one paint per batch, and RxJS `auditTime` independently yields that count. All 59 focused tests passed after the change, including touch, desktop gestures, tile refresh and GPU recovery.

| Live profile after the change | First stable frame | Pan frame P95 | Zoom frame P95 | Zoom repaint after release | Tiles delivered |
| --- | ---: | ---: | ---: | ---: | ---: |
| Desktop Chrome WebGPU | 12.951 s | 17 ms | 50 ms | 226 ms | 49 |
| Chrome Pixel 7 emulation, WebGPU | 7.409 s | 17 ms | 50 ms | 1.052 s | 180 |
| Chrome Pixel 7 emulation, Canvas2D with frame timing | 13.187 s | 17 ms | 33 ms | 452 ms | 132 |
| Desktop WebKit WebGPU | 23.740 s | 77 ms | 82 ms | 347 ms | 49 |
| Desktop Firefox automatic Canvas2D fallback | 56.813 s | 151 ms | 449 ms | 569 ms | 49 |

The instrumented Canvas2D run measured 13 pan draws taking 92 ms total and 25 pinch draws taking 582 ms total. The longest individual pinch draw took 49 ms. The aggregate visibility check time rounded to 0 ms for both gestures. A preceding run after the same change observed a 50 ms pinch frame P95 and 1.518 s from release to changed pixels. These runs varied in compiler load and tile delivery and establish the repaint behavior without isolating a percentage change in live latency.

The Firefox zoom P95 came from 11 frame gaps. The shell became ready at 51.355 s, a stable map appeared at 56.813 s, and browser WebGPU was exposed while the map selected Canvas2D. The WebKit iPhone 15 and iPad Mini emulations delivered 65 tile paths each and passed trusted taps and keyboard navigation at 12.516 s and 17.756 s respectively. The WebKit desktop and iPad probes ignored two and one Vite reload messages, respectively. No ignored update or console error was reported in the Firefox pass.

The diagnostic `📜️devices.mjs --trace-frames` measures the exact served React callback and reports long frames when the browser exposes them. The regular smoke run exercises the app without the timing hooks.
