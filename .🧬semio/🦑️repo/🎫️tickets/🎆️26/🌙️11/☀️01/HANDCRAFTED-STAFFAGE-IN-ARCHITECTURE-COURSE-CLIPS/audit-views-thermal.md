# Staffage depiction audit: views, options, thermal, transport, export

Read-only static audit of the staffage library. Nothing was executed and no file was changed.

Notation: `S` = `C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage`, `M` = `S/🔨️modules`. Line numbers refer to the files as present in the working tree on 2026-11-01. The export cast module `M/📤️export/🎭️cast/🟦️.ts` is untracked in git (`??` in the session status), so it is work in progress.

## 0. Bottom line

- Figure-only frames for a dark background are available today through the cast exporter (`M/📤️export/🎭️cast/🟦️.ts`, `exportCast`), with the caveat that it cannot set a theme, a transport overlay, or a per-frame environment outside the thermal view. It emits SVG only.
- Animated thermal colours from a clip are possible: `solveThermalAt` integrates the thermal response over the clip's own inputs, so each frame's colours depend on the time history. Cast and direct `depict` calls do this. The CLI does too, but it always uses the asset's default environment and annotations on.
- Respiration plumes are a separate transport overlay (`DepictionOptions.transport`), not part of the thermal view. Neither the cast schema nor the CLI can enable it.
- The CLI (`🎬️cli`) is not usable for figure-only video: it turns every annotation on, uses a light thermal theme, and has no environment, transport or theme flags. Its rasterised frames are viewBox-stable through a fixed canvas, but its SVG and PDF output is not.
- Two gaps for dark video: (a) cast thermal and forces/moments frames are drawn with the light theme ink, so outlines vanish on `#001117`; (b) transport labels and emitter markers ignore `annotations.labels`, so they leak into "figure-only" output built from `depict`.

## 1. The seven views

Source: `S/M/🔗️contracts/🟦️.ts:217` (`VIEWS`), dispatch in `S/M/🎨️depiction/🟦️.ts:122-128`.

Theme rule (`S/M/🎨️depiction/🟦️.ts:69-71`): `dark-*` views always use the dark theme, `light-*` always the light theme, and `thermal`, `forces`, `moments` use `options.theme`, defaulting to light.

| View | What it draws | Palette and paint |
|---|---|---|
| `dark-line` | Line drawing of every shape. Filled closed shapes are painted in the background colour to hide what lies behind them. Role-weighted outlines (body, detail, texture, glazing, ground) are drawn with hidden lines removed. Hatches are drawn too. | Background `#001117`, ink `#f7f3e3` (`S/M/🖍️palette/🟦️.ts:22`). Hatch and outline ink is the same colour. Line views have no tone fills (`baseStyle`, `S/M/🎨️depiction/🟦️.ts:73-81`). |
| `dark-solid` | Tone fills per shape (greyscale ramp), role outlines of open strokes only, hatches at 0.55 opacity. Glazing at 0.45 opacity. | Tone ramp `#2e3c3d` (tone 0, darkest) to `#f1edde` (tone 1) via `toneColour` (`palette:50`). Filled-shape outline is `weightMm × 0.5` in ink, but only when `scale.denominator <= 100` (`🟦️.ts:79`). |
| `light-line` | As `dark-line` on paper. | Background `#f7f3e3`, ink `#001117`. |
| `light-solid` | As `dark-solid` on paper. | Ramp `#243235` to `#d3d2c5` (`palette:23`). |
| `thermal` | Each region coloured by a physical quantity on a diverging ramp (`thermal-view/🟦️.ts:40-119`). Open strokes get a halo of the region colour (`thermal-view:92-101`). Legend with unit, range, sign convention, conditions and assumptions (`thermal-view:104-117`). Hatches are not drawn (`shapes:168`). | Warm = `#fdae61` to `#d7301f`, cool = `#abd9e9` to `#2c7bb6`, neutral `#d9d9d9` (`S/M/🏗️structure/🟦️.ts:36`). Dark theme lifts the strong ends 25 % toward `#f7f3e3` (`palette:72`). Unregioned shapes use the tone ramp. |
| `forces` | Members coloured by axial, shear, stress or contact pressure (`structural-view:61-101`). Load arrows (applied red, reaction teal, weight grey, inertia orange), support symbols, centre of mass, optional axial and shear diagrams. Optional free-body inset (`structural-view:309-367`). | Overlay colours (`palette:52-55`): dark applied `#ff6177`, reaction `#34d1bf`, weight `#a2a59d`, joint `#fa9500`, vapour `#5fe0cf`. Light applied is `mix(#ff344f, #001117, 0.18)`. Arrow length scales with force, 1.5 to 14 mm (`structural-view:21-22`). |
| `moments` | Joint, reaction and applied moments as arcs with radius `1.6 + 3.2·√(|M|/max)` mm, arcs below 2 % of max omitted (`structural-view:382-414`, constants `:21-25`). Bending-moment diagram on members. | Same overlay palette; moment diagram uses `divergingColour` with the structural ramp. |

