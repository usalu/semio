# Catalog audit: Ajv conformance and `validateAsset` over all catalog assets and asset fixtures

Ticket `ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`. Script: `catalog-audit.ts` (Ajv `strict`, `allErrors`, against `#/$defs/Asset` of `🧬️schema/🔣️.json`, plus the owned `validateAsset`). 152 catalog assets plus 4 asset fixtures (`👤️sample-figure`, `🦶️sparse-smooth-walk`, `🔄️continuous-rotation`, `🚶️reference-walker`) = 156 documents.

Ajv reports every branch of a failed `oneOf`; the script drops the branches whose `kind` or `status` discriminator already mismatches and the `oneOf` row that has a surviving branch error, and removes duplicates, so one defect is one row. The snapshot below is the first full run (raw rows in `🗑️generated/catalog-fix/audit-before.json`, deleted with the ticket's generated output; taken before any data edit); its filter still hid the `required: tone` rows of `tower-server` (7 rows) and a few other branch rows, which a later run with the corrected filter showed (listed at the end). Final state: `catalog-fix.md`.

Totals of the snapshot: 22068 rows (759 volume-related, 21309 other), 434 distinct (asset, rule, pointer pattern, message) groups.

## Rows by rule

| scope | source | rule / keyword | rows |
|---|---|---|---|
| other | ajv | pattern | 15228 |
| other | ajv | oneOf | 2292 |
| other | ajv | required | 1833 |
| other | ajv | propertyNames | 1260 |
| volume | ajv | pattern | 666 |
| other | semantic | solid-sweep | 365 |
| other | ajv | additionalProperties | 176 |
| volume | ajv | oneOf | 93 |
| other | ajv | type | 40 |
| other | semantic | extent-front-height | 26 |
| other | semantic | lateral-ambiguous | 19 |
| other | ajv | minimum | 14 |
| other | semantic | extent-plan-width | 12 |
| other | semantic | extent-front-depth | 12 |
| other | semantic | extent-plan-depth | 12 |
| other | ajv | exclusiveMinimum | 10 |
| other | semantic | extent-width | 4 |
| other | semantic | representation-copied | 2 |
| other | semantic | extent-height | 2 |
| other | semantic | unavailable-but-drawable | 2 |

## Findings outside `volume`

| asset | source | rule | pointer (pattern) | message | rows | first pointer |
|---|---|---|---|---|---|---|
| mvhr-unit | ajv | minimum | `/thermal/emitters/N/vapourKgPerKg` | must be >= 0 | 1 | `/thermal/emitters/1/vapourKgPerKg` |
| tower-server | ajv | required | `/representations/N/geometry/elevation/N` | must have required property 'tone' | 5 | `/representations/50/geometry/elevation/7` |
| tower-server | ajv | required | `/representations/N/geometry/front/N` | must have required property 'tone' | 1 | `/representations/50/geometry/front/6` |
| tower-server | ajv | required | `/representations/N/geometry/plan/N` | must have required property 'tone' | 1 | `/representations/50/geometry/plan/4` |
| split-ac-indoor-unit | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 2 | `/thermal/regions/0/operating/heatW` |
| ceiling-diffuser | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 1 | `/thermal/regions/0/operating/heatW` |
| solid-brick-wall-1960 | semantic | representation-copied | `representations[N]` | The drawing 1:500 is a copy of 1:200 (identical: 3 of 3 shapes identical); every scale must be redrawn. | 1 | `representations.500` |
| etics-brick-wall-30 | semantic | representation-copied | `representations[N]` | The drawing 1:500 is a copy of 1:200 (identical: 5 of 5 shapes identical); every scale must be redrawn. | 1 | `representations.500` |
| cafe-chair | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 40 | `/representations/50/geometry/plan/0/solid` |
| cafe-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-band" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.500.geometry.plan[0].solid.sweep` |
| cafe-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "back-panel" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 4 | `representations.500.geometry.plan[5].solid.sweep` |
| cafe-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 1 | `representations.50.geometry.plan[0].solid.sweep` |
| cafe-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:500 plan (top view) differs from the real dimension N m by more t | 1 | `representations.500.geometry.plan` |
| cafe-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:200 plan (top view) differs from the real dimension N m by more t | 1 | `representations.200.geometry.plan` |
| cafe-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:100 plan (top view) differs from the real dimension N m by more t | 1 | `representations.100.geometry.plan` |
| cafe-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:50 plan (top view) differs from the real dimension N m by more th | 1 | `representations.50.geometry.plan` |
| operating-table | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 80 | `/representations/50/geometry/front/0/solid` |
| operating-table | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 60 | `/representations/50/geometry/plan/0/solid` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is  | 2 | `representations.500.geometry.front[0].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "top" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is  | 2 | `representations.500.geometry.plan[0].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top-end-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.front[1].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top-end-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.front[2].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 2 | `representations.100.geometry.front[0].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "carrier-end-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 2 | `representations.100.geometry.front[3].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "carrier-end-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 2 | `representations.100.geometry.front[4].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top-carrier" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 2 | `representations.100.geometry.front[19].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "top-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 2 | `representations.100.geometry.plan[0].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "top-carrier" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 2 | `representations.100.geometry.plan[1].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "pad-joint-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.100.geometry.plan[6].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "pad-joint-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 2 | `representations.100.geometry.plan[7].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "pad-joint-2" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 2 | `representations.100.geometry.plan[8].solid.sweep` |
| operating-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "pad-joint-3" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.50.geometry.plan[11].solid.sweep` |
| three-seat-sofa | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/representations/200/geometry/elevation/3/id` |
| three-seat-sofa | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 2 | `/representations/200/geometry/elevation/3` |
| three-seat-sofa | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/representations/200/geometry/plan/6/id` |
| three-seat-sofa | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 2 | `/representations/200/geometry/plan/6` |
| bar-stool | ajv | required | `/representations/N/geometry/elevation/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/elevation/9/solid` |
| bar-stool | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/front/2/solid` |
| bar-stool | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/plan/2/solid` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "back" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 1 | `representations.500.geometry.elevation[5].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "back" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 1 | `representations.500.geometry.front[1].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "back" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 1 | `representations.500.geometry.plan[1].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "back-post" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.elevation[5].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "back-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.elevation[6].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "back-post" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.front[1].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "back-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.front[2].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "back-post" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.plan[1].solid.sweep` |
| bar-stool | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "back-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.plan[2].solid.sweep` |
| single-bed | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 90 | `/representations/50/geometry/front/0/solid` |
| single-bed | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 60 | `/representations/50/geometry/plan/0/solid` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "mat-head" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 4 | `representations.500.geometry.front[0].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "mat-mid" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.front[1].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "mat-foot" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 4 | `representations.500.geometry.front[2].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "mat-head" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 4 | `representations.500.geometry.plan[0].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "mat-mid" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.plan[1].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "mat-foot" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 4 | `representations.500.geometry.plan[2].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "slats-head" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 2 | `representations.100.geometry.front[3].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "slats-mid" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 2 | `representations.100.geometry.front[4].solid.sweep` |
| single-bed | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "slats-foot" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 2 | `representations.100.geometry.front[5].solid.sweep` |
| single-bed | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| single-bed | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| single-bed | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| single-bed | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| folding-deck-chair | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 200 | `/representations/50/geometry/front/0/solid` |
| folding-deck-chair | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 200 | `/representations/50/geometry/plan/0/solid` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "rear-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.front[0].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "front-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.front[1].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.front[2].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "rear-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.front[3].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "front-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.front[4].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.front[5].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "canvas-back" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.500.geometry.front[6].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "canvas-seat" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.500.geometry.front[7].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "rear-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.plan[0].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "front-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.plan[1].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.plan[2].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "rear-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.plan[3].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "front-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.plan[4].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.plan[5].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "canvas-back" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.500.geometry.plan[6].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "canvas-seat" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.500.geometry.plan[7].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "canvas-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.front[6].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "canvas-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.front[7].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "canvas-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.plan[6].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "canvas-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 3 | `representations.200.geometry.plan[7].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "bolt-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 2 | `representations.100.geometry.front[8].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "bolt-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 2 | `representations.100.geometry.front[9].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "top-rail" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 2 | `representations.100.geometry.front[10].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "front-rail" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 2 | `representations.100.geometry.front[11].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "bolt-0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 2 | `representations.100.geometry.plan[8].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "bolt-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 2 | `representations.100.geometry.plan[9].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "top-rail" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 2 | `representations.100.geometry.plan[10].solid.sweep` |
| folding-deck-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "front-rail" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 2 | `representations.100.geometry.plan[11].solid.sweep` |
| folding-deck-chair | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:500 front elevation differs from the real dimension N m by more th | 1 | `representations.500.geometry.front` |
| folding-deck-chair | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| folding-deck-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:500 plan (top view) differs from the real dimension N m by more t | 1 | `representations.500.geometry.plan` |
| folding-deck-chair | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:500 plan (top view) differs from the real dimension N m by more th | 1 | `representations.500.geometry.plan` |
| folding-deck-chair | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:200 front elevation differs from the real dimension N m by more th | 1 | `representations.200.geometry.front` |
| folding-deck-chair | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| folding-deck-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:200 plan (top view) differs from the real dimension N m by more t | 1 | `representations.200.geometry.plan` |
| folding-deck-chair | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:200 plan (top view) differs from the real dimension N m by more th | 1 | `representations.200.geometry.plan` |
| folding-deck-chair | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:100 front elevation differs from the real dimension N m by more th | 1 | `representations.100.geometry.front` |
| folding-deck-chair | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| folding-deck-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:100 plan (top view) differs from the real dimension N m by more t | 1 | `representations.100.geometry.plan` |
| folding-deck-chair | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:100 plan (top view) differs from the real dimension N m by more th | 1 | `representations.100.geometry.plan` |
| folding-deck-chair | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:50 front elevation differs from the real dimension N m by more tha | 1 | `representations.50.geometry.front` |
| folding-deck-chair | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| folding-deck-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:50 plan (top view) differs from the real dimension N m by more th | 1 | `representations.50.geometry.plan` |
| folding-deck-chair | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:50 plan (top view) differs from the real dimension N m by more tha | 1 | `representations.50.geometry.plan` |
| office-swivel-chair | ajv | required | `/representations/N/geometry/elevation/N/solid` | must have required property 'sweep' | 160 | `/representations/50/geometry/elevation/25/solid` |
| office-swivel-chair | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 36 | `/representations/50/geometry/elevation/32/id` |
| office-swivel-chair | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 120 | `/representations/50/geometry/front/18/solid` |
| office-swivel-chair | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 30 | `/representations/50/geometry/front/25/id` |
| office-swivel-chair | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 85 | `/representations/50/geometry/plan/17/solid` |
| office-swivel-chair | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 24 | `/representations/50/geometry/plan/22/id` |
| office-swivel-chair | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 1 | `/representations/200/geometry/plan/11` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-band" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.500.geometry.elevation[5].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 1 | `representations.500.geometry.elevation[6].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-band" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.500.geometry.front[7].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "backrest" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 1 | `representations.500.geometry.front[8].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-band" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.500.geometry.plan[10].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "backrest" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 1 | `representations.500.geometry.plan[11].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-cushion" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 3 | `representations.200.geometry.elevation[6].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest-shell" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 3 | `representations.200.geometry.elevation[7].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest-strut" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 3 | `representations.200.geometry.elevation[8].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.200.geometry.elevation[9].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.200.geometry.elevation[10].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "backrest-strut" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 3 | `representations.200.geometry.front[13].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.200.geometry.front[14].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.200.geometry.front[15].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-cushion" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 3 | `representations.200.geometry.front[16].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "backrest-shell" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 3 | `representations.200.geometry.front[17].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-cushion" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 3 | `representations.200.geometry.plan[12].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "backrest-shell" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 3 | `representations.200.geometry.plan[13].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-mechanism" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.elevation[7].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-post-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.elevation[10].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-post--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 2 | `representations.100.geometry.elevation[11].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-pad-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 2 | `representations.100.geometry.elevation[12].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-pad--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.elevation[13].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest-post-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.front[14].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest-post--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 2 | `representations.100.geometry.front[15].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-mechanism" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.front[17].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest-pad-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 2 | `representations.100.geometry.front[19].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "armrest-pad--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.front[20].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat-mechanism" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.plan[11].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "armrest-pad-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 2 | `representations.100.geometry.plan[13].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "armrest-pad--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 2 | `representations.100.geometry.plan[14].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-shell" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.50.geometry.elevation[26].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-piping" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 1 | `representations.50.geometry.elevation[27].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-fabric-fold" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone norma | 1 | `representations.50.geometry.elevation[28].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "seat-lever" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.50.geometry.elevation[30].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-line-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 1 | `representations.50.geometry.elevation[35].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "armrest-line--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[36].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest-pad" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 1 | `representations.50.geometry.elevation[39].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest-rim" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 1 | `representations.50.geometry.elevation[40].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "backrest-texture" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone norma | 1 | `representations.50.geometry.elevation[41].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat-piping-ring" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone norma | 1 | `representations.50.geometry.front[18].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "backrest-strut" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 1 | `representations.50.geometry.plan[20].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "armrest-post-1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal) | 1 | `representations.50.geometry.plan[21].solid.sweep` |
| office-swivel-chair | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "armrest-post--1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.plan[22].solid.sweep` |
| office-swivel-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:500 plan (top view) differs from the real dimension N m by more t | 1 | `representations.500.geometry.plan` |
| office-swivel-chair | semantic | extent-width | `representations[N].geometry` | The default-pose width N m of the 1:500 drawing differs from dimensions.widthM N m by more than max(2 % of the | 1 | `representations.500.geometry` |
| office-swivel-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:200 plan (top view) differs from the real dimension N m by more t | 1 | `representations.200.geometry.plan` |
| office-swivel-chair | semantic | extent-width | `representations[N].geometry` | The default-pose width N m of the 1:200 drawing differs from dimensions.widthM N m by more than max(2 % of the | 1 | `representations.200.geometry` |
| office-swivel-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:100 plan (top view) differs from the real dimension N m by more t | 1 | `representations.100.geometry.plan` |
| office-swivel-chair | semantic | extent-width | `representations[N].geometry` | The default-pose width N m of the 1:100 drawing differs from dimensions.widthM N m by more than max(2 % of the | 1 | `representations.100.geometry` |
| office-swivel-chair | semantic | extent-plan-width | `representations[N].geometry.plan` | The default-pose forward length N m of the 1:50 plan (top view) differs from the real dimension N m by more th | 1 | `representations.50.geometry.plan` |
| office-swivel-chair | semantic | extent-width | `representations[N].geometry` | The default-pose width N m of the 1:50 drawing differs from dimensions.widthM N m by more than max(2 % of the  | 1 | `representations.50.geometry` |
| picnic-table | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 40 | `/representations/50/geometry/front/0/solid` |
| picnic-table | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 40 | `/representations/50/geometry/plan/0/solid` |
| picnic-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "bench-r" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.front[0].solid.sweep` |
| picnic-table | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "bench-l" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.front[1].solid.sweep` |
| picnic-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "bench-r" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.plan[0].solid.sweep` |
| picnic-table | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "bench-l" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there | 4 | `representations.500.geometry.plan[1].solid.sweep` |
| picnic-table | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| picnic-table | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| picnic-table | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| picnic-table | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| multi-stem-birch | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 6 | `/representations/50/geometry/elevation/56` |
| miscanthus-grass | ajv | pattern | `/rig/bones/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/rig/bones/13/id` |
| miscanthus-grass | ajv | pattern | `/rig/bones/N/parent` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 6 | `/rig/bones/14/parent` |
| miscanthus-grass | ajv | type | `/rig/bones/N/parent` | must be null | 6 | `/rig/bones/14/parent` |
| miscanthus-grass | ajv | pattern | `/anchors/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/anchors/1/bone` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 972 | `/representations/50/geometry/elevation/26/id` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/elevation/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 972 | `/representations/50/geometry/elevation/26/bone` |
| miscanthus-grass | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 90 | `/representations/50/geometry/elevation/26` |
| miscanthus-grass | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 80 | `/representations/50/geometry/elevation/80` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 720 | `/representations/50/geometry/front/1/id` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/front/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 720 | `/representations/50/geometry/front/1/bone` |
| miscanthus-grass | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 120 | `/representations/50/geometry/front/1` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 720 | `/representations/50/geometry/plan/1/id` |
| miscanthus-grass | ajv | pattern | `/representations/N/geometry/plan/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 804 | `/representations/50/geometry/plan/1/bone` |
| miscanthus-grass | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 134 | `/representations/50/geometry/plan/1` |
| miscanthus-grass | ajv | pattern | `/poses/N/loads/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 24 | `/poses/0/loads/0/id` |
| miscanthus-grass | ajv | pattern | `/poses/N/loads/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 24 | `/poses/0/loads/0/bone` |
| miscanthus-grass | ajv | pattern | `/poses/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 30 | `/poses/1/bones` |
| miscanthus-grass | ajv | propertyNames | `/poses/N/bones` | property name must be valid | 30 | `/poses/1/bones` |
| miscanthus-grass | ajv | pattern | `/clips/N/keys/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 180 | `/clips/0/keys/0/bones` |
| miscanthus-grass | ajv | propertyNames | `/clips/N/keys/N/bones` | property name must be valid | 180 | `/clips/0/keys/0/bones` |
| miscanthus-grass | ajv | pattern | `/clips/N/tracks/N/target` | must match pattern "^(activityMet/externalWorkW/breathing/emitter:[a-z0-9-]+/operating:[a-z0-9-]+/load:[a-z0-9 | 6 | `/clips/1/tracks/0/target` |
| weeping-willow | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 10 | `/representations/50/geometry/elevation/54` |
| lombardy-poplar | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 6 | `/representations/50/geometry/elevation/52` |
| ivy-on-trellis | ajv | pattern | `/rig/bones/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 4 | `/rig/bones/1/id` |
| ivy-on-trellis | ajv | pattern | `/anchors/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 4 | `/anchors/0/id` |
| ivy-on-trellis | ajv | pattern | `/anchors/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 4 | `/anchors/0/bone` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 84 | `/representations/50/geometry/elevation/0/id` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/elevation/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 84 | `/representations/50/geometry/elevation/0/bone` |
| ivy-on-trellis | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 14 | `/representations/50/geometry/elevation/0` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 84 | `/representations/50/geometry/front/0/id` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/front/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 84 | `/representations/50/geometry/front/0/bone` |
| ivy-on-trellis | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 14 | `/representations/50/geometry/front/0` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 72 | `/representations/50/geometry/plan/0/id` |
| ivy-on-trellis | ajv | pattern | `/representations/N/geometry/plan/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 72 | `/representations/50/geometry/plan/0/bone` |
| ivy-on-trellis | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 12 | `/representations/50/geometry/plan/0` |
| ivy-on-trellis | ajv | pattern | `/poses/N/loads/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 6 | `/poses/0/loads/8/id` |
| ivy-on-trellis | ajv | pattern | `/poses/N/loads/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 6 | `/poses/0/loads/8/bone` |
| ivy-on-trellis | ajv | pattern | `/poses/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 6 | `/poses/2/bones` |
| ivy-on-trellis | ajv | propertyNames | `/poses/N/bones` | property name must be valid | 6 | `/poses/2/bones` |
| ivy-on-trellis | ajv | pattern | `/clips/N/keys/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 38 | `/clips/0/keys/0/bones` |
| ivy-on-trellis | ajv | propertyNames | `/clips/N/keys/N/bones` | property name must be valid | 38 | `/clips/0/keys/0/bones` |
| ivy-on-trellis | ajv | pattern | `/thermal/emitters/N/anchors/N` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 2 | `/thermal/emitters/0/anchors/4` |
| ivy-on-trellis | ajv | pattern | `/structure/supports/N` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 2 | `/structure/supports/0` |
| date-palm | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 22 | `/representations/50/geometry/elevation/21` |
| date-palm | ajv | additionalProperties | `/representations/N/geometry/front/N` | must NOT have additional properties | 16 | `/representations/50/geometry/front/34` |
| date-palm | ajv | additionalProperties | `/representations/N/geometry/plan/N` | must NOT have additional properties | 16 | `/representations/50/geometry/plan/7` |
| date-palm | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 18 | `/representations/200/geometry/elevation/3/id` |
| date-palm | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 3 | `/representations/200/geometry/elevation/3` |
| english-oak | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 8 | `/representations/50/geometry/elevation/63` |
| hosta-clump | ajv | pattern | `/rig/bones/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 18 | `/rig/bones/1/id` |
| hosta-clump | ajv | pattern | `/rig/bones/N/parent` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 9 | `/rig/bones/2/parent` |
| hosta-clump | ajv | type | `/rig/bones/N/parent` | must be null | 9 | `/rig/bones/2/parent` |
| hosta-clump | ajv | pattern | `/anchors/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 9 | `/anchors/1/bone` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/elevation/0/id` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/elevation/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/elevation/0/bone` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/elevation/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 135 | `/representations/50/geometry/elevation/0/solid/axisBone` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/elevation/N/solid` | must match exactly one schema in oneOf | 135 | `/representations/50/geometry/elevation/0/solid` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 45 | `/representations/50/geometry/elevation/0` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/front/0/id` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/front/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/front/0/bone` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/front/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 135 | `/representations/50/geometry/front/0/solid/axisBone` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/front/N/solid` | must match exactly one schema in oneOf | 135 | `/representations/50/geometry/front/0/solid` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 45 | `/representations/50/geometry/front/0` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/plan/0/id` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/plan/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/plan/0/bone` |
| hosta-clump | ajv | pattern | `/representations/N/geometry/plan/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 135 | `/representations/50/geometry/plan/0/solid/axisBone` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/plan/N/solid` | must match exactly one schema in oneOf | 135 | `/representations/50/geometry/plan/0/solid` |
| hosta-clump | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 45 | `/representations/50/geometry/plan/0` |
| hosta-clump | ajv | pattern | `/poses/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 63 | `/poses/0/bones` |
| hosta-clump | ajv | propertyNames | `/poses/N/bones` | property name must be valid | 63 | `/poses/0/bones` |
| hosta-clump | ajv | pattern | `/poses/N/loads/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 27 | `/poses/0/loads/0/id` |
| hosta-clump | ajv | pattern | `/poses/N/loads/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 27 | `/poses/0/loads/0/bone` |
| hosta-clump | ajv | pattern | `/clips/N/keys/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 324 | `/clips/0/keys/0/bones` |
| hosta-clump | ajv | propertyNames | `/clips/N/keys/N/bones` | property name must be valid | 324 | `/clips/0/keys/0/bones` |
| hosta-clump | semantic | extent-height | `representations[N].geometry` | The default-pose height N m of the 1:100 drawing differs from dimensions.heightM N m by more than max(2 % of t | 1 | `representations.100.geometry` |
| hosta-clump | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| hosta-clump | semantic | extent-height | `representations[N].geometry` | The default-pose height N m of the 1:50 drawing differs from dimensions.heightM N m by more than max(2 % of th | 1 | `representations.50.geometry` |
| hosta-clump | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| bamboo-clump | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 348 | `/representations/50/geometry/elevation/42/id` |
| bamboo-clump | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 58 | `/representations/50/geometry/elevation/42` |
| bamboo-clump | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/representations/500/geometry/front/1/id` |
| bamboo-clump | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 2 | `/representations/500/geometry/front/1` |
| bamboo-clump | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 12 | `/representations/500/geometry/plan/0/id` |
| bamboo-clump | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 2 | `/representations/500/geometry/plan/0` |
| scots-pine | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 4 | `/representations/50/geometry/elevation/47` |
| agave | ajv | pattern | `/rig/bones/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 19 | `/rig/bones/2/id` |
| agave | ajv | pattern | `/rig/bones/N/parent` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 8 | `/rig/bones/3/parent` |
| agave | ajv | type | `/rig/bones/N/parent` | must be null | 8 | `/rig/bones/3/parent` |
| agave | ajv | pattern | `/anchors/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 13 | `/anchors/1/bone` |
| agave | ajv | pattern | `/representations/N/geometry/elevation/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 666 | `/representations/50/geometry/elevation/1/id` |
| agave | ajv | pattern | `/representations/N/geometry/elevation/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 672 | `/representations/50/geometry/elevation/1/bone` |
| agave | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 112 | `/representations/50/geometry/elevation/1` |
| agave | ajv | pattern | `/representations/N/geometry/elevation/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/elevation/2/solid/axisBone` |
| agave | ajv | oneOf | `/representations/N/geometry/elevation/N/solid` | must match exactly one schema in oneOf | 270 | `/representations/50/geometry/elevation/2/solid` |
| agave | ajv | pattern | `/representations/N/geometry/front/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 672 | `/representations/50/geometry/front/4/id` |
| agave | ajv | pattern | `/representations/N/geometry/front/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 678 | `/representations/50/geometry/front/4/bone` |
| agave | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 113 | `/representations/50/geometry/front/4` |
| agave | ajv | pattern | `/representations/N/geometry/front/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/front/6/solid/axisBone` |
| agave | ajv | oneOf | `/representations/N/geometry/front/N/solid` | must match exactly one schema in oneOf | 270 | `/representations/50/geometry/front/6/solid` |
| agave | ajv | pattern | `/representations/N/geometry/plan/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 678 | `/representations/50/geometry/plan/0/bone` |
| agave | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 113 | `/representations/50/geometry/plan/0` |
| agave | ajv | pattern | `/representations/N/geometry/plan/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 648 | `/representations/50/geometry/plan/2/id` |
| agave | ajv | pattern | `/representations/N/geometry/plan/N/solid/axisBone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 270 | `/representations/50/geometry/plan/2/solid/axisBone` |
| agave | ajv | oneOf | `/representations/N/geometry/plan/N/solid` | must match exactly one schema in oneOf | 270 | `/representations/50/geometry/plan/2/solid` |
| agave | ajv | pattern | `/poses/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 77 | `/poses/0/bones` |
| agave | ajv | propertyNames | `/poses/N/bones` | property name must be valid | 77 | `/poses/0/bones` |
| agave | ajv | pattern | `/poses/N/loads/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 33 | `/poses/0/loads/0/id` |
| agave | ajv | pattern | `/poses/N/loads/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 33 | `/poses/0/loads/0/bone` |
| agave | ajv | pattern | `/clips/N/keys/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 342 | `/clips/0/keys/0/bones` |
| agave | ajv | propertyNames | `/clips/N/keys/N/bones` | property name must be valid | 342 | `/clips/0/keys/0/bones` |
| heat-pump-outdoor-unit | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 1 | `/thermal/regions/1/operating/heatW` |
| heat-pump-outdoor-unit | ajv | minimum | `/thermal/emitters/N/vapourKgPerKg` | must be >= 0 | 1 | `/thermal/emitters/0/vapourKgPerKg` |
| scissor-lift | ajv | pattern | `/rig/bones/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 10 | `/rig/bones/3/id` |
| scissor-lift | ajv | pattern | `/rig/bones/N/parent` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 9 | `/rig/bones/6/parent` |
| scissor-lift | ajv | type | `/rig/bones/N/parent` | must be null | 9 | `/rig/bones/6/parent` |
| scissor-lift | ajv | pattern | `/representations/N/geometry/elevation/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 300 | `/representations/50/geometry/elevation/0/bone` |
| scissor-lift | ajv | oneOf | `/representations/N/geometry/elevation/N` | must match exactly one schema in oneOf | 50 | `/representations/50/geometry/elevation/0` |
| scissor-lift | ajv | pattern | `/representations/N/geometry/front/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 360 | `/representations/50/geometry/front/3/bone` |
| scissor-lift | ajv | oneOf | `/representations/N/geometry/front/N` | must match exactly one schema in oneOf | 60 | `/representations/50/geometry/front/3` |
| scissor-lift | ajv | pattern | `/representations/N/geometry/plan/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 240 | `/representations/50/geometry/plan/5/bone` |
| scissor-lift | ajv | oneOf | `/representations/N/geometry/plan/N` | must match exactly one schema in oneOf | 40 | `/representations/50/geometry/plan/5` |
| scissor-lift | ajv | pattern | `/poses/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 40 | `/poses/0/bones` |
| scissor-lift | ajv | propertyNames | `/poses/N/bones` | property name must be valid | 40 | `/poses/0/bones` |
| scissor-lift | ajv | type | `/poses/N/thermal/emitters/motor-vent` | must be number | 4 | `/poses/0/thermal/emitters/motor-vent` |
| scissor-lift | ajv | type | `/poses/N/thermal/emitters/battery-plume` | must be number | 4 | `/poses/0/thermal/emitters/battery-plume` |
| scissor-lift | ajv | pattern | `/clips/N/keys/N/bones` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 160 | `/clips/0/keys/0/bones` |
| scissor-lift | ajv | propertyNames | `/clips/N/keys/N/bones` | property name must be valid | 160 | `/clips/0/keys/0/bones` |
| excavator | ajv | exclusiveMinimum | `/representations/N/geometry/plan/N/w` | must be > 0 | 6 | `/representations/50/geometry/plan/21/w` |
| scroll-compressor | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 1 | `/thermal/regions/3/operating/heatW` |
| expansion-valve | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 4 | `/thermal/regions/0/operating/heatW` |
| solar-thermal-collector | ajv | minimum | `/thermal/regions/N/operating/heatW` | must be >= 0 | 1 | `/thermal/regions/2/operating/heatW` |
| kneeling-gardener | semantic | unavailable-but-drawable | `representations[N].status` | The asset measures N mm on paper at 1:500, at least N mm: draw it, or offer a true-extent symbol that has all  | 1 | `representations.500.status` |
| kneeling-gardener | semantic | unavailable-but-drawable | `representations[N].status` | The asset measures N mm on paper at 1:200, at least N mm: draw it, or offer a true-extent symbol that has all  | 1 | `representations.200.status` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 11 flat shape(s) (figure, sil-body, torso, badge, jacket-zip, lanyard, ...) on bone "torso" would be drawn up  | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 4 flat shape(s) (upper-arm-l, sil-arm-l, sleeve-l, upper-arm-l-shoulder) on bone "upper-arm-l" would be drawn  | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 4 flat shape(s) (forearm-l, cuff-l, forearm-l-elbow-0, forearm-l-elbow-1) on bone "forearm-l" would be drawn u | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 3 flat shape(s) (upper-arm-r, sleeve-r, upper-arm-r-shoulder) on bone "upper-arm-r" would be drawn up to N mm  | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 5 flat shape(s) (forearm-r, wrist-skin-r, cuff-r, forearm-r-elbow-0, forearm-r-elbow-1) on bone "forearm-r" wo | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 2 flat shape(s) (pole, pole-finial) on bone "flag-pole" would be drawn up to N mm wrong on paper (above the N  | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 3 flat shape(s) (flag, flag-emblem, flag-stripe) on bone "flag" would be drawn up to N mm wrong on paper (abov | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 2 flat shape(s) (pointer, pointer-tip) on bone "pointer" would be drawn up to N mm wrong on paper (above the N | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 28 flat shape(s) (sil-head, head, hair, neck, cap, shoulders, ...) on bone "head" would be drawn up to N mm wr | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 9 flat shape(s) (hand-l, pole-grip, thumb-l, fingers-l-0, fingers-l-1, fingers-l-2, ...) on bone "hand-l" woul | 1 | `representations` |
| tour-guide | semantic | lateral-ambiguous | `representations` | 8 flat shape(s) (hand-r, thumb-r, fingers-r-0, fingers-r-1, fingers-r-2, finger-r-0, ...) on bone "hand-r" wou | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 4 flat shape(s) (sil-arm-l, upper-arm-l, sleeve-l, shoulder-l) on bone "arm-l" would be drawn up to N mm wrong | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 5 flat shape(s) (sil-forearm-l, forearm-l, elbow-l, cuff-l, cuff-rib-l) on bone "forearm-l" would be drawn up  | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 4 flat shape(s) (sil-arm-r, upper-arm-r, sleeve-r, shoulder-r) on bone "arm-r" would be drawn up to N mm wrong | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 5 flat shape(s) (sil-forearm-r, forearm-r, elbow-r, cuff-r, cuff-rib-r) on bone "forearm-r" would be drawn up  | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 6 flat shape(s) (hand-l, fist-l, thumb-l, finger-l-0, finger-l-1, finger-l-2) on bone "hand-l" would be drawn  | 1 | `representations` |
| pregnant-person | semantic | lateral-ambiguous | `representations` | 6 flat shape(s) (hand-r, fist-r, thumb-r, finger-r-0, finger-r-1, finger-r-2) on bone "hand-r" would be drawn  | 1 | `representations` |
| sleeping-person | semantic | lateral-ambiguous | `representations` | 2 flat shape(s) (sil-foot-l, foot-l) on bone "foot-l" would be drawn up to N mm wrong on paper (above the N mm | 1 | `representations` |
| sleeping-person | semantic | lateral-ambiguous | `representations` | 2 flat shape(s) (sil-foot-r, foot-r) on bone "foot-r" would be drawn up to N mm wrong on paper (above the N mm | 1 | `representations` |
| bicycle-stand | ajv | minimum | `/clips/N/fps` | must be >= 1 | 1 | `/clips/1/fps` |
| charcoal-grill | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/front/13/solid` |
| charcoal-grill | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 20 | `/representations/50/geometry/plan/24/solid` |
| charcoal-grill | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "handle" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.front[6].solid.sweep` |
| charcoal-grill | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "handle" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there  | 4 | `representations.500.geometry.plan[6].solid.sweep` |
| charcoal-grill | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "vent-dial" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 3 | `representations.200.geometry.front[7].solid.sweep` |
| market-parasol | ajv | required | `/representations/N/geometry/elevation/N/solid` | must have required property 'sweep' | 90 | `/representations/50/geometry/elevation/22/solid` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-b-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[22].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-b-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[23].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-b-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[24].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-c-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[26].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-c-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[27].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-c-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[28].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-d-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[30].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-d-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[31].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-d-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[32].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-f-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[38].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-f-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[39].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-f-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[40].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-g-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[42].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-g-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[43].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-g-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[44].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-h-tip" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 1 | `representations.50.geometry.elevation[46].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-h-scallop-a" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[47].solid.sweep` |
| market-parasol | semantic | solid-sweep | `representations[N].geometry.elevation[N].solid.sweep` | Shape "rib-h-scallop-b" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal | 1 | `representations.50.geometry.elevation[48].solid.sweep` |
| chalkboard | ajv | additionalProperties | `/representations/N/geometry/elevation/N` | must NOT have additional properties | 8 | `/representations/50/geometry/elevation/17` |
| playground-swing | ajv | exclusiveMinimum | `/representations/N/geometry/front/N/w` | must be > 0 | 2 | `/representations/50/geometry/front/13/w` |
| playground-swing | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 70 | `/representations/50/geometry/front/17/solid` |
| playground-swing | ajv | exclusiveMinimum | `/representations/N/geometry/plan/N/h` | must be > 0 | 2 | `/representations/50/geometry/plan/13/h` |
| playground-swing | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 60 | `/representations/50/geometry/plan/15/solid` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "chain-left" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 4 | `representations.500.geometry.front[3].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "chain-right" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 4 | `representations.500.geometry.front[4].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "seat" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 4 | `representations.500.geometry.front[5].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "chain-left" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 4 | `representations.500.geometry.plan[9].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "chain-right" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), t | 4 | `representations.500.geometry.plan[10].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "seat" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 4 | `representations.500.geometry.plan[11].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "shackle-left" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 1 | `representations.50.geometry.front[19].solid.sweep` |
| playground-swing | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "shackle-right" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 1 | `representations.50.geometry.front[20].solid.sweep` |
| drinking-fountain | ajv | minimum | `/clips/N/fps` | must be >= 1 | 1 | `/clips/1/fps` |
| waste-bin | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 40 | `/representations/50/geometry/front/17/solid` |
| waste-bin | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 120 | `/representations/50/geometry/plan/6/solid` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "lid" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is  | 4 | `representations.500.geometry.front[1].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-a0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 4 | `representations.500.geometry.plan[1].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-b0" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 4 | `representations.500.geometry.plan[2].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-a1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 3 | `representations.500.geometry.plan[3].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-b1" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 3 | `representations.500.geometry.plan[4].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-span" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), ther | 4 | `representations.500.geometry.plan[5].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-a2" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 1 | `representations.200.geometry.plan[6].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-corner-b2" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), | 1 | `representations.200.geometry.plan[7].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "lid-skirt" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 2 | `representations.100.geometry.front[10].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-skirt" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), the | 2 | `representations.100.geometry.plan[8].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "lid-handle" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.50.geometry.front[19].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "hinge-barrel" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 1 | `representations.50.geometry.front[20].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "lid-handle" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), th | 1 | `representations.50.geometry.plan[12].solid.sweep` |
| waste-bin | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "hinge-barrel" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal),  | 1 | `representations.50.geometry.plan[13].solid.sweep` |
| waste-bin | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| waste-bin | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| waste-bin | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| waste-bin | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| shopping-trolley | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 20 | `/representations/50/geometry/front/32/solid` |
| shopping-trolley | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 20 | `/representations/50/geometry/plan/21/solid` |
| shopping-trolley | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "gate" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 4 | `representations.500.geometry.front[7].solid.sweep` |
| shopping-trolley | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "gate" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is | 4 | `representations.500.geometry.plan[2].solid.sweep` |
| shopping-trolley | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:500 front elevation differs from the real dimension N m by more th | 1 | `representations.500.geometry.front` |
| shopping-trolley | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| shopping-trolley | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:500 plan (top view) differs from the real dimension N m by more th | 1 | `representations.500.geometry.plan` |
| shopping-trolley | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:200 front elevation differs from the real dimension N m by more th | 1 | `representations.200.geometry.front` |
| shopping-trolley | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| shopping-trolley | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:200 plan (top view) differs from the real dimension N m by more th | 1 | `representations.200.geometry.plan` |
| shopping-trolley | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:100 front elevation differs from the real dimension N m by more th | 1 | `representations.100.geometry.front` |
| shopping-trolley | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| shopping-trolley | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:100 plan (top view) differs from the real dimension N m by more th | 1 | `representations.100.geometry.plan` |
| shopping-trolley | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:50 front elevation differs from the real dimension N m by more tha | 1 | `representations.50.geometry.front` |
| shopping-trolley | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| shopping-trolley | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:50 plan (top view) differs from the real dimension N m by more tha | 1 | `representations.50.geometry.plan` |
| 🔄️continuous-rotation | ajv | required | `/representations/N/geometry/front/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/front/1/solid` |
| 🔄️continuous-rotation | ajv | required | `/representations/N/geometry/plan/N/solid` | must have required property 'sweep' | 35 | `/representations/50/geometry/plan/1/solid` |
| 🔄️continuous-rotation | ajv | required | `/thermal/environment` | must have required property 'diffuseFraction' | 1 | `/thermal/environment` |
| 🔄️continuous-rotation | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "rim" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is  | 4 | `representations.500.geometry.front[1].solid.sweep` |
| 🔄️continuous-rotation | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "rim" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is  | 4 | `representations.500.geometry.plan[1].solid.sweep` |
| 🔄️continuous-rotation | semantic | solid-sweep | `representations[N].geometry.front[N].solid.sweep` | Shape "spoke" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there i | 3 | `representations.200.geometry.front[2].solid.sweep` |
| 🔄️continuous-rotation | semantic | solid-sweep | `representations[N].geometry.plan[N].solid.sweep` | Shape "spoke" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there i | 3 | `representations.200.geometry.plan[2].solid.sweep` |
| 🔄️continuous-rotation | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:500 front elevation differs from the real dimension N m by more th | 1 | `representations.500.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:500 front elevation differs from the real dimension N m by more than max( | 1 | `representations.500.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:500 plan (top view) differs from the real dimension N m by more th | 1 | `representations.500.geometry.plan` |
| 🔄️continuous-rotation | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:200 front elevation differs from the real dimension N m by more th | 1 | `representations.200.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:200 front elevation differs from the real dimension N m by more than max( | 1 | `representations.200.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:200 plan (top view) differs from the real dimension N m by more th | 1 | `representations.200.geometry.plan` |
| 🔄️continuous-rotation | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:100 front elevation differs from the real dimension N m by more th | 1 | `representations.100.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:100 front elevation differs from the real dimension N m by more than max( | 1 | `representations.100.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:100 plan (top view) differs from the real dimension N m by more th | 1 | `representations.100.geometry.plan` |
| 🔄️continuous-rotation | semantic | extent-front-depth | `representations[N].geometry.front` | The default-pose lateral width N m of the 1:50 front elevation differs from the real dimension N m by more tha | 1 | `representations.50.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-front-height | `representations[N].geometry.front` | The default-pose height N m of the 1:50 front elevation differs from the real dimension N m by more than max(2 | 1 | `representations.50.geometry.front` |
| 🔄️continuous-rotation | semantic | extent-plan-depth | `representations[N].geometry.plan` | The default-pose lateral depth N m of the 1:50 plan (top view) differs from the real dimension N m by more tha | 1 | `representations.50.geometry.plan` |

## `volume`-related findings (listed only; not part of this fix except the id renames the coordinator assigned)

| asset | source | rule | pointer (pattern) | message | rows | first pointer |
|---|---|---|---|---|---|---|
| miscanthus-grass | ajv | pattern | `/volume/solids/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 216 | `/volume/solids/13/bone` |
| miscanthus-grass | ajv | oneOf | `/volume/solids/N` | must match exactly one schema in oneOf | 36 | `/volume/solids/13` |
| ivy-on-trellis | ajv | pattern | `/volume/solids/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 24 | `/volume/solids/0/bone` |
| ivy-on-trellis | ajv | oneOf | `/volume/solids/N` | must match exactly one schema in oneOf | 4 | `/volume/solids/0` |
| hosta-clump | ajv | pattern | `/volume/solids/N/id` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 108 | `/volume/solids/0/id` |
| hosta-clump | ajv | pattern | `/volume/solids/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 162 | `/volume/solids/0/bone` |
| hosta-clump | ajv | oneOf | `/volume/solids/N` | must match exactly one schema in oneOf | 27 | `/volume/solids/0` |
| scissor-lift | ajv | pattern | `/volume/solids/N/bone` | must match pattern "^[a-z0-9]+(?:-[a-z0-9]+)*$" | 156 | `/volume/solids/14/bone` |
| scissor-lift | ajv | oneOf | `/volume/solids/N` | must match exactly one schema in oneOf | 26 | `/volume/solids/14` |

## Added by the corrected filter (second full run, before the data edits)

- `appliances/tower-server`: Ajv `required` property `tone` missing on 7 shapes (elevation: `grille-slits`, `psu-slits`, `intake-slits`, `fan-ring`; front: `intake-ribs`; plan: `psu-top-slits`) at every scale that draws them.
- Shapes of `office-swivel-chair` (`armrest-*--1`, double hyphen) and `three-seat-sofa` (`back-seam-0.73` and three more) with ids that are no `Slug`, and `date-palm` `ring-2.4`, `ring-4.4`, `ring-6.4` (already in the table above for the first two assets only through the `pattern` rows).
- `scissor-lift` pose `thermal.emitters` nested one level too deep (`{ "motor-vent": { "motor-vent": 0, ... } }`, Ajv `type` number).
- `chalkboard` ellipses `hanger-a`, `hanger-b` still carrying rect members `x`, `y`, `w`, `h` (Ajv `additionalProperties`).
- `playground-swing` front/plan `beam-cap-left|right` and `excavator` plan `hood-louvre-1..6`: rects of zero width or height (Ajv `exclusiveMinimum`).
- `bicycle-stand`, `drinking-fountain` clip `fps` 0.01 (Ajv `minimum` 1), `heatW` below 0 on six assets (ceiling-diffuser, expansion-valve, heat-pump-outdoor-unit, scroll-compressor, solar-thermal-collector, split-ac-indoor-unit), `vapourKgPerKg` below 0 on `mvhr-unit`, `heat-pump-outdoor-unit`.
- `noJoin` (Ajv `additionalProperties`) on 7 vegetation assets: 168 shapes.

