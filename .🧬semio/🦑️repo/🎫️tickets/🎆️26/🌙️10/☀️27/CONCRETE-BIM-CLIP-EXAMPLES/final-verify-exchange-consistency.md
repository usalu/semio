# Final verification: 🔌️exchange (Beat5_Interfaces) and 🔗️consistency (Beat6_Consistency)

Read-only verification, no source file edited. Scope: all 32 exchange frames (16 keys, en and de) and all 40 consistency frames (20 keys, en and de) in `T/🗑️generated/frames/`, both `-layout.md` reports, the scene `NARRATION` lists, `fleet-r2.md` "Coordinator Decisions", `research-villa-savoye.md`, `research-ifc.md`, the owner notes `scene-exchange.md` and `scene-consistency.md`, and the prior verdicts. Numbers were recomputed with the project interpreter (`T/🗑️generated/final-verify/values.py`, `values2.py`, `cons_values.py`, `basis.py`). Zoom crop: `T/🗑️generated/final-verify/ex-en-slab-plan.png`. The stale file `Beat6_Consistency-en-limits.png` / `-de-limits.png` (18:29) is not a current key and was not counted.

Severity: blocker = breaks a stated rule or teaches a wrong fact or object link; major = a printed number, label or count that breaks a binding decision or drawn value without meaning; minor = wording, typography, unit or cosmetic.

## Verdicts

| Clip | Verdict | Blockers | Majors | Minors |
| --- | --- | --- | --- | --- |
| 🔌️exchange (Beat5_Interfaces, 16 keys) | **FAIL** | 0 | 4 | 5 |
| 🔗️consistency (Beat6_Consistency, 20 keys) | **PASS** | 0 | 0 | 5 |

The exchange clip fails on four majors: one German formula with English number format, the binding piloti-count marking missing in four beats, an unlabelled hatched area on the first-floor plan, and an unmarked assumed room area. The consistency clip has no majors; its five minors are polish and carried-over audit items.

## Prior findings: status

**Exchange (`verify-r2-exchange.md`, 2 blockers, 6 majors, 9 minors).** Resolved: B1 `domain` (one shell, member and support, all wired), B2 `purpose` (schema section named as section, entities listed), M1 `interface` (53 sender and 33 receiver objects, B3 in the design model), M2 `cells` (22.56 − 7.03 = 15.53 m² in en), M3 `feedback` (151 kN, 15.53 m²), M4 mostly (stars on +3.50, 3.25 m, t, cantilever, opening, net area, B3 cell, example input), M6 `mesh`/`roundtrip` (true circle slivers drawn), m1, m2, m3, m4 (except X8), m5, m6, m7, m8. Partly open: m9 badge (see X2 and X6), M5 (see X3 and X4).

**Consistency (`verify-r2-consistency.md`, 0 blockers, 5 majors, 15 minors).** Resolved: `identity` pointer (ends on the roof edge), `metadata` pointers (IsExternal on the external edge, LoadBearing on the roof), planted-area claim (200.2 m² labelled as model, solarium 70.0 m² sourced, 110 m² not compared), piloti count (25 with star and legend "sources differ: 15–25" on `grid`, `spread`, `retranslate`), `resolve` (analysis 146 kN against model 146 kN, rev 1 105 kN ≠ 146 kN), recap leaders, DE table header, load-table asterisks, one German term for the soil layer, teaching-choice label for ribbon windows, analysis colour tied to the link pill, "+0.20 m" callout, badges attached to the link. Still open as minor: M19 and M21 (C4, C5).

## Exchange defects (🔌️exchange)

