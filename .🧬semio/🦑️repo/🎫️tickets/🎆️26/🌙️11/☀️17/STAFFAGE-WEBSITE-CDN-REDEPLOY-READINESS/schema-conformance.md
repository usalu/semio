# Schema conformance of the staffage catalog

The test `schema-conformance` failed for 19 assets while `validateAsset` passed all 152. The schema was right in every class; the data was malformed and the validator too lax. No schema or twin change remained (a first extension for `slender`/`tags` on `{ of }` references was reverted: `solidsOf` documents that a reference is never `slender`, and the target already carries the same tags).

## Classes, decisions

| Class | Assets | Fix |
|---|---|---|
| thermal loft without `capStart`/`capEnd` (30 solids) | gas-boiler, hot-water-cylinder, kitchen-extractor-hood, pendant-lamp | data: both `true` (the tessellator treated absent as open; the closed solid is the intended thermal body; all 152 stay valid) |
| `slender` / `tags` on a reference (217 + 6) | tower-crane, oil-pumpjack, pendant-lamp, scissor-lift | data: removed (redundant, the body solid has them) |
| body solid with `region` / `layer` | wheel-loader, waste-bin, tealight | data: removed (only thermal solids bind a region) |
| `sections: null` on solids | cyclist, child-on-kick-scooter | data: removed |
| body revolve without `tone` (13) | oil-pumpjack | data: tone of the 2D shape of the same id, else 0.55 |
| structure solid without `role` | solar-thermal-collector (mass-headers) | data: `body` (like the 2D headers) |
| `sectionFaces: []` | charcoal-grill | data: removed |
| ids with a dot (`bow-1.50`, `post-2.75-l`, `crossmember-0.5`) | city-bus | data: hyphen, uniqueness checked |
| `sections: {}` on a representation | sliding-door 1:500 | data: removed |
| `closed` on rect shapes | height-adjustable-desk | data: removed |
| `followTip` on ellipse (ignored by evaluation) | person-on-crutches | data: removed |
| poly without `z`, `tone` | panel-radiator end-cap-far | data: z 3, tone 0.7 (as `panel-rear`) |

## Validator (schema and validator now agree)

New rules, each with a vector: `volume-field` (unknown, missing or null member per list and kind, reference rules), `volume-id` (slug), `volume-section-faces` (empty, duplicate, unknown) in `🔨️modules/🗂️catalog/🧊️volume`; `shape-field` and `section-empty` in `🔨️modules/🗂️catalog/🟦️.ts`. Vectors: `🧫️fixtures/🧊️volume-schema` (7 new), `🧫️fixtures/✅️asset-validation` (4 new).

## Gas hob

Burner region: `ratedPowerW` 2500 with `ratedPowerBasis` `fuel-input` (summary: flame crown of about 2.5 kW); manifest `powerW` 2500; `gas-hob` removed from the pending list in `🧪️tests/⚡️asset-power`.

## Result

See the final report in chat for test numbers.
