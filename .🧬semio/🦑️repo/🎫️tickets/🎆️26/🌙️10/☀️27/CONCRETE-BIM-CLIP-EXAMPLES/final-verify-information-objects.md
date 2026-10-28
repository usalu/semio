# 🔍 Final verify: 📊️information and 🧬️objects (Beat1 and Beat2)

Read-only verification. No source file was edited. Crops and scratch scripts are in `T/🗑️generated/final-verify/`.

## 1. Verdicts

| Clip | Verdict | Blocker | Major | Minor |
|---|---|---|---|---|
| 📊️information (`Beat1_DataToInformation`, 21 keys) | **PASS** (with minors) | 0 | 0 | 8 |
| 🧬️objects (`Beat2_ObjectOrientation`, 19 keys) | **FAIL** | 0 | 3 | 7 |

Information passes on the owner bar. Objects fails on three majors: a count without the binding legend (class beat), a kN value that does not follow from the printed numbers (pset beat), and one abstract beat (inheritance) with no Villa Savoye element.

## 2. Evidence and method

- Frames: all 42 information PNGs (21 en, 21 de) and all 38 objects PNGs (19 en, 19 de) were read with the Read tool at 1280 × 720. Source: `T/🗑️generated/frames/Beat1_DataToInformation/` (captured 01:13–01:16) and `T/🗑️generated/frames/Beat2_ObjectOrientation/` (captured 01:16–01:18). Both sets are newer than their scenes (information 23:22, objects 23:10).
- Full-size frames were read first; regions were zoomed for the leader ends (objects `values`), the pinch vignettes (information `pinch`), the load and total labels (objects `pset`, `total`).
- Layout reports (`…-layout.md`): information en 12 strict findings in 7 frames, de 14 in 8; objects 0 in both (the objects strict check was not relied on). All information findings sit at the flagged label or probe text (`5.00 m`, `1.35 m`, `1.05 m`, `0.24 m`, `H−`, `H=`, `H,`, `Hcar`). Zoomed and read in full frame: no clipping, no overlap, text readable; treated as false positives, as in the round-3 scene notes.
- Scene strings checked in code: information `"25 in teaching model*"` (scene line ~765), `"Geschossdecke"` for the first-floor slab (line ~766), `"passt · Annahme"` (line ~345); objects `CLASS_NOUN` for IfcColumn gives `"pilotis"` / `"Pilotis"` with the count and no legend (line 43); `"151.46 kN"` from `quantity(reaction, "kN", 2)` (line ~479), with no load intensity printed; ObjectPlacement `f"({x}, {y}) m"` through `number()` (line ~756), which yields `(10,75, 4,75)` in German.

### Arithmetic, as printed

| Check | Result |
|---|---|
| Information: 4 × 4.75 = 19.00; 19.00 + 2 × 1.25 = 21.50 | holds |
| Information: 4.75 − 0.30 = 4.45; 1.35 + 1.75 + 1.35 = 4.45; 1.75 + 2 × 0.50 = 2.75 ≤ 4.45 | holds |
| Information: threshold 1.75 + 0.30 + 2 × 0.50 = 3.05; sensitive 2.05 + 0.60 + 2 × 0.50 = 3.65; margins 1.05 + 2.05 + 1.05 = 4.15 = 4.75 − 0.60 | holds |
| Information: 3.50 − 0.25 = 3.25; 0.03 + 0.24 + 0.03 = 0.30; 18.50 × 1.10 = 20.35; 21.50 × 3.00 = 64.50; 20.35 / 64.50 = 31.6 % | holds |
| Objects: π · 0.15² = 0.07069 m²; 0.07069 × 3.25 = 0.2297 m³; 25 × 0.2297 = 5.7425 ≈ 5.74 m³ | holds |
| Objects: 380.42 × 0.25 = 95.105 m³ (bar 95.1); 20.35 m² and 64.50 m² in the window beat | holds |
| Objects: 15.53 m² × 9.75 kN/m² = 151.42 kN, printed 151.46 kN, intensity not printed | **fails as printed (O-M2)** |
| STEP attribute counts: IFCCOLUMN 9, IFCRELDEFINESBYTYPE 6, IFCEXTRUDEDAREASOLID 4, IFCCIRCLEPROFILEDEF 4, IFCPROPERTYSINGLEVALUE 4, IFCPROPERTYSET 5; GlobalIds 22 characters from the IFC alphabet | holds |

