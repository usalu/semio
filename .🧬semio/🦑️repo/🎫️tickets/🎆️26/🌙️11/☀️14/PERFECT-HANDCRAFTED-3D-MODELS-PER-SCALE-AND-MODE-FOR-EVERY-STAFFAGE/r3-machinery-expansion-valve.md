# Round 3: machinery/expansion-valve

State: finished and spliced (2026-11-20). No earlier round-3 work existed; the asset was rebuilt from the roots. Everything below describes the catalog file as it is now.

## Final state

- `r3-validate.ts machinery/expansion-valve`: `VALID ... schema 0, validateAsset 0`.
- `volumes-check.ts machinery/expansion-valve --product`: **FAIL: 5 measurement(s) over 2 rule line(s)**, all of them `volumes-thermal-coverage` on declared areas I am not allowed to correct (C16, ratio above 1.5): `body`, `head`, `capillary` at 1:50 and `body`, `head` at 1:100. Every other rule is green at both authored scales (silhouette, bounds, feature, monotonic, cross-scale, structure coverage, bones, slender, copied, views-disagree). The three areas are listed in `area-review.md` (2026-11-20 entry) with hand calculations and the `heatW` consequence.
- 1:200 and 1:500 stay unavailable (the asset is 1 mm and 0.4 mm wide on paper).

## Scripts (all in the ticket folder)

| Script | What it holds |
|---|---|
| `r3-expansion-valve-kit.py` | Measures, bone frames, world/bone conversions, capillary paths (riser, coil, lead), band outline |
| `r3-expansion-valve-model.py` | Every solid of 1:50 and 1:100, body, thermal, structure, and the decisions |
| `r3-expansion-valve-roots.py` | Texts, reference, dimensions, material, rig, anchors, 2D of 1:50 and 1:100, poses, clips, thermal region, structure |
| `r3-expansion-valve-build.py` | Candidate document (and with `--source` the volumes source) |
| `r3-expansion-valve-apply.py` | Writes changed root members into the catalog file in its formatting |
| `r3-expansion-valve-ship.sh` | build, validate candidate, apply, splice, validate |
| `r3-expansion-valve-zoom.ts` | Magnifies the product's 2D SVG panels (after `r3-2d.ts --svg`) |

To resume: `sh r3-expansion-valve-ship.sh`.

## 1. Reference

