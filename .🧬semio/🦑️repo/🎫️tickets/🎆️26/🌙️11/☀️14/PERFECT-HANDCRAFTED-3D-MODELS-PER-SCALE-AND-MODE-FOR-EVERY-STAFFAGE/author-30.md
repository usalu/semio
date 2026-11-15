# Author 30 report (round 2): person-with-umbrella, person-on-crutches, parent-carrying-toddler

Scripts: `author_30_lib.py` (profiles, station lofts, nested shells, structure helpers), `author_30_umbrella.py` (one script per asset, `all <scale>` rebuilds body, thermal, structure and decisions of a scale from the previous author's person lofts plus my own parts), `author_30_ins2d.py` (text-level repairs of the catalog 2D), `author-30-*.ts/py` (dumps, per-solid exposed area, probes). Tool output under `🗑️generated/author-30/`.

Finding that applies to all assets: the default check is the PRODUCT validator ([p]); its burial test treats every open loft as a closed solid through ray parity, so an open canopy or open collar buries everything under it. All solids are therefore closed. `sectionFaces` must sit on the thermal reference too.

## 1. humans/person-with-umbrella (spliced)

Counts body/thermal/structure: 1:50 55/85/41, 1:100 43/47/37, 1:200 35/39/25, 1:500 23/30/18 (slender: 3/3/3/2). Check (`volumes-check.ts`, all rules, all scales): 2 measurements fail, both `areaDeviation:torso-clothing` (1:50 0.81, 1:100 0.756): see "imperfect". Everything else is green, including cross-scale.

Modelled by hand:
- Umbrella at every scale: canopy as a revolve of the drawn dome profile (11/7/6/4 points, rim hem band at 1:50-1:100) with a bored axis (so the shaft stays exposed) and `sectionFaces: ["-u"]` on the flat underside (one-sided patch); slender shaft (r 8.8/9/6/37.5 mm as the scale draws it), J handle as a capsule chain on the drawn elliptical hook (10/8/5 capsules, none at 1:500), hub and ferrule, two ribs as slender rods on the rib bones.
- Thigh split into trouser loft and a raincoat skirt loft that is wider sagittally (elevation) and 13.5 mm wider laterally, ending at the drawn hem, so trousers show only below the hem; hip trousers as an inner solid 30 mm inside the coat; shoes as stepped station lofts (ankle shaft at 1:50, cut at the trouser hem for the thermal shell), cuffs as closed rings, shirt collar as a closed ring, hair cap and hands as thermal-only solids at 1:500.
- 1:500 person regenerated from the 1:500 drawings (torso/pelvis merged, head, arms, thighs, shanks, 2-8 sections), not from the 1:50 list.
- Thermal at 1:50: nested shells per layer (coat torso, sleeves and skirt, trousers, shoes), outer layer by reference; coarser scales one surface solid per part. Structure: member cores, joints (1:50/1:100), masses, shaft/handle/ribs by reference.

2D repairs at the root (catalog asset, text-level edits): (a) canopy-core-front/back at all four scales filled the hollow sliver between rib chord and the canopy underside; (b) 1:200/1:500 front shaft and ferrule were filled polygons while the elevation draws the shaft as an open stroke (whole-asset height disagreement 91 mm): both now open strokes; (c) 1:200 and 1:500 got the missing front and plan `hips`, 1:500 got the missing elevation `pelvis` and `head`; (d) the 1:500 elevation `figure` was a compound path whose overlapping subpaths (torso, pelvis, head, hair) cancelled by even-odd fill (hole at the waist): reduced to the torso subpath; (e) the 1:500 front `sil-torso` carried two arm blocks that do not follow the posed arms: replaced by the 1:200 torso outline.

`areaM2` changes (non-body regions, within factor 1.5, from the 1:50 model): canopy 0.87 -> 1.15 (dome area, one-sided, horizontal-plate-up), umbrella-frame 0.08 -> 0.064. No body-model region was touched.

What I saw (render sheets of all four scales opened after the last change): overlays green at 1:50 (0.946/0.963/0.995), 1:100 (0.928/0.957/0.998), 1:200 (0.926/0.907/0.973), 1:500 (0.921/0.930/0.967); dome, rim band, J handle and ribs read as an umbrella in the axonometric; person is still a segmented mannequin with faceted limbs (superellipse lofts), not an anatomical model; structure shows ribs, shaft and limb cores inside the ghost body.

Imperfect, stated plainly:
- `torso-clothing` (shirt collar, 0.037 m2): the collar is fully covered by the raincoat collar and the head; the honest ring exposes 0.007 m2 (-81 %). `areaM2` of body-model regions is not changed (C18); I did not stretch the ring to hit the number.
- Shoes expose 0.20 m2 at full length against 0.128 declared when the ankle shaft is included; the thermal shell is cut at the drawn trouser hem which brings it within 30 %.
- Ribs are buried inside the solid canopy dome (only two of the eight real ribs exist in the rig); the declared frame area was reduced to what the shaft, handle and hub expose.
- The canopy is a solid lens with a bore; it is a thin fabric in reality.

## 2. humans/person-on-crutches (spliced)

Script `author_30_crutches.py` on top of `author_30_person.py` (generic per-region person lofts from the scale's own drawings). From scratch at every scale (no 08 lofts). Counts body/thermal/structure: 1:50 35/61/32, 1:100 35/38/30, 1:200 26/32/19, 1:500 20/26/17. Check, all rules, all scales: 13 measurements fail (was 28): 1:50 hoodie 0.311, shorts 1.09, sneaker 0.44, crutch-grips 0.78, crutch-tips 0.83; 1:100 shorts 1.13, sneaker 0.40, crutch-grips 0.80, crutch-tips 1.07; 1:200 bounds width 0.033 (limit 0.030); 1:500 silhouette iou 0.879/0.782/0.806 (limits 0.90/0.88/0.88). Cross-scale, structure, monotonic, feature, bones: green at all scales.

Modelled by hand: per garment lofts (bare thigh/shank inside a wider loose-shorts loft, left shank inside the boot shaft, hoodie over the hips plus a hood ellipsoid, 12.5 mm hoodie pad over the shorts at the hips); crutches: tapered tube loft whose centre line follows the drawn splay (w moves 0.1 m outward), tip loft, grip capsule from the drawn handle axis, bracket rod and elliptical cuff ring on each forearm; 1:200/1:500 real thermal solids where the scale draws no part (hair cap, cuff rings, tips, grips). Thermal at 1:50: nested shells (hoodie, shorts, boot, sneaker, hair), skins by reference.

2D repairs (root, text-level): 1:50 plan got the missing `crutch-tip-l/r` (front 650 mm vs plan 637 mm whole-asset width, views-disagree 0.013 at the limit); 1:200 and 1:500 elevation got the left crutch tube (only the right was drawn); 1:100 hips copied into the 1:200 and 1:500 front and plan; 1:500 front `sil-torso` lost two arm blocks that do not follow the posed arms (replaced by the 1:200 torso outline).

`areaM2` (non-body regions, within factor 1.5): crutch-cuffs 0.04 -> 0.06, crutch-tips 0.006 -> 0.009. No body-model region touched.

What I saw (all four sheets opened after the last change): 1:50 overlays 0.948/0.922/0.897 and nested shells in the cutaway; 1:100 0.921/0.912/0.898; 1:200 0.912/0.902/0.882; 1:500 0.879/0.782/0.806 with 82 mm crutch tubes (smallest feature 75 mm) that are wider than the drawn 52 mm and show as blue in the front and plan overlays; the persons are faceted lofts.

Imperfect, stated plainly:
- Shorts expose 0.42 m2 against 0.197 declared: the drawn loose shorts (skirt shapes 0.32 m long, 0.22 m wide) are about twice the declared surface; body-model area (C18) not changed, model follows the drawing.
- Grips expose 0.0086 of 0.025 m2 because the hand loft covers most of the handle; tips 0.0164 of 0.009 (the 2D tip is 70 mm tall, a real tip is about 45 mm); sneaker 0.10 against 0.0697; hoodie -31 % (just over 30 %).
- 1:500 silhouettes fail: the 2D at 1:500 draws arms as open strokes and merged silhouettes; front and plan are blocked by the 75 mm feature rule on the crutch tubes and by an unresolved neck/pelvis band. 1:200 bounds: the asset width (x extent) is 33 mm short of 0.556 because the 1:200 hand/forearm ends were not extended.

## 3. humans/parent-carrying-toddler (spliced)

Script `author_30_toddler.py` on `author_30_person.py`, built from the POSE profiles (`author-30-posprof.ts`: shapes and bone frames of `standing-carry`, because the toddler's limbs are not vertical in the rest pose). From scratch at every scale. Counts body/thermal/structure: 1:50 32/67/40, 1:100 31/38/38, 1:200 29/38/29, 1:500 25/36/27. Check, all rules, all scales: 20 measurements fail (was 28): 1:50 silhouette `p95SoftM:plan` 0.077 (limit 0.03) and thermal scarf 0.665, gloves 0.867, jeans 0.39, boots 0.85, toddler-face 0.644, toddler-suit 0.498, toddler-mittens 0.501, toddler-boots 0.996; 1:100 `p95SoftM:plan` 0.075 (0.06) and the same eight thermal regions (scarf 0.81, gloves 0.90, jeans 0.31, boots 0.75, toddler-face 0.64, suit 0.47, mittens 0.43, boots 0.95); 1:200 green; 1:500 silhouette iou elevation 0.889 (0.90) and plan 0.845 (0.88). Bounds, cross-scale, structure, monotonic, feature, bones and views-disagree are green at every scale.

Modelled by hand: parent and toddler as two figures of per-garment lofts (head and beanie, torso and scarf, toddler head and hat, toddler shank and boot); flat toe lofts where the elevation draws the whole boot on the foot; scarf ring, toddler hat cap, glove and mitten blobs as thermal-only real solids at the scales that draw no such part; member capsules from the bone length for member bones without body solid at 1:500; nested shells at 1:50.

2D repair: 1:50 plan `shoulders` ellipse lateral radius -5 mm (whole-asset width 460 vs 450 mm between plan and front).

`areaM2`: nothing changed (all affected regions are body-model regions, C18).

What I saw (all four sheets opened after the last change): 1:50 overlays 0.971/0.953/0.925, 1:100 0.965/0.951/0.916, 1:200 0.935/0.913/0.898, 1:500 0.889/0.886/0.845; the toddler reads as a second figure held on the parent's arm in the axonometric; the far toddler leg and arm are thin lofts; the toddler's 2D plan draws its legs horizontal while the front draws them hanging, so the plan overlay keeps a red block at the toddler's feet (the p95 soft-mass measurement).

Imperfect, stated plainly: the thermal area findings are physiological-area mismatches that I did not hide by scaling: scarf exposes 0.026 of 0.079 m2 (buried by coat collar and head), gloves 0.019 of 0.097 (buried by the toddler suit and sleeves), jeans -39 %, parent boots +85 % (0.2 against 0.106), toddler face +64 %, toddler suit 0.29 of 0.58 (the drawn toddler is smaller than the declared suit), toddler mittens -50 %, toddler boots +100 %. 1:500 plan: the toddler's feet are drawn at x 0.31-0.45 m and no loft reaches there.

## Summary of what remains open (coordinator)

- All three assets are spliced with failing measurements named above (umbrella 2, crutches 13, toddler 20), none hidden by weakening a tool or scaling coverage.
- Tool/ruling observation (not written to tool-requests): `volumes-feature` summary `minDimensionM` fails whenever any non-thin solid is thinner than the scale's feature even when each solid passes its own C13 limit (1:500 crutch tubes, toddler boots): C13 is applied per solid but not in the summary line.
- Tool observation: the product measurement treats an open loft as a closed solid by ray parity in the burial test, so any open canopy/collar buries all regions below it.
- Product 2D observation: compound paths (`d` with several subpaths) are filled even-odd; the 1:500 umbrella `figure` shape lost its waist to overlapping subpaths.
