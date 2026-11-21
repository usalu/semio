# Round 3 — machinery/heat-pump-outdoor-unit

State: DONE (2026-11-20). Scripts: `r3-heat-pump-outdoor-unit-kit.py` (constants of the real unit, every part of every scale stated by hand with its solid and its 2D shapes, root members, candidate build; `--apply` writes the catalog root members, the manifest entry by a scoped text edit and the volumes source), `r3-heat-pump-outdoor-unit-texts.json` (bilingual texts and decisions), `r3-heat-pump-outdoor-unit-run.sh` (build, validate candidate, apply, splice, validate), `r3-heat-pump-outdoor-unit-diag.ts` (product silhouette masks of one view as a PNG), `r3-heat-pump-outdoor-unit-manifest.ts` (product `validateManifest` for this entry). Resume: `./r3-heat-pump-outdoor-unit-run.sh`.

## 1. Reference

Reference class: 10 to 12 kW single-fan air-to-water monobloc outdoor units with a front discharge. S1 = Vaillant aroTHERM plus VWL 105/6 A and VWL 125/6 A technical data (casing width 1,100 mm, depth 450 mm; 194 kg ready for operation; fan air flow up to about 5,100 m3/h), read 2026-11-19 through the Vaillant installer pages and the aroTHERM plus technical sheet (professional.vaillant.co.uk, product 146624; oceanairuk.com aroTHERM plus tech sheet). S2 = the asset's own class data (`class-data-note`, EN 14511 test points): 1.10 x 0.45 x 1.45 m, 150 kg, 5,500 m3/h. K = my knowledge of the construction of this machine class (not a published number).

| quantity | real value | source |
|---|---|---|
| casing width (lateral) x depth | 1.10 x 0.43 to 0.45 m; with the top cover overhang 0.47 m | S1, S2, K |
| height incl. feet and cover | 1.45 m | S2 |
| mass | 150 kg (class), 194 kg for the larger Vaillant | S2, S1 |
| air flow / heating duty | 5,500 m3/h (1.53 m3/s), 12 kW at A7/W35 | S2 |
| air path | air drawn through an L-shaped finned coil on the back and one side, blown out horizontally through the front by one axial fan; the coil is behind a louvred or wire guard | K |
| fan | one axial fan, blade diameter about 0.56 m, 3 sickle-swept blades on a hub with an external-rotor motor, motor on a vertical bracket behind the fan; flared inlet ring (bell mouth) about 0.58 m throat, 10 mm tip gap | K (5,500 m3/h at about 6 to 7 m/s through 0.22 m2) |
| fan guard | domed grille of concentric rings and radial spokes, about 0.62 m, proud of the front panel by 15 to 20 mm | K |
| coil | 2 tube rows, fin pack about 40 mm deep, about 1.2 m high; back leg the width of the fan chamber, side leg about 0.3 m deep, bend radius about 70 mm | K |
| compartments | fan chamber (about 0.8 m wide) beside a closed compressor / hydraulics / electronics column (about 0.3 m wide) separated by a sheet partition; compressor (scroll or rotary, about 0.4 m high) on the base pan in the column | K |
| casing | galvanised, powder-coated steel sheet 1.0 to 1.2 mm: front panels (fan section, column section), service side panel with grip, column back panel with the water flow and return connections (G 1 1/4, about 42 mm), rear corner post at the coil bend, top panel under a weather cover | K |
| base | galvanised base pan about 60 mm deep (condensate tray, defrost heating), standing on two anti-vibration rubber feet about 0.34 x 0.18 x 0.09 m | K, asset anchors |
| top cover | sheet cover with 20 mm overhang front and back and side skirts, here hinged at the rear edge for service access | K, asset rig |

## 2. Audit of the asset as found

