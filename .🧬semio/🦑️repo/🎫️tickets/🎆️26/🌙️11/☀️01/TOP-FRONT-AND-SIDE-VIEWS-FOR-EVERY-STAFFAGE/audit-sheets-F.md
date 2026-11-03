# Audit Sheets F: Top, Front and Side Views for Every Staffage (furniture, 12 assets)

Read-only visual audit. Catalog and source files were not edited. Sheets were rendered with the F driver (`🧰️F/render.ts`, bun, one process per asset and scale) into `🗑️generated/R-F`, reviewed per sheet, and then deleted. Five cited sheets are kept in `🗑️generated/R-F-cited`.

Coverage: 47 authored representations (12 assets, scales 1:500, 1:200, 1:100, 1:50; operating-table has no 1:500). Every pose and three instants per clip (clip durations split at t/3, 2t/3). 169 sheets (4 states per sheet, elevation, front and plan per state).

## Verdict

**NOT READY.** Views are present and non-empty for every asset, and parts attach in nearly every state. Three assets carry defects that break the goal: park-bench occupant and sun states show no change, three-seat-sofa front backrest disagrees between scales, and operating-table tilt overlaps the column in elevation. bar-stool and office-swivel-chair (priority) pass at 1:500, 1:200, 1:100 and 1:50 with notes.

## Findings

| asset | scale | pose/instant | view | finding |
|---|---|---|---|---|
| bar-stool | 500, 200, 100, 50 | unoccupied, occupied, turned-90, seat-proof-load | elevation, front, plan | OK. Backrest post on seat, legs reach floor, plan pad at seat edge in turned-90. |
| bar-stool | 500, 200, 100, 50 | turned-270-key, occupied-turned-key, swivel-turn@0/2/4, sit-and-turn@0/1.67/3.33 | elevation, front, plan | OK. Plan backrest orbits the seat circle; front and plan agree. Note: at 90 degrees the side view is a thin post (accepted flat-profile limit). |
| bar-stool | 500, 200, 100, 50 | stand-up@0/1/2 | elevation, front, plan | OK. Seat rise reads; legs attached. |
| office-swivel-chair | 500 | seat-low | elevation | NOTE. Backrest bottom sits ~4 px above the seat band; attachment reads as near-touching. Verify gap. |
| office-swivel-chair | 500, 200, 100, 50 | swivel-quarter, swivel-quarter-opposite-key, swivel-rotation@1.33/2.67 | plan | OK. Backrest and arms rotate in plan; extents match front. Note: at 1.33 and 2.67 backrest and arms extend beyond the seat outline in plan (expected sweep). |
| office-swivel-chair | 200, 100, 50 | reclined | elevation | OK. Backrest tilts on its hinge; front and plan consistent. |
| office-swivel-chair | 500, 200, 100, 50 | height-adjust@0/2/4 | elevation, front | OK. Gas lift column and base attached; base to floor. |
| park-bench | 100, 200, 50, 500 | occupied-one, occupied-two, occupant-sit-down@0/0.67/1.33 | elevation, front, plan | **FAIL.** No occupant geometry; states are identical to unloaded (verified 1:100 sheet-01, `park-bench/100/sheet-01.png`). |
| park-bench | 100, 200, 50, 500 | sun-exposure-cycle@0/20/40 | elevation, front, plan | **FAIL.** No visible change across the cycle. |
| park-bench | 500 | all states | elevation, front | NOTE. Back and seat drawn as solid blocks; arms omitted. |
| three-seat-sofa | 500 | unoccupied, one-seated, two-seated, lounging | front | **FAIL (cross-scale).** Backrest is near-vertical at 1:500 but leans ~40 px at 1:200, 1:100 and 1:50 (verified `three-seat-sofa/500/sheet-01.png` vs `three-seat-sofa/200/sheet-01.png`). The elevation is upright at every scale, so front and elevation disagree at 1:200 and below. |
| three-seat-sofa | 500, 200, 100, 50 | all 16 states | elevation, plan | OK. Legs reach floor, arms, seat and backrest attached; plan width matches elevation. |
| three-seat-sofa | 500, 200, 100, 50 | all 16 states | plan | NOTE. Plan is identical in every state, so recline and lie-down read only in elevation and front. |
| operating-table | 100, 200, 50 | tilt-table@6.67, tilt-table@13.33 | elevation | **FAIL (candidate).** Tilted top crosses the boom column in elevation (verified `operating-table/100/sheet-02.png`, about x 388 to 403 at 6.67; head end overlaps the column at 13.33). Confirm in 3D whether the column is behind the top; if it is a real clash the state is wrong. |
| operating-table | 100, 200, 50 | head-down | elevation, front | FAIL (same overlap as tilt). |
| operating-table | 100, 200, 50 | light-off, light-on, switch-on-warm-up | elevation, front, plan | OK. Base, pedestal, boom and lamp attached. |
| operating-table | 200 | all states | elevation | NOTE. Wheel circles omitted at 1:200. |
| cafe-chair | 500, 200, 100, 50 | tip-back@1.00, tip-back@2.00, tilted-on-two-legs | elevation, plan | NOTE. Back post passes through the seat near the rear corner; at 1:50 rear leg finials sit just outside the seat outline in plan. |
| cafe-chair | 500, 200, 100, 50 | all other states | elevation, front, plan | OK. Attachment, leg contact and extents correct. Back is a thin post at 1:500 (accepted level of detail). |
| bookshelf | 500, 200, 100, 50 | empty, loaded, shelf-adjusted, top-two-loaded-key, load/re-pin/unload clips | elevation, front, plan | OK. Shelves, plinth and shelf re-pinning attached. Notes: no back or sides at 1:500 and 1:200 (level of detail); an extra plan line at 1:200 in loaded states that 1:500 does not show. |
| single-bed | 500 | flat, head-raised, asleep, reading-in-bed | elevation, front | NOTE. Centre support leg missing at 1:500 (present at 1:200 to 1:50). |
| single-bed | 500, 200, 100, 50 | raised head-section states | plan | NOTE. Raised head section footprint not drawn, leaving an empty gap at the head end. Deck under the raised section is a sliver at 1:500 and 1:200. |
| single-bed | 500, 200, 100, 50 | get-into-bed, sit-up-to-read | elevation, front, plan | OK. Mattress sections, frame and posts attached; posts reach floor. |
| folding-deck-chair | 500, 200, 100, 50 | fold, unfold, sit-and-recline clips | elevation, front, plan | OK. Legs reach floor, hinges and sling attached; motion consistent in elevation and plan with matching widths. Notes: thin-line elevation at 1:500 and 1:200 (level of detail); sling colour changes with state (cosmetic). |
| dining-table | 500 | extended, set-for-dinner | elevation, plan | NOTE. Centre insert ~5 px higher than the leaves; confirm intent. |
| dining-table | 500, 200, 100, 50 | extend and retract at 1.33 | plan | NOTE. Leaves overlap the insert in the 1.33 states (reads as sliding leaves). |
| dining-table | 500 vs 200 | closed | elevation | NOTE. Leg inset about 45 px at 1:500 vs about 33 px at 1:200; legs plain lines at 1:500, tapered at 1:200. Confirm against model. |
| dining-table | 500, 200, 100, 50 | lay-the-table clip | elevation, front, plan | OK. Extents match; apron, legs and insert attached. |
| height-adjustable-desk | 100, 200, 50 | seated, mid, standing, raise-and-lower, load-on-surface | elevation, front, plan | OK. Feet reach floor; column segments attached. Note: red accessory block visible only at 1:50. |
| height-adjustable-desk | 500 | all states | elevation, front | NOTE. Columns and top drawn as hairlines; monitor stand not visible in seated and standing states. |
| picnic-table | 500, 200, 100, 50 | deployed, fold-benches, deploy-benches, half-folded-key | elevation, front, plan | OK. Frame, legs and brackets connected; rotation reads. |
| picnic-table | 100, 50 | stowed | elevation, front | NOTE. Stowed benches stop ~25 px above the floor line. |
| picnic-table | 100, 200, 50 | occupied, bench-proof-load | elevation, front, plan | NOTE. Identical to deployed; no occupant or load marker. |
| picnic-table | 500, 200, 100, 50 | sit-down-together | elevation, front, plan | OK. Bench lowers and stays attached; occupants not drawn. |