Ground line and dimension line are added by `depict` in all views (`🟦️.ts:118-120`, `:137`).

## 2. `defaultDepictionOptions` and `depict` options

Definition: `S/M/🎨️depiction/🟦️.ts:45-64`. Type: `S/M/🔗️contracts/🟦️.ts:237-256`.

Defaults from `defaultDepictionOptions(view, language, denominator = 100, viewpoint = "elevation", theme?)`:

| Field | Default | Meaning |
|---|---|---|
| `language` | none, must be given | `en` or `de`, no default language. |
| `scale` | `{denominator}` | `1:denominator`. `mmPerMetre = 1000 / denominator` (`S/M/📐️units/🟦️.ts:35-37`). |
| `representation` | `{kind: "scale"}` | The authored representation at that scale (pens, hatches). Resolved by `resolveRepresentation`, throws `RepresentationError` if unavailable (`🟦️.ts:94-97`). |
| `profile` | `DEFAULT_PRINT_PROFILE` | Print-profile checks (`representation/🧾️profiles`). |
| `transparent` | `false` | `false` paints a background rect in the theme colour. `true` omits it. |
| `marginMm` | `5` | Paper margin. |
| `annotations.dimensions` | `true` | Overall-height dimension line (`dimensionLine`). |
| `annotations.scaleBar` | `true` | Scale bar (`scaleBar`). |
| `annotations.legend` | `true` | Legend blocks. |
| `annotations.labels` | `true` | Title line and value labels (see gating below). |
| `annotations.ground` | `true` | Ground line. |
| `thermalQuantity` | `surface-temperature` | One of 7 `THERMAL_QUANTITIES` (`contracts:96`). |
| `transport` | `[]` | Transport overlay layers (see section 4). |
| `forceQuantity` | `axial` | `axial`, `shear`, `stress`, `contact-pressure`. |
| `diagrams` | axial false, shear false, moment true, freeBody true | Member diagrams and free-body inset. |
| `range` | `{mode: "shared"}` | Colour range: `shared`, `auto`, `fixed`. |
| `patterns` | `false` | Colour-independent hatch fills for analytic views (non-colour cue). |
| `theme` | absent (light for analytic views) | `"light"` or `"dark"`; only read by `thermal`, `forces`, `moments`. |
| `environment` / `environmentTimeline` | absent | Thermal environment; absent means `asset.thermal.environment`. |
| `viewpoint` | `elevation` | `elevation` or `plan`. Ground line only for elevation (`:118`). |

