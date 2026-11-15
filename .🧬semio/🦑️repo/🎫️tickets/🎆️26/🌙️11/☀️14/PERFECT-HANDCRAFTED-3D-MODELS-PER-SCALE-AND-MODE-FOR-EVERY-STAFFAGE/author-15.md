# Author 15 — furniture (bar-stool, bookshelf, cafe-chair, dining-table, folding-deck-chair, height-adjustable-desk)

All six assets are spliced (`volumes-insert.py`), every scale authored from `author_15_<asset>.py` (generator, world coordinates through `author_15_lib.py`, rest frames from `author-15-frames.ts`). Final check outputs: `🗑️generated/author-15/final-<asset>.txt` (temporary, delete with the folder).

## Solid counts (body / thermal / structure, as listed; repeats not expanded)

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| bar-stool | 7 / 9 / 16 | 6 / 6 / 16 | 5 / 5 / 14 | 4 / 5 / 12 |
| bookshelf | 26 / 26 / 39 | 16 / 16 / 24 | 16 / 16 / 24 | 16 / 16 / 24 |
| cafe-chair | 27 / 27 / 21 | 12 / 12 / 20 | 11 / 11 / 19 | 10 / 10 / 18 |
| dining-table | 14 / 14 / 18 | 8 / 8 / 14 | 8 / 8 / 12 | 5 / 8 / 10 |
| folding-deck-chair | 9 / 7 / 22 | 9 / 7 / 22 | 5 / 6 / 18 | 5 / 6 / 18 |
| height-adjustable-desk | 31 / 32 / 22 | 20 / 20 / 22 | 17 / 17 / 21 | 14 / 14 / 19 |

## What was modelled by hand
- **bar-stool**: legs/feet/ring as repeats of one solid (4 x 90 degrees), ring as a true torus (closed revolve), plywood plate and rounded foam pad as revolves, back cushion as a 7-section superelliptic arc loft around the swivel axis, nested foam/leatherette shells (1:50), hidden plywood core, swivel bearing, pad guide, masses, 4 foot supports (with `anchor`). Coarser: merged seat disc, boxes for back, fewer torus sections.
- **dining-table**: chamfered plates, apron frames, tapered legs with shoulder + corner blocks, flat steel runners and cleats (1:50); hollow apron rings (1:100/1:200); slab blocks (1:500). Leaf under the plates, 4 supports.
- **cafe-chair**: tilted back (5 deg) with panel/rail on the back-bone axes, splayed legs with domed feet and knuckles, stretchers, rounded-corner seat on a four-board apron; coarser scales hollow apron frames, merged rails, thicker tubes.
- **folding-deck-chair**: 45 mm posts and bars along the real rest frames with true z offsets, dowel rails, headed bolts, two canvas panels (7.5 mm at 1:50), foot/slide members and supports.
- **bookshelf**: 18 mm carcass, shelf halves with the hinge overlap, 20 round shelf studs (1:50), hollow carcass at every scale (no filled block), plinth, supports.
- **height-adjustable-desk**: hollow nested telescoping square tubes (1:50), stacked solid posts (coarser), foot plates, drive housings on mounts, hollow rail with mounting pads, U cable tray, keypad, monitor with glazed screen, laptop, two-shell top (laminate/board).

## Roots fixed in the asset files (2D / thermal data)
- bar-stool: thermal layers of `seat-pad` and `backrest` reordered to innermost first (foam, then leatherette; schema says innermost first, region material is the cover); plan views of 1:100/1:200/1:500 gained the footrest ring (it is visible beyond the seat); 1:500 elevation/front legs and ring are filled 75 mm capsules instead of open strokes (a stroke crossing three 75 mm legs measured as a 300 mm member).
- height-adjustable-desk: `top-board` layers reordered (board, then laminate).
- cafe-chair: removed the stray 1:100 front `stretcher` line across the full width (the 1:50 drawing and the plan show two side stretchers only).
- bookshelf: front-view shelf shapes belong to `shelf-N-l` (was `shelf-N`), so elevation and front agree per bone.
- `areaM2` corrected to the honestly modelled exposed 1:50 area (old -> new): bar-stool backrest 0.06 -> 0.021, seat-plate 0.102 -> 0.124; cafe-chair seat-beech 0.17 -> 0.524, back-beech 0.1 -> 0.242, steel-tube 0.13 -> 0.154; dining-table apron 0.42 -> 0.744; folding-deck-chair canvas-back 0.3 -> 0.49, canvas-seat 0.23 -> 0.368; bookshelf carcass 1.9 -> 2.952, shelves 1.07 -> 2.294, back-panel 1.4 -> 2.616, plinth 0.12 -> 0.153; height-adjustable-desk steel-desk 0.8 -> 2.15.

## Final check state (see `🗑️generated/author-15/final-*.txt`; the check tool changed several times during the work)
- **Pass** all rules at all scales: nothing completely. Remaining lines per asset:
  - bar-stool: silhouette elevation 1:200/1:500 (IoU 0.77/0.79: 30 / 75 mm tubes against line drawings), thermal area at coarse scales (merged seat, 75 mm ring/legs).
  - bookshelf: only `volumes-monotonic` 1:100 (structure triangle count at the finer fixed quality). Section p95M (A-A) was reported in `author-requests.md` and no longer fails in the current tool.
  - cafe-chair: silhouette 1:100/1:200/1:500 (thin tubes and slivers), feature at 75 mm tubes, thermal area of tubes at 1:200/1:500, 1:50 structure outside-body 7.8 mm vs 7.5 mm.
  - dining-table: only 1:500 (silhouette IoU 0.87-0.91 against stroke legs, thermal areas of slab/apron/hidden rails, the 110 mm slab is heavier than the real plate).
  - folding-deck-chair: silhouette 1:200/1:500 (45 / 75 mm bars vs strokes), thermal area of canvas/frame at coarse scales, 1:50 structure by 0.1 mm.
  - height-adjustable-desk: silhouette 1:200/1:500 (75 mm top against 25 mm), thermal top-board exposed area (0.76 m2 measured against 1.28, cause not found: the exposed-surface count of a face partly under the monitor and laptop seems larger than their footprint), monotonic 1:100 structure triangles.
- Rule/tool observations: thin plates and tubes at 75 mm give heavy volume and silhouette deviations by design (feature rule vs 25 mm real parts); `hidden` regions have no exposed surface once buried (seat-plate, slide-rails at 1:500).

## What I saw in the renders
Looked at 1:50 of every asset and at least one coarser scale (bar-stool 50/200/500, dining-table 50/100/500, cafe-chair 100/200, deck chair 50, bookshelf 50/200, desk 50): nested shells and members are inside the bodies, no floating parts after the fixes (desk rail got mounting pads, bar-stool legs end in caps). Not viewed: bar-stool 1:100, deck chair 1:100-1:500, desk 1:100-1:500 renders (only numbers). The deck-chair canvas is two flat panels (no sag). The desk drive mounts show as small blue slivers between the drives in the front overlay.

## Imperfect, stated plainly
- Coarse-scale silhouettes of thin-tube objects fail by design (see above); no rule was weakened.
- Plain `areaM2` fixes change thermal inputs of these assets (listed above).
- Desk `ZS` (front-view sign) was assumed +z; the overlays agree at 1:50/1:100.
