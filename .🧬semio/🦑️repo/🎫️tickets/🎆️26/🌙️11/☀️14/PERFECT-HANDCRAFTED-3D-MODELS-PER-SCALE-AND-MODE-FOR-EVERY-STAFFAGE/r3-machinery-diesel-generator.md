# Round 3 — machinery/diesel-generator

Status: DONE (round-3 author, 2026-11-20). Spliced; `r3-validate` VALID; `volumes-check --product` PASS at every scale. No earlier round-3 state existed (no `r3-diesel-generator-*` scripts, no report, catalog `volumes` identical to the source `T/volumes/machinery/diesel-generator.json`); start: round 2, `r3-validate` VALID, `volumes-check --product` red on `volumes-thermal-coverage` at all four scales (enclosure-walls 72.9 / 43.5 m², roof 31.7 / 14, discharge-hood 9.3 / 4 at 1:50; totals at 1:200 / 1:500).

Scripts (ticket folder): `r3-diesel-generator-cycle.sh` (build, print suitability, validate, write roots, splice, validate), `r3-diesel-generator-suit.ts` (print suitability per scale of a document), `r3-diesel-generator-kit.py` (one table of real measures in world metres that the 2D drawings and the solids of every scale both read, frame converters per bone), `r3-diesel-generator-roots.py` (rig, anchors, masses, areas, 2D of all scales and views), `r3-diesel-generator-volumes.py` (body, thermal, structure, decisions of all scales, every section stated by hand), `r3-diesel-generator-build.py` (candidate document + volumes source), `r3-diesel-generator-apply.py` (writes the root members into the catalog file). Output under `🗑️generated/r3-diesel-generator/`.

## 1. Reference (real subject)

A 400 kVA prime-power diesel generating set (about 360 kW engine, 320 kW electrical, ISO 8528-1) built into a purpose-made silenced 20 ft ISO container: welded container frame (corner posts, top and bottom side and end rails, eight ISO 1161 corner castings), insulated acoustic wall panels with access doors and inlet louvres with acoustic attenuators behind them, a base frame with the integral fuel base tank, radiator at one end discharging upward into a roof discharge hood with an attenuating plenum and a gravity weather flap, and a horizontal residential-grade exhaust silencer on two saddles on the roof with a vertical stack and a flapper rain cap.

| quantity | real reference | source | asset (round 2) | verdict |
|---|---|---|---|---|
| container length x width x height | 6.058 x 2.438 x 2.591 m (20 ft standard) | ISO 668 (20 ft 1CC); MTU 20 ft container data sheet (6058 x 2438); 400 kVA "super silent" container sheet 6050 x 2438 (machineseeker listing) | 6.06 x 2.44 x 2.59 | ok |
| corner casting | 178 x 162 x 118 mm (L x W x H), oval apertures on the outer faces (about 124 x 63 mm top) | ISO 1161 (hz-containers glossary; Finsterwalder ISO castings) | bottom only, 250 x 200 x 250, no top castings although the plan draws them | wrong |
| corner post | pressed / built-up post about 150 x 150 mm between bottom and top castings, outer faces flush with the castings | ISO 1496-1 container construction; container manufacturers' drawings | absent | missing |
| bottom side rail | pressed channel about 160-200 mm deep between the castings, 12.5 mm above the casting bottom (only the castings touch the ground) | ISO 1496-1 (bottom aperture clearance) | a 6.06 x 1.94 x 0.2 box resting on the ground over its whole length, walls overhanging it by 0.19 m (standing on nothing between the castings) | wrong |
| top side rail / roof | tube rail about 60 x 110 mm; roof sheet below the top of the castings (castings stand proud) | ISO 1496-1 | 0.1 m roof band 2.44 wide flush, plus a 15 mm "roof edge" plate standing ABOVE the castings | wrong |
| walls | flat insulated acoustic panels (2 mm steel skin, 50 mm mineral wool) inset about 30 mm behind the frame | asset `thermal.regions[enclosure-walls].layers`; acoustic container practice | one solid box 2.32 wide | form ok, no frame |
| base fuel tank | integral base tank filling the base frame, 2.5 m3 for a 400 kVA 20 ft set | 400 kVA super silent container sheet (2500 l) | box 1.94 wide | narrow |
| inlet louvres | framed weather louvres, blades sloping DOWN outward (rain), acoustic attenuator behind | louvre practice | blades sloping UP outward | wrong |
| doors | personnel and service doors with three hinges, lockable lever handle, drip rail | container genset practice; asset 1:100 elevation draws two side doors | 1:50 draws one side door only (1:100 two) | 1:50 2D wrong |
| silencer | residential silencer for an 8 in (0.2 m) outlet: cylindrical body about 0.55-0.6 m diameter, about 2.2 m long, 2:1 dished heads, side or bottom inlet, top outlet, on two saddles with straps, raised above the roof | Nett cylindrical residential silencer sheets; Kohler G6-39 silencer data | capsule (hemispherical ends) lying tangent on the roof, no saddles, no inlet | wrong |
| exhaust stack | 0.2 m pipe, short vertical stack above the silencer, flapper rain cap 1.5 x pipe diameter with a skirt, hinged on a bracket | asset (0.2 m, 53 m/s jet), common flapper caps | 0.3 x 0.3 x 0.1 square box cap | wrong form |
| discharge hood | 2 mm sheet plenum over the radiator, sloped back, mouth at the radiator end closed by a gravity flap hinged on top; internal splitters | container genset practice | plates 40 mm thick | thick plates |
| mass | 7.2 t dry (6.5 t container with set) | asset; HSS 400 kVA silenced 6.4 t wet; Himoinsa HDW-400 5.25 t open set | 7200 kg | ok |

