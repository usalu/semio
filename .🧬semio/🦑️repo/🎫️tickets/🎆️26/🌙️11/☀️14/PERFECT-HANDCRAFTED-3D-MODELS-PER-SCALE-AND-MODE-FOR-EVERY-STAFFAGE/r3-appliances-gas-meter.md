# Round 3: appliances/gas-meter

State on 2026-11-20: finished and spliced, with one failing check line that I did not make green on purpose (housing `areaM2`, ruling C16, see §6). No earlier round-3 work existed for this asset (only the round-1 script `author-12-gas-meter.py`). At the start `r3-validate.ts` was VALID, the catalog `volumes` equalled the source, and `volumes-check.ts --product` FAILED twice (views disagree 8.25 mm at 1:50; shell area 17.6 % at 1:50).

Final state:
- `r3-validate.ts appliances/gas-meter`: `VALID: schema 0, validateAsset 0`. With `--catalog-only --volumes`: `INVALID ... checkVolumes 1` (the one line below).
- `volumes-check.ts appliances/gas-meter --product`: every rule ok at 1:50, 1:100, 1:200 except `volumes-thermal-coverage 1:50 areaDeviation:shell=0.295<=0.15` (0.141 m2 modelled against 0.20 declared). 1:500 stays unavailable, as its representation.
- Catalog `volumes` equal the source `volumes/appliances/gas-meter.json` (spliced with `volumes-insert.py`).

Scripts in the ticket folder (inputs, kept):
- `r3-gas-meter-kit.py`: the measures of the meter and the solid writers.
- `r3-gas-meter-model.py` + `r3-gas-meter-texts.json`: every solid of every scale and list, and the volume decisions (en, de).
- `r3-gas-meter-roots.py` + `r3-gas-meter-roots-texts.json`: the root fixes and the 2D drawings of all scales and views. Builds and validates a candidate; `--apply` writes the changed members (with `volumes`, in one write, because the new support solids need the new anchors) and the manifest entry. It can be re-run.
- `r3-gas-meter-zoom.ts`: enlarges the 2D panels that `r3-2d.ts --svg` writes (the meter is 4 mm on paper).

## 1. Reference

