# Round 3: machinery/evaporative-humidifier

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset, so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

## Final state

- `volumes-check.ts machinery/evaporative-humidifier --product`: **PASS** (4.5 s) at 1:50, 1:100 and 1:200.
- `r3-validate.ts machinery/evaporative-humidifier --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Print suitability on the office laser profile (`r3-plate-heat-exchanger-suit.ts` on the candidate): suitable at all three scales.

## Scripts (ticket folder)

| Script | What it holds |
|---|---|
| `r3-evaporative-humidifier-geo.py` | Measures of the real unit, bone frames, shared outlines |
| `r3-evaporative-humidifier-volumes.py` | Every solid of every scale and list, stated by hand |
| `r3-evaporative-humidifier-roots.py` | Rig, anchors, texts, thermal data, sections and the 2D of 1:50, 1:100, 1:200 |
| `r3-evaporative-humidifier-build.py` | Writes the candidate `🗑️generated/r3-evaporative-humidifier/🔣️.json` and the source `volumes/machinery/evaporative-humidifier.json` |
| `r3-evaporative-humidifier-apply.py` | Writes the changed root members into the catalog file, keeping its formatting |
| `r3-evaporative-humidifier-ship.sh` | Build, validate the candidate (stops when invalid), suitability, apply, splice, check |

To resume: `./r3-evaporative-humidifier-ship.sh` does all steps in order (the candidate under `🗑️generated/r3-evaporative-humidifier/` was deleted at the end and is rebuilt by it), then look at the sheets.

## 1. Reference

The subject is the pad-type evaporative humidifier (adiabatic cooler) section of a compact air handler of the 1000 m3/h class.

| Quantity | Real value / practice | Source | Model |
|---|---|---|---|
| Principle | Air passes horizontally through a wetted cross-fluted media cassette; water is pumped from a tank under the cassette to a distribution header on top and trickles down through the media; no aerosol | ASHRAE Handbook (asset reference `ashrae-2021`), VDI 3803-1 (`vdi-3803-2010`), Munters FA6 manual | Air along +x through the pad; tank, cassette, header trough, pump, riser |
| Media depth against efficiency | Munters FA6-65 / FA6-85 / FA6-95: cassette depth 100 / 200 / 300 mm for 65 / 85 / 95 % saturation efficiency | Munters FA6 user manual (manualslib.com/manual/1260680/Munters-Fa6.html, dimension table, read 2026-11-20) | The asset declares 90 % efficiency: pad depth 0.2 m (was 0.1 m in the notes and 0.4 m in the drawing) |
| Media | Cross-fluted cellulose or glass-fibre sheets, flute height about 7 mm, flute angles 45/15 degrees, sheets vertical and parallel to the air flow | Munters CELdek 7090 product sheet (depths 100 and 150 mm listed), product knowledge | Pad block with vertical corrugation on both faces (15 mm pitch at 1:50 = two sheets per rib) |
| Face velocity | 1.5 to 3 m/s; below about 3 m/s no droplet separator | ASHRAE, Munters | 0.28 m3/s over 0.36 x 0.40 m = 1.9 m/s; no droplet separator |
| Wet media density | about 90 to 110 kg/m3 wet | product knowledge | 0.36 x 0.40 x 0.20 m = 0.0288 m3, 3 kg declared = 104 kg/m3 (with 0.1 m depth it would be 208 kg/m3: the declared mass fits 0.2 m) |
| Installation length | FA6 dimension C = 630 mm for 100 and 200 mm cassettes | Munters FA6 manual | 0.5 m casing (asset `dimensions`, unchanged): 0.13 m upstream, 0.24 m header, 0.13 m downstream |
| Tank | Welded tray under the cassette over the whole section length, water 30 to 60 mm deep, float valve for make-up water, drain; pump stands in the tank | Munters FA6 manual (drain, 1/2 inch supply), product knowledge | Tank = lower 70 mm of the casing; 6 L declared = 32 mm of water over 0.48 x 0.398 m |
| Bypass sealing | Air must not pass beside or over the media: blanking frame around the cassette | VDI 3803-1, product knowledge | Bulkhead frame on the inlet side with a window on the pad face and a reveal to the cassette, dipping into the water as a seal |
| Casing | Galvanised sheet 0.8 to 1 mm, duct connection frames of 30 mm profile, removable service panel with quarter-turn fasteners | VDI 3803-1, product knowledge of rectangular duct construction | 1 mm shell (real thickness, hollow profile), 30 x 10 mm connection frames, service cover with four fasteners, two base runners of 40 x 20 x 2 mm hollow section with end plugs |
| Envelope | 0.5 x 0.4 x 0.6 m, 13.8 kg with full tank | asset `dimensions` | unchanged |

The web search gave the Munters FA6 depth table only; the CELdek sheet and two brochures answered 404, so flute height, wet density and face-velocity limits are from my own knowledge and marked as such.

## 2. Audit of the state before round 3

**Rig**
- `casing` started at the base centre and ran 0.6 m along +x: 0.35 m of the bone lay outside the 0.5 m casing.
- `pad` (0.4 m), `trough` (0.4 m) and `water` (0.44 m) also started on the centre line and ran to the right, so each stuck out of its part on one side.

**Construction (the root defect)**
- The pad was a block of 0.40 (air direction) x 0.36 x 0.40 m. The notes say 0.1 m deep, the drawing 0.4 m; `author-requests.md` line 160 already reported the views disagreeing. A real cassette for the declared 90 % is 0.2 m deep (reference table).
- The trough covered the whole plan (0.42 x 0.38 m) and nine drops fell through a solid block.
- The casing was a box with a window in each end and an open top frame (qa-visual-2: "does not read as a humidifier"). No tank, no pump, no bypass seal, no service access, no base.
- The water body was 0.44 x 0.38 x 0.05 m = 8.4 L against 6 L declared.

**2D**
- All three views drew internals inside an unfilled casing outline without being declared sections; the plan showed the trough through a closed top.
- "0.5 mm drops" in the 1:500 reason (they are 20 mm).

**Lists**
- `structure` at every scale referenced the whole body (D3 defect; qa grade C).
- `thermal` casing measured 2.08 m2 against 1.5 declared (inner faces of 10 mm panel boxes); the check FAILED at 1:50 and 1:100 before round 3.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.casing` | (0, 0), length 0.6, com (0, 0.3) | (-0.25, 0) base edge at the inlet, length 0.5, com (0.25, 0.26) |
| `rig.pad` | (0, 0.08), length 0.4, com (0, 0.21) | world (-0.1, 0.08) = lower edge of the inlet face, length 0.2 (pad depth), com (0.1, 0.2) |
| `rig.trough` | (0, 0.48), length 0.4, com (0, 0.03) | world (-0.12, 0.48), length 0.24, com (0.12, 0.025) |
| `rig.water` | (0, 0.02), length 0.44, com (0, 0.03) | world (-0.24, 0.02), length 0.48, com (0.24, 0.017) |
| `rig.drip` | (0, 0.02) in the trough | (0.12, 0.02) in the trough = the same world point (0, 0.5); poses and clips unchanged |
| Bone titles | part names | say where the bone runs |
| `anchors.base-contact` | (0, 0), footprint 0.5 x 0.4 | (0.25, 0) = same world point, footprint 0.42 x 0.4 (over the runners) |
| `anchors.outlet` | (0.25, 0.3) | (0.5, 0.3) = same world point, the outlet frame |
| `thermal.regions.casing.areaM2` | 1.5 | **1.07** (factor 1.40, within C16): four outer panel faces 0.98 minus runner contact 0.03, runners 0.07, frame reveals 0.03, fasteners and fittings; measured 1.069 |
| `thermal.regions.trough.areaM2` | 0.5 | **0.35** (factor 1.43, within C16): channel 0.24 x 0.38 x 0.05 inside and outside 0.27, end caps and rim 0.05, riser hose 0.03; measured 0.348 |
| `thermal.regions.pad.layers[0].thicknessM` | 0.1 | 0.2 (Munters FA6 depth for the 85 % class) |
| `thermal.regions.sump.layers[0].thicknessM`, `characteristicLengthM` | 0.05, 0.44 | 0.032, 0.48 (6 L over the tank floor) |
| `thermal.regions.trough.characteristicLengthM` | 0.42 | 0.38 |
| `structure.assumptions` | stands on its sump tray, 0.5 x 0.4 | stands on two runners, 0.42 x 0.4; pad and trough on the rails, water on the tank floor |
| `summary`, `provenance.notes` (en, de) | 0.1 m pad, pump not drawn | the real unit as modelled, the three sections |
| `provenance.references` | 3 | added `munters-fa6` |
| `representations.*.sections` | none | A-A (elevation, z = 0), B-B (front, x = 0), C-C (plan, y = 0.56) |
| `representations.100.pens.detail` | null | 0.13 (the hose stroke) |
| `representations.500.reason` / `limitingFeature` | "0.5 mm drops" | "20 mm drops and 3 mm trough wall" |

