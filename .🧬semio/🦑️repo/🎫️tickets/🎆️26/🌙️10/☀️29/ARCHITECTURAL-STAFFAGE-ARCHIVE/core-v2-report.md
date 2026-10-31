# Core v2 report (core worker, session 2)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Commands and API: [core-v2-status.md](core-v2-status.md). Results: [check-v2-results.md](check-v2-results.md), [coverage-matrix.md](coverage-matrix.md). Generated outputs (not committed, to be deleted at ticket close): `T/🗑️generated/core/` (proof sheets `proof/`, `check-v2.json`, `coverage-matrix.csv`, logs).

## What changed

**Schema / validation (`🧬️schema/🔣️.json`, `🗂️catalog`)**
- `🔣️.json` regenerated from the TS twin (`T/🧰️scripts/core/generate-schema-v2.ts`): now includes `passengers`, `body.subject`, `hidden`.
- New `validateAsset` rules: `representation-copied` (identical shapes, finer drawing with shapes deleted, scaled copy; module `🖨️representation/🪞️copies`), `valid-range-overlap` (strict; a shared endpoint is allowed per ruling), `pen-units` (pens, `penMm`, hatch pen/spacing must be plausible paper millimetres), `limiting-size`, `required-missing` (absent `thermal.passengers` / `presentation.defaultScale` give an issue instead of crashing). Existing: all four scales decided, unavailable with reason/limiting feature/alternatives, default-pose extent ±2 % or 15 mm, every non-hidden region drawn, valid range contains its own scale.
- Check CLI returns a structured `CheckReport` (`issues`, `scales` with suitability); uses `solveThermalAt` (play = scrub). No `--no-physics` needed any more: thermal, structure and all seven views run on every asset.

**Print suitability (`🖨️representation`)**
- Measured on the visible ink of the evaluated paper geometry over all named poses: pens/hatch pens, hatch spacing, feature width (morphological opening), gap between visible outlines of different shapes (centreline distance − half pens), and negative-space slivers in the inked silhouette (raster, pitch ≤ minGap/6).
- Coordinator ruling applied: touching, crossing or ink-overlapping outlines are joins; only `0 < ink gap < min gap` is a violation; hidden-line removed outlines only.
- Resolution: exact → authored; custom inside a valid range → `derived` (bilingual label naming source drawing and range, paper size on the requested scale); outside → `unavailable` (supported scales + alternatives); measured violation → `unsuitable`; symbols resolved separately; `RepresentationError` when drawing is refused. Zoom: `viewportMm` only; nothing upstream takes a zoom (test).

**Depiction**
- Seven modes draw the selected representation (pens, hatches, merged `regions`); `Analysis.transport` + transport overlay hook in `depict`/`depictAll` (any view); `legendBlock` draws `conditions` and `assumptions` as their own lines; thermal comes from `solveThermalAt(asset, query, environmentTimeline ?? environment)` in depict, preview and check.
- Evaluation: `ThermalInputs.passengers` filled from pose/key `ThermalBinding.passengers`, blended like activity with breathing as a step (empty record without passengers).
- Preview CLI gained `--transport parcels,pathlines,streamlines,condensation`.

**Tooling (all in `T/🧰️scripts/core/`)**: `check-all.ts` (+ `summarize-check.py`), `suit-probe.ts`, `proof-sheet.ts` (A3 landscape per asset, four scales side by side at true paper size, per-column labels with availability and measured min stroke/gap/feature, 10 mm and 1 m references, 0–5 m scale bar; SVG, PNG and one PDF with every page; rows wrap, pages split), `coverage-matrix.ts` (asset × scale × 7 views × (named pose | clip at mid time): availability, thermal, transport emitters, overlay, svg, pdf, dxf; md + csv), `generate-sample-figure.ts`.

## Tests run (library vitest via `repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts`)

- Last complete run (vitest-5, before the passenger/legend edits): 47 files, **506 passed, 14 failed**, 1 skipped. All cases of this worker pass: representation-resolution (exact/derived/unavailable/unsuitable/symbol/zoom/labels), representation-copied, print-suitability (positive control, join vs gap, sliver, thin feature/pen/hatch), physical-invariance (bones, anchors, loads, thermal inputs, respiration, kinematics, thermal field, structure, transport identical for every selector; geometry differs between scales), seven-modes, asset-validation (63 cases), schema-conformance, clip-sampling, smooth-continuity, evaluation-state, paper-dimensions, line-hierarchy, legend-units, view-synchrony.
- Failures belonging to other owners: `🎨️thermal-legend` (float last-digit equality), `🧊️thermal-scenarios`, `⏳️thermal-transient` ×2, `💨️transport-field` (5 s timeout), `🎞️frame-timing` ×2 and `🗜️zip-reopen` (export tests draw `walking-commuter` at 1:100 under office-laser, which the measured suitability rejects: the tests must pass an unchecked profile or a suitable asset), plus condensation/parcel/vapour oracles under load.
- Final run after the passenger, `solveThermalAt` and legend edits (vitest-6): 48 files, **756 passed, 11 failed**, 1 skipped; every core case passes including the new passenger blend and legend conditions/assumptions cases. Remaining failures: `🌡️catalog-physics` dining-table/single-bed (thermal worker on new assets), condensation oracles ×2, thermal-scenarios, transport-field timeout, frame-timing ×2 and zip-reopen (export tests, commuter unsuitable at 1:100).
- Third-party oracles in the new cases: d3-scale (paper mm), vitest deep equality/`structuredClone` mutations, hand-computed exact distances of axis-aligned rectangles (suitability), ajv (schema), d3-ease/three (animation, existing).

## Check results (see check-v2-results.md)

73 assets present at the time of the run: 2 pass, 71 fail. Counts by code: `suitability-gap` 68 assets, `suitability-feature` 28, `required-missing` 18 (missing `presentation.defaultScale`), `representation-copied` 10, `suitability-stroke` 4, `clip` 4, `extent-width` 3, `region-undrawn` 3, others 1 each. Engine bugs found and fixed meanwhile: false gaps from joins (zero-gap rule), crash on absent `passengers`, 1:undefined message. Content defects are listed with positions in the table, nothing was weakened.

## Coverage matrix

10 184 rows (first run, 48 + new assets), 25 failing rows: `forklift` thermal view PDF (`#NaNNaNNaN` colour in a clip at mid time: thermal colour of a NaN value, thermal worker) and `panel-radiator` pose `off` thermal view SVG/PDF (`x.toFixed` of `null`: a null value in the thermal view labels, thermal worker). The matrix is regenerated at the end of the run.

## Images looked at

`T/🗑️generated/core/proof/proof-sheet-walking-commuter.png` (four scales side by side at true paper size, labels, references, scale bars correct) and `proof-sheet-city-bus.png` (row wrap, 1:50 drawing 240 mm wide, hatches, scale bars). Commuter at 1:500 is a 3.6 mm figure with a single-colour silhouette; the labels name the measured violations (gap, feature).

## Unresolved

- 71 of 73 assets fail the check; content workers must act on `check-v2-results.md` (sample positions in the summary section).
- Export worker tests need a profile or asset choice (see above).
- `passengers` blending is tested with a synthetic passenger; no authored asset has one yet.
