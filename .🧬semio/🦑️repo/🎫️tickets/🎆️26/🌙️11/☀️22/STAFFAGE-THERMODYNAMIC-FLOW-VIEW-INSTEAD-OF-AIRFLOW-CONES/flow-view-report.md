# Thermodynamic flow view of the staffage product

Request: "the airflow preview is currently extremely undersophisticated by showing cones, etc. Show proper thermodynamic view (different depending on the humidity and temperature with proper streams, particles and flow directions, etc)".

## What was there

- The transport module (`S/🔨️modules/💨️transport`) already solved a sound integral model: Lagrangian parcels of jets, plumes, breath and transpiration with entrainment, buoyancy and moist-air mixing (condensation by saturation adjustment), under any environment.
- Nothing of it reached the reader: the web viewer never asked for the transport layers, and the 3D stage and the axonometric drawing showed one cone per emitter (`emitterGlyph`). The cones were the whole "airflow preview".

## What it is now

One field, one set of marks, every view.

| Part | Where | What |
|---|---|---|
| Flow field | `S/🔨️modules/💨️transport/🌊️flow` | Eulerian velocity, temperature, vapour and condensate field reconstructed from the parcels: wind + Gaussian mean flow of every jet and plume (same volume and momentum flux as the top hat) + entrainment sinks `q = 2π α(Ri) b ℓ |u|` with ground images + inhalation sinks. Streams (Heun, fixed arc length) from documented seeds; tracers advected along them as a pure function of time; generalisation by paper spacing; flow domain around the body. `solveTransport(…, { flow: true })` → `TransportField.flow`; `developTransport` integrates the history in slices with progress and cancellation. |
| Contracts | `S/🔨️modules/🔗️contracts` | `FlowField`, `FlowStream`, `FlowPuff`, `FlowMarks`, `FlowRibbon`, `FlowStreak`, `FLOW_LAYERS` (`streams`, `tracers`, `moisture`, `fog`), depiction options `flow`, `flowDevelopS`. |
| Paper | `S/🔨️modules/🎨️depiction/🌊️flow-view` | `flowView` draws the layers over elevation, front, plan and axonometric; `flowPaint` (colours, widths) and `flowLegends` (K, m/s, g/kg, ambient air and dew point, English and German). The axonometric drawing has no emitter cones while a flow layer is drawn. |
| Screen | `S/🎯️targets/⚛️react/🔨️modules/🌊️flow` | `FlowServer` (solver behind a message protocol, in a worker; in the page where workers are missing), `FlowSession` (progress, cancel, retry, back-pressure), `FlowOverlay` (canvas renderer with the library's paint and marks), `FlowStage` (plain DOM: canvas, progress bar with cancel, retry, polite status). The drawing figure and the 3D stage both mount `FlowStage`; tracers move with the shared clock, the field is refreshed up to 8 times per second of clock time. |
| Viewer | `🎛️viewer`, `🗺️routing`, `🧍️figure`, `🧊️spatial` | Layer group "Air flow" (four toggles), address parameter `fl`, reference environment select, flow legends, the transport of the clock also for poses. The figure takes the paper of the flow domain; the 3D camera leaves room for the air (fit margin 2.6) and the model is built without emitter cones. |
| Exports | CLI, downloads | `--flow all|streams,tracers,moisture,fog` for `--view thermal` in svg, pdf, png-sequence, mp4, webm; the browser downloads carry the layers the viewer shows, developed first with progress and cancellation. |

No asset content, no volume rule and no rig code was touched; no asset data was missing: the 220 emitters of the catalog (breath 80, plume 61, transpiration 28, vent 28, fan 11, exhaust 9, flue 3) feed the view as they are. No runtime dependency was added.

## Physics shown

- Warm body → plume rises, entrained ambient air converges on it (grey streams with arrow heads turn into the warm stream); colder than air → it sinks; isothermal → no streams (verified in the viewer with the standing adult at 32 °C: only breath is left).
- Jets (fan heater, steam, split AC) leave along their nozzle, entrain, and bend by buoyancy: the hot jet lifts off the floor, the cold jet stays on it.
- Vapour: blue-green clouds and dotted lines where the air carries more vapour than the ambient air (breath, steam, transpiration drifting downwind of the oak).
- Fog only where the mixture is below its dew point: kettle steam condenses at the spout and re-evaporates sooner in warm air (short cloud at 30 °C / 80 %) than in cold air (cloud to the top of the domain at −5 °C); the split AC fogs at its outlet at 30 °C / 80 % (dew point 26 °C) and not at 22 °C / 50 %.
- The ground deflects: no air crosses `y = 0` (image system), induced air glides along the floor.

## Tests

New, all green:

| Test | Oracle | Count |
|---|---|---|
| `S/🧪️tests/🌊️flow-field` (fixture `🧫️fixtures/🌊️flow-field`, `🐍️.py`) | numpy re-implementation of the field; scipy `quad` (entrainment line sink with ground image, flux through a disc); scipy `solve_ivp` DOP853 (streams in arc length, particle advection in time); parcels from the Morton–Taylor–Turner plume and the top-hat round jet | 7 |
| `S/🧪️tests/🌊️flow-environments` (fixture, `🐍️.py`) | CoolProp 8 and psychrolib 2.5: dew point and mixing-line fog for all 12 reference environment presets; plume direction by Newton's law of cooling | 26 |
| `S/🧪️tests/🌊️flow-view` (fixture) | hand-derived tracer schedules; d3-scale, d3-interpolate, d3-quadtree | 10 |
| `S/🎯️targets/⚛️react/🧪️tests/🌊️flow` | library `solveTransport` as oracle of the solver protocol (bit-identical field), address round trip, jsdom figure | 9 |

Findings the oracles forced:

- The sampled volume flux of the plume was 20 % above `π b² w` at 1 m: not an error of the Gaussian profile but the flux the entrainment sinks drive through the same disc. With that term from scipy `quad` the match is 1.8–2.3 % (tolerance 3 %).
- The colour ramp had a step at the edge of the neutral band; d3-interpolate exposed it, the band now blends to the colour at its edge.
- Below 0 °C CoolProp and psychrolib give the frost point over ice; the model has no ice phase and gives the dew point over supercooled water, up to 0.5 K lower (documented tolerance in the fixture).

- The React test found a real defect: where a `Worker` exists but its module fails to load, the session waited for ever. The session now hands its assets, open sessions and latest frames to the in-page solver on a worker error.

Existing suites run after the change, all green (no test was changed or weakened):

- Library: `transport-field`, `transport-view`, `transport-assets`, `parcel-jet`, `condensation`, `vapour-pressure`, `phase-change`, `thermal-multiview`, `breathing-catalog`, `leaf-transpiration`, `seven-modes`, `axonometric-depiction`, `model-glyph-edges`, `model-glyph-size`, `model-looks` — with the three new suites 350 tests in two runs (94 + 256).
- React target: `address-roundtrip`, `figure-render`, `model-controls`, `model-view`, `model-viewer-page`, `custom-element`, `downloads`, `viewpoints`, `range-span` and the new `flow` — 150 tests.
- `tsc --noEmit` of the library package: no error. Of the React package: 8 errors, all in `🧰️framework/🔨️modules/🖱️ui` (not touched here), none in the staffage files.
- Not run: the rest of the staffage suite (the whole suite takes hours on this host), in particular the export suites (`cast-export`, `model-catalog`, `export-performance`, `pdf-reopen`); they do not pass `flow` and the default depiction is unchanged.

## Verified in the browser (screenshots looked at)

Site `staffage-site-native`, port 6072, thermal view, side elevation unless noted.

| Asset | Cold, dry (−5 °C, 30 %) | Warm, humid (30–32 °C, 80–85 %) | Neutral (22 °C, 50 %) |
|---|---|---|---|
| Standing adult | strong plume from the head, entrained streams, breath vapour | no body plume (isothermal), weak breath only | moderate plume, breath vapour cloud |
| Electric kettle | fog cloud to the top of the domain, entrainment from both sides, warm plume | short fog cloud at the spout | medium fog cloud |
| Electric fan heater | hot jet lifts off the floor, entrainment from above | same structure, weaker buoyancy | — (own environment in the library render) |
| Split AC, pose cooling | — | cold jet with fog at the outlet | cold jet along the floor, no fog |
| English oak, 1:200 | few vapour puffs (2 °C) | vapour puffs downwind (30 °C / 85 %) | vapour puffs downwind (28 °C / 35 %) |

Also looked at: standing adult at 1:500 in the light theme with German legends (one bold plume stream with its arrow head, breath fog at 5 °C / 80 %, legends "Übertemperatur der strömenden Luft (K)", "Luftgeschwindigkeit und Richtung (m/s)", "Wasserdampfüberschuss (g/kg)"); panel radiator on a phone viewport (375 px, German, dark, no horizontal overflow, status "Luftströmung gezeigt: 56 Stromlinien in Luft von 18,0 °C …"); cooking pot and kettle in the 3D model, light theme, perspective and side-elevation camera (cones gone, fog, plume and entrained streams projected by the camera); the wind direction of the standing adult's own environment (bearing 180°) against the drawn plume.

## Limits and follow-ups

- The asset's own body does not deflect air; only the ground does. Wind streams pass through a crown or a torso. A body obstacle model is the next physical step.
- The 3D overlay is drawn over the model without depth occlusion (the kernel viewer has no dynamic line pass). GLB, FBX and the neutral model keep the emitter cones; a flow in those files needs line primitives in the kernel exporters.
- A wall-mounted unit (split AC, ceiling diffuser) is modelled in its own frame with the ground at its lower edge, so its cold jet cannot fall; that is the asset frame of the existing transport model, not of the view.
- The reference environment select of the viewer applies the five values the address carries (air, radiant, humidity, air speed, sun), not sky and ground temperature.
- The `<staffage-figure>` custom element has no `flow` attribute yet: its drawing path is its own SVG pipeline; it needs the same `FlowStage` mount as the React figure.
- The whole-suite run and the cast manifest (`transport` layers of the energy course) are untouched; a cast option `flow` would bring the new layers into the course clips.
- Transpiration reads weakly at 1:200 (small faint puffs); it is correct in amount but could use its own mark.
- The Python oracle of `flow-field` runs in the `geometry` set of the staffage oracles; the one of `flow-environments` needs CoolProp and psychrolib (`uv run --no-project --with …`), because the energy project's test environment does not build on this host (pycairo). Adding a `psychrometrics` group to `S/🧪️tests/🧰️support/🐍️oracles/pyproject.toml` and its lock would pin them.

## Files

New: `S/🔨️modules/💨️transport/🌊️flow/🟦️.ts`, `S/🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts`, `S/🎯️targets/⚛️react/🔨️modules/🌊️flow/` (`🟦️.ts`, `🧵️server/🟦️.ts`, `🧵️server/worker.ts`, `📡️session/🟦️.ts`, `🖌️overlay/🟦️.ts`, `🖥️stage/🟦️.ts`), tests and fixtures `🌊️flow-field`, `🌊️flow-environments`, `🌊️flow-view` under `S/🧪️tests` and `S/🧫️fixtures`, `S/🎯️targets/⚛️react/🧪️tests/🌊️flow`.

Changed: `S/README.md`, `S/📦️packages/🟦️typescript/🟦️.ts`, `S/🔨️modules/🔗️contracts/🟦️.ts`, `S/🔨️modules/💨️transport/🟦️.ts`, `S/🔨️modules/💨️transport/🌫️air/🟦️.ts` (`dewPointC`), `S/🔨️modules/🎨️depiction/🟦️.ts`, `S/🔨️modules/🧊️volume/🟦️.ts` (`emitterMarkers`), `S/🔨️modules/📤️export/🎬️cli/🟦️.ts`, and in the React target `🧍️figure`, `🎛️viewer` (+ `🗺️stage`, `🧾️reading`), `🗺️routing`, `🧊️spatial` (`🟦️.ts`, `🖥️stage`, `🏭️prepare`, `⚛️figure`), `🧮️pipeline`, `📤️downloads`, `🔬️detail`, `🌐️i18n` (English and German viewer texts).

Scripts kept in this folder: `flow-smoke.ts` (develops assets in presets, writes SVGs), `flow-profile.ts` (timings), `flow-session-probe.ts` (solver outside the browser).

Timings on this host under load (standing adult, 14–16 s of history): develop about 3.5 s of CPU in 56 slices; a refreshed field about 45 ms (it was 600 ms before the sampler was packed and far sinks aggregated); marks about 1 ms per frame.