| Key | Sev | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| X1 `cells` (de) | major | Frame `Beat5_Interfaces-de-cells.png`, formula line under "B3-Zelle"; code `_beat_cells` | German frame prints "B3-Zelle = 22.56 - 7.03 = 15.53 m²*": English decimal points and hyphen-minus. Arithmetic holds (22.5625 − 7.0275 = 15.535, printed 15.53). | Format with the German formatter: "B3-Zelle = 22,56 − 7,03 = 15,53 m²*" (U+2212 minus). |
| X2 piloti count, binding rule (`interface`, `mapping`, `domain`, `check`; en and de) | major | `interface` badge "Teaching model: 25 pilotis (sources differ)" / "Lehrmodell: 25 Pilotis [Quellen uneinig]"; `mapping` "25 members" / "25 Stäbe"; `domain` "IfcStructuralCurveMember · 25 piloti members" / "25 Stützen als Stäbe"; `domain` "IfcStructuralPointConnection · 25 …"; `check` "25 / 25 IfcColumn pass" / "25 / 25 IfcColumn bestanden" | `fleet-r2.md` decision: every count reads "25 in this teaching model" with the assumption star and the legend "sources differ: 15–25". None of these four beats has the star, and no beat shows the 15–25 legend. The `interface` badge has the phrase without the star or range. | On every beat that shows the count: "25 in this teaching model*" and legend "* sources differ: 15–25" (de "* Quellen weichen ab: 15–25"), the wording already used in consistency `grid`. |
| X3 `slab` (en and de) | major | First-floor plan, right half (grid 3–5 × C–D, dashed outline, hatched). Crop `ex-en-slab-plan.png`. | Hatched area with no label, no leader, no legend entry and no narration. It reads as a slab area. It is most likely the first-floor hanging garden `SP-TERRACE` (110.0 m², sourced, `reference_area`). A drawn value without its meaning. | Label it at the area with a leader ("hanging garden, first floor · 110 m² (sourced)"; de equivalent), or remove the hatch. If it is open-air terrace inside the slab, say in the caption or card that the net area 380.4 m² still includes it. |
| X4 `slab` (en and de) | major | Label "SP-SALON · Salon, 63.5 m²" / "SP-SALON · Salon, 63,5 m²" | `SP-SALON` area is teaching-model geometry (`quantities` area 63.51; the room has `basis` None and research has no room area). Unlike the other geometry on this frame, it has no star. Same class as the r2 major M4. | Add the star: "63.5 m²*" / "63,5 m²*". The legend "* assumed value (teaching model)" is already on the frame. |
| X5 `mesh`, `roundtrip` (en and de) | minor | "true cylinder 0.2297 m³ (−2.6 %)" (`mesh`); "volume −2.6 %" (`roundtrip`); de "[-2,6 %]" and "Volumen -2,6 %" | Exact values give −2.55 %, which rounds to −2.6 %. From the printed values (0.2239 and 0.2297) the reader gets −2.5 %, so "arithmetic true as printed" fails. Also hyphen-minus in de. | Compute the percentage from the printed, rounded volumes and print −2.5 %, or print more digits of both volumes that round to −2.6 %. Use U+2212 in both languages. |
| X6 `loss` (en and de) | minor | "× 4 Ribbon window" chip and the all-faces ribbon band in the axonometric | Research fact 4.4 says the extent is disputed (FR Wikipedia: south façade only). The consistency clip labels its ribbon choice "teaching choice: ribbon windows on all faces"; this clip does not (r2 m9 only covered the piloti count). | Add "teaching choice: ribbon windows on all faces" (de "Lehrentscheidung: Bandfenster an allen Seiten") at the `loss` chip or on the `interface` badge. |
| X7 `properties`, `mesh` (en and de) | minor | `properties`: "r = 0.15 m" and STEP token "0.15" (en and de); `mesh`: "3.25 m" dimension (de "3,25 m") | Same assumed values are starred elsewhere (Ø 0.30 m*, 3.25 m* in `properties`, 3.25 m* in `roundtrip`) but not here. | Star "r = 0.15 m*" and the `mesh` dimension "3.25 m*". |
| X8 `feedback` (de) | minor | Table row "Lasteinzugsfl. A 15,53 m²" | Abbreviation; "Lasteinzugsfläche" is spelled out everywhere else in this clip and in consistency. | "Lasteinzugsfläche A". |
| X9 `cells` (en) | minor | "B3 cell = 22.56 - 7.03 = 15.53 m²*" | Hyphen-minus in an equation. | U+2212 "−". (Same fix as X1 for de.) |

Strict layout check (`layout.md`): en 23 findings in 4 frames, de 24 in 4 frames. The owner note claim holds. The strict list contains only token rings (SurroundingRectangle around STEP tokens, 10 in en) and intended strike-throughs (mesh and roundtrip, 13 in en). No other category. The plain "line through" list (non-strict) has wires that end at a token ring and cross the trailing `);` of the same STEP line; these are legible in all frames read and are not reported as defects.

## Consistency defects (🔗️consistency)

| Key | Sev | Location | Defect | Fix |
| --- | --- | --- | --- | --- |
| C1 `stale` (de) | minor | Caption "…Revision 1: 105 / kN, nicht 146 kN." (line break between "105" and "kN") | The U+00A0 between number and unit is in the source (18 lines carry NBSP). The caption wrapper uses `textwrap`, which splits on NBSP as whitespace, so the number and unit still break. The r3 "NBSP fix" is therefore not effective for captions. | Make the caption wrapper treat U+00A0 as non-breaking (wrap on regular spaces only), then re-capture `stale` and check the other de captions with units. |
| C2 `derived` (de) | minor | Card header "Einflussflächen-Schätzung, keine FEM"; source is `tributary()` `method_de` in `🧱️model/🧮️analysis/🐍️.py` | "Einflussfläche" is the wrong term for a tributary area; the clip uses "Lasteinzugsfläche" elsewhere and the exchange clip uses "Lasteinzugsflächen-Näherung" (r2 m4). | Model `method_de` → "Lasteinzugsflächen-Schätzung"; re-capture `derived` de. The scene reads the value from the model, so the fix is in the model text. |
| C3 `retranslate` (en and de) | minor | Review checklist "…24 shells, 35 slab nodes and 25 base nodes" / "…25 Fußknoten" | Count of pilotis without the teaching-model star. The counter box above it has the star and the legend, so the rule is met in the same frame but not on this line. | Add the star after "25 base nodes*" / "25 Fußknoten*". |
| C4 `grid` (en and de) | minor (carried over, M19) | Roof plan, `Beat6_Consistency-en-grid.png` | No section cut marker on the plan at axis 3, although `derived` takes its section there. | Add a section cut with arrowheads on the plan, labelled "3" (the previous collision with the caption band must be avoided by moving the cut marks inside the plan). |
| C5 `check`, `kept`, `report` (en and de) | minor (carried over, M21) | `check` table rows; `kept` rows; `report` card | Table and card rows are not tied to the drawn element by a leader or highlight (the 146 and 105 kN arrows, the slab for "stays the same"). | One leader or highlight per row, as in r2 M21. |

