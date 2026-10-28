# Model report: Villa Savoye teaching model (core, JSON, schema)

Scope after the ownership change: I own `C/🧱️model/🐍️.py` (core), `C/🧱️model/🔣️.json`, `C/🧱️model/🧬️schema/🔣️.schema.json`, the builder `T/model-build-json.py` and sections 1-2 of `T/api.md`. `🔌️exchange` (step_excerpt, mesh, ids_check), `🧮️analysis` (translate_structure, tributary, clash_check, link_values) and all tests (`🧪️cases`, `🧪️tests`) belong to other agents. Milestone 1 text with exact keys and examples: `T/model-milestone-1.md` (milestone 2 content is now the other agents' submodules, so there is no separate milestone-2 file).

## Files

| File | State |
| --- | --- |
| `C/🧱️model/🔣️.json` | version 2, 85 KB, generated once by `T/model-build-json.py` (handcrafted data in the script, `python T/model-build-json.py` rewrites it and refills `links.used` from the analysis submodule), 60 elements, 6 spaces, 3 types, 11 materials / layer sets, 52 relations, 3 models, 3 links, 3 demo events |
| `C/🧱️model/🧬️schema/🔣️.schema.json` | JSON Schema 2020-12, `additionalProperties: false` everywhere, AJV strict compiles; rejects bad GlobalId length, unknown class, missing basis, negative thickness, missing German text, extra keys (checked with `T/model-validate.cjs`) |
| `C/🧱️model/🐍️.py` | stdlib only, 25 public functions, every docstring starts with its own emoji, no comments inside definitions |

Negative-test paths for the tests agent (valid in the new JSON): `elements/0/guid` (piloti A1), `elements/0/class`, `elements/25/attributes/thickness/basis` and `.../value` (slab `SL-1`), `building/name/de`, `languages`, `revision`, `links/0/source_revision`.

## Decisions

- One orientation, stated in `orientation_note`: x west to east (21.5 m), y south to north (19 m), entrance with the curved glass wall north, salon south.
- Frame on a 5 x 5 grid, pilotis `A1`..`E5` (row letter = y row, number = x column), pilotis flush with the long façades (y = 0 and 19), set back 1.25 m from the short façades. 25 pilotis are a teaching assumption (sources disagree), noted in `notes`.
- Walls are infill (LoadBearing FALSE), pilotis LoadBearing TRUE. Ribbon window on all four first-floor façades, sill 0.90 m and head 2.00 m above the floor, solid ends of 1.5 m (sources conflict, noted).
- The car loop is derived, not typed: turning radius 5.5 m, width 1.75 m, wall clearance 0.25 m give the glass wall radius 3.5 m = 5.5 - 1.75 - 0.25 (`clearance()["curve_radius"]`, `building_geometry()["curtain_wall"]["radius"]`). The car enters from the north between E3 and E4, turns right then left (both on the 4.625 m centre line) along the glass wall, ends under the house at x = 3.875. Minimum clearance of the real car rectangle to every piloti along the path: 0.237 m (checked numerically; the glass wall centre (8.5, 12.0) was chosen by a search so that the path clears all 25 pilotis).
- Ramp: two `IfcRamp` HALF_TURN_RAMP (ground to first, first to roof), each two straight flights of 10.5 m, rise 1.75 m, slope 1:6, width 1.0 m, in the central bay x 7.6 to 10.0. Slab voids cover the whole ramp bay in the first-floor and roof slab (headroom).
- Roof slab (`SL-R`, ROOF) has the layer set `LS-ROOF`; zones `planted` (all four layers) and `solarium` (no soil) so the soil change affects only the planted part. The hanging garden (terrace, 110.21 m²) is an open void in the roof slab; the solarium (70.0 m²) sits on the roof slab.
- Services: boiler in the boiler room, one radiator under the south ribbon window, four roof drains with four downpipes, three heating pipe segments. The downpipe `DP-1` stands 0.18 m from piloti `A1` (rule: 0.25 m) as the deliberate teaching clash.
- Models: architecture (walls, openings, windows, curtain wall, ramps, screen, both slabs, spaces), structure (pilotis, both slabs, ramps), services. Same `schema_id` `IFC4X3_ADD2`; slab and ramp GlobalIds are identical in architecture and structure (same objects).
- Events: `OpeningResized` bumps window, opening, wall and the architecture model; `LayerThicknessChanged` bumps the slab and architecture + structure; both raise `revision` by 1 and keep every GlobalId. The `OpeningResized` event creates no analysis issue because no link uses the window; the soil event creates `stale-link` + `stale-value` entries on `LK-ROOF` until `resolve_link`.
- Scene event payloads are in `model["demo_events"]` (`sill`, `roof_soil`, `roof_soil_alt`); scenes call `demo_event(name)`.

## Sourced and assumed values

Sourced (research-villa-savoye): box 21.5 x 19 m (2.1), height 9.4 m (2.2), grid 4.75 m and initial 5.0 m (2.10), roof garden 110 m² (2.7) and solarium 70 m² (2.8) as reference areas, years 1928 / 1931 / 1965 / 1985 / 1997 / 2016, latitude/longitude (10.7), LoadBearing TRUE of the frame and FALSE of the infill walls (3.1).

Assumed (every one carries `basis: "assumed"` in the JSON): piloti diameter 0.30 m; storey elevations 0.00 / 3.50 / 6.70; first-floor slab 0.25 m; roof layers concrete 0.20 / waterproofing 0.02 / screed 0.06 / soil 0.10 m; wall layers render 0.03 / brick 0.24 / plaster 0.03 m; ribbon sill 0.90 and head 2.00 m, window extents; ramp width 1.0 m and slope; car length 4.6 m, width 1.75 m, turning radius 5.5 m, wall clearance 0.25 m, minimum side margin 0.50 m; densities (concrete 25, waterproofing 10, screed 22, soil 18, render 20, brick 18, plaster 18 kN/m³); finishes 1.5 kN/m², imposed 2.0 / 1.0 / 2.0 kN/m²; fire ratings (R 60, EI 30); boundary condition fixed; rule clearance 0.25 m.

Fits stated in `notes`: terrace 110.21 m² (+0.2 %), solarium 70.00 m²; enclosed ground area about 96 m² (the 190 m² of one source is not reproduced).

## Numbers (from the model)

Piloti 0.2297 m³ (Ø 0.30, height 3.25), all 25 pilotis 5.743 m³, first-floor slab 380.42 m² net (408.5 gross, ramp void 28.08) = 95.1 m³, roof slab 270.21 m² net, wall `W-S` 64.5 m² gross, 20.35 m² opening, glazing share 31.6 % (37.3 % after the `sill` demo event), clear width between pilotis 4.45 m, margin 1.35 m per side, with the initial 5.0 m grid 4.70 m and 1.475 m.

## Checks run (by me, not repository tests)

`T/model-smoke.py` (all functions print), `T/model-validate.cjs` (schema accepts the model, six mutations rejected), invariant script: 86 objects with unique tags and GlobalIds, every GlobalId equals `global_id(tag)`, every relation endpoint exists, every element is in a model, GlobalId codec extremes (`0000…`, `3$$$…`), consistency cycle (fresh model has no issues, soil event creates issues, `resolve_link` clears them and keeps revision 2), invalid events raise `ValueError`. No unittest counts from me: the tests belong to the tests agent.

## Not done / handed over

`step_excerpt`, `mesh`, `ids_check`, `translate_structure`, `tributary`, `clash_check`, `link_values` live in the other agents' submodules (present and working at the time of writing; the core re-exports them, falling back to path loading when the package is not on `sys.path`, but the analysis submodule itself needs the repository root on `sys.path`). Tests and the AJV test are the tests agent's.
