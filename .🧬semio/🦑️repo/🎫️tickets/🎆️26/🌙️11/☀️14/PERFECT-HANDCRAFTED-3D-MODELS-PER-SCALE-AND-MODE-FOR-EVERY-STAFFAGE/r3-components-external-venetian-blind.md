# Round 3 — components/external-venetian-blind

State: DONE

`volumes-check.ts components/external-venetian-blind --product`: `PASS (39.5 s)` at 1:50, 1:100, 1:200 and 1:500. `r3-validate.ts components/external-venetian-blind --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Print suitability: all four scales `authored`. Spliced into the catalog; the manifest `distinction` follows the new summary.

No earlier round-3 work existed (no `r3-external-venetian-blind-*`, no report, source equal to the catalog, four authored scales, no new digital scales from the bulk passes). Started at step 1.

Scripts (ticket folder):

- `r3-external-venetian-blind-kit.py`: every real dimension, the section polygons (Z 90 lamella with camber, flanges and beads, cover sheet, head rail, bracket, end plate, guide rail, bottom rail, frame rail, sill board), the stack kinematics and the mass budget.
- `r3-external-venetian-blind-volumes.py`: every solid of every scale and list; writes `volumes/components/external-venetian-blind.json`.
- `r3-external-venetian-blind-roots.py`: root fixes and the 2D drawings of all four scales; without flag the candidate for `--file` checks, with `--apply` it replaces only the listed catalog members (byte layout kept, everything else verified unchanged).
- `r3-external-venetian-blind-zoom.ts`: rasterises one SVG panel of `r3-2d.ts --svg` enlarged, to judge the 1:50 section.

## 1. Reference

| item | real value | source |
|---|---|---|
| type | Facade-mounted raffstore: lamellae on ladder cords, lifted by two tapes, stack hidden behind a cover panel, lateral guide rails | asset summary; own knowledge |
| lamella | Z 90: 90 mm wide, 0.45 mm aluminium, 80 mm pitch, profile about 93 x 20 mm over the beads; inner edge up, outer edge down, cambered web, rolled beads | asset reference `blind-catalogues`; [Z 90 price list](https://cdn.nextproducts.at/m20/30/43576730/etc/22/50116222.pdf) (search result: "Z90 ... 93 x 20 mm"); Bandalux sheet (0.45 mm) |
| head rail | 58 x 56 mm roll-formed U of galvanised steel | [Hella AF 80 K manual, p. 9](https://www.manualslib.com/manual/1670901/Hella-Af-80-K.html?page=9); the Z 90 price list |
| bottom rail | extruded aluminium, 80 x 18.6 mm for 80 mm slats; taken 90 x 20 mm for Z 90 | Hella AF 80 K manual |
| lift tape | 6.0 x 0.28 mm (80 mm slats); taken 8 x 0.5 mm, two tapes | Hella AF 80 K manual |
| guide rail | aluminium C about 30 x 20 mm, guide pins on every second lamella and on the bottom rail | own knowledge; the 2D drew 30 x 20 mm |
| cover and package | cover 150 x 210 mm; the package (head rail + stack + bottom rail) must fit inside it. 56 + 18 x 6 + 20 mm = 184 mm plus clearances | asset (150 x 210); package arithmetic; the manufacturer package table could not be read (PDF tools missing), stated below |
| window | fixed light, 4-16-4 unit, thermally broken aluminium frame 70 mm deep with 50 mm face, exterior stop, interior bead, 12 mm edge spacer | asset layers; EN ISO 10077-1; own knowledge |
| wall | 15 mm plaster, 175 mm clay block (238 mm courses, 12 mm bed joints), 113 mm flat lintel, 70 mm EPS, 10 mm render; bent 2 mm aluminium sill board with 5 degree fall and drip | asset thermal layers; own knowledge of ETICS reveals |

Not verified: the manufacturer package-height table for Z 90 and the exact guide-rail profile. The stack pitch of 6 mm per lamella is my estimate.

## 2. Audit findings (before)

Rig and kinematics

- The blind hung at x = 0.105 m, so the 90 mm lamellae ended flush with the outer face of the 150 mm box: the fascia had no room. A real cover clears the lamellae on both sides.
- 17 lamellae from y = 2.325 m down. With the head rail at the top of the box, the first rung at 65 mm below it (y = 2.405 m) was missing: one lamella short.
- `retracted` stacked the lamellae up to y = 2.52 m, where the head rail sits (box top 2.53 m), with the 40 mm bottom rail under them: no room for a head rail at all.
- Closed poses lifted `slide-16` by 5 mm to dodge the bottom rail and still left the closed lamella inside the 40 mm rail.
- Bottom rail 90 x 40 mm; real about 90 x 20 mm.

2D

- The side elevation was a section by text only (`sections` not declared), so no cut marker and no section comparison.
- No window frame: the glass floated between the wall stubs with 50 mm of air on both sides. No sill board, no lintel, no reveal returns.
- Lamellae as 12 mm filled bars; the box a filled block with an invented slot; guides as two lines.
- Box bone: elevation extent 210 mm against front 1500 mm (listed in `author-requests.md`).

Solids

- Box a hollow block with 7.5 mm walls and a 90 mm slot; no head rail, no brackets, no tapes, no holders; lamellae lens-shaped lofts; glass two bare panes; walls plain layer boxes.
- 1:200 was 1:100 with thicker slats; thermal at 1:500 kept lamellae the body lacked (QA); structure referenced the body and marked "joints" at slat ends without a hanging path.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones` | 17 `slide-*` / `slat-*` at x 0.105, y 2.325 - 0.08 i | 18 at x 0.085, y 2.405 - 0.08 i |
| `bottom-rail` bone | x 0.105, y 1.02 | x 0.085, y 0.985 (hangs 25 mm below the sill line in front of the drip, so the closed lowest lamella clears it) |
| masses | facade 355, box 5.7, rail 1.4, slat 0.14 kg; total 364.48 | facade 349.4, box 10.2 (cover 3.0, head rail 2.6, motor 1.8, shaft 0.7, guides 1.5, hardware 0.6), rail 1.33, slat 0.149; total 363.61 kg |
| poses `retracted`, `lift-1..4` | stack of 7.25 mm pitch up to the box top | rail lift 1.347 / 0.27 / 0.54 / 0.81 / 1.08 m; the rail collects the lamellae from below at 6 mm pitch (`kit.lifted`); retracted stack 2.354 to 2.456 m under the head rail (2.468 m), rail bottom 2.322 m |
| tilt poses | `slide-16` dy 0.005 | removed; angles unchanged for 18 lamellae |
| anchors | `blind-centre` x 0.105; `box-bracket` (0, 0.1) | x 0.085; (0, 0.1375) at the bracket anchors |
| materials | aluminium, glass, masonry, eps | + `steel`, `polyamide` |
| `thermal.regions` | 4 regions | + `window-frame` (conditioned, Uf about 2.8 W/m2K, authored estimate), in every pose and clip track like the glazing |
| `areaM2` | lamellae 3.672, box-and-rails 1.58, glazing-behind 1.5, facade-wall 1.8 | 4.28 (18 real Z sheets, both faces; x 1.17), 2.05 (open cover counted inside and outside, rails with channel; x 1.30), 1.35 (pane 1.13 x 1.18 m; x 0.90), 2.33 (plus reveal soffit and sill board; x 1.29); window-frame 0.28 (new). All within the factor 1.5 of C16 |
| `representations.*` | see 2D | `sections.elevation` A-A at z = 0; geometry and decisions rewritten at all four scales; `pens.texture` 0.1 at 1:200 and 1:500 |
| `summary`, provenance, assumptions | 17 lamellae, headrail box | 18 lamellae, head rail behind a cover, fixed aluminium window; references `hella-af-80`, `z90-price-list` |

