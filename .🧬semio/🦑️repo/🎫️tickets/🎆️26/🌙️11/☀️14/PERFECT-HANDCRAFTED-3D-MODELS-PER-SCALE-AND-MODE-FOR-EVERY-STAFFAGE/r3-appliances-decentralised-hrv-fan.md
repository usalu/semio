# Round 3: appliances/decentralised-hrv-fan

State: finished and spliced on 2026-11-20. No earlier round-3 work existed for this asset (no scripts, no report, catalog `volumes` equal to the ticket source), so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-decentralised-hrv-fan-kit.py`: the measures, the blade sections (pitched, twisted, symmetric lens lofts with a buried root boss and a rounded tip), the honeycomb cells, strut and plan-form outlines. The blade and quaternion helpers are copied from `r3-duct-fan-kit.py`.
- `r3-decentralised-hrv-fan-volumes.py`: every solid of every scale and list, stated by hand. Writes `volumes/appliances/decentralised-hrv-fan.json`.
- `r3-decentralised-hrv-fan-roots.py`: the root fixes and the drawings of 1:50, 1:100 and 1:200. Writes the candidate `🗑️generated/r3-decentralised-hrv-fan/🔣️.json`. It can be run again on the fixed catalog and gives the same result.
- `r3-decentralised-hrv-fan-apply.py`: writes the candidate's root members into the catalog in the file's own formatting (copy of the duct-fan apply script).
- `r3-decentralised-hrv-fan-ship.sh`: runs volumes, roots, `r3-validate.ts --file --volumes`, apply, `volumes-insert.py` and the catalog validation in this order. To resume: `sh r3-decentralised-hrv-fan-ship.sh`.

## 1. Reference

| Quantity | Real value | Source | Model |
|---|---|---|---|
| Class | Decentralised push-pull unit: reversible axial fan and ceramic regenerator in a round wall sleeve, removable inner cover with filter, outer hood | DIN EN 13141-8 (asset reference); Helios KWL EcoVent Verso description (wall sleeve, EPP fan casing, grille, flow straighteners, ceramic accumulator) | Same assemblies, shown with inner cover, filter and outer hood taken off |
| Sleeve and core | Lunos e2 60: duct 160 mm, plug-in module 154 mm diameter by 243 mm (short 160 mm). Ambientika: ceramic 170 mm by 150 mm with hexagonal cells in a 200 mm pipe | Lunos TechInfo E348 02.20; Ambientika technical sheet (web search 2026-11-20) | Sleeve 196 mm outside, 3 mm wall; ceramic 160 mm by 100 mm. This is the 200 mm pipe class, not "DN160" as the old notes said |
| Wall thickness | Blauberg Vento Expert A50: minimum wall 240 mm; Lunos short version for walls from 200 mm; Maico PP 45: 320 to 790 mm | Blauberg data sheet V.2; Lunos; Maico PP 45 technical data (search summary, page itself returned 403) | Sleeve 0.22 m behind the 20 mm tray (`depthM` 0.24 kept): the short end of the real range |
| Core holder | Ceramic in an EPP foam housing | Lunos brochure; Helios | Black EPP jacket 189 mm outside, 160 mm bore, 45 degree funnel to the fan, retaining lip at the outer end |
| Fan | Reversible EC axial fan, 5 to 8.5 W over three speeds, cycle about 70 s | Blauberg data sheets, DIBt Z-51.3-399 | 172 mm impeller, 2 mm tip clearance in a 176 mm throat, 4 W, 70 s (unchanged data) |
| Blades of a reversible fan | Symmetric, uncambered sections so that both directions work alike; pitch falling from root to tip | Fan design practice (my knowledge) | 5 radial, unswept blades, lens sections 3 to 2 mm, chord 34 to 56 mm, pitch 36 to 24 degrees |
| Motor | External rotor: the can is the hub and carries the blades; the stator sits on a boss held by struts | Product knowledge | 56 mm steel can, 30 mm long, domed front, 20 mm cap; 40 mm stator boss reaching into the can; 3 struts 8 x 12 mm |
| Inner cover base | Moulded plastic base screwed to the wall with 4 screws, terminal block inside, cover clipped over it | Product knowledge (installation manuals of this class) | ABS tray 260 x 260 x 20 mm, corner radius 20 mm, 2.5 mm rim, 4 bosses of 14 mm with 8 mm screw heads at (+-0.108, +-0.108), terminal box 60 x 20 x 10 mm |
| Ceramic cells | About 3 mm channels | Product knowledge | Generalised as the drawing does: 37 channels of 18 mm with 3.5 mm walls (1:50), 19 of 25 mm (1:100), plain disc (1:200) |
| Mass | Ceramic honeycomb at about 800 kg/m3 bulk: 0.002 m3 gives 1.6 kg; PVC or ABS sleeve 196 x 3 x 220 mm about 0.45 to 0.55 kg | Hand calculation | 2.65 kg kept; split corrected (section 3) |

Sources: [Lunos e2 60 TechInfo](https://cdn.shopify.com/s/files/1/2174/7939/files/TechInfo_e260_E348-02.20.pdf), [Ambientika 200 mm sheet](https://ambientika.eu/cms/wp-content/uploads/AMBIENTIKA-200mm.it_.en_.pdf), [Blauberg Vento Expert A50-1 W V.3 data sheet](https://blaubergventilatoren.net/download/vento-expert-a50-1-w-v3-datasheet-8769.pdf), [Blauberg V.2 data sheet](https://blauberg-na.com/download/vento-expert-a50-1-w-v2-bl-na-datasheet-14914.pdf), [DIBt Z-51.3-399](https://dibt.de/pdf_storage/2018/Z-51.3-399(1.51.3-70!16).pdf), [Helios KWL EcoVent Verso](https://exhibitorsearch.messefrankfurt.com/images/original/mics/10000016202401/0010823780/1659081633387_KWL_EC_45_EN_0320.pdf), [Maico PP 45 technical data](https://www.maico-ventilatoren.com/cms-live/media/AnleitungenHTML5/PP-Bedienung/en-GB/152821771.html). The manufacturer figures come from the search summaries; I could not open the Maico page itself (403).

## 2. Audit before round 3

- **Body (the worst defect).** The plate was a solid 20 mm slab without a bore, at the room end. Seen from the room (the elevation) the 3D unit was a blank square; fan and core were only visible from the back, through the wall. The four screw heads sat on the wall side of the plate. The blades were flat extruded plan-forms without pitch. The hub was a plain cylinder floating without any motor mount. The "core frame" was a second ceramic disc of 176 mm round the core. The sleeve was a 10 mm thick tube.
- **Rig and anchors.** Both support anchors sat at (0, +-0.108): the middle of the top and bottom edge, while the drawing has the four screws in the corners. Mass split 0.45 kg plate, 0.5 kg sleeve: the sleeve alone weighs about that, the tray about 0.2 kg. Bone lengths 0.2 (ring) and 0.08 (rotor) matched no part.
- **2D.** Front and plan carried a "wall line" at w = 0.02 and a "cover joint" at w = -0.085: the wall face is the back of the plate (w = -0.10), a wall line 120 mm behind it puts the plate in mid-air; the sleeve has no joint there. The hub cap was drawn off centre (+3 mm, +3 mm). Blades were swept plan-forms reaching r 0.078 in a 0.088 opening (10 mm tip gap). No motor mount was drawn. At 1:100 the 19 cells of 44 mm reached r 0.095, outside the 0.08 disc and the 0.086 opening. At 1:200 front and plan were one T polygon on the plate bone.
- **Thermal.** Hub and blades were buried in the core region's solids; the emitter was one open loft in the middle of the core for both directions. `hub` area failed at 1:50 (20 %) and 1:100 (44 %), `blades` at 1:100 (31 %): the check was red before round 3.
- **Structure.** Rotor member a 16 mm bar, no load path from the rotor to the housing, supports at the wrong anchors, one support lying inside the plate.
- **Texts.** Notes said "DN160 class" for a 196 mm sleeve with a 160 mm core, and "2.7 kg" against `massKg` 2.65.

## 3. Root fixes (catalog outside `volumes`)

| Member | Old | New |
|---|---|---|
| `materials` | 3 | + `epp-foam` (black EPP, 60 kg/m3, 0.039 W/mK, emissivity 0.92; authored estimate, ISO 10456) for the jacket |
| `anchors.wall-screw-top` | (0, 0.108, z -0.11), "Upper screw" | (-0.108, 0.108, z -0.10), "Upper left screw" (pin) |
| `anchors.wall-screw-bottom` | (0, -0.108, z -0.11), "Lower screw" | (0.108, -0.108, z -0.10), "Lower right screw" (roller) |
| `anchors.outlet.title` | Centre of the sleeve opening | Axis of the ceramic core |
| `rig.bones[].mass.kg` | plate 0.45, ring 0.5, core 1.6, rotor 0.1 | plate 0.25, ring 0.7, core 1.6, rotor 0.1 (sum 2.65 unchanged) |
| `rig.bones[].lengthM` | ring 0.2, rotor 0.08 | ring 0.196 (sleeve diameter), rotor 0.086 (tip radius) |
| `rig` titles | Inner cover plate; Wall sleeve with fan housing; Reversible rotor with blades | Base tray of the inner cover; Wall sleeve with fan housing, stator and EPP jacket; Reversible external rotor with blades |
| `thermal.regions.hub.areaM2` | 0.0075 | 0.01: the measured 1:50 can with dome and cap is 0.0096, factor 1.33 (within C16) |
| `thermal.regions.casing.title` | Cover plate and wall sleeve | Base tray, fan housing and wall sleeve |
| `thermal.regions.casing.areaM2`, `blades.areaM2` | 0.39, 0.028 | unchanged (measured 0.423 and 0.0292) |
| `structure.assumptions` | hangs on upper screw, leans on lower | two of the four corner screws (upper left pin, lower right roller); says plainly that in the building the bedded sleeve carries core and fan, the idealisation is a loose sleeve |
| `provenance.references` | 4 | + `lunos-e2-60`, `ambientika-200`, `blauberg-vento-expert-a50`, `helios-ecovent-verso` |
| `provenance.notes`, `summary` | DN160 class, 2.7 kg | 200 mm pipe class, construction, frame (rim at w -0.12, wall face -0.10, outer end 0.12), mass split with its calculation |
| `representations.50/100/200.geometry.elevation` | see audit | redrawn as the true view (E1), below |
| `representations.50.front/plan` | plate, ring, rim, cover-joint, wall-line | plate, ring, rim (the two untrue lines removed) |
| `representations.200.front/plan` | one T polygon on bone plate | plate (20 mm) and ring as two rects on their bones |
| `representations.500` | "mounting screws of 14 mm" | "screw bosses of 14 mm" (reason and limiting feature) |
| `representations.*.decisions` | | rewritten (en, de) |

The elevation redraw in detail (ids, bones, roles, layer order kept; new ids `terminal`, `strut-1..3`):
- **Plate, screws (now the 14 mm bosses), ring 196 mm, core disc and 1:50 cells:** unchanged, they were right.
- **Opening:** radius 0.088 at 1:50 and 1:100 (was 0.086 at 1:100), now the black funnel of the EPP jacket (`material` epp-foam); at 1:200 radius 0.098, the end of the merged tube (was 0.094).
- **Cells at 1:100:** 19 hexagons of 30 mm that stay inside the disc.
- **Struts:** three filled bars of 8 mm (1:50) and 15 mm (1:100) at 90, 210 and 330 degrees; one upright bridge bar of 30 mm at 1:200.
- **Blades:** the true plan-form of each scale's blades (root, tip arc), radial instead of swept.
- **Hub and cap:** 56 / 60 / 80 mm; the cap centred, 20 mm.
- **Terminal box:** a detail rect at 1:50.

Unchanged: `dimensions` (0.26 x 0.26 x 0.24 m, 2.65 kg: inside the cited range), poses, clips, emitters, environment, the other region data.

## 4. What was modelled (solids per list; expanded counts after repeats in brackets)

Frame: axis w, room at -w.

### 1:50: body 19 (27), thermal 19, structure 18

- **Body:** tray floor with the bore (7.5 mm, see section 7); tray rim 2.5 mm x 17.5 mm with rounded corners; 4 screw bosses of 14 mm and 4 domed 8 mm screw heads (repeats); terminal box with rounded edges; fan housing as one revolve (196 mm collar standing in the tray, 6 mm inlet radius, 176 mm throat, 189 mm barrel plugged 55 mm into the sleeve); sleeve 196 / 190 mm and outer collar 208 mm x 15 mm as 72-sided tubes; EPP jacket with funnel and lip; ceramic block with 37 hexagonal channels right through; 3 struts with rounded edges; stator boss; 5 blades lofted through 10 lens sections (buried root boss, twist, two shortening sections that round the tip along the 172 mm circle); the can with its domed front; the cap.
- **Thermal:** references for every casing solid, the honeycomb, blades, can and cap in their single layers; `th-supply-in` and `th-exhaust-out` as 2 mm patches on the room face and the outer face of the ceramic (D2). The jacket and the steel screw heads belong to no region and are absent.
- **Structure:** tray floor, sleeve, fan housing, struts, stator boss (references: they are the members), the jacket, the ceramic as a plain cylinder, the can on an 8 mm steel shaft; joints at the tray bore, the core seat and the bearing; 4 masses at their centres; pin = screw from its head into the wall plug, roller = 8 mm cylinder on the wall face in the tray.

### 1:100: body 11 (13), thermal 12, structure 13

- **Body:** one 20 mm plate with the bore; one tube that merges fan housing, sleeve and a 20 mm outer collar; EPP liner without funnel or lip; ceramic with 19 channels; 3 struts of 15 mm meeting behind the hub; 5 blades with the same pitch law, sections 5 to 4 mm, 15 mm root boss; plain 60 mm hub with a chamfer.
- **Thermal:** references and the two outlet patches of 3 mm.
- **Structure:** plate, tube, struts, plain ceramic cylinder, hub on a 15 mm shaft, bearing joint, 4 masses, pin through the plate into the wall, roller half sunk in the plate face.

### 1:200: body 8, thermal 10, structure 8

- **Body:** plate with bore; one thick tube 196 / 164 mm; ceramic as a plain disc; one upright bridge bar of 30 mm behind the hub (three 30 mm struts read as three more blades, I saw that in the first render and merged them); 3 radial blade plan-forms through the 30 mm fan stage; 80 mm hub.
- **Thermal:** references and two 4 mm patches. **Structure:** plate, tube, bar, disc, hub; one mass of the whole unit; pin and roller of 30 mm.

1:500 stays unavailable. Each authored scale has three bilingual decision sentences.

## 5. Checks

- `$BUN volumes-check.ts appliances/decentralised-hrv-fan --product`: **PASS** (7.0 s). Before round 3 it was FAIL (3 thermal area measurements).
- `$BUN r3-validate.ts appliances/decentralised-hrv-fan --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Silhouette IoU (elevation / front / plan), p95 0 mm everywhere:

  | Scale | Elevation | Front | Plan |
  |---|---|---|---|
  | 1:50 | 0.948 | 1.000 | 1.000 |
  | 1:100 | 0.952 | 1.000 | 1.000 |
  | 1:200 | 0.998 | 1.000 | 1.000 |

  The elevation is below 1 at 1:50 and 1:100 because the channels of the honeycomb are open right through in 3D, while the drawing fills the ceramic disc with a tone and draws the cell walls. That is the only difference (red hexagons between the blades in the overlay).
