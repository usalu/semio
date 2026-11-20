# Round 3 — furniture/office-swivel-chair

State: finished and spliced (2026-11-19). The asset was rebuilt by a first round-3 author (cut off after writing the catalog file, before the report) and reviewed, corrected and completed by a second one. Everything below describes the catalog file as it is now.

Scripts (all in the ticket folder): `r3-office-swivel-chair-kit.py` (rig, world to bone-local bookkeeping), `-body.py` (every solid of every scale stated by hand), `-lists.py` (thermal, structure, volume decisions), `-roots.py` (references, notes, region areas, representation decisions), `-outline.ts` + `-draw.py` (2D shapes as true views of the bodies plus hand-placed strokes), `-build.py` (candidate document), `-run.sh` (build, project, redraw, validate), `-apply.py` (writes the validated candidate into the catalog file, formatting kept).

## 1. Reference

Subject: a typical European task chair of type A/B after EN 1335-1 (plastic five-star base on twin-wheel castors, gas spring, synchro mechanism, upholstered seat on a plastic pan, height-adjustable T-armrests, upholstered backrest on a plastic shell carried by a bent flat-steel strut).

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Seat height (top of cushion, unloaded) | 400-510 mm adjustable range; products typically 420-550 mm | EN 1335-1:2020; manufacturer sheets from memory | 445 mm low, 545 mm high (100 mm stroke) |
| Seat depth | 400-420 mm minimum, products 420-470 mm | EN 1335-1 | 460 mm |
| Seat width | at least 400 mm, products 460-500 mm | EN 1335-1 | 480 mm |
| Backrest width | at least 360 mm, products 420-460 mm | EN 1335-1 | 440 mm, corners rounded with 45 mm (bottom) and 55 mm (top) |
| Backrest height | at least 260 mm (type A 360 mm), task chairs 450-550 mm | EN 1335-1 | 500 mm (0.497 to 0.998 m) |
| Lumbar apex above seat | 170-220 mm | EN 1335-1 (point S) | 200 mm (0.645 m) |
| Backrest inclination | 15 degrees of adjustment minimum; upright about 10-12 degrees behind vertical | EN 1335-1 | 12 degrees at rest, 0 to 20 degrees recline |
| Armrest height above seat | 200-250 mm | EN 1335-1 | 215 mm (top 0.66 m) |
| Clear width between armrests | 460-510 mm | EN 1335-1 | 490 mm |
| Armrest pad | at least 200 x 40 mm, products about 250 x 85 x 25 mm | EN 1335-1 | 250 x 85 x 25 mm |
| Underframe | five support points, projection at most 415 mm | EN 1335-1 | five arms at 72 degrees, stems on a 300 mm radius, overall 0.66 m |
| Castors | twin wheel, 50 mm wheel, 11 mm stem, hooded | EN 12529:1998 | 50 mm wheels, 2 x 13 mm, 20 mm trail, 11 mm stem (structure) |
| Gas spring | standpipe 50 mm, piston tube 28 mm, tapers | EN 16955:2017 | 50 mm / 28 mm, taper socket, guide bush |
| Mass | 12-18 kg for a task chair with armrests | product data from memory | 15.4 kg (base 1.8, castors 5 x 0.15, gas spring 1.6, seat with mechanism and armrests 8.2, backrest 3.05) |

Material data come from the asset's own references (Ashby 2011, ISO 10456, Incropera 2018). No web search was needed.

## 2. Audit of the state before round 3 (committed file)

