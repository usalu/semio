# Round 3 — objects/stepladder

State: DONE (2026-11-20). No earlier round-3 work existed; started at step 1. The roots (rig, dimensions, anchors, areas, texts), the 2D of every scale and view and the volumes of every scale were rebuilt as a real five-step platform stepladder. Catalog written and spliced (catalog `volumes` == source). `r3-validate.ts objects/stepladder --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. `volumes-check.ts objects/stepladder --product`: `PASS (32.5 s)`. Print suitability: all four scales `authored`, no violations.

Scripts (all in T): `r3-stepladder-kit.py` (hand-stated geometry, bone frames, masses, sections, helpers), `-roots.py`, `-draw.py` (2D, 4 scales x 3 views, decisions), `-volumes.py` (body, thermal, structure, decisions of 4 scales), `-areas.json` (area corrections), `-build.py` (candidate; `--source` writes the volumes source, `--apply` the catalog members). The pre-round-3 catalog and source are kept in `🗑️generated/r3-stepladder/` (`catalog-before-r3.json`, `source-before-r3.json`). The bulk passes of 03:40 to 04:10 had not changed this asset (catalog volumes equalled the source at start).

## Reference (real subject)

Class: single-sided aluminium stepladder with platform, five steps (four treads plus platform), EN 131.

| quantity | value | source |
|---|---|---|
| platform height | 1.06 m | Hailo L60 StandardLine 8160-507 (retailer data: toom, topleiter) |
| ladder length (folded) / open height | 1.82 m / 1.68 m | topleiter (1.82 m), memolife (B 48 x H 168 cm) for the same model |
| width at the feet | 0.48 m (asset 0.47 kept) | memolife; Krause Corda class 0.47 (my knowledge) |
| tread depth | 80 mm | toom product image note for the L60 |
| rated load | 150 kg | Hailo L60, Krause Corda (bauhaus); EN 131-2 |
| mass of the household model | 4.6 kg (Hailo), 5.0 kg (Krause Corda 6 steps, obi) | retailer data |
| climbing leg inclination | 60 to 70 degrees; here 67.5 (asin 1.68 / 1.82 = 67.4) | EN 131-1 (my knowledge of the functional sizes), Hailo figures |
| hand/knee rail above the platform | at least 0.6 m; here stile tops 0.62 m, grab bar axis 0.555 m | EN 131-1 (my knowledge) |
| step pitch | 1.06 / 5 = 0.212 m | Hailo figures |
| construction | box stiles converging upwards, extruded grooved treads riveted between them, platform pivoting on an axle and locking on a cross tube of the rear frame, rear frame hinged to the stiles above the platform, two-link spreaders, rubber shoes | my knowledge of the class (Hailo, Krause, Zarges) |

Web search gave no manufacturer PDF; all product numbers above are retailer quotations. Wall thicknesses (1.5 / 1.2 mm) are the asset's own thermal layers.

## Audit of the state before round 3

- **Proportions.** Height 1.50 m with the platform at 1.25 m (rail only 0.25 m above the platform; pitch 0.25 m); footprint 0.68 m for a 1.25 m platform (real class about 1.0 m for 1.06 m); front rail at 81 degrees (EN 131: 60 to 70), so the ladder was a steep, narrow, unstable A. The rear frame hinged directly under the platform, which is not how the platform locks in this class.
- **Rig.** Both bones lay horizontally (angle 0, length 1.5 and 1.25 m along +x) through points unrelated to the members: the bones were not the frames.
- **Orientation.** The treads pointed to -x while the front view drew them in front: the front elevation contradicted the body (from +x the rear frame was nearest).
- **Body.** Rails 60 x 45 and 50 x 45 mm (twice the real lateral width), spreaders as 24 mm round bars, treads 120 mm deep plain blocks, no grab rail, no platform support, no fixings.
- **2D.** True to the wrong body; 1:500 inflated the rails to 134 to 160 mm.
- **Mass.** 5.3 kg with 1.5 mm walls and steel spreaders does not add up (see root fixes).

## Root fixes (catalog, old -> new)

- `dimensions`: H 1.5 -> 1.68, W 0.68 -> 1.017, D 0.47 (kept), mass 5.3 -> 7.1 kg (sum of the parts from the sections with the densities of the materials: stiles 2.25, treads 1.46, platform with steel axle 0.95, rear rails 1.12, spreaders 0.68, rest: the declared 1.5 / 1.2 mm walls and steel spreaders give a trade-weight ladder; this is my computation, not a product figure).
- `rig.bones`: front-frame (0, 0) / 0 deg / 1.5 m -> (0.4636, 0) / 112.5 deg / 1.8184 m (from the front foot along the stile to its top); rear-frame (-0.13, 1.2) / 0 deg / 1.25 m -> (1.4396, 0) on the front bone (the hinge at 1.33 m) / 141.5 deg / 1.3836 m (to the rear foot). Masses 3.95 / 1.35 -> 5.26 / 1.89 kg with their centres; both bones get a `section` (pair of box profiles 60 x 22 x 1.5 and 45 x 20 x 1.2 mm). Joints stay `fixed` (the folded state remains unauthored, see the provenance note).
- Climbing side now ahead (+x), so the front elevation shows the treads in front, as drawn.
- `anchors`: feet front (0, 0, +-0.216) on the front bone, rear (1.3836, 0, +-0.198) on the rear bone, footprints 88 x 38 and 76 x 32 mm; standing-point on the platform centre (1.1802, 0.0795).
- `thermal.regions[].areaM2` (C16, all within factor 1.5, from the 1:50 model): front-rails 0.66 -> 0.6, treads 0.4 -> 0.35, platform 0.22 -> 0.23, rear-frame 0.535 -> 0.43, spreaders 0.071 -> 0.087, feet 0.08 -> 0.062. Region titles and the spreader layer title updated.
- `summary`, `provenance.notes` (rewritten), `provenance.references` (+ `hailo-l60-8160-507`), `structure.assumptions` (new support positions).
- `representations`: geometry and decisions of all four scales redrawn (E1). `poses`, `clips`, `materials`, `presentation` unchanged (loads act through the standing-point anchor).

## Volumes per scale

| scale | body / thermal / structure | modelled |
|---|---|---|
| 1:50 | 40 (52 expanded) / 36 refs / 40 | Stiles 60 x 22 with rounded edges, end caps, 28 mm grab bar; four treads with rounded nose and anti-slip grooves, two rivet heads per tread end (texture, repeat 4); ribbed platform deck with rims, folded cheeks, stepped steel axle with spacer sleeves, rear rim hooked behind the rest tube; rear rails 45 x 20 with round hinge heads, hinge bolts with heads, 22 mm rest tube, two braces 30 x 15; per side two U-channel spreader links with pivot rivets and knuckle; four shoes with soles. Structure: hollow stiles, rails, treads, grab bar, rest tube and braces with real walls, deck and axle, links as ties, joints at tread ends, axle, rest, pivots, knuckles and hinges, four contact pads, four half masses. |
| 1:100 | 21 / 21 / 27 | Plain stiles and rails to their ends, grab bar, plain tread boxes, platform slab, rest tube, braces, one hinge boss per side, each spreader one true-size slender strap, shoes. Structure: hollow members, joints at hinge and rest, pads, masses. |
| 1:200 | 17 / 17 / 22 | Stiles, rails and spreaders slender at true size; 30 mm treads, platform, grab bar and one brace; 90 x 45 x 45 shoe blocks. |
| 1:500 | 16 / 16 / 21 | Stiles, rails and spreaders slender at true size; 75 mm tread bars, platform slab and grab bar; 100 x 75 x 75 shoe blocks. |

Silhouette IoU (elevation / front / plan): 1:50 0.999 / 0.977 / 0.968; 1:100 0.998 / 0.980 / 0.940; 1:200 0.977 / 0.997 / 0.960; 1:500 0.990 / 0.999 / 0.980. Bounds, feature, monotonic, cross-scale, thermal and structure coverage, bones, slender, copied, views-disagree: ok at every scale.

## What the final renders show (all opened after the last change)

- **1:50 body** (`🗑️generated/r3-sheet/objects/stepladder.s50.body.png`, plus `.three-quarter` and `.right`): side: an A of the 67.5 degree stile and the 74 degree rear rail meeting at the hinge head 0.27 m above the platform, tread noses in front of the stile with two rivets each, the platform spanning from stile to rest tube, the horizontal two-link spreader with its knuckle, shoes with soles. Front and back: stiles converging to the grab bar, four treads, the narrower rear frame inside with two braces. Top: stiles running back over four grooved treads and the ribbed platform. Three-quarter: reads at once as a real platform stepladder; the bones lie on the stile and the rear rail.
- **1:50 thermal**: six regions in six colours, no stray solids. **1:50 structure**: blue hollow members, orange joints, green pads under the shoes, magenta half masses on stiles and rails.
- **1:100, 1:200, 1:500** body, thermal and structure sheets: each a correctly proportioned ladder at its own abstraction; 1:500 has chunky 75 mm treads and shoe blocks by the feature rule; its structure omits the grab bar (ghost).
- **2D** (`🗑️generated/r3-2d/objects/stepladder.s50|s100|s200|s500.png` and `.s50.open-in-use.png`): clean drawings that match the bodies; 1:500 reads as an A sign, a ladder sign and a plan with four feet. The pose `open-in-use` draws the same rigid ladder (sheet `stepladder.s50.body.open-in-use.png` rendered; I opened the 2D of that pose, not that 3D sheet).

## Still imperfect (stated plainly)

- The mass (7.1 kg) and the EN 131-1 figures (60 to 70 degrees, 0.6 m rail) rest on my computation and knowledge; no manufacturer sheet or the standard text was read. The product numbers are retailer quotations.
- Sub-feature fixings are simplified: rivet heads are 10 mm texture domes only at 1:50 and are not drawn in 2D; tread-to-stile swaging, the platform latch, labels and a tool tray are absent. Stiles are plain rounded boxes without extrusion grooves.
- Tread ends are cut square although the stiles splay by 1.8 degrees (under 1 mm).
- 2D generalisation: at 1:50 the 1.8 mm play between rear rail and stile is closed in the front view; at 1:100 and coarser each tread is drawn up to the next in plan (8 mm gap not printable); thin parts are strokes of their true width.
- At 1:100, 1:200 and 1:500 the front view does not draw the spreaders (hidden behind the stiles); the `overlays` decisions say so.
- `r3-playground-swing-suit.ts` (another author's read-only helper) was used for the print-suitability check.
