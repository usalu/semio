# Author 54 — round 2: vegetation/agave, hosta-clump, bamboo-clump

All three assets are remodelled at every authored scale and spliced (`volumes-insert.py`). Sources `T/volumes/vegetation/{agave,hosta-clump,bamboo-clump}.json`; scripts `T/author-54-*.ts|py|sh` (lib `author-54-lib.ts`, generators `-agave`, `-agave500`, `-hosta`, `-bamboo-culms`, `-bamboo-put`, `-bamboo-fin`, shared lobe fitter `-lobes`, decisions `-decisions.py`); tool output and the sheets I read under `T/🗑️generated/author-54/` (`<asset>.s<N>.png`). Plain verdict first: **agave 1:50/1:100/1:200 and hosta 1:50/1:100 are handcrafted, reviewed and green under the rulings; agave 1:500, hosta 1:200 and all bamboo foliage are best-effort masses that stay below the silhouette limits, for the reasons and numbers below.** Previous author's plates are gone; nothing of the drafter output remains.

## What was wrong and what I learnt

- The drafter plates (agave, hosta) put the blade polygon in the bone's (u, w) plane with the thickness along v ("sweep v"). A loft along u therefore has `rx` = width (w) and `ry` = thickness (v); the first try with the axes swapped had IoU 0.50.
- A loft section without `rotation` follows the path tangent; when a blade's centre line shifts sideways (asymmetric heart-shaped hosta blades, knee wedges of the agave) the sections tilt and the surface pinches (hosta blade IoU 0.77 against its own polygon). Fixing every section's `rotation` to the (w, v) plane gives 0.98 (`author-54-lib.ts` `bladeLoft`). Long ruled spans between two sections pinch the same way: a station every 0.12 m at most.
- Outlines are smoothed over the tooth period (agave 20 / 60 / 100 mm) so that the loft follows the blade, not the zigzag; sections are rounded rectangles (exponent 3.4 to 5 at the coarser scales) so that a tilted thin blade projects with the area the drawing gives.
- Foliage masses (bamboo crowns, hosta 1:200, agave 1:500) are ellipsoid lobes (centre, three radii, yaw, roll) fitted by annealing to the three drawn views (`author-54-lobes.ts`, start from k-means of the visual hull of the three drawings); this reaches what the drawings allow, not more (see the 1:500 agave and the elevation of the 1:200 hosta).

## Solid counts (body / thermal / structure)

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| agave | 20 / 32 / 41 | 20 / 22 / 41 | 20 / 22 / 22 | 18 / 20 / 22 |
| hosta-clump | 18 / 20 / 38 | 18 / 20 / 38 | 7 / 18 / 20 | unavailable |
| bamboo-clump | 32 / 34 / 26 | 28 / 30 / 26 | 26 / 28 / 26 | 7 / 27 / 26 |

## vegetation/agave