- Rig: `base` joint at y 0.025 m (the castor axle height, not the hub, which sits at 0.09 m); castor bones at the arm tips at hub height with no lateral offset, so all castors lay in the symmetry plane and no joint was at a wheel axle; `column-sleeve` 0.30 m and `column` 0.42 m long (real standpipe about 0.24 m, piston visible about 0.19 m); `seat` bone 0.46 m long pointing up from the top of the column (it carried the seat depth as a length, not a real member); `backrest` joint 0.23 m behind the column at seat height with 0 degrees of rest inclination (a real backrest stands 10-12 degrees back and hinges at the mechanism under the seat). Masses: base 5.5 kg and seat 4.5 kg (a plastic star weighs under 2 kg; the mechanism, pan, cushion and armrests are the heavy part).
- Dimensions: 1.08 x 0.60 x 0.60 m. The star with castors on a 300 mm stem radius is 0.66 m across; the backrest top of a chair with a 0.445 m seat and a 0.50 m back is at 1.00 m.
- 2D: drawn independently per scale (cross-scale IoU of the old drawings 0.66 and 0.43, see `author-requests.md`), column as a zero-width stroke at 1:500, backrest vertical, no lumbar curve, castors without hoods, armrests as bars.
- Solids: 40 / 19 / 16 / 14 body solids fitted to those drawings: box seat, slab backrest, capsule arms, revolve wheels with implied closure (listed in `kernel-traps.md`), structure mostly references to the body.
- Thermal region areas: plastic 1.10, steel 0.22, rubber 0.035 m2, below what the honest model measures.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `dimensions` | 1.08 / 0.60 / 0.60 m, 15.4 kg | 1.00 / 0.66 / 0.66 m, 15.4 kg |
| `rig.base` | y 0.025, mass 5.5 kg | y 0.09 (hub on the arm axis), mass 1.8 kg |
| arm bones | 0.30 / 0.093 / 0.243 | 0.30 / 0.0927 / 0.2427 (projections of the true 0.30 m at 0, 72 and 144 degrees) |
| castor bones | at the arm tip, y 0, no z | at the wheel axle: 20 mm trail behind the stem, 65 mm below the hub, z +-0.2853 (side) and +-0.1763 (back); back castors turned 180 degrees with their arms |
| `column-sleeve` | 0.30 m | 0.195 m |
| `column` | 0.42 m, prismatic 0 to 0.1 | 0.25 m (hub to seat joint at 0.34 m), same travel |
| `seat` | at 0.42 on the column, 0.46 m long, 4.5 kg | at 0.25 on the column, 0.105 m (seat joint to cushion top), 8.2 kg with its centre of mass |
| `backrest` | x 0, y 0.23, 0 degrees, 0.62 m, 3.0 kg | hinge of the mechanism (0.19 m behind the column, 0.365 m high), 12 degrees, 0.645 m, 3.05 kg; recline 0 to 20 degrees unchanged |
| `provenance` | no chair references | EN 1335-1, EN 12529, EN 16955 added; notes rewritten (en, de) |
| `thermal.regions[].areaM2` | plastic 1.10, steel 0.22, rubber 0.035 | 1.41, 0.26, 0.05 (measured on the 1:50 model, inside the C16 factor); fabric 1.0 unchanged; castor assumption rewritten |
| `representations[*].geometry` | 42/28/27, 14/21/17, 13/18/14, 7/9/12 shapes | 50/51/45, 24/36/34, 18/28/25, 17/16/25 shapes (elevation/front/plan at 1:50, 1:100, 1:200, 1:500), ruling E1 |
| `representations[*].decisions` | old texts | rewritten per scale, en and de |

Unchanged and still valid: anchors (contact 25 mm under each castor joint, now the true axle), poses, clips, supports, structure settings. Poses and clips were rendered after the rig change (section 6).

2D (ruling E1): every filled shape of every scale and view is the orthographic outline of a named group of that scale's body solids, bound to its bone with the ids, roles, layer order and pens of the previous drawing; wheels are ellipses; rib, ring, groove, piping, fold, rim and seam strokes of 1:50 are placed by hand. Seat-tree shapes stay extruded `solid` shapes so the swivel turns them.

## 4. What was modelled

### 1:50 — body 85, thermal 89, structure 51
- Body: revolved hub with conical bore; five lofted arms (five sections each) falling from the hub to revolved stem bosses; per castor swivel head, steel neck, centre web, hood, two rubber tyres on plastic rims; standpipe with guide bush, piston tube and taper socket; lofted mechanism pan with seat plate, hinge block and pin, two levers with paddles, tension knob; plastic seat pan (seven sections) and cushion (nine sections, waterfall front); armrests as strap under the seat, bent post, inner stem, cap and pad; backrest as a doubly curved surface of revolution (lumbar S-curve as profile, wrap as sweep about an axis in front of the sitter, tilted 10 degrees) as shell and pad, each with lofted side bands; bent flat-steel carrier with foot and plastic cover.
- Second author's corrections: the side bands of shell and pad were sampled only at the profile stations, which gave chamfered, octagonal corners and a notch at the top; they now follow an outline function with true quarter rounds (45 mm bottom, 55 mm top) through 19 (shell) and 17 (pad) stations. The shell bands were 2 mm proud of the central surface at their inner end, which produced jagged sliver seams on the back; they are now 2.5 mm thinner at the back so the seam is one clean step line.
- Thermal: every body solid as the surface of its material's region with its layer; foam cores of cushion, pad and pad bands nested 1 mm inside the fabric. Structure: hub, arms and bosses, standpipe and piston as tubes with real walls (3.5 mm), mechanism, plate, hinge, seat pan, armrest straps, posts and stems, carrier, cover and shell; joints: castor stems and axles, guide bush, taper, hinge pin; one contact pad per castor; four masses.