| item | found | real | deviation |
|---|---|---|---|
| discharge vane (rig bone `vane`, region `discharge-vane`, clip `heating-sweep`) | a 0.245 x 0.05 x 0.72 m flap hinged at the fan centre, standing 0.25 m proud of the front and swinging +-25 degrees across the fan | outdoor units of the class blow straight out through a fixed grille; no swinging vane exists (qa-visual-2: "reads as a tray or shelf") | invented assembly; removed at the root |
| `dimensions.widthM` (depth along x) | 0.72 m (casing 0.45 plus the vane) | 0.45 m casing, 0.47 m with the cover overhang (S1, S2) | +0.25 m |
| air path | coil windows in BOTH side walls, back a closed sheet | L coil on the back and one side behind guards, closed side on the compressor column | wrong: no air could reach the coil from the back, the compressor side was open |
| compressor | a 0.17 m full-width band at the bottom of the front | a 0.4 m compressor needs a column; class units put it in a closed column beside the fan chamber | a band 0.17 m high cannot hold it |
| fan | centre z +0.12 (unit's left), opening 0.66 m, five pitched blades of a 0.36 m disc, motor on a full-height stay across the opening | fan in the fan chamber beside the column, 0.56 m three-blade sickle fan in a bell mouth, motor on a bracket behind | blade diameter too small for the opening; layout moved with the column |
| fan guard | flat rings in front of a cylinder sleeve, 0.26 m to the front | domed grille 15 to 20 mm proud | +0.025 m proud, flat |
| cover | a 0.04 m box hinged on two cylinders that reached 0.02 m above and behind it | sheet cover with skirts, hinge knuckles under the rear overhang | form and hinge |
| feet | 0.34 x 0.09 x 0.18 boxes | chamfered anti-vibration rubber blocks | detail |
| walls | 15 to 20 mm slabs, coil a solid 0.22 m block | 1.0 to 1.2 mm sheet, 40 mm fin pack | wall model |
| structure | references to every body slab plus one central ellipsoid ("solid copy of the box", qa-visual-2 C) | corner posts, top frame, base pan, partition, motor bracket, compressor mass | D3 defect |
| thermal | each region one body reference; emitters as discs 27 mm in front of the guard | patches on the grille and the coil (D2) | emitters floating |
| 2D (all scales, all views) | follow the vane, the centre fan and the double coil windows | | redrawn (E1) |
| rig `lid` | joint (-0.235, 1.45) at the top surface, mass com (0.23, 0) | joint at the hinge axis under the rear overhang (-0.235, 1.41) | moved with the hinge |
| areas | casing 6.8 (box of all faces), fan-guard 0.77, compressor-casing 1.5 | see section 5 | |

## 3. Root fixes (old -> new)

- `rig`: bone `vane` removed. `unit` mass 141 -> 143 kg (com (0, 0.70) -> (0, 0.55): compressor, base and coil low), title rewritten. `lid` joint (-0.235, 1.45) -> (-0.235, 1.41) (hinge axis under the 20 mm rear overhang, so the knuckles no longer stand above the cover; checked open at 80 degrees: the side skirts clear the casing), com (0.23, 0) -> (0.235, 0.0175), title "top cover".
- `dimensions`: widthM 0.72 -> 0.47 (S1 450 mm casing class, cover overhang); height 1.45, depth 1.10, mass 150 kept (S2).
- `anchors`: `fan-outlet` (0.26, 0.8, 0.12) -> (0.235, 0.80, -0.15) on the grille face; `coil-top` (-0.05, 1.3, 0) -> (-0.2125, 1.34, -0.125) on the upper rear coil face (D2). Contacts kept.
- `poses`: `vane` offsets and the `discharge-vane` region states removed from all five poses; heating and cooling descriptions rewritten.
- `clips`: `heating-sweep` -> `heating-modulation` (compressor and fan modulating 60-80-60 percent, phases `load-rise`, `load-fall`); `presentation.defaultClip` follows; `defrost-sequence` kept (it opens the cover).
- `thermal`: region `discharge-vane` removed; emitter `diameterM` 0.5 -> 0.56 (the fan); `areaM2` see section 5.
- `summary`, `provenance` (reference `vaillant-arotherm-plus` added, notes rewritten), `symbols.heat-pump-symbol` (vane removed, coil guard and column added, texts), manifest entry (`widthM`, `clips`, `distinction`; product `validateManifest`: 0 issues for the entry).
- `representations` 50/100/200/500: all three views redrawn from the parts (section 4), decisions rewritten, hatch `fins` spacing 0.4 -> 1.0 mm (one line per louvre, 50 / 100 mm).

## 4. 2D drawings (ruling E1)

Every shape is the true orthographic view of the part it stands for, stated next to the part in the kit: side elevation from -z shows foot, base pan, casing, rear corner post, the louvred coil guard (hatch one line per louvre), the grille dome, the cover profile with its side skirt (an extruded `solid`, so front and plan of the cover are generated and follow the hinge), water connections and hinge bracket at 1:50. Front: chamfered feet, base pan, fan-chamber and column panels, the grille disc, three blade outlines computed from the blade lofts, rings, spokes, cap. Plan: casing under the cover. Detail shapes inside the grille carry no region so that the line views keep their outlines (same-region fills hide inner outlines). Elevation/front/plan shape counts 9/18/2, 8/14/2, 5/8/2, 3/5/2. The `serviced` pose was drawn (r3-2d `--pose serviced`): the cover stands open about its rear hinge in side, front and plan.

## 5. Areas (C16, C29, C31)

- `fan-guard` 0.77 -> 1.15 (measured 1.149: everything in free air; factor 1.49).
- `casing` 6.8 -> 4.6 (factor 1.48, the most C16 allows; measured 4.39 / 4.34 at 1:50 / 1:100; by C31 the honest value is about 3.8, proposal in `area-review.md`).
- `compressor-casing` 1.5 kept (C31 outer faces 1.4). The product measures 2.69 because `outerArea` does not implement C31 (faces inside the closed column count): `tool-requests.md` 2026-11-20. This is the only failing measurement.
- base-tray 1.2 (measured 1.12), top-lid 1.2 (1.24), feet 0.43 (0.42) unchanged.

## 6. Models per scale

| scale | body / thermal / structure | what is modelled |
|---|---|---|
| 1:50 | 36 / 39 / 22 | chamfered rubber feet, base pan; casing of 7.5 mm sheets (the scale's feature): fan-chamber front panel with the 0.60 m opening, column front panel, right front strip, folded rear corner post, top panel, service side panel with grip, column back with two 42 mm water connections, partition; L fin pack 40 mm (70 mm bend) behind 25 louvres (50 mm pitch, 40 degrees) and a rear wire guard of 15 + 13 wires (texture); bell mouth (flared, 7.5 mm wall), three sickle blades (one loft, 8 sections, pitch 36 -> 21 degrees, repeated), spinner hub, external-rotor motor, motor plate between two bracket bars; domed grille: rolled rim, five rings, eight spokes (slender, true 5 mm), cap; cover plate, U skirt, hinge knuckles and leaves on the lid, brackets and knuckles on the casing. Thermal: references per region, two outlet patches in the grille dome, a defrost patch on the rear coil face. Structure: four corner posts (folded angles), four top rails, base pan, feet, partition, motor bracket bars and plate, cover plate and skirt, hinge knuckles on both bones as joints, compressor (0.38 m dome on the pan) and the two bone masses, two 7.5 mm pads. |
| 1:100 | 27 / 30 / 17 | 15 mm sheets; 12 louvres; no wire guard, grip, motor plate or casing knuckles; motor and hub one turned body; bell mouth simplified; three blades lofted through every second section; grille rim, two rings, four spokes, cap. Structure: three posts and the body post, top frame ring, base pan, feet, partition, bracket bars, cover members, cover knuckles as joint, masses, pads. |
| 1:200 | 11 / 14 / 15 | hollow casing of 30 mm walls (C30: the cavity is far above the feature, so it stays hollow, which also keeps the union volume in the cross-scale band): front panel with the opening, column shell, side strip, top, partition, L coil slab 40 mm, dark grille disc with rim filling the opening and the hub, feet, base pan, cover slab. Structure: four 30 mm posts, top frame, base pan, feet, partition, cover with a 30 mm hinge pin, masses, pads. |
| 1:500 | 8 / 11 / 13 | 75 mm walls: base pan slab, front panel with the opening filled by the dark fan disc 20 mm proud, column shell, coil L slab, top, feet, cover slab. Structure: four 75 mm posts between base and top frames, feet, cover with a 75 mm hinge pin, masses, pads. |

Check (`$BUN volumes-check.ts machinery/heat-pump-outdoor-unit --product`, 2026-11-20): every rule ok at every scale (silhouette IoU elevation/front/plan 1:50 0.994/0.999/0.995, 1:100 0.996/0.999/0.998, 1:200 elevation 0.989, 1:500 elevation 0.982; bounds, feature, monotonic, cross-scale, structure, slender, copied, views ok) except `volumes-thermal-coverage` 1:50 and 1:100 on `compressor-casing` only (0.79 / 0.81, product without C31, see section 5). `$BUN r3-validate.ts machinery/heat-pump-outdoor-unit`: VALID (schema 0, validateAsset 0).

## 7. What the final renders show

- 1:50 front: a modern single-fan monobloc: domed grille of rings and spokes over three sickle blades and the hub, the motor bracket bars visible behind, the compressor column panel with its seam on the right of the view, chamfered rubber feet under a galvanised pan, the cover with 20 mm overhang. No knuckles above the cover any more.
- 1:50 right side (product elevation): 25 inclined louvres between the front strip and the folded rear post, the grille dome proud at the front, the water connection behind, cover skirt.
- 1:50 back: the welded wire guard over the coil on the fan side, the closed column back with two connections, hinge brackets under the cover overhang.
- 1:50 left side: plain service panel with grip; three-quarter: reads at first sight as a heat pump outdoor unit. Serviced pose: cover open 80 degrees about the rear hinge, no collision.
- Thermal cut: regions in place (casing panels, column, coil L with louvres, bell mouth and fan, base pan, feet, cover). Structure: a frame of posts, rails, pan, partition and bracket with the compressor mass, cover on its hinge.
- 1:100 the same with 12 louvres and a two-ring grille; 1:200 and 1:500 a box with dark coil faces, a dark fan disc (with hub at 1:200), feet and cover.

## 8. Still imperfect

- `compressor-casing` area check red at 1:50 / 1:100 until the product implements C31 (tool request); the declared 1.5 m2 is the honest value.
- Sheet panels are 7.5 / 15 mm thick at 1:50 / 1:100 instead of 1.2 mm: the feature rule judges the overall extents, and a flat panel with a hole cannot be a thin-walled hollow solid; folded parts (rear post, cover skirt, bell mouth, column shell) keep their thin-walled form.
- The back of the coil at 1:100 shows the fin face without the wire guard (4 mm wires are below the scale). Fin texture and screws are not modelled.
- The 1:500 front panel shows a shading wedge from the cap triangulation around the round opening in `r3-sheet` (renderer, already reported for delivery-van).

## Round 5 handcraft (2026-11-21)

Scripts: `r5-heat-pump-outdoor-unit-kit.py` (imports the round 3 kit, replaces 1:200 and 1:500, areas, 1:200 hatch and texts, the 1:500 band; `--apply` as round 3), `r5-m03-render.sh` (all sheets of one asset), `r5-m03-capprobe.ts` (corner normals of a cap, used to rule out a mesh defect behind the shading wedges).

### Reference (manufacturer data, checked 2026-11-21)

| quantity | Vaillant aroTHERM plus VWL 105/6 and 125/6 (installer quick guide, product sheet) | Mitsubishi Ecodan PUZ-WM112 class (K) | asset class value (modelled) | model 1:50 | deviation from class value |
|---|---|---|---|---|---|
| height incl. feet | 1,565 mm | about 1,340 mm | 1,450 mm (S2) | 1,450 mm | 0 |
| width (lateral) | 1,100 mm | 1,050 mm | 1,100 mm | 1,100 mm casing, 1,115 mm cover | 0 / +1.4 % (cover overhang) |
| depth | 450 mm (449 in the German planning guide) | 480 mm | 450 mm casing, 470 mm with cover | 430 casing + 20 mm grille dome, 470 mm cover | -0.4 % / 0 |
| fan | one axial fan (single-fan 10/12 kW class) | one | 0.56 m blade, 0.60 m opening | 0.552 m blade tip circle, 0.60 m opening | -1.4 % |
| mass | 194 kg | about 130 kg | 150 kg | 143 + 7 kg bones | 0 |

The 1.45 m class height lies between the two manufacturer units (1.34 and 1.565 m); `dimensions` are not changed without a cited single reference, so the class value stays.

### Grades before -> after (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- (unchanged model; thermal now honest C31 references) |
| 1:100 | A- / A- / B+ | A- / A- / B+ |
| 1:200 | B (side coil a flat dark slab, cover sunk 20 mm into the casing with a ragged seam) / B+ / B+ | A- (13 louvre bars at 90 mm in a frame over the recessed fin pack, cover rests on the casing) / A- / B+ |
| 1:500 | B (cover sunk into the casing) / B+ / B | B+ / B+ / B |
| poses | heating, cooling, defrosting, off, serviced (cover open 80 degrees about the rear hinge, no collision) and `defrost-sequence` t = 15 s opened: closed and natural | unchanged |

Opened: r3-sheet body, structure, thermal --cut at 1:50, 1:100 (body), 1:200 (body, thermal, before and after), 1:500 (body); all five poses and the clip frame at 1:50; r3-2d at 1:50, 1:200 (candidate), 1:500.

### Changes

- 1:200: casing (front panel, column shell, side strip, top panel, partition, posts, top frame) ends at y = 1.405 under the cover instead of 1.425 inside it; the fin pack moves 30 mm behind the side face and 13 louvre bars (30 mm, 90 mm pitch) with a top and bottom frame bar form the guard; 2D side elevation gets a `louvres` hatch (0.45 mm) and the decisions say so.
- 1:500: casing top at 1.405 under the cover slab.
- Thermal: the round-4 stand-in `th-compressor-envelope` (box with `sectionFaces` on faces looking into the closed column, forbidden by C31 g) is gone; the column panel, column back and service panel are plain references again. With the flood the closed column no longer counts: `casing` 4.6 -> 3.75 (measured 3.736), `base-tray` 1.2 -> 1.0 (0.992), both within C16, listed in `area-review.md`.
- `representations.500.validRange.maxDenominator` 1000 -> 900 (feet and cover print at 0.2 mm there; 1:950 and 1:800 measured below).

Check: `r3-validate.ts machinery/heat-pump-outdoor-unit --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0).

### Remaining

- The front panel of 1:50 to 1:500 shows soft triangular shading wedges in `r3-sheet` at some zooms; the corner normals of the cap are exactly (1, 0, 0) (`r5-m03-capprobe.ts`), so it is the sheet rasteriser, not the mesh (already filed for delivery-van and sorption-wheel).
- The cover-to-casing contact of 1:200 still draws a faint grainy seam line (coplanar contact seen edge-on).
- Sheet panels stay at the scale feature (7.5 / 15 mm) instead of 1.2 mm (round 3 reason).