Gating in `depict` (`S/M/🎨️depiction/🟦️.ts:137-151`):
- Ground line: `annotations.ground && viewpoint === "elevation" && metric` (`:118`).
- Dimension line: `annotations.dimensions && metric` (`:137`). `metric` is false only for paper-unit symbols.
- Title block: `annotations.labels` (`:146`). Scale bar: `annotations.scaleBar` (`:147`). Legends: `annotations.legend` (`:148`).
- Value labels: `ctx.labels.flush` runs unconditionally (`:140`), but labels are only queued when `annotations.labels` is true (`structural-view:274`, `:329-330`, `:378`). So value labels are gated correctly.
- Free-body inset: gated only by `diagrams.freeBody` (`structural-view:367`). Its title "Free-body diagram" and residual text are not gated by `annotations.labels`.
- Legend data: `DrawingScene.legends` is always filled (`🟦️.ts:141`, `:157`). Suppression happens only in drawing, so the SVG metadata still carries the legend (`svg/🟦️.ts:75-77`, harmless).

Figure-only recipe (what the cast exporter does, `🎭️cast/🟦️.ts:74-75`):
- `transparent: true`, `marginMm: 1`
- `annotations: {dimensions: false, scaleBar: false, legend: false, labels: false, ground: false}`
- `diagrams.freeBody: false`

Remaining drawn content is the figure itself, plus any force arrows and moment diagrams from the `forces` and `moments` views, because `diagrams.axial/shear/moment` are not part of the cast's figure-only switch set.

Gaps for figure-only output:
- Transport labels are not gated. `transport-view/🟦️.ts:86-93` draws emitter arrows and `emitter-label:*` text unconditionally, and `:94-108` draws `label:streamlines`, `label:pathlines`, `label:condensation` unconditionally, all in the `annotations` layer. Figure-only transport output needs either a code change or a post-filter that drops the `annotations` layer from `DrawingScene.layers` before `writeSvg`.
- Free-body inset ignores `annotations.labels`. Set `diagrams.freeBody: false` as the cast does.

Scale and line weights:
- Pens come only from the authored representation at the chosen scale (`shapes/🟦️.ts:70-73`, `:35-40`). No global pen table, no LOD cut (`shapes:5`). Example from `catalog/👤️humans/cyclist/🔣️.json` (pens in mm):
  - 1:50: body 0.35, detail 0.25, texture 0.10, glazing 0.13, ground 0.35
  - 1:100: body 0.25, detail 0.15, texture 0.10, ground 0.25 (no glazing pen)
  - 1:200: body 0.18, detail 0.13, texture 0.10 (no hatches)
  - 1:500: body 0.13, detail none, texture none
- Annotation text 2.5 mm (`units:21`), label text 1.8 mm (`units:27`), annotation pen 0.13 mm (`units:24`), leaders 0.1 mm.
- Scale-bar lengths at presets 500, 200, 100, 50 m: 5, 2, 1, 1 m (`annotations:18`). Other denominators pick the 1-2-5 length nearest 12 mm of paper.
- Label budget: 8 at 1:50, 6 at 1:100, 4 at 1:200, 3 at 1:500 and smaller (`canvas:102-105`).
- Output size: a 1.80 m figure is 36 mm tall at 1:50 and 3.6 mm at 1:500 (`units:1-5`). Video size therefore comes from the rasterisation dpi, not from the scale.

Theme and background pitfalls:
- `thermal`, `forces`, `moments` with the default theme use light ink `#001117`. On a `#001117` background their outlines vanish. Fills still show. Fix: pass `theme: "dark"` to `depict` (the cast manifest has no such key, see section 6).
- `dark-line` and `dark-solid` are the only graphic views with dark ink and dark paper. Use them for dark video without any theme option.

## 3. Thermal

### 3.1 `solveThermal` vs `solveThermalAt`

`S/M/🌡️thermal/🟦️.ts:122-172`, `S/M/🌡️thermal/🕰️timeline/🟦️.ts:95-108`.

- `solveThermal(state, environment?, options)` (`thermal/🟦️.ts:125`) reads only `state.thermal`, `state.timeS`, the figure's centre-of-mass velocity and the asset. It does not evaluate other times.
  - Without `options.history` and with a constant environment, the response is steady state of the current inputs (`:136`, `:171`; note `:276` in `assemble`).
  - It becomes transient if `options.history` is given or the environment is a timeline (`:136`). The history integrates from `t0 = max(0, t − window)` in the steady state of the inputs at `t0` (`⏳️transient:50-53`).
