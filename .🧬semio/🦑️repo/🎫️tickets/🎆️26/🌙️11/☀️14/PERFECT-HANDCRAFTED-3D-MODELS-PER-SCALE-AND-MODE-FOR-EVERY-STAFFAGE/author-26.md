# Author 26 report (machinery, round 2)

Assets: plate-heat-exchanger (done, spliced, two area findings), solar-thermal-collector (done, spliced, area findings), sorption-wheel (done, spliced, area findings). Generators: `author-26-common.py`, `author-26-<slug>.py` (they write `volumes/machinery/<slug>.json`), inspect script `author-26-inspect.ts`.

## machinery/plate-heat-exchanger

Counts (body / thermal / structure list entries): 1:50 18 / 20 / 21, 1:100 14 / 16 / 18, 1:200 10 / 12 / 13, 1:500 unavailable. Spliced with `volumes-insert.py`.

Check line (full check, product validator `[p]`): every rule green at 1:50, 1:100, 1:200 (availability, silhouette IoU 0.999/0.999/1.0 at 1:50 and 1.0 at the coarser scales with p95 0 mm, views-disagree, bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied) except `volumes-thermal-coverage`: `areaDeviation:ports` (measured/declared 8.9) and `areaDeviation:damper` (2.46) at 1:50 and 1:100. Both are beyond the C16 factor 1.5, so they are NOT applied: appended to `area-review.md` with numbers. At 1:200 only the total is compared: green (24 %).