`dimensions` (0.5 x 0.4 x 0.6 m, 13.8 kg) and the bone masses are unchanged.

### 2D at 1:50, 1:100 and 1:200 (ruling E1)

Redrawn by hand from the same measures as the solids (`geo.py`). Kept ids: `casing`, `sump-tray` (now the two cut tank end walls), `water`, `pad`, `trough`, `film`, `drop-1..9`. New ids: `runner-*`, `end-frame-*`, `pump`, `riser-hose`, `float-ball`, `float-valve`, `pad-rail-*`, `bulkhead`, `pad-reveal`.

- **A-A (elevation):** far panel as the light casing fill, connection frames with the cut tank walls, runners, water with pump, hose stroke and float ball, rails, bulkhead stroke and reveal, the 0.2 m pad with a 45 degree cross hatch (cut along its sheets), trough with film, three drops (inlet face, inside, outlet face).
- **B-B (front):** inlet runner, water, inlet rail, pad with the vertical hatch (cut across its sheets), trough with film, three drops.
- **C-C (plan):** frames, the two open water surfaces, bulkhead, trough with film and nine drip points, pump with hose, float valve and ball.
- The product sorts by depth: in pose `running` the drops are under the trough and hidden in the plan; in `dry` all nine show in the trough (checked with `r3-2d.ts --pose dry`).
- 1:100 drops the frames, film (elevations), float valve; five drops in a plus. 1:200 keeps casing, water, bulkhead, pad, trough.
- All eight decision texts per scale rewritten (en, de). Silhouette IoU against the 3D in the final check: 1 in all three views at all three scales (the 1:50 elevation was 0.995 while the runners were open channels).

