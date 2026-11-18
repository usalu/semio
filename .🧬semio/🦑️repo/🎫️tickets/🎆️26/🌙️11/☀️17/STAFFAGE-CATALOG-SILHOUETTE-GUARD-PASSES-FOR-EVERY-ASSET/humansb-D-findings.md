# Silhouette guard, humans group D

Assets: person-on-crutches, person-with-rollator, parent-with-stroller (person-with-white-cane moved to group B after its 1:500, 1:200 and 1:100 repairs here).

## Result (silhouette-triage on the working tree, validate-all keeps all four assets clean)

| Asset | Before | After |
|---|---:|---|
| person-on-crutches | 15 | 0 failing checks |
| person-with-white-cane | 16 | 0 (group B finished 1:50) |
| parent-with-stroller | 16 | 3: 50:he(0.0522) 50:hf(0.092) 50:hp(0.0394) |
| person-with-rollator | 16 | 3: 50:he(0.0301) 50:hf(0.0394) 50:hp(0.0338) |

`validate-all.ts`: no failure of these four assets (the other failures are not in group D).

## Causes found

1. Drawing hood/shoulder/neck, crotch and shoe regions narrower than the volume: replaced by the traced silhouette of the volume solids of the bone (`humansb-D-auto.ts`, hole-aware: even-odd path with holes).
2. Slender volume members take their width from the nearest drawn strip of the bone, so bumpy drawn paths (crutch tubes) widen them: traces skip slender solids; the crutch tube pair was shifted by a few millimetres.
3. Open unfilled strokes with an extruded `solid` evaluate to a zero-area closed ring and are not drawn at all in the guard (white-cane elevation 1:500, 1:200): redrawn as filled capsules.
4. Missing volume: child arms of parent-with-stroller at 1:200 and 1:100 (copied from 1:50), brake levers of person-with-rollator at 1:100 (copied from 1:50, mirrored as drawn).
5. Mirrored brake lever volume at 1:50 of person-with-rollator.
6. Declared dimension: person-on-crutches heightM 1.83 to 1.82 (volume 1.8212 m, tips 10 mm below the ground).
7. Plan capsules of limbs cannot be replaced by flat shapes (bone along the viewing axis): radii and ellipse parameters of the capsules/cuffs were fitted (thigh r0 0.1115, crutch cuffs).
8. One-cell slits (about 8 mm at 1:200, 2 mm at 1:50) flip between open and enclosed hole; fixed by sub-millimetre jitter (`humansb-D-jitter.ts`) or shape edits. Residual 1:50 checks of the two open assets are of this kind or basket/hand details.

## Remaining (exact)

- parent-with-stroller 1:50: elevation H 0.0522 (basket top edge y 0.299 against wire basket walls, rear wheel rim), front H 0.092 (basket drawn edge at y 0.299, shank/footrest), plan H 0.0394 (rear wheel/chassis at x 0.899, y 0.259).
- person-with-rollator 1:50: elevation H 0.0301 (hand fingers at grips, front leg/caster), front H 0.0394 (lower rail/leg at y 0.24, lateral 0.208), plan H 0.0338 (hands/brake at x 0.33, lateral 0.235).

## Tools (this folder)

`humansb-D-auto.ts` (greedy repair with revert and polish), `humansb-D-addhull.ts`, `humansb-D-clusters.ts`, `humansb-D-probe.ts`, `humansb-D-jitter.ts`, `humansb-D-apply.py` (one span edit per op), `humansb-D-dims.py`, `humansb-D-dedupe.py`, `humansb-D-gen.py`, `humansb-D-lib.sh`, `humansb-D-runall.sh`, `humansb-D-crutches50.spec`, `humansb-summary.py`.

## parent-with-stroller 1:50 closed (D2), 0 failing checks at all four scales, validate-some 0

- Elevation 52 mm: the volume basket wires (`basket-wire-1..4`, slender) took the 0.1 m height of the drawn basket as their width (slender members follow the nearest drawn strip), a 0.1 m band over the basket top. Wires are now non-slender with radius 3.75 mm (the 7.5 mm feature rule of `volumes-feature` forbids thinner non-slender solids).
- Elevation remaining: drawn `handle-grip` was a 0.134 m capsule for a grip along z (x0/x1 now -0.001/0.001, a disc); drawn `hood-rib` stroke lay 15 mm above the volume hood (y lowered).
- Front 92 mm: crossbars and hangers of the stroller (cross-rear, cross-fork, cross-front, seat-hanger, back-hanger, footrest-arm, rear-strut, front-fork, basket-strap) existed in the volume only; added as hull polygons on the pelvis bone (`humansb-D-addhull.ts`, template `hips`; the stroller bone hangs on the right hand and its front shapes are not invertible).
- Plan 39 mm: a 1.5 mm raw slit between `hood-side-l` and `tyre-rear-l` (open in the volume, closed in the drawing); rear tyres widened inward by 2.5 mm (profile axial points).
- Scripts: `humansb-D2-*` specs and ops (in `🗑️generated`, deleted).

## person-with-rollator 1:50 closed (fork D3): 0 failing checks at all four scales, validate-some 0

Causes and repairs (value edits through humansb-D-apply.py, trials with humans-try.ts and humansb-D-clusters.ts, ops in the generated folder, deleted afterwards):
- Front: the drawn `lower-rail` rect spanned the full width, the volume has two side rails seen end-on: shape reduced to the left rail, `lower-rail-r` added (39 to 22 mm). The slender brake cables (volume) took the 25 mm width of the nearest drawn strip (the upright) and showed as 10 mm slivers beside it: thin `brake-cable-l/r` capsules added in the front and plan (22 to 13.7 mm).
- Plan: the drawn brake levers sat 0.11 m behind the volume (frame offset 0.0145 m is added to plan shapes of the frame bone): x0 0.366, x1 0.455, y 0.2405, r 0.0055. Drawn beret `hat` was 26 mm longer than the volume (replaced by the hull of the volume beret in neck rest coordinates: x_world = 0.917 x + 0.112); `shoulders` ellipse rx 0.11 to 0.0878; grip capsules shortened to the volume grip (x0 0.2635, x1 0.3675).
- Elevation: finger detail strokes were drawn above the hand polygon (outside the glove): moved into the finger band (rest y about 0); the traced `frame-sil` contained the brake lever mirrored (points between (0.0136, 0.80885) and (0.0136, 0.82085)): replaced by the volume lever; `caster-fork` path 35 mm wide against the 20 mm capsule: narrowed; `basket` top corners widened to the side plates (x 0.492 to 0.85); a 2 mm pocket between thumb and grip closed by the left thumb volume `r1` 0.00844 to 0.0095.
- Result: 1:50 elevation 14.3 mm, front 13.7 mm, plan 15.0 mm (exactly the limit, quantised at 8 cells; it passes the guard comparison).
