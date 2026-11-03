# R3 thermal report (W06)

Details and requirement matrix: `r3-thermal-model.md`. Requests: `r3-requests/thermal.md`.

## Finding
The thermal and transport library already met most requirements (world-space Lagrangian parcels, transpiration as vapour with latent flux, operating-state exhausts, seeded deterministic grid, breath emitters on head anchors, separate surface/radiation/sensible/latent/moisture quantities). Gaps were closed as below.

## Changed (S = `C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage`)
- New `S\🔨️modules\🌡️thermal\🛋️comfort\🟦️.ts`: clo/met conversions (ISO 9920/8996), relative air speed, ISO 7730:2005 PMV/PPD with validity limits, `clothingInsulationClo(asset)`, `bodyComfort`. Exported from `S\🔨️modules\🌡️thermal\🟦️.ts`.
- New test `S\🧪️tests\🛋️pmv-comfort\{🟦️.ts,🐍️.py,🥒️.feature}` + fixture `S\🧫️fixtures\🛋️pmv-comfort\🔣️.json` (oracle pythermalcomfort 4.8 `pmv_ppd_iso`, 11 cases, PMV 0.01, PPD 0.1).
- New test `S\🧪️tests\🫁️breathing-catalog\🟦️.ts`: breathing 12-16 bpm at rest, exhale >= inhale, breath emitters on head anchors, trees transpire not plume, equipment emitters name declared operating keys.
- `S\🔨️modules\🎨️depiction\💨️transport-view\🟦️.ts`: parcel legends carry the bilingual "explanatory visualization, not smoke or fog" assumption; condensation veil doc corrected (coordinator F5: ink 12 %, not white). Test added in `🧪️tests\💨️transport-view`.
- `S\🔨️modules\🎨️depiction\🌡️thermal-view\🟦️.ts`: legend range note states "Absolute values" (shared, fixed) vs "Comparative illustration" (auto, span), en/de. Test added in `🧪️tests\🎨️thermal-legend`.
- Scripts `T\🧰️r3-thermal\audit.ts` (catalog audit); outputs in `T\🗑️generated\r3-thermal`.

## Coordinator finding F6 (DILATION 120 vs 200): not applied
Installed pythermalcomfort 4.8 `two_nodes_gagge.py:255` has `c_dil = 120` (comment "ashrae says 50"). The library constant equals the oracle; 200 would break the oracle match. Kept, documented in the model file.

## Checks
`repo tools exec bun -- x vitest run --config q2-vitest-alias.config.ts thermal transport respiration pmv breathing phase leaf penman gagge`: 14 of 15 files pass, 317 of 318 tests. The one failure, `🌡️thermal-span` "accepts a positive span in ajv and in the twin" (`/casts/span` requires `scale`), is a concurrent cast-schema edit by another worker, unrelated to my changes. Python oracle run: `uv run ... python -I 🐍️.py --write` produced the fixture; check mode not rerun separately.

## Open
- Jogger, cyclist, worker-carrying-box default `patterns[0]` is an exertion pattern (poses bind explicitly; W08 may reorder if clip tracks are re-indexed).
- Full catalog sweep (`thermal-v2.md` defects: brake and engine-bay peak powers) not revisited.
- Full suite and React suite not run.

DONE r3-thermal
Evidence: 318 tests run (317 pass, 1 unrelated failure); new files listed above; no renders inspected.
