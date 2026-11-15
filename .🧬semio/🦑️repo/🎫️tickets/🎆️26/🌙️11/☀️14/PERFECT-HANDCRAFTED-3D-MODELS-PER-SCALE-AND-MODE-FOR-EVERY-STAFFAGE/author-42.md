# Author 42 — objects (stepladder, storage-crate-stack, street-lamp) and the bollard re-check

Generators (all rebuild the scale lists of `volumes/objects/<slug>.json` from scratch, then `volumes-insert.py`): `author-42-lib.py` (oriented bar / round rod / extrusion helpers on the author-20 lib), `author-42-stepladder.py`, `author-42-crate.py`, `author-42-lamp.py`; `author-42-2d.py` (2D root edits, text level); probes `author-42-probe*.ts` (product silhouette internals). Checks are the product engine (`volumes-check.ts <asset> --product`). No tool output was written under `🗑️generated/author-42`.

## stepladder — spliced (all four scales), rebuilt on its own files, nothing of the DRAFT left

| scale | body / thermal / structure |
|---|---|
| 1:50 | 24 / 24 / 26 |
| 1:100 | 21 / 21 / 24 |
| 1:200 | 15 / 15 / 18 |
| 1:500 | 13 / 15 / 16 |

Final check: silhouette, views-disagree, bounds, feature, monotonic, cross-scale, structure, bones, copied pass at every scale (elevation/front/plan IoU 0.992/0.975/0.961 at 1:50, 0.957/0.975/0.970 at 1:100, 0.922/0.954/0.970 at 1:200, 0.927/0.884/0.890 at 1:500). Failing lines, all `volumes-thermal-coverage`: 1:50 `areaDeviation:feet` 0.34 (limit 0.15), 1:100 feet 0.56 (limit 0.30), 1:500 `areaDeviationTotal` 0.84 (limit 0.30).

What I built by hand (the old seed solids are not used): rails as 60 x 45 mm (front) and 50 x 45 mm (rear) chamfered bars on the drawn centre lines in all three views (the front pair leans in z towards the top, the rear pair leans back to the hinge; one oriented extrusion each), treads with the drawn nosing, platform, domed top cap, spreaders as two 24 mm round rods per side (front and rear link) with pivot knuckle, hinge rivets as two discs, two round braces between the rear rails, four tapered rubber feet as lofts. Structure at 1:50/1:100: real hollow tubes inside the skin (rails 1.5 mm and 1.2 mm wall, 1.5 mm inside the body), hollow treads and platform, hollow braces, spreader rods, hinge and knuckle pins as joints, masses on material, foot pads as supports. Coarser scales are modelled against their own drawings (1:100 sharp bars and 30 mm spreaders without cap; 1:200 60/100 mm bars, 36 and 60 mm slabs, one 40 mm spreader, block feet; 1:500 the pen-floor widths 134 x 85 and 160 x 85 mm, 78.5 mm slabs, block feet, two thermal-only spreader rods because the region needs a solid).

Roots fixed in the asset (`🪑️objects/stepladder/🔣️.json`):
- plan `rear-brace-1` at 1:50 and 1:100: x 0.105 -> -0.025 (it was drawn in bone-local x while its siblings and the elevation use world x: elevation and front put the brace on the rear rail at y 0.85, x -0.025).
- 1:500 pen strokes shortened by the half pen (32.5 mm) at each end (elevation treads 0-3, platform, both feet; front treads 0-3, platform, front feet): the validator widens an open stroke by half a width at both ends, so the strokes made `views-disagree` fail (x 80 mm, z 90 mm) and cost 4 % elevation IoU; now x 15 mm, z 25 mm, elevation IoU 0.927 (script `author-42-2d.py`, the diff collapses the shortened points arrays to one line each).
- `areaM2` (old -> new, all within 1.5): treads 0.5 -> 0.39, platform 0.3 -> 0.21, rear-frame 0.43 -> 0.506, spreaders 0.06 -> 0.074, feet 0.02 -> 0.03 (the C16 limit). Rows in `area-review.md`.

