# Author 33 report (round 2): humans/jogger, humans/elderly-with-cane, humans/kneeling-gardener

Status: jogger and elderly-with-cane are finished, green at every authored scale under `volumes-check.ts <asset> --scale N` (the product validator, C1-C19) and spliced. kneeling-gardener is finished except one C18 finding (hat area, below); it is NOT spliced because its 1:50 and 1:100 check lines are red on that one measurement. Source files: `T/volumes/humans/<slug>.json`.

Scripts (all in T, inputs): `author_33_lib.py` (load/save, hull, shell, person structure, coarsen/thermal/check wrappers), `author-33-hull.ts` (hull loft of an OBLIQUE bone from the scale's own drawings, built for 1:500), `author-33-joints.ts` (fits joint/mass/member solids inside the body silhouette), `author-33-exposed.ts` (exposed area per thermal solid), `author-33-frames.ts`, `author-33-ext.ts`, per asset `author-33-jogger50.py`, `author-33-j100.py`, `author-33-j500.py`, `author-33-eld50.py`, `author-33-eld100.py`, `author-33-eld200.py`, `author-33-eld500.py`, `author-33-gar50.py`, `author-33-gar100.py`, and `author-33-*-decisions.py`. Rebuild of an asset = its scripts in that order.

## humans/jogger (spliced)

Counts body / thermal / structure: 1:50 42 / 68 / 37, 1:100 25 / 28 / 37, 1:200 21 / 28 / 21, 1:500 17 / 22 / 20. Decisions: 3, 2, 2, 2 (en + de).
Check line: PASS at 1:50, 1:100, 1:200, 1:500 (all 11 rules per scale, `[p]` product engine).

By hand:
- 1:50: kept the kit fit, then torso exponent 3.0 and 4 % fuller (loose shirt), thigh skin tucked 10 % inside the shorts, a new 20 cm shirt-hem loft on the pelvis bone (it buries the waistband and the pelvis top), shoulder caps rebound to the shirt region (kit-thermal `--map`), structure replaced: tube members joint to joint without pointed ends, ball joints centred on the child loft axis (the kit put them at w = 0, so they floated outside the legs), mass ellipsoids on the loft path. Thermal at 1:50 within the C18 30 % band: shirt exposes 0.59 m2 of 0.783 (-25 %), shorts 0.235 m2 of 0.182 (+29 %).
- 1:100: coarsen of the finished 1:50 with absorb 0.995, five details dropped by hand (right sleeve, stripe, headband, shoulder caps) so that left and right stay symmetric, hem extended to the crotch line, thermal and structure rebuilt; leg-clothing +28 %.
- 1:200: coarsen of 1:100 (absorb 0.995), socks dropped, thermal count kept under 1:100, masses as boxes.
- 1:500: modelled anew with `author-33-hull.ts` (one loft per bone cut from the 1:500 elevation outline and the plan or front width, then `kit-fit --sections`), pelvis sagittal depth set by hand (the drawing has no pelvis elevation), 17 solids; elevation IoU 0.902 (was 0.79-0.84), front 0.912, plan 0.910; structure with mass boxes, pelvis mass merged into the trunk mass so the structure list strictly drops (21 -> 20).
Seen in the sheets (all four, after the last change): overlays green with thin rims, axonometric body reads as a jogger in the initial-contact pose (loose shirt, shorts, shoe toe caps), thermal shells and regions complete, structure a load path inside the body. Imperfect: 1:500 limbs are cones with two to five sections and the right-arm swing is stiff; at 1:50 the hands are mitten-like with a capsule thumb; the plume disc (a declared 0.35 m) is the only thermal solid outside the figure.
Not changed: no areaM2 of any region.

## humans/elderly-with-cane (spliced)

Counts: 1:50 55 / 94 / 38, 1:100 28 / 33 / 38, 1:200 27 / 32 / 21, 1:500 19 / 25 / 20. Decisions 3, 2, 2, 2.
Check line: PASS at all four scales.

By hand:
- 1:50: the 07 seed body (55 solids, good) kept. Repairs: the glasses (6 mm lenses and wire are below the 7.5 mm feature): lenses `texture: true`, temples now run to the ears and are `slender`, one slender front bar replaces the bridge; jacket front lines `slender`. Thermal rebuilt: layered shells only for the large masses (kit shells of buttons, pockets, brim, glasses were 1-4 mm slivers and were pruned), loft shells rebuilt with the real cumulative thickness (`prune_and_rebuild_shells`: the kit reduced the inset because of tapered caps); the plume disc moved 15 mm down so it stays in the body box; structure from `person_structure` (neck and cane are members).
- 2D repair at the root (`catalog/👤️humans/elderly-with-cane/🔣️.json`): the cane in the 1:100, 1:200 and 1:500 elevation was an OPEN stroke (`filled: false`) so the whole-asset length of elevation and plan disagreed by 0.067 m (`volumes-views-disagree` extentDifferenceM:x). It is now a filled capsule from 0 to 0.86 m, r 0.0125 / 0.018 / 0.0325 (half the pen width of each scale), like at 1:50. Backup before the edit: `T/🗑️generated/author-33/elderly-asset.before-2d-fix.json`.
- 1:100: coarsen of 1:50 with details dropped; cane rebuilt (shaft r 0.01, three crook capsules, ferrule), cap brim re-sized (22 x 90 x 130 mm) and bound to the cap region (the kit had bound it to the face, which hid cap area).
- 1:200: coarsen of 1:100; cane shaft/ferrule thickened to the 33 mm feature (the 1:200 draws 36 mm), two crook capsules of 30 mm.
- 1:500: modelled anew with the hull tool from the 1:500 drawings (19 solids; the right arm from the front view because the elevation only strokes it), cane 75 mm at the plan's lateral offset.
Seen (all four sheets): a stooped man with flat cap, jacket hump, glasses, trousers, cane in the right hand; nested shells visible in the thermal cutaway, tube members and joints inside the body, cane member separate. Imperfect: at 1:500 the neck is a hand-set 100 mm tube mostly inside torso and head; 1:200 is only one solid fewer than 1:100 (the 1:100 model was already near the 1:200 outline); plume disc 15 mm lower than its anchor.

## humans/kneeling-gardener (NOT spliced: one C18 finding)

Counts (200 and 500 stay `unavailable`): 1:50 43 / 60 / 42, 1:100 27 / 28 / 41. Decisions 3, 2.
Check lines: 1:50 and 1:100 fail only `areaDeviation:hat`: 0.994 at 1:50 and 0.946 at 1:100 (limit 0.3). Everything else ok, incl. silhouette (IoU 0.957/0.976/0.969 at 1:50), bounds, views-disagree, feature, structure, bones, copied, cross-scale.
Finding: `hat` declares 0.226 m2 (C18, never changed). The drawn and modelled straw brim is a 0.47 m disc (both faces 0.35 m2) plus crown and band: 0.45 m2 exposed. No honest solid exposes less. Written to `author-requests.md` and `area-review.md` with the proposal (areaM2 about 0.45). When the coordinator rules, the asset splices with one command.

By hand:
- 2D repairs at the root (`catalog/👤️humans/kneeling-gardener/🔣️.json`): the trug was 0.255 m radius in front and elevation but 0.255 x 0.25 in plan, a 0.019 m whole-asset width contradiction between front and plan (`volumes-views-disagree` extentDifferenceM:z 0.0192 > 0.0104); all trug radii are now 0.25 m in elevation, front and plan at 1:50 and 1:100 (also fits `dimensions.depthM` 0.5). The first attempt with 0.255 everywhere broke `volumes-bounds` (depth 0.51 against 0.5). Backup: `T/🗑️generated/author-33/gardener-asset.before-2d-fix.json`. The 3D trug profile was refitted to the 2D (base radius 0.215, rim 0.25).
- areaM2 corrections within factor 1.5 (listed in `area-review.md`): pad 0.28 -> 0.395 (the model exposes the whole foam top and its edges), trowel 0.025 -> 0.0375.
- 1:50: the 07 seed (40 solids) kept; bones toe-l and toe-r had no solid (the clog loft covered the toes): clog lofts cut at the ball of the foot and clog toe lofts added from the drawings; mouth line r 3 -> 4 mm (feature); shirt 6 %/5 % fuller with exponent 3 and a shirt tail loft over the pelvis (shirt now -27 %); thermal bound by hand (kit had bound the toes to pad and trug, the left glove to the trousers); weeds and clog straps left out of the thermal list.
- 1:100: coarsen of 1:50 (absorb 0.99), thermal regions bound by hand (glove-l, sleeves, forearms, hat), structure rebuilt.
Seen (both sheets): kneeling figure with straw hat, leaning trunk, weeding arm with trowel into the trug, foam pad under the shins; nested shells in the thermal cutaway; structure tubes along legs, trunk, arms, plus pad, trug and trowel members. Imperfect: the hat brim is a 4.7 cm thick puck (the seed's revolve profile), not a thin straw brim; the trug carries a polyethylene-material weed dome (material of the seed); structure joints are plain balls.

## Requests and findings filed
- `author-requests.md`: hat area (gardener), emitter solids and `outsideBodyBoxM`.
- `area-review.md`: pad, trowel, hat.
- `tool-requests.md`: render ignores `--source`; emitter solids count in exposed area only at scales without layers (shirt 0.59 m2 at 1:50, 0.755 m2 at 1:100); empty thermal/structure lists skip the silhouette; kit-fit raster coarser than the product (0.912 fitted, 0.882 measured).
