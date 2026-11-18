# Silhouette guard: machinery (second pass)

Scope: the nine machinery assets of `catalog-silhouette` that still failed after `silhouette-machinery-vehicles.md` (heat-pump-outdoor-unit, wheel-loader, concrete-mixer-truck, scissor-lift, forklift, excavator, diesel-generator, oil-pumpjack, tower-crane). The guard was not loosened; every repair is a span-preserving value edit or an inserted shape (`mvlib.py` on `sil-span.py`), per mismatch decided as drawing wrong, volume wrong or declared dimension wrong. Declared dimensions were never changed.

Result: see the table at the end. Library gates: `validate-all.ts`, `machinery-validate-some.ts` (validation of the machinery assets only), `catalog-silhouette` per asset through `silhouette-triage.ts current`.

## Tools of this pass (ticket folder)

`machinery-env.sh` (shell helpers `tri`, `diag`, `blobs`), `machinery-blobs.ts` (connected mismatch blobs per view with the nearest volume solids and drawn shapes and an ASCII patch), `machinery-hull.ts` (projected convex hull of a volume solid, optionally in a bone frame), `machinery-bones.ts` (bone frames), `machinery-validate-some.ts`, `machinery-validate-file.ts`. Repairs: `machinery-<asset>*.py`.

## Findings that apply to several assets

1. Extruded profiles (track belts, cabs, counterweights) have square plan and front corners; drawn rounded rectangles (`r`) cut 20 to 60 mm off every corner. Fix: `r` 0 or an explicit polygon.
2. A revolved tyre has a chamfered tread; a plain drawn rectangle or rounded rectangle differs by 20 to 60 mm at the corners. Fix: the polygon of the revolve profile (lug blocks included) in the front and plan (wheel loader, forklift).
3. A flat drawn pen outline or stroke that ends at a silhouette edge stands out by half its pen; a hairline crack between two volume parts (a polygon arch against a tyre circle, a slab floating over a rounded corner) is an extra boundary in the volume that a drawing cannot copy. Fix: overlap the parts by 10 mm in the volume (forklift wheel arches, excavator hood slab, concrete mixer windscreen), end strokes inside the outline.
4. A revolved cylinder has flat ends; a drawn capsule rounds them by its radius and a drawn rectangle extruded along the lateral axis is a box in the plan. Fix: circular `member` shapes between the volume end points (excavator cylinders, all scales): they are drawn in the true foreshortened outline in every view. The validator counts member shapes per bone against structure entries; one entry per drawn member is added (a duplicate `of` is accepted).
5. Pins, bosses and eyes (revolves about the lateral axis) were drawn only in the elevation; the front and plan need the same ellipse shapes with the lateral extent of the revolve (origin z to origin z plus profile length).
6. A slender volume member widens to the nearest drawn strip of its bone (see the earlier note); a two millimetre wide pen stroke is dropped by the raster (use two strokes 20 mm apart at 1:100).
7. A static shape without solid is carried by its bone at rest; the validator (`lateral-ambiguous`) refuses flat shapes longer than 0.2 mm paper times the denominator on bones that leave their plane in a pose.

## Per asset