## 4. Model per scale (body / thermal / structure solid counts)

### 1:50 (32 / 23 / 20)

**Body**
- Two runners of 40 x 20 x 2 mm hollow section with plastic end plugs.
- Casing shell: top, far side and floor as one folded 1 mm profile; service cover as a 1 mm pan with 10 mm returns and four quarter-turn fasteners; two connection frames 30 mm wide and 10 mm deep whose bottom bars (70 mm) are the tank end walls; make-up water and drain fittings on the far wall.
- Tank water 0.48 x 0.398 x 0.032 m (6.1 L).
- Two 20 x 30 mm angle rails of 2 mm; bulkhead frame from the water to the top panel with a 0.34 x 0.375 m window; reveal ring from the window to the cassette.
- Pad 0.36 x 0.40 x 0.20 m as one profile with 24 vertical flutes on each face; two cassette C-channels of 1 mm.
- Trough: 3 mm channel with anti-splash rim, two end caps with rim, 10 mm water film.
- Pump (revolved: strainer foot, motor body, shoulder, nipple), riser hose (slender loft Ø 16 over the trough rim), float valve, lever and ball.
- Nine drops on the drip bone.

**Thermal**: four 1 mm casing skins (layer `casing-skin`, inner faces marked), frames (both faces marked), runners as closed bars, fasteners, fittings; pad, film, drops; trough, end caps, hose; water; outlet disc Ø 0.3 m flush in the outlet frame.