## Checks that held (recomputed)

- Roof load (`load`, `derived`): 5.00 + 0.20 + 1.32 + 5.40 + 1.00 = 12.92 kN/m²; before 9.32 (5.00 + 0.20 + 1.32 + 1.80 + 1.00); Δq = 0.20 × 18 = 3.60; build-up 0.38 → 0.58 m. Model zone `planted` area_load 9.32, zone area 200.21 m²; solarium 70.0 m².
- Reactions (`derived`, `spread`, `check`, `resolve`): A3 11.28 × 12.92 = 145.7 → 146 (before 105.1 → 105, +41); C3 2.375 × 12.92 = 30.7 → 31 (before 22.1 → 22, +9); D3 11.41 × 3.6 = +41; B3 102 (7.86 × 12.92 = 101.5, rounded up); E3 146; Σ ΔR = 3.6 × 200.2 = 721; 2,462 → 3,183 kN (model `total_reaction` 2,462.36 = zone sums). C4 area 0.00 (under the terrace void).
- Fork (`fork`, `decide`): soil 0.30 m gives q = 12.92 kN/m²; soil 0.20 m gives 11.12 kN/m² (5.00 + 0.20 + 1.32 + 3.60 + 1.00).
- GlobalId `2DXklET1jK3AZcJz_ZtAN9` is 22 characters; same GlobalId in every frame of the clip and in the STEP excerpt; `#101` local; IFC names IfcSlab, PredefinedType ROOF, Pset_SlabCommon.LoadBearing (sourced) and IsExternal (assumed, starred), IFC4X3_ADD2, ISO 16739-1:2024, IDS 1.0, BCF-XML 3.0, IfcRoot.GlobalId, IfcGloballyUniqueId.
- Historical facts: 1928–1931, Villa Savoye, Poissy; "water through the roof each autumn" (fact 8.1); storey marks without heights; gap labelled "illustration · place not documented" (fact 7.4); no 110 m² on the roof.
- Exchange arithmetic: 25 pilotis, 24 shells, 35 nodes (7 × 5 drawn lines), 25 members, 28.1 m² ramp void (overlap of 7.6–10.0 × 2.8–7.1 m with B3 cell = 7.03 m²), 408.5 − 28.1 = 380.4 m², self-weight 25 × 0.25 = 6.25, 6.25 + 1.50 + 2.00 = 9.75, 9.75 × 15.53 = 151, 9.75 × 380.4 = 3,709, 9.75 × 11.28 = 110, 9.75 × 8.61 = 84. Mesh 34 vertices, 64 triangles, volume 0.22387 vs cylinder 0.22973 m³ (see X5).
- STEP field counts: IFCCOLUMN 9, IFCCOLUMNTYPE 10 (`'Piloti'` in Name position), IFCPROPERTYSET 5, IFCPROPERTYSINGLEVALUE 4, IFCEXTRUDEDAREASOLID 4, IFCCIRCLEPROFILEDEF 4, IFCSLAB 9. The type-level set is bound through IFCRELDEFINESBYTYPE as in the fleet decision.
- German: all visible strings present in both languages; decimal comma in running text in both clips (except X1 and the NBSP case in C1); "Ausbaulasten", "Schichtdicke", "Lehrentscheidung" wording consistent; "Pflanzsubstrat" one term across consistency.
- Narration ⇄ picture sync: each clause's object is on screen in the same frame, both clips, en and de.
- Overlaps and clipping: none found in the 72 frames read. Consistency strict = 0 in all frames.
- Depth against energy clips: the energy frame `Beat5_Verschattung-raffstore` has one diagram, one value and a 2-line caption. Exchange (16 clauses) and consistency (20 clauses, with chip, reference chip, formula and drawn plans) are deeper.

## Notes

- Piloti count: consistency is compliant everywhere except the footnote-free line in C3. Exchange is not (X2).
- Titles: no persistent title chip in either clip, as decided.
- The stale `Beat6_Consistency-*-limits.png` files (18:29) belong to a removed key and should be deleted from `T/🗑️generated/frames/` so they are not mistaken for current frames.
- Not verified: IFC entity names against buildingSMART 4.3.2.0 (research marks the pages unreachable); names checked against `research-ifc.md` only.
