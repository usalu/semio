# Work package `controls`: full control over the air of the flow view

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `R` = `S/🎯️targets/⚛️react/🔨️modules`, `T` = `S/🎯️targets/⚛️react/🧪️tests`.

## What changed

| Part | Files | What |
|---|---|---|
| Quantities | `R/🌤️environments/🟦️.ts` | `Quantity` (field, address parameter, element attribute, range, step, decimals, unit, shown ↔ stored), `ENVIRONMENT_QUANTITIES` for all ten `Environment` fields, `readQuantities` / `writeQuantities`, `environmentOf(asset, preset, changes)` (a preset is applied whole; `own`, `changed`, `base`). |
| Flow rules | `R/🌊️flow/🟦️.ts` | `FlowSettings` (flat reader settings), `FLOW_DEFAULTS`, `FLOW_QUANTITIES`, `flowConditionsOf` (→ library `FlowConditions`, only what was set), `MARK_DEFAULTS`, `MARK_QUANTITIES` (spacing, shortest streak, tracer period, exposure, plus trail of the parcel paths and stroke width, which arrived in the contract while I worked), `WIND_SECTORS` / `windSector`, layer code `p` for `pathlines`, `flowTimeS(…, developS)`. |
| Solver path | `R/🌊️flow/🧵️server`, `📡️session`, `🖥️stage` | Protocol `open` carries `conditions` (parcel life always stated) and `pathlines`; `FlowSession.open(asset, query, environment, conditions, pathlines)`; `FlowStageOptions.conditions` restart the solve, `.detail` only redraws; `FlowStage.mount(parent)` (re-parenting, style into a shadow root), `.flow`, `paperProjection` (shared by React figure and custom element), status words and `data-wind` state the wind. |
| Overlay (physics' file, two small edits) | `R/🌊️flow/🖌️overlay/🟦️.ts` | `FlowFrame.detail` passed to `flowDetailOf` and its `strokeMm` to `flowPaint`. Nothing else. |
| Address | `R/🗺️routing/🟦️.ts` | `ViewerState.preset`, `.environment` (all fields), `.flowSettings`, `.flowDetail`. Parameters, stable order: `env air mrt sky gnd rh wind wdir sun dif hpa wp z0 zr ti tf tt life seed fsp fst fpe fex ftr fsw`. A value equal to its default is dropped, a bearing folds into 0…360, every value is held in its range. |
| Controls | `R/🎛️viewer/🌬️air/🟦️.tsx` (new), `R/🎛️viewer/🧭️wind/🟦️.tsx` (new), `R/🎛️viewer/🟦️.tsx`, `R/🎛️viewer/🗺️stage/🟦️.tsx` | Four collapsible sections (environment, wind, turbulence and wind profile, flow marks). Every number: label with unit, field in the reader's number format, slider, "changed" in words, back-to-default button; sections count their changes. Wind dial: speed and direction together, drawn as the ground in the current viewpoint (circle in plan, foreshortened ellipse in elevations, axonometric ellipse for axonometric and 3D), one `slider` with keyboard (arrows, Shift, Home, End), text alternative ("The air moves towards the asset's left (90°) at 1.5 m/s."). Preset select applies all fields, names the active preset, reads "Changed by hand, starting from: …". |
| Texts | `R/🌐️i18n/🔠️english/🎛️viewer.ts`, `🔡️german/🎛️viewer.ts` | 66 new keys, key for key; the five `viewer.env.<slider>` keys are replaced by `quantity.*`. |
| Figures | `R/🧍️figure/🟦️.tsx`, `R/🧊️spatial/⚛️figure/🟦️.tsx`, `R/🧊️spatial/🖥️stage/🟦️.ts` (`StageOptions`, `#syncFlow` only) | Props `flowConditions`, `flowDetail`. `ModelFigure` now hands omitted optional settings to the stage as unset (before, going back to the asset's own environment left the 3D model in the old one). |
| Export request | `R/🧮️pipeline/🟦️.ts` (`DepictionRequest`, `depictionOptions`), `R/🔬️detail/🟦️.tsx` | Request carries `flowConditions`, `flowDetail` and the whole environment; `flowDevelopS` follows the chosen parcel life. `R/📤️downloads` not touched. |
| Custom element | `R/🧩️element/🟦️.ts` | Attributes `flow`, `environment`, one per quantity (`wind-speed`, `wind-direction`, `air-temperature`, …, `turbulence`, `wind-profile`, `roughness`, `reference-height`, `flow-life`, `flow-seed`, `flow-spacing`, …), property `.flow`, event `staffage-flow`. The same `FlowStage` in the drawing, in every cell of the sheet and in the 3D model. |
| Sheet, comparison | `R/🔭️viewpoints/🟦️.ts`, `🪟️sheet/🟦️.tsx`, `R/⚖️compare/🟦️.tsx` | `sheetOf(…, { flow })` leaves room for the flow domain; all three cells draw the flow on the shared clock. Thermal comparison panes show the flow (asset's own environment, default conditions) on the shared clock. |
| Tests | `T/🌬️air-controls/{🟦️.tsx,🥒️.feature,🐍️.py}`, `S/🧫️fixtures/🧭️wind-dial/🔣️.json`, `T/🗺️address-roundtrip`, `T/🌊️flow` | See below. |

## Verified

- `tsc --noEmit` of the React package: no error in staffage files (only the known `🧰️framework/🔨️modules/🖱️ui` ones).
- React suites, last run: `air-controls address-roundtrip 🌊️flow custom-element 🔭️viewpoints figure-render compare-state shared-borders model-controls` → 9 files, 116 tests, all green. `air-controls` alone: 16 tests (dial against the numpy fixture, 30 cases; keyboard; labels and units in both languages; number format; presets; mark detail redraws while wind and conditions reopen the session and `ambient.windMS` / `windDirectionDeg` follow; the viewer's dial reaching the solver; element attributes on drawing, sheet and model; export request). `address-roundtrip`: 2 new tests.
- Oracle: `uv run … --only-group geometry python -I T/🌬️air-controls/🐍️.py` → "wind dial fixture agrees with numpy: 30 cases".
- Real site (built-in browser, `http://localhost:6072`, German):
  - `#/humans/standing-adult?view=thermal&pose=relaxed&zoom=2.2&wind=0.6&wdir=0&life=8`: plume and breath lean ahead of the figure; four Shift+Right on the dial → address drops `wdir` (180° is the asset's own), status "Wind 0,6 m/s, nach hinten, hinter das Asset (180°)", plume and vapour lean behind the figure; Home → back ahead. Screenshots looked at.
  - Three-view sheet (`vl=all&wind=0.8&wdir=90`): three flow layers ready, each `0.8 90`.
  - Comparison with two thermal panes: two flow layers ready.
  - Phone viewport 375 px: sections closed at first, open by button, no horizontal overflow, smallest target 24 px.

## Not done, and why

- **Playgrounds carry no flow.** Exactly why: (1) every placement plays through its own track (start, speed, hold, loop window; `readTrack`), while a `FlowSession` maps one clock to one transport timeline, so each placement would need a derived clock; (2) wind is defined in the asset's frame, placements are yawed and mirrored, and each flow is solved for one asset alone, so plumes would pass through neighbours and "one wind for the scene" needs a per-placement transform that does not exist; (3) the section and plan figures are `PlacedFigure` (`R/📍️placed`, not mine, drawn by the frame driver outside React) and the 3D playground is one composed scene (`ModelPlayground`) with no projection hook like `ModelStage.#projection`. Doing poses only would have been the half-done version.
- Comparison panes have no air controls of their own (they use each asset's own environment and the default conditions).
- `loopS` of the mark detail is not a reader control: it belongs to looping exports.
- `T/🧊️model-viewer-page` "switches to the 3D model…" fails on this loaded host at its 1 s default wait for the lazy 3D chunk; a copy with a 30 s wait passed. Not changed (not my test); I could not tell whether it also failed before my edits.
- In `air-controls` a testing-library `getByRole("combobox")` hung (worker killed) when it followed a test that mounted the controls twice; I split that test per language and query rows directly. The cause inside jsdom / testing-library is not found.
- Contracts moved while I worked (`pathlines` layer, `trailS`, `strokeMm`, no `streamlines` option); I adapted, but did not check how the overlay draws the `pathlines` layer.
- No ticket was opened or closed; no temporary files are left in the ticket folder.
