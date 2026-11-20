# Round 3 — components/curtain-wall-bay (2026-11-20)

Status: DONE. The roots and volumes are spliced. The catalog file is VALID (`r3-validate --volumes --catalog-only`: schema 0, validateAsset 0, checkVolumes 0), and `volumes-check --product` passes.

## 0. Starting state

This was a fresh start: no earlier round-3 scripts or report existed for this asset.

- At the start, the catalog `volumes` equalled `T/volumes/components/curtain-wall-bay.json`.
- `r3-validate` said VALID.
- `volumes-check --product` failed on three areas:
  - glass-edge at 1:50 (+18 %);
  - glass-edge at 1:100 (+86 %);
  - spandrel at 1:100 (+51 %).
- The copies of the starting files are in `🗑️generated/r3-curtain-wall-bay/` (`catalog-before.json`, `volumes-before.json`).

Scripts in T, all re-runnable:

| script | what it does |
|---|---|
| `r3-curtain-wall-bay-kit.py` | real dimensions, member profiles, section properties, solid writers |
| `r3-curtain-wall-bay-volumes.py` | every solid of every scale, and the masses |
| `r3-curtain-wall-bay-roots.py` | root fixes and all 2D drawings; `--apply` writes the catalog; `--area=` |
| `r3-curtain-wall-bay-layout.py` | member splice in the catalog layout |
| `r3-curtain-wall-bay-statics.ts` | reactions and residual of every pose |
| `r3-curtain-wall-bay-suit.ts` | print suitability per scale |
| `r3-curtain-wall-bay-meshcheck.ts` | mesh volume of every extrude against the profile area |
| `r3-curtain-wall-bay-crop.ts` | enlarged crops of renders |

## 1. Reference (the real subject)

The subject is one bay of a stick-system (mullion-transom) curtain wall with 50 mm face width:

- mullions and transoms are thermally broken aluminium tubes on the room side;
- the infills sit in front of them, in glazing rebates;
- the infills are clamped by a screwed pressure plate behind a snap-on cover cap, with EPDM gaskets on both faces of the glass;
- a polyamide isolator sits in the 20 mm glass joint.

| item | real value | source |
|---|---|---|
| face width of mullion, transom, pressure plate and cover cap | 50 mm | Schüco FWS 50 product data (schueco.com) |
| mullion basic depth | 50–250 mm; this bay uses 150 mm | Schüco FWS 50 |
| infill thickness | 4–62 mm; this bay uses 28 mm (6-16-6) | Schüco FWS 50; EN ISO 10077-1 |
| mullion depth needed | I ≥ 2.0e-6 m⁴: the 3.3 m span under 1.08 kPa on a 1.35 m bay, limit L/200 | hand calculation, EN 13830 |
| modelled mullion 50 x 150 | 3 mm walls, screw channel; A = 1266 mm², I = 359 cm⁴, about 9 mm deflection (L/370) | kit `section()` |
| transom | 50 x 100 tube, about 4.5 kg/m with its cover parts | profile catalogues |
| glass bite and joint | 15 mm bite, 20 mm joint between glass edges (50 − 2 x 15) | DIN 18545 and glass setting practice |
| setting blocks | 5 mm thick, under the full unit thickness, on glass carriers near the quarter points | DIN 18545 and glass setting guideline |
| IGU edge seal | warm-edge spacer, butyl and secondary seal, 12 mm deep from the glass edge (hidden in the rebate) | EN 1279-1 |
| cover cap and pressure plate | flat snap-on cap 50 x 12 with 2 mm wall; plate 46 x 10 with gasket grooves; screws every 250 mm | system practice (FWS 50 / THERM+ 50 class) |
| spandrel | 6 mm enamelled glass in the glazing plane; 20 mm air; 100 mm mineral wool in a folded 1.5 mm galvanised steel back pan | provenance `profile-catalogues`; EN ISO 12631 |
| supports | each mullion hangs from a slab bracket (dead load and wind) and is guided at its foot by a spigot that takes wind and slides vertically | stick-system practice |
| thermal split | glass area = visible glass, with a 65 mm edge zone (psi_g); the frame zone holds the glass edges that sit in the rebate | EN ISO 10077-1, EN ISO 12631 |
| conductivities | EPDM 0.25, PA 6.6 GF25 0.30, edge seal 0.35 W/mK | EN ISO 10077-2 Annex |

