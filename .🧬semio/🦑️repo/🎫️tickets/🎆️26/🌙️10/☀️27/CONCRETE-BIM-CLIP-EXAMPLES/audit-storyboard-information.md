# Audit: Storyboard "Data Become Information" (`storyboard-information.md`)

Read-only audit. Checked against `research-ifc.md` (sections 1, 2, 4, 6, 8, 11, 12, 14, 15), `research-villa-savoye.md` (facts 2.6, 2.10 to 2.13, 3.1, 5.1, 5.3, 5.4, 10.3, 10.5, recommended values), `audit-current.md` (Prior Decisions To Keep, caption and timing policy), `agent-rules.md` (quality bar), `api.md` (number sources). Only this report was written.

Extra IFC check (WebSearch, standard mode): an element can be positioned against a grid with `IfcGridPlacement`, whose `PlacementLocation` is an `IfcVirtualGridIntersection` of two `IfcGridAxis` objects. That makes "grid position via ObjectPlacement" valid IFC only when the placement is of that type. The pages found are labelled "under development" (IFC4X3_ADD2 build stamps), so confirm on the released 4.3.2.0 page before publishing. Sources: https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcGridPlacement.html and https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcGridPlacement.htm

## Passed checks (no action)

- No DIKW, no "ISO 19650 defines" claim, no `IfcBuildingElement` or `IfcBuiltElement`, no window on screen (the opening chain rule does not bite).
- IFC names are real: `IfcColumn` COLUMN, `IfcSlab` FLOOR, `Pset_ColumnCommon.LoadBearing` and `.IsExternal`, `IfcRelContainedInSpatialStructure`, `IfcRelDefinesByType`, `IfcRelConnectsElements` (marked as a modelling choice, which is fine), `IfcPropertySingleValue`, `ObjectPlacement`. GlobalId is shown as a 22-character value, not a UUID.
- Grid 4.75 m, first 5 m, reduced for cost, and "wide enough for a car to pass" are sourced (research 2.10). The curved wall principle from a car's turning circle is sourced (5.1). The dark green ground floor is sourced (5.3). Vehicles going under the building is sourced (5.4). The clip says "Teaching model" once.
- No number is typed into any narration clause (en or de).
- German: "Achsabstand", "lichte Weite", "Wendekreis", "Kostengründen", "Lehrmodell", "Menschen lesen die Zeichnung" are natural and correct.

## Issues

Severity: high = wrong or overclaimed fact, or a clear breach of the quality bar; medium = unsupported or mis-sourced detail, layout or timing risk that is likely to show; low = wording or polish.

