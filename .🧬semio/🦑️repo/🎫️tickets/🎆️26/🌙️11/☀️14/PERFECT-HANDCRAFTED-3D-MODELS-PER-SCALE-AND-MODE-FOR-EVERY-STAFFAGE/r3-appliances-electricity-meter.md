# Round 3 — appliances/electricity-meter

State: DONE

Resumed on 2026-11-20 from the interrupted session (scripts of 03:14 to 03:18, empty report). Scripts: `r3-electricity-meter-kit.py` (frames, helpers), `r3-electricity-meter-model.py` (every solid by hand, writes `volumes/appliances/electricity-meter.json`), `r3-electricity-meter-roots.py` (candidate document with the root corrections), `r3-electricity-meter-apply.py` (writes the validated candidate members into the catalog file), `r3-electricity-meter-peek.sh` (1:50 without the pane), `r3-electricity-meter-peek500.sh` (1:500 sheets), `r3-electricity-meter-print.ts`.

## What the resume found

- The catalog still held the first root version of 03:15 (lower screws 150 mm under the lug, bone `body` at y = -0.050) while kit, model and roots had moved to 158 mm at 03:18: the source solids were 8 mm off the catalog rig. Candidate rebuilt, validated, applied.
- The bulk pass of 03:47 (`digital-models.py`) had written a 1:500 "digital" volume (5 / 6 / 5 solids, cover as an octagonal prism, two-sentence decisions) into catalog and source. Source 1:50, 1:100, 1:200 were byte-equal to the output of my model script. The 1:500 is now authored by hand in `scale_500()`; nothing of the bulk version is kept.

## Reference

| item | value | source |
|---|---|---|
| kind | single-phase two-wire Ferraris (induction) meter, class 2, 10(40) A or 10(60) A | NZR Ferraris WS data (`nzr-ws-ferraris`), IEC 62053-11 |
| envelope | 120 to 145 mm wide, 210 to 270 mm high, 100 to 150 mm deep; model 132 x 212 x 118 mm | NZR |
| mass | 1.0 to 1.5 kg; model 1.35 kg | NZR |
| fixing | three points: hanging lug above, two screws beside the terminal block | DIN 43857-1 (title and principle); the distances 158 mm upright and 105 mm across are remembered, NOT read from the standard (a web search on 2026-11-20 returned only the catalogue pages of the standard) |
| housing | base of moulded insulation about 25 mm deep running down into the terminal block; hollow cover with 2 to 3 mm wall, draft of 2 to 3 degrees, rounded front edge, window; terminal cover with one sealing screw, cover with two | knowledge of the type (AEG, Siemens, Landis & Gyr household meters) |
| register | six rollers (five whole kWh, one decimal), roller about 7 mm wide, 14 mm diameter | NZR (register 5/1), knowledge |
| rotor | aluminium disc 80 to 90 mm, about 1.2 mm thick, horizontal on an upright spindle of about 3 mm between a lower and an upper bearing; dark rim mark | knowledge |
| measuring system | voltage electromagnet above, current electromagnet with two coils below the disc, brake magnet gripping the rim, die-cast frame on the base | knowledge |
| own consumption | voltage circuit under 2 W, current circuit about 1 W at full load | IEC 62053-11 |

## Audit (state before round 3)

- Rig: bone `body` was placed at the envelope, not on a real part; the lower joint is now the pair of lower screws (y = -0.058), the tip the lug hole (y = +0.100), in the plane of the centre of mass (z = 0.015). Bone `disc` was drawn as a disc facing the room (a round disc in the face view); a Ferraris disc is horizontal and seen edge-on. It is now the upright spindle from the lower bearing (y = -0.030, z = -0.004), 62 mm long, turning by twist.
- Poses and clip rotated the disc by `angleDeg` in the paper plane; they now use `twistDeg` about the spindle (same amounts: 18, 36 degrees; clip 0, 270, 630 degrees).
- Anchors: `wall-screw-top` moved to the lug hole (x = 0.158 on the bone, wall plane z = 0.044), `wall-screw-bottom` to the bone origin with a 115 mm footprint for the pair.
- 2D: round disc and ring window in the face view, plain box in side view and plan. Redrawn at every scale and view as the true orthographic view of that scale's body (ruling E1); the disc, hub and mark carry `solid` profiles on the rotor bone so that the mark passes the window with the twist.
- Solids: the QA rows stand (sharp box cover with a step, counter as plate, body block as structure, dot supports, th-plate in the terminal zone at 1:200). All replaced.
- Thermal areas against the new 1:50 model: casing 0.205 to 0.139 m2 (within factor 1.5), window 0.011 to 0.0096, disc 0.0107 kept, plate 0.0135 to 0.0104. `heatW` untouched.

## Root fixes in the catalog file

| member | old | new |
|---|---|---|
| `rig.bones.body` | y -0.050, lengthM 0.150, comX 0.055 (first r3 version) | y -0.058, lengthM 0.158, comX 0.063 (centre of mass unchanged in space) |
| `rig.bones.disc` | x 0.020 on the parent | x 0.028 (same place in space) |
| `anchors.wall-screw-top` | x 0.150 | x 0.158 |
| `structure.assumptions`, `provenance.notes` | 150 mm | 158 mm |
| `representations` 50 / 100 / 200 | first r3 drawings | redrawn in the new bone frame (same forms) |
| `volumes` | 33/11/14, 14/8/9, 6/6/5, bulk 5/6/5 | see below |