| Asset | Causes | Repairs (scripts) |
|---|---|---|
| heat-pump-outdoor-unit | lid 1.12 m deep and vane 5 mm short: bounding box z 1.12 against 1.10 m and x 0.7124 against 0.72 m; 1:200 front drew the casing down to the ground between the feet; 1:50 slender guard spokes followed the 0.045 m cage polygon | lid 1.10 m in volume and drawings, vane 0.245 m, casing on the two feet at 1:200, cage rectangle plus a thin spoke stroke at 1:50 (`machinery-heat-pump.py`) |
| wheel-loader | rounded tyre rectangles against chamfered lug tyres; lugs gap; wheel bolt heads 13 mm off the tyre; hub plate on one side only; tilt cylinder end caps (elliptical in the plan); lift pivot, tilt lugs, pin and bucket ears missing in the plan or front; tilt cylinder and rod missing in the 1:500 and 1:200 plan | `machinery-wheel-loader.py` to `-5.py` |
| scissor-lift | vent outlet 10 to 30 mm beyond the 1.85 m chassis; chassis bays left a see-through slit above the boxes; lift cylinder and rod eye drawn in the front but missing in the volume; plan chassis ignored the wheel wells | `machinery-scissor-lift.py` to `-5.py` (vent outlet one feature thick, 5 mm protrusion at every scale, bay holes to 0.40 m, cylinder and eye solids, wheel well polygon) |
| forklift | tyres, plan chassis and counterweight corners, seat back height 1.66 m, pallet runner missing in the front, hubs and rim bolts 6 mm beyond the 1.2 m track, wheel arches touching the tyre (crack), elevation outlines of chassis and counterweight, vent beyond the chamfer, steering wheel edge-on, 2 mm wedge beside the pallet goods | `machinery-forklift.py` to `-6.py` |
| concrete-mixer-truck | hub caps 21 mm beyond the 2.55 m track at 1:100; rounded cab corners; cab outline; hopper and drum seam; chute side walls not on the flat floor (1:50); windscreen slab 10 mm out of the cab front; 1:500 cab bottom | `machinery-concrete-mixer.py` to `-8.py` (hopper hulls with `machinery-hull.ts`) |
| excavator | rounded track belts, front track frame bottom, hood rail posts, hood top slab, pins/bosses/eyes only in the elevation, capsule cylinders against flat-ended revolves, stack tip, bucket teeth against the lip wedge at 1:100, no cylinders at all at 1:500 | `machinery-excavator.py` to `-11.py` |
| diesel-generator | door line, door handle and end door stood out of the 6.06 x 2.44 m envelope; silencer hairline crack; missing roof edge slab, lifting eyes, stack hinge; hood slab 20 mm out | `machinery-diesel-generator*.py` |
| oil-pumpjack | the slender polished rod followed the 2.4 m slab strip in the plan (stroke 3.4 m wide); cracks in skid and motor; missing crank hub and wellhead flange shapes; gearbox corner radius; leg and horsehead outlines | `machinery-oil-pumpjack*.py` |
| tower-crane | see below | `machinery-tower-crane-*.py` |

## Tower crane

Guard: 0 failing checks at the four scales. Causes and repairs (see the scripts): a filled tower head triangle counted as a 2.4 m strip and widened every slender leg and pendant; unfilled member shapes without pen made volume bars seven times thinner than the drawing; the drawn skip and mast foot ignored the taper of their extrusions; the guard tiles drop pen strokes whose ring bounds miss the tile box; wide pens are capped at 2 mm; ballast, cab and hook silhouettes were off.

## Tower crane, third pass (zero failing checks, zero validation issues)

Scripts `crane-1.py` to `crane-10.py`, tools `crane-tiles.ts` (reproduces the banded guard tile by tile and lists the mismatch blobs of the failing tiles with worst cell, nearby solids and shapes; env `COLS`, `PATCH=x,y,rx[,ry]`, `NEAR`), `crane-strokes.ts` (the strips the guard draws for slender volume solids), `crane-shapes.ts` (evaluated drawn shapes).

1. The 1:50 elevation H of 1.03 m was one missing drawing: the volume bars `jib-chord` and `jib-rail` (named without a trailing hyphen at 1:50) were skipped by the generator pattern `jib-(chord|rail|...)-`, so the lower chord strip stood 1.1 m from the nearest drawn boundary along the whole jib. Both got elevation members (0.1 m, the 2 mm pen cap) and four end-on dot members in the front.
2. A slender volume strip ends where the guard says, not at the bar end: jib strips x 0.059 to 55.050 m at 1:50 and -0.057 to 54.949 m at 1:100, counter-jib strips -16.550 to 0.014 m (1:50) and -16.564 to 0.031 m (1:100). The flat member end lies 2 mm inside its end point; chord members now end on those values (`crane-3.py`, `crane-8.py`).
3. An end-on member whose end circle covers more than half of the volume dot is filled by the guard as a disc of its section, not stroked: a 4 mm section draws nothing, the 0.127 m pen section drew a disc twice the volume dot and closed a gap that stays open in the volume. End-on chord members take the section of the dot (1:50 jib 0.07 m, 1:100 counter-jib 0.078 m, jib 0.056 m).
4. A 7 mm slit between two upright strips (jib diagonal, counter-jib diagonal) closed in the volume by a tilted dot corner but stayed open in the drawing, where the pocket edge became a drawn-only boundary (hole filling is per mask). The four counter-jib diagonals of the 1:100 front are 0.078 m wide.
5. 1:50 front tower head legs end 18 mm further along their axis.

Result: `catalog-silhouette` triage reports zero failures for tower-crane at all scales; `validateAsset` reports zero issues. The 1:100 front and elevation margins are small (front H 0.0300 m against the 0.03 m limit).