- `solveThermalAt(asset, query, environment?, options)` (`🕰️timeline:95-108`) evaluates the query at every integration time. For a clip or transition it supplies `history` from those evaluations, so the response follows the clip's own inputs (pose binding, tracks, thermal bindings). For a pose it is steady unless the environment is a timeline (a pose holds its inputs; `:93`).
- Both are deterministic in (asset, query, environment) and independent of drawing scale (`🕰️timeline:94`, `thermal/🟦️.ts:11`).

Which one `depict` uses (`S/M/🎨️depiction/🟦️.ts:123`): `analysis.thermal ?? solveThermalAt(asset, query, options.environmentTimeline ?? options.environment)`. So plain `depict` on a clip state is already transient.

Transient mechanics (`S/M/🌡️thermal/⏳️transient/🟦️.ts`):
- Exact first-order update `T(t+Δt) = T* + (T − T*) e^(−Δt/τ)` (`relax`, `:59`), with `τ = C/G` per region.
- Window `min(3 h, 7·τ_max)`; at most 180 steps; base step 0.5 s doubled until the step fits (`historyPlan`, `:64-78`). Grid aligned to absolute time, so scrubbing and playing agree (`:65-66`).
- Core temperature uses the body heat capacity (`thermal/🟦️.ts:142`, notes at `:273-275`).

### 3.2 Environment presets

File: `S/🗂️catalog/🌤️environments/🔣️.json` (`presets[]`, id, title, `environment`, `source`, `reference`). Loaded by `loadEnvironments` (`S/M/🗂️catalog/📥️files/🟦️.ts:47-48`).

| id | air °C | mean radiant °C | sky °C | ground °C | wind m/s | sun W/m² | RH |
|---|---|---|---|---|---|---|---|
| office-neutral | 22 | 22 | – | – | 0.1 | 0 | 0.50 |
| indoor-heated-winter | 21 | 19 | – | – | 0.1 | 0 | 0.35 |
| winter-overcast | 2 | 2 | −2 | 2 | 3 | 80 | 0.85 |
| winter-clear-night | −5 | −5 | −31 | −4 | 1 | 0 | 0.80 |
| spring-mild-sun | 15 | 15 | −3 | 18 | 2 | 600 | 0.55 |
| summer-hot-sun | 32 | 36 | 21 | 50 | 1.5 | 850 | 0.40 |
| summer-heatwave-calm | 38 | 44 | 26 | 58 | 0.3 | 900 | 0.30 |
| summer-humid | 30 | 32 | 19 | 38 | 0.5 | 500 | 0.80 |
| summer-night | 20 | 20 | 4 | 19 | 0.5 | 0 | 0.60 |
| windy-cool | 10 | 10 | 6 | 10 | 8 | 100 | 0.70 |

Presets are used only by cast manifests (`🎭️cast/🟦️.ts:50-58`) and tests. The CLI and `defaultDepictionOptions` never select one.

Timelines (`S/M/🌡️thermal/🌤️environment/🟦️.ts`):
- `EnvironmentTimeline = {keys: [{t, environment, ease: "step" | "linear"}]}` (`contracts:145`).
- Before the first key the first environment holds; after the last, the last (`:41-43`). `step` holds the key's environment until the next key (`:46`). `linear` blends every numeric field; wind direction blends along the shorter arc (`:19`, `:29`).
- A timeline makes a pose transient too (`thermal/🟦️.ts:136`).

