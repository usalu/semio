# 🔎 Verification r2: objects clip (Beat2_ObjectOrientation)

Verdict: **FAIL**. Defects: 0 blocker, 7 major, 18 minor.

Scope: clip `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py`, class `Beat2_ObjectOrientation`, 19 narration keys, en and de. Read-only: no source file was edited.

## 1. Evidence and method

- Fresh frames: `T/🗑️generated/objects-r2/frames/Beat2_ObjectOrientation/` (en and de, 19 PNG each, 1280x720). The first capture (22:05 to 22:08) was superseded. The model JSON `🧱️model/🔣️.json` (22:19) and the panels module `🎨️visuals/🧾️panels/🐍️.py` (22:18) changed after it, so both languages were captured again (22:22 to 22:24). Only the two `pset` frames changed, from "sourced" to "inferred" / "gefolgert" for LoadBearing. The verdict is based on the re-captured frames. Old frames are kept in `T/🗑️generated/verify-objects/first-capture/`.
- All 38 PNGs were read with the Read tool. Zoom crops: `T/🗑️generated/verify-objects/crop-*.png`, made by `crop.py`.
- Model values were dumped with `verify-objects/facts.py` into `verify-objects/facts.txt`. Checked against it: counts, box 21.5 x 19 m, grid 1.25 + 4 x 4.75 + 1.25, B3 at (10.75, 4.75), slab SL-1 380.42 m² x 0.25 m = 95.105 m³, wall W-S 21.50 x 3.00 = 64.50 m², opening 18.50 x 1.10 = 20.35 m², glazing 31.55 %, B3 tributary 15.534 m² (per piloti 8.61 to 22.56 m²), r 0.15 m, h 3.25 m, A 0.070686 m², V 0.229729 m³, 25 x 0.2297 = 5.7425 ≈ 5.74, layers 0.03 + 0.24 + 0.03 = 0.30.
- Arithmetic: every printed product, sum and rounding holds in both languages. The German decimal comma is correct everywhere (`1.250,5` rule applies to the four-digit case; none occurs in this clip).
- Layout: the `### strict` blocks in both layout reports are empty (0 findings in 0 of 19 frames). The strict check does not see the defects below, so the reports were not relied on; the frames were read by eye.
- Benchmark: `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/Beat8_Hauptgleichung-chain.png` (substituted formula, bar, source tag, title chip, takeaway line).
- Limit: the `change` still is the reverted 0.30 m state (the clip ends there). The 0.40 m mid-state was not re-captured, so the live change is verified only from the code (`_change`, lines 661 to 674).

## 2. Checklist results

1. Value to meaning on the villa at the same moment: mostly met. Diameter and height are dimension lines, the area is hatched, the volume is filled, the tributary area is hatched on the plan, the opening is hatched and dimensioned. Not met: the LoadBearing value (see MAJ-7), the GlobalId leader (MAJ-4), and the 25 headline (MAJ-6).
2. Numbers and IFC: arithmetic correct everywhere. Open issues: basis of the opening numbers (MAJ-5), the tributary claim (MAJ-1), the exchange relation target (MAJ-3), the 60 vs 36 object count (MIN-1). IFC names are real IFC 4.3 spellings (IfcColumn, IfcColumnType, IfcRelDefinesByType, IfcRelVoidsElement, IfcRelFillsElement, IfcRelContainedInSpatialStructure, Pset_ColumnCommon, IFCRELDEFINESBYTYPE, IFCPROPERTYSINGLEVALUE and the STEP attribute counts). The STEP attribute counts were checked for IFCCOLUMN, IFCCOLUMNTYPE, IFCRELDEFINESBYTYPE, IFCRELDEFINESBYPROPERTIES, IFCPROPERTYSET, IFCPROPERTYSINGLEVALUE, IFCEXTRUDEDAREASOLID, IFCCIRCLEPROFILEDEF, IFCRELASSOCIATESMATERIAL and IFCSHAPEREPRESENTATION. All match.
3. Overlaps, clipping, readability: no clipping and no overlaps of text blocks. Small issues only (MIN-8, MIN-9, MIN-12, MIN-13, MIN-14). The strict blocks report 0 findings but do not catch these.
4. Narration and visual sync: values narration contradicts itself (MAJ-2). Composition narration omits the layer set, but the layers beat has its own row, so that is fine. Distinction heading is stale (MIN-18).
5. German: complete, every clause has a frame. Correct: `±0,00`, `Südfassade · Lehrannahme`, `angenommen`, `gefolgert`, `Vom Schema zum Objekt`, `1.250,5`-style decimals (`20,35`, `64,50`, `15,53`, `5,7425`, `0,07069`). Open: `Piloti-Stütze` and `Piloti` still on screen (MIN-17), untranslated `Axis` and `FootPrint` labels (MIN-17).
6. Depth compared with the energy clips: comparable or deeper. Substituted formulas, live trackers, bars on one scale, value change, dimension strings, hatch conventions, source tag and takeaway chips are all present. Remaining gaps are listed as minors.
7. Audit r2 status: see section 4.

