# Round 2 final visual QA of the new catalog assets (agent qa)

## Method
- Scope: the 41 untracked asset documents under `S/🗂️catalog` at run time (the brief said about 51; `git status -uall` listed 41 new `🔣️.json` files: 8 machinery, 4 humans, 19 appliances, 1 vehicle, 1 furniture, 4 objects, plus `ceiling-diffuser`, `electricity-meter`, `gas-meter`, counted in the appliances). The listing is in `T/🧰️qa/new.txt`.
- Export: `T/🧰️qa/export.ts` runs the archive's own library path in one bun process (`loadCatalogFromFiles`, `evaluate`, `defaultDepictionOptions`, `depict`, `writeSvg`, the same functions the export CLI uses) for the default pose at 1:50, dark-line and thermal, every viewpoint the asset declares (elevation, plus plan or front where declared). Thermal sheets have the legend block switched off so the figures are legible. 114 SVGs including 8 pre-existing references (cyclist, seated-reader, park-bench, bookshelf, heat-pump-outdoor-unit, single-bed, three-seat-sofa, stepladder), 0 export errors.
- Sheets: `T/🧰️qa/sheets.py` (PyMuPDF) built 17 labelled contact sheets, one per category and page (dark-line left, thermal right), plus `T/🧰️qa/zoom.py` for the tiny assets. All sheets were viewed. Generated output deleted.
- Catalog check: full run `repo tools exec bun -- 🩺️check/🟦️.ts` (no edits made by me, so no per-asset rerun was needed); result below.

## Result
No real defect found, no asset file edited. Set is consistent: same Semio line language, ground and dimension conventions, scale bar and title block, thermal palette from one diverging scale. Items for the record are minor and either by design or library-level.

## Per-asset verdict
Humans (4): all OK.
- person-cooking: OK. Elevation only, pot red on the hob, body orange, cabinet grey.
- seated-office-worker: OK (elevation and plan). Plan has desk top grey, laptop warm; proportions match seated-reader.
- sleeping-person: OK by design. Figure sits at the mattress datum with no bed or ground contact in the standalone export (asset summary: made to lie on the catalog single-bed), so the 0.45 m dimension and the gap to the ground line look odd alone only.
- standing-adult: OK (elevation and plan). Same stroke and hatch language as cyclist.

Appliances (19): all OK.
- ceiling-diffuser, circulation-pump, decentralised-hrv-fan, duct-fan, electricity-meter, gas-meter, room-thermostat, thermostatic-radiator-valve: OK. Wall or duct datum line runs through the device on purpose (in-line or wall-mounted parts).
- desktop-monitor, laptop: OK, edge-on thin profile in elevation as intended.
- electric-fan-heater, electric-kettle, toaster, gas-hob (elevation and plan): OK. Thermal in operating pose is saturated red over the whole housing; acceptable for the heating poses, plan of the gas hob hides burner ring detail under the flat red fill (nit, left as is).
- gas-boiler, hot-water-cylinder (grey cold bottom, warm top), kitchen-extractor-hood, mvhr-unit, server-rack (upper half grey blanking panels, lower half hot servers), shower-head, vacuum-cleaner, split-ac-indoor-unit (elevation, front and plan, all off and grey): OK.
- pendant-lamp: OK. Datum line passes the bulb tip (reference level of the 0.80 m dimension); nit.

Machinery (8): all OK.
- evaporative-humidifier (blue cooling pad), expansion-valve, finned-heat-exchanger, plate-heat-exchanger, scroll-compressor, sorption-wheel (red regeneration sector), oil-pumpjack (large drawing, hairline weights follow from the drawing size at fixed sheet width, not a defect).
- solar-thermal-collector (elevation, front, plan): OK geometry. Note: thermal views from the default environment (21 degC, solar 0 W/m2, no environment passed by the CLI path) show the absorber blue and only the headers red in the `operating` pose; with the declared `summer-heatwave-calm` environment the absorber is the hot part. Agent sc owns that file, so I left it; the CLI and my export do not pass the asset's reference environment (known limit in `audit-views-thermal.md`).

Other categories: all OK.
- vintage-car-1929 (elevation and plan): OK, heavier detail than city cars but internally consistent, windows grey, body warm.
- operating-table: OK; light dome red, table top and column grey.
- chalkboard (wall-mounted, left leaf), cooking-pot (red boiling water), storage-crate-stack, tealight: OK. Tealight and cooking-pot are 0.04 m and 0.10 m high; their dimension text sits over the extension line (library dimension layout for objects under about 0.1 m, not asset geometry).

## Cross-set observations
- Line weights: the new appliances use the same role weights as the references; apparent thinness of large drawings (operating-table, pumpjack, server-rack) comes from the sheet scaling.
- Detail level: new machinery and appliances are at or slightly above the reference heat-pump-outdoor-unit; vintage-car-1929 is the most detailed asset in the set, acceptable.
- Thermal: all at-rest palettes sit on the diverging scale (grey and orange for ambient solids, blue for evaporative cooling and cold sinks, red for heaters, kettle, pot, compressor).
- No stray geometry, gaps, or overlapping fills seen.

## Open
- Possible library follow-up: dimension label collides with the extension tick for objects below about 0.1 m (tealight, expansion-valve, gas-hob, cooking-pot, thermostat).
- Export CLI has no option to pass the asset's reference thermal environment (solar-thermal-collector looks cold in static pose exports).

## Catalog check
Full run (no arguments, after the exports): 149 `✓` lines (manifest plus assets), 0 `✗`, exit 0. No asset edited by this agent.
