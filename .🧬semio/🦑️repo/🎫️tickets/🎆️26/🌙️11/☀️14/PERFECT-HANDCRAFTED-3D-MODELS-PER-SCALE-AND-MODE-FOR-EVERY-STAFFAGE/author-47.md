# Author 47 — round 2: vegetation (miscanthus-grass, weeping-willow, small-leaved-lime)

Kit written for this: `author-47-lib.ts` (visual-hull pieces: `pieceHull` = a loft along one world axis whose section centres and half widths are read from two views, explicit section rotations so it sits in any planar bone frame; `ribbon` mode = flat sheet sections, width horizontal and perpendicular to the path, thickness solved from the drawn elevation; `custom` spans; `simplifyRing`, `clipHalf`, `inUnion`), `author-47-grass.ts` (miscanthus bodies per scale), `author-47-struct-fix.ts` (cores along each bone's own body piece, joints at the child's start, horizontal support plate), `author-47-thermal-patch.ts`, `author-47-grass-decisions.py`, 2D repair scripts `author-47-fix2d-grass-*.py` (text-level, via `author-47-lib2d.py`). Checks: `🗑️generated/author-47/miscanthus.check.txt` (full `--product` run).

## 1. vegetation/miscanthus-grass — NOT spliced (1:50 and 1:500 body green; 1:100 and 1:200 silhouette and bounds, cross-scale and 1:500 thermal total open)

Counts body / thermal / structure: 1:50 73/74/50, 1:100 49/50/44, 1:200 47/48/26, 1:500 3/5/26 (the previous draft had 238/250/62 at 1:50, IoU 0.51/0.60/0.47).

### Final check line (`volumes-check.ts vegetation/miscanthus-grass --product`, 15 s)
`FAIL: 17 measurement(s) over 7 rule line(s)`: silhouette 1:100 and 1:200, bounds 1:100 and 1:200, cross-scale 1:100 and 1:500, thermal-coverage 1:500. **Every rule of 1:50 passes** (silhouette IoU elevation/front/plan 0.920/0.945/0.969, p95 5.6/5.6/24 mm; views-disagree, bounds, feature, thermal, structure, bones, monotonic, copied). 1:500 passes silhouette (0.943/0.901/0.930), bounds, feature, structure, bones, monotonic.

### What I modelled by hand
- **1:50.** Each of the 60 blades is a flat, sheet-like loft along its own path: stations from that blade's elevation and plan outline, section width horizontal and perpendicular to the path (plan width corrected for the diagonal), thickness solved from the elevation interval (x1.4 calibrated); steep blades (the l3/r3 `a` segments) are built along y from elevation and front; the mirror pairs (azimuth fans) are separate pieces for z<0 and z>0. Eight culms slender lofts from elevation and front, four plumes lofts along their bone, rhizome tuft a cone loft. The render shows a real fountain of arching blades with four plumes.
- **1:500.** Plan-outline skirt (extrusion of the simplified drawn plan outline, 0.06 to 0.4 m), dome loft to 1.15 m and spire loft above, from the scale's own outlines.
- **Structure (all scales).** Cores follow the centre path of each bone's own body piece (blade-1 piece for a blade bone): culm cores 9 mm, plume rachis 9 mm (1:50), blade midribs; joints are balls at the start of the child core (1:50 and 1:100 only; 1:100 merges the `b` joints); support is a horizontal plate under the tuft (the kit had made it a vertical plate; fixed in `author-47-struct-fix.ts`).
- **Thermal.** Hand map blades -> blade pieces, plumes -> plume pieces, culms -> culms + tuft; leaf-model blades are exchange area (coverage only, `areaM2` untouched). 1:50 culm exposure measured 0.603 against 0.5: `areaM2` of `culms` corrected 0.5 -> 0.603 (factor 1.2, not a C16/C18 region); no other `areaM2` changed. 1:200 culm capsule chains are referenced as culms.
- `decisions`: three real bilingual sentences per scale (body, thermal, structure).

### 2D drawing defects repaired at the root (catalog asset `🔣️.json`)
1. 1:50 elevation plumes were drawn 0.30 m long (bone length) while the 1:50 front/plan, 1:100-1:500 and `dimensions.heightM 1.75` use 0.403 m (elevation 1.675 vs front 1.771 m, C11 fail): elevation plume paths scaled 0.403/0.3 along the bone (`author-47-fix2d-grass50.py`).
2. 34 shapes: closed outlines with `filled: false` (1:50 culm tops and plugs, 1:100 culm axes, 1:200 blade tips and plumes) set filled; open detail strokes with `penMm` 1.1-1.99 (that is 0.1-0.4 m of real width at these scales, drawn over a 15 mm culm) lose the override and use the scale's pen table (1:50 culm nodes, 1:200 culm axes) (`author-47-fix2d-grass-fill.py`). The nodes alone drew twelve 90 mm squares of false "drawn only" area in the 1:50 elevation.
3. 1:50 front: the left blade of every mirror pair (z<0) was drawn with a 0.022 m thick band, the right one with 0.04 m, while the plan draws both 0.055 m wide and the pairs are mirror-symmetric by construction: 24 shapes replaced by the mirror of the right one (separate `b` shapes) or by right half plus its mirror (merged `a` V shapes) (`author-47-fix2d-grass-front50.py`). Front IoU 0.83 -> 0.945 by this alone.
4. 1:100 plan x-blade capsules `*-1-b` and 1:200 `*-2-b` ended one tip radius beyond the blade (plan 51 mm longer than elevation, 39 mm deeper than front, C11): end point moved back by the tip radius (`author-47-fix2d-grass-caps.py`).

