# Author 07 report — INCOMPLETE (no asset spliced)

Assets: child-on-kick-scooter, courier-with-hand-truck, cyclist, elderly-with-cane, jogger, kneeling-gardener, leaning-person.
Status: none of the seven is finished or spliced. The work done is infrastructure plus one pilot (cyclist 1:50). The shared check became very slow (over 2 minutes per asset under load from the other authors) while the tools were being changed for rulings C1-C6, which stopped the iteration loop.

## Built (all in T, prefix author-07 / author_07)
- `author_07_lib.py`: per-scale builder. Body from the scale's own drafts (`humanize`: extrude to loft), thermal list from the body region map (references, nested layer shells at 1:50, skin-scale area fitting), structure (human members, joints, masses, `anchor` on supports per C6), emitter solids with `tags`, `snap` (lateral fit of lofts to the posed front/plan drawings).
- `author-07-diag.ts` (worst Hausdorff cells, blamed per 3D solid and per 2D shape), `author-07-shape.ts`, `author-07-areas.ts`, `author-07-bbox.ts`, `author-07-members.ts`, `author-07-rawdraft.ts`, `author-07-snap.ts`, `author-07-cyclist.py` (pilot, scale 50), `author-07-seed/` (1:50 seed bodies with old regions).

## Findings
- Spokes cannot be in body at any scale: the 2D draws them as `texture`, but the silhouette reference counts every solid, so the interior of the wheel fails Hausdorff (143 mm at 1:50). Dropped from the cyclist.
- Emitters must carry `tags: [<emitter id>]`; layers need shells even at 1 mm (shrink with a floor); thin detail solids (fingers, zips, straps) must be left out of layered thermal regions (below 7.5 mm).
- Front/plan 2D shapes are decals on the bone planes: a forearm pointing at the viewer is drawn as a 32 mm sliver in the default pose, so a physical 3D forearm cannot match it (cyclist front, about 30 mm Hausdorff). Front also omits chainstays, bottom bracket and axles that the plan draws.
- Cyclist 1:50 state: thermal area deviation within 15 % for all regions, emitters/layers covered, elevation Hausdorff 14.3 mm; front IoU 0.88 and Hausdorff 29 mm, plan Hausdorff failing because of thumbs, still open. Source `volumes/humans/cyclist.json` holds 1:50 only in a non-final state; 1:100-1:500 are empty.

## Left to do (all seven assets)
Run the scale builds for 100/200/500, the snap and diag loops, decisions text, renders, check, splice; re-fit thermal areas to the exposed-surface measure of C1 once `volumes-lib.ts` reports it.
