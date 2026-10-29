# Staffage web component, clock, custom element and browser exports

Owner: component worker. Root `S/🎯️targets/⚛️react/🔨️modules/`; public entry `@teaching/staffage-react` (`📦️packages/🟦️typescript/🟦️.tsx`).

## Modules

| Directory | Content |
|---|---|
| `🧮️pipeline` | the one shared, memoised evaluator: `evaluateState`, `thermalOf`, `structureOf`, `sceneOf`, `frameOf`, `allViews`, `inlineSvg`, `defaultQuery`, `queryAt`. Keyed by asset identity, query, environment. Analyses are solved only for views that read them; `frame.thermal` / `frame.structure` are lazy getters. |
| `⏱️clock` | `StaffageClock` (core in `🧱️core`, no React) and `useClock(clock)`. |
| `🧍️figure` | `StaffageFigure`, `StaffageValueTable`. |
| `🧩️element` | `defineStaffageElement(catalog, name?)`, `<staffage-figure>`. |
| `📤️downloads` | `exportFigure`, `exportCapabilities`, `exportFilename`, `saveExport`; `🗜️zip` (stored ZIP, PNG tEXt, CRC-32), `🖼️raster` (canvas rasteriser, MediaRecorder). |
| `🗣️labels` | bilingual view names and the spoken description (`describeScene`). |

## StaffageFigure

Props exactly as specified. Semantics:

- `fit="paper"`: element size is `scene mm x zoom` in CSS `mm`, so zoom 1 is print size on screen. `fit="contain"`: `width/height: 100%`, `aspect-ratio` of the scene, SVG `preserveAspectRatio="xMidYMid meet"`; the scene is untouched.
- No `query`: default pose, or the default clip at the clock time when a `clock` is given. Clip and transition queries follow the clock; poses do not. The clock time is quantised to frames (`frame / fps`), so figures on one clock are frame-identical.
- `live=false` (default): an `IntersectionObserver` freezes the figure while offscreen (no re-evaluation per clock tick). `live` keeps it running. Without `IntersectionObserver` (jsdom) the figure is treated as visible.
- Accessibility: `role="img"`, `tabIndex=0`, `aria-label` from `describeScene` (asset, pose/clip/time, view, scale in `language`), `lang`, `aria-describedby` the visually hidden `<StaffageValueTable>`; arrow keys step through valued items and call `onInspect`, Escape clears; a polite `role="status"` region announces the inspected value. Pointer hover calls `onInspect` (items are matched by `data-id` / `id` of the SVG element against `ItemValue` carrying path ids).
- `theme` ("light" | "dark") sets ink and paper of analytic views; `frame` (SceneFrame, mm relative to `metadata.origin`) pads the viewBox so several views share one window (`frameOf(scenes)` = union frame; `figureFrame(asset, query, request)` = one evaluated frame).
- `onEvaluated` fires once per distinct frame (`state`, `thermal`, `structure`, `scene`).

```tsx
import { StaffageClock, StaffageFigure } from "@teaching/staffage-react";

const clock = new StaffageClock({ fps: 30, durationS: 1 });
<StaffageFigure asset={asset} clock={clock} query={{ kind: "clip", clip: "walk", timeS: 0 }} view="thermal"
  scale={{ denominator: 100 }} language="en" thermalQuantity="surface-temperature" onInspect={(value) => show(value)} />
```

## StaffageClock

`new StaffageClock({ fps = 30, durationS = null, loop = true, rate = 1, playing, scheduler, reducedMotion })`. `play() pause() toggle() setRate() setLoop() setDuration() seek(t) step(frames) advance(dtS) dispose() subscribe(fn) state`. Elapsed time is accumulated from scheduler timestamps times `rate`; `seek`/`step` wrap (loop) or clamp; `step` lands on exact frames `n / fps`. `prefers-reduced-motion` starts it paused. A `scheduler` can be injected for deterministic tests; the default is `requestAnimationFrame`.

## Custom element

```html
<staffage-figure asset="reference-walker" view="dark-line" scale="100" lang="en" pose="stand"></staffage-figure>
<script type="module">
  import { defineStaffageElement } from "@teaching/staffage-react";
  defineStaffageElement(assets);
  document.querySelector("staffage-figure").addEventListener("staffage-evaluated", (e) => console.log(e.detail.scene.widthMm));
</script>
```

Attributes: `asset view scale lang pose | clip time playing rate loop zoom fit viewpoint transparent patterns analytic thermal-quantity force-quantity live no-dimensions no-scale-bar no-legend no-labels no-ground`. `lang` is read from the element or the nearest ancestor `[lang]`; without a language nothing renders (no default language). `.state` is `{ asset, query, frame, language } | null`; `.clock` accepts a shared `StaffageClock`. Events: `staffage-evaluated` (detail `state, scene, query`), `staffage-inspect` (detail `value, item`). Renders into an open shadow root so the SVG pattern ids never collide; it imports no React code (clock core, pipeline and labels are React-free).

## Exports

`exportFigure(asset, query, options, format, control?, host?) => Promise<ExportResult>` with `ExportResult = done | unavailable | cancelled`. `options` is the depiction request (`view, scale, language, ...`) plus `sheet: "single" | "all-views"`, `fps`, `durationS`, `pixelsPerMm`. Filenames: `<asset>_<pose | clip-tX>_<view>_1-<denominator>.<ext>` (sequence/video: clip id; all-views sheet: view `all-views`).

| Format | Implementation | Metadata |
|---|---|---|
| svg | `writeSvg` + `<metadata>` with title, software, description, JSON comment (when the writer has none) | yes |
| pdf | `writePdf`; `sheet: "all-views"` = 7 pages in view order | Info dictionary (writer) |
| dxf | `modelScene` + `writeDxf`; analysis and diagram kinds follow the view | `999` comment lines before the header |
| dwg | library `writeDwg` (Rust to WASM) when the build exports it; otherwise `{ status: "unavailable", reason: { en, de } }`; the capability probe encodes an empty drawing and checks the `AC10` magic | file name only |
| png-sequence | per frame `evaluate` (memoised) -> `depict` -> `writeSvg` -> canvas -> PNG (alpha kept when `transparent`), first-party stored ZIP | PNG `tEXt` chunks, `metadata.json`, ZIP comment |
| mp4 / webm | frames pre-rendered, then replayed in real time into `MediaRecorder` (`canvas.captureStream(0)`); the reported `details.codec` is the container the browser actually chose; `details.alpha` is always `false` (transparent frames are flattened on white and the result says so) | none (container limit), stated in `details.notes` |

`exportCapabilities(host?)` probes: canvas rasterisation, `MediaRecorder.isTypeSupported` per container, DWG writer. The UI shows `reason` for unavailable formats. Progress is `control.progress(done, total, {en, de})`; cancellation is `control.signal` (result `cancelled`).

## Capability notes and open points

- MP4: no first-party browser encoder is bundled; MP4 works only where `MediaRecorder` supports `video/mp4` (Chromium 126+, Safari). Elsewhere it reports unavailable. A first-party encoder from `🎥️video` can be plugged in through `ExportHost.record`.
- WebM alpha is not preserved by `MediaRecorder` canvas capture; the action says so instead of claiming it.
- Video frames are paced in real time (a 4 s clip takes about 4 s to record).
- `ExportHost` (`rasterise`, `record`, `dwg`) lets tests, workers or native shells replace browser capabilities.