### What I saw (renders of all four scales opened after the last change)
- 1:50: overlays green with thin red/blue edge slivers, body a convincing arching tussock with four plumes, thermal blades red/plumes green/tuft purple, structure thin midribs and culm cores inside the ribbons and a horizontal plate. Good.
- 1:100: front green (0.972), plan and elevation carry large red/blue patches; the body is a pile of flat, interpenetrating bundle plates. Not a good model; stated plainly.
- 1:200: see numbers below; the same construction with fewer plates; same verdict.
- 1:500: skirt + dome + spire read as the merged clump; overlays green except thin edge slivers.

### What is not done and why (numbers)
- **1:100 and 1:200 body.** The scale's own drawings are not views of one body: the elevation draws each blade bundle as a filled wedge lens (area 0.1-0.3 m2 each), the plan four large filled lobes ("crown-fill") plus thin capsules, the front only thin bands; the three-view visual hull has IoU 1.000/1.000/0.978 against the drawings (1:100) so a body exists, but it is a thick mass, and my slab-like bundle pieces approximate it only to IoU 0.852/0.972/0.849 (1:100, limits 0.90/0.88/0.88) and 0.800/0.866/0.724 (1:200). A faithful model there is a fan of ribbons at all azimuths filling the plan lobes (as at 1:50), which needs the 2D to be redrawn consistently (elevation wedge = projection of that fan, front bands = its extreme ribbons); I did not do that. Bounds: 1:100 width 0.0449 vs 0.0428, 1:200 width 0.075 and depth 0.359 vs limits 0.043/0.036 (z-blade tips dropped by the slab scan).
- **Cross-scale 1:100 (vs 1:50) and 1:500 (vs 1:200).** Follows from the above (1:100 elevation IoU 0.537 vs limit 0.285 passes, but plan/Hausdorff fail; list in the check file).
- **1:500 thermal total.** Exposed 2.25 m2 against `areaM2` 0.9 m2 (non-exchange regions): the merged spire loft (culm and plume zone) exposes far more than the physical plume area, and the culms are buried in it. Written to `author-requests.md`.
- Monotonic triangle counts: 1:100 structure needed a strict drop (joints of `b` bones removed), 1:200 mass pieces use fewer stations; both pass now.

Experiment recorded for 1:100: pairing every front band with every elevation lens of its side (`maskPiecesLens` in `author-47-grass.ts`, 269 pieces) reaches IoU 0.881/0.965/0.916 (elevation still below 0.90) and breaks the monotonic solid count against 1:50 (282 vs 73); pairing within the same bone segment gives 0.848/0.950/0.794 (174 solids). Neither was adopted; the stored 1:100 and 1:200 bodies come from `maskPieces` (49 and 47 solids). To reproduce everything: `bun author-47-grass.ts --scale N --write`, then `kit-thermal.ts <asset> --scale N --map "$(bun author-47-grass.ts --scale N --map)" --write` (1:50 with `--apply-area`), `author-47-thermal-patch.ts --scale N` (200, 500), `kit-structure.ts <asset> --scale N --write`, `author-47-struct-fix.ts <asset> --scale N [--r 0.0045 at 1:50]`, `author-47-grass-decisions.py`; at 1:100 the joints of the `b` blade bones were removed by hand (monotonic structure).

## 2. vegetation/weeping-willow — not started

I read its drawings (1:50 elevation 31 shapes: trunk, six limbs, eight 600-point curtain outlines `curt-*`, three caps `cap-*`; front 54 shapes and plan 48 shapes: 44 azimuth sheets `f-c<k>-<angle>`; heights 11.1 m, width 16.2 m) and judged that the `pieceHull` kit built here would carry it (trunk and limbs as hull lofts from elevation and front, each azimuth sheet a curtain loft along y from its elevation/front outline, caps as lofts), but I did not build it. Re-assign. Note for the next author: the author-02 source still holds 692/702/48 solids at 1:50 (untested); do not reuse it.

## 3. vegetation/small-leaved-lime — not started

Not opened beyond its row in the assignment list. Re-assign; same remark about the author-02 source (573/591/49 solids at 1:50, untested).
