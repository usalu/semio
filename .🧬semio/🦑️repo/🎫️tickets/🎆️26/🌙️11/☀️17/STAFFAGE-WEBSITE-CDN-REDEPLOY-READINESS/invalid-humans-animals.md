# Invalid Humans and Animals Repaired

All eight owned assets now report zero issues from `validateAsset` (checked with `validate-some.ts`, the validator unchanged). The declared dimensions were kept in every case: each is backed by a real-world figure and the 1:50 or 1:100 drawing already matched it, so the drawing of the failing scale was the wrong party. Only authored data of the failing scale or view changed. Edits went through `humans-animals-edit.py`, which rewrites only the changed top-level members and keeps the hand formatting (diffs are a few lines each); the per-asset scripts `fix-*.py` hold the exact edits, `extent-probe.ts` names the shape that sets each extreme.

| Asset | Finding | Cause and decision | Fix |
|---|---|---|---|
| animals/hedgehog | front height 0.146 vs 0.128 m at 1:100 | The merged limb rects of the 1:100 front view were longer than their 1:50 parts and sank 13 mm below the ground; 0.128 m (Reeve 1994, dome height) is right. | Near and far hind limb bottoms -0.092 to -0.080, fore limb bottoms -0.082 to -0.074. Front extent now 0.141 m. |
| animals/horse | tail-upper (tail-2) and tail-lower (tail-4) flat shapes leave their plane by 31 and 50 degrees at 1:500 | The tail swings sideways in poses and clips; the other scales and the front and plan twins already use solids. | Elevation shapes of the 1:500 tail now carry `extruded` `sweep: w` solids (+-0.075 and +-0.0675 m, the values of the 1:500 front twins). |
| animals/pig | length 1.196 (1:500) and 1.204 (1:200) vs 1.23 m | The merged 1:200 and 1:500 hindbody lists the region `tail` but its outline stopped at the rump, while 1:100 reaches 1.243 m with the tail. Declared 1.23 m kept. | Rear outline of the hindbody pulled back by 0.035 m (elevation) and about 0.03 m (plan) at 1:200 and 1:500; lengths about 1.24 m. |
| humans/elderly-with-cane | length 0.566 (1:500), 0.557 (1:200) vs 0.54 m | The cane is drawn thicker at coarse scales for print legibility and grew forward past the true shaft edge (1:50 gives 0.541 m). | Cane capsule centre line offset rearward by the legibility growth (y 0.0075 at 1:200, 0.022 at 1:500), so the front edge stays at the true shaft face. |
| humans/parent-with-stroller | lateral 0.635 vs 0.6 m at 1:500 | Legibility-widened tyres (0.075 m) grew outward to 0.3175 m; track 0.6 m is right (1:50 to 1:200 give 0.600). | Tyre solids at 1:500 keep the outer faces at 0.3 m (rear) and 0.22 m (front) and widen inward. |
| humans/person-with-rollator | lateral 0.657 vs 0.62 m at 1:500 | Grips, handlebar and tyres widened around their true centres past the 0.31 m half width of ISO 11199-2 style rollators (1:100 gives 0.620). | Outer faces held at 0.31 m: tyres 0.233..0.31, front grips shifted inward, plan grips at 0.2715 and handlebar end at 0.2775. |
| humans/tour-guide | forearm-r-part-2 flat shape leaves plane by 51.8 degrees at 1:500 | The waving hand part of the forearm is a second elevation path without a solid, unlike its first part. | `extruded` `sweep: w` solid -0.195..-0.185, the lateral position of the forearm part. |
| humans/wheelchair-user | height 1.481 vs 1.45 m at 1:500 | The legibility-thickened rear wheel ring grew outward to radius 0.3305 m and sank 26 mm under the ground; the 24 inch wheel (0.61 m, Cooper 2014) is right. | Ring outer radius back to 0.305 m, inner 0.24 m, thickness kept at 0.065 m. |

## Changed files

All under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`: `🐕️animals/{hedgehog,horse,pig}/🔣️.json`, `👤️humans/{elderly-with-cane,parent-with-stroller,person-with-rollator,tour-guide,wheelchair-user}/🔣️.json`.

Ticket scripts: `humans-animals-edit.py`, `extent-probe.ts`, `fix-hedgehog.py`, `fix-pig.py` (second step only; the first rump pull-back was parameterised and is described above), `fix-horse.py`, `fix-elderly.py`, `fix-stroller.py`, `fix-rollator.py`, `fix-tour-guide.py`, `fix-wheelchair.py`.
