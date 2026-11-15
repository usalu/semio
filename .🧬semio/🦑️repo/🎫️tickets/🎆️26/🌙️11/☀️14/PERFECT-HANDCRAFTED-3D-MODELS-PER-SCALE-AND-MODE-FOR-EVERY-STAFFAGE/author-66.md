# Author 66 - machinery/wheel-loader (pumpjack and scissor lift not taken)

Scripts: `author-66-wl.py` (generator, on `author_48_wllib.py`; writes `volumes/machinery/wheel-loader.json`), `author-66-area.py` (areaM2 within 1.5 only), `author-66-far.ts` / `author-66-probe.ts` (boundary-cell probes, read only).

## Not taken
`machinery/oil-pumpjack` and `machinery/scissor-lift` are DONE and spliced in `author-35.md` (the lift source was touched again at 22:33 by someone else), so I left both alone.

## machinery/wheel-loader - spliced (all four scales authored)
Solids body/thermal/structure: 1:50 80/47/30, 1:100 50/30/23, 1:200 31/24/22, 1:500 28/21/18.

Final product check (`volumes-check.ts machinery/wheel-loader --product`, per scale): every rule PASS except
- thermal-coverage `areaDeviation` of engine-bay, counterweight, front-frame, bucket, tyres at 1:50 and 1:100 (per region, limits 15 % / 30 %) and `areaDeviationTotal` 0.66 at 1:200 and 1:500 (limit 0.30): the honest total outer areas are beyond a factor 1.5 of the committed values, so they are in `area-review.md` (section "66 machinery/wheel-loader") with hand calculations and proposals for reviewer 2; with the proposals the declared sum is 168 against 163 m2 measured;
- `volumes-cross-scale` 1:200 `p95M:elevation` 0.1875 (limit 0.06): measure at fault, see `tool-requests.md` "66 machinery/wheel-loader"; with the five handrail tubes removed from 1:50 and 1:100 (diagnostic, restored) the line reads IoU 1, p95 0, Hausdorff 0, so by C2 (slender excluded) it passes.
Silhouette, views-disagree, bounds, feature, monotonic, structure-coverage, bones, slender and copied are green at every scale (IoU 0.988-0.997, p95 1:50 elevation/front/plan 11/21/26 mm against 15/30/30, 1:100 8/11/19, 1:200 8/8/15, 1:500 19/19/19 mm).

### Root causes found
- 1:50 elevation `p95M` 24.4 -> 33.75 -> 11 mm: the MODEL was at fault, not the measure. The 24 lugs per tyre were boxes with their inner face at r 0.843 against a tread of r 0.845: at the box corners the flat face lay 1 mm outside the circle, a hairline gap ring along the whole lower tyre boundary (the "tangent contact" trap of tool-request 49), about 12 % of the boundary cells at 30-40 mm from the drawn disc. Lugs now reach 75 mm into the tread (r 0.80 to 0.875), two staggered rows per tyre (7.5 degree phase, 140 mm wide), so the side view shows the full tread circle and the front/plan see the 0.52 m tread; far cells 111 of 31 814.
- 2D defect repaired at the root (`🔣️.json`, scales 1:50 and 1:100): the hood handrail was drawn in elevation and plan but not in the front view, while the 3D has it (its posts stood 162 mm outside the front drawing); an open stroke `handrail` (z 2.3, detail, region engine-bay) was added to the front views at both scales.
- Thermal: the previous 0.2 mm skins and tunnels counted both faces and their cavity (engine-bay 41.9 m2, front-frame 68.6 m2). Hood, cab, front frame (box + tower rails) and lift arm are now closed envelope solids of their drawn profile (outer surface of the steel plate), counterweight, bucket parts, panes, cylinders and tyres reference body solids, the stack is a solid revolve, the engine block is buried in the hood, emitters stack-jet and radiator-air at every scale. Outer areas are consistent across scales (engine-bay 21.1/21.0/21.0/21.0, cab 19.0, front-frame 25.8/24.9/24.8/24.9).

### What I changed by hand
Staggered chevron lugs (1:50), the rear grille pocket (counterweight as notched centre part plus two full sides, five grille bars at 1:50; at 1:200/1:500 the grille is not drawn so one block), bucket wear edge at all four scales (the red strip along the bucket front in the 1:200/1:500 plan is gone), real bilingual `decisions` (three entries per scale), `areaM2` within 1.5 of the committed values: cab 14 -> 18.98, cab-glazing 6 -> 7.98, lift-arm 9 -> 12.51, hydraulics 2.2 -> 2.79, exhaust-stack 0.5 -> 0.524. Author 48's engine-bay 9.66 and counterweight 7.9 stay (within 1.5 of 8 / 6) and are part of the review rows.

### What I saw (sheets of all four scales opened after the last change)
Overlays green at every scale; at 1:50 the front shows a thin blue handrail line above the hood (stroke widths differ), at 1:500 the plan shows the tilt cylinder between the arms as a blue 3D-only strip (the 1:500 2D omits cylinders; the model keeps them at true width) and two thin red marks at the counterweight corners. The axonometric reads as a wheel loader at every scale: boxy hood and cast counterweight, tall cab with window openings and raked windscreen, cranked lift arms with cylinders, scooped bucket, four lugged tyres at 1:50. Thermal cutaway: nested regions complete, hidden engine block inside the hood, grille pocket at the rear. Structure: box-beam chassis, cab frame, arm, bucket, cylinders and wheels with mass ellipsoids and four support pads under the tyres.

### Imperfect, stated plainly
- Five thermal regions stay red until reviewer 2 decides (areas above); the 1:200 cross-scale line stays red until the tool owner reads C2 as written.
- The tread is a 32 mm chevron-lug approximation, not the real pattern; the grille has plain horizontal bars.
- 1:100 tyres have no lugs (area 30.6 against 35.0 at 1:50); structure joints exist at 1:50 only in the lists that had them before (pins, axles), merged into the members at coarser scales.
