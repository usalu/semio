# Round 3: components/glass-balustrade

Scripts: `r3-glass-balustrade-kit.py` (dimensions shared by 2D and 3D), `r3-glass-balustrade-volumes.py` (volumes source), `r3-glass-balustrade-roots.py` (root fixes, `--apply`), `r3-glass-balustrade-crop.ts` (enlarged crops for looking). Tool output under `🗑️generated/r3-glass-balustrade/`, `🗑️generated/r3-sheet/components/`, `🗑️generated/r3-2d/`, `🗑️generated/volumes-tools/render/components/`.

No earlier round-3 work existed for this asset. The catalog `volumes` equalled `volumes/components/glass-balustrade.json` at the start (the bulk pass of 03:41 had added a digital 1:500 of three blocks); all four scales are now authored by hand here.

## 1. Reference

| item | real value | source |
|---|---|---|
| type | barrier glazing clamped along its base with a continuous handrail, category B | DIN 18008-4:2013 (formerly TRAV), from memory, not looked up on the web |
| glass | laminated safety glass of 2 x 10 mm toughened panes with 1.52 mm PVB = 21.52 mm; width 0.5 to 2.0 m, height 0.9 to 1.1 m | DIN 18008-4 annex B table for category B; the asset's own 21.5 mm |
| clamp | steel clamp plates at least 12 mm thick on both faces, clamping height at least 100 mm, bolts through the glass at no more than 300 mm, elastomer interlayers between glass and steel, plastic sleeves in the glass holes, glass standing on setting blocks | DIN 18008-4 annex B |
| chosen clamp | plates 15 x 110 mm, clamping height 100 mm, elastomer strips 10 mm, slot 41.5 mm, setting blocks 10 x 21.5 x 100 mm at 0.65 m from the centre | design within the rule; 10 mm strips so that the fillet weld inside the slot clears the glass |
| bolts | M12: head height 7.5 mm, nut height 10.8 mm, width across flats 19 mm; seven at 300 mm, 80 mm from the glass ends; glass hole 26 mm, steel hole 14 mm; grip 71.5 mm, bolt M12 x 90 | ISO 4014 / ISO 4032, DIN 931 / 934 |
| base plate | steel flat 300 x 15 mm (the asset's `widthM` 0.3), fixed plate welded with two fillet welds a6 (leg 8.5 mm), ten M12 anchor studs at 400 mm, 110 mm either side of the axis | steelwork practice |
| handrail | circular hollow section 42.4 x 2 mm with a 27 mm slot, U gasket 2.75 mm over the glass edge, glass 17 mm into the tube, dished end caps, top at 1.10 m | EN 10219-2; slotted-tube handrail systems |
| heights | handrail top 1.10 m, clamp top 0.125 m, glass 0.025 to 1.08 m (1.055 x 1.96 m) | `dimensions`, DIN 18008-4 |
| check of the clamp plate | handrail load 1.0 kN/m at 1.1 m: M,d = 1.5 x 1.1 = 1.65 kNm/m; one 15 mm plate, W = 37.5 cm3/m, sigma = 44 MPa, far below 235 MPa | hand calculation |
| areas | glass 2 x 1.96 x 1.055 + edges 0.13 = 4.27 m2; tube pi x 0.0424 x 1.984 + caps = 0.27 m2; clamp group 2.24 m2 (measured on the 1:50 model, hand estimate 2.29) | hand calculation, `volumes-check` |

## 2. Audit (before)

- Rig: one fixed bone `panel` at (0, 0), 90 degrees, 1.1 m; section 1 m strip 21.5 mm (A 0.0215 m2, I 8.28e-7 m4, extreme fibre 10.75 mm); mass 59.13 kg = 1 m x 1.1 m x 21.5 mm x 2500 kg/m3 at 0.55 m. Correct. Anchors `base-clamp` (0), `impact-point` (0.9), `handrail-top` (1.1): correct.
- `dimensions` 1.1 x 0.3 x 2.0 m: kept (base plate width, handrail top, panel length).
- Shoe, the root defect: in every drawing and in the 1:50 solid the shoe was a solid steel bar 300 x 100 mm with a 55 mm slot, i.e. two cheeks 122.5 mm thick (about 200 kg of steel per metre). No such part exists. The clamping height was 80 mm (below the 100 mm of the rule). "Gaskets" 16.5 mm wide were bound to material `steel`. Two bolt circles at 100 mm either side of the glass ran along the panel inside the solid cheeks and clamped nothing; the 3D turned them into 300 mm rods across the bar.
- Shoe height disagreed between scales: 100 mm at 1:50 and 1:100, 125 mm at 1:200.
- 1:200 handrail drawn and modelled 53 mm instead of 42.4 mm.
- Solids: 1:100, 1:200, 1:500 were three blocks each (kerb, sheet, bar); the handrail a solid rod with ellipsoid ends; the glass one box; structure referenced glass and handrail and a thin plate under a solid kerb; no joints at 1:100 and coarser.
- Materials: no elastomer and no PVB material.

## 3. Root fixes (catalog, `r3-glass-balustrade-roots.py --apply`)

| path | old | new |
|---|---|---|
| `summary` | steel floor shoe with gaskets and bolts, shown in section | clamp plates, elastomer strips, M12 at 300 mm, anchored base plate, DIN 18008-4 category B, slotted tube |
| `provenance.references` | 13 entries | plus `din-18008-4`, `iso-4014`, `en-10219-2` |
| `provenance.notes` | section note | end-view note, glass 1.96 m, width = base plate |
| `materials` | glass, steel | plus `elastomer` (EPDM strips, blocks, gasket, sleeves) and `pvb` |
| 1:50 elevation | `shoe`, `gasket-a/b`, `glass`, `bolt-a/b`, `handrail` | `base-plate`, `cheek-fixed`, `cheek-clamp`, `weld-outer`, `weld-inner`, `pad-fixed`, `pad-clamp`, `setting-block`, `glass` (0.025 to 1.08), `bolt-head`, `bolt-nut`, `anchor-fixed`, `anchor-clamp`, `handrail`; thin parts with a 0.13 mm pen |
| 1:50 front | shoe 100 mm, five round bolts, glass, handrail, elliptic caps | base plate, clamp plate to 125 mm, weld, seven hexagon heads, five anchor nuts, glass 0.125 to 1.0578, tube, dished caps |
| 1:50 plan | shoe, gaskets, glass, handrail | base plate, weld, both clamp plates, strips and glass under the tube, seven heads and seven nuts, ten anchor nuts, tube with caps, axis |
| 1:100 elevation | block 300 x 100, glass stroke, tube | base plate, two clamp bars, bolt, two anchor studs, glass stroke from the plate, tube |
| 1:100 front, plan | block 100 mm; plan block and tube | shoe 125 mm with seven bolt dots and five studs; plan plate, clamp, bolts, ten studs, tube |
| 1:200 elevation | block 300 x 125, glass line to 1.07, disc 53 mm | inverted T (flange 30 mm, upstand 71.5 mm to 125 mm), glass line to 1.0578, disc 42.4 mm |
| 1:200 front, plan | handrail 53 mm | handrail 42.4 mm; plan gains the clamp strip |
| decisions 1:50, 1:100, 1:200 | texts of the old shoe | rewritten (silhouette, retained, merged, omitted, thin elements, overlays, hatching where they changed) |
| 1:500 `reason` | "balustrade section" | "balustrade end view" |
| `thermal.regions` `panel-glass` areaM2 | 4.29 | 4.27 |
| `thermal.regions` `clamp-steel` areaM2, title | 2.0, "Floor shoe, gaskets and bolts" | 2.24 (factor 1.12, inside C16), "Base plate, clamp plates, elastomer strips and bolts" |
| `thermal.regions` `handrail-steel` areaM2 | 0.27 | 0.27 |

Rig, anchors, dimensions, poses, clips, symbols, structure data: unchanged (correct, and no bone moves in any pose).

## 4. Model

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 23 solids (64 with repeats): base plate; fixed and loose clamp plate with seven 14 mm holes; two fillet welds; two elastomer strips with holes; two setting blocks; two 10 mm panes and the PVB film with seven 26 mm holes; seven sleeves, shanks, hexagon heads and nuts; ten anchor nuts and studs; slotted tube; U gasket; two dished caps | 22: every clamp, glass part and cap by reference; the handrail as the round outer skin of the closed tube | 19: two panes and tube as members; strips, blocks, clamp plates, welds, bolts with heads and nuts, anchors with nuts as joints; base plate as support; mass disc at 0.55 m |
| 1:100 | 10 (24): base plate; two 25 mm clamp bars; one glass body on the plate; seven 20 mm rods; ten 20 mm studs; slotted tube without gasket; two flat plugs | 8: by reference, handrail as one round over 2 m | 9: glass and tube members; clamp bars, bolts, anchors joints; plate support; mass |
| 1:200 | 3: inverted-T shoe, glass sheet over 2 m, round handrail with glass notch | 3 by reference | 5: glass and handrail members, clamp upstand joint, flange support (own solids), mass |
| 1:500 (digital only) | 3: coarse inverted-T foot (75 mm clamp, 37.5 mm flange), glass sheet, square handrail bar | 3 by reference | 3: glass and handrail members, foot as support |

Final checks: `volumes-check.ts components/glass-balustrade --product`: PASS, every rule ok at every scale (1:50 silhouette IoU 0.999 / 1.000 / 1.000, 1:100 0.999 / 1.000 / 1.000, 1:200 1 / 1 / 1; 1:50 areas glass 4.270, clamp 2.243, handrail 0.265 m2). `r3-validate.ts components/glass-balustrade --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced with `volumes-insert.py`.

## 5. What I saw

- 1:50 body, six views and enlarged crops of clamp end, handrail end and the three-quarter corner: reads at once as a bolted clamp balustrade. End view: plate, two plates with strips, both panes with the film line, setting block, both welds on the fixed plate, bolt head on the fixed side, nut with thread stub on the loose side, anchor nut and stud each side, tube over the glass with the gasket lips. Front: seven hexagon heads at 300 mm, five anchor nuts, weld line. Back: seven nuts. Top: plate, ten anchors, clamp plates, tube. Nothing floats, nothing interpenetrates (bolts run in real holes).
- 1:50 thermal with cut: glass red with the film seam, the clamp group green with bolt, sleeve and strips visible in the cut, handrail violet. The front and back panels show a moire on the glass because the cut plane lies in the film; the end view is the readable one.
- 1:50 structure: panes and tube blue, clamp plates, strips, bolts and anchors orange, plate green, mass disc magenta standing 5 mm proud of the glass.
- 1:100 body, thermal, structure: plate, two bars, glass, bolt stubs, studs, tube; a clean reduction, not the 1:50 with parts removed. Structure shows bolts and studs as joints.
- 1:200 body, structure: inverted T, sheet, round handrail; structure with green flange, orange upstand, mass.
- 1:500 body, thermal: a first version with a wedge foot read as a concrete barrier and was replaced by the coarse inverted T, which reads as plate and clamp.
- After the last change every sheet was rendered again and opened: `r3-sheet` body, thermal (cut) and structure at 1:50, 1:100, 1:200 and 1:500, the `volumes-render` overlay sheets of the four scales (overlays fully green at 1:50, 1:100, 1:200; 1:500 has no drawing), the clip frame `impact-pulse` at 0.4 s (no bone moves, the body is unchanged and closed) and the catalog 2D sheets of 1:50, 1:100 and 1:200.
- 2D at 1:50, 1:100, 1:200 (line and solid, with crops): end view with hatched plate and clamp plates, strips, bolt and anchors; the front and plan line views do not draw the filled detail shapes (bolt heads, anchors), as before the change; they show in the solid views.

## 6. Still imperfect

- C31 (closed cavities do not count) is not implemented in the product measure: a hollow closed tube counts its inner wall (0.66 against 0.27 m2). The thermal handrail is therefore its own round solid for the outer skin instead of a reference to the hollow tube; the check reports an `exposureDeviation` of 0.88 for it (information only) because the body tube covers that skin. Filed in `tool-requests.md` (2026-11-20).
- Washers, glass arrises and the anchors inside the slab are not modelled (below 7.5 mm or below ground).
- The DIN 18008-4 values are quoted from memory; no web source was consulted.
- `dimensions.massKg` 59.13 kg is the one-metre glass strip of the structural model, not the 2 m assembly (glass 111 kg, steel about 130 kg); left as it is because the rig mass and the solver use the strip.

None of the points above is a defect of the model; body, thermal and structure are grade A by my own look at every scale.

State: DONE

## Canonical component readback, 2026-11-21

Canonical source `63b1103ae294592f`. Two drilled 10 mm glass panes surround 1.5 mm PVB. Seven cross bolts run through the clamping cheeks. Two separate five-stud rows go through the perforated 15 mm base plate. All twelve exported scenes reopened (1,035,088 bytes). The strict catalog source readback passed.