### 1:100 — body 53, thermal 53, structure 48
Authored for 15 mm: arms with three sections, one head, centre body and two plain wheels per castor, standpipe and piston one revolve each, mechanism as one pan with hinge block and knob, levers dropped, pan and cushion with fewer sections, armrest as bent post and pad, backrest shell and pad with six profile stations and rounded bands (10 and 8 stations), carrier and cover. Thermal by material; structure as at 1:50 with thicker pins.

### 1:200 — body 32, thermal 33, structure 33
Authored for 30 mm: arms through to rounded tips, boss reaching down as castor head, one wheel body per castor, plain column, mechanism wedge, one seat mass, armrest post and pad, backrest as one curved mass with two rounded bands (8 stations), carrier at its true 10 mm. Thermal adds the pad surface of the backrest so the fabric region keeps its area. Structure: members, a seat pan of its own, supports, masses.

### 1:500 — body 29, thermal 30, structure 31
Authored for 75 mm: hub, five two-section arms with bosses over wheel bodies, stepped column, mechanism wedge, seat mass, armrest post and pad, one curved backrest panel with its carrier. Second author: top of the panel lowered 4 mm so the chair measures 1.00 m at this scale too (the drawing was dimensioned 1.01 m).

## 5. Checks (final)

- `r3-validate.ts furniture/office-swivel-chair --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only`: `VALID`. Catalog `volumes` equal `volumes/furniture/office-swivel-chair.json` (spliced with `volumes-insert.py`: all four scales authored).
- `volumes-check.ts --product`: PASS at every scale. Silhouette IoU elevation / front / plan: 1:50 0.994 / 0.992 / 0.999; 1:100 0.992 / 0.992 / 0.998; 1:200 0.973 / 0.977 / 0.997; 1:500 0.978 / 0.972 / 0.991. Height deviation 2.9 / 2.9 / 2.9 / 1.6 mm. Thermal area deviation at 1:50: plastic 0.000, fabric 0.081, steel 0.012, rubber 0.000.

## 6. What the final renders show

Opened with the Read tool: `r3-sheet` of body at every scale, structure at 1:50, 1:100 (reclined) and 1:500, thermal cut at 1:50 and 1:100, back and front alone at 1:50, a low three-quarter from behind, poses `reclined` and `swivel-quarter`, and `r3-2d` at every scale plus `reclined` and `swivel-quarter` at 1:50.

- Front: rounded backrest with a narrow shell rim around the pad, two clean panel seams, T-armrests on bent posts rising from straps under the pan, pan under the cushion, mechanism with knob and both levers, stepped column, star with five hooded twin-wheel castors.
- Side: lumbar S-curve, backrest 12 degrees back on its bent carrier from the hinge under the seat, waterfall seat front, armrest post behind the seat centre, arms falling from hub to bosses, castors trailing.
- Back: plain shell with rounded corners, two straight step seams to the side bands, cover over the carrier; no slivers left.
- Top: seat with rounded corners, wrapped backrest, pads over their caps, front arm and castor ahead of the seat.
- From below and behind: seat plate, mechanism pan, hinge, straps and levers are all there and connected; nothing floats.
- Reclined (32 degrees) and swivel: carrier, cover, shell and pad move as one about the hinge; seat tree turns on the column, star stays; no gaps. Height-adjust frame: piston slides out of the standpipe, bush stays.
- Structure reads as the load path (shell, carrier, pan, mechanism, tubes, star, stems, contact pads), cushion, pads, hoods and levers absent. Thermal cut shows the foam inside the fabric at 1:50.
- Coarser scales stay a recognisable task chair down to 1:500.

## 7. Still imperfect

- The lowest seat position is 445 mm; EN 1335-1 asks for a range that reaches 400 mm. 445-545 mm matches many real products and the asset's previous seat height; lowering it would move every part and was not done.
- `swivel-quarter` in 2D: the product turns the extruded `solid` shapes, so the backrest seen from behind in the side elevation is a rectangle without rounded corners and the chair seen from the side in the front view loses the lumbar curve and the armrest shape. The 3D poses are right. This is the limit of extruded 2D shapes, not of this asset's data; a true quarter-turn drawing would need the 2D to be projected from the volumes.
- The top edge of the backrest is 5 mm higher at the sides than at the centre (surface of revolution about a tilted axis); real shells are flat or slightly crowned there.
- 1:200 and 1:500: the merged backrest mass carries the plastic material, so its front shows the plastic tone in the tone views; the fabric pad exists there only as the thermal surface. Splitting a 30-45 mm mass into two layers is below the feature size of these scales.
- `r3-sheet --list thermal` at 1:100 and coarser paints the fabric region grey because its legend only knows the two 1:50 layers; the data are right (region `chair-fabric`, no layer).
- The plan drawing at 1:50 keeps the short fold stroke on the cushion from the previous drawing (role `texture`); it is the only free-hand mark and reads a little arbitrary.
