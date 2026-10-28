# Final Verify: Beat3_SameObject (👁️representations) and Beat4_PartialModels (🧩️models)

Read-only verification. No source file was edited. Scenes: `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py` (class `Beat3_SameObject`) and `…/🧩️models/🐍️.py` (class `Beat4_PartialModels`). Frames read: `T/🗑️generated/frames/Beat3_SameObject/` (18 keys × en, de = 36 PNGs) and `T/🗑️generated/frames/Beat4_PartialModels/` (19 keys × en, de = 38 PNGs). All 74 PNGs were read with the Read tool, and the en and de layout reports (strict blocks) were read for both clips. Zoom crops are in `T/🗑️generated/final-verify/`. Values were checked against `🧱️model/🔣️.json` (values and basis flags), `T/research-villa-savoye.md`, `T/research-ifc.md` and the coordinator decisions in `T/fleet-r2.md`.

## Verdicts

| Clip | Verdict | Blocker | Major | Minor |
| --- | --- | --- | --- | --- |
| 👁️representations, Beat3_SameObject | **FAIL** (one marker to fix; otherwise close to pass) | 0 | 1 | 6 |
| 🧩️models, Beat4_PartialModels | **FAIL** | 1 | 4 | 8 |

Defects are counted once per defect; the key column lists every affected key (en and de unless stated).

---

## Clip 1: 👁️representations (Beat3_SameObject)

### Defects

| ID | Key | Severity | Location | Defect and fix |
| --- | --- | --- | --- | --- |
| R-M1 | `stored` (en, de) | major | Scene line 634 (`tick_label` and dashed "window starts" line in the footprint row, `_row_footprint` / `_beat_stored`) | The dashed "window starts" marker stands at the wall midpoint (about 10.75 m of 21.50 m). The model's window OP-S starts at 1.50 m (basis assumed) and the elevation shows the same. The marker contradicts the printed coordinates [0.00; 0.00] → [21.50; 0.30]. Fix: place the marker at `op["start"]` (1.50 m) from the wall start, add a "1.50 m*" dimension, and star it. Check the x argument of `view(start, 0.5)` on line 634. |
| R-m1 | `plan`, `section`, `elevation`, `three` (en, de) | minor | `_dock` (line 337), `_slot` (line 322) | Thumbnail slots fill one beat late. The plan beat shows all four slots empty; the section beat shows only Plan; the elevation beat shows Plan and Section; the three beat leaves 3D empty. Each beat's own view is absent from its own keyframe. Fix: fill each slot in the beat that draws its view, so the keyframe holds it. |
| R-m2 | `update`, `takeoff` (en, de) | minor | Readout helper line 772 (`OpeningResized` card); `_beat_takeoff` line 1002 | "sill 0.90 → 0.70 m" and "head 2.00 m" (de: "Brüstung 0,90 → 0,70 m", "Sturz 2,00 m") print without the assumption star. The model's OP-S sill and head are basis assumed, and `sweep`, `source` and `derived` star them. Fix: append `self._basis("sill")` and `self._basis("head")` to these lines. |
| R-m3 | `section` (en, de) | minor | Level marks in `_beat_section` (around lines 510–535) | The top level "+9.40" ("+9,40") has no star, while "±0.00*" (the reference, assumed) is starred. The 9.40 m height is sourced (VS-2.2), but its reference is assumed, so the value depends on an assumption. Fix: star "+9.40" on this sheet, or state the reference as sourced in one place and apply the same rule in every beat. |
| R-m4 | `derived` (en, de) | minor | `_beat_derived` and `_area_hatch` (around lines 666–700) | Zoomed (4×), the wall band carries two hatch angles, and the window band carries the gross and the opening hatch. The green net hatch does not visibly stop at the opening, so the 44.15 m² (net) region cannot be separated on the drawing from the gross region. Fix: draw net wall as its own polygon (wall minus opening) with its own hatch; do not run the gross hatch under the opening. |
| R-m5 | all 18 de frames (persistent card) | minor | Object card helper (around lines 240–254, `_card`) | German description lines "Ob das Bauteil an die Außenluft grenzt" and "Feuerwiderstandsklasse des Bauteils" run into the basis word "angenommen" with no gutter (zoomed). Fix: shorten these German meanings or give the basis column its own width. |
| R-m6 | all 36 frames (persistent card) | minor | Model `🧱️model/🔣️.json`, element W-S, `Pset_WallCommon.LoadBearing` (basis "sourced", source VS-3.1) | Fleet-r2 (pending follow-ups) plans basis "inferred" for LoadBearing. VS-3.1 only states that infill walls are brick or plaster, so "sourced" overstates it. Fix: set the basis to "inferred" in the model (data change, then re-capture). |