Note on sources: bar-stool and office-swivel-chair were reviewed sheet by sheet at all four scales by me. The other ten assets were reviewed sheet by sheet by three review agents (groups: bookshelf, single-bed, folding-deck-chair; three-seat-sofa, cafe-chair, dining-table; height-adjustable-desk, operating-table, park-bench, picnic-table). Their findings are summarised above. The park-bench, sofa-front and operating-table items were checked directly by me.

## Cited sheets (`🗑️generated/R-F-cited`)

- `bar-stool-1-500-sheet-01.png`: unoccupied, occupied, turned-90, seat-proof-load at 1:500. Parts attached, plan backrest reads.
- `park-bench-1-100-sheet-01.png`: occupant states identical to unloaded. FAIL.
- `operating-table-1-100-sheet-02.png`: tilt-table@6.67 slab crosses the column. FAIL candidate.
- `three-seat-sofa-1-500-sheet-01.png`: front backrest near-vertical at 1:500.
- `three-seat-sofa-1-200-sheet-01.png`: front backrest leaning at 1:200. FAIL.

## Required follow-ups (not done, read-only audit)

1. park-bench: add occupant geometry or motion for occupant-sit-down and sun-exposure states.
2. three-seat-sofa: make the front backrest profile identical across 1:500 to 1:50.
3. operating-table: confirm the tilt clash in 3D and fix the tilt or the column placement.
4. office-swivel-chair (1:500): confirm the seat-low backrest gap.
5. single-bed: draw the 1:500 centre leg and the raised head-section plan footprint.
6. picnic-table: stowed bench floor gap; occupancy markers.