| # | beat key | severity | problem | concrete fix |
| --- | --- | --- | --- | --- |
| 1 | `raw` (beat 1) | high | Bare number "4.75" alone on a black stage for 4.5 s with ghost units. Breaches the quality bar: "never a bare number without its object and unit". | Keep the number unit-less, but place it as a dimension between two faint, unlabelled piloti circles on a 25 % ghost ground-floor plan at axes 3 and 4 (`plan.to_screen`). The number has an object but still no unit or meaning. Narration unchanged. |
| 2 | `meaning` (beat 3, narration en+de, table header) | high | "The schema gives it meaning … whether it is sourced or assumed" and the "SCHEMA · quantity" table imply that IFC carries a sourced/assumed basis. IFC has no such property. The `grid.spacing` label is a teaching-model property id, not an IFC property. The prior decision says data gains meaning through context, not only through a schema. | Narration (see corrected table). Header "SCHEMA · quantity" becomes "TEACHING DATA MODEL · quantity" / "LEHRDATENMODELL · Größe". Muted line under the basis chip: "teaching-model basis, not an IFC property" / "Lehrmodell-Angabe, keine IFC-Eigenschaft". |
| 3 | `handoff` (narration en+de) | high | "people and applications read it the same way" overclaims. A shared model does not make readings identical (prior decision: a shared schema does not imply shared instances or consistent values). "BIM connects data" is loose. | Replace with the corrected en/de below. The on-screen GlobalId line "same piloti · same GlobalId" stays, because it claims one instance, which is what the model shows. |
| 4 | beat 2 `context` (plan label "25 pilotis") and beat 9 `relations` ("type × 25", "the count 25 lights 25 real pilotis") | high | Research 2.11 says the piloti count is not settled (lex.dk 25, a student slideshow 15). The "Recommended teaching values" table says do not state a count. The storyboard states 25 as fact and calls the drawn pilotis "real". | The type row shows "× 25" with an orange `assumed` chip and the muted text "teaching-model count, sources disagree" / "Lehrmodell-Anzahl, Quellen widersprechen". Beat 9 meaning line: "the count of this teaching model lights its 25 pilotis". Remove "real". |
| 5 | beat 2 plan orientation; beat 6 car start pose | medium | Entrance and row E on the north side, and the car approaching from the north, are not in the sources. Research 10.3 (lodge orientation) is not found; 10.5 (façade orientation) conflicts. Only "simplified, rounded" is on screen, which does not cover orientation. | Extend the top-right note to "Teaching model · simplified, rounded · orientation schematic" / "Lehrmodell · vereinfacht, gerundet · Ausrichtung schematisch" (keep one mark), and add a small muted "N (schematic)" arrow in the plan corner away from the grid labels. |
| 6 | beat 9 `relations` row 4 "Grid position" / "ObjectPlacement" | medium | The row shows only the attribute name. IFC gives a grid position through `IfcGridPlacement` referencing `IfcVirtualGridIntersection` of `IfcGridAxis`. A plain `IfcLocalPlacement` only holds coordinates. The sources cited are development builds. | Row text: "ObjectPlacement · IfcGridPlacement (axes E × 3)" / "ObjectPlacement · IfcGridPlacement (Achsen E × 3)". The model must write the piloti placement as `IfcGridPlacement`. Verify on the released 4.3.2.0 page. Narration keeps "its grid position". |
| 7 | beat 9 `relations` ("elevation from g['slabs']"); beat 2 storey row | medium | First-floor elevation (+3.50 in the model) and storey heights are not in the sources (research 2.6: "not found"). The slab elevation is shown with no basis chip. | Any shown storey or slab elevation above ±0.00 gets an orange `assumed` chip. Beat 2 storey row shows only the ground storey (±0.00). |
| 8 | beat 8 `properties` row 3 (LoadBearing TRUE) | medium | "TRUE" is shown in GOOD with no basis chip. The research supports a load-bearing frame (3.1, "suppression des murs porteurs") but not this piloti's value. The storyboard's own rule says TRUE is for this piloti only. | Add an orange `assumed` chip next to TRUE, with the muted line "teaching model, this piloti only" / "Lehrmodell, nur diese Stütze". |
| 9 | beat 4 `question` (row "Turning radius") vs beat 6 (dashed turning circle) | medium | The turning-radius value appears in beat 4 but its drawn meaning (the dashed turning circle) only appears in beat 6, about 6 s later. Breaches "every value shown at the same time by its meaning". | When the curved wall lights in beat 4, draw the dashed turning circle (radius `c["turning_radius"]`, `assumed` chip, faint). Beat 6 keeps only the car and swept area. |
| 10 | beat 3 `meaning` (basis chip) | medium | The "sourced" chip points at nothing on the building. The owner's rule requires the meaning to be drawn. | Draw a short green leader from the sourced chip to the plan dimension line, with a small source tag on the dimension ("EN Wikipedia, Cité sheet") in MUTED 18. |
| 11 | beat 2 `context` (plan) and beat 9 `relations` (level mark) | medium | "±0.00" is typed into the spec and into the plan. Rule 3: every number on screen comes from a model function. | Format from the model: `self.number(g["storeys"]["ground"], 2)` with the "±" sign added by the formatter. Same for the level mark. |
| 12 | beat 5 `clearance` (narration en+de) | medium | "That is the room the car has" overclaims: Ø 0.30 m is assumed (research 2.12 not found), the car width is assumed, and the clearance under the house is not found (2.13). German "minus der Durchmesser" is wrong case. | Narration corrected (see table). Ø keeps its orange chip. |
| 13 | beat 9 `relations` (narration de) and beat 9 overall | low | "die getragene Decke" is vague. "Geschossdecke" is the natural German term. | de: "die getragene Geschossdecke". en: "the floor slab it carries". |
| 14 | `raw` (narration de) | medium | "ein Datum" in German reads as a calendar date (Datum). Rule 5: the German must mean the same as the English "data". | de: "Diese Zahl ist ein Datenwert. Allein sagt sie nicht, was gemessen wurde." |
| 15 | `question` (narration de) | medium | 116 characters, the only clause over the ~115-character caption risk. | de: "Information dient einer Frage: Kann ein Auto von 1929 zwischen den Pilotis hindurch unter dem Haus wenden?" (106 characters). |
| 16 | `properties` (narration de) | medium | "Pilotisstütze" is not natural German (the owner asked for Stütze/Piloti). "außen steht" is wrong for a column. | de: "Eine Stütze (Piloti) trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen liegt." (103 characters). |
| 17 | `reduced` (narration en+de) | low | "shrank" and "schrumpfte" describe a grid that was reduced, which is not what happened. "Auch als" reads awkwardly. | en: "Even after the grid was reduced to save cost, the answer holds: the car still fits." de: "Auch nachdem das Raster aus Kostengründen verkleinert wurde, bleibt die Antwort: Das Auto passt noch." |
| 18 | `context` (narration de) | low | "Der Kontext:" is awkward, and "zweier Pilotis" is stiff. | de: "Kontext: Es ist der Achsabstand zwischen zwei Pilotis der Villa Savoye." en unchanged. |
| 19 | title (de) | low | "Daten Werden Information" uses English-style title capitalisation. | de: "Aus Daten wird Information". en unchanged. |
| 20 | beat 10 `handoff` (muted line and "same object") | low | "the same object is a drawn piloti for people and a typed entity for applications" blurs product and representation (prior decision: a product is distinct from its representations). | Spec wording: "the same product has a drawn representation for people and a typed entity for applications". Muted line stays "same piloti · same GlobalId" / "dieselbe Stütze · dieselbe GlobalId". |
| 21 | beat 5 (spec) "so the number is never on screen twice" | low | False as written: the strip label "4.75 m" and the formula "4.75 m − 0.30 m = 4.45 m" both show 4.75. | Spec: "the plan label moves into the strip, so the plan never repeats the strip value". |
| 22 | beat 7 verdict chip at (3.6, 3.1) | medium | The chip sits at x 3.6, y 3.1, on top of the strip spacing label at y 3.05 (also x 3.6). Layout overlap. | Move the chip to (5.85, 3.1), right-aligned in the panel (about 1.5 wide, so x 5.1 to 6.6). |
| 23 | beat 10 `step_panel` | medium | "four wrapped monospace lines" is unlikely at width 6.0 with ≥ 18 pt type. Font size is not given. The panel can run into the GlobalId chip at (3.6, −1.35). | Set monospace at 18 pt explicitly, limit each line to about 45 characters with `only=`, and keep the panel bottom above y −1.1. Verify in capture. |
| 24 | beat 2 `context` (list rows IfcSite, IfcBuilding) | low | Site name and building name/years are shown with nothing drawn at the same time. Rule 4. | When each row lights, highlight its drawn part in DATA: site boundary, building box outline (dashed), ground storey plan. |
| 25 | beat 2 (spec: "ribbon-free ground floor") | low | Not in the sources (research has ribbon windows on the first floor only). | Remove "ribbon-free ground floor" from the spec, or mark the ground-floor glazing as a teaching-model choice. |
| 26 | beat 4 `question` (car silhouette "1929 car") | low | The 1929 car is sourced as the design basis (5.1, Voisin turning circle), but the car's size is assumed. | Add a muted 18 pt label under the car: "1929 car · size assumed" / "Auto von 1929 · Maße angenommen". |
| 27 | beats 3, 5, 9 (timing: stated budgets) | low | The clip says animations "sit inside these budgets", but four animations exceed the stated speech estimate: context 6.0 s vs 5 s; meaning 6.5 s vs 6 s; drive 6.5 s vs 6 s; relations 7.5 s vs 7 s. | Set context 5.0, meaning 6.0, drive 6.0, relations 7.0 (or change the stated budgets). |
| 28 | whole clip (timing estimate) | medium | "about 65 s of clip" uses the speech estimate. The audit's policy (hold budget = max of TTS, words/2.5 en or /2.2 de, and words/1.6 reading floor) gives windows summing to about 95 s (en) and 88 s (de). | Restate the clip length as about 95 s (en) and 88 s (de) under the policy, or change the policy text in `audit-current.md`. Do not keep 65 s. |
| 29 | beat 2 plan (grid labels at y ≈ −2.45) | low | The grid labels sit 0.17 above the caption band (y < −2.62). With label height they likely touch it. | Move PLAN_POINT up 0.15 (to y 0.30), or move the grid labels inside the box. |
| 30 | beat 6 (turning circle, radius assumed) | low | A circle of about 1.2 to 1.6 units centred near the north curve can reach the title chrome (x < −2.7, y > 3.2) and the plan dimension. | Clip the dashed circle to the plan box, or verify in capture and shrink it. |
| 31 | all beats (font sizes) | low | Sizes for plan labels, property rows, chips, and STEP monospace are not given. The capture check uses 18 pt; the owner's bar flags anything under 16 pt. | Put an explicit size of at least 18 on every text element in the spec. |