## 3. Defects

Keys follow the audit tags. "Location" uses the clip file line numbers.

### Majors

- **MAJ-1 [numbers, sync] major. The tributary area is presented as a type property.** EN `en-pset`, DE `de-pset`: under "B3 carries this part of the slab: 15.53 m²" the line reads "a property of the type, shared by all 25" (de: "Eigenschaft des Typs, gemeinsam für alle 25"). The 15.53 m² is B3's own derived area; the model gives 8.61 to 22.56 m² across the 25 pilotis, so the area is not shared. Only LoadBearing, IsExternal and FireRating are type properties. Location: `_pset`, clip lines 434 to 436 (EN and DE strings in the same lines). Fix: EN "derived for B3 alone; the area differs for every piloti" and DE "abgeleitet für B3 allein; je Stütze verschieden". Keep the type property line on the Pset table only.

- **MAJ-2 [sync] major. The values narration contradicts itself.** EN `en-values`, DE `de-values`: "Here every inherited attribute has a value: GlobalId, name, place on the grid and tag; three stay empty." (de: "Hier hat jedes geerbte Attribut einen Wert: … drei bleiben leer.") Nine attributes exist; six carry values (GlobalId, Name, ObjectPlacement, Representation, Tag, PredefinedType), three are empty ("not in model", "not set", "not set"). "every" and "three stay empty" cannot both hold, and the list omits Representation and PredefinedType. Location: NARRATION row `values`, clip lines 94 to 96. Fix: EN "Here six inherited attributes have values, among them GlobalId, name, placement and tag; three stay empty." DE "Hier haben sechs geerbte Attribute einen Wert, darunter GlobalId, Name, Rasterplatz und Tag; drei bleiben leer."

- **MAJ-3 [IFC, sync] major. The Pset is attached to the occurrence in the exchange but to the type in the pset beat.** EN `en-exchange`, DE `de-exchange`: STEP line `#118=IFCRELDEFINESBYPROPERTIES(…,(#101),#117)` binds Pset_ColumnCommon to the occurrence B3. The chip "IfcRelDefinesByProperties → Pset_ColumnCommon" sits beside B3. The pset beat (`en-pset`, `de-pset`) and the model say the set belongs to the type "Pset_ColumnCommon · IfcColumnType" (`TY-PILOTI` psets in `🔣️.json`). In the STEP, `#119=IFCCOLUMNTYPE(…)` has HasPropertySets = `$`. Location: model STEP writer for the excerpt (`🧱️model/🐍️.py`, the IFCRELDEFINESBYPROPERTIES line) and `_exchange`, clip lines 988 to 991. Fix: attach `#117` to the type through IfcColumnType.HasPropertySets (`#119`) and make the exchange chip read "IfcRelDefinesByProperties → IfcColumnType" or drop it, or change the pset beat to say the set is on the occurrence. Note: the rule that type-level psets hang on IfcTypeObject.HasPropertySets is my knowledge, not verified against 4.3.2.0 (research-ifc §4, §16 item 2).

