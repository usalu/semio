# Round 3 — machinery/solar-thermal-collector

State on 2026-11-20: finished and spliced. No earlier round-3 work existed (only the round-1 script `author-26-solar-thermal-collector.py`). At the start `r3-validate.ts` was VALID, the catalog `volumes` equalled the source, and `volumes-check.ts --product` FAILED (1:100 header areas 31 % over).

Catalog asset: `S/🗂️catalog/🏗️machinery/solar-thermal-collector/🔣️.json`. Scripts in `T`:

- `r3-solar-thermal-collector-build.py`: every construction dimension, solid, bone, anchor, 2D shape and text, stated by hand. Without arguments it writes `T/volumes/machinery/solar-thermal-collector.json` and a candidate document; `--apply` re-reads the catalog file and replaces `provenance`, `rig`, `anchors`, `representations`, `thermal` and `volumes` in one write. Re-running changes nothing.
- `r3-solar-thermal-collector-print.ts`: the product's print suitability on a candidate (the park-bench author's tool with the path changed).

Final lines:

- `r3-validate.ts machinery/solar-thermal-collector --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- `volumes-check.ts machinery/solar-thermal-collector --product`: `PASS`.
- Print suitability: suitable at all four scales (smallest feature 0.33 / 0.28 / 0.21 / 0.28 mm on paper).
- `repo run` test targets were not run by me.

## 1. Reference

| item | real value | source | used |
|---|---|---|---|
| Gross size, depth | 2.0 m2 class collectors are 85 to 100 mm deep (1235 x 1625 x 85 mm for 2.00 m2) | Romstal Vision installation manual (web search 2026-11-20) | 2.0 x 1.0 x 0.1 m kept (asset `dimensions`) |
| Cover | 3.2 mm low-iron tempered solar glass is the usual pane (same manual; Stiebel Eltron SOL 27: 3.2 mm) | web search | 3.2 mm, 1.99 x 0.99 m (was "4 mm", which contradicts the declared 14 kg: 4 mm weighs 19 kg, 3.2 mm 15.8 kg) |
| Harp | nine riser pipes, 22 mm header tubes, mineral wool (Bongioanni collector, 2.5 m2: risers 8 mm) | web search | nine 10 mm risers at 100 mm pitch (asset notes), 22 mm headers |
| Back insulation | 50 mm and more of mineral wool; air gap about 25 mm | Duffie and Beckman (asset reference `duffie-beckman-2013`, flat-plate examples); asset thermal assumption (25 mm gap). Not confirmed by a data sheet in my search | 58 mm, which is what the 100 mm box leaves after sheet, harp, 25 mm gap, glass and cover |
| Tray and absorber sheet | 1.5 mm aluminium tray, 0.5 mm selective sheet | asset notes and layer data | kept |
| Stand | light flat-roof stands for one collector weigh about 4 kg (TWL for FK200: aluminium, 3.82 kg; Engel Helios: galvanised steel, 4 kg) | web search | cold-formed galvanised angle 25 x 25 x 2 mm, take-off 6.67 kg against 6.4 kg declared |
| View convention | front elevation is seen from ahead (+x) | `S/README.md` line 40, schema `Viewpoint` | collector turned to face ahead |

Sources: [Romstal Vision manual](https://cdn.contentspeed.ro/pimromstal.websales.ro/cs-content/cs-docs/products/manualinstalarecolectorplanromstalvisionpeterasacusuportimodularien_64382_31_1784576199.pdf), [Bongioanni collector](https://www.archiexpo.com/prod/bongioanni-caldaie/product-61920-1711905.html), [Stiebel Eltron SOL 27](https://www.stiebel-eltron-usa.com/sites/default/files/pdf/tech-specs-sol-27-premium.pdf), [TWL stand](https://www.klimaworld.com/products/flachdach-aufstanderung-1-flachkollektor-fk200), [Engel Helios holder](https://cdn.shopify.com/s/files/1/0794/9402/1463/files/Datenblatt_Helios_Flachdachhalterung.pdf).

## 2. Audit (before)

- **Orientation (root).** The low edge was at -x, so the aperture faced backwards. The product's front view looks from +x and would see the tray back and the rear struts; the 2D front drew the glass and absorber instead. The silhouette rule cannot see this.
- **Rig.** All five bones sat at (0, 0) with angle 0 and lengths 1.5 or 2 m: the stand bone ran from the middle of the rails 0.75 m out into the air, the collector bones lay flat on the ground, and glass, absorber and headers hung on the stand instead of the housing.
- **Dimensions.** Overall 1.6957 x 1.0 x 1.4091 m is right for 2.0 x 1.0 x 0.1 m at 35 degrees. Internal data contradicted itself: headers 22 mm in the notes but 33 mm drawn and modelled (44 mm at 1:100), risers 10 mm against 12 mm, glass 4 mm against 14 kg, 26 mm wool in a box that leaves 58 mm.
- **2D.** Sheets were thickened without system (tray 12 to 20 mm, glass 10 mm, absorber 8 mm); the struts ran 10 mm into the tray floor; no brace, no connection between strut and tray.
- **Solids.** Body: thickened plates (10 mm glass, 8 mm absorber), 30 mm solid side walls, 33 mm headers, 50 x 50 mm solid rails and struts that stood unbraced and pierced the tray. Thermal: outlet was a 0.5 m disc centred 0.1 m from the glass edge, so half of it hung in the air (the "hatched half-disc" of `qa-visual-2.md`). Structure: the whole body by reference, a slab (the QA finding).

## 3. Root fixes

| member | old | new |
|---|---|---|
| orientation | low edge at -x | low edge at +x: every elevation and plan shape, anchor and centre of mass redrawn in the new frames |
| `rig.bones` stand | (0, 0), 0 deg, 1.5 m | (-0.75, 0), 0 deg, 1.5 m: along the base rails; centre of mass (0.532, 0.309) from the take-off (was world (0, 0.35)) |
| housing | parent stand, (0, 0), 0 deg, 2 m | parent stand, (0.19706, 1.12067), -35 deg, 1.64 m: joint at the upper cleat on the tray back, u down the slope, v out of the aperture; centre of mass (0.64, 0.023) |
| glass | parent stand, (0, 0) | parent housing, (-0.355, 0.0969), 1.99 m: in the pane |
| headers | parent stand, (0, 0) | parent housing, (-0.315, 0.0648), 1.91 m: from the upper to the lower header axis |
| absorber | parent stand, (0, 0) | parent headers, (0.015, 0.0051), 1.88 m: in the sheet |
| `anchors` base-contact | stand (0, 0) | stand (0.75, 0), same floor point; footprint unchanged |
| glass-top | glass (0.70856, 1.35171) | glass (0.11, 0.0016): on the glass, 0.115 m below the upper edge; title "Upper strip of the glass" |
| `representations` 50/100/200/500 | see section 2 | all three views redrawn as views of the new body (section 4); decisions rewritten en/de |
| `thermal.regions[glazing]` | layer 4 mm, R 0.004, 7500 J/m2K; area 3.74 | 3.2 mm, R 0.0032, 6000 J/m2K (2500 x 750 x 0.0032); area 3.96 (2 x 1.99 x 0.99 + edges) |
| absorber area | 4.1 (with 12 mm risers) | 3.88 (sheet both faces 3.42 + nine 10 mm risers outside the weld 0.45); `heatW` -620 unchanged |
| header-lower/-upper | area 0.093, Lc 0.033 (33 mm pipe) | area 0.073 (22 mm x 1.0 m, caps, two sockets), Lc 0.022 |
| housing area | 5.0 | 5.27 (tray outside 2.55, wool top and channel 2.0, walls above the wool 0.22, ledge and cover angle 0.5) |
| stand area | 1.13 (50 mm solid bars) | 0.82 (6.83 m of angle x 0.10 m both faces, braces, cleats, bolts) |
| `provenance` | notes with 26 mm wool, 4 mm glass | notes rewritten en/de; reference `collector-data-sheets` added |

Every area change is within a factor 1.5 (C16) and follows a hand calculation of the 1:50 model; measured outer areas at 1:50 are 5.27 / 3.96 / 3.88 / 0.073 / 0.073 / 0.82.

Unchanged: `dimensions`, masses, materials, poses, clips, `structure`, presentation, summary. No other file was touched (the catalog index carries only unchanged dimensions).

## 4. What was modelled

Solid counts (body / thermal / structure): 1:50 38 / 39 / 36, 1:100 14 / 15 / 23, 1:200 8 / 9 / 12, 1:500 2 / 7 / 9.

**1:50.**
- Collector: folded 1.5 mm tray (floor and end walls with ledge as one section, two side walls with ledge), four clamping cover angles (20 mm leg, 12 mm skirt), 58 mm wool with a channel under each header, 3.2 mm pane on the ledge, 25 mm air gap, the harp as one fin-and-tube section (0.5 mm sheet welded 2.4 mm wide onto nine 10 mm risers), riser ends into two 22 mm headers that end flush in four connection sockets.
- Stand: two rails, four posts and two knee braces of 25 x 25 x 2 mm angle, four folded 3 mm cleats, two crossed 20 x 2 mm flats, 20 M8 bolts.
- Thermal: references per region, outlet patch in the glass under the upper cover leg.
- Structure: four frame rails (no floor, no wool), pane, harp, headers, stand members; cleats, foot and knee bolts, riser ends and sockets as joints; mass discs and lumps; two contact strips.

**1:100.** Tray shell with lips in one section plus two side walls, flat wool slab, pane wall to wall, one 16.4 mm absorber plate, 22 mm headers; stand as 25 mm bars with knee bars, flats slender. Structure uses a frame ring and the nine risers at true size.

**1:200.** Tray with 40 mm floor and 30 mm walls, 30 mm glass and absorber slabs, 30 mm header bars; each stand side one merged frame plus knee bar. Structure: frame ring, glass slab, risers and flats slender.

**1:500.** One slab and two 75 mm side frames. Thermal: housing block, glazing and absorber as their layers, headers slender. Structure: frame ring with pane slab, three risers, two header pipes.

2D: 1:50 section shows tray, hatched wool with channels, glass, sheet on the cut riser, headers, far wall, rail, posts, knee brace, cleats; front shows end wall, cover frame, glass, absorber with nine seams, upper header, front posts; plan the same from above with both headers. Coarser scales merge as their decisions say.

## 5. What I saw

Opened with the Read tool: 1:50 body sheet (six views with rig), 1:50 three-quarter from the front and from behind (turn 235), 1:50 back alone, 1:50 right side in pose `stagnation` with rig, 1:50 thermal cut sheet, 1:50 structure sheet and structure from behind, body sheets of 1:100, 1:200, 1:500, structure from behind at 1:200 and 1:500, thermal cut sheet at 1:500, 2D sheets of 1:50 (candidate), 1:100 and 1:200.

- **1:50 front three-quarter:** a glazed collector in a slim frame with a socket at each corner, on a light angle stand; nothing floats.
- **1:50 from behind:** angle posts with visible legs, cleats with two bolts each gripping the tray back, the X of flats bolted to the rear posts, knee braces down to the rails. This is the view that convinces.
- **1:50 right side with rig:** housing bone on the tray back from the upper cleat to the low edge, glass, headers and absorber bones inside their parts, stand bone along the rail. A 10 mm gap between post top and tray is bridged by the cleat, as built.
- **1:50 thermal cut:** housing, glazing, absorber sheet under the glass, the outlet patch on the upper glass; headers are small dots.
- **1:50 structure:** first render read as a slab (pane on top, floor behind). After removing floor and wool it reads from behind as an open frame with the harp inside; from the front the pane still covers the frame.
- **1:100 to 1:500:** same proportions, recognisable at once. Two defects found by looking and fixed: a side wall 18 mm outside the box at 1:100, and a seam along the sides at 1:200.
- **2D:** collector slopes down to the right (ahead), front view shows the aperture; clean at every scale.

## 6. Still imperfect

- **Masses do not close.** Take-off of the modelled parts: tray and wool about 17 kg (declared 10), absorber 4.6 kg dry (declared 8), glass 15.8 kg (14), stand 6.67 kg (6.4). I left the declared masses (authored estimates, not steered by the model); a reviewer should decide them.
- **Wool thickness** rests on the box geometry and a textbook, not on a data sheet.
- **Glass is opaque in the sheets,** so I never saw the absorber through the pane in a body render, only in the thermal cut.
- **Outlet patch** must be one smallest feature thick (`volumes-feature`), so at 1:500 it is a 75 mm block in the upper end of the slab.
- **Structure from the front** still looks like a slab because the pane is a real member.
- **2D conventions:** sheets and cleats are drawn 7.5 mm thick (air gap reads 15 instead of 25 mm); the riser seams are drawn although the risers lie under the sheet.
- **Omitted real parts:** gasket, vent holes, sensor pocket, rivet nuts in the tray back.
- **Not opened:** 1:100 and 1:200 thermal sheets, 1:100 structure, the `night-idle` 2D sheet and the 1:500 2D sheet were rendered but not looked at.
- During the work one failed apply let `volumes-insert.py` splice new volumes onto the old rig for about a minute; the next write repaired it.

## Round 5 handcraft

No geometry change. Every scale and list rendered and opened (body, structure, thermal cut at 1:50 to 1:500; poses `night-idle`, `operating`, `stagnation` and clip `operating` t 150 at 1:50; 2D 1:50 to 1:500). Reference as in section 1 (2.0 x 1.0 x 0.1 m class, 3.2 mm pane, 22 mm headers, 25 x 25 x 2 mm angle stand); model extents equal `dimensions` (bounds 0 mm).

| scale | before = after (body / thermal / structure) |
|---|---|
| 1:50 | A- / A- / B+ (structure: the pane member covers the frame from the front, it reads as a slab there; from behind an open frame) |
| 1:100 | A- / A- / B+ |
| 1:200 | A- / A- / B+ |
| 1:500 | B+ / B+ / B+ |

### Check: red, not by the model

`volumes-check.ts --product` fails at 1:50, 1:100 (per region) and 1:200 (total): since the C31 flood the glazed box is a closed cavity, so the absorber (3.88 m²) and both headers measure an outer area near 0, glazing 1.99 against 3.96 and housing 2.94 against 5.27. The values are beyond the factor 1.5 an author may apply (C16) and the absorber/header case is a rule conflict (C31 e asks for `hidden`, B4 and the `hidden` text forbid it for a visible part). Logged in `area-review.md` and `tool-requests.md` (2026-11-21); not forced.

## Canonical component readback, 2026-11-21

Canonical source `08a286c22a4637fd`. Two bolted 2 × 20 mm straps cross behind the folded foot channels, posts and knee stays. The sheet shows the straps and their bolts from the rear. Inside the folded tray the 3.2 mm cover, absorber, mineral wool and two actual headers remain distinct. All twelve exported scenes reopened (850,844 bytes). The strict catalog source readback passed.