Earlier in round 3 (first session, already in the catalog): `dimensions`, `materials` (`steel-zinc`, `copper-winding` added), references `nzr-ws-ferraris` and `din-43857-1-1978`, summary, poses, clip keys, region titles, lengths and areas, the 1:500 unavailability text.

## Models

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 34 solids: base tray (back wall and rim with hole), steel lug with 5.5 mm hole, terminal block with four bores from below, terminal cover as channel plus top wall, its sealing screw, hollow cover (2.5 mm wall, 2.8 degree draft), shoulder and bevel, window frame, pane, two sealing-screw towers, dial plate with register opening, roller drum and decimal roller, register case, frame back on four posts, two shelves, lower bearing, voltage core and coil, current core and two coils, C-shaped brake magnet on its bracket, disc, hub, spindle (slender), rim mark (texture) | 12: closed cover form and shoulder, bevel, frame, base, block, terminal cover (casing, layer shell); pane with inner face as `sectionFaces`; disc; dial plate | 14: lug, base tray and rim, frame back, posts, shelves, spindle, two bearings as joints, three wall screws with heads as supports, mass at the centre of mass |
| 1:100 | 15: base slab, lug tab, block, terminal cover channel with top, hollow cover, one 45 degree bevel as window reveal, pane, dial plate with slot, register, two electromagnet blocks, frame block, disc, mark | 9 | 9 |
| 1:200 | 6: base, terminal block, thick hollow cover with draft and chamfered corners, pane, works block, disc | 6 | 5 |
| 1:500 | 5: square base slab, terminal block, square hollow cover without draft, pane, works block (disc merged) | 6: closed cover form, base, block, pane, the real disc sheet inside the block, dial sheet | 5: base slab, spindle, two 24 mm support pads in the back of the base, mass |

Final check line: `volumes-check.ts appliances/electricity-meter --product` = `PASS (2.2 s)`, every rule `ok` at 1:50, 1:100, 1:200, 1:500 (silhouette IoU 1:50 0.998 / 0.999 / 0.996, 1:100 0.999 / 1 / 1, 1:200 0.994 / 1 / 1; areas 1:50 casing 2.2 %, window 1.9 %, disc 5.0 %, plate 1.0 %). `r3-validate.ts appliances/electricity-meter --volumes` and `--catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced with `volumes-insert.py`: `1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored`.

## What I saw (2026-11-20, after the resume)

- 1:50 body sheet (`🗑️generated/r3-sheet/appliances/electricity-meter.s50.body.png`): side views show the 26 mm base against the wall, the cover with its draft and the two-facet rounded front edge, the set-back terminal cover with its screw, the lug strap; face view the rounded cover, the raised window frame, the two sealing-screw towers, the lug with its hole; top view the tapering cover. Reads as a household Ferraris meter at first sight. The pane is drawn opaque by the tool.
- 1:50 without the pane (`🗑️generated/r3-electricity-meter/peek.s50--onlyright--no-rig.png`, `...three-quarter...png`): dial plate with the register opening and the roller drum with its darker decimal roller; under the plate the disc edge-on with hub and spindle, the brake magnet at the left, the two current coils below. Nothing floats, nothing pokes through the cover.
- 1:50 structure: lug, tray, frame back, both shelves, spindle between its bearings, mass sphere, three green screws on the wall plane; the body is only a ghost.
- 1:50 thermal with cut: casing as closed forms, pane green at the front, dial plate yellow behind it, disc violet and horizontal.
- 1:50 clip `run` at 5 s, face view: rig and solids move together, housing unchanged.
- 1:100 and 1:200 body sheets: the same three masses with one bevel (1:100) and a chamfered thick shell (1:200); lug kept as a tab at 1:100, dropped at 1:200.
- 1:500 body, structure (also from behind), thermal cut (`🗑️generated/r3-electricity-meter/s500*.png`): three square masses and the window; the first version had the upper support floating above the base (no lug at this scale), corrected to pads let into the slab.
- 2D 1:50, 1:100 at paper size (`🗑️generated/r3-2d/appliances/electricity-meter.s50.png`, `.s100.png`) and 1:50 enlarged (`🗑️generated/r3-electricity-meter/zoom2d.s50.png`): face view, side view and plan are the views of the body; dial with three text lines, register strip, disc edge with hub.

## Still imperfect, stated plainly

- The fixing distances (158 mm, 105 mm) are not verified against DIN 43857-1; the standard is not freely readable. `provenance.notes` says so.
- A black band surrounds the cover in the face view of the r3 sheets: it is the 2.8 degree draft wall seen at a grazing angle under the tool's head light, not geometry (the three-quarter views show the wall lit).
- `r3-sheet.ts` cannot render 1:500 (no 2D representation of that scale); rendered through the slot swap of `r3-electricity-meter-peek500.sh`, so those sheets carry the footer "1:200". Logged in `tool-requests.md`.
- The 2D of 1:200 was opened only at paper size (0.66 mm by 1.0 mm), not enlarged in this session; the plan of 1:50 is cropped at the lower edge of the enlargement.
- At 1:500 the supports lie flush in the back face of the base slab (coincident faces flicker in the back view).