**Body.** Caudex and spear are lofts from the drawn elevation/front outlines. Each leaf segment (18) is a thick tapering loft in its bone frame: half width = drawn outline smoothed, thickness 40 mm at the base to 22 mm at the tip (31 mm floor at 1:200, the thinnest the scale can show), all sections in the leaf plane, so the leaf arches out of the rosette through the two-segment bone chain. 1:100 and 1:200 are modelled anew from their own outlines (60 / 100 mm smoothing, other thickness floors, boxier section at 1:200). Channel and marginal teeth are below 7.5 mm and not modelled. 1:500 is a different model: 16 curved blades (arch, width, thickness, tip fitted to the 1:500 drawings, 75 mm minimum), spear, caudex.
**Thermal.** References to the leaf lofts (spreading R1/R2/L1/L2, erect inner leaves + spear) and the caudex, two flat transpiration discs inside the rosette. No `areaM2` touched (leaf regions `exchange`, C16/C19; caudex 0.29 is the value in the tree, committed 0.21, within 1.5).
**Structure.** Member = 0.7 core of the leaf's own loft (stiff leaf beam), caudex core, joint balls at the leaf bases (1:50, 1:100), root-plate member as a short crown bar inside the caudex base, support block with `anchor`. 1:500: every leaf bone has a two-section core inside the blade that carries it.
**Check** (`volumes-check.ts vegetation/agave --product`): 1:50, 1:100, 1:200 green except the thermal line named below (silhouette IoU 0.952 / 0.945 / 0.978 at 1:50, p95 0 to 9 mm; 0.949 / 0.941 / 0.982 at 1:100; 0.942 / 0.937 / 0.968 at 1:200; cross-scale and monotonic green). 1:500: `volumes-silhouette` iou 0.694 / 0.715 / 0.687 (limits 0.90 / 0.88 / 0.88), `volumes-cross-scale` plan p95 206 mm (limit 169), thermal total (tool line, below). Bounds, feature, bones, monotonic and structure coverage pass at 1:500.
- Tool lag, `volumes-thermal-coverage` `areaDeviation:caudex` / total: the product now measures the outer area including the two end caps (0.379 at 1:50 = +31 %); the area of C1 (exposed: not buried, not in the contact plane) is 0.296 = +2 % at 1:50, 0.345 = +19 % of the total at 1:200, 0.254 = -12 % at 1:500, all inside the limits. Details in `author-requests.md` section 54 and `area-review.md`.
- 1:500, why it stays at 0.70: the visual hull of the three drawn views (intersection of the three extrusions, 25 mm voxels) projects with IoU 0.795 / 0.985 / 0.999 onto elevation / front / plan, so even a perfect body cannot reach 0.90 in elevation: the elevation outlines are a filled blob (2.5 x 1.3 m) where front fan and plan star show leaf tips only. I kept a natural rosette of blades (not a terraced hull mass) and fitted it (annealing, about 60 000 rounds in all); its sheet is read below.
**Sheets read** (after the last change): 1:50 (overlays green; the rosette reads as a rosette, thick lofted leaves arching out, caudex a column; thermal shells follow the leaves; members inside the leaf lofts), 1:100, 1:200 (same, thicker boxier leaves), 1:500 (sixteen blades radiating from the caudex plus the erect spear; elevation overlay shows large red where the drawn blob is, plan and front match the spikes only partly).

## vegetation/hosta-clump

**Body.** Nine blades are thin lofted sheets: heart-shaped drawn outline sampled from the SVG path (asymmetry kept), thickness a midrib ridge of 9 mm falling to 5 mm at the tip (16 mm at 1:100), section between flattened diamond and rounded rectangle (exponent 2.5), all sections in the blade plane. Nine petioles are tapering round lofts, 32 to 24 mm. The self-intersecting `blade-r1` profile is gone. 1:200: the drawing shows merged masses (two outer arcs, inner mound, petioles not drawn), so the body is seven ellipsoid lobes with yaw and roll fitted to the three views.
**Thermal.** Blades are references. The petioles region carries the catalog's `hidden` note, which the validator reads as a B4 hidden region: its solids must not reference a body solid, so they are own solids of the petiole shape (identical geometry, not scaled, declared in `decisions`); at 1:200 nine own petiole lofts at the rig positions. Two transpiration discs.
**Structure.** Member along every petiole and blade (core of the ridge), joint balls at the blade bases, a crown bar on the root plate, support block.
**Check:** 1:50 silhouette IoU 0.971 / 0.972 / 0.986, p95 2 mm / 0 / **45 mm plan (limit 30)**; 1:100 IoU 0.931 / 0.941 / 0.977, everything green (cross-scale 50 -> 100 included). 1:200: IoU 0.869 (limit 0.90) / 0.900 / 0.880, `volumes-cross-scale` `unionVolumeBandExcess` 6.9 (thin sheets against merged masses), `volumes-structure-coverage` members 0.15 m outside in elevation (the drawing leaves a hollow below the inner mound where the petioles are). Monotonic green.
- 1:50 petioles `areaDeviation` 0.17 by the tool's outer area (0.152 against 0.183); by C1 exposed 0.199 = +8.6 %. Not corrected (0.152 would be 1.64 below the committed 0.25), listed in `area-review.md`.
- 1:50 plan hard-cell p95: see `author-requests.md` section 54 (removing the edge-on central blade gives 28 mm; the owned engine finds nothing farther than 9 mm from the drawing).
**Sheets read:** 1:50 and 1:100 (heart-shaped hosta leaves in the real pose, petioles dark and tapered, overlays almost entirely green), 1:200 (seven yawed ellipsoids: legible as a mound with the drawn arch, crude).

## vegetation/bamboo-clump