Thermal colour legend and scale (`thermal-view:44-58`, `quantities:30-70`):
- Shared comparison ranges are fixed across all assets, scales and times: surface temperature −20…60 °C centred on air temperature; skin 10…40 °C centred on 33 °C; emitted radiant flux 230…700 W/m² centred on σT_air⁴; net radiative and sensible ±200 W/m²; latent ±300 W/m²; moisture ±450 g/(m²·h). Values beyond the range saturate the colour but are not clipped in the inspector (doc of `quantities:3-5`).
- The colour centre follows the environment at each instant: `centre = spec.centre(field.environment)` (`thermal-view:45`). With a timeline, the zero point of temperature colours moves per frame while the range stays fixed. This is intended but worth knowing when comparing frames.
- `auto` range is normalised per figure and must not be compared across frames (`thermal-view:106`).

### 3.3 Can a clip query produce frame-by-frame animated thermal colours?

Yes, in three ways, with different limits:
- Cast manifest (`🎭️cast/🟦️.ts`): `{asset, clip, view: "thermal", environment?, timeline?}`. `framesOf` gives one frame per clip sample at `fps` (default 15). `render` calls `solveThermalAt` per frame with the environment (`:82`). Figure-only and shared range by default. The timeline's `t` is on the clip's absolute timeline. Cast schema rule: environment and timeline are allowed only for `view: "thermal"` (`🧬️schema/🟦️.ts:65`).
- Programmatic `depict(evaluate(asset, {kind:"clip", clip, timeS}, selector), options)` per frame. Same physics, same history. Set `options.environment` or `environmentTimeline` as needed, plus `theme` for dark.
- CLI `runExport` (`🎬️cli/🟦️.ts:127-134`): `--view thermal --clip <slug>` animates, but `depict` gets no environment, so the asset's default environment is used, and the light theme applies.

Caveats:
- Frame 0 of a clip is steady state (`thermal/🟦️.ts:141`, `t > 0` condition). Later frames integrate from `t0 = max(0, t − window)`.
- Cost: each frame runs `solveThermalAt`, which evaluates up to 180 clip states (cache is per call only, `🕰️timeline:99-107`). Neither cast nor CLI shares thermal history between frames. Not measured in this audit.

## 4. Transport and respiration

Enabling: `DepictionOptions.transport` (`contracts:246`), a non-empty array of `TRANSPORT_LAYERS` (`contracts:178`): `parcels`, `pathlines`, `streamlines`, `condensation`. Any view can carry the overlay (`🟦️.ts:131-136`). Default `[]` (`🟦️.ts:61`).

Is respiration part of the thermal view? No.
- The thermal view colours regions only (`thermal-view:41-119`). Respiratory heat appears in `ThermalField.totals.emitterSensibleW/LatentW` and in body fields (`thermal/🟦️.ts:270`, `:262-263`), not in the drawing.
- Breath is an emitter (`kind: "breath"`) of the asset's `thermal.emitters`. Human assets carry `breath-nose`/`breath-mouth` plus a plume. Example: `catalog/👤️humans/cyclist/🔣️.json` has `breath-nose`, `breath-mouth`, `plume`.
- Its air is an airflow carrier with excess temperature and humidity (`sources:39`, `:149`: `carries = emitter.kind === "transpiration" ? "vapour" : "airflow"`). Exhaled volume is the integral of the half-sine flow, routed nose/mouth by the pattern (`sources:83`, `respiration:1-21`).
- Vapour carriers come only from transpiration emitters (`sources:6`, `:149`). Those are plants (e.g. `catalog/🌳️vegetation/agave`). So "vapour" plumes appear around vegetation, not people. Human breath can condense (`condensation` layer, `transport/🟦️.ts:102`), but it is drawn as a condensation disc, not a vapour colour.

Transport field (`transport/🟦️.ts:79-122`): `solveTransport(asset, query, environment, options)` gives a TransportField at the query's absolute time. It is deterministic in absolute time and independent of playback order (`:5-6`). Tracks are cached per asset (6 entries, `:72-73`).