## 3. Defects

Severity: blocker = breaks a binding rule (agent rules, coordinator decision) in a way a reader would be misled by; major = visibly wrong or a clear gap against the owner bar; minor = wording, polish, evidence gap.

### 📊️information

| Key | Severity | Location | Defect | Fix |
|---|---|---|---|---|
| I-m1 | minor | `relations`, EN "25 in teaching model*" (line ~765), DE "25 im Lehrmodell*" | Coordinator wording is "25 in this teaching model", with the legend "sources differ: 15–25" (the legend is present and correct). DE also differs from the objects clip ("in diesem Lehrmodell"). | EN "25 in this teaching model*", DE "25 in diesem Lehrmodell*". |
| I-m2 | minor | `relations`, DE: type "Piloti-Stütze" (EN "Piloti"); "Geschossdecke" (EN "First-floor slab") | Type name differs between languages. The German value drops the level, so the slab is no longer the first-floor slab. | DE type "Piloti"; DE slab "Decke über dem Erdgeschoss" (1. OG). |
| I-m3 | minor | `chain`, row "Building box 21.50 m", chip "sourced" (carry-over V-m6) | Research 2.1 is medium confidence, one source. The sub-line says so, but the chip says "sourced". | Use a "one source" chip in the inferred tone (EN and DE). |
| I-m4 | minor | `pinch`, narration EN/DE "that margin" (carry-over V-M2) | The 0.50 m margin is defined as the straight-driving rule in `fits`. `pinch` does not name it, so the reader must infer why 0.42 m and 0.24 m are acceptable. | Narration: "On the curve, two pilotis come closer than the 0.50 m straight-driving margin; the car still clears them, but only just" (DE equivalent). |
| I-m5 | minor | `drive`, "Distance driven 25.3 m" (carry-over V-m8) | The path has unlabelled ticks every 5 m. The 25.3 m value has no visible measure. | Label the ticks 0 to 25 m, or add a ruler that grows with `drive_t`. |
| I-m6 | minor | `fits`, DE chip "passt · Annahme" (scene line ~345) | Every other chip in the clip uses "angenommen". | "passt · angenommen". |
| I-m7 | minor | `diameter` (magnified panel), `drive` (right column) | Lower part of the right panel is empty (`diameter`: below about y 420 of 720; `drive`: under the legend). | Enlarge the bay or move the legend down. Cosmetic. |
| I-m8 | minor | All beats with "IfcColumn", "IfcRel…", "GlobalId" (carry-over MIN-8) | Capital I and lowercase l look the same in the shared display font: "IfcRelContainedInSpatialStructure" reads "lfcRel…", "GlobalId" reads "Globalld". The IFC names are the subject of the clip. | Use a font with distinct I and l for IFC names and GlobalIds (the STEP mono face works), or raise it with the toolkit owner. |

### 🧬️objects