2D (ruling E1, drawn from the kit numbers of the solids):

- 1:50 section: hatched block with bed joints and lintel, plaster band, stippled EPS and sill wedge, render stroke with soffit, cut frame rails with bead and stop, both panes with spacers, sill board stroke, far jamb light; cover as a stroke with the far end plate behind it, head rail U, motor and shaft circles, bracket, guide rail on two holders, 18 Z lamella strokes, hollow bottom rail.
- 1:50 front and plan: frame ring, glass, sill board, cover, end-plate seams, guides, holders; wall head layers and cover in plan.
- 1:100, 1:200, 1:500: stepwise merged (texts in `decisions`).
- Found on the way: the line view drops edges between shapes of one region, so the cut frame vanished inside the far jamb; explicit edge strokes added. A flat lamella stroke gives no front shape, so the coarse lamellae keep their Z stroke.

## 4. Volumes

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 106 | 55 | 82 |
| 1:100 | 38 | 36 | 51 |
| 1:200 | 31 | 30 | 11 |
| 1:500 | 15 | 28 | 9 |

- **1:50 body**: wall as built (courses, bed joints, lintel, plaster and render with reveal returns, EPS, sill wedge, bent sill board); window frame (two rails, two jambs, two jamb stops, bead ring), two panes and spacer ring; cover as a bent 2 mm sheet with wall flange and hem, two flanged end plates, steel head rail U with lips, two strap brackets with nuts, motor, tilt shaft, two tape bearings with shaft bore, two C guide rails, four spacer holders; 18 Z 90 lamellae as real 0.45 mm sections with beads, guide pins on every second one, hollow bottom rail with caps and pins, two lift tapes.
- **Lift tapes**: each run between two lamellae is two pieces blended between the neighbouring slide bones, so the tape shortens as the stack closes (checked in the `raise-lower` frame and in `retracted`).
- **1:50 thermal**: four wall layers, three glazing layers, frame, blind surfaces, every lamella sheet. The head rail, motor, shaft and brackets are deliberately not thermal solids: with them the region would be 2.49 m2, beyond the factor 1.5.
- **1:50 structure**: masonry leaf, lintel with bearings, foot support, two mass marks; head rail, brackets, anchors through the insulation, tape bearings, tapes, bottom rail, guide rails with holders, lamellae as beams with their pins, motor mass.
- **Coarser scales**: authored anew (merged wall masses, frame ring, plain cover sheet, solid bars, plain Z strips; 1:500 a closed box and five strips). At 1:500 the thermal list keeps all 18 lamella sheets, 13 of them thermal-only: five strips left the total area 38 % short.