- **MAJ-4 [sync, layout] major. The GlobalId leader does not reach B3.** EN `en-values`, DE `de-values`, crop `crop-en-values-leader.png`: the yellow leader ends on a dot on the column-3 guide between rows D and E, in mid-air, not on the ringed piloti B3 at row B. Location: `_values`, clip lines 731 to 734 (`route` = midpoint of grid y[3] and y[4]). Fix: end the leader on `plan.centres[SELECTED]` (for example a horizontal run at B3's height, then a dot on the ring), so the GlobalId visibly points at the piloti.

- **MAJ-5 [numbers] major. Opening, wall and glazing values have no basis badge.** EN `en-opening`, `en-window`, DE `de-opening`, `de-window`: "18.50 m × 1.10 m = 20.35 m²", "wall W-S 21.50 m × 3.00 m = 64.50 m²", "glazing share 20.35 m² / 64.50 m² = 31.6 %" (de: "Glasanteil"). Research 4.5: ribbon size, sill and head height "not found, do not state". The model values are teaching assumptions (`opening` sill 0.9 m, head 2.0 m, 1.10 m height in the model). The layer strip in `en-layers` does carry "assumed"; these readouts do not. Location: `_opening` lines 860 to 864; `_window` lines 878 to 879. Fix: add `self._assumed(...)` next to the opening readout and to the glazing line (both languages), and a MUTED footnote "ribbon size: teaching assumption". Audit NUM-8 rated this minor; raised to major because the research rule is "do not state".

- **MAJ-6 [numbers] major. The headline count 25 is stated without the source conflict.** EN/DE `occurrences` (counter "25 / 25" with "assumed"), `instance` ("1 / 25"), `schema` ("60 objects"), `pset` ("shared by all 25"), `total` ("25 × 0.2297 m³"). Research 2.11 and the recommended-values table: "Do not state a count in the model; mark as open", with lex.dk 25 against a student slide 15. The clip shows the caption "sources disagree on the real count; the model assumes this one" but neither number of the conflict, and the big headline still reads 25. Audit NUM-1 fix (n / N, sub-caption with both sources) is not applied. Location: `_occurrences` lines 407 to 412 (counter and note), `_instance` line 384, `_pset` line 436, `_total`/`_sum_panel` lines 610 to 616. Fix: counter "n / N" with a sub-caption "research: 25 (lex.dk) or 15 (student slide), open" in both languages; keep "25" only inside the model-computed formulas, labelled assumed.

- **MAJ-7 [depth, sync] major. LoadBearing TRUE has no drawn meaning.** EN `en-pset`, DE `de-pset`: the Pset table lists "LoadBearing TRUE inferred" and the narration says "LoadBearing says that each piloti carries its share of the floor". The only drawing is the hatched tributary area on the plan. No load arrow runs from the first-floor slab underside to B3's base, so the value is not shown by its meaning (audit ABS-3, NB-2 not applied). Location: `_pset`, clip lines 422 to 440. Fix: draw load arrows from SL-1 underside to B3 base with a label "LoadBearing TRUE" (EN) / "LoadBearing TRUE" (DE), scaled from the tributary piece, in the same beat.

### Minors

- **MIN-1 [numbers] minor. 60 objects vs 36 drawn.** EN/DE `schema` chip "60 objects = 60 GlobalIds" (line 314). The class beat shows 25 + 2 + 5 + 4 = 36. The model's other 24 (IfcOpeningElement 4, IfcRamp 2, IfcRampFlight 4, IfcCurtainWall 1, IfcBoiler 1, IfcSpaceHeater 1, IfcWasteTerminal 4, IfcPipeSegment 7) are never named. Fix: "36 of 60 objects are shown here" or name the other classes in the class beat.
- **MIN-2 [numbers] minor. Diameter and height stated with a note.** EN/DE `geometry` and `methods`: Ø 0.30 m and h 3.25 m, "assumed", note "no source gives the real size". Research 2.12 says "do not state". The conflicting student-slide values (r 0.30 m, h 2.87 m) are not shown. Fix: add MUTED footnote "student slide, unverified: h 2.87 m" in both languages.
- **MIN-3 [IFC] minor. Unconfirmed [M] IFC claims on screen.** IfcBuiltElement as direct supertype of IfcColumn/IfcWindow, IfcRelContainedInSpatialStructure, IfcRelAggregates, IfcMaterialLayerSetUsage, IfcRelAssociatesMaterial, Qto_ColumnBaseQuantities, the FILE_DESCRIPTION view string. Research-ifc §16 asks for confirmation against 4.3.2.0. Not checkable here (standards pages returned 403). Fix: confirm before final render, or mark the teaching caveat once per clip.
- **MIN-4 [depth] minor. Axonometric style varies.** Schema and class solid, instance x-ray, total ghosted slab (audit DEP-4). Fix: one style for the whole clip, ghost only in the instance beat.
- **MIN-5 [depth] minor. Schema rule not drawn on parts.** EN/DE `schema`: "every IfcRoot object has a GlobalId" is shown as sparks in motion and a count chip. No GlobalId chip stays on B3, SL-1 or W-S (audit ABS-2). Fix: keep three GlobalId chips at the end state.
- **MIN-6 [sync] minor. Exchange tokens not tied to the drawn piloti.** EN/DE `exchange`: underlined GUID, IFCCOLUMN and 'B3' have no leader to the piloti (audit ABS-5, LAY-10 partly). Fix: one leader from the GUID token to the mini piloti.
- **MIN-7 [IFC] minor. Relationship arrows labelled "IfcBuildingStorey".** EN/DE `spatial`: the arrows from building to storeys are labelled with the class name, not with IfcRelAggregates. Fix: label the arrows "IfcRelAggregates" (as on site → building).
- **MIN-8 [layout] minor. Font: capital I and lowercase l are not separable.** EN `en-class`, crop `crop-en-class-tree.png`: "IfcRoot", "IfcObjectDefinition", "IfcBuiltElement" read as "lfc…". The font is the shared energy toolkit font (audit LAY-16, not fixed). Fix: raise with the toolkit owner, or use a font where I and l differ for IFC names.
- **MIN-9 [layout] minor. The instance GlobalId leader crosses the x-ray villa.** EN/DE `instance`: the leader from the chip at left to B3 runs across the building (audit LAY-17, not fixed). Fix: place the GlobalId chip beside B3 or use the MINI axon.
- **MIN-10 [numbers] minor. Ribbon on all four façades.** EN/DE `schema`, `class`, `instance`: ribbon bands appear on every face. Research 4.4 and 10.5: extent disputed (FR Wikipedia: south only). No note on screen (audit NUM-9). Fix: draw the south face only, or add MUTED "ribbon extent disputed (research 4.4)" in the schema beat.
- **MIN-11 [depth] minor. Unlabelled drawn objects.** EN/DE `spatial`, `opening`, `window`, `layers`: the grey block above the roof is the solarium screen SC-1 (model: IfcWall, SOLIDWALL) and is unlabelled. In `spatial` the ramp zigzag in the section is unlabelled (RM-1, RM-2). Fix: label both, or hide them in the section beat.
- **MIN-12 [layout] minor. Dimension ticks cross the labels.** EN `en-occurrences`, crop `crop-en-occurrences-dims.png`: the extension ticks cut through "1.25" at both ends of the top dimension string. Fix: put the labels above the ticks (offset the text).
- **MIN-13 [layout] minor. German plan title touches the grid bubble.** DE `de-occurrences`, crop `crop-de-occurrences-plan.png`: "Grundriss Erdgeschoss" starts at the bubble "5". Fix: shift the label right or down, or shorten to "Grundriss EG".
- **MIN-14 [layout] minor. German central text nearly touches both boxes.** DE `de-recompute`: "ein Ergebnis darf gespeichert sein als" is about 9 px from both box borders. Fix: wrap to two lines.
- **MIN-15 [numbers, depth] minor. Symbolic equality.** EN/DE `recompute`: "V = V ✓" uses letters. The owner's rule is no bare symbols. Fix: "0.2297 m³ = 0.2297 m³" (or "0,2297 m³ = 0,2297 m³").
- **MIN-16 [sync] minor. Pset hatch lacks a plan label.** EN/DE `pset`: the hatched area is the first-floor slab's share, drawn on the ground-floor plan. The plan title was removed in this beat. Fix: restore "Ground-floor plan" and add "first-floor slab above, projected" in MUTED.
- **MIN-17 [de] minor. Terms and untranslated labels.** DE `de-instance`, `de-occurrences`, `de-values`, `de-exchange`: "Piloti-Stütze" is used for the singular, the narration says "Stütze" (audit DE-7 partly fixed). DE `de-representations`: the drawn labels "Axis" and "FootPrint" stay English while the narration says "Achse, Grundfläche". Fix: one term per concept (`Stütze`; `Achse`, `Grundfläche`); if IFC identifiers stay, say so in the card.
- **MIN-18 [sync] minor. Heading chip stale.** EN/DE `distinction`: the chip reads "Composition: has-a" although the beat is "is-a and has-a" (`_distinction` does not call `_act`, clip lines 927 to 960). Fix: call `_act` with "Is-a and has-a" / "Ist ein und hat ein".

## 4. Audit r2 status

Blockers
- LAY-1 (south elevation collapse): resolved. Elevation draws in composition, distinction, exchange.
- SYN-1 (class tree wrong parents): resolved. IfcWall and IfcWindow hang under IfcBuiltElement on a bus.

Majors resolved
- ABS-1 (colour), ABS-4 (opening in has-a chain, is-a cross on arrow), NUM-3 (no storey value), NUM-4 (arithmetic), NUM-5 (substituted formulas), NUM-6 (slab with assumed thickness), NUM-7 (layer names assumed), LAY-2, LAY-3, LAY-5, LAY-6, LAY-7, LAY-9, LAY-11, LAY-12, LAY-13, SYN-2 (bars replace leaders), SYN-3 (layers have own beat), SYN-4, DE-1 (±0,00), DE-2 (five German frames present), DE-3 (title), DE-4, DE-5, DE-6, DEP-2 (live value change), DEP-3 (bars on one scale), DEP-5 (source tag and takeaway chip), NB-1 to NB-4 present.

Majors partial or open
- ABS-2 (schema rules): partial, see MIN-5.
- ABS-3 (relations and loads): open, see MAJ-7 and MIN-6.
- ABS-5 (STEP tokens tied to piloti): partial, see MIN-6. Pset lines added, so partly resolved.
- NUM-1 (count headline): open, see MAJ-6.
- NUM-2 (diameter and height): partly, see MIN-2.
- NUM-8 (opening size): open, see MAJ-5.
- DEP-1 (drawing conventions): partly. Hatch and grid dimension strings present; no north arrow, no section cut mark on the plan, no poché on wall outlines.
- DEP-4 (one axon style): partly, see MIN-4.
- DE-7 (one term for piloti): partly, see MIN-17.

Minors not fixed: NUM-9 (MIN-10), NUM-11 (MIN-3), LAY-16 (MIN-8), LAY-17 (MIN-9), LAY-8 (one leader now, but it ends at the wrong point: MAJ-4), SYN-5 (narration now contradicts itself: MAJ-2), SYN-7 (term drift resolved in the table and formula: "height" used consistently).

Not checked closely: LAY-18 (total fill strokes), SYN-6 (arrow cross now at box edge, looks right on the frame).

## 5. Verdict basis

FAIL because seven majors remain: a false type-property claim (MAJ-1), a self-contradicting values narration (MAJ-2), an IFC relation placed on the wrong object (MAJ-3), a leader that does not reach its element (MAJ-4), unbased numbers on the opening (MAJ-5), the research-rejected count headline (MAJ-6), and a stated load value with no drawn load (MAJ-7). The owner's bar ("every value shown by its meaning, with a specific example, nothing abstract") is not met on MAJ-4, MAJ-6 and MAJ-7.

Re-verify after fixes with a fresh capture (en and de), then re-check: pset text, values narration and leader, exchange STEP line #118 target, opening and glazing badges, counter label.
