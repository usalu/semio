# Author 28 report

Order: humans/cyclist (finished except the findings below, NOT spliced), humans/child-on-kick-scooter (built at all four scales, unfinished, NOT spliced), humans/courier-with-hand-truck (not started).

Tools made (all in T): `author-28-opt2.ts` / `author-28-opt3.ts` (coordinate descent plus random search over solid parameters against the PRODUCT silhouette `measureAsset`, exact same IoU as `volumes-check.ts`; opt3 has scale-aware floors), `author-28-blame.ts` (per-solid IoU delta), `author-28-mask.ts` + `🗑️generated/author-28/sil-copy.ts` (PNG of the product's drawn / ink / volume masks; the copy must be regenerated when the product swaps), `author-28-prod*.ts`, `author-28-saddle.ts`, `author-28-areas.py`, `author-28-tri.ts`, `author-28-scooter.py` (scooter builder, all scales), `author-28-2d-cyclist.py` (superseded, see 2D).

## humans/cyclist
Counts (body / thermal / structure): 1:50 83/169/56, 1:100 53/56/53, 1:200 48/51/49, 1:500 42/45/43; bilingual decisions at every scale (author 22's).
Product check (`volumes-check.ts`, rulings through swap 12):
- 1:500 PASS (silhouette 0.919 / 0.920 / 0.912, previously 0.886 / 0.855 / 0.830 on the owned engine; 0.617 / 0.875 / 0.829 on the product engine of that moment).
- 1:200 PASS.
- 1:50: only `areaDeviation:saddle` 0.52 (limit 0.15).
- 1:100: `areaDeviation:saddle` 0.52 (0.30), `helmet` 0.309 (0.30), `gloves` 0.424 (0.30).
Why not spliced: those four measurements. Helmet and gloves at 1:100 are body-model regions (C18: areaM2 untouched): exposed 0.0916 vs 0.07 and 0.0564 vs 0.0983 (the 1:50 model exposes 0.0709 and 0.0734, ratios 1.01 / 0.75), so the 1:100 helmet shell is a little large and the 1:100 gloves a little small; I did not tune them in the remaining budget.

What I changed by hand
- Saddle question (answered in `area-review.md`): the model is right, the value is the surface of the whole saddle. Closed loft surface 0.0724 m2 (1:50) / 0.0739 (1:100) equals the declared 0.07 (a real 270 x 155 mm city saddle has about 0.07-0.08); under a seated rider the pelvis buries 0.0126, thighs 0.0075 + 0.0069, leaving 0.033 (ratio 0.47, outside the 1.5 of C16, so not applied). Needs a coordinator decision (0.033 or exempt).
- 1:500: product-metric optimisation of every solid parameter (sections, lateral/sagittal offsets, tube ends), shoulder yoke `neck` (it was 0.47 m wide, a 0.23 half-width plate on the neck bone) replaced by a real 77 mm neck, hair tail tucked inside the helmet (a 77 mm tail is bigger than any real ponytail, the 2D head shape contains hair), helmet/torso fitted. Floors kept (77 mm human, 65 mm tubes, 100 mm bars). 
- 1:200: restored author 22's body (the optimiser overshot front p95) with the same shoulder fix: neck sections limited to rx 0.05, collar to 0.17 (they had 0.23 half-width wedges at the shoulders).
- Tyre areaM2 (passive, C16 factor 1.5 of the committed 0.6): author 22's 0.66 was measured on an old engine; 1:50 exposes 0.448, 1:100 0.609; set 0.5 (deviation 0.10 at 1:50, 0.18 at 1:100). Other areaM2 values are as left by the coordinator's restore.
- 2D root repair: at 1:200 the plan tyre ellipses `rx` 0.34 -> 0.358 (the elevation hoop is widened by the 0.18 mm pen to 0.358, `volumes-views-disagree` x was 0.036 > 0.0353). I first converted the 1:200/1:500 hoops and 1:100 fender arcs to filled annuli because the product then drew a 0.3-0.8 m wide member ink on strokes that cross a fork or stay (tool-requests.md "28 humans/cyclist"); the product fixed that itself (swap 10) so I restored the original 2D (`cyclist.asset.before.json` identical except the lines above).

What I saw: sheets of 50, 100, 200, 500 read after the last change: overlays green with thin red/blue only at glove tips, helmet rim, shoulders; axonometric rider and bike read correctly (faceted 16-gon tyres at 1:500 intended); thermal cutaway shows nested clothing at 1:50 and single shells coarser; structure shows a believable skeleton plus every frame tube. Imperfect: 1:500 face-skin and helmet exposed about twice areaM2 (feature floor on the head; total rule passes); 1:100 findings above; the 1:500 hair is buried in the helmet.

## humans/child-on-kick-scooter
State (source `volumes/humans/child-on-kick-scooter.json`, builder `author-28-scooter.py`): body / thermal / structure 1:50 60/106/46, 1:100 31/34/42, 1:200 29/34/40, 1:500 26/33/35, all with bilingual decisions and support pads under both wheel contacts. Not spliced, not finished.
Findings so far (product check): 
- 1:100: silhouette 0.980-0.901 passes; open: `sneakers` exposed 0.097 vs 0.0677 (0.44, C18, the loft follows the drawn shoe, the declared value looks low; raising the deck 12 mm only buries 0.008), `stem` 0.175 vs 0.12 (1.46x; apply 0.12 -> about 0.148 from the 1:50 model), `grips` fully buried by the hands (info only, C21).
- 1:50: bounds depth fixed in the builder (handlebar to 0.225), grips lowered so their ends are exposed; remaining by the earlier run: sneakers 0.42 (as above), backpack 0.18, stem 0.24, wheels 0.17 against 0.15 (areaM2 to be set from this 1:50 model within 1.5: backpack 0.213, stem 0.148, wheels 0.076), `views-disagree` z 0.014 > 0.009 (front/plan width 2D).
- 1:200: after optimisation silhouette 0.917 / 0.948 / 0.911, rest unverified after the last write.
- 1:500: elevation 0.916, front 0.927, plan 0.859 (< 0.88): a 2D inconsistency, the plan draws hands/grips about 0.05 m further forward (x 0.45 at plan) than the elevation (x 0.6), no 3D can satisfy both; fix the 2D at the root, then rerun `python3 author-28-scooter.py 500` and `author-28-opt3.ts`.
Neck: the drafted `neck` loft had 0.157 half-width yoke ends (face-skin exposed 0.087 vs 0.058); replaced by a 90 mm neck at 100/200/500 (0.059).

## humans/courier-with-hand-truck
Not started. The scooter builder is the template (humanize, snap, product-metric optimiser, `fill_missing`, `capsule_to_loft` for the monotonic triangle rule at 1:500 because a capsule costs 1024 triangles).
