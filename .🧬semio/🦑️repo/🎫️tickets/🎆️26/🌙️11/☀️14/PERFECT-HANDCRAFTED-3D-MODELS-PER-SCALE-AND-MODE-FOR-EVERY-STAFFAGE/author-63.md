# Author 63 report (round 2): person-on-crutches, parent-carrying-toddler, tour-guide, worker-carrying-box

All four assets are modelled at all four scales and re-spliced (tour-guide: see its section for the state at the end). Check lines are the product validator (`volumes-check.ts <asset> --product`, rulings through swap 14; swap 15 landed while I worked, the silhouette numbers below were re-measured after it). The validator and the area decisions changed under me several times (C26b soft outlines, C29 outer area, the reviewer's new `areaM2`), so every `areaDeviation` line below is the value at the time of the last run and not something I steered.

Scripts and tools (all in T): `author_63_lib.py` (load/save/check wrapper that prints only the failing measurements; `patch_section` rewrites ONE top-level section of an asset file textually, so 2D repairs touch nothing else), `author-63-mask.ts` (PNG of the product's own drawn/volume/soft masks; needs `🗑️generated/author-63/sil-copy.ts`, a copy of the product silhouette module with a debug hook, regenerate it when the product swaps), `author-63-soft.ts` (where the soft-mass p95 cells lie, in world coordinates), `author-63-vd.ts` (the `volumes-views-disagree` extents with the pen widening and the shape that sets each end), `author-63-shapes.ts`, `author-63-frames.ts`, `author-63-tour500.py` (tour-guide 1:500 hull build). Reused: `author-33-hull.ts`, `author-28-opt3.ts` (coordinate descent on the product silhouette), `kit-diagnose.ts`. Backups before every root repair of a drawing are in `🗑️generated/author-63/*asset.before-2d-fix.json`.

## humans/person-on-crutches (spliced)

Counts body / thermal / structure: 1:50 35/61/32, 1:100 35/38/30, 1:200 26/32/19, 1:500 20/26/17 (decisions 3 per scale, en + de).
Product check now: every rule green at all four scales except the exposed-area comparison at 1:50 and 1:100 (`face-skin`, `shorts`, `legs-skin`, `sneaker`, `crutch-tips`; the reviewer's own list names shorts, sneaker, grips, tips for this asset as model findings of the drawn loose shorts and bare thighs; the values moved several times while I worked because `areaM2` and the C28/C29 measurement were changed under me).

By hand:
- Crutch tubes back to true size: 22 mm tubes (rx = ry 0.011) with `slender: true` at 1:100, 1:200 and 1:500 (they were 25, 33 and 82 mm). Under C24 they are compared as strokes widened to the drawn pen (50 mm at 1:500), so the thin tube no longer costs silhouette. At 1:50 the tube is already true size and the 2D draws it as a filled outline with the bracket bump; marking it `slender` there made the 1:50 elevation p95 fail (24 mm against 15), so it stays a plain solid.
- 1:500 silhouette 0.879 / 0.782 / 0.806 -> 0.924 / 0.885 / 0.885: coordinate descent on the product silhouette over every section of the 18 non-slender solids (rx, ry, lateral and sagittal centre; opt3 with random search), the hood ellipsoid narrowed to the width the front view draws (it stuck out 130 mm above the shoulders in the front overlay).
- 1:200 bounds width 0.033 -> 0.017: the left heel was 19 mm short of the drawn 0.556 m asset length; heel sections moved back, then the 1:200 body refitted with opt3 (silhouette 0.926 / 0.910 / 0.911).
- Not touched: `areaM2`, 2D drawings (the earlier 1:200/1:500 left-crutch repairs of author 30 stand).
Seen (all four sheets after the last change): overlays green with thin rims; the figure reads as a person on two crutches in the tripod stand; at 1:500 the crutches are thin yellow slender lines in the axonometric view, faceted lofts for the body; thermal shells nested at 1:50, single shells coarser; structure tubes and joints inside the body.
Imperfect, stated plainly: at 1:500 the thighs and trunk are two- to five-section cones (faceted); 1:500 front IoU 0.885 and plan 0.885 have only 0.005 margin; the crutch tips (r 0.0154 to 0.0176, 3-section lofts with a 40 mm bulge copied from the drawn bump) expose 0.0166 m2 against 0.009 m2: a real rubber tip of 30 x 50 mm exposes 0.0045 m2 each, but its elevation drawing contains the bump and the silhouette limit leaves no room, so I did not touch it; the pose declares a 60 % ground contact for the tips and 50 % for the grips (C28), which the validator does not apply yet.

## humans/parent-carrying-toddler (spliced)

Counts: 1:50 32/67/40, 1:100 31/38/38, 1:200 29/38/29, 1:500 25/36/27.
Product check now: all silhouette, bounds, views-disagree, feature, monotonic, cross-scale, structure, bones, copied rules green at every scale; open are only the exposed-area lines (parent face, coat, boots, toddler face, suit, mittens, boots at 1:50 and 1:100; the reviewer lists "toddler" among the clothing models with buried layers and kept the physiological values).

By hand:
- The two `p95SoftM:plan` lines (0.077 at 1:50, 0.075 at 1:100) were NOT the hat: located with `author-63-soft.ts`, the 28 offending cells lie at x 0.318-0.326 m, y 0.049-0.09 m, the front of the toddler's head in the plan, where the plan draws a 32 mm gap between the two toddler boots (near boot y -0.014..0.054, far boot 0.086..0.154). The near boot loft (`toddler-shank-n-toddler-boots`) was a 138 mm ball centred on the far leg's plane (the generator centred its last sections at w = 0 instead of -0.04 and gave them rx = ry = 0.069), so it filled the gap. Now the boot is centred on the near leg and 74 mm wide at 1:50, 1:100 and 1:200 (same bug in all three), the soft p95 is 0.0 at 1:50 and 1:100.
- 1:500 (elevation 0.889 -> 0.931, plan 0.845 -> 0.910, front 0.886 -> 0.913): the toddler's thighs, shanks and boots rebuilt with `author-33-hull.ts` from the 1:500 elevation and plan (oblique bones; the far leg takes its elevation from the near leg because the 1:500 elevation hides it), cut to two or three sections each so that the thermal triangle count stays under 1:200 (`volumes-monotonic` thermal delta first +512, now -64), then all solids refitted with opt3.
- Not touched: 2D drawings, `areaM2`.
Seen (all four sheets after the last change): parent with a toddler on the left arm; the toddler's legs now hang as two separate legs with boots; faceted lofts at 1:500; thermal nested shells at 1:50; structure tubes along both bodies.
Imperfect, stated plainly: the plan draws the far toddler boot (and in all four scales the far shank) about 70 mm further forward than the rig's far shank reaches (the far shank bone hangs 20 degrees more vertically than the near one in the pose, the plan draws both legs parallel), so a red patch at the far boot stays in every plan overlay; it is a pose-versus-drawing inconsistency (needs the pose or the 2D plan edited), not a model defect, and no limit fails because of it. Hands of the toddler at 1:500 are thermal-only blobs as before.

## humans/tour-guide (spliced)

Counts: 1:50 30/54/33, 1:100 23/27/31, 1:200 21/25/30, 1:500 20/25/26.
Product check now: every rule green at all four scales except the exposed-area lines at 1:50 and 1:100 (`head-skin` 0.67/0.61, `arm-skin` 2.2, `shoes` 0.52/0.60).

2D repairs at the root (`catalog/👤️humans/tour-guide/🔣️.json`, text-level through `patch_section`; backups `🗑️generated/author-63/tourguide-asset.before-2d-fix.json` and `...-fix2.json`):
- 1:50 `volumes-views-disagree` z (front 0.500 m, plan 0.486 m, 14.3 mm against 10): the front sleeves scaled to 0.95 of their half-width and the plan right upper arm to 1.05; now 0.495 against 0.488 m.
- 1:500 elevation `figure` (the cause of the elevation IoU 0.83-0.88 that no 3D body could cure): the shape was ONE path of four overlapping subpaths (torso, pelvis and legs, head, hair/cap) and the silhouette fills several rings even-odd, so every overlap was a hole: 148 cm2 in the head and 130 cm2 at the waist. Split into the shapes `figure`, `figure-part-2..4`; the same for the elevation `forearm-l/-r` and the front `sil-forearm-l/-r` (two subpaths each). The second subpath of the front forearms was a stray hairline of another view's coordinates (z 0.25..0.45 m on the wrong side), a 0.198 m width contradiction as soon as it counted: removed. Written up in `tool-requests.md` (section 63) because `leaning-person`, `person-with-umbrella` and `jogger` carry the same overlapping subpaths at 1:500.
By hand in the 3D:
- 1:50: the cap loft (crown pinched to 25 mm lateral width, 20 mm below the drawn cap) rebuilt from the cap outline with the hull tool and a crown that tapers to a 60 mm button; `p95SoftM:elevation` 18.8 -> 5.6 mm (limit 15), elevation IoU 0.927 -> 0.931.
- 1:200: pointer 10 mm true (it was 16), then coordinate descent on the 15 lofts: front 0.881 -> 0.902, plan 0.886 -> 0.904, elevation 0.930.
- 1:500: modelled anew with `author-63-tour500.py`: one hull loft per bone from the 1:500 elevation, front and plan (head, torso and pelvis from the torso-bone `figure` outline cut by station ranges, hands with `range 0,0.18`, 2 to 6 sections each), the head's first section set to a 100 mm neck, pointer and flag pole at true size as `slender` (10 mm and 24 mm; they were 75 and 64 mm), then opt3. Silhouette 0.827 / 0.873 / 0.865 -> 0.920 / 0.917 / 0.893 (p95 19 / 38 / 19 mm); cross-scale green (the p95 plan 0.206 of the earlier run is gone).
- `areaM2` untouched.
Seen (all four sheets after the last change): overlays green with thin red/blue rims; flag, raised pointer arm, held pole, cap and hair read right; at 1:500 the flag pole and pointer show as thin slender lines in the axonometric view and as red outlines of their drawn pen width in the overlay (the sheet renderer does not widen slender strokes yet, the check numbers do: C24).
Imperfect: plan 1:500 margin 0.013; the 1:50 thermal `arm-skin` (declared 0.004 m2, the real wrist-band ellipsoid has 0.013 m2 of surface, the exposed part 0.004) and `shoes` findings; the 1:200 and 1:100 cap is merged into the hair as before.

## humans/worker-carrying-box (spliced)

Counts: 1:50 27/64/36, 1:100 20/24/34, 1:200 20/24/33, 1:500 19/24/29.
Product check now: every rule green at all scales except the exposed-area lines (helmet, torso clothing, vest, shoes at 1:50 and 1:100, gloves at 1:100, area totals at 1:200 and 1:500; the reviewer lists "worker helmet +100 %" as a model finding: the drawn helmet is a 310 x 200 mm hard hat with a visor, 2.4 times the declared 0.067 m2, and the vest and the tee are layers of one torso solid).

2D repairs at the root (`catalog/👤️humans/worker-carrying-box/🔣️.json`, text-level through `patch_section`, backup `🗑️generated/author-63/worker-asset.before-2d-fix.json`):
- `volumes-views-disagree` y at 1:100 and 1:200 (38.3 mm against 36.6): the 1:100/1:200 elevation draws the hard-hat dome 1.828 m high, the front view 1.789 m. The front helmet outline is redrawn as the same dome (crown 1.828 m, 200 mm wide at the base, 146 mm at 0.9 of its height) in both scales.
- `volumes-views-disagree` x at 1:100 (19.9 against 15 mm) and 1:200 (32.9 against 30): (a) the elevation vest outline overshot 19 mm behind the plan trunk because of a Bezier control point at 0.1778 (now 0.1478); (b) the 1:100 vest band and the 1:200 vest bands/box tape ended outside the outline, and their pen (a 26 mm stroke widens the extent by half the pen) added 13 mm; they now end inside it (box tape 0.2 -> 0.187 m, vest bands 0.15 -> 0.135 m). Elevation/plan length now 0.6876 / 0.6790 m at 1:200 and 0.6801 / 0.6715 m at 1:100.
By hand in the 3D: the hard hat at 1:100 and 1:200 is a real dome loft cut from the scale's own helmet outline with `author-33-hull.ts` (it had a 16 mm lateral pinch below the crown, which showed as red above the helmet in the front overlay); the heel cap of the boots was 180 mm wide against the drawn 105 mm boot (blue discs beside the feet in the front view at 1:50, 1:100, 1:200): 1:50 front IoU 0.939 -> 0.977 and its p95 front 35.6 mm (new failing line under swap 14) -> 7.5 mm; 1:100 front 0.917 -> 0.967.
Seen (all four sheets after the last change): overlays green; the worker holds the box at the belly, hard hat with a visor, vest, boots; at 1:500 the broad-limbed merged model with the hat as one loft.
Imperfect: thermal findings above (helmet drawn too big; vest/tee layers on one solid); the 1:100 plan keeps a 30 mm blue rim at the back of the trunk (the torso loft is 0.14 deep against the drawn 0.126 trunk radius); 1:500 elevation 0.912 with feet 40 mm too long.

## Requests and findings filed
- `tool-requests.md` section 63: overlapping subpaths of one 2D path shape are holes for the silhouette (list of the other 1:500 humans that carry them).
- No `author-requests.md` entry.

## Not done / open
- Exposed-area lines (see each asset): none was steered by scaling or by dropping coverage; the C28 contact fraction and the C29 outer-area measurement were not both live while I worked.
- The far toddler boot in the toddler's plan (pose versus drawing), the crutch tips at 1:50/1:100, the worker helmet drawn 2.4 times the declared area.