Sources: [ISO 1161 glossary (hz-containers)](https://hz-containers.com/en/glossary/iso-1161-standard), [ISO castings (Finsterwalder)](https://www.finsterwalder.eu/en/zubehoer/iso-gussecken), [MTU 20 ft standard container data sheet](https://dtsheet.com/doc/1318335/20--standard-container-mtu-2000-ds), [400 kVA super silent container sheet](https://cdn.machineseeker.com/data/listing/doc/23/17414853.pdf), [HSS 400 kVA silenced generator](https://hss.com/abird-apex/p/400kva-silenced-generator), [Himoinsa HDW-400](https://www.himoinsa.com/electric-generators/generator-set-400kva-diesel-mobile-hdw/generator-sets-for-stationary-installations/hi-range/3-6-6239/eng.html), [Nett cylindrical residential silencer](https://www.nettinc.com/wp-content/uploads/2017/03/Cylindrical-Residential-Grade-Silencer.pdf), [Kohler G6-39 silencers](https://techcomm.kohler.com/techcomm/pdf/g6039.pdf); the asset's own `iso-8528`, `heywood-2018`, `incropera-2011`, `ashrae-2021`.

## 2. Audit (round 2 against the reference)

Rig (`asset.rig.bones`):
- `container` joint at world x 0 (the middle of the container), `lengthM` 6.06 → the bone ran from the middle to 3.03 m beyond the +x end. The structural member diagram (`🏗️structure` `memberDiagram`) drops every station with s < 0, so the supports at x -2.9 (s -2.9) and the whole -x half of the container were outside the member: root defect.
- `silencer` joint at the ground origin (0, 0), length 2.2 along the floor; the silencer lies at y 2.865, x -2.01..0.21: joint 3.1 m away from its part.
- `outlet-hood` joint at the ground origin, length 1.13; the hood stands at x 1.9..3.03, y 2.59: joint 3.4 m away.
- `stack`, `cap-flap`, `outlet-flap` joints were right (stack base, hinge of the cap at the stack rim, hinge of the flap on top of the mouth).
- Masses: container COM (0, 1.0) fine; silencer COM in its old frame (-0.9, 2.86), hood (2.5, 2.9): numbers only valid for joints at the origin.

Anchors: contact supports at x ±2.9, z ±1.13 with 0.3 x 0.3 m footprints, not under the castings (casting centres ±2.941, ±1.139, footprint 0.178 x 0.162).

Dimensions: 6.06 x 2.44 x 3.57 m, 7200 kg: consistent with ISO 668 plus roof silencer and stack; kept.

2D drawings:
- 1:50 elevation had one side door although 1:100 (and the 1:50 decisions "all 1:100 parts plus") drew two; no castings in the 1:50 elevation, the plan drew four corner castings on the roof that the 3D did not have (no top castings).
- A 15 mm `roof-edge` plate drawn above the roof band (2.59..2.605) at 1:50 and 2.585..2.615 at 1:100: a container roof lies below the tops of its castings.
- The silencer was drawn with hemispherical ends (a capsule) lying tangent on the roof, no saddles, no inlet; the rain cap as a 0.3 x 0.1 m block, square in plan.
- 1:500 was one plain block (QA: "cannot be told from a cabinet").

Solids (round 2):
- Body 1:50: container = one wall box 2.32 m wide on a 1.94 m skid box lying on the ground over its whole length; no frame, no corner posts, no top castings, walls overhanging the skid by 0.19 m; louvre blades sloping UP outwards (wrong for rain); door and end door z-fighting with the wall plate (visible noise in the old render); hood plates 40 mm; rain cap a rounded box; silencer a capsule without saddles.
- Thermal: the wall box as skin counted top and bottom (72.9 m² against 43.5), roof band both faces (31.7 against 14), hood 40 mm plates both faces (9.3 against 4); the radiator-air emitter a 1.2 m disc that stuck 0.3 m out of the 0.66 m high hood above and below (D2 violated).
- Structure: the whole wall box as one member (cladding as structure, D3 violated), no frame members, no engine bed, castings as members, one large mass ellipsoid.
- Check: `volumes-thermal-coverage` red at 1:50 / 1:100 per region and at 1:200 / 1:500 in total.

## 3. Root fixes (old → new)

Rig (catalog, all poses and clips evaluated; poses carry only flap angles and a zero root):
- `container`: x 0 → -3.03 (joint at the -x end, the bone spans the container 0..6.06); children re-expressed in its frame: `stack` x -1.2 → 1.83, `cap-flap` x -1.05 → 1.98, `outlet-flap` x 3.03 → 6.06; mass comX 0 → 3.03.
- `silencer`: (0, 0) length 2.2 → (1.02, 2.965) length 2.22 (joint at the -x head of the shell, along its axis; the silencer is raised 0.1 m onto saddles, see below); mass COM (-0.9, 2.86) → (1.11, 0).
- `outlet-hood`: (0, 0) → (4.93, 2.576) (joint at the back foot of the hood on the roof sheet), length 1.13; mass COM (2.5, 2.9) → (0.6, 0.318).

Anchors: `contact-*` moved under the bottom castings (local x 0.089 / 5.971, z ±1.139, footprint 0.3 x 0.3 → 0.178 x 0.162, titles name ISO 1161); `stack-mouth` x → 1.83; `radiator-outlet` x 6.06, y 2.9 → 2.925 (mouth centre); `fuel-cog` x → 3.03.

References added to `provenance.references`: `iso-668`, `iso-1161`, `nett-silencer`.

Region areas (C16, both within factor 1.5, measured on the honest 1:50 model):
- `skid-tank` 27.5 → 32.9 m² (base now 2.44 m wide: tank between bottom rails, end sills, castings; measured 33.07).
- `silencer` 3.87 → 5.0 m² (0.57 m shell with 2:1 heads 4.17 + saddles, bands, bellows inlet, spigot; measured 5.03).
The pose fractions of `skid-tank` (exposed 0.14, contact 0.43) were derived from the old 27.5 composition; with 32.9 the ground share is 14.0 / 32.9 = 0.43 still, the free share about 0.10: left to the area reviewers (C29.2, information only).

2D drawings (E1, all four scales, all three views redrawn from the same measures as the solids, `r3-diesel-generator-roots.py`):
- Elevation 1:50 / 1:100: bottom rail band 13 mm clear of the ground, four corner castings (oval side apertures at 1:50), two corner posts, top side rail, wall field, framed louvres (hatched), both side doors; at 1:50 also hinges, lever handles, drip rail, fuel fill on the rail, lifting eyes; silencer outline with 2:1 heads raised on two saddles with bands, inlet (bellows at 1:50), stack spigot, hinge bracket, domed rain cap with its skirt, hood, flap (and its knuckle at 1:50). `roof-edge` above the castings and `silencer-flange` removed.
- Front 1:50 / 1:100: end sill, castings, posts, end wall field, top end rail, end door (hinges and handle at 1:50), hood with mouth, flap, silencer disc on its saddle, inlet (1:50), stack with spigot and bracket, the rain cap as a revolved solid (true dome front and circle in plan).
- Plan 1:50 / 1:100: roof, inner edge of the frame, four top castings (top apertures at 1:50), silencer with heads and bands, spigot ring, stack, cap, hood with ridge, flap, lifting eyes (1:50).
- 1:200 / 1:500: base band, wall between four corner posts, top band, two dark louvre fields, door outlines (1:200), silencer on two saddles, stack, cap, hood up to the flap (2.96 / 2.93), flap; at 1:500 louvres and saddles carry role `body` (the 1:500 pens have no detail pen).
- Representation decisions `silhouette` rewritten per scale.
- Print suitability (default profile) of all four scales: suitable (min feature 0.35 / 0.25 / 0.28 / 0.21 mm). The revolved rain-cap hint made a 0.18 mm sliver at 1:200 and 1:500, so the coarse scales keep the extruded hint.

## 4. Models per scale and list

Coordinates: world x along the container, y up, z left; the drawn elevation side is -z. Every section is stated by hand in `r3-diesel-generator-volumes.py` from `r3-diesel-generator-kit.py`.

1:50 (feature 7.5 mm) — body 61, thermal 66, structure 43 solids
- Body: eight ISO castings 178 x 118 x 162 (extrudes with the oval aperture), four 160 x 150 corner posts, bottom side rails (12.5 mm clear, 50 mm deep), bottom end sills, the base fuel tank between them, top side rails (60 mm) and deeper top end rails, wall core 30 mm behind the frame, roof sheet 6 mm below the casting tops; four framed louvres (both long sides) with 15 blades each sloping down outwards (texture) and the dark attenuator faces behind them (texture); two side doors and the end door with three hinges each, escutcheons and levers, drip rail; fuel fill on the rail; two lifting eyes. Silencer: revolved shell Ø 0.57 x 2.22 m with 2:1 dished heads (six stations per head), two strap bands, two saddle webs with cradle arcs and base plates, inlet collar and a bellows (four convolutions) into the shell; stack: 6 mm wall tube on a spigot, hinge bracket; rain cap: domed lid, a 3 mm skirt revolved over 300° and turned so its gap sits at the hinge, hinge knuckle; hood: 8 mm plates (pentagon sides, sloped back, top, back) and the flap seat frame, four attenuator splitters inside (visible with the flap open); flap 60 mm with three knuckles.
- Thermal: walls as nested shells per panel (2 mm `skin` on 50 mm `wool`) plus posts, top rails, louvre frames, doors and their hardware as skin; roof = its 2 mm sheet (+ top castings, lifting eyes); louvres = the 60 blades; skid-tank = tank, rails, sills, bottom castings, fuel fill; silencer = shell, bands, saddles, bases, inlet, spigot; stack = its closed outer cylinder + bracket; rain cap = lid, skirt, knuckle; hood = plates; flap = flap + knuckles; emitters: a 7.7 mm patch in the stack bore (flush, D2) and a 1.6 x 0.55 m elliptic patch in the hood mouth in front of the closed flap (D2). Faces looking into the closed container or hood carry `sectionFaces` (stand-in for the C31 flood the product lacks, as delivery-van, mvhr-unit and city-car do; request filed 2026-11-20 by the heat-pump author in `tool-requests.md`).
- Structure: bottom side rails, end sills, nine floor cross members, four posts, top side and end rails (members); the genset skid rails on six anti-vibration mounts (joints); the eight castings (joints); four supports under the bottom castings at their anchors; masses at the COM of container with the set, fuel (fuel-cog), silencer and hood; silencer shell and saddles (members), bands and saddle plates (joints); stack (member) with spigot and bracket (joints); cap lid (member) and knuckle (joint); hood panels (members) and base angle (joint); flap (member) and hinge axis (joint).

1:100 (15 mm) — body 46, thermal 45, structure 39
- Body: plain castings, posts, top rails, sills; bottom rails and tank merged into one base block; louvre frames with eight 50 mm blades; doors as 15 mm leaves without hardware; silencer (four stations per head), 15 mm bands and saddles, plain inlet pipe with collar; solid stack on its spigot, bracket; domed cap with skirt and knuckle; 16 mm hood plates without splitters; flap with three knuckles.
- Thermal: wall core with its top and bottom as the closed interior, frame, doors, louvre frames; roof slab with its underside marked; the rest by reference; 15.5 mm emitter patches.
- Structure: rails inside the base block, sills, five cross members, posts and top rails, castings as joints, skid rails on four mounts, supports, two masses; silencer, saddles, bands, stack, bracket, cap, knuckle, hood panels, base angle, flap, hinge.

1:200 (30 mm) — body 13, thermal 15, structure 27
- Body: base band 0.2 m and top band 0.15 m over the full footprint (castings, rails, roof merged), four 150 mm corner posts standing 30 mm proud of the wall block, four dark 30 mm louvre fields flush with the frame plane, silencer (three stations per head) on two 30 mm saddles, solid stack, domed cap without skirt, hood as one prism up to the 70 mm flap.
- Thermal / structure: own solids or references of this scale; structure 60 mm rails and sills, three cross members, eight castings as joints, two hood side plates and the top plate inside the prism.

1:500 (75 mm) — body 13, thermal 15, structure 27
- Body: dark base band, top band, four 150 mm posts standing 75 mm proud of the wall block, four dark 75 mm louvre panels flush with the frame plane, silencer (two stations per head) on two 80 mm saddles, stack, domed puck cap, hood prism to the 0.1 m flap. Reads as a genset container (QA asked for exactly this: base, castings, darker louvres, silencer and hood on top).
- Thermal / structure: as at 1:200 with 77 mm members and patches.

## 5. Check lines

- `$BUN volumes-check.ts machinery/diesel-generator --product`: **PASS (51.3 s)**, every rule at every scale. Silhouette IoU elevation / front / plan: 1:50 0.997 / 0.997 / 0.999, 1:100 0.998 / 0.998 / 1.000, 1:200 0.998 / 0.999 / 1.000, 1:500 1.000 / 0.999 / 1.000.
- Thermal outer areas 1:50 (declared): walls 44.3 (43.5), roof 14.5 (14), louvres 5.78 (5.8), skid-tank 33.1 (32.9), silencer 5.03 (5.0), stack 0.313 (0.323), rain cap 0.247 (0.287, -14 %), hood 4.44 (4, +11 %), flap 2.42 (2.4). Totals 1:200 / 1:500: +11 % / +13 %.
- `$BUN r3-validate.ts machinery/diesel-generator`: VALID (schema 0, validateAsset 0); `--catalog-only` VALID after the splice.

## 6. What I saw in the final renders

`r3-sheet.ts` (six panels with the rig), `r3-2d.ts` (product drawings):
- 1:50 body, front (from +x): end wall between two posts, top end rail, castings at the four corners, end door with three hinges and a lever handle; hood with the closed flap and three knuckles on top; silencer disc with the saddle and bellows below it, stack with spigot and domed cap. An end-sill repeat error (copy at x 2.67 instead of 2.85, z-fighting noise on the bottom band) was seen here and fixed.
- 1:50 right side (drawn elevation): ISO frame with apertured castings, louvres with blades and dark attenuators, two doors with hinges, handles and drip rail, fuel fill on the rail; silencer raised on two saddles with bands and the bellows inlet; hood pentagon; the rig now runs along the container, the silencer axis and the hood foot.
- 1:50 back and left side: plain end wall in its frame; louvres on the left side too.
- 1:50 top: roof inside the frame, four top castings, lifting eyes, silencer with bands and stack, hood with flap.
- 1:50 three-quarter: reads as a silenced container genset at first sight.
- 1:50 pose `rated-load`: flap swung out 80° on its top hinge exposing the four splitters, rain cap standing vertical on its hinge beside the stack mouth; everything stays closed and attached.
- 1:50 structure: blue ISO frame (rails, sills, nine cross members, posts, top rails), orange castings and mounts, skid rails, silencer on saddles, hood panels, magenta masses; no cladding.
- 1:50 thermal cut: wool shell nested inside the skin, roof sheet, base, louvre blades, silencer, stack, cap, hood; the coplanar faces of tank and rails z-fought in the cut view, the tank was lowered 5 mm from both faces.
- 1:100, 1:500 body sheets: same reading with fewer parts; 1:500 is base band, framed walls, dark louvre panels, silencer on saddles, stack, cap, hood.
- 2D 1:50 / 1:200 / 1:500: drawings match the bodies; 1:50 front and plan show the round domed cap (revolved hint).

## 7. Still imperfect

- `sectionFaces` on inward faces of walls, roof and hood stand in for the C31 flood that the product `outerArea` does not implement (filed in `tool-requests.md` 2026-11-20 by the heat-pump author); they can go once it does.
- The engine, alternator and radiator are not modelled in `thermal`: the asset declares no hidden regions for them, and adding regions would invent physical data (heat flows). They appear only as the mass at the declared COM and the skid rails with mounts in `structure`.
- The rain cap at 1:200 and 1:500 keeps the extruded 2D hint (a 0.3 m square in plan) because the revolved hint gave a 0.18 mm sliver under the print profile; 1:50 and 1:100 are round.
- Louvre frames, attenuator faces and splitters render reddish/dark through the aluminium material colour of the sheet renderer; tones were set lighter, the hue is the renderer's.
- The genset itself (engine, alternator, radiator) is not in `body`: the container is closed and nothing of it is visible except through the open flap (the splitters).
- The `skid-tank` pose fractions (exposed 0.14, contact 0.43) were derived for the old 27.5 m²; with 32.9 m² the free share is about 0.10, left to the area reviewers.