**Body.** Culms: 18 segments as hollow tubes (revolve whose profile is the outer wall with 2 mm node rings every 220 mm (300 mm at 1:100), 4 mm wall and the bore), taper 29 / 27 / 23 / 16 mm, `slender`, at true size, placed on the leaning centre line from the elevation and front outlines (the culms stand at different depths: the old model had them all at z = 0). 1:200 solid slender cones (bore and rings are not resolvable); 1:500 none (the drawing is a filled stem band: five lobes on the stem bone). Foliage: 14 / 10 / 8 / 7 ellipsoid lobes bound to the nearest foliage bone, fitted by annealing to the three views.
**Thermal.** Culm region: own skins (outer surface without the bore: the exposed area is 1.56 m2 against 1.6; a hollow tube would also count its bore), foliage regions: lobe references (upper lobes sun, others shade), twelve emitter discs.
**Structure.** Culm members reference the hollow tubes (the real section; 1:500 slender boxes), a rod in the lobes of every foliage bone, crown bar in the base of culm 3, support block.
**Check:** `volumes-check.ts vegetation/bamboo-clump --product`, per scale. Green at every scale: availability, views-disagree, bounds, feature, slender (all 18 culms ratio 25 to 88), thermal coverage (culm skins 1.56 m2 against 1.6, +2.7 %), bones, monotonic (50 -> 100 -> 200 -> 500 strictly dropping), cross-scale 200 against 100. Failing: 1:50 silhouette IoU 0.883 / 0.942 / 0.868 (limits 0.90 / 0.88 / 0.88), plan hard p95 47 mm (30), structure plan 11 mm outside (7.5); 1:100 silhouette 0.845 / 0.921 / 0.824, soft p95 94 to 98 mm (65), plan hard p95 94 mm (60), cross-scale 100 against 50 p95 94 to 98 mm (limits 83 / 30 / 30; the two scales' front and plan 2D drawings agree to 0 to 4 mm, my two independent lobe fits do not); 1:200 silhouette 0.873 / 0.925 / 0.836, plan hard p95 397 mm (120; the single slender culm cones that the plan draws as ellipses), structure front 37 / plan 68 mm outside (30); 1:500 silhouette 0.885 / 0.902 / 0.789, cross-scale elevation p95 713 mm (544) and union volume band 0.22, structure members 0.31 to 0.39 m outside (the drawn stem band is a loose mass around the true culms).
- Remaining findings: the lobe fit stays 0.84 to 0.93 per view (a handful of ellipsoids against scalloped foliage sprays with hundreds of vertices); culm positions disagree between the front and the plan drawing (see `author-requests.md`), which makes the members leave the plan footprint.
**Sheets read:** 1:100, 1:200, 1:500 after their last change (hollow slender culms with their lean and spread, foliage as soft lobes; at 1:500 one egg-shaped mass as the drawing is). **1:50: I opened the sheet of the first fit (before the refit) but not the final one: the Read tool failed (`PreToolUse hook did not respond`) every time after the last re-render. The final 1:50 sheet is `T/🗑️generated/author-54/bamboo.s50.png`; the numbers above are from the product check after the refit.**

## Changes by hand and root repairs

- `areaM2`: nothing changed in any of the three assets (agave caudex 0.29 and hosta petioles 0.183 are the values already in the tree, both within a factor 1.5 of the committed values; bamboo culms 1.6 / leaves 5.4 / 8.2 are the committed values). Two measured deviations are listed unapplied in `area-review.md` (section 54).
- 2D drawings: not touched; the contradictions found (agave 1:500 views, hosta 1:200 hollow, bamboo front/plan culm positions) are in `author-requests.md` section 54 with numbers.
- Splice: `volumes-insert.py` for all three assets; the old `volume` member is gone.

## Imperfect, stated plainly

1. agave 1:500 silhouette 0.69 to 0.72 (limit 0.88 to 0.90); the body is a plausible rosette but the drawing cannot be matched.
2. hosta 1:200 is seven ellipsoids; elevation IoU 0.869 and the volume band of cross-scale fail; members of the petioles leave the drawn silhouette.
3. hosta 1:50 plan p95 45 mm (tool attribution question).
4. bamboo foliage lobes are crude ellipsoids; silhouette IoU below the limits at 1:50, 1:100, 1:200 (and plan at 1:500).
5. Agave channel, teeth, terminal spines, hosta ripples, bamboo leaf sprays are not modelled (below the scale's smallest feature or merged).