**Structure**: runners, tank floor (member of `water`), four folded longitudinal edges 20 mm, two connection frames, two rails with four fixings, two cassette channels (members of `pad`) on two seat joints, trough channel on two seats, masses of water, pad and casing, contact pads under the runners.

### 1:100 (19 / 17 / 18)

Closed 1 mm shell, 15 mm frames, solid runner bars, rail bars, 15 mm bulkhead and reveal ring, pad with 12 flutes, trough as floor slab and 3 mm wall ring with film, plain pump cylinder with hose, float ball, five drops. Structure with 30 mm edges and two cassette posts.

### 1:200 (9 / 12 / 12)

30 mm base slab, shell, 30 mm frames, 40 mm water slab, plain pad block from wall to wall (rails, reveal, cassette, side gaps merged), 30 mm bulkhead, trough floor slab and wall ring from wall to wall. Structure with 45 mm edges, posts, one seat, two masses.

`decisions`: 4 / 2 / 2 bilingual sentences. Monotonic, copied, slender, cross-scale all pass.

## 5. What I saw in the final renders

Opened with the Read tool: `r3-sheet` body, body `--cut`, thermal `--cut`, structure at 1:50; body, body `--cut` at 1:100; body `--cut`, structure at 1:200; a single three-quarter from the inlet side (turn 300); clip `drip` at t = 0.9 s; `r3-2d` at all three scales and in pose `dry`; `volumes-render` overlay at 1:50. Not opened: 1:100 thermal and structure sheets, 1:200 thermal sheet, the overlays of 1:100 and 1:200 (rendered, judged by the check numbers only).

**1:50 body**
- Front (outlet, +x): connection frame, the fluted outlet face with drops, the trough across the top, the yellow riser hose from the pump top, the tank wall below.
- Right side (service side): plain cover with four fasteners, runner ends with plugs.
- Back (inlet): frame, bulkhead with the window on the fluted face and three drops, tank wall.
- Left side: plain wall with the two fittings. Top: plain panel.
- Three-quarter from the inlet: reads at first sight as a duct humidifier section; float valve, lever and ball stand in the tank in front of the bulkhead.
- Rig: every bone lies inside its part.

**1:50 cut**
- Side: pad between reveal and outlet, trough on top, hose over the rim, pump and float ball in the water, drops on both faces and in the middle.
- Top cut: the pad looks hollow and the rails show through it. That is the viewer not capping cut solids, not the model.

**1:50 thermal cut**: red casing skins, green pad with film and drops, purple trough and hose, olive water. Nothing pokes out. The outlet disc is not visible in the cut sheet; it shows in the `volumes-render` cutaway as a green half disc in the outlet.

**1:50 structure**: a blue edge frame with two end rings on the runners, the tank floor, the cassette plates on orange seats over the rails, the trough channel, magenta masses, green pads. A load path, not the body.

**1:100**: same reading with coarser flutes, plain pump, five drops. **1:200**: casing, water, pad block, bulkhead, trough; structure frame with posts.

**Clip `drip` t = 0.9 s**: the nine drops ride down the faces together, everything else stays.

## 6. Still imperfect

