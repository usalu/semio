# Author 25 report: labrador-dog, dairy-cow, horse (round 2)

All three assets are re-modelled at all four scales and spliced (`volumes-insert.py`, last splice after the final run). None is fully green: the remaining failing measurements are listed per asset with numbers. `areaM2` was not changed anywhere; the declared values of the small skin regions that the honest model cannot reach are in `area-review.md` (25 rows).

## What changed against author 05

Author 05's generator produced lateral-plate organic forms (chest sections 9 mm thick, shifted 6 cm off the midline, `--shift` fits of the kit make the same plates), exponent-9 boxes, a fill capsule for the head axis, pads on the midline between the legs. I did not use the generator or the fitting kit for the bodies. New builder (inputs kept in `T`):

- `author-25-lib.ts`: a loft along a bone from the scale's own drawings. Sagittal extent per station from the elevation outline in bone-local coordinates, lateral half width and centre from plan (horizontal bones) or front (vertical bones) at the world position of the section, section exponent chosen by anatomy (trunk 3.0, thigh and shoulder 2.8, limbs 2.5, head and neck 2.4, tail and ear 2.2), end sections shrunk, floors from the thinnest drawn extent (C13). Sections carry an explicit `rotation` (the kernel's frame without rotation flips between lateral and sagittal on near-parallel tangents). Ear flaps are rolled about their bone axis (20 to 22 degrees). Thighs narrow towards the hip.
- `author-25-quad.ts`: one loft per drawn bone with a per-animal table (names, tones, exponents, twins such as the far ear following the near ear, tail bones cut from one drawn tail, lateral sources such as the head-axis bone), structure builder (capsule member per eligible bone B1 inset from the bone ends, joint ellipsoids at 1:50 and at 1:100 for members of at least 20 mm, mass ellipsoids for bones with at least 10 % of the mass, no supports because the three assets declare none), `surfaceW`.
- `author-25-dog.ts`, `-cow.ts`, `-horse.ts`: per-asset configuration, hand-set parts (eyes, noses, tongue, pads, teats, patches, muzzles, mane and forelock sheets), thermal extras, bilingual decisions (`author-25-text.ts` plus per-scale paragraph). `author-25-run.sh <dog|cow|horse> <scale>` runs body, kit-thermal with an explicit region map, thermal extras, structure, decisions. 2D repair scripts: `author-25-filled.py`, `-redraw.py`, `-addshapes.py` with `-addfoot.ts` and `-simplify.py`, `-cmp.py`. Diagnostics: `author-25-probe.ts`, `-drawn.ts`, `-each.py` (checks every structure solid alone against its scale), `-culprit.py`.

## 2D drawing defects repaired at the root (text patches of the asset files)

- `filled: false` on closed body shapes: dog 1:200 (far/near hind lower, tail) and 1:500 (12 shapes), cow 1:200 (tail-1, tail-2, jaw), horse 1:100 (ears), 1:200 (jaw, ears, mane), 1:500 (fore lower, tail lower, ears).
- Dog 1:500 elevation: the four open leg strokes had `penMm` 0.49 to 0.84 (a leg 0.24 to 0.42 m thick on a 0.72 m dog, pens computed with the 1:100 scale); set to 0.2 mm. Then all 17 body shapes of that view were found to carry outlines that do not belong to their bones (hind foot triangle 0.2 m forward of the foot, neck wedge over the chest, tail polygon not in the tail frame; local bounding boxes 8 to 27 cm away from the 1:200 shapes of the same bones). Redrawn from the 1:200 outlines of the same bones, Douglas-Peucker 12 mm, closed and filled (`author-25-redraw.py`): elevation IoU 0.21 -> 0.87.
- Cow 1:500 elevation: tail outline was not in the tail frame (whole-asset height of elevation and front differed by 0.49 m): redrawn from the 1:200 tail; the lower legs (cannon, hoof) were missing, four shapes added as the hull of the 1:200 cannon and hoof outlines in the cannon frame.
- Horse 1:500 elevation: seven outlines (four lower legs, upper and lower tail, neck wedge) replaced by hulls of the 1:200 outlines in the bone frame, simplified to 12 mm.
- Not repaired (reported in `author-requests.md`): plan ears drawn as capsules longer than the elevation ears (dog, cow); cow 1:200 head and tail extents elevation against plan (53.7 against 51.3 mm).

## labrador-dog

Solids (body, thermal, structure): 1:50 37, 41, 63; 1:100 37, 40, 44; 1:200 23, 31, 33; 1:500 20, 31, 33.

By hand: 1:50 and 1:100 full dog with a 13 mm eye pair, nose leather, tongue, four pads on the underside of each paw (they were on the midline), skull solid inside the head on the head-axis bone (so that bone has a solid without a fill capsule outside the head); 1:200 merges hock and paw into one lower-limb loft, three tail lofts from one drawn tail; 1:500 legs 53 mm and thicker (below the 75 mm feature where the drawing is thinner, C13), ears, nose, tongue and pads only in the thermal list as real solids (pads sunk into the foot). Every scale has bilingual decisions (4 paragraphs).