Where it draws (`transport-view/🟦️.ts:65-109`):
- `parcels`: discs coloured by temperature excess (diverging, ±20 K shared, `:19-20`, `:45`) or vapour excess (sequential, 0…20 g/kg). Hatched when vapour.
- `pathlines`: thin lines since emission (world-fixed).
- `streamlines`: dashed instantaneous streamlines with arrowheads (`〰️streamlines` header).
- `condensation`: cyan-outlined discs.
- Emitter markers are drawn whenever any layer is on (`transport-view:7`, `:86-93`).

How to enable (programmatic only):
- `depict(state, {...defaultDepictionOptions("dark-line", "en", 100), transport: ["parcels", "pathlines"], environment?})`. The example is in `🧪️tests/🎨️seven-modes/🟦️.ts:57`.
- Environment for transport: `options.environmentTimeline ?? options.environment ?? asset.thermal.environment` (`🟦️.ts:132`).
- Not available through the cast manifest (no `transport` key, `🧬️schema/🟦️.ts:48`) or the CLI (`CliOptions` has no transport field, `🎬️cli:53`).

## 5. Export CLI, frame schedule, viewBox stability

CLI: `M/📤️export/🎬️cli/🟦️.ts`. Usage on line 55 and header line 2. Options parsed at `:58-74`:
- `--asset`, one of `--clip` or `--pose`, `--view`, `--scale`, `--fps` (default clip fps), `--format svg|pdf|dxf|dwg|png-sequence|mp4|webm`, `--out`, `--lang`, `--viewpoint`, `--catalog`, `--px-per-mm` (default 8), `--background` (default `#ffffff`).
- Not present: environment, timeline, theme, transport, annotation switches, thermal quantity, range, diagrams, forceQuantity.

Pipeline `runExport` (`:127-176`):
- Calls `defaultDepictionOptions(view, language, scale, viewpoint)` (`:132`), so every annotation is on, the thermal theme is light, the environment is the asset default and no transport is drawn.
- Transparency: only `png-sequence` and `webm` are transparent (`:133`). `mp4` is composited on `--background`, default white (`:165`, `flatten` in `🎥️video:53-61`).
- Formats: `svg` one file per frame (`:140-141`), `pdf` one page per frame (`:144`), `dxf`/`dwg` per frame (`:146-152`), raster formats via `rasterise` (`:111-124`) then `ffmpeg` (`:164-166`, libx264 yuv420p for mp4, libvpx-vp9 yuva420p for webm, `-fps_mode cfr`).

Frame schedule `M/📤️export/🎞️frames/🟦️.ts`:
- `frameSchedule(clip, fps)` (`:15-20`): `count = round(duration · fps) + (loop ? 0 : 1)`, times `n / fps` (`:18-19`). Integer fps 1..120.
- Pose: a single frame at t = 0 (`🎬️cli:85`).
- `frameName` → `frame-000000.png` (`:23`). `frameManifest` schema `semio.staffage.frames/v1` (`:29-44`). `writePngSequenceZip` checks frame count (`:56`).

viewBox stability:

| Output | Stable across frames? | Why |
|---|---|---|
| Cast SVG (`🎭️cast`) | Yes | `render` takes the union of origins and extents across frames (`:87-93`) and `placed` rewrites every frame to one `width`/`height`/`viewBox` and one world-origin alignment (`:60-66`). Test: `🧪️tests/🎭️cast-export/🟦️.ts:76-79`. |
| CLI rasterised PNG/MP4/WebM | Yes | `fixedCanvas` takes the union of origins and extents (`🎬️cli:103-109`), `rasterise` renders each SVG at its offset on that canvas (`:111-124`). |
| CLI SVG (`--format svg`) | No | Each frame is a separate `writeSvg` with its own `viewBox = 0 0 widthMm heightMm` (`🖼️svg:106`). `widthMm/heightMm` come from that frame's content extent (`🧱️canvas:98`), which follows the state's bounds and parcels. |
| CLI PDF | No | One page per scene, MediaBox = that scene's size (`📕️pdf:225-242`). |
| CLI DXF/DWG | Not applicable | Model-space, no paper frame. |