### Checked and clear (representations)

- Arithmetic, all printed as shown: volume 19.350 − 7.215 = 12.135 m³; 40.45 m² × 0.30 m = 12.135 m³; rev 1 void 6.105 m³ and wall 13.245 m³; areas 21.50 × 3.00 = 64.50 m², 18.50 × 1.10 = 20.35 m², 64.50 − 20.35 = 44.15 m², 20.35 ÷ 64.50 = 31.6 %; rev 2 18.50 × 1.30 = 24.05 m², 64.50 − 24.05 = 40.45 m², 24.05 ÷ 64.50 = 37.3 %; levels 3.50 + 0.90 = 4.40, 4.40 + 1.10 = 5.50, mid-window cut 4.95 and rev 2 4.85; wall chain 1.00 + 1.10 + 0.90 = 3.00 m; sweep cut 3.50 + 1.45 = 4.95 m.
- Scale: 1:200 gives 0.30 m → 1.5 mm, 21.50 m → 107.5 mm, 0.03 m → 0.15 mm, and the drawn 0–5 m bar is consistent. 1:20 gives 0.30 m = 15 mm, layers 0.03 → 1.5 mm, 0.24 → 12.0 mm; the 0–2 m bar and the 1.50 m pier are consistent.
- Copies: 0.90 − 0.70 = 0.20 m, the "missing 0.20 m" strip matches.
- Drafted line: 19.20 m (plan) vs 20.00 m (model and elevation) is shown consistently across the drafted, regenerate and table frames.
- IFC names: IfcWall, IfcWindow, IfcOpeningElement, IfcRelVoidsElement, IfcRelFillsElement, IfcExtrudedAreaSolid, IfcRectangleProfileDef, IfcBuildingStorey, IfcBuilding, IfcSite, IfcProject, Pset_WallCommon.IsExternal / LoadBearing / FireRating, representation identifiers Axis, FootPrint, Body with Curve2D, SweptSolid. The GlobalId `3fNY9OuTHQ5QVew2$dtdIV` has 22 valid characters and is identical in rev 1 and rev 2. Carry-over, not counted: FootPrint RepresentationType Curve2D is still open in research-ifc §7 (item 16.2); the label is kept as decided.
- Facts: south-wall ribbon window only, other façades noted as a teaching simplification (research 4.4); the window length is shown as a teaching value; the piloti count is not used on this clip.
- Narration ⇄ picture: every clause matches its frame in en and de (checked frame by frame; the layout tool's single strict item, "stroke through 31.6%", is the opaque chip over the window mullions, a false positive in en and de).
- German: decimal commas and thin spaces correct; "%" spaced in DE; English terms only where IFC names are kept.

### Round-prior status (representations, from `T/verify-r2-representations.md`)

| Prior | Status |
| --- | --- |
| B1 volume arithmetic | resolved (three decimals, all operands and results printed consistently) |
| M1 assumed values unstarred | mostly resolved; sill and head in update and takeoff, and +9.40, remain (R-m2, R-m3) |
| M2 card shows translated labels | resolved (IFC property names with localized meaning under each) |
| M3 FootPrint drawn as fill | resolved (outline only; the Curve2D label stays as the open research item) |
| m1 star rule | partly (R-m2, R-m3) |
| m2 no title | intentional (per-beat headings are present) |
| m3 sweep poché overrun | resolved |
| m4 dimension tick through label | resolved |
| m5 "[white]" brackets | resolved |
| m6 rev 1 value before | resolved |
| m7 hatch coincidence | partly (R-m4) |
| m8 "belegt" | resolved in the card ("Quelle"); the models clip still uses "belegt" (see M-m1) |
| m9 LOD scale statement | resolved |
| m10 other façades note | resolved |
| m11 German terms | resolved ("Schnittebene", "Öffnung geändert") |

---

## Clip 2: 🧩️models (Beat4_PartialModels)

### Defects

| ID | Key | Severity | Location | Defect and fix |
| --- | --- | --- | --- | --- |
| M-B1 | `owners`, `check` (en, de) | **blocker** | `_owners` (line 622), `_check` (line 647) and the ground-floor plan helper | The ground-floor plan draws no walls. Zoomed: a dashed perimeter, grid, the curved glass wall (IfcCurtainWall), ramp, pilotis, boiler and pipes; no wall poché anywhere. The caption "every line belongs to one model: walls and glass" (de "Wände und Glas") claims walls that are not drawn, and the architecture count (7) lists no IfcWall. The bar's "never a bare box standing for a wall" is broken. Carry-over of V-B1. Fix: draw the ground-floor walls as poché from the model's wall outlines (0.30 m, architecture colour), keeping pilotis, pipes and glass; or remove "walls" from the caption and the owner count. |
| M-M1 | `structure`, `schema`, `owners`, `translation`, `members`, `supports`, `properties` (en, de) | major | Structure beat spec line 409 ("pilotis (teaching choice)"), note line 418 ("The sources disagree on the piloti count"); count labels in `_owners` (622), `_translation` (693), `_members` (730), `_supports` (819), `_properties` (957) | The coordinator decision is binding: any piloti count reads "25 in this teaching model*" with the assumption star and the legend "sources differ: 15–25". No frame has "in this teaching model", a star, or "15–25" (checked in the scene and on the frames). The 25 appears as a plain count in schema, owners, translation, members and supports. Fix: one piloti label helper that prints "25 in this teaching model*" and the legend "sources differ: 15–25" wherever 25 appears, in both languages. |
| M-M2 | `services` (en, de) | major | Services label line 462 ("drained by … roof drains" / "entwässert über … Dachabläufe"); IfcWasteTerminal name line 448 | "110 m² hanging garden, first floor, drained by 4 roof drains" attaches the first-floor, open-air garden to roof drains. The dashed legend on the first-floor plan says the roof slab has an opening above the garden, so no roof drains can serve it. This re-attaches the garden's drainage to the roof, against the decision "never move the 110 m² garden to the roof". Fix: name the drains as terrace (floor) drains on the first-floor storey, or say the roof drains serve the roof and not the garden. Change the DE line the same way. |
| M-M3 | all 19 keys (en, de) | major | Class `Beat4_PartialModels` (line 45 onward): no `HEADINGS`, no `_fonts.section_heading` | No per-beat heading appears on any frame. Fleet-r2 decides "no persistent title chip; per-beat section_heading only", and the representations, information and objects scenes carry per-beat headings. Fix: add `HEADINGS` (en, de) and `_fonts.section_heading` per beat, as in the representations scene (line 36 and line 233). |
| M-M4 | `rooms`, `midsurface`, `properties`, `slab`, `buildup` (en, de) | major | Room footnote line 359; `_midsurface` (line 768); `_properties` (line 957); `_slab` (line 593); build-up legend | Fleet-r2 says teaching-model assumptions are "always starred". This clip marks them with words ("assumed", "angenommen", "teaching levels, not sourced", "Lehrwerte") and no star. Unstarred assumed values: levels +3.50, +6.70, ±0.00 (rooms); piloti height 3.25 m and slab 0.25 m (midsurface; 3.25 m also on properties); decimal room areas 71.7, 17.5, 7.0, 63.5 m² (rooms, teaching geometry from outlines); build-up 0.03 / 0.24 / 0.03 m (buildup). The representations clip uses the star, so the two clips use different rules. Fix: star every assumed value on screen and keep the word tag as the legend. |
| M-m1 | `rooms` (de), `owners` (de), `slab` (de) | minor | Line 359 "Lehrhöhen, nicht belegt · belegte Flächen in ganzen m²"; line 642 "Tragraster … (belegt)"; slab card "LoadBearing TRUE belegt" | "belegt" reads as "occupied" in German, the objection already accepted for the representations card ("Quelle"). Fix: use "Quelle" (or "gesichert") for sourced in this clip too; EN "sourced" stays. |
| M-m2 | `services` (en narration) | minor | Narration line 62: "boiler, radiators, drains, downpipes" | The model has one radiator (IfcSpaceHeater ×1; the frame says "radiator under the salon window"). The DE narration is singular and correct. Fix: "boiler, radiator, drains, downpipes". |
| M-m3 | `slab` (en, de) | minor | `_slab` line 593 | Row "Net floor area" is a teaching label, not an IFC name (IFC: Qto_SlabBaseQuantities.NetArea). "LoadBearing TRUE sourced" has no slab LoadBearing source (research 3.1 implies a post-and-slab frame but gives no value; carry-over of V-m3). Fix: show Qto_SlabBaseQuantities.NetArea and Pset_SlabCommon.LoadBearing with basis "inferred". |
| M-m4 | `trace` (en, de) | minor | `_trace_footer` and `_records` (about lines 1007–1035) | "82 loss records in 13 kinds" cannot be read from the drawing: the orange squares are not labelled per kind, so the 13 kinds are not identifiable on screen. Fix: give each group a small kind tag or bracket. |
| M-m5 | `translation` (de) | minor | Header line 693, "IFC4X3_ADD2 Structural Analysis Domain" | The German frame keeps the English header. Fix: "Strukturanalyse-Domäne (IFC Structural Analysis Domain)". |
| M-m6 | `midsurface` (en, de) | minor | `_midsurface` (line 768) | The zig-zag ramp flights in the section are not labelled, so they read as an unidentified structure. Fix: label "ramp flights" / "Rampenläufe". |
| M-m7 | `members`, `supports`, `loads`, `owners` (en, de) | minor | Plan dimension "1.25" at the top-left and top-right | The "1.25" labels are crossed by their extension lines (zoomed). Fix: offset the label above the tick. |
| M-m8 | `check` (en, de) | minor | Inset dimension "0.18 m" / "0,18 m" | The leader starts on the last glyph of the label (DE clearly). Fix: start the leader 0.05 units clear of the text. |

### Checked and clear (models)

- Counts and sums: architecture 5+4+4+1+2+4+2+6 = 28; structure 25+2+4+2 = 33; services 1+1+4+7 = 13; 28+33+13 = 74; 74 − 8 shared = 66; loss rows 4+4+4+1+1+2+4+6+1+13 = 40 (render and ramp void uncounted); kept 25 + 1 = 26; 40 + 26 = 66. Schema bars match the class rows.
- Geometry: grid 1.25 + 4 × 4.75 + 1.25 = 21.50 m; 4 × 4.75 = 19.00 m; mesh 7 × 5 = 35 nodes and 6 × 4 = 24 shells; members 25 line + 1 surface.
- Slab: 21.50 × 19.00 = 408.50 m²; 408.50 − 28.08 = 380.42 m² (model slab, outline-derived net); section 0.25 m.
- Loads: q = 6.25 + 1.50 + 2.00 = 9.75 kN/m²; 0.25 m × 25 kN/m³ = 6.25 kN/m²; 380.42 × 9.75 = 3,709 kN (3.709 kN in DE); bar proportions match.
- Midsurface: piloti 3.250 m + half slab 0.125 m = 3.375 m (line member); level marks +3.50 and +6.70 and +9.40 on scale (checked against the scale bar).
- Clash: 0.25 − 0.18 = 0.07 m short; GlobalIds DP-1 and A1 22 characters, valid alphabet.
- Room areas: Salon 7.3 × 8.7 = 63.5; Garage 5 × 3.5 = 17.5; Boiler 2 × 3.5 = 7.0; Hall 52.5 + half circle r 3.5 = 71.7; hanging garden reference 110 m² (outline 10.7 × 10.3 ≈ 110); solarium reference 70 m² on the roof. The on-screen positions match the model outlines.
- Coordinator decisions: hanging garden 110 m² on the first floor and solarium 70 m² on the roof (rooms beat, correct); the roof-slab opening legend on first-floor plans.
- Historical and IFC names: Villa Savoye 1928–1931, white render, ribbon windows on all sides labelled a teaching choice; IfcCurtainWall, IfcRamp, IfcRampFlight, IfcSpace, IfcSlab (ROOF), IfcBoiler, IfcSpaceHeater, IfcWasteTerminal, IfcPipeSegment, IfcColumn, IfcStructuralCurveMember, IfcStructuralSurfaceMember, IfcStructuralPointConnection, IfcBoundaryNodeCondition, Pset_ColumnCommon.LoadBearing / FireRating / IsExternal, Pset_WallCommon.LoadBearing, IFC4X3_ADD2, IfcRelAggregates all exist; the "fixed" support is shown as a teaching sketch (V-M5 resolved).
- Prior majors resolved: V-M1 (translation 1 IfcSlab → 24 shells), V-M2 (loss table sums to 40), V-M3 (garden on the first floor), V-M4 (Ø and density tagged assumed, though by word, see M-M4), V-M5 (IfcBoundaryNodeCondition wording), V-M6 (ramp flights drawn inclined in the section inset), V-M8 (scale bars on plans and sections), V-B2 (analysis views use the structure axon with grid A–E / 1–5 and the labelled C3 member).
- Narration ⇄ picture: each clause matches its frame in en and de except M-B1 (owners) and M-m2 (services).
- German: decimal commas and thousands dots (3.709 kN) correct; "Lehrwerte", "Lehrentscheidung" and "gestrichelt" legends present; no bracketed placeholders remain.

### Round-prior status (models, from `T/verify-r2-models.md`)

| Prior | Status |
| --- | --- |
| V-B1 ground-floor plans draw no walls | **open** (M-B1) |
| V-B2 analysis views generic grid | resolved |
| V-M1 translation 24 IfcSlab | resolved |
| V-M2 loss table sum | resolved |
| V-M3 roof garden on the wrong level | resolved (garden on first floor), but the services drains now re-attach it to the roof (M-M2) |
| V-M4 Ø and density unmarked | partly (word tags, no star; M-M4) |
| V-M5 IfcBoundaryNodeCondition | resolved |
| V-M6 ramp not identifiable | resolved |
| V-M7 midsurface reads as broken | partly (roof reads as one band; flights unlabelled, M-m6) |
| V-M8 scale bars | resolved |
| V-M9 trace records and kinds | partly (82 squares drawn; kinds unlabelled, M-m4) |
| V-m3 slab LoadBearing "sourced" | open (M-m3) |
| V-m12 no title | open as M-M3 (per-beat heading required by fleet decision, not the title chip) |
| other V-m items | resolved (levels teaching-tagged, services narrated, federation narrated, brackets removed, schema header clear, labels clear) |

---

## Not verified

- The 82 loss records and the 13 kinds were not recomputed (they come from `loss_groups` in code); only the drawn squares were checked.
- The ground-truth IFC 4.3.2.0 names were checked against `T/research-ifc.md` and standard IFC names, not against the official schema (no network access).
- Animation intermediates (sweep, takeoff, volume, update, the loss leaders) were judged on keyframes only, as in the prior verifications.
- The DE layout report for the models clip was read (no strict findings); the DE representations layout report was read (one false positive, the same 31.6 % chip).

## Counts

- Representations: 0 blocker, 1 major (R-M1), 6 minor (R-m1 to R-m6).
- Models: 1 blocker (M-B1), 4 major (M-M1 to M-M4), 8 minor (M-m1 to M-m8).
- Total: 1 blocker, 5 major, 14 minor.