## 2. Audit (before)

| part | found | real / required |
|---|---|---|
| depth (`dimensions.depthM`) | 0.15 m: a "50 x 150" mullion with the glass, the caps and the 150 mm all inside 150 mm | 0.20 m: cap 12, gasket 4, glass 28, gasket 6, mullion 150 |
| members | each mullion and transom was three stacked boxes (35/30/85 mm): no tube, no isolator, no pressure plate, no cap, no gaskets | real hollow profiles with screw channel, isolator, plate, cap and gaskets |
| glass | the panes stopped at the sight line. A 65 mm "edge" pane ring and four 12 x 16 spacer bars were drawn visibly 65 mm inside the glass edge | 1.33 x 2.38 m unit, 15 mm bite, edge seal hidden in the rebate |
| spandrel | enamel glass and 1.5 mm pan as free plates; no packer; the glass not clamped | 6 mm glass with 22 mm packer in the rebate; folded back pan holding the wool |
| neighbouring bays | outer rebates empty | infills of the neighbours cut at the sample edge |
| supports | pin at (0.70, 3.30) in the middle of the head transom, and a base roller in-plane only. **The wind-suction pose was unbalanced**: residual Fz −3689 N, Mx −3852 N·m (measured with `r3-curtain-wall-bay-statics.ts`) | a bracket pin at the head of each mullion; a lateral spigot at the foot of each mullion |
| frame section | "50 x 150" with A 0.00135, I 1.4e-6 (a 95 mm-deep tube would give that, and it fails L/200) | A 0.0012661, I 3.589e-6, c 0.081 (computed from the profile) |
| masses | frame 57.85, IGU 95.65, spandrel 33.07; total 186.57 kg | 61.62, 96.66, 41.56; total 199.84 kg (computed per part) |
| 2D 1:50 | the elevation drew the thermal zone line and a "shadow-box" rectangle, which no one sees on a real wall. The sections showed boxes of 35/30/85 mm. Depth 0.15 at all scales | true views of the new body (section 3) |
| 2D 1:100 | elevation shapes offset by 0.5 mm in x; a reflection stroke inside a 28 mm cut glass strip | corrected |
| thermal 1:50 | edge ring +18 % (inner and outer edge faces counted); frame `cap` = a box face of 50 mm only, ignoring the cap sides | zones as cuts in continuous glass; caps with their real sides |
| structure | full-depth boxes; glass as one block; supports 8 mm thin and invisible (QA round 1) | tubes, plates, screws, connectors, setting blocks, brackets, spigots, masses |

## 3. Root fixes (catalog, old → new)

**Dimensions and rig**
- `dimensions`:
  - depthM 0.15 → 0.20
  - massKg 186.572 → 199.84
- Rig `frame`:
  - mass 57.85 → 61.62 kg, comY 1.65 → 1.559, source authored-estimate
  - section areaM2 0.00135 → 0.0012661
  - section secondMomentM4 1.4e-6 → 3.589e-6
  - section extremeFibreM 0.075 → 0.081
  - section title → "Mullion 50 x 150 mm, aluminium tube, 3 mm walls, with screw channel"
- `igu` mass 95.65 → 96.66.
- `spandrel` mass 33.072 → 41.56.
- Cradle title updated: the hook now sits on the mid transom cap.

