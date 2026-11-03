# R2 ts: thermal span range and wall-mounted split unit

Scripts in `🧰️ts/` (`ts_list.py` lists thermal casts, `ts_manifest.py` set the span on the course casts, `ts_sheet.py` exports before/after casts onto a sheet, `ts_render.sh` renders one scene). Generated output in `🗑️generated/ts/` was deleted.

## 1. Thermal span (`thermalSpanK`), schema first
- Contract: `DisplayRange` gets a fourth mode `{ mode: "span"; spanK }` (`🔗️contracts/🟦️.ts`). `scaleOf` (`🖍️palette/🟦️.ts`) returns `centre - spanK ... centre + spanK` for it.
- Thermal view (`🌡️thermal-view/🟦️.ts`): span applies to the surface temperature only; skin temperature and all flux quantities fall back to `shared` (their documented ranges are not air-centred). New `rangeNote` gives the legend sentence in en and de ("Span ±25 K around the air temperature (a ... b °C), identical for every cast of one scene; values beyond it saturate"); `Legend.range` is `"span"`. Transport view treats span like shared (`💨️transport-view`).
- Cast schema: `thermalSpanK` (number > 0, thermal view only) in `📤️export/🎭️cast/🧬️schema/🔣️.schema.json` and its TS twin (validator, `KEYS`, type); the exporter passes `range: { mode: "span", spanK }` to depict. The cast key changes with the entry JSON, so every thermal cast re-exports.
- Default: the absolute shared range (-20 to 60 degC) stays available when the field is absent (scenes comparing environments). The course uses 25 K (21 degC room: -4 to 46 degC; 35 degC laptop reads 0.56 of the warm ramp instead of 0.36).
- Manifest: all 85 thermal casts of `V/🧍️staffage/🔣️.json` (after agent xc's merge to 97 casts including the 28 `shared-*` ids) carry 25. One value everywhere means every scene group and every shared cast is consistent by construction; no cast was split by span. Guard: `V/🧪️tests/🐍️staffage_only.py` `test_thermal_span_every_clip_draws_its_thermal_casts_on_one_span` (resolves the casts of each energy clip and requires at most one distinct span) with vectors under `thermal_span` in `🔣️staffage_only.json`.
- Legend/glyph pathway: the clips contain no thermal-scale glyph (casts render figure-only, legend off); the analytic legend states the span. The React viewer does not offer span (its range control stays auto/fixed).
- Tests: `🧪️tests/🌡️thermal-span/🟦️.ts` + `🥒️.feature` + fixture `🧫️fixtures/🌡️thermal-span/🔣️.json` (scales, signed fractions vs d3-scale piecewise linear, colour equality to the shared colour of the same fraction, legend text en/de, other quantities keep shared, ajv and twin accept/reject, course manifest all 25).

## 2. Split indoor unit (`❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`, `_build_ahu`)
The cooler now hangs on a wall: casing top at `MOUNT_TOP_FRACTION` = 0.88 of a storey (about 2.2 m of 2.5 m, 0.3 m below the ceiling), drawn 0.7 high (schematic enlargement, the house storey is 1.2 units), a white wall line from the floor to a ceiling tick and two bracket ticks at the casing back. Supply path start, coil centre, fan and extract path follow the unit; the unit label moved up (top + 0.95) so the "18 °C" readout and the Kühllast symbol (moved left of the Fortluft stream end) no longer overlap. The open vane now ends above the floor.

## 3. Visual checks (rendered and viewed)
- Before/after sheet of laptop, person, tealight, toaster, seated worker, radiator (cast SVGs): after is more saturated; person and seated worker are only slightly stronger at 25 K (clothing 28 degC at 21 degC air), radiator and toaster clearly. The thin laptop and tealight lines stay small at 1:50 (asset size, not range).
- Heating-comparison Beat1, Beat2, Beat3 rendered (ql, 15 fps) and viewed with the new keys: people, laptop, lamps warm and readable; Beat3 unit mounted high, no overlaps. Other thermal scenes in the same run were not separately rendered (rc rendered the cooling set earlier; all share the same exporter path).

## 4. Tests
- Staffage library vitest (ticket alias config): 53 files; the run first failed two suites because another agent's schema edit (`transportAgeS`, `required` without declared property, ajv strict) broke ajv; I declared `transport`/`clip` in those two `then` clauses; thermal-span + cast-export then pass (71 passed, 1 skipped). Full-library run before that fix: 1001 passed, 10 skipped in the 51 loadable files.
- React vitest: 17 of 17 files, 107 passed, 1 skipped.
- Visuals tests (`staffage_only` + `🧪️tests.🐍️`): 27 tests OK; cooling clip tests: 4 OK.
- Catalog check: stopped after 55 assets checked, 0 failures, because it ran too slowly on the loaded host (catalog files not touched by this work).

## Open
- A span of 25 K makes the human clothing only moderately warmer; a 15 K span would punch more but was left at the coordinator's example.