| Key | Severity | Location | Defect | Fix |
|---|---|---|---|---|
| O-M1 | major | `class`, EN "25 pilotis" (model count via `CLASS_NOUN`, line 43), DE "25 Pilotis". No star, no legend. | Coordinator decision: any count on screen reads "25 in this teaching model" with the star and the legend "sources differ: 15–25". Here 25 is a bare count. This is the only count without the legend in the clip. | Leaf text "25 in this teaching model*" (DE "25 in diesem Lehrmodell*") and the legend line under the tree. |
| O-M2 | major | `pset`, load label "151.46 kN*" (EN and DE; line ~479) | Printed area 15.53 m² and printed kN do not agree: 15.53 × 9.75 = 151.42 kN. The load intensity is not printed, so a reader cannot reproduce 151.46 kN from the screen. Owner rule: arithmetic true as printed. | Print the intensity and the product at matching precision, for example "15.534 m² × 9.75 kN/m² = 151.46 kN" (DE "15,534 m² × 9,75 kN/m² = 151,46 kN"), or print the kN from the printed area ("151.42 kN"). |
| O-M3 | major | `inheritance`, EN and DE end frames: class chain and attribute names only | This is the only frame in either clip with no Villa Savoye part. It breaks "nothing is abstract" and "never a bare box". | Show B3 beside the attribute table (plan dot with its grid position and the B3 axonometric), or merge the beat into `values`. |
| O-m1 | minor | `occurrences` (material "Reinforced concrete", no chip); `methods` (r, h, Ø with no basis chip) | Information clip marks material "assumed"; `geometry` marks r, h, Ø. Repeated values lose their basis on the way. | Add the basis chip (same wording as the geometry beat) to the occurrence card and the method formulas. |
| O-m2 | minor | `change` (EN and DE) | Fresh capture shows only the reverted state (Ø 0.30 m, total 5.74 m³). The 0.40 m mid-state (total 10.21 m³) appears only in an earlier capture. The live change is not evidenced in the frames read. | Hold a mid-state key or capture the 0.40 m state into the frame set. |
| O-m3 | minor | `values` DE, placement "(10,75, 4,75) m" (line 756) | The comma separates coordinates and also marks decimals. | German list separator: "(10,75; 4,75) m". |
| O-m4 | minor | `geometry` and `opening`, DE "Lehrannahme" (lines ~593, ~918) | Not a German term in common use. | "Annahme des Lehrmodells" (or "Lehrmodell-Annahme"), same in both places. |
| O-m5 | minor | `spatial` section: ramp zigzag and roof block unlabelled (carry-over MIN-11) | The ramp and the roof block are drawn without a label, and the narration does not mention them. | Label "Rampe" (DE "Rampe") and the roof block, or remove them from this section. |
| O-m6 | minor | All beats with "IfcColumn", "GlobalId", "IfcRel…" (carry-over MIN-8) | Same I / l legibility problem as I-m8. | As I-m8. |
| O-m7 | minor | Count legend, `instance`, `occurrences`, `total`, `change` (EN "in this teaching model", DE "in diesem Lehrmodell") | The objects wording is the decision wording. The information clip uses a different DE phrase ("im Lehrmodell", see I-m1). | Keep the objects wording and align the information clip to it. |

## 4. Status of prior verdicts

### 📊️information (`verify-r2-information.md`, 3 blockers, 5 majors, 14 minors)

| Item | Status |
|---|---|
| V-B1 piloti count | Resolved. Count shown as 25 with star and legend; only wording (I-m1). |
| V-B2 "the Savoyes' car" | Resolved. "a car (size assumed)" in EN and DE. |
| V-B3 units on numbers | Resolved. Units on all dimensions, margins, chain terms, formulas. |
| V-M1 slab leader in empty hatch | Resolved. Leader ends on the E/3 piloti head. |
| V-M2 pinch vs margin | Partly. The clip no longer contradicts itself, but the rule is not named (I-m4). |
| V-M3 basis hard-coded | Resolved. Chips follow the model (material assumed, LoadBearing inferred). |
| V-M4 unlabelled remainder and totals | Resolved. 1.25 m per side, 19.00 m and 21.50 m labelled. |
| V-M5 poché and section mark | Resolved. Section mark A–A and poché present. |
| V-m1 trunk and leader overlap | Resolved. |
| V-m2 leader through north arrow | Resolved. |
| V-m3 gauge ticks | Resolved. Ticks 0 to 5 m. |
| V-m4 1.07 rounding | Resolved. Now 1.05 m with Ø 0.60 m (car 2.05 m). |
| V-m5 Ø "assumed" chip | Resolved. |
| V-m6 box length chip | Open (I-m3). |
| V-m7 count note placement | Acceptable. Status is in the row chip; legend below the table. |
| V-m8 drive ruler | Open (I-m5). |
| V-m9 turning circle tag | Intentional (round-3 decision). Not counted. |
| V-m10 spacing before units | Resolved. |
| V-m11 IfcGrid | Resolved. Class name dropped. |
| V-m12 "+3.50 m" basis | Resolved through the caveat and the assumption colour. |
| V-m13 height tracker | Optional. Not counted. |
| V-m14 model note | Not on screen. Not counted. |

### 🧬️objects (`verify-r2-objects.md`, 0 blockers, 7 majors, 18 minors)