**Anchors and supports**
- Anchors `head-anchor` (pin at 0.70 / 3.30) and `base-guide` (roller, normal x) are replaced by four anchors:
  - `head-anchor-left` and `head-anchor-right`: pins at x 0.025 / 1.375, y 3.275, z 0.06 (slab bracket bolts through the mullion tops);
  - `base-guide-left` and `base-guide-right`: rollers with `tiltDeg` 90 at the mullion feet (spigots, restrain across the wall).
- `structure.supports` lists the four anchors.
- Load anchors now carry their depth:
  - `igu-centre` z −0.07;
  - `spandrel-centre` z −0.0244;
  - `cradle-hook` z −0.094.
- Statics after the fix: every pose has residual 0. Wind suction gives head 1261 N + base 1234 N per mullion.

**Loads, materials, references, texts**
- Seismic loads follow the masses:
  - `seismic-igu` fxN 112.6 → 113.79;
  - `seismic-spandrel` 38.93 → 48.92.
- Materials added: `epdm`, `polyamide`, `edge-seal` (EN ISO 10077-2 values).
- References added: `schueco-fws-50`, `en-1279`, `din-18545`. The `profile-catalogues` citation is rewritten.
- The summary, provenance notes, dead-load description and structural assumption are rewritten in en and de.

