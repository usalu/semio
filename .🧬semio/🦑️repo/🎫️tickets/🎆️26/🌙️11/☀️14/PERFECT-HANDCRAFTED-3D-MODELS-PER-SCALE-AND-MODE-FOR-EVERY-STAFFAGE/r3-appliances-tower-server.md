# Round 3 — appliances/tower-server

State: DONE (2026-11-20), spliced into the catalog; the residual imperfections are listed plainly in section 7.

No earlier round-3 work existed for this asset; it was rebuilt from the roots. The catalog `volumes` equalled `volumes/appliances/tower-server.json` at the start (the bulk pass of 03:40 to 04:10 had added an authored 1:500 digital volume of two boxes; it is replaced). Everything below describes the catalog file as it is now.

Scripts (ticket folder): `r3-tower-server-kit.py` (measures, world to upright-bone helpers, component mass table, corrected rig), `-model.py` (every body, thermal and structure solid of every scale by hand, volume decisions), `-roots.py` (anchors, emitter directions, area, references, notes, structural assumption, the 2D shapes and decisions of 1:50, 1:100, 1:200), `-build.py` (candidate under `🗑️generated/r3-tower-server/` and `volumes/adhoc/r3-tower-server.json`), `-apply.py` (writes changed root members into the catalog file in its formatting), `-ship.sh` (build, validate the candidate with the volume rules, print suitability, apply, splice, validate the catalog file). Running the chain again changes nothing.

## 1. Reference

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Envelope | 440 x 195 x 480.5 mm (H x W x D), from 13.5 kg | HPE ProLiant ML110 Gen10 data sheet a00021851enw | 0.45 x 0.20 x 0.45 m, 11 kg kept (asset dimensions are inside the class) |
| Envelope, small class | 360 x 175 x 453.75 mm with bezel, at most 11.68 kg | Dell EMC PowerEdge T150 Technical Specifications | confirms depth 0.45 m and the 11 kg |
| Power supply | 150 x 86 x 140 mm, rear top | ATX12V Power Supply Design Guide 2.2 | 150 x 86 x 140 at y 0.352 to 0.438, flush in the rear panel |
| Mainboard | 305 x 244 mm on the right wall seen from the front, port field 158.75 x 44.45 mm, seven slots at 20.32 mm | ATX Specification 2.2 | board 305 x 244 on the fixed wall, port field 159 x 44.5, seven slot apertures 12.5 x 100 at 20.32 |
| Fans | 120 x 120 x 25 mm, 7 blades | product knowledge, asset emitter (0.12 m) | rear fan, cooler fan, front fan, each frame, hub and 7 pitched blades |
| Drive bay | 5.25 inch half height, 146 x 41.3 mm | SFF-8551 (knowledge) | bay 148 x 42 with optical drive and tray front |
| Sheet and bezel | 0.8 mm SECC steel, ABS bezel 2.5 mm wall, 25 mm deep | asset layers, product knowledge | chassis sheet 0.8 mm, bezel 25 mm |
| Feet | four rubber feet 40 x 30 x 12 mm, footprint 0.4 x 0.17 m | asset anchor and layers | same |
| Handedness | removable cover on the left seen from the front, board on the right | ATX convention | cover with grille at -z (the side the product elevation looks at), board wall at +z |

Sources: https://www.laptopsdirect.co.uk/PDFs/P03687-425_ProductDataSheetBrochure1.PDF , https://www.dell.com/en-uk/shop/povw/poweredge-t150 , ATX Specification 2.2. Three references were added to `provenance.references`.

## 2. Audit of the state before round 3

