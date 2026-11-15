# Author 39 — components (tied-canopy, tilt-window, wheelchair-ramp)

Generators `author-39-canopy.py` (+ `author-39-canopy-geom.py`, `author-39-lib.py`), 2D root fix `author-39-fix2d-canopy.py`, helpers `author-39-dump.py`, `author-39-crop.py`, `author-39-areas.sh`. Check = product engine (`--product`). Counts are body / thermal / structure entries.

## components/tied-canopy — spliced

| | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| solids | 28 / 22 / 17 | 16 / 16 / 14 | 5 / 7 / 4 | 4 / 5 / 4 |

Check (all scales): every rule passes (silhouette IoU elevation 0.990-1.0, front 1.0, plan 0.999-1.0, p95 <= 19 mm; bounds, feature, monotonic, cross-scale, slender, copied, structure incl. both support anchors) except `volumes-thermal-coverage` `areaDeviation:wall-plate` (0.81 at 1:50, 0.90 at 1:100; declared 0.48 m2, model exposes 0.093: ratio beyond 1.5, in `area-review.md`, not applied). 1:200/1:500 pass (total 1 %).

Modelled by hand (1:50): IPE 160 as one extrusion of the true section (root radii 9 mm, 20.2 cm2; the seed's three boxes had a 5 mm web below the feature); wall in its three layers with cut-outs round the anchors, hidden 12 mm reinforcement mats (repeat), anchor blocks with 20 mm end plates and hex nuts, glass sheets 20 mm with a slot in the sheet the strut passes through, the strut as a 60.3 mm tube with real fork cheeks (hull gussets outside the flange tips at the beam, on the end plate at the wall) and pins. Thermal: stacked real-thickness layer slabs for wall and anchor region (render slab exposed, cut edges as section faces), glass underside as section face. Structure: IPE member, hollow tube 60.3 x 3.2 (wall inside the skin), cheeks/pins/end plates as joints, 16 mm anchor bolts 450 mm into the concrete, pin and bushing supports. Coarser scales are own models (1:100 merged anchor blocks, 15 mm cheeks, no reinforcement; 1:200/1:500 slender node-to-node tube, glass slab with hole, wall masses).

Roots fixed in the asset: 2D 1:50/1:100 clevis rectangles (0.24 tall around a 0.16 beam, plan width 0.08 vs front 0.1) redrawn as the cheek outlines (elevation polygons, front/plan rectangles), `pin-wall` moved onto the strut axis 0.1 m from the wall node (where the 2D axis line already ended), cross pin `pin-beam-2` added (1:50). areaM2: canopy-steel 3.5 -> 2.97 (the glass covers the top flange), rod-steel 0.93 -> 1.09 (both within 1.5).

Renders: sheets of all four scales looked at, plus a 4000 px body view at 1:50 (slot in the third glass sheet, I-beam, strut ends); overlays green. Imperfect: the strut node at the beam is a fork above the flange (the authored node at web height lies inside the beam), the cross-drilled tube end is not modelled as a hole; 1:200/1:500 thermal wall-plate region is a zero-area reference/blocks.

## components/tilt-window — spliced (1:50, 1:100, 1:200; 1:500 unavailable)

| | 1:50 | 1:100 | 1:200 |
|---|---|---|---|
| solids | 11 / 15 / 9 | 9 / 9 / 9 | 5 / 6 / 5 |

Check (product engine): every rule passes (silhouette IoU 1.0 in all three views at every scale, p95 0, bounds, feature, monotonic, cross-scale, copied, structure with the hinge support at its anchor) except `volumes-thermal-coverage`: `frame-timber` exposes 2.52 m2 against 0.238 and `frame-sill` 0.39 against 0.09 (ratios 10.6 / 4.3, beyond 1.5, in `area-review.md`, not applied); at 1:200 the same two make `areaDeviationTotal` 2.15. Pane-glass (0.551 vs 0.6) and pane-edge (0.216 vs 0.24) are within 10 % at 1:50 and 1:100.

By hand: frame and sash in the bone frames (sash local u up, v = -x), rails as extrusions with the real 26 x 15 mm glass rebate, stiles 90 x 45 mm, spacer bars in the rebates, the 24 mm insulating unit as one glazing solid, hinge barrels. Thermal 1:50: glass centre and edge zone as stacked real-thickness slabs / extruded rings (4 / 16 / 4 mm, outer slab exposed, rims as section faces, C20). Structure: real stiles/rails/sill/jambs as members, hinge barrels joints, a 20 mm pin along the whole hinge axis as support. 1:100 merges rebates and spacers, 1:200 one sash frame ring with the pane inside it.
Roots fixed in the asset: added a `steel` material (hinge fitting; the asset had none); 2D 1:50 front/plan hinges were circles (a hinge with its axis along z cannot show a circle in the front): now rectangles 80 x 40 mm.
Renders: all three sheets looked at; overlays green. Not drawn, not modelled: the head stay.

## components/wheelchair-ramp — spliced (all four scales)

| | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| solids | 25 / 29 / 26 | 14 / 15 / 15 | 12 / 13 / 14 | 4 / 6 / 8 |

Check (product engine): every rule passes at all four scales (silhouette IoU elevation 0.96-0.998, front 0.93-1.0, plan 1.0; bounds, feature, monotonic, cross-scale, slender, copied, structure incl. the four ground pads; thermal areas 1:50: ramp-surface +2.4 %, ramp-side 0 %, kerb 0.1 %, handrail-steel 5.6 %; totals 8 % at 1:200, 12 % at 1:500). No `areaM2` changed; nothing to `area-review.md` (the declared values fit the honest model exactly: sides 3.51 m2 each, kerb top plus inner face 3.49).

By hand: solid concrete wedge in the four pours the 2D marks with joint lines, kerbs as extrusions of the drawn kerb outline, handrail as a capsule chain on the true path (0.30 m extensions, 0.10 m end returns, 42.4 mm tube) with six 50 mm posts and 120 x 100 x 20 mm base plates per side, plates flat on the slope (up to 5 mm in/out of the kerb, stated). Thermal 1:50: slab top as real 100 mm skins (the declared `slab-top` layer, so the kerbs do not hide area through the sampled burial test), sides as references of the pours with section faces, kerb outer faces flush as section faces. Structure: pours as members, hollow 42.4 x 2.6 mm tube segments (wall inside the skin), posts, plates as joints, a mass ellipsoid at the declared centre of mass, four contact pads under the 2 m footprints (the last cut at the ramp end). Coarser scales are own models: 1:100 one slab, no plates/returns; 1:200 three-segment rail with true tubes (`slender`) and 36 mm post bars as the 2D draws them (slender); 1:500 slab plus rails at the drawn 65 mm width, solid rods of equal stiffness in structure, structure slab member from 2.4 m (where the wedge reaches the 75 mm feature, because the raster cannot hold the thinner tip).
Roots fixed in the asset: 2D 1:500 elevation `ramp` started at x 2.4 with a 0.075 step while the plan draws 1.5 m: now the true wedge from 1.5 m.
Renders: all four sheets looked at. Imperfect: 1:500 front shows red strips (the drawn rail stroke runs from the ramp top, our rail starts at 0.83 m, posts are below the feature); the painted walking markers are not modelled; 1:500 rails cut the 0.3 m extensions short by up to 23 mm.

## Summary
- All three assets spliced and checked with the product engine. Open lines: `areaDeviation:wall-plate` (tied-canopy 1:50/1:100), `areaDeviation:frame-timber` / `frame-sill` and `areaDeviationTotal` at 1:200 (tilt-window), all in `area-review.md` as unapplied (factors 5.2 / 10.6 / 4.3).
- Not started: none.
