# Author 59 report: labrador-dog, dairy-cow, horse (round 2, continuation of 25)

Order: dog, cow, horse. All three are re-run, checked with the product (`volumes-check.ts <asset> --product`), opened as sheets at all four scales after the last change, and spliced (`volumes-insert.py`, last splice after the last run). Not fully green: findings below with numbers. No `areaM2` touched, nothing in rules or shared tools loosened.

Drivers: `author-25-run.sh <dog|cow|horse> <scale>` with `author-25-{dog,cow,horse}.ts`, `author-25-quad.ts`, `author-25-lib.ts` (new: `thirdRadius`/`ellArea` to size small skin solids to the declared surface, `tolFactor`, `latFallback`, `diagonal`/`centreV` structure members, mass clamp). 2D repair: `author-59-patch.py` (text patch of single shapes, add shapes), `author-59-dp.py`. Helpers: `author-59-chk.sh <slug> [scale]`, `author-59-splice.sh <slug>`, `author-59-crop.sh`, `author-59-drop.py` (which solid drives a boundary-distance number).

## Root causes found (and fixed) in the shared builder

- Station tolerance 0.45 feature lumped the outlines of the coarse scales (dog 1:500 elevation IoU 0.875, plan 0.879; cow 1:500 plan 0.867): 0.2 feature (horse 0.3) at 1:200 and 1:500 gives dog 1:500 0.931/0.934/0.901, cow 1:500 0.956/0.907/0.931.
- Lower-leg bones without their own front/plan shape (cow cannon, hoof; horse pastern, hoof; 1:500 lower legs) took a 5 cm default half width on the MIDLINE: legs stood between the legs (the "blue bars" of the earlier sheets, front IoU 0.875 and 0.869). Now they take the lateral position of the neighbouring drawn leg bone (`latFallback`), structure members of undrawn horse pasterns/hooves likewise.
- Thigh tapers removed (cow, horse): the drawn widths are followed.
- Cow ear structure members left the ear flaps by 20 mm in plan (author 25 could not find the cause): the drawn ear runs diagonally (out and back), the member was straight along the bone at the mean lateral position. Members of ears are now oblique inside the flap (`diagonal`): structure-coverage green at all four scales.
- Mass ellipsoids are clamped into the loft section at their centre of mass (cow lumbar mass poked 36 mm out at 1:200).
- Dog and cow tails at 1:500 and horse trunk: see 2D repairs.

## 2D drawing defects repaired at the root (text patches of the asset files)

- Dog plan trunk: hindbody, chest and neck at 1:200 and 1:500 were three-point triangles (revolved bicone profiles: the plan showed a bow-tie with pinched waist), at 1:100 four points: replaced by the 1:50 barrel profiles (7, 6, 5 points). Plan IoU 1:200 0.862 -> 0.937, 1:500 0.839 -> 0.901.
- Dog plan ears (1:50, 1:100, 1:200): the capsule's tip follows the posed ear bone so the plan ear was 0.138 m long against 0.089 m in the elevation: rest coordinates changed (x0 0.8055, x1 0.7885) so that default-pose extents agree (0.774 to 0.863 in both views). Not done for the other poses (the tip offset is pose-dependent).
- Cow 1:500 elevation had only the middle tail bone (tail-2): shapes for tail-1 and tail-3 added as 12 mm Douglas-Peucker hulls of the 1:200 masses (the front view already drew the whole tail 1.05 m; tail-3 had no solid and its member lay 0.14 m outside).
- Cow 1:200 plan head tip moved back 0.03 so that the whole-asset length agrees between views (53.7 mm against limit 51.3 mm -> 39 mm).
- Cow 1:500 plan ear span: the 3D bar sat 0.1 m in front of the drawn stroke (bone frame sign): moved onto it.
- Horse plan (all scales): the pelvis capsule and the chest capsule left a waist of 0.17 m half width where the back is straight in the elevation (lumbar bone had no plan shape): a plan capsule `loin` on the lumbar bone (x 0.25 to 0.75, r 0.3) added at 1:50, 1:100, 1:200, 1:500.
- Reported, not repaired: cow plan ear drawn 0.09 m wide against the 36 mm flap thickness the C13 floor and the roll give; cow 1:50 front hindbody is an extruded slab (corners at +-0.33) while the plan capsule is rounded (see `tool-requests.md` `## 59`).

## labrador-dog (spliced)