- Rig: the single bone `case` lay horizontal from the floor centre (0, 0) for 0.45 m, so it left the case through the bezel and ended 0.225 m in front of it; centre of mass (-0.02, 0.2) an unexplained estimate.
- Anchors: `rear-outlet` at height 0.30 m while the drawn fan ring sat at 0.20 m; a 120 mm fan at 0.30 m collides with the power supply drawn from 0.33 m.
- 2D: the elevation drew the power supply as a strip on the side sheet and slits of bezel, power supply and fan ring that no side view shows; elevation bezel 0.024 to 0.438 against the front view 0.012 to 0.45; front view with two 8 mm "side wall" strips beside a full-width bezel; plan with a grille edge standing proud; 1:100 with widened bezel and rear panel; 1:200 with a 24 mm plinth cutting into the case.
- Solids (qa-visual: B/B/B, B/B/C, C/B/C): 14 boxes, the case a filled block, front details invisible, grille on the side the elevation does not show, thermal only thin plates, structure four free posts with a floating ellipsoid, ring support; 1:200 and 1:500 two plain boxes.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `rig.case` | angle 0, length 0.45 (horizontal, out of the front) | angle 90, length 0.45: upright from the floor under the case centre to the top |
| `rig.case.mass` | com (-0.02, 0.2) world | comX 0.245, comY 0.005 on the upright bone = 5 mm behind the centre, 0.245 m high, from the component table (chassis 4.9, bezel 0.4, power supply 1.7, board and cooler 1.6, two disks 1.5, optical drive 0.7, fans 0.2 kg; sum 11.0) |
| `anchors.rear-outlet` | x -0.225, y 0.3 | x 0.285, y 0.225, z -0.025 (the real fan centre on the upright bone) |
| `anchors.case-top` | x -0.1, y 0.45 | x 0.45, y 0.1 (same world point) |
| `thermal.emitters` directionDeg | rear-exhaust 180, case-plume 90 | 90 and 0 (same world directions on the upright bone) |
| `thermal.regions.front-bezel.areaM2` | 0.09 | 0.125: face 0.0876 plus the 25 mm skirt on four sides, mesh and bay face (measured 0.1259 on the 1:50 model, factor 1.39, C16) |
| `structure.assumptions[0]` | centre 0.02 m behind | 0.005 m behind, 0.245 m above the floor |
| `provenance` | 6 references | + `hpe-ml110-gen10-2020`, `dell-t150-2021`, `intel-atx-2-2`; notes extended by the construction and the mass table |
| `representations[50,100,200].geometry` | 11/9/6, 8/6/5, 2/2/1 shapes | redrawn as true views of the body: 8/7/4, 7/5/4, 3/2/1 (see section 4) |
| `representations[*].decisions` | old texts | silhouette, retained, merged, omitted, thinElements, styles, overlays rewritten (en, de) |

Other areas stay as declared (deviation at 1:50: side-panel 7.3 %, cpu-grille 6.3 %, psu-vent 8.9 %, rear-panel 3.2 %, feet 1.3 %, front-bezel 0.7 %). Dimensions, poses, clips, materials, summary unchanged; the rig has one fixed bone, so poses and clips move nothing (thermal and forces views of the default pose were drawn and looked at).

2D, ruling E1: elevation = the cover side: case sheet, 12 mm rim of the rear panel, bezel, grille field, latch, two feet, the joint under the top panel (1:50). The validator requires every region in the elevation; the power supply cannot be seen from the side, so the rear rim shape carries `regions: [rear-panel, psu-vent]` and the power supply has its own shape in the plan (intake field). Front: bezel with mesh, bay, tray and button on two feet. Plan: top sheet, rear rim, bezel, intake field. 1:200: one case shape with the five regions on two 12 mm skids.

## 4. What was modelled

### 1:50 — body 43 solids (83 with repeats), thermal 26, structure 25 (30)
Body: chassis tub of 0.8 mm sheet (floor and board wall, bent corner), removable cover (10 mm deep, windows for grille and latch) with honeycomb grille, latch well and lever; top panel with the honeycomb intake field of the power supply; 12 mm rear panel with openings for power supply, fan, port field and seven slots; inner front plate with fan and bay openings; ABS bezel with rounded corners, mesh window with honeycomb mesh, bay, button and USB block; power supply as hollow 0.8 mm case with dark core, honeycomb rear face, inlet and switch; stamped fan guard with three slot rings and four spokes; three fans (frame with duct, hub, seven blades); port field shield with four connector blocks; seven slot covers; mainboard; tower cooler (base, 13 fins, four heat pipes); optical drive with tray front; four rounded feet.
Thermal: 0.8 mm skins of cover, wall and top, grille patch, 2.5 mm bezel face, four skirt skins, mesh and bay face, power supply rear and top faces, rear face and four rim skins, feet as references, exhaust patch in the fan opening and plume patch on the top (1 mm proud), and inside the power supply, heat sink and two fans (no area).
Structure: floor pan with upturned edges, board wall with returns, two posts, four top rails and two floor cross rails as folded 20 mm angles in 0.8 mm, shelf under the power supply, drive cage, bay cage, eight riveted corners, the feet as members on one floor patch (the support), masses: power supply, board, cooler, two disks, optical drive.