What I saw: opened the sheets of all four scales after the last change. 1:50 reads as a ladder in the third dimension (rails, nosed treads, platform with cap on top, rods across the legs, feet), overlays green in the three views (red crescents where the drawn plan rail ends are round and the 3D ends are square cuts); thermal is the cutaway (half of the objects), structure members nest inside; 1:100 the same without cap and nosing; 1:200 blunt bars with the spreader (blue sliver in the elevation, which omits it at 1:200); 1:500 three pen-floor bars with slabs, thick as drawn.

Imperfect, stated plainly:
- Feet: the declared 0.02 m2 is the 4 x 0.08 x 0.06 m footprint of the pads (a contact face, not counted); the modelled caps expose 0.040 m2 (1:50), 0.047 m2 (1:100). With the C16 limit 0.03 the deviation stays +34 % and +56 %. Proposal in `area-review.md` (0.04 or document the footprint).
- 1:500 thermal total +84 %: the pen-floor rails expose 2.3 times the real surface (request `halo` of author 19 in `tool-requests.md`, my evidence appended). The 1:500 front IoU is 0.884 (limit 0.88) and elevation/plan are limited by the drawings giving the rails 134/160 (elevation), 80 (front) and 100 mm (plan) widths.
- The 1:200 elevation omits the spreader that front and plan draw; modelled, cost a blue sliver (IoU 0.922).
- The top cap is drawn 0.20 m wide, 10-13 mm narrower than the gap between the rails: modelled as drawn, so it does not touch the rails (it is not a structural member). The mass of the front frame is at the tread (16 mm in front of the declared centre, which lies in air); the rear mass is split on the two rails at the declared height.

## storage-crate-stack — spliced (all four scales), full PASS

| scale | body / thermal / structure |
|---|---|
| 1:50 | 50 / 15 / 30 |
| 1:100 | 15 / 10 / 25 |
| 1:200 | 10 / 5 / 11 |
| 1:500 | 5 / 5 / 11 |

Final check line: `PASS (5.7 s)`, every rule, all four scales (IoU elevation/front/plan 0.992/0.989/0.999, 0.993/0.986/0.999, 0.989/0.976/0.959, 1/1/1).

By hand: each of the five crates on its own bone as a 20 mm foot ring (8 mm wall), 12 mm floor plate, the 4 mm hollow wall tray tapering from 0.56 x 0.34 m to the drawn rim, and the 20 mm rim ring whose inner opening is a stacking ledge. The old seed had a rim opening wider than the foot ring, so no crate rested on another: the ledge (48 x 58 mm wide) carries the foot ring of the next crate (the drawn sizes do not otherwise touch each other). Handle slots and floor ribs are `texture` decals (2-6 mm, below every feature; real openings would need walls thinner than the feature). Thermal: the declared 0.6 m2 per crate is the outer wall area, so each region is the outer envelope (wall block, rim and foot edges) with the top and bottom faces as `sectionFaces`; it exposes 0.56 m2 (-6.6 %, `areaM2` kept). Structure: foot ring, 2 mm wall tray inside the 4 mm skin, rim ledge and floor as members, the stack ring joint where a foot ring meets the ledge, the foot ring of the lowest crate as the `base-contact` support, one mass lump of 1.9 kg per crate at the declared centre. Coarser scales: 1:100 floor + 6 mm tray + rim ring (no foot), 1:200 one 15 mm tray per crate with a 30 mm floor, 1:500 the five drawn blocks.
What I saw: all four sheets opened after the last change: stack reads as five crates with rim ledges; overlays green; at 1:200 the plan shows blue 9 mm at the short ends and red 6 mm at the long sides (the single taper cannot give both drawn ratios).
Imperfect: the 12 mm floor plate and the texture decals are not honest to the real 3-4 mm floor and the real openings; the 1:200 taper is uniform 1.14.

## street-lamp — spliced (all four scales)

| scale | body / thermal / structure |
|---|---|
| 1:50 | 24 / 18 / 14 |
| 1:100 | 8 / 9 / 13 |
| 1:200 | 6 / 8 / 10 |
| 1:500 | 4 / 6 / 9 |