- **Casing mass.** 1 mm sheet for shell, cover, frames, bulkhead and cassette weighs about 9 kg; the asset declares 4 kg for the `casing` bone. Not changed (no cited reference for this unit); listed for the physics reviewers.
- **Pad depth and efficiency.** 0.2 m is the Munters 85 % class; the asset's 90 % was kept. The pad layer `resistanceM2KW` (0.5, "illustrative") was not rescaled with the thickness.
- **Sump thermal layer** thickness follows the 32 mm water depth; its `heatCapacityJM2K` (125000 = 30 mm of water) was already consistent and stays.
- **Stainless tank.** The notes used to call the tray stainless; the asset has no stainless material, so tank and cassette are bound to `galvanised-steel`.
- **Middle drop row** sits inside the pad block (water inside the media). It is only seen in the sections.
- **Drops and the reveal.** For a few clip frames near the top the inlet-face drops pass through the 7.5 mm top bar of the reveal.
- **Mass markers.** Pad and casing centres of mass are 20 mm apart, so the two markers interpenetrate at 1:50.
- **Sheet at scale floor.** Bulkhead and reveal are 7.5 / 15 / 30 mm instead of 1 mm; shell, cover, cassette, trough, runners and rails keep real walls.
- **C31 workaround.** The product has no flood for enclosed faces, so casing skins and frames carry `sectionFaces`, and the runners are closed bars in `thermal`.
- **1:200 base.** Support pad and base slab occupy the same 30 mm volume.
- **Area corrections** casing 1.5 to 1.07 and trough 0.5 to 0.35 are within factor 1.5 and listed here for the area reviewers; `exposedFraction` of the poses was not touched (measured exposure of the casing 0.98 against the pose's 0.81).
- **Not run:** the `repo` test targets of the staffage package and the model exports.

## Round 5 handcraft (2026-11-21)

Script: `r5-evaporative-humidifier-build.py` (runs the round 3 build, then the round 5 fixes); apply and splice with `r3-evaporative-humidifier-apply.py` and `volumes-insert.py`. Sheets: `r5-m03-render.sh evaporative-humidifier`.

### Reference vs model (1:50)

| quantity | reference | source | model | deviation |
|---|---|---|---|---|
| cassette depth for 85-90 % saturation | 200 mm | Munters FA6 manual, dimension table | 200 mm | 0 |
| pad face | 0.36 x 0.40 m class | asset, VDI 3803-1 | 0.36 x 0.40 m | 0 |
| installation length | 630 mm (FA6 C) for the duct unit; this section 0.5 m | Munters FA6, asset | 0.50 m | 0 to asset |
| face velocity | 1.5-3 m/s, no eliminator below 3 | ASHRAE, Munters | 1.9 m/s | in band |
| tank water | 6 L | asset | 0.48 x 0.398 x 0.032 m = 6.1 L | +1.7 % |
| envelope | 0.5 x 0.4 x 0.6 m | asset | 0.5 x 0.4 x 0.6 m | 0 |

### Grades before -> after (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / B (two interpenetrating 90-200 mm mass ellipsoids hid the cassette seats) | A- / A- / A- (20 mm markers at the two centres of mass) |
| 1:100 | A- / A- / B+ | A- / A- / A- |
| 1:200 | B+ / B+ / B+ | B+ / B+ / A- |
| poses | running (default), dry, wetting and `drip` t = 0.6 s opened: unchanged, closed | unchanged |

Opened: r3-sheet body, thermal --cut, structure (before and after) at 1:50; body at 1:200. Rendered but not opened this round: the 1:100 sheets, 1:200 thermal and structure, poses dry and wetting and the `drip` t = 0.6 s frame (round 3 opened `dry` and `drip`).

### Changes

- `structure` masses: 1:50 `mass-pad` and `mass-casing` radius 10 mm (were 30 x 60 x 100 and 45 mm), 1:100 `mass-casing` 20 mm (50), 1:200 30 mm (55).
- `representations.200.validRange.maxDenominator` 350 -> 280 (the 40 mm water slab prints at 0.2 mm there; 1:300 measured 0.198).

Check: `r3-validate.ts machinery/evaporative-humidifier --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0).

### Remaining

- Drops are 20 mm marks on the pad faces (the asset's `drip` clip); real water runs inside the media.
- The round 3 open points (casing mass 9 kg against 4 kg declared, sheet at the scale floor for bulkhead and reveal) stay.