- Thermal outer areas at 1:50: casing 0.423 against 0.39 (8.5 %), blades 0.0292 against 0.028 (4 %), hub 0.0096 against 0.01 (4 %). 1:100: 1 %, 15 %, 23 %. 1:200 total 6 %.

## 6. What I saw in the final renders

Opened with the Read tool: `r3-sheet` 1:50 body (default pose and clip `reverse-spin` at 1.3 s), 1:50 body elevation alone and three-quarter alone, 1:50 thermal cut, 1:50 structure, 1:100 body, 1:100 structure (sheet and three-quarter), 1:100 thermal cut front, 1:200 body; `volumes-render` sheets of 1:50, 1:100 and 1:200 (overlays, thermal and structure panels); the 2D elevations of all three scales enlarged from the product SVG.

- **1:50 from the room (elevation):** reads at once as an opened wall ventilator: tray with rim, four bosses with screw heads, terminal box, collar with a soft inlet radius, five paddle blades with visible twist on a domed hub with cap, one strut straight up and the honeycomb behind. Two struts are 12 degrees from a blade and mostly hidden in the default pose (with 5 blades and 3 struts there is no position where all are free); they show in the clip frame.
- **1:50 from the side, top, back:** T shape of tray and sleeve with the collar at the outer end; from outside one looks into the sleeve onto the jacket lip and the honeycomb, with fan and struts visible through the channels.
- **1:50 three-quarter:** the tray is a real shell, the fan sits recessed in the housing, nothing floats.
- **Clip frame:** rotor and rig turn together, the blades stay rooted in the hub, the housing stays closed.
- **1:50 thermal cut:** casing, core with its channels, blades, hub and stator boss are all visible in the cut; the fan sits in its own stage in front of the core (the round-2 complaint that hub and blades were swallowed is gone).
- **1:50 structure:** plate, sleeve, housing, three struts and hub with the pin in the upper left corner; the masses of ring and core sit inside the tube and are only seen in a cut.
- **1:100:** same object with a plain plate and merged tube; 19 channels; supports visible after I moved them (the first version had both buried in the plate).
- **1:200:** plate, thick tube, three broad blades, hub, the bar behind; the back shows the plain ceramic disc.
- **2D:** the elevation of every scale agrees with the model; the hub cap is centred; at 1:200 the bar shows only as two short stubs at top and bottom because its tone (0.6) is close to the ceramic (0.62).