**Thermal**
- The `break` layer of mullion and transom: thicknessM 0.03 → 0.038 (the rebate zone from the plate to the tube face). The titles are corrected. R is unchanged.
- **mullion areaM2 0.33 → 0.477.**
- **transom areaM2 0.195 → 0.285.**
- Why the frame areas change:
  - The old values were the 50 mm cap faces only.
  - The honest outer area (C29, reviewers' rule R-C: "outer face plus exposed edges") includes the two 12 mm cap sides.
  - The factors are 1.45 and 1.46, inside the C16 author limit of 1.5. Both are measured on the 1:50 model.
- Consequence for the area reviewers: R and U_f were declared per projected area. With the larger area, the frame regions lose about 45 % more heat in the solver, unless the reviewers rescale the layer resistances. I did not touch physical data beyond the area.
- The glass, edge and spandrel areas are unchanged and now match exactly.
- Thermal assumption 2 is rewritten (EN ISO 12631 frame zone).

**2D, all four scales, redrawn as the true views of the new body (E1)**
- **Elevation, 1:50 to 1:200:**
  - removed the thermal zone rectangle `spacer` and the `spandrel-box`; the glass carries `regions` [centre, edge];
  - the 0.5 mm offset at 1:100 is gone;
  - 1:500 is unchanged.
- **Front A-A, 1:50:**
  - the mullion side beyond the cut, from −0.10 to +0.10, with the cap and face lines;
  - each transom as its real section outline (rounded cap corners), with the isolator light, a light cap line and a light chamber outline;
  - pane lines at the true depths;
  - edge seals 12 x 16 in the rebates;
  - enamel line, wool and pan.
- **Plan B-B, 1:50:** both mullions drawn the same way; the mid transom below (−0.10..0.05) with cap and face lines; panes and edge seals.
- **1:100:** section outlines, a 28 mm glass strip, a spandrel strip from the glass to the pan.
- **1:200:** bars of the true depth; the glass is drawn 32 mm so that it prints.
- **1:500:**
  - front: one strip from −0.10 to 0.10;
  - plan: the vision layer with the full-depth mullions and an 80 mm printable step.
- The decisions texts of all scales are rewritten in en and de.
- Print suitability (`r3-curtain-wall-bay-suit.ts`): all four scales `authored`. The 1:50 drawing has min feature 0.42 mm and no gap violations.

## 4. Models per scale

| scale | body (solids/expanded) | thermal | structure | what |
|---|---|---|---|---|
| 1:50 | 44 / 54 | 23 / 25 | 30 / 71 | See the 1:50 description below the table. |
| 1:100 | 7 / 8 | 7 / 8 | 19 / 25 | Body: each member one extruded bar of its true section outline (cap corners rounded); unit 28 mm to the sight lines; spandrel glass zone and back-pan block. Thermal: glass centre and ring as 28 mm pieces; spandrel `of`; frame as 15 mm rounded cap zones. Structure: 3 mm tubes (C30), plates, setting blocks, connectors, hook, pins, spigots, three masses. |
| 1:200 | 4 / 5 | 6 / 8 | 11 / 13 | Body: one extruded 150 mm grid with two openings, mullion backs to 200 mm, 32 mm glass slab, 131 mm spandrel block. Thermal: zones and 30 mm cap strips. Structure: solid bars, plates, pins, spigots, masses of glass and spandrel. |
| 1:500 | 3 / 4 | 6 / 8 | 9 / 9 | Body: full-depth mullion bars at 75 mm; 75 mm vision layer; 120 mm spandrel block. Thermal: 75 mm zone pieces ("Minimal:"). Structure: mullion bars, mid transom, vision plate, spandrel, pins, spigots, IGU mass. |

What the 1:50 model contains:
- **Members:**
  - mullions: 50 x 150 tubes with gasket lips and a screw channel;
  - transoms: 50 x 100 tubes;
  - on each member: 14 x 30 polyamide isolator, 46 x 10 pressure plate with gasket grooves, 50 x 12 snap-on cap (2 mm wall), two outer and two inner EPDM gaskets, each with its foot in its groove.
- **Glass and spandrel:**
  - 6-16-6 unit, 1.33 x 2.38 m, in a 15 mm rebate, with the 12 mm edge seal ring;
  - 6 mm enamelled spandrel glass with a 22 mm packer;
  - back pan as flange ring and back sheet (`texture`, 1.5 mm), with 100 mm wool.
- **Neighbour stubs:** the edges of the neighbouring infills, cut at the boundary of the sample.
- **Thermal (1:50):**
  - glass: centre and 65 mm ring, each as nested pane / cavity / pane shells; the zone boundaries in the continuous glass are section faces;
  - spandrel: enamel, air, wool and pan shells;
  - each member: a 38 mm break shell and the cap shell (cap `of` plus the plate). The face against the break is `+w` (C25 b); the mullion ends are `±v` (the wall continues).
- **Structure (1:50):**
  - members: tubes (`of`), pressure plates (`of`), the panes and the spandrel glass and pan as clamped plates;
  - joints: plate screws every 250 mm (Ø8), six transom connectors, setting blocks on the transoms, the cradle hook on the mid transom cap;
  - supports: bracket pins at the mullion heads, spigots in the mullion feet;
  - masses: frame, IGU, spandrel.

Final check line: `$BUN volumes-check.ts components/curtain-wall-bay --product` → **PASS (5.8 s)**.

Silhouette IoU (elevation / front / plan):
- 1:50: 1.000 / 0.9998 / 0.936;
- 1:100, 1:200 and 1:500: 1.000.

Areas at 1:50 (deviation from `areaM2`):

| region | deviation |
|---|---|
| glass-centre | 0.0 % |
| glass-edge | 0.0 % |
| spandrel | 0.0 % |
| mullion | +0.04 % |
| transom | +0.1 % |

At 1:100 every region is within +8.3 %. The totals at 1:200 and 1:500 are +8.3 % and +4.9 %.

The mesh check (`r3-curtain-wall-bay-meshcheck.ts`) finds the mesh volume of every extrude equal to its profile area times its length.

## 5. What I saw (final renders, opened with Read)

**1:50 body, six views (`r3-sheet`)**
- Elevation, from −z: the 50 mm cap grid over the vision glass and the dark spandrel, with no false inner rectangles.
- Front and back: the 200 mm depth in profile, with the stepped cap and glass line at the exterior.
- Interior, from +z: the mullion tubes stand 150 mm into the room, the transoms 100 mm. The back pan of the spandrel is recessed between them; the renderer tints steel green.
- Top, enlarged crop of the mullion end: reads like a manufacturer's section.
  - tube with its chamber, gasket lips and screw channel;
  - the isolator in the glass joint;
  - inner and outer gaskets;
  - pressure plate and cap;
  - the neighbour's glass stub in the outer rebate and this bay's glass in the inner rebate.
- Three-quarter, turn 60, enlarged corner: the head transom cap butts the mullion cap; the gasket lips, the neighbour stub and the open tube end at the cut show.
- Three-quarter from the interior, turn 215: reads as a stick facade seen from the room.

**1:50 thermal**
- Cut sheet: red glass centre, green 65 mm ring, olive mullion break, cyan transom break, purple spandrel layers.
- Top view, enlarged: the break shell under the cap shell for mullions and transoms, and the wool behind.

**1:50 structure**
- Panes and spandrel read as blue clamped plates; tubes and caps frame them.
- Cut front view: the plate screws every 250 mm along the mullion, transom connectors (orange), the three masses, and the mullion member over its full depth.
- Clip `cradle-pass` at t 6 s: the hook block sits on the mid transom cap at about x 1.0 m.

**1:50 2D (`r3-2d`, enlarged crops)**
- Elevation: clean cap grid and reflections.
- Sections: dark profiles with light isolator, cap and chamber lines; pane lines into the rebates; edge seals.
- 1:100, pose `cradle-at-mid`, forces view: four supports at the mullion heads and feet, cradle load, residual 0.

**1:100, 1:200, 1:500 (`volumes-render`)**
- Every overlay is green. The 1:500 plan step reads.
- Bodies: generalised but right in proportion (mullions deeper than transoms, spandrel recessed from the mullion backs).

## 6. Still imperfect (stated plainly)

**2D at 1:50**
- The plan section B-B has IoU 0.936.
  - The 3D mullion tube is hollow, so the section shows its 46 x 140 mm chamber open.
  - The 2D draws the cut aluminium profile solid dark, with the chamber as a light line.
  - Drawing the bore as a hole (C23) would leave 3 mm walls, 0.06 mm on paper, which does not print.
  - It is a drafting convention, within the limits.

**Thermal and area**
- The thermal frame areas changed by a factor of about 1.45 (section 3). This affects the frame heat flow in the solver. The reviewers own `resistanceM2KW` and U_f per area.
- The `break` shells are an idealised 50 x 38 mm frame zone per EN ISO 12631, which contains the glass edges.
  - They are not the isolator shape. The isolator stays in the body.
  - The room-side tubes are not a thermal layer: they are on the conditioned side, as in the component method.

**Structure model**
- The structural rig is still one rigid `frame` bone, horizontal at the base, with the mullion section.
  - The solver's member diagrams are free-body cuts of a tree, so they cannot show mullion bending between two supports.
  - A per-mullion rig would only give misleading linear diagrams; I left it.
  - The support reactions are now correct, and in-plane loads are shared by the solver's equal-stiffness assumption.
- The base spigots restrain across the wall only (lateral roller).
  - A real spigot also fits in-plane; the head pins carry the in-plane loads.
  - Two rollers per foot would be needed to model that.
- The cradle "hook on the mid transom" is kept, because it is the asset's load case. Real building-maintenance cradles are restrained by mullion buttons and do not hang from transoms.

**Body detail**
- Setting blocks (5 mm), screws, glass carriers and transom connectors are below the 7.5 mm feature or hidden. They are in structure only.
- The back-pan back sheet is a `texture` solid (1.5 mm), so it is excluded from silhouettes and from the feature rule.

**Renderers (tool side)**
- `r3-sheet` and `volumes-render` show thermal regions with merged layers (1:100 to 1:500) in grey or hatch. The legend has no plain region key. Filed in `T/tool-requests.md` (2026-11-20).
- `r3-sheet` draws a tiny white sliver at one corner of the pressure plate in the top view. The mesh volume is exact (mesh check), so this is a rasteriser artefact.
