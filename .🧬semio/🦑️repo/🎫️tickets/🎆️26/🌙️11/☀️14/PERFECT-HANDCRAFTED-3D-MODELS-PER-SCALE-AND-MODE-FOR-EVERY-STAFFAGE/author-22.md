# Author 22 report

Assets in order: humans/cyclist (done except 1:500 silhouette and one C16 area item, not spliced), humans/child-on-kick-scooter (not started).

## humans/cyclist
Source `volumes/humans/cyclist.json`; builder `author-22-cyclist.py` (all four scales, one command: `python3 author-22-cyclist.py [50 100 200 500]`), helpers `author_22_lib.py` (product-measure thermal loop, root `areaM2` edit), `author_22_bike.py` (bicycle per scale), `author-22-blame.ts` (per-solid IoU blame), on top of the author 07 kit.

Solid counts (body / thermal / structure): 1:50 83 / 169 / 56, 1:100 53 / 56 / 53, 1:200 48 / 51 / 49, 1:500 42 / 45 / 43. Bilingual decisions at every scale.

Check result (`volumes-check.ts humans/cyclist --scale N --json`, owned engine, rulings C9-C16):
- 1:50 passes every rule except `areaDeviation:saddle` (0.033 exposed vs 0.07, see C16 below).
- 1:100 same: only the saddle area.
- 1:200 passes every rule.
- 1:500 fails only `volumes-silhouette`: IoU elevation 0.886 (0.90), front 0.855 (0.88), plan 0.830 (0.88); every other rule passes (thermal by C15 sum rule).

Modelled by hand
- 1:50: author 07 body repaired: thumbs removed (the plan draws none; they were 30 mm outside it), thermal rebuilt as exposed-surface model (outer layer = body solid, nested inner shells of at least 7.6 mm, breath nose/mouth and plume as tagged emitter solids), structure: bone members, joints, masses, every frame tube/fork/bar/rim/crank.
- 1:100/200/500: humans from each scale's own drafts (extrudes to elliptic lofts), lateral fit by `author-07-snap.ts` with a cap that lets the torso reach the drawn shoulder width (it had stayed 14 mm wide: the cap `rx*grow+0.02` was too small, `grow=60`); hair excluded from the snap. Bicycle hand-built per scale: tube radii from the scale's pens (100: 17/16/18/22 mm tubes, fenders cut from the drawn 2D arcs; 200: 18 mm tubes, hubs, 30.6 mm pedals, no fenders; 500: frame tubes 65 mm, bars/grips 100 mm as the plan draws them, tyres 77 mm, no hubs/cranks/fenders). Wheels are tyre rings of 36/24/20/16 segments with radial width from the 2D annulus (100: 40 mm, 200/500: pen width). Merged: 200 forearm+hand, 500 head-with-helmet and hair tail (helmet as upper-head shell), arms from the 1:100 lofts floored to 77 mm, cranks as structure-only members at 1:500.
- Spokes not modelled (2D texture, C10; they would need `texture: true` solids, left out).

What I saw in the sheets (50, 100, 200 read; 500 read earlier at an intermediate state)
- 1:50: overlays green throughout; thin red/blue only at glove tips and helmet rim. Axonometric: rider, bike, nested clothing shells and member skeleton read correctly; thermal cutaway shows the layered jacket and trousers and the breath emitter at the face.
- 1:100/1:200: the human matches the drawings; the red square blocks in the elevation overlays (saddle/headlamp at 1:100, frame triangles at 1:200) are the render's defect, not the model (tool-requests.md): the check treats those closed unfilled paths as pen strokes and passes.
- 1:500: faceted 16-gon tyres and blocky limbs are intended (75 mm feature); front/plan IoU lost to the 77 mm floor on neck, shoulders and arms.

Root `areaM2` changes in the asset (C15/C16, only from the 1:50 model, all within factor 1.5): trousers 0.64 to 0.757, shoes 0.125 to 0.148, bike-metal 0.9 to 1.05, tyres 0.6 to 0.66, gloves 0.098 to 0.0734, helmet 0.07 to 0.08. face-skin 0.098, hair 0.027 (C16 region), jacket 0.921, saddle 0.07 unchanged. Saddle: measured/declared 0.48 is outside 1.5, therefore not applied and listed in `area-review.md`; until decided `areaDeviation:saddle` fails at 1:50 and 1:100.

Left imperfect, stated plainly
- 1:500 silhouette below the C9 IoU limits (see numbers above); the body is fatter than the 65 mm drawn strokes because tyres/limbs are held at 77 mm.
- Saddle area (above). Not spliced for these two reasons; run `python3 volumes-insert.py volumes/humans/cyclist.json <asset>` once the coordinator rules on the saddle and accepts or relaxes the 1:500 front/plan limits.
- 1:500 face-skin and helmet exposed areas are about twice `areaM2` (feature floor on the head); the sum rule of C15 passes.
- 2D defect worth fixing at the root: front torso 380 mm vs plan 250 mm (shoulder width), reported by `volumes-views-disagree` as information; I followed the front drawing.

## humans/child-on-kick-scooter
Not started (budget). The kit works unchanged: copy `author-22-cyclist.py`, replace `author_22_bike.py` by a scooter builder and the region/emitter lists; the lessons above (snap cap, thumbs, exposed-surface thermal, areaM2 only from 1:50, thermal refs to body solids) carry over.