| Quantity | Real subject | Source | Model |
|---|---|---|---|
| Class | Domestic G4 diaphragm meter, two-pipe, top in and top out, 0.04 to 6 m3/h, 0.5 bar | EN 1359 (`din-en-1359-2017`); Elster data sheets | as below |
| Connection spacing | 110 mm (compact meter); the 2 litre BK-G4 has 152.4, 220 or 250 mm | Elster BK-G4 compact data sheet "BK-G4 DS Rev. 0" (web, 2026-11-20; new reference `elster-bk-g4-compact`); Elster BK-G4/G4T data sheet 2012 | 110 mm |
| Width, height, depth | 193 x 221 x 157 mm at 110 mm (aluminium screwed model at 130 mm: 202 x 215 x 165 mm; 2 litre model at 250 mm: 327 x 241 x 163 mm) | same | 193 wide over the seam, 221 over the bare stubs, 157 over the index |
| Mass | 1.9 kg (steel), 2.15 kg (aluminium); 2 litre model 3.0 to 3.5 kg | same | 1.952 kg unchanged (1.9 plus two unions) |
| Cyclic volume | 1.2 litres (compact), 2 litres (large) | same | text corrected to 1.2 |
| Housing | steel, "folded model": two deep-drawn shells joined by a folded seam with a visible band; powder coated pale grey RAL 7035; rounded shoulders; aluminium only as the screwed model | same, product photo of the 2012 sheet | two lofted shells with 2.5 mm draft, 18 mm shoulders, superelliptic plan (n 5.5), seam band 9 mm high and 5 mm proud |
| Connection thread | M30 x 2 (110 mm model) | same | stub Ø30, bore Ø22, on a Ø36 boss |
| Union | hexagon nut over the stub, flat-sealing tailpiece of the pipework | my knowledge of meter unions | nut 36 mm across flats, 17 mm high; tailpiece Ø26/20 with Ø30 collar, cut 20 mm above the nut |
| Index | plastic head on the front of the upper shell, roller counter with eight digits, test element | EN 1359; photo | head 130 x 66 mm, 28 mm proud, 7.5 mm frame; counter 104 x 18 mm with 11 x 14 mm digits; test dial Ø36 with pointer (the asset's concept, kept) |
| Inside | measuring unit with four chambers behind two diaphragms, slide valves, crank gear, drive to the index, outlet channel to the outlet stub; the inlet opens into the housing | Elster operating principle text | hidden region `measuring-unit` |
| Mounting | hung by its two unions on a meter bar, clear of the wall | installation practice | supports at the unions |

## 2. Audit of the state before round 3

- **Dimensions**: 0.24 x 0.27 x 0.17 m with 110 mm stubs and 1.95 kg: width +24 %, housing height 220 against about 195 mm, depth +8 %. Stub spacing over width was 0.46 (real 0.57): the stubs sat too close together on a box of the 2 litre size.
- **Material and text**: "two die-cast aluminium shells with a bead seam" mixes the two real constructions (folded steel or screwed aluminium); reference text claimed 2 litres at 110 mm.
- **Rig**: `pointer` bone at angle 0 (along x) while its needle was drawn along local y: bone perpendicular to the part it carries. Its length 0.025 m and dial Ø56 mm belonged to an index head 190 x 98 mm that does not fit a compact meter. `body` length 0.27 m along x (longer than the body is wide).
- **Anchors and statics**: two "wall brackets" on the mid-plane (pin above, roller below, both normal along x). A G4 meter has no wall brackets; with both supports on one vertical line and horizontal normals the weight had no vertical bearing in the drawn plane. Thermal poses declared 20 % wall contact of the housing.
- **2D**: boxes with 12 mm corner radii; stub 28 mm with the nut in the middle of the stub and a "sealing face" plate on top (no real part); elevation 8.25 mm taller than the front view at 1:50; 1:200 index enlarged by 20 mm.
- **Solids** (`qa-visual.md`: body A/B/B, thermal B/B/B, structure C/C/C): two rounded boxes, a flat seam extrude, round "unions"; thermal without internals; structure an open-ended box with supports hovering inside.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions` | 0.27 high, 0.24 wide, 0.17 deep | 0.245 (221 + nut and 20 mm tailpiece), 0.193, 0.157; mass unchanged |
| manifest `🗂️catalog/🔣️.json` | heightM 0.27, widthM 0.24, distinction text | 0.245, 0.193, new text |
| `summary`, `provenance.notes` | aluminium box 0.24 x 0.27 | compact steel meter, construction, mounting, 1.2 litres |
| `provenance.references` | 2 | + `elster-bk-g4-compact` (with url); EN 1359 citation no longer claims spacing and volume |
| `materials` | `die-cast-aluminium` | `sheet-steel` (7850 kg/m3, 50 W/mK, 480 J/kgK, emissivity 0.9); + `polyester-gf` (measuring unit) |
| `rig.body` | lengthM 0.27, comY 0 | 0.0965, comY −0.02 (unit in the lower shell) |
| `rig.pointer` | y 0.018, angle 0, length 0.025 | y 0.027, angle 90 (along the needle, up at rest), length 0.0155; pose and clip offsets unchanged (they add to the rest angle) |
| `anchors` | `wall-bracket-top` (0, 0.1), `wall-bracket-bottom` (0, −0.1) | `union-inlet` pin (−0.055, 0.1215), `union-outlet` roller (0.055, 0.1215), normals up |
| `structure` | supports and text of the brackets | the two unions; assumption rewritten (2 x 9.6 N) |
| poses (3), clips (2) | contacts at the brackets; shell exposed 0.8 with contact 0.2 | contacts at the unions; shell exposed 1, no contact; + `measuring-unit` |
| `thermal.regions.shell` | aluminium 2 mm, areaM2 0.30, length 0.27, 2400 J/m2K | steel 1 mm, areaM2 0.20 (C16 limit, honest 0.141), 0.195, 3770 J/m2K, R 2e-5 |
| `thermal.regions.index` | areaM2 0.05, length 0.19 | 0.034, 0.13 |
| `thermal.regions.pipes` | areaM2 0.02, tube 2.6 mm | 0.014; stub wall 4 mm, 15300 J/m2K, R 8e-5 |
| `thermal.regions` | 3 | + hidden passive `measuring-unit` 0.147 m2, one layer |
| `thermal.assumptions` | 1 | + free hanging, hidden unit |
| `representations.50/100/200` | see §2 | every view redrawn as the orthographic view of that scale's body (E1), same ids where the part exists; `face-*` removed (no such part), new `tail-*`, `union-*-edge-*`, `index-window`, plan `flow-arrow`; tones for pale grey powder coat; all eight decision texts per scale rewritten (the duplicated sentences are gone) |
| `representations.500` | 0.48 x 0.54 mm, needle 12 mm | 0.39 x 0.49 mm, pointer 5 mm |

## 4. Volumes per scale (own solids / expanded)

**1:50: body 19/31, thermal 18/21, structure 13/13**
- Body: `lower-shell`, `upper-shell` (one loft each, seam to crown), `seam` (rounded band loft), `index-base` + `index-frame` (rounded prism, frame with window hole), `stubs` x2 (revolved tube on boss, open bore), `unions` x2 (hexagon with thread hole), `tails` x2 (collar and pipe end, open bore). Texture sheets: counter, 5 + 3 digit drums, dial, 4 ticks, needle and pivot (pointer bone), data plate with two lines, flow arrow on the crown.
- Thermal: shell = both shells, seam, plate; index = base, frame, needle; pipes = closed stubs, nuts, closed tailpieces with the cut as section face; hidden unit = frame, two domed diaphragm pans, valve deck, two slide covers, gear tower with crank, drive shaft on the pointer axis, outlet duct to the outlet stub.
- Structure: stubs (members), union nuts (joints), both shells as open sheet cups (lofts without the seam cap), seam ring, outlet duct tube, unit frame, valve deck, pointer spindle and bearing, two tailpiece supports at the anchors, mass at (0, −0.02, 0).

**1:100: body 8/10, thermal 9/11, structure 11/12** — one housing loft at bead width with three-step shoulders, index block, counter, dial, needle, plate sheets, plain Ø30 pins, round Ø40 union drums; thermal with frame, two pan blocks, deck, gear, duct; structure with stub tubes, round joints, two cups, 15 mm seam ring, unit slab, duct, spindle, pin supports, mass.

**1:200: body 6/7, thermal 4/4, structure 8/9** — rounded block, index block, counter bar, Ø40 dial, bolder needle, two Ø40 pins; thermal block, index, pins, one unit core; structure two hanger tubes, two chamfered cups, unit slab, spindle, two supports, mass.

Silhouette IoU elevation / front / plan: 1:50 0.998 / 0.996 / 0.997, 1:100 0.997 / 1.000 / 0.997, 1:200 1.000 / 1.000 / 0.996; bounds within 0.2 mm. Outer areas 1:50 (declared): shell 0.141 (0.20), index 0.032 (0.034), pipes 0.0140 (0.014), measuring unit 0.147 (0.147); 1:200 total 27 %.

## 5. What I saw in the renders (all opened with the Read tool; outputs deleted afterwards)

- Before: `r3-sheet` 1:50 body and `r3-2d` 1:50 of the old asset: a two-box case with flat seam strip, round unions, an index nearly as wide as the case, bone lines perpendicular to the needle.
- 2D candidate and final, 1:50, enlarged (line and solid; forces and thermal): elevation reads as a compact meter: drafted shells with shoulders, proud seam, stubs on bosses under hexagon nuts with edge lines, tailpieces, index with frame, eight digits, dial with ticks and pointer at 4 o'clock (rated flow), plate with two lines. Front: index head standing out to the left, one stub. Plan: bead outline, hexagon nuts with dark bores, flow arrow, index strip. Forces: two reactions upward at the unions and the weight. The line view does not draw counter and dial outlines (only frame and ticks), as before. The product's ground line runs through the seam (origin on the seam, as in the sibling electricity meter). 1:100 and 1:200 2D I judged through the silhouette numbers and the 3D sheets only, not enlarged.
- `r3-sheet` 1:50 body, six panels: front (+x) shows the side with the index head and the seam band; elevation the face; back and left closed; top the crown with arrow, hexagon nuts and bores; three-quarter reads as a real meter at first sight. Pose `off` (elevation only): pointer straight up along its bone. Clip `run` at 0.7 s rendered, not opened.
- 1:50 thermal cut: unit frame, both pans as domes, valve deck with covers, gear tower with crank, shaft into the index, duct under the outlet stub. 1:50 structure: stubs, orange nuts, green tailpieces, blue cups with seam ring, spindle with bearing.
- 1:100 body and thermal cut, 1:200 body and structure: generalised as described, recognisable. 1:100 structure and 1:200 thermal were rendered but not opened.

## 6. Still imperfect (stated plainly)

- **Check not green**: `volumes-thermal-coverage` fails on `shell` at 1:50. The honest housing area of the corrected meter is 0.141 m2; the old 0.30 was the box of the wrong dimensions. C16 forbids a correction beyond factor 1.5, so 0.20 is applied; the hand calculation is in `area-review.md`, the request in `author-requests.md`. Applying 0.14 makes the check pass.
- **Structure looks like the housing**: a pressed meter case is a monocoque, so the load path is the sheet itself. The cups hide the unit frame, the duct and the mass in every outside view; a reviewer may still read it as "the body in blue".
- The cups are zero-thickness loft skins (the 1 mm sheet cannot be hollowed by a loft); the body shells are solid lofts.
- The clear cover of the index is not modelled (it would hide counter and dial in opaque renderers).
- The index back plane sinks 0.7 to 4 mm into the crowned shell front; the data plate is a 3 mm sheet, 1.5 to 2 mm proud.
- Revolves (dial, stubs) show 12- to 16-gon facets (kernel segment count); the six-step shoulders shade with small dark wedges at the corners in `r3-sheet`.
- The test dial with a pointer and "0.2 m3 per turn" is the asset's own concept; the Elster index uses the last drum as test element. Kept, because rig, poses and clips are built on it.
- The rig has no depth for the centre of mass; it lies on the stub axis plane by assumption.
- Not run: the staffage test suites and the exports; the energy project uses the asset through the clip `run` (`shared-gas-meter-run`), whose id is unchanged, but its fitted picture is now a smaller, differently proportioned meter.
- During the work a helper wrote one PNG into a wrongly named folder in the repository root (percent-encoded path); I moved the file and removed the empty folders within the same minute.

Sources: [Elster BK-G4 compact data sheet](https://add-furnace.com/product-catalog/elster-thailand/bk-g4-gas-meter-manual.pdf), [Elster BK-G4 and BK-G4T data sheet 2012](https://www.dmsltd.com/upload/pdfs/4ba45f9e966741a7aa14d19d472e0cb6.pdf), [same sheet at sensors.nl](https://www.sensors.nl/wp-content/uploads/2022/11/BK-G4.pdf).