## 7. Still imperfect

- **Tray floor 7.5 mm instead of 2.5 mm.** `volumes-feature` refused the real 2.5 mm floor (C30 is not seen for a shell made of two solids). The floor is modelled 7.5 mm (shell, ribs and foam gasket) and the decisions say so. Request with the measurement is in `author-requests.md`.
- **Polygonal rounds.** The product quality gives this small asset about 16 to 22 sides for a full turn: the collar of the fan housing, the throat and the hub are visibly polygonal in every render (the hub a 10-gon). Sleeve and outer collar are round because I wrote them as 72-sided extrudes; a revolve cannot be given segments. Listed in `tool-requests.md` together with the uneven shading of flat plate faces in `r3-sheet.ts` (the plate looks pillowed at 1:100 and 1:200; the geometry is flat).
- **Honeycomb is a generalisation.** 18 mm channels stand for 3 mm cells, as in the drawing. A true 3 mm honeycomb is below the feature of every scale.
- **Supports are an idealisation.** The real load path is the sleeve bedded in the wall. The two support anchors must be two different points in the elevation, so the unit hangs on two corner screws; the structure assumption says so.
- **Not modelled:** cable, filter, clip-on cover, outer hood (the asset shows the unit with them taken off), the hollow inside of the fan housing and the motor windings.
- **Structure looks close to the body** from outside at every scale, because for this object the casing parts are the members; rim, bosses, screws, terminal box, blades, cap, outer collar (1:50) and the channels are absent from it.
- **Not run:** `repo` test targets and the model export; only the ticket tools above.
