# Wave Ledger — Used Park Objects

One row per asset, brought up to date by the finisher on 2026-11-22 (the wave was cut off at about 17:30; its ledger had duplicate and stale rows). Every state below was measured in the finishing session: full proof (`r3-validate.ts --file … --volumes`), check CLI, `verify.ts` (candidate = catalog copy, manifest row = recomputed row, member of `used-park-objects`). Details per asset: `reports/<slug>.md`. Requests: `../GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md`.

| row | slug | state | finishing session | still open (asset) |
|---|---|---|---|---|
| 01 | used-backrest-park-bench | published, proof VALID, check ✓ | proven, unchanged | Reactions of the lifting pose were already exact (416.5 N per sling): the footings are massless, so the ledger's proposal to add a hook force did not apply. |
| 02 | used-concrete-bench-block | published, proof VALID, check ✓ | proven, unchanged | Lifting pose already exact (5061.2 N per socket). |
| 03 | used-tree-surround-bench | published, proof VALID, check ✓ | Lifting pose corrected | No tree trunk is drawn; the 1:50 plan is dense (ledger). |
| 04 | used-park-armchair | published, proof VALID, check ✓ | proven, unchanged | Perforation is a hatch; the slender-tube request stays open. The ledger's point about oversized rust discs at 1:500 is stale: the document has no such shapes. |
| 05 | used-sun-lounger-bench | published, proof VALID, check ✓ | proven, unchanged | Lifting pose already exact (226.0 and 200.1 N per sling); the worn lawn is not drawn (ledger). |
| 06 | used-beer-garden-set | published, proof VALID, check ✓ | proven, unchanged | Open plan strokes of the table top show mirrored in the plan of the turned-over table (request carried over). In the stacked and maintenance poses the set is solved as one rigid body (solver request). |
| 07 | used-concrete-table-tennis | published, proof VALID, check ✓ | Footing treatment made the same as rows 01 and 05: the two strip footings are massless bones `footing-a`, `footing-b` with body, thermal (`footings` region, 7; Footing depth set to the frost-free 0; With 0 | `dimensions.heightM` 1.7125 m includes the footings below ground; the height above ground (0.9125 m) is in the texts only. |
| 08 | used-chess-table | published, proof VALID, check ✓ | Lifting pose corrected | In `maintenance` the table hangs 60 mm above its bed with no sling drawn. |
| 09 | used-litter-bin-post | published, proof VALID, check ✓ | proven, unchanged | Ledger points not re-worked: the tipped liner is approximate in the front view, the liner slides on its axis, the dent is stepped. |
| 10 | used-dog-waste-station | published, proof VALID, check ✓ | Bag slot lowered from 1; Consequence: the rain hood of the bin can open only 45 degrees under the dispenser (20 mm clear) instead of 85; joint limit, the poses `maintenance` and `lids-open` and their texts changed accordingly | The hood opens only 45 degrees; emptying is through the front under the hood. |
| 11 | used-recycling-bin-cluster | published, proof VALID, check ✓ | proven, unchanged | The lid held open in `maintenance` is a load of the worker's hand (already the applied-force pattern). |
| 12 | used-park-lantern | published, proof VALID, check ✓ | Lifting pose corrected | While the column is lowered, the fading mat force stands for the rope; struts are approximate in the front view of the tilted pose and the rear door shows in the front view (ledger, not re-worked). |
| 13 | used-bollard-light | published, proof VALID, check ✓ | proven, unchanged | Lifting pose already exact (75.5 N per grip). |
| 14 | used-double-swing | published, proof VALID, check ✓ | Lifting pose corrected | The post foundations below the gravel are not modelled; the gravel sample (about 1.6 t) is not in `massKg`; the gravel mesh is large (ledger: 1.8 MB). |
| 15 | used-slide-tower | published, proof VALID, check ✓ | Lifting pose corrected | The sliding child is a moving load only (ledger). |
| 16 | used-seesaw | published, proof VALID, check ✓ | Lifting pose corrected | Lawn hollows are text only (ledger). |
| 17 | used-spring-rocker | published, proof VALID, check ✓ | Lifting pose corrected; Real defect found by the statics: both sling pairs (handle ends x 0 | 1:500 is unavailable (2.4 mm on paper); the 1 degree rest lean of the spring is ignored (ledger). |
| 18 | used-playground-roundabout | published, proof VALID, check ✓ | proven, unchanged | At 1:50 the rim is a 60 x 60 mm envelope (ledger). |
| 19 | used-sandpit-digger | published, proof VALID, check ✓ | Lifting pose corrected | The eye nut stands 0.12 m behind the centre of mass of the folded digger (stated in the assumptions); in service the post socket, sand and edging are solved as one rigid body on seven supports (solver request); no lever linkage (ledger). |
| 20 | used-climbing-net-pyramid | published, proof VALID, check ✓ | proven, unchanged | The net faces are rigid; at 1:500 the light-line drawing of the net is dense (ropes close to the print floor). Mass markers lie up to 0.1 m off the member centres (ledger, not re-measured). |
| 21 | used-outdoor-fitness-station | published, proof VALID, check ✓ | Lifting pose corrected | At 1:500 the bench legs are slender (ledger). |
| 22 | used-basketball-hoop | published, proof VALID, check ✓ | proven, unchanged | The chain net is rigid and tilts with the ring (ledger); the lowering pose was already statically separate (hinge pin 463.2 N, sling 858.8 N). |
| 23 | used-football-goal | published, proof VALID, check ✓ | proven, unchanged | The mesh net is opaque in the 3D sheets (ledger). |
| 24 | used-pump-drinking-fountain | published, proof VALID, check ✓ | Lifting pose corrected | The head's centre of mass lies 32 mm beside the single lifting point; the jet is three slices that jump (ledger). |
| 25 | used-water-play-table | published, proof VALID, check ✓ | proven, unchanged | No moving water stream; the screw flights are static in 2D; five permanent supports share by equal stiffness (ledger, a true indeterminate case). |
| 26 | used-bubbler-stone-fountain | published, proof VALID, check ✓ | proven, unchanged | The boulder is smooth; limescale, moss and algae are strokes only. |
| 27 | used-park-noticeboard | published, proof VALID, check ✓ | proven, unchanged | At 1:200 and 1:500 glass and pinboard are thicker in the thermal list (ledger); the reading clip barely moves. |
| 28 | used-wayfinding-signpost | published, proof VALID, check ✓ | Lifting pose corrected | The swap slides the finger only 85 mm; at 1:500 the fingers are 75 mm (ledger). |
| 29 | used-bicycle-shelter | published, proof VALID, check ✓ | Published for the first time (manifest row inserted, collection now 29 assets); the author had stopped after writing `distinction; Lifting pose corrected | The post footings are not modelled (stated in the structure assumptions); bicycle, stands and frames are solved as one rigid body (solver request). The dimension of the height reads 2.60 to 2.63 m between the scales (coarser gutter and sheets). |

Common to all rows: lifting gear, workers and users are not drawn (no per-pose presence of solids in the schema), wear below the smallest feature is 2D strokes only, and no author's reference table exists.

## Never started (dev's order of 16:29: finish the started rows only)

| row | slug | title |
|---|---|---|
| 30 | used-ebike-charging-pillar | objects |
| 31 | used-pram-gate-barrier | components |
| 32 | used-chain-post-fence | components |
| 33 | used-picket-playground-fence | components |
| 34 | used-pergola | components |
| 35 | used-park-pavilion | components |
| 36 | used-raised-garden-bed | objects |
| 37 | used-compost-bins | objects |
| 38 | used-insect-hotel | objects |
| 39 | used-wheelbarrow | objects |
| 40 | used-leaf-collection-cart | objects |

## Manifest

`verify.ts` at the end of the finishing session: 29 assets, 0 with findings, collection `used-park-objects` has 29 members. Earlier drift: [manifest-drift.md](manifest-drift.md).
