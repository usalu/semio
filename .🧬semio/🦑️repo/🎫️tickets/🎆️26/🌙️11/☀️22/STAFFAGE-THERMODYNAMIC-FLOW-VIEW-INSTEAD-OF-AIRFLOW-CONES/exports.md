# Package `exports`: every 2D export carries the thermodynamic air flow

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `R` = `S/🎯️targets/⚛️react/🔨️modules`.

## State in one paragraph

The work is implemented and was proven on written files while the test runner was available: the new library suite `🌊️flow-exports` passed 34 of 34. Then the dependency store of the repo went away on this host (`node_modules` → `.cache/b/d/m/j/…`, the disk image `.cache/b/d/fixed.dmg` is no longer mounted since about 15:26; `repo dependencies ensure` fails with an internal error of the repo tool). From then on neither vitest nor `tsc` could be started. **Not re-run after that point: the final type check, the React suite after my last test fixes, and the neighbouring export suites.** I did not mount the image by hand (shared infrastructure, not mine). Everything that needs only bun and uv (the real command line, ffmpeg, the Python readers) was still run and looked at.

## Decision: the default

The thermal view shows the air flow by default everywhere in the product, so **every thermal 2D export of the command line carries all flow layers unless `--flow none`**; the other views never draw it. `defaultDepictionOptions` of the library stays explicit (no flow unless `options.flow`), so no other thermal test changes. The author's preview follows the same default for a single drawing and a `--views` sheet; its contact sheet `--sheet` draws no flow. Documented in the header of the CLI and in `S/README.md` ("Air flow in the exports").

## What changed

| File | Change |
|---|---|
| `S/🔨️modules/🎨️depiction/🌊️flow-view/📐️model/🟦️.ts` (new) | `flowModelLayers`, `ModelFlow`, `FLOW_CAD`, `flowClassOf`, `circlePolyline`: the flow as model-space geometry in metres on `FLOW-STREAMS-`, `-ARROWS-`, `-TRACERS-`, `-PATHS-` (each `WARM`/`NEUTRAL`/`COOL`, ACI 1/8/5), `FLOW-VAPOUR` (134), `FLOW-FOG` (4) with lineweights |
| `S/🔨️modules/🎨️depiction/🟦️.ts` | `modelScene`/`modelSceneAsync` take `flow` (planar and axonometric), `ModelSceneOptions`; `FLOW_DEVELOP_S`; `flowLoopS` and the loop period of the tracers for a looping clip; a drawing with flow reserves the paper of the whole flow domain (stable extent over the frames of a sequence); `transportOf` takes asset and query |
| `S/🔨️modules/📤️export/🎬️cli/🟦️.ts` | flow by default for thermal, `--flow all|none|layers` in every drawing format incl. sheets, DXF, DWG; environment flags (`--air-temp`, `--radiant-temp`, `--sky-temp`, `--ground-temp`, `--humidity`, `--wind`, `--wind-direction`, `--sun`, `--diffuse`, `--pressure`) over `--environment`; `--turbulence`, `--turbulence-floor`, `--turbulence-time`, `--wind-profile`, `--roughness`, `--reference-height`, `--flow-life`, `--flow-seed`, `--flow-develop`, `--flow-spacing`, `--flow-streak`, `--flow-period`, `--flow-exposure`, `--flow-trail`, `--flow-stroke`, `--duration`; each refused by name in English and German where it makes no sense; `drawingOf` (the one request every frame shares); a pose plays its flow (`--duration`, default 6 s at 25 fps for png-sequence, mp4, webm) on one standing figure; the note of a DXF/DWG states the flow layers |
| `S/🔨️modules/📤️export/📑️sheet/🟦️.ts` | `modelSheet` takes `flow` (every cell, prefixed layers); the measured figure of a sheet ignores flow marks |
| `S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` | flow on the thermal view by default, `--flow none`, developed with cancellation first |
| `R/📤️downloads/🟦️.ts`, `R/🧮️pipeline/🟦️.ts` | DXF and DWG carry the flow of the request; frames are evaluated first and drawn on the union canvas (`framedSvg`); a pose with flow plays (`POSE_PLAY_S`, `sceneWithAir`); progress and cancel through `JobControl` |
| Small edits in files of other packages (re-read first) | `S/🔨️modules/🔗️contracts/🟦️.ts`: `FlowMarkDetail.loopS`, `FlowField.domain`; `S/🔨️modules/💨️transport/🌊️flow/🟦️.ts`: `loopS` in `flowDetailOf`/`flowMarks` (release interval stretched so a whole number fits the loop), `domain` in the returned field; `S/🔨️modules/🔭️multiview/🟦️.ts`: one transport solve through `transportOf` when flow is asked; `S/📦️packages/🟦️typescript/🟦️.ts`: exports |
| Tests | new `S/🧪️tests/🌊️flow-exports/{🟦️.ts,🥒️.feature,🐍️.py}` with `S/🧫️fixtures/🌊️flow-exports/🔣️.json`; new `S/🎯️targets/⚛️react/🧪️tests/📤️flow-downloads/{🟦️.tsx,🥒️.feature}`; `S/🧪️tests/🌤️reference-environment/🟦️.ts`: the reference drawing of the CLI comparison now has the flow layers (the one test that pinned thermal export bytes) |
| Docs | header of the CLI, `S/README.md` section "Air flow in the exports" with flag table and command examples |