Final check: silhouette IoU 0.994 / 0.999 / 0.999 at 1:50, 0.993 / 1 / 1 at 1:100, 1 / 1 / 1 at 1:200 and 1:500; thermal areas at 1:50 within 0.4 %; totals 13 % at 1:200 and 1:500.

## 5. What I saw (final renders, `🗑️generated/r3-sheet/components/external-venetian-blind.*`, `🗑️generated/r3-2d/components/external-venetian-blind.*`)

- **1:50 body, three-quarter**: reads as a raffstore at first sight: cover on the wall head, lamellae with bent edges, two tapes, rails on holders, sill board under the bottom rail, frame and glass behind.
- **1:50 body, from below**: open cover, lamella undersides, tapes, holders with flanges, frame with stops.
- **1:50 section cut**: head rail with motor at the top of the cover, top lamella inside the cover, Z sections, hollow rail, sill board with drip, coursed wall.
- **Poses**: `retracted` stack under the head rail, rail flush with the cover bottom; `lowered-closed` a shingled closed screen; `raise-lower` at 5 s the stack collected from below with shortened tapes; 45 degrees in 2D with the outer edge down.
- **1:50 thermal cut**: layers of wall and glazing distinct, frame yellow, lamellae red, blind green. **Structure**: hanging path readable, green foot, magenta masses, orange bearings and holders.
- **1:100, 1:200, 1:500**: all three lists opened; recognisable, no solid failed to draw.

## 6. Still imperfect

- Ladder cords are not modelled at any scale (2 mm, and rigid bones cannot carry a cord through lift and tilt). The tapes exist in 3D only; the 2D omits them and says so.
- The bottom rail does not tilt with the lamellae (its bone is prismatic only).
- Thermal: the mechanism inside the cover has no thermal solid (area limit above); the layerless coarse wall and glass render grey in `r3-sheet` (tool colouring).
- Render artefacts, not geometry: coplanar cover back and wall face flicker in the thermal back panel at 1:100 and 1:500; a small dark mark on the cover top at 1:500.
- Frame corners are butt-jointed bars, not mitred; gaskets and thermal break strips are omitted.
- Package height and guide-rail profile rest on my estimate, not on a manufacturer table.

## Round 5 handcraft (2026-11-21)

Renders (opened with Read) under `🗑️generated/r5-external-venetian-blind/before/` and `after/` (made by `r5-c01-sheets.sh`). (renders in `🗑️generated/r3-sheet`). I opened body, structure and the cut thermal sheets at 1:50; body at 1:100, 1:200 and 1:500; the 2D at 1:500; the clip `raise-lower` at 5 s; and the pose `closed-in-gust`. Further poses were rendered but not opened.

Against the reference (Z 90 lamella 90 mm at 80 mm pitch, 58 x 56 head rail, 30 x 20 guide rails, 18 lamellae): no deviation.

| scale | before = after (body / thermal / structure) |
|---|---|
| 1:50 | A- / A- / A- |
| 1:100 | A- / A- / A- |
| 1:200 | A- / A- / A- |
| 1:500 | B+ / A- / B+ |

At 1:500 the five 75 mm strips read as a ladder rather than as a lamella screen. The 2D draws the same strips, so a toned screen would need 2D and 3D together.

Poses: the stack collects from below with tapes that shorten; the gust pose closes as a shingled screen.

No change. The source equals the catalog; it is VALID.

The insulation unit contains two 4 mm panes around a 16 mm sealed argon space. The 12 mm spacer follows a 6 mm inner bend radius. Its physical profile persists in every model, while the cavity outline uses fewer stations at smaller scales. A Three.js ray and bounding boxes independently checked the weather-side and room-side faces, pane and cavity dimensions, spacer, and thermal layer order. Measured external area, structural coverage, scale agreement, silhouette and mesh-count checks passed at all four scales. The twelve exported scenes occupy 2,222,780 bytes.

A visual comparison then showed the coarse physical blind had only five of the eighteen slats. The 1:500 physical model, paper views and heat-transfer surfaces now retain all eighteen 1.13 m slats and their separate tilt axes. Three.js stock checks, the full document validator, measured scale checks and independent GLB readback passed. The updated GLB is 2,280,848 bytes.