Subject: a thermostatic expansion valve of the 3 kW class. Reference product: Danfoss T 2 / TE 2, flare x solder, data sheet AI241486443133en-001401 (2021), figures 1 to 3 and table 7, read from the PDF pages (https://assets.danfoss.com/documents/175152/AI241486443133en-001401.pdf).

| Quantity | Real value | Source | Model |
|---|---|---|---|
| Pattern | Angle valve: inlet from below (orifice and filter inside), outlet to the side, element on top, setting spindle under a cap on the side opposite the outlet, inclined downwards | Data sheet fig. 1 | Same |
| Outlet axis to element top | 30 mm | Fig. 2, 3 | 30 mm |
| Outlet axis to inlet end | 35 mm | Fig. 2, 3 | 35 mm |
| Axis to end of solder outlet | 54 mm | Fig. 3 | 54 mm |
| Axis to spindle cap | 35 mm | Fig. 2, 3 | 34.6 mm |
| Element diameter | about 42 mm (scaled from fig. 2: 87 px at 2.06 px/mm) | Measured on the figure | 42 mm |
| Connections | 3/8 in flare inlet, 12 mm (1/2 in) solder outlet | Table 7, table 11 | 9.52 mm liquid line, 19 mm flare nut; 12.7 mm copper stub with 12 mm socket |
| Capillary | 1.5 m | Ordering tables | 1.47 m, 1.6 mm (diameter kept from the asset; the data sheet gives none) |
| Bulb | 14.5 mm x 50 mm, stainless steel, charging nib, fixed with a 0.4 mm band | Fig. 2, features, table 17 | 14.5 x 50 mm plus nib, 10 mm band (0.5 mm modelled) with lock and screw |
| Weight | 0.3 kg | Table 7 | 0.4 kg with nut, three pipe pieces and band |
| What moves | Nothing outside; diaphragm and cone inside | Fig. 1 | All joints fixed |
| Bulb position | On the suction line, on top for small lines | Product practice (my knowledge) | On top of a 16 mm x 64 mm suction line piece |
| Element and capillary material | Stainless steel | Bulb per data sheet; element and capillary from my product knowledge, not verified in the data sheet | New material `stainless-steel` (AISI 304 values, Incropera) |

## 2. Audit of the state before round 3

- Form: a 64 x 50 x 60 mm box with two 18 mm stubs sideways, a 50 mm dome, and a "stem" cylinder on top of the dome. No real expansion valve looks like this; the real body is about a third of that size and T-shaped.
- Rig: `stem` was a prismatic bone on top of the diaphragm head that travelled 8 mm with the flow. The real setting spindle sits on the side of the body under a cap and does not move in operation; the capillary, not a stem, leaves the top of the element.
- Bulb: a 35 mm upright cylinder floating in the air beside the valve (real 50 mm, lying on the suction line); at 1:100 the capillary was dropped and the bulb was unconnected (qa-visual-2).
- Capillary: 0.15 m in three straight bars (real 1.5 m, coiled).
- Support: a ground contact under the body "as a stand-in for pipe clamps"; the real valve hangs in its pipes.
- Dimensions: 0.09 x 0.19 x 0.06 m, 0.45 kg, with no source.
- Structure lists: references to the whole body (D3 defect).
- 2D: rectangles for body and stubs, matching the wrong form.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions` | 0.09 / 0.19 / 0.06 m, 0.45 kg | 0.121 / 0.196 / 0.062 m, 0.4 kg |
| `rig.body` | (0, 0), length 0.064 | (0, 0.071) = valve centre on the outlet axis, length 0.054 (axis to socket end), com (0.0084, -0.0093), 0.24 kg |
| `rig.head` | (0, 0.05), angle 0, length 0.03 | element seat (0, 0.015 in body), angle 90, length 0.015, 0.065 kg |
| `rig.stem` | child of head, prismatic, 90 deg, -8..0 mm | child of body at (-0.012, 0.001), angle 210, length 0.022, fixed, 0.015 kg |
| `rig.bulb` | (0.108, 0.03), angle 0, length 0.035 | capillary entry (-0.057, 0.00025), z 0.024, angle 180, length 0.05, 0.08 kg |
| `anchors` | `base-contact` (contact) | `liquid-line` (pin), `outlet-pipe` (roller), `suction-line` (roller) at the three cut pipe ends, as the pipe-mounted siblings (circulation-pump, thermostatic-radiator-valve) do |
| `structure.supports`, poses and clips contacts | base-contact | the three anchors |
| Poses | `stem` dy -4 / -8 mm | no bone offsets (nothing moves outside); descriptions adjusted |
| Clip `modulate` description | "the stem moves in and out" | the diaphragm follows the superheat, nothing moves outside |
| `materials` | brass, copper | plus `stainless-steel` |
| `thermal.regions` | head brass, bulb and capillary copper | stainless steel; new region `suction-line` (0.0045 m2, -0.8 W, authored estimate); titles of head and inlet; assumptions rewritten |
| `areaM2` body, head, capillary | 0.02 / 0.007 / 0.0006 | unchanged (C16), see `area-review.md` |
| `summary`, `provenance.notes`, `provenance.references` | block with stem | T 2 pattern described; reference `danfoss-t2-2021` added |
| `representations.200/500` | limiting feature "18 mm stubs" | "9.5 mm liquid line" |
| 2D 1:50 and 1:100 | rectangles | redrawn from the same measures as the solids (E1): kept ids `capillary`, `bulb`, `stem`, `head`, `inlet`, `outlet`, `body`; new `inlet-nut`, `outlet-pipe`, `capillary-coil`, `suction-line`, `bulb-strap` |

The liquid line and outlet pipe lengths (28 mm each visible) were chosen so that `inlet` and `outlet` keep their declared 0.0032 m2.

## 4. What is modelled

| Scale | List | Solids | Content |
|---|---|---|---|
| 1:50 | body | 19 | liquid line with flare and bore mark; flare nut (turned skirt and collar, 19 mm hexagon); threaded inlet; forged body as a loft from round neck to rounded square and back; outlet boss; copper stub with swaged socket; soldered pipe; element as one turned profile; capillary riser with spiral, seven-turn coil, lead (three slender tubes, 1.47 m); spindle boss with end cap and 17 mm hexagon; bulb with nib; suction line; band, lock, screw (texture parts) |
| 1:50 | thermal | 19 | one reference per body solid, seven regions, cut pipe ends as section faces |
| 1:50 | structure | 17 | six members (three tubes, body pressure shell, element lower shell, spindle housing), five joints (flare nut, solder socket, element seat, spindle thread, band), three supports, three masses |
| 1:100 | body | 9 | inlet column, barrel, outlet bar, element disc, coil disc, spindle stub, bulb, suction line, true-size capillary lead |
| 1:100 | thermal | 9 | references |
| 1:100 | structure | 10 | column, four members that are their own pipes or housings, one joint, three supports, one mass |

## 5. What I saw

- 1:50 body sheet (six views) and the three-quarter frame of clip `modulate` at 1.5 s: reads at once as a T 2 type valve. Element disc with rim and nipple, coil above it, hexagon cap pointing down-left, flare nut under the body, stepped outlet with socket, bulb banded on the pipe. Parts touch or overlap everywhere.
- 1:50 structure sheet: tubes, shells, orange joints at the real joints, green rollers in the pipe ends, masses inside. The pin in the liquid line end is hidden inside the tube in most views.
- 1:50 thermal sheet with cut: seven regions in the right places; no internals, as no hidden region is declared.
- 1:50 overlay sheet: green except the pen-wide margins of the capillary strokes.
- 1:50 2D, magnified: valve, coil rectangle, lead, bulb on pipe are readable in the solid view; at 2.4 mm on paper the 0.18 mm pen is heavy, so I removed hexagon edges, bores, rim crease, spiral and one of two band lines.
- 1:100 body sheet and structure three-quarter: column, barrel, disc, coil disc, stub, bar; still recognisable.

## 6. Still imperfect

- The check is not green: five thermal-area measurements fail until the area reviewers decide body, head and capillary (and confirm the new suction-line region and the stainless materials).
- The 1:50 front silhouette passes at 0.882 against 0.88; the margin is the pen-wide capillary stroke.
- 1:100 2D: I magnified only the plan; the two elevations I saw only at sheet size (about 30 px) and through the overlay numbers (IoU 0.98 / 0.94 / 0.99).
- I did not open the 1:100 thermal render, and looked at the 1:100 structure from one direction only.
- The riser shows a small pinch where it bends into the spiral.
- At 1:100 the coil is a solid 60 mm disc, not a ring, so the element is hidden in the plan.
- The capillary diameter (1.6 mm) and the element diameter (42 mm, scaled from a figure) are not printed dimensions.
- The suction line piece is a second cut sample beside the valve, tied to it only by the capillary; structurally it has its own roller, but the rig still treats bulb and valve as one rigid body.

## Round 5 handcraft

Script: `r5-expansion-valve-model.py` (`--apply` writes the two root edits and the volumes source; then `volumes-insert.py`). 1:50 kept as round 3 built it; 1:100, 1:200 and 1:500 re-authored by hand. The 1:200 and 1:500 scales exist in 3D only (the coordinator's digital scales, `equipment-digital.py`); their drawings stay unavailable.

### Reference (Danfoss T 2 / TE 2, data sheet AI241486443133en-001401) and model at 1:50

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| outlet axis to element top | 30 mm | 30 mm | 0 % |
| outlet axis to inlet end | 35 mm | 35 mm | 0 % |
| axis to solder outlet end | 54 mm | 54 mm | 0 % |
| axis to spindle cap | 35 mm | 34.6 mm | -1.1 % |
| element diameter | 42 mm (scaled from fig. 2) | 42 mm | 0 % |
| liquid line | 3/8 in = 9.52 mm | 9.52 mm | 0 % |
| outlet | 12 mm solder (1/2 in stub) | 12.7 mm stub, 12 mm socket | 0 % |
| capillary length | 1.5 m | 1.47 m | -2 % |
| bulb | 14.5 x 50 mm | 14.5 x 50 mm | 0 % |
| suction line | 16 mm (asset) | 16 mm | 0 % |

### Grades (body / thermal / structure), before -> after

| scale | before | after | what changed |
|---|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- | unchanged; all poses are geometrically identical (nothing moves outside), `closed` and `open` sheets and clip `modulate` t 1.5 opened |
| 1:100 | B / B+ / A- | A- / A- / A- | the coil was a closed 60 mm puck hiding the element; now an 18 mm ring band with a 30 mm opening, carried by the capillary which rises from the nipple into the band (one slender tube nipple-band-bulb); 2D plan coil redrawn as the same ring (path with hole) |
| 1:200 | B- / B / B | B+ / A- / A- | was the 1:100 list with a tapered inlet; now its own abstraction: straight liquid line with the flare nut as a chamfered collar, element lens with the coil as one closed drum seated on the cap, round-ended bulb, joint as a sphere at the valve centre |
| 1:500 | C+ / C+ / B- | B+ / A- / A- | carrot-shaped inlet, puck, no spindle in the body (stem only in thermal and structure); now a straight tee, element and coil one turned head, spindle stub restored in the body, thermal by reference only, structure column stepped to the barrel |

### Root edits

- `thermal.regions[inlet].areaM2` 0.0032 -> 0.00215 (the 1:50 model's outer area under C31; factor 1.49, within C16).
- `representations.100.geometry.plan[capillary-coil]`: filled circle r 30 mm -> ring path r 30 / 15 mm.

### Checks

`volumes-check.ts machinery/expansion-valve --product`: PASS (was FAIL on the inlet area). `r3-validate.ts machinery/expansion-valve`: VALID.

### Remaining

- 1:50 coil is a tidy helix stacked over the element; real surplus capillary is coiled more loosely (kept: it is the drawn form).
- 1:500 head (lens under drum) is a convention for the coil; the 1:200/1:500 2D are unavailable, so only the 3D carries these scales.
- The 1:100 2D is 1.2 mm tall on paper; I judged it through the plan/elevation shapes, not by eye at print size.