Hand work:
- Casing: base slab, top ring (opening 0.4 x 0.4 m), four corner posts, two 30 mm end walls whose openings line up with the four duct collars (60 mm rectangular stubs with 10 mm walls), plus a 10 mm bottom and top end plate of the pack that close the 10 mm gaps (nothing floats; before, the elevation silhouette had a hole between pack and ring: p95 43 mm).
- Pack: twelve 45 mm sub-stacks alternately 0.40 and 0.38 m deep (the alternating open/closed channel ends of a cross-flow pack); at 1:100 a single block; at 1:200 a single block and the corner posts merged into 50 mm end panels.
- Flap: 14 / 20 / 40 mm plate (drawn thicknesses) with a `slender` 20 mm hinge barrel at 1:50 (plan draws the barrel at x -0.22..-0.2).
- Thermal: references to body solids per region; the two vent emitters are real outlet plates (one feature thick, `sectionFaces` +-u, tagged with the emitter id) in the mouth of the upper-right and lower-left collar. Regions have one layer, so no nested shells.
- Structure: members are references to the casing, pack, collars and flap; hinge and end plates are joints (1:50, 1:100), compact mass boxes for pack (12 kg) and casing (5 kg), contact pad with `anchor: base-contact` as the support.
- Root fixes: `dimensions.heightM` 0.634 -> 0.62 (all three scales' 2D draw the asset 0.62 high: elevation hinge and front flap top at 0.62; 0.634 contradicted every drawing); `areaM2` frame 0.9 -> 1.2 (measured 1.205 at 1:50, ratio 1.34) and plates 0.6 -> 0.69 (measured 0.685, ratio 1.14), both within 1.5.

What I saw (all three sheets opened after the last change): overlays fully green, the 1:50 axonometric shows an open-fronted casing with the stacked, alternately recessed plate edges between the posts, two collars on the visible side, flap with hinge on top; thermal cutaway shows frame (red), plates (green), ports (purple), damper (yellow) and the hatched emitter plates in the collar mouths; structure shows the frame and pack members with the hinge joint and pad. 1:100 and 1:200 read as the same object, merged to single blocks.

Imperfect: ports and damper areas (see above, needs the coordinator's decision); the hinge is the only part drawn differently by elevation (line) and plan (filled 20 mm), modelled as plan draws it.

## machinery/solar-thermal-collector

Counts (body / thermal / structure list entries): 1:50 11 / 12 / 18, 1:100 10 / 11 / 17, 1:200 9 / 10 / 15, 1:500 4 / 9 / 12. Spliced with `volumes-insert.py`.

Check line: every rule green at all four scales (silhouette IoU 1.0 in all views at 1:50-1:200; at 1:500 front IoU 0.895 with p95 188 mm, see below; bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied) except `volumes-thermal-coverage`: `areaDeviation:housing` (measured/declared 6.4 at 1:50, 6.7 at 1:100) and `areaDeviation:stand` (2.1 / 2.4) per region, and the 1:500 total (+42 %; 1:200 total passes at 22 %). Beyond C16's 1.5, so not applied; two rows appended to `area-review.md`.

Hand work:
- 2D root repair (listed): the 1:50 and 1:100 elevations draw the tray only as a section outline; the near side wall that front and plan show (30 / 35 mm) was missing, so no solid could agree with all three views. Added one filled shape `tray-side-wall` (bone housing, region housing, the tilted outer rectangle, z 3.1, tone 0.35) in front of nothing and behind the layer shapes, to both elevations. `areaM2` absorber 1.8 -> 2.38 (measured 2.38 at 1:50, ratio 1.32).
- Everything is modelled in the collector frame (35 degrees, s 0..2 m along the slope, t 0..0.1 m, z +-0.5 m) taken from the drawn tray points; tray = the drawn C-profile as one extrusion; strut tops cut by the drawn outlines; nine copper risers as one repeated slender revolve; insulation, glass 10 mm, absorber 8 mm (thickened to the 7.5 mm feature), headers 33 mm.
- Coarser scales: 1:100 merges insulation into the tray floor and thickens plate/risers; 1:200 is tray plus absorber slab plus glass slab and buried 45 mm headers; 1:500 is one slab plus 75 mm rails and struts, with glass, absorber and headers as real hidden thermal and structure solids (the body has none).
- Thermal: references to body solids; the thermal reference of glass and absorber plate declares `sectionFaces: ["-v"]` (their lower faces lie inside the sealed collector, the hidden back of a one-sided patch, C1); at 1:500 the hidden slabs carry the same flag. Plume emitter: a 0.5 m disc on the glass at the upper edge, one feature thick.
- Structure: refs for members, bracket plates at the strut tops as joints (1:50, 1:100), masses at the centres of housing, glass, absorber (and headers at 1:50); the stand mass (COM 0.35 m, in the open space below the tray) is split into front and rear strut lumps; contact pads under the two rails with `anchor: base-contact`. Masses are inset or coincident so they do not bury thermal faces of their own parts (burial also tests against the structure list).

What I saw (four sheets opened after the last change): overlays green; the tilted tray reads as a collector with stand, struts meeting the tray underside, the thermal cutaway shows glass, absorber, header, housing layers and the plume disc; at 1:500 the front overlay shows the drawn foot block (red) between my two leg strips (blue slivers): IoU 0.895 >= 0.88, p95 188 mm <= 300 mm, a deliberate generalisation to the 75 mm feature, not a full-width foot slab (which would add about 2.4 m2 of exposed stand area).

Imperfect: area findings above; the 1:500 front overlay is visibly not the drawn foot block; the glass underside hatch (section faces) is hidden inside the collector.

## machinery/sorption-wheel

Counts (body / thermal / structure list entries): 1:50 8 / 10 / 15, 1:100 8 / 10 / 15, 1:200 5 / 8 / 11, 1:500 unavailable (expanded counts drop through the repeated spokes). Spliced with `volumes-insert.py`.

Check line: every rule green at 1:50, 1:100, 1:200 (silhouette IoU 0.998 / 1.0 / 1.0 in elevation / front / plan, p95 0-4 mm; bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied) except `volumes-thermal-coverage`: per-region area at 1:50 and 1:100 for casing (ratio 2.95), disc (4.4), regen (2.3) and the 1:200 total (3.4x declared). Beyond C16's 1.5: not applied, three rows in `area-review.md`. Root fix: `areaM2` hub 0.012 -> 0.0081 (measured; ratio 0.68). Tool finding in `tool-requests.md`: at 1:200 a disc touching or overlapping its ring radially erased the ring's exposed area in the product measurement (0.05 against 0.62 m2 by the owned engine); I designed around it.

Hand work:
- Casing: revolved C-section drum (band + two flange rings) on a trapezoid stand block; matrix annulus around a flush 45 mm hub with 5 mm clearance to the band, held by the flange rings (planar contact); eight radial slender spokes per face half embedded (1:50), four texture spokes (1:100), none at 1:200; seal plates (84 degree sectors, 10 / 15 / 30 mm) on the fixed seal bone lifted off or flush so that spokes pass under; honeycomb strokes not modelled. At 1:200 the drum has a 30 mm band and 30 mm flanges and the matrix is a 0.16 m deep solid cylinder, the seal plates stand 2.5 mm proud of the casing faces (bounds still within the 30 mm tolerance).
- Thermal: references; the two vent emitters are one-feature plates just outside the faces, computed as the emitter disc (0.4 m / 0.25 m) clipped to the matrix and to the regeneration sector (convex polygon clipping), with `sectionFaces` +-w; at 1:200 the hub is a real hidden cylinder inside the matrix.
- Structure: refs for members, a two-bar spider behind the wheel (structure only, inside the silhouette, not drawn in 2D) with an axle joint through the hub (1:50, 1:100), three compact masses, contact pad with `anchor: base-contact`.

What I saw (all three sheets opened after the last change; the first 1:200 version showed a red 10 mm ring gap and was rebuilt with the flange design): overlays green, axonometric shows the drum with the lit hot sector plate and spokes (1:50), the thermal cutaway the matrix/hub/emitter regions, the structure a solid wheel on its stand.

Imperfect: the three area findings; the 2D honeycomb is not a solid; the spider bars and axle exist only in structure; at 1:200 the matrix is 0.16 m deep instead of 0.18 m and the seal plates protrude 2.5 mm.