| Item | Status |
|---|---|
| MAJ-1 tributary as type property | Resolved ("occurrence value, B3 alone; all pilotis 8.61–22.56 m²"). |
| MAJ-2 values narration | Resolved (six have values, three empty; frame matches). |
| MAJ-3 Pset on occurrence in STEP | Resolved (`IFCCOLUMNTYPE` HasPropertySets, exchange chip matches). |
| MAJ-4 GlobalId leader | Resolved. Verified in a zoom: leader runs along the grid 3 guide and ends on the B3 ring. |
| MAJ-5 opening basis | Resolved (star and footnote "ribbon size, wall height"). |
| MAJ-6 count headline 25 | Mostly resolved; the class beat still has a bare count (O-M1). |
| MAJ-7 LoadBearing without drawn meaning | Load arrows drawn; the kN value still does not follow from the print (O-M2). |
| MIN-1 60 vs 36 | Resolved ("+ 24 other objects"). |
| MIN-2 Ø and h notes | Resolved (student-slide footnote). |
| MIN-3 unverified IFC [M] claims | Open; not checkable from frames (see section 5). |
| MIN-4 one axonometric style | Intentional (x-ray in instance, ghost in total). Not counted. |
| MIN-5 GlobalId chips at end state | Resolved (three GlobalIds in `schema`). |
| MIN-6 exchange tokens | Resolved (elbow leader to the GlobalId chip). |
| MIN-7 relation arrow labels | Resolved (`IfcRelAggregates`). |
| MIN-8 I / l font | Open (O-m6). |
| MIN-9 instance leader across the villa | Resolved (short leader to chip). |
| MIN-10 ribbon on all faces | Resolved ("ribbon windows: extent disputed"). |
| MIN-11 unlabelled solarium and ramp | Solarium labelled; ramp still open (O-m5). |
| MIN-12 dimension ticks through labels | Resolved. |
| MIN-13 DE plan title on bubble | Resolved ("Grundriss EG"). |
| MIN-14 DE text against box | Resolved. |
| MIN-15 "V = V ✓" | Resolved ("0,2297 m³ = 0,2297 m³"). |
| MIN-16 pset hatch without plan label | Resolved. |
| MIN-17 DE terms | Partly. "Piloti-Stütze" and English IFC identifiers in the cards kept by round-3 decision; not counted. |
| MIN-18 stale distinction heading | Resolved ("Is-a and has-a" / "Ist-ein und hat-ein"). |

## 5. Not counted (intentional or outside the frames)

- Information `raw` and `guess`: bare number 4.75 and unit candidates. This is the point of the first two beats ("This number is data. Alone…"). The earlier verdict asked the owner to decide whether the literal rule "never a bare number" applies here. Still open for the owner; not counted.
- Information `wall` shows only symbols (R_glass, R_turn, b_car, c) and no radius. Intentional: radii not sourced.
- Objects `pset`: "FireRating R 60 assumed" is a teaching value (fire rating not in research). Shown as assumed.
- Objects `representations` card and `exchange` STEP: IFC names in English inside the German clip (kept by round-3 decision).
- Residual IFC risk, not a frame defect: research §16 items still open against IFC 4.3.2.0 (IfcBuiltElement as direct supertype, IfcMaterialLayerSetUsage on walls, Qto_ColumnBaseQuantities, IfcRelAssociatesMaterial, and the RepresentationType "Curve3D" shown for Axis, which the research table does not list). Confirm before the final render.
- Layout strict findings in information: judged false positives (see section 2).

## 6. Files

- Report: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/final-verify-information-objects.md`
- Frames: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/frames/Beat1_DataToInformation/` and `…/Beat2_ObjectOrientation/`
- Crops and scripts: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/final-verify/` (`crop-final.py`, `crop-batch.py`, `crop-b.py`, `obj-en-values-leader.png`, `c1` to `c7` PNGs)

## 7. Minimal re-check after fixes

Re-capture objects en and de (19 keys each). Check: `class` legend, `pset` kN derivation, `inheritance` villa element, `values` German separator, `change` mid-state. Re-capture information en and de only for `relations`, `chain`, `pinch`, `drive`, `fits`.
