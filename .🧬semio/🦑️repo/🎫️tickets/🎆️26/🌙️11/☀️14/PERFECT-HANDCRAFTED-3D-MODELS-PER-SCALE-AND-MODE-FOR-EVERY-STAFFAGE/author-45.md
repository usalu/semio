# Author 45 report (round 2): humans/kneeling-gardener, humans/leaning-person, humans/standing-adult

Status: leaning-person and standing-adult are green at every authored scale under `volumes-check.ts <asset> --product` and spliced. kneeling-gardener has a real hat now and is red on exactly one measurement (`areaDeviation:hat`, 0.594 at 1:50, 0.788 at 1:100): the hand calculation (below, and in `area-review.md`) shows that the declared 0.226 m2 is the one that is off. It is NOT spliced.

Scripts (all in T, inputs): `author-45-gar-hat.py`, `author-45-gar-hair.py`, `author-45-gar-decisions.py`; `author-45-lean50a.py`, `-lean50b.py`, `-lean50c.py`, `-lean100.py`, `-lean200.py`, `-lean500.py`, `-lean500b.py`, `-lean-decisions.py`; `author-45-sta.py`, `author-45-sta-decisions.py`; tools `author-45-descent.ts` (coordinate descent of per-section ry and v-centre on the PRODUCT silhouette), `author-45-ext.ts` (which solid sets the min/max of an axis), `author-45-band.ts` (x-extent of the drawn elevation per height band). Method from author 33 (hull tool, exposed tool, joints tool, kit coarsen, `author_33_lib.py`). Rebuild of leaning-person = lean50a, b, c, lean100, lean200, lean500 (kit-fit `--sections --shift --exp --write`, then `author-45-descent.ts`), lean500b, decisions.

## humans/kneeling-gardener (not spliced: one red measurement, the hat)

Counts body / thermal / structure: 1:50 44 / 60 / 42, 1:100 28 / 29 / 41. 1:200 and 1:500 stay `unavailable`: the representations are unavailable (`below-printable-feature`, the 2D drawings do not exist), and `volumes-availability` mirrors that, so there is nothing to author. Decisions 3, 2 (en + de).
Check line: 1:50 and 1:100 fail only `areaDeviation:hat`; every other measurement, incl. silhouette (IoU 0.958 / 0.965 / 0.971 at 1:50), bounds, feature, views-disagree, structure, bones, copied, monotonic, cross-scale, is ok.

By hand:
- Hat: the 4.7 cm brim puck is replaced by a hollow dished brim (revolve with a closed profile: r 0.085 to 0.2355 m, 9.5 mm thick at 1:50 and 15.5 mm at 1:100, drooping 31 mm to the rim; the crown dome and band are the existing lofts). The brim hair-pack shell is gone (hair does not lie under a brim), brim straw is a `{ of }` reference.
- Hair: with the puck gone the skull skin above the brow was exposed (face region 0.3655 off). A `hair` loft (new asset material `hair`, root edit of `materials` in the catalog asset: emissivity 0.95, absorptivity 0.8, ashrae-2021) covers back and top of the skull (hair mass sits under the hat, bound to region `hat` whose title is "Straw hat and hair"); face now 0.06 m2 of 0.0685 (-12 %), ears and nose unchanged.
- Hand calculation of the hat (also in `area-review.md`): brim one face pi (0.2355^2 - 0.10^2) = 0.143 m2, both faces 0.286 m2, rim 0.014 m2; crown 0.061 m2 and band 0.009 m2. One-sided brim + crown + band = 0.213 m2 = the declared 0.226 m2 within 6 %; both faces = 0.37 m2, which the model exposes (0.360 m2 at 1:50). The declared value counts the sky-facing side of the brim once (`geometry: horizontal-plate-up`), C1 counts every face not buried, and the underside of a brim is exposed. Proposal: `areaM2` of `hat` 0.37. Not changed by me (C18).
- 1:100: shirt 3 % fuller (shirt-torso 0.3010 -> 0.2914 after a validator change moved the number).
Seen (both sheets after the last change): a kneeling figure with a wide straw hat with a visibly thin dished brim and domed crown, leaning trunk, weeding arm with trowel into the trug, foam pad; nested shells in the thermal cutaway; structure tubes along legs, trunk, arms, plus pad, trug, trowel. Overlays green; the elevation shows thin red slivers where the 2D draws the brim as a 4.7 cm band. Imperfect: that 2D brim band (a puck in the drawing; I did not redraw it, the 3D brim therefore leaves 2-3 mm red slivers); the trug carries a polyethylene weed dome; structure joints are plain balls.

## humans/leaning-person (spliced)

Counts: 1:50 51 / 77 / 38, 1:100 27 / 30 / 38, 1:200 23 / 26 / 22, 1:500 19 / 25 / 21. Decisions 4, 3, 3, 3 (en + de).
Check line: PASS at 1:50, 1:100, 1:200, 1:500 (all rules, `[p]`).