## Corrected narration (changed keys only; en and de)

Caption lengths are characters, counted before and after.

| key | en (current → corrected) | de (current → corrected) |
| --- | --- | --- |
| `raw` | unchanged (62) | "Diese Zahl ist ein Datum. Allein sagt sie nicht, was gemessen wurde." (68) → "Diese Zahl ist ein Datenwert. Allein sagt sie nicht, was gemessen wurde." (72) |
| `context` | unchanged (72) | "Der Kontext: Es ist der Achsabstand zweier Pilotis der Villa Savoye." (68) → "Kontext: Es ist der Achsabstand zwischen zwei Pilotis der Villa Savoye." (71) |
| `meaning` | "The schema gives it meaning: property, unit, grid, and whether it is sourced or assumed." (88) → "The data model adds meaning: property, unit, grid, and whether the value is sourced or assumed." (95) | "Das Schema gibt ihr Bedeutung: Eigenschaft, Einheit, Raster und ob sie belegt oder angenommen ist." (98) → "Das Datenmodell ergänzt die Bedeutung: Eigenschaft, Einheit, Raster und ob der Wert belegt oder angenommen ist." (111) |
| `question` | unchanged (97) | "Information dient einer Frage: Kann ein Auto von 1929 zwischen den Pilotis hindurchfahren und unter dem Haus wenden?" (116) → "Information dient einer Frage: Kann ein Auto von 1929 zwischen den Pilotis hindurch unter dem Haus wenden?" (106) |
| `clearance` | "The clear width is the spacing minus the piloti diameter. That is the room the car has." (87) → "The clear width is the spacing minus the piloti diameter. In this model, that is the room the car has." (102) | "Die lichte Weite ist der Abstand minus der Durchmesser der Pilotis. So viel Platz hat das Auto." (95) → "Die lichte Weite ist der Abstand abzüglich des Durchmessers der Pilotis. Im Lehrmodell hat das Auto so viel Platz." (114) |
| `reduced` | "Even when the grid shrank to save cost, the answer holds: the car still fits." (77) → "Even after the grid was reduced to save cost, the answer holds: the car still fits." (83) | "Auch als das Raster aus Kostengründen schrumpfte, bleibt die Antwort: Das Auto passt noch." (90) → "Auch nachdem das Raster aus Kostengründen verkleinert wurde, bleibt die Antwort: Das Auto passt noch." (101) |
| `properties` | unchanged (98) | "Eine Pilotisstütze trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen steht." (101) → "Eine Stütze (Piloti) trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen liegt." (103) |
| `relations` | "Relationships complete the meaning: storey, type, the slab it carries and its grid position." (92) → "Relationships complete the meaning: storey, type, the floor slab it carries and its grid position." (98) | "Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Decke und die Rasterposition." (102) → "Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Geschossdecke und die Rasterposition." (110) |
| `handoff` | "BIM connects data in a structured building model, so people and applications read it the same way." (98) → "BIM keeps the data in one structured building model, so people and applications refer to the same piloti." (105) | "BIM verbindet Daten zu einem strukturierten Gebäudemodell, das Menschen und Anwendungen gleich lesen." (101) → "BIM bündelt Daten in einem strukturierten Modell, in dem Menschen und Anwendungen dieselbe Stütze finden." (105) |
| title | "Data Become Information" (23) | "Daten Werden Information" (24) → "Aus Daten wird Information" (26) |

