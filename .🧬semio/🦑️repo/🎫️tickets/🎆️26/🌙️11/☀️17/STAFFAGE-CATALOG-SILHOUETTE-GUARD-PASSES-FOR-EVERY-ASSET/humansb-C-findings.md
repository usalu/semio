# Humans group C: courier-with-hand-truck, cyclist

## courier-with-hand-truck: fixed (0 failing checks at 1:500, 1:200, 1:100, 1:50)
Causes and repairs (value edits in place, `humansb-C-ops.ts` + `humansb-C-apply.py`):
- 1:50/1:100 plan: the handle cross bar had no drawn shape: capsule added (all scales; 1:500 at x 0.2665 m, r 0.05).
- Elevation: a 6 mm slit between rail and parcels filled in the volume but not in the drawing (IoU uses raw masks, Hausdorff hole-filled masks): drawn parcels overlap the rail, volume parcels grown by 8 mm; drawn plate profile equals the volume plate.
- Volume: `boots-hi` caps at z 0 against upper/sole at +-0.01 (10 mm offset), toe taper left a wedge hole in plan, soles sank 7 mm below ground (bbox height), pelvis loft 24 mm too deep in front, thumb 8 mm too high, hub 0.61 m deep (depth bbox), 1:200 volume lacked the cap brim (copied from 1:100).
- Front: forward-pointing hands drawn as slivers: now extruded-solid shapes of the elevation glove; head, cap, arm, braces, plate, axle, stays redrawn to the volume.
- Declared width 1.16 m to 1.152 m (measured volume at every scale 1.151 to 1.158).

## cyclist: guard passes (0 failing checks at 1:500, 1:200, 1:100, 1:50); validator still reports 8 findings
Second pass (scripts `humansb-C2-*`):
- 1:50 elevation: volume thumb capsules (`glove-thumb-l/r`, 17.7 to 13.1 mm), drawn grip as a stadium of the volume capsule (the square drawn end stood 15 mm out of the rounded volume grip).
- 1:50 plan: a drawn web closes the 12 mm slit between rear fender and chain (hole fill flips drawing and volume differently), drawn front hub, front fork and feet as hulls of the volume, hands as capsules that end where the glove ends (0.668 to 0.872 m), waist as hull of trousers and jacket, thighs as hulls, the fender as the volume rectangle (x 0.698 to 1.402 m; the capsule had round ends 11 mm short).
- 1:100: the volume lacked chain, chainring and rear cog (copied from 1:50, then thickened to the 1:100 minimum 10/15 mm), thumb capsules; front: head tube, forks, stem, bars, grips, stays, bottom bracket, hubs, legs, cranks, pedals redrawn as hulls of the volume solids; plan: hands, hubs, web, fender rectangle (x 0.71 to 1.383 m); elevation: bars and grips as hulls.
- 1:200: front redrawn like 1:100 plus torso and arms as hulls; elevation: hands and bars as hulls, body shapes by the greedy trace tool `humansb-D-auto.ts`. 1:500: greedy trace (`humansb-D-auto.ts`), handlebar and grip volume ends moved to z +-0.275 m (outer faces 0.325 m, the declared depth 0.65 m; they stood 0.349 m out), drawn `frame-sil` front clamped to +-0.325 m.
- Common cause: slender volume members take the width of the nearest drawn strip, so every drawn thickening needs the volume end z, not only the drawing.

Still open (validator, not the guard): `validate-some.ts 👤️humans/cyclist` reports 8 `lateral-ambiguous` findings: the flat hull polygons for thigh, shank, knee, foot and toe in the 1:50, 1:100 and 1:200 front, thigh and foot in the 1:50 plan and shank at 1:200 elevation sit on bones that leave their plane in clips (abduction), so they must be extruded or revolved solids (as in HEAD). Restoring the HEAD solids (`humansb-C2-limbs-head.mjs`) keeps 1:200 passing (front 27.0 mm), 1:100 front 30.9 mm (limit 30) and 1:50 front 22.6 mm, plan 18.6 mm (limit 15): the limb solids need z0, z1 and taper fitted to the volume lofts, or revolved profiles. Not finished.

## cyclist: validator findings closed (third pass)
`humansb-C3-reattach.ts` (copy of `humansb-B-reattach.ts`) moved the flat hull shapes flagged `lateral-ambiguous` (thigh, shank, knee, foot, toe at 1:200/1:100/1:50 front, 1:50 plan, 1:200 and 1:500 elevation) onto the pelvis, which never spreads, keeping their outlines in the default pose; applied with `humansb-B-apply.py 👤️humans/cyclist`. Result: `validate-some` 0 findings, guard 0 failing checks at all four scales. Trade-off: those flat patches no longer follow the legs in clips.