By hand:
- 1:50: the 07 seed kept (loft hoodie, pelvis, limbs). Repairs: mouth and hood strings 6 -> 8 mm (feature); forearm sleeves cut at 0.24 m so that the wrist skin region has its own solids (`wrist-l/r`), hand lofts start at the wrist (they had buried it); cuffs; a hoodie hem and a rib band over the pelvis; torso exponent 3.6 and 4 % fuller, hood 12 % bigger, pocket 120 x 270 x 16 mm, sleeves 3 % fuller (limited by the depth bounds: the arms span 0.459 of 0.45 m). Thermal by `kit-thermal` with hand-bound regions (`--map`), details pruned from the layer shells, shells rebuilt with the real thickness. Structure from `person_structure` and joints fitted inside the body.
- The wall (bone `wall`, 2D role `ground`): a 0.38 m masonry box (layers: 15 mm plaster inside, 365 mm brick outside) with `role: ground`, `texture: true` so that it stays out of silhouette and bounds like the 2D does; thermal: the brick box is the `{ of }` reference with `sectionFaces: ["+v"]` on the reference (the front face is in the contact plane of the wall anchors and does not count under C1(b); back and sides give 3.72 m2 against 3.5 m2); 15 mm plaster slab at the back.
- Wall bones in structure: the three massless wall bones are load-bearing for the validator; their members are three 80 x 120 x 220 mm contact pads inside torso and pelvis (invented, see tool-requests.md).
- 1:100: kit coarsen of 1:50 (51 -> 31 body solids), shoulder caps and pocket kept, wrists re-added (40 -> 24 mm), hands trimmed to the wrist, phone 132 x 34 x 64 mm, sleeves 3 % fuller, regions bound by hand (the kit had bound the hand to the phone surface and the phone to the hand skin).
- 1:200: kit coarsen of 1:100 (26 -> 24), then hair, shoulders and pocket removed, 40 mm wrists.
- 1:500: modelled anew with `author-33-hull.ts` per bone, head and pelvis by hand (the drawing has no pelvis elevation; the hood is a 14 mm sliver), `kit-fit` on the three views, then coordinate descent on the product silhouette (elevation IoU 0.880 -> 0.904, front 0.939, plan 0.929); phone as the 75 mm minimum box.
- No areaM2 changed. C18 findings, not changed: torso-clothing exposes about 0.56 of 0.80 m2 (-28 %, silhouette-limited, arms bury the sides) and arm-clothing 0.19 of 0.27 m2 (-30 %), both at the 30 % edge (1:50 deviation 0.284 and ~0.27).
Seen (all four sheets after the last change): a person in oversized hoodie and beanie leaning back against the wall, hands in front with the phone, overlays green with thin rims; at 1:50 hood, pocket, cuffs and wrists read; at 1:500 the figure is a lofted generalisation. The thermal cutaway shows nested shells inside the body and the hatched wall; structure shows the tubes and joints in the person and the pads. Imperfect: the wall box dominates the axonometric render (it is hatched and texture-flagged); torso-clothing and arm-clothing sit at the 30 % edge; the 1:500 elevation IoU margin is 0.004 (descent-fitted to the validator, the kit-fit raster alone gave 0.88); at 1:500 the hands carry the arm-skin region too (no wrist is drawn), the head loft carries face and hair; wall pads are invented structure.

## humans/standing-adult (spliced)

Counts: 1:50 34 / 49 / 32, 1:100 29 / 28 / 29, 1:200 20 / 23 / 28, 1:500 18 / 21 / 24. Decisions 3 per scale, rewritten with real content (en + de).
Check line: PASS at all four scales.

By hand:
- `dimensions.widthM` 0.34 (author 10, was 0.328) verified against the drawings: elevation spans 0.342 m (1:50), 0.337 (1:100, 1:200), 0.326 m (1:500), hood back to toe; kept.
- 2D repair at the root (`catalog/👤️humans/standing-adult/🔣️.json`, 1:50 plan): the hood ellipse centred at -0.0617 (rx 0.065) ended at x -0.1267 while the elevation draws the hood to -0.1354, a 9.5 mm whole-asset length difference (`views-disagree` 0.0094 > 0.0075; this, not the arms, was the failing extent); now cx -0.0667, rx 0.0695. Backup: `🗑️generated/author-45/standing-asset.before-2d-fix.json`.
- Thermal: ears and nose (exposed skin, unbound before) are `{ of }` references of the face-skin region at 1:50 and 1:100 (face-skin deviation 0.309 -> 0.26 at 1:50); at 1:100 the ears and the nose are body solids again (they had been merged).
- 1:100: the sneakers exposed 0.142 m2 against 0.1005 (+41 %) because the bare shoe bottoms were exposed; the 1:50 sole lofts (raised to the 15 mm feature) are back under the shoes.
- No areaM2 changed (author 10's changes had been restored by the coordinator).
Seen (all four sheets): a standing figure in a hoodie with bunched hood, jeans, sneakers; overlays green; shells nested, structure tubes with joints and masses inside the silhouette. Imperfect: at 1:500 the hands are 75 mm columns, the neck is a hidden yoke and the plan IoU margin is small (0.909 of 0.88; elevation 0.911 of 0.90); hoodie -26 % and hands -22 % against the declared areas at 1:50 (within the 30 % of C18).

## Requests and findings filed
- `area-review.md`: gardener hat (calculation; proposal areaM2 0.37).
- `tool-requests.md`: wall bones counted as load-bearing with member containment in the body silhouette; `sectionFaces` is only honoured on the thermal reference itself, front face of a leaned-on wall excluded as contact plane; the validator stopped counting emitter discs between my 1:100 runs (leaning-person torso-clothing 0.14 -> 0.31 at 1:100 without any change by me, gardener shirt-torso 0.27 -> 0.30).