### 1:100 — body 23 (25), thermal 25, structure 13 (15)
15 mm plates, perforations as 15 mm cells, guard with two rings, ports one block, slots one field, power supply and cooler plain masses, plain feet; no button, USB, blades, fins, board, front fan.

### 1:200 — body 7 (8), thermal 11, structure 9 (10)
20 mm plates around one dark core; grille, intake field, power supply, fan, slot field, mesh and bay are windows onto the core; two cross skids.

### 1:500 — body 4, thermal 11, structure 8 (9)
Digital only (the drawing is unavailable): steel block on a recessed plinth, bezel block with intake window, rear plate with the recesses of power supply and fan.

## 5. Checks (final, on the catalog asset)

- `r3-validate.ts appliances/tower-server --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `r3-validate.ts appliances/tower-server`: `VALID`.
- `volumes-check.ts appliances/tower-server --product`: `PASS (17.3 s)`. Silhouette IoU elevation/front/plan: 1:50 1.000/0.992/1.000, 1:100 1.000/1.000/1.000, 1:200 1.000/1.000/1.000. Area totals 1:200 0.005, 1:500 0.096.
- Print suitability (`r3-server-rack-suit.ts` on the candidate): 1:50, 1:100, 1:200 suitable under `office-laser`.

## 6. What the renders show (opened with the Read tool)

Seen: `r3-sheet` body at 1:50 (three times while fixing), 1:100, 1:200, 1:500; 1:50 back alone; 1:50 thermal cut and structure; 1:100 structure; `volumes-render` sheets of 1:50, 1:200, 1:500 (overlays, body, thermal, structure); `r3-2d` 1:50 (line, solid, thermal, forces) and 1:200.

- Front: dark bezel with bay and tray, button, USB block and a honeycomb mesh through which the front fan shows; two feet.
- Right side (product elevation): cover with honeycomb grille over the cooler (fins and pipes show through), latch, rear rim on the left, bezel on the right, joint under the top panel.
- Back: power supply with honeycomb, inlet and switch; fan guard with rings and spokes over the bladed fan; port field with four connector blocks; seven slots with covers.
- Left side: the plain board wall. Top: intake field at the rear, bezel strip in front.
- Thermal cut: red case skins, yellow power supply inside under its top field, green heat sink behind the grille patch, the fans, purple bezel, feet. Structure: closed frame of floor, wall, posts and rails with orange corners, masses sitting on shelf, wall and cages, green floor patch under the feet.
- Coarser scales stay a tower server at a glance: 1:100 with coarse honeycombs and a two-ring guard, 1:200 with dark windows, 1:500 a block with bezel and intake.
- 2D: the three views agree with the body; forces view shows the upright member with the weight at 0.245 m and the reaction at the feet.

## 7. Still imperfect

- Cover, top panel, rear panel and bezel are plates as deep as their folded edges (10, 10, 12, 25 mm), not 0.8 mm sheet with separate returns: a thin flat plate fails `volumes-feature`, and splitting a panel into strips to get real returns draws false seams. Only the window reveals show it.
- Sheet edges are square; the 2 to 3 mm bend radii of real panels are modelled only on the tub corner, the rear panel and the bezel outline.
- The 5 mm honeycomb is generalised to 8 mm cells (1:50) and 15 mm cells (1:100); LEDs, eject button and thumbscrews are below the feature size and absent.
- The product's light-line view does not outline latch, mesh and bay (shapes of the same region as the shape under them are merged by the product); they show in the solid and thermal views.
- `psu-vent` shares the rear rim shape in the elevation (merged regions), because the rule `region-undrawn` wants every region in the side view and a power supply cannot be seen from the side.
- `r3-sheet` shades the flat caps of extrudes with holes with streaks (rear panel); filed in `tool-requests.md`, not geometry.
- The support is one floor patch under four feet, because a support anchor takes exactly one solid and a repeat cannot place four pads of a rectangle.
- No hidden thermal regions were added; the internals carry the colour of the region they stand behind and no area. Thermal data other than the bezel area is unchanged.
- The repo test suites were not run; only the checks of the brief.