Check, per scale after the last run: silhouette 1:50 and 1:100 pass; 1:200 plan IoU 0.862 (limit 0.88; ears and thigh corners); 1:500 elevation 0.876 and plan 0.839; bounds, feature (per solid), bones, monotonic, copied, cross-scale, views-disagree, structure-coverage pass at every scale (the aggregate `body.minDimensionM` line of 1:500 fails although every solid passes C13: `tool-requests.md`). Thermal: `volumes-thermal-coverage` fails at all four scales on nose-skin (0.55 of 0.004 at 1:50), tongue (0.24 of 0.012, mouth closed in the default pose), the 1:200 and 1:500 totals (min features of the small regions, 0.48 and 0.85) and the plume disc outside the body box (29 to 146 mm); every region has a solid at every scale and pads are within 15 to 28 %.

Saw in the renders (opened with the Read tool): 1:50 and 1:200 sheets after the final body, 1:500 sheet before the final run (bodies unchanged since): overlays green except plan ears (red) and 1:500 tail and head (blue/red), the 1:50 body reads as a labrador in all three axonometric views with a real trunk, hocks and elbows; 1:500 reads as a generalised dog with fat limbs; structure is a believable skeleton inside the body; thermal cutaway shows the plume as a large vertical ring above the back (that is the emitter, 0.3 m disc). Not opened after the last change: 1:100.

## dairy-cow

Solids: 1:50 40, 42, 63; 1:100 40, 42, 59; 1:200 29, 36, 33; 1:500 19, 30, 33.

By hand: patches (rump, barrel, chest) and the rumen flank as `texture: true` lens skins flush on the near side of the trunk (not whole rings), four teats under the udder at the lateral offsets of front and plan, muzzle and eyes, ears rolled out of the sagittal plane, jaw with a fallback half width (no plan or front jaw shape), 1:500 legs from the drawn open strokes at 125 mm, ear span as a 100 mm bar (the drawing is one stroke across both ears).

Check: silhouette passes 1:50 to 1:200; 1:500 plan IoU 0.860 (rump corners drawn square), front 0.884, elevation 0.943. Fails: `volumes-views-disagree` 1:200 (53.7 against 51.3 mm, 2D, requests), structure-coverage: ear members and joints leave the rolled ear flaps by 20 mm in plan at 1:50 and 1:100 (found with `author-25-each.py`, not solved), 37 and 51 mm at 1:200, 140 mm in elevation at 1:500 (tail-3 and members of bones that no body solid draws at that scale); thermal: hooves 4.8 times the declared area (0.242 against 0.05, `area-review.md`), eyes 1.45, udder 0.70, 1:200 total 0.49, plume disc outside box. Patches, rumen and teats are modelled now; patches are mostly hidden flush skins.

Saw: sheets 1:50, 1:100 and 1:500 (after the last body change); 1:200 not opened. A cow with a grazing head, barrel, udder with teats in all three axonometric views; 1:500 elevation red at the lower legs where the drawing's stroke legs are straighter than the 3D lofts.

## horse

Solids: 1:50 43, 45, 67; 1:100 43, 45, 58; 1:200 40, 42, 35; 1:500 24, 34, 35.

By hand: the pelvis bone has a plan shape (the rump capsule is bound to it, not to the lumbar bone): pelvis and lumbar are two abutting lofts that split the drawn hindbody at x = 0.2 m; mane (three sheets at 1:50 and 1:100, one at 1:200) and forelock as 44 mm thick sheets on the crest, tail as four lofts, muzzle and eyes, ears rolled 20 degrees, 1:500 generalised in two trunk masses and two neck wedges.

Check: silhouette passes IoU everywhere except 1:200 front (0.869, limit 0.88; the hooves of the lifted legs are drawn at other lateral positions than the legs) and the elevation 95th percentile slightly high at 1:50 (18.8 against 15 mm) and 1:100 (33.8 against 30 mm); cross-scale 1:200 front p95 90 against 60 mm (the 2D pair itself differs by 15 mm in p95; limit not met); 1:500 structure outside the body in front by 107 mm; thermal: muzzle-skin 0.51 and eyes 0.63 of declared at 1:50, hooves 1.34, 1:500 total (min feature of muzzle and eyes) 4.9, plume disc outside box. Everything else (bounds, feature, bones, monotonic, copied, views-disagree) passes.

Saw: sheets 1:50, 1:200 and 1:500; 1:100 not opened. The 1:50 body reads as a horse with real shoulders, neck and legs, but the scapula and thigh lofts are still slab-like at the flanks (visible from the near side) and the lifted foreleg is angular; overlays all green at 1:50.

## Imperfect, stated plainly

- Nothing is fully green (see the lists). The coarse silhouettes of the dog (1:200, 1:500) and the cow (1:500) are 1 to 5 points below the IoU limits; the hind quarter of both is rounder in 3D than the squared plan drawing.
- Thermal skins of small physiological regions are buried or forced by the feature (tongue, nose, eyes, muzzle at 1:500): findings, not corrected.
- Ear flaps of the dog and cow cannot reach the plan footprint of the 2D capsules (2D contradiction, `author-requests.md`).
- Cow structure: ear members still leave the flaps by 20 mm in plan; I did not find the cause.
- Not opened after the last change: dog 1:100, cow 1:200, horse 1:100 (bodies were produced by the same run as the opened scales and pass the numbers).
- Tool defects and proposals: `tool-requests.md` (three sections `## 25`).