Why SVG is unstable: `depict` extent is the drawn content (bounds, shapes, annotations) plus margin, and `Canvas.finish` translates the model origin to match (`🧱️canvas:88-98`). The model origin's paper position therefore moves between frames unless the consumer aligns frames.

Do not use the CLI for figure-only frames. It has no way to switch annotations or theme off, and it always renders the asset default environment.

## 6. Cast exporter gaps for dark video

File: `M/📤️export/🎭️cast/🟦️.ts`. Schema: `M/📤️export/🎭️cast/🧬️schema/🟦️.ts`.

Good for this use:
- Figure-only by construction (`:74-75`), one stable viewBox per cast (`:87-95`), mirror option, environment and timeline for thermal, per-frame `solveThermalAt` (`:82`).
- `index` file (`🔣️.json`, `:96-109`) gives `originMm`, `mmPerMetre`, `boundsM` and `frames`, enough to rasterise with the same dpi rule as the CLI (`dpi = 25.4 · pxPerMm`).

Gaps:
- No theme key. Cast thermal, forces and moments frames are light-themed (`defaultDepictionOptions` at `:74` gets no theme). On `#001117` the outlines (ink `#001117`) are invisible, and the forces arrows and labels use light-theme colours (`palette:52-55`). Dark video should use `dark-line` or `dark-solid` for figures, and use a separate thermal pass with `theme: "dark"` through `depict`.
- No transport key (`KEYS`, schema `:48`).
- Environment and timeline only for `view: "thermal"` (schema `:65`), so thermal is the only view where the environment matters.
- Clip casts ignore `timeline` for frame timing (`framesOf`, `:36-42`), but still apply it through `environmentOf` on absolute clip time (`:72`). Document this if it matters.
- Default fps is 15 (`:26`) and default scale 50 (`:25`). The CLI uses the clip fps, 30 for most assets.
- SVG output only. Raster and video need a separate step.
- Untracked in git, so no stable release.

## 7. Verified and not verified

Verified by reading the code and the catalog files:
- Views, palettes, option defaults and gating, thermal solver entry points, environment presets, timeline semantics, transport gating, CLI options and viewBox behaviour, cast manifest keys and output alignment.
- Test references: `🧪️tests/🎭️cast-export/🟦️.ts` (viewBox test), `🧪️tests/🎨️seven-modes/🟦️.ts` (transport usage), `🧪️tests/🎨️thermal-legend/🟦️.ts`, `🧪️tests/🎨️view-synchrony`, `🧪️tests/🫁️respiration`.

Not verified (no run was performed, as requested):
- Actual rendering, colours on screen, legibility of thin outlines at 1:50 or 1:100.
- Thermal and transient numbers. The solver was not executed.
- Per-frame cost of `solveThermalAt` and of the transport track cache.
- rsvg-convert offset behaviour in `rasterise`, which the code assumes to align frames.
- Whether the untracked cast module is complete or tested beyond its test file.

## 8. Recommended figure-only route for dark video (for the planning step)

1. Figures in dark-line or dark-solid through `exportCast` with `view: "dark-line"` or `"dark-solid"`, one cast per figure and clip. Output: `<out>/<cast-id>/NNNN.svg` plus `🔣️.json`.
2. Thermal in dark: call `depict` directly with `view: "thermal"`, `theme: "dark"`, `transparent: true`, all annotations off, `diagrams.freeBody: false`, `thermalQuantity`, `range: shared`, and `environment` or `environmentTimeline`. This needs a small driver, since the cast has no theme key.
3. Transport (respiration plumes): direct `depict` only, with `transport` set, and the `annotations` layer removed from the scene before `writeSvg` (gap in section 2).
4. Raster and video: rsvg-convert per frame at `-d 25.4·pxPerMm` on the shared viewBox, then ffmpeg with the same codec settings as the CLI. Use `frameSchedule` for timing.