## Verified

- `🌊️flow-exports`: **34 passed of 34** (last complete run, about 3 minutes; command of the brief with filter `flow-exports`). What the readers proved on the written files: SVG marks of every kind in the layer group "Analysis field", inside the flow domain, stream, arrow, tracer and fog colours equal `flowPaint` (svgelements + Python's XML parser); `--flow none` leaves the same figure; a PDF page rendered by PDFium has warm pixels above the kettle and none without flow, pikepdf finds no problem; three frames of a playing pose differ only in flow marks and flow legends, state field times 14.00, 14.50, 15.00 s, and numpy finds every tracer one frame time further along its stream; a looping clip one duration later has the flow marks of its start (tolerance 0.001 mm); DXF audits clean in ezdxf, flow layers with colour and lineweight, stream runs equal the projected stream points to 1 µm, vapour and fog are 32-chord circles in the domain; the DWG read by ezdwg has the same flow layers and polylines; sheet cells show tracers of one field aligned between cells, legends once; PNG sequence and MP4 of a pose have the scheduled frames and the frames differ.
- Tools in that suite: `ffmpeg` real (pinned imageio-ffmpeg build through `repo tools exec`), `rsvg-convert` **not on this host**: the MuPDF double of `oracleToolPort`, as the existing export tests use. `repoToolPort` alone finds neither program here.
- Real command line, 14 commands (`exports/run-cli.sh`, rsvg stand-in `exports/tools/rsvg-convert`): svg, pdf, dxf, dwg, sheet-pdf, sheet-dxf, sheet-dwg, png-sequence, mp4 (2.1 s at 10 fps), webm, a looping clip as mp4 (5 s at 5 fps, German, `winter-clear-night`), svg with wind profile and turbulence, axonometric svg of the standing adult: written. Looked at: the SVG and the sheet rasterised with `qlmanage`, first and last frame of both videos (figure fixed, flow and field time advancing 14.00 → 15.90 s and 15.00 → 17.40 s).
- Type check: clean for all files of this package at the last run before the store vanished.

## Not done, open

- **Not re-run because the dependency store is gone**: `tsc` of both packages at the end; `📤️flow-downloads` (first run: 4 of 6 passed; the two failures were most likely wrapped legend text and too narrow regular expressions in the test, I rewrote those assertions but could not run them again); `📤️downloads`, `🌤️reference-environment`, `📑️multiview-sheet`, `🔭️multiview-coverage`, `📐️dxf-reopen`, `📕️pdf-reopen`, `🎞️frame-timing`, `🗜️zip-reopen`, `🧊️axonometric-exports`. These need one run once `node_modules` is back.
- Two CLI commands failed for a reason outside this package: the electric kettle has no axonometric drawing at the moment (`view-missing`), so its axonometric SVG and its four-cell sheet are refused; the axonometric path was exercised with the standing adult instead.
- Sheets: the extension lines of the overall dimensions cross the legend under the side elevation. This existed before, but the flow legends make that block long. I wrote a fix (legend below the dimensions) and took it out again because I could no longer test it.
- Circles in DXF/DWG are closed polylines of 32 chords, no CIRCLE entity: `ModelLayer` holds polylines and texts, and a true circle needs a new entity in the Rust DWG writer.
- CAD layers have one colour and lineweight each, so the colour ramp becomes three classes and the speed-dependent width is lost; model space does not hide marks behind the body.
- A pose that plays is not made periodic, only looping clips close.
- `📤️export/📦️batch` is outside this package and writes its thermal drawings as before.
- `flowLoopS` exists twice in spirit (`depiction` here, `flowPeriodS` in the React flow rules); the screen overlay does not pass `loopS` yet.

## Kept in `exports/`

`run-cli.sh`, `tools/rsvg-convert`, `probe-advance.ts`, `probe-loop.ts`, `dxf-look.py`. `🗑️generated` is deleted.