Final check: every rule passes at every scale except `areaDeviation:plinth-concrete` at 1:50 (0.76, limit 0.15) and 1:100 (0.71, limit 0.30). Silhouette IoU elevation/front/plan 0.986/0.934/0.931 (1:50), 0.987/0.936/0.942, 0.980/0.953/0.933, 1.00/1.00/0.997.

By hand: concrete plinth, base plate with four nuts, flared hollow cover shell, the 5.88 m galvanised pole as a hollow revolve of the drawn taper (13 points at 1:50, 7 at 1:100 on the drawn Bezier, 3 at 1:200, a cone at 1:500; the seed had a SOLID pole) with domed cap, collar ring and the access hatch as a texture decal, the arm as a hollow tapered tube with a ring clamp (a hollow ring, not a block, so the pole stays hollow), a hollow 24 mm brace, the housing loft (6/5/4 sections) with its spigot and the cooling fins as texture decals, the polycarbonate lens as a thin glazing dome. Thermal: the pole and base cover are closed outer envelopes with their end faces as `sectionFaces` (the declared 2.4 m2 is the outer tube surface, exposes 2.56 m2), the lamp-plume emitter is a thin tagged disc (all faces section faces, 120 mm instead of the declared 300 mm because a 300 mm disc would leave the head box by 0.15 m). Structure: hollow pole and arm tubes inside the skin, hollow brace, plinth and head core blocks, joints (weld ring, clamp ring, brace pin, spigot), masses at the declared centres of the four bones, fixed support under the plinth.
Root fixes: `areaM2` arm-alu 0.18 -> 0.22, lamp-housing 0.25 -> 0.19, lamp-lens 0.08 -> 0.05 (all within 1.5, they are what the hollow arm, the loft and the dome expose). plinth-concrete 0.5 stays: the drawn 0.30 x 0.30 x 0.10 m plinth exposes 0.12 m2 (ratio 0.24, beyond 1.5): row in `area-review.md`.
What I saw: all four sheets opened after the last change; the 6 m lamp is small in the default sheet, the hollow pole and the arm/head read in the axonometric panels; plan overlay shows blue slivers at the loft edge (housing wider than the drawn path by up to 15 mm at 1:50) and at 1:200 a polygonal head (red corners, IoU 0.933).
Imperfect: plinth area as above; the emitter disc is smaller than the declared plume; the housing loft is solid (3 mm skin is the thermal layer); the tube end faces are `sectionFaces` so the pole ends add no area.

## retractable-bollard — re-check: the measurement is at fault, the model is not

Evidence (full numbers in `tool-requests.md`, last section): the 1:50 elevation of the section view reads p95 24.4 mm / Hausdorff 208 mm with the product engine only because cell = feature/4 makes the raster 850 cells across (above TILE_CELLS 512), so `measureBanded` runs. The same product `measureView` on the unchanged model gives p95 6.3 / 4.5 / 3.2 / 4.8 mm for 300 / 420 / 600 / 800 cells and 60.8 / 43.4 mm at 1000 / 1400 cells (Hausdorff up to 209 mm); the non-section stepladder improves with the raster as it should; the owned engine gives 0 mm / 13 mm; the overlay and `author-42-probe3.ts` show no boundary of the drawing farther than 20 mm from the 3D boundary. Nothing in the asset or the model was changed (a variant run that dropped cover-frame, sleeve, hydraulic parts or any drawn shape left p95 at 24.4 mm, so no single part causes it). Request appended to `tool-requests.md`.

## Files outside the volumes sources

- `🪑️objects/stepladder/🔣️.json`: plan rear-brace-1 x (1:50, 1:100), 1:500 stroke shortening (14 shapes), `areaM2` of five regions, spliced `volumes`.
- `🪑️objects/storage-crate-stack/🔣️.json`, `🪑️objects/street-lamp/🔣️.json`: spliced `volumes`; street-lamp `areaM2` of three regions.
- `area-review.md` (stepladder feet, street-lamp plinth, applied corrections), `tool-requests.md` (stepladder 1:500 halo / stroke extension, bollard banded section).