Solids (body / thermal / structure): 1:50 37/41/63; 1:100 37/40/44; 1:200 23/31/33; 1:500 20/31/33; decisions 4 paragraphs per scale (bilingual).
By hand: nose leather an ellipsoid of 33 x 33 x 40 mm whose whole surface (0.0037) equals the declared 0.004; four flat pads sized to the declared 0.02 (thickness 7.6 mm at 1:50, 15 mm at 1:100 where C13 demands it); tongue 42 mm wide in the closed mouth; skull solid, eyes, ears, tails and legs as author 25 (bone by bone), now with finer stations.
Check (product, final): all rules pass at 1:50, 1:100, 1:200 (silhouette IoU elevation/front/plan 0.983/0.934/0.962 at 1:50, 0.973/0.932/0.957 at 1:100, 0.960/0.945/0.937 at 1:200, 0.931/0.934/0.901 at 1:500; bounds, feature, bones, monotonic, cross-scale, copied, views-disagree, structure-coverage pass everywhere). Fails only `volumes-thermal-coverage` 1:500: total 2.66 (limit 0.3): nose skin, tongue and pads are each at the 75 mm feature floor (0.0184 m2 as a ball) against declared 0.036 together: floor finding, proposal in `author-requests.md` `## 59`.
Saw (sheets of all four scales opened after the last run): a labrador with barrel trunk, hocks, elbows, ears, tail; the plan is a barrel, not a bow-tie; 1:500 reads as a generalised dog with fat legs and a blocky tail root (3D tail 61 mm thick where the drawing is thinner, blue at the tail end); structure is a skeleton inside the body; plume is the vertical ring above the back (the 0.3 m emitter disc as before).

## dairy-cow (spliced)

Solids: 1:50 40/42/63; 1:100 40/42/59; 1:200 29/37/33; 1:500 21/33/33.
By hand: as author 25 (patches, rumen, teats, jaw, diagonal ear flaps rolled 22 degrees); muzzle an ellipsoid sized to the declared 0.03 (+21 %), eyes 18 mm balls sized to the declared 0.002 (outer 0.0019); 1:500: three tail lofts, legs from the drawn open strokes with the cannons now on the leg lines, ear span bar on the drawn stroke, members of forearm/shank shortened to the straight part of the stroke (62 %), thermal muzzle and eyes as small solids (they were bound to the whole head loft and counted 0.19 m2 each).
Check (product, final): silhouette passes 1:100, 1:200, 1:500 (1:500 0.956/0.907/0.931); views-disagree, bounds, feature, bones, monotonic, cross-scale, copied, structure-coverage pass at every scale (structure outside body at most 0.023 m at 1:200 against 0.03). Fails: `volumes-silhouette` 1:50 `p95M:front` 0.0469 against 0.03 and soft 0.0375 against 0.03 (cell 3.75 mm: 12.5 cells; the slab corners of the front hindbody drawing, tool change documented in `tool-requests.md` `## 59`); `volumes-thermal-coverage` 1:50 and 1:100: lower-legs +42 %/+41 %, hooves 5.1 times (0.306 against 0.05: the drawn hooves, `author-requests.md`); 1:500 total 0.475 (lower legs 0.715 against 0.30 at 125 mm). 1:200 passes (total 0.30 within).
Saw (all four sheets): grazing cow with long hanging head, barrel, udder with four teats, diagonal ears, structure with oblique ear members; 1:200 is two big faceted masses (the drawing at that scale is two polygon masses) with thin legs; 1:500 legs follow the strokes and reach the ground; 1:500 head and neck are one cone, ears one bar.

## horse (spliced)

Solids: 1:50 43/46/67; 1:100 43/45/58; 1:200 40/42/35; 1:500 24/35/35.
By hand: as author 25 (pelvis and lumbar lofts, mane sheets, forelock, tail lofts, ears) with the plan loin repair, no thigh narrowing, no end shrink on pelvis (a 19 mm notch at the croup caused p95 18.75 mm at 1:50), eyes 22 mm balls and muzzle ellipsoid sized to the declared areas, undrawn pastern and hoof members on the cannon line.
Check (product, final): silhouette, bounds, feature, bones, monotonic, cross-scale (1:200 front p95 0.09 -> within), copied, views-disagree and structure-coverage pass at all scales (IoU 0.992/0.944/0.939 at 1:50; 1:200 front 0.937 and 1:500 structure front within). Fails only `volumes-thermal-coverage`: hooves +76 % (1:50, 0.211 against 0.12) and +73 % (1:100); 1:200 total +68 % (hooves 0.213); the other regions (muzzle 0.0135, eyes 0.0028) are within the tolerance at 1:50 and 1:100 (muzzle 0.0135 of 0.015, eyes 0.0028 of 0.003); 1:500 passes.
Saw (four sheets): a standing horse with lifted foreleg, neck, crest mane, tail; the trunk still reads as three drums (pelvis, barrel, chest with a visible seam at x = 0.2 m, the plan shows the seam line), the shoulder and thigh lofts are slab-like at the flanks, the head is long and wolf-like with large ears; plan shows 3D pelvis corners beyond the drawn capsule.

## Imperfect, stated plainly

- Cow 1:50 front p95 (3.75 mm cells) and the hooves, lower legs of the cow, horse hooves: model findings, not corrected by shrinking or thickening.
- Dog 1:500 total: scale floor (feature cube of three tiny regions).
- Dog tongue is inside the closed mouth (pose standing-alert); after narrowing to 42 mm its outer area is within the 30 % at 1:50 and 1:100 (0.333 before).
- Horse trunk seam and slab shoulders; cow 1:200 is two big masses because its drawing is.
- Plan ears of cow: 3D flap narrower than the drawn capsule (floor 36 mm).
- The horse and dog structures of earlier authors kept their joint/member counts.
- Tool/rule notes: `tool-requests.md` `## 59`, `author-requests.md` `## 59`.