## Caption length check (threshold about 115 characters at 24 pt, 11.5 units)

| key | en current | de current | risk (current) | en proposed | de proposed |
| --- | --- | --- | --- | --- | --- |
| raw | 62 | 68 | ok | 62 | 72 |
| context | 72 | 68 | ok | 72 | 71 |
| meaning | 88 | 98 | ok | 95 | 111 (close) |
| question | 97 | 116 | **at risk (de)** | 97 | 106 |
| clearance | 87 | 95 | ok | 102 | 114 (close) |
| drive | 92 | 88 | ok | 92 | 88 |
| reduced | 77 | 90 | ok | 83 | 101 |
| properties | 98 | 101 | ok | 98 | 103 |
| relations | 92 | 102 | ok | 98 | 110 (close) |
| handoff | 98 | 101 | ok | 105 | 105 |

The proposed German `clearance` (114) and `meaning` (111) are close to the limit. The real test (`caption_bar` wrap at 11.5 units, two lines maximum) is the binding check; run it on the corrected text.

## Timing check (audit policy, en and de)

Per-clause window = max(animation seconds, hold budget), where hold budget = max(TTS estimate, words/2.5 en or words/2.2 de, words/1.6 reading floor).

| key | animation s (doc) | window en (policy) | window de (policy) | stated speech budget (doc) | status |
| --- | --- | --- | --- | --- | --- |
| raw | 4.5 | 7.5 | 7.5 | 5 | ok |
| context | 6.0 | 8.1 | 6.9 | 5 | animation exceeds budget (fix 27) |
| meaning | 6.5 | 9.4 | 9.4 | 6 | animation exceeds budget (fix 27) |
| question | 8.0 | 10.6 | 11.3 | 8 | ok |
| clearance | 8.0 | 10.6 | 10.6 | 8 | ok |
| drive | 6.5 | 10.0 | 8.8 | 6 | animation exceeds budget (fix 27) |
| reduced | 6.5 | 9.4 | 8.8 | 6.5 | ok |
| properties | 7.0 | 10.0 | 9.4 | 7 | ok |
| relations | 7.5 | 8.8 | 7.5 | 7 | animation exceeds budget (fix 27) |
| handoff | 6.5 | 10.6 | 8.1 | 6.5 | ok |
| total | 67.0 | 95.0 | 88.1 | about 65 | fix 28 |

## Summary

1. Three high issues need fixing before capture: the bare-number opening (#1), the overclaimed "schema gives meaning" and sourced/assumed basis (#2), and the "read it the same way" handoff (#3).
2. The 25-piloti count is stated as fact (#4), but research says it is unsettled; mark it as a teaching-model assumption and drop "real".
3. Orientation (north entrance and car approach) and first-floor elevations are unsourced; mark them as teaching-model (#5, #7), and check "grid position" as `IfcGridPlacement` against the released 4.3.2.0 page (#6).
4. German fixes: "Datum" (#14), "Pilotisstütze" and "außen steht" (#16), "Decke" to "Geschossdecke" (#13), "schrumpfte" (#17), and the 116-character `question` clause (#15). Corrected text is in the table above.
5. Layout and timing: move the verdict chip (#22), fix the step panel size (#23), and restate the clip length as about 95 s (en) and 88 s (de) instead of 65 s (#28).
