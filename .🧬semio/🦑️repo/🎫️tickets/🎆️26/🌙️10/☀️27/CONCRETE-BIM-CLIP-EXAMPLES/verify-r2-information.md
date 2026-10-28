# 🔍 Verify r2: 📊️information (`Beat1_DataToInformation`)

Read-only verification. No source file was edited. Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py` (21 keys). Model: `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py` and `🔣️.json`.

Inputs: `agent-rules.md`, `audit-r2-information.md`, `scene-information.md`, `research-villa-savoye.md`, `research-ifc.md`; all 21 English and all 21 German PNGs (1280 × 720) with the `-layout.md` reports; zoomed crops in `🗑️generated/verify-information/`; a read-only model check (`verify-information-model-check.py`, output in the transcript only); energy benchmark `Beat5_Verschattung-raffstore.png` and `Beat2_VolumeFlowEquation-formula.png`.

Staleness: the frames (EN 21:04–21:07, DE 21:08–21:12) are newer than the scene file (21:03:44). No re-capture was needed.

## 1. Verdict

**FAIL.** 3 blockers, 5 majors, 14 minors.

The clip has real depth now (Villa Savoye plan, magnified bay, column elevation, STEP handoff), and the German is clean. Four things block a pass:

1. The piloti count is still stated on screen (`relations`), contrary to the research's "do not state a count".
2. "The Savoyes' car" is an unsourced historical claim in both languages.
3. Many numbers on screen have no unit (`context`, `chain`, `fits`, `sensitive`, `properties`).
4. The glass-wall clearance story contradicts itself: `fits` requires a 0.50 m side margin, while `pinch` shows the car passing at 0.42 m and 0.24 m and calls that "clears them".

## 2. Defects

Severity: blocker = breaks an explicit rule of `agent-rules.md` or a research "do not state" rule; major = visibly wrong or misleading, or a clear gap against the owner's bar; minor = polish or non-visible model fix.

Line numbers refer to the scene file unless marked JSON (model `🔣️.json`) or model py.

### Blockers

| Key | Location | Defect | Fix |
|---|---|---|---|
| **V-B1** | `relations`, EN and DE. Note at approx. (735–1190, 425–500). Code lines 707, 717, 720–721. Model note `piloti-count` (JSON about lines 19–22). | The note reads "25 pilotis on a 4.75 m grid are a teaching assumption" and 25 rings are drawn lit (`type_occurrences` returns 25). Research "Recommended teaching values": "Number of piloti: Do not state a count in the model; mark as open." Audit N-02 is only half-fixed: the row says "count open", but the count is still asserted. | Delete the "25 pilotis … are a teaching assumption" sentence. Keep only the open conflict: "count open: 25 (lex.dk) vs 15 (student slideshow)". Draw all grid candidates as dashed rings, not lit solid pilotis. Model: store the piloti set with `count_status: "open"` instead of 25 occurrences of TY-PILOTI, and rewrite `piloti-count` to match. |
| **V-B2** | Narration `question`: EN line 67 "can the Savoyes' car drive…", DE line 68 "Fährt das Auto der Savoyes…". Car note code line 598 "The Savoyes' car · generic body, size assumed". | Historical claim not in the research. Research 5.1 says the curve was calculated from "the turning circle of a car (1929 Voisin)". Nothing in the research says the Savoyes owned a car. `agent-rules` line 28 requires historical claims to match the research. | EN: "can a 1929 car, size assumed, drive between the pilotis and turn under the house?" DE: "Fährt ein Auto von 1929 (Maße angenommen) zwischen den Pilotis durch und wendet unter dem Haus?" Line 598: "Auto um 1929 · Karosserie schematisch, Maße angenommen" / "Car of c. 1929 · generic body, size assumed". |
| **V-B3** | Unit-less numbers on screen (agent-rules line 25: "Never … a bare number without its object and unit"). EN and DE alike. See the locations column in the next row. | Dimension labels and formula terms without `m`. Code: line 498 (`self.number(spacing)` on the E/3–E/4 dimension, shown in `context` and `place`); line 525 (`self.number(x1 - x0)` on each chain bay, `chain`); line 534 ("4 × 4.75 = 19.00 m", middle term has no unit); line 537 ("2 × 1.25 = 2.50 m", `1.25` has no unit); line 302 (`self.number(lim["margin"])` for margins under 0.8 m, `fits` and `sensitive`); lines 311–312 (need row "1.75 + 2 × 0.50 = 2.75 ≤ 4.45 m", only the last term has a unit); line 688 (caveat "3.50 − 0.25 · assumed", no units). Approx. positions in EN: `context` (392–443, 128–150); `chain` bay labels (200–540, 128–150) and remainder row (1033–1225, 388–408); `fits` margins (875–920 and 1018–1060, 400–422) and need row (694–1034, 525–548); `sensitive` margins (same spots, reading "1.07"); `properties` caveat (890–1225, 445–467). | Route every value through `self.quantity(…, "m")`. For narrow margin labels, keep the unit and drop the font size or move the label to a leader; do not drop the unit. Need row: "1.75 m + 2 × 0.50 m = 2.75 m ≤ 4.45 m". The `wide` switch at line 301 must not remove the unit. Apply the same to `context`, `place` and `chain` bay labels. |

### Majors

| Key | Location | Defect | Fix |
|---|---|---|---|
| **V-M1** | `relations`, "Connected to" leader, code line 728 `at(19.0, 7.1)`. EN approx. dot at (533, 407), inside the slab hatch between C/4 and B/5. DE same. | The row says the first-floor slab is connected (IfcRelConnectsElements), but its leader ends in empty slab hatch, not at a piloti head or connector. Audit A-04 asked for a connector at the E/3 piloti head. Narration says "the floor slab it carries". The value is drawn as a leader only, so the connection stays abstract. | End the leader at the E/3 piloti head (or the nearest candidate ring) and draw a short CONCRETE connector labelled IfcRelConnectsElements. Keep it `always_redraw` with the slab elevation. |
| **V-M2** | `pinch` (line 660–667, vignettes at approx. (698–1238, 103–585)), narration `pinch` EN line 83 and DE line 84. `fits` narration line 73 uses a "safety margin on each side". Model: `clearance()` decides fit with `path["min_clearance"] > 0` (model py ~line 417); `min_side_margin` = 0.50 m (JSON line 2003). | The clip requires a 0.50 m margin on each side (`fits`, line 303–304 and the need row). `pinch` then shows the car passing E/3 at 0.42 m and D/2 at 0.24 m, both below 0.50 m (`path_pinches` returns 0.418 and 0.237). The narration says "the car still clears them", so the same clip contradicts its own margin rule without saying so. | Pick one rule and show it. Either (a) show each pinch against the margin: "0.42 m · below 0.50 m margin, clear" and say the margin applies to straight bays, not to the curve; or (b) make the path rule the margin rule (`min_clearance >= min_side_margin`) and redesign the path. Narration must name the rule. |
| **V-M3** | `properties`: line 680 (material basis hard-coded `self._word("sourced")`), line 682 (LoadBearing basis hard-coded `self._word("inferred")`). `chain`: lines 535–537 (`sourced`/`open` hard-coded), line 536 (box 21.50 m `sourced`). Model: `information()` returns material basis `assumed` and LoadBearing basis `sourced` (JSON line 313, VS-3.1). Box length is JSON line 262 `sourced`, VS-2.1. | Every number on screen must come from the model (agent-rules line 27). Basis chips are typed in the scene and contradict the model: the screen says material "sourced" and LoadBearing "inferred", the model says material "assumed" and LoadBearing "sourced". The r2 report calls this an override; it is a second source of truth. Box length is shown "sourced" although research fact 2.1 is "medium (one source)". | Render every basis chip from `info["basis"]`. Model: LoadBearing basis `inferred` (source VS-3.1 as the system basis); material basis `sourced` (VS-3.1, research 3.1); box length basis `medium` or a new `one source` value. Remove the hard-coded words. |
| **V-M4** | `chain`: lines 526 (WARN remainder dimensions drawn with `None` label), 531 and 537 (1.25 m per side is `gx[0]` taken as symmetric). EN approx. outer ticks (152–188 and 555–590, 140–160). No labelled dimension for the 19.00 m or 21.50 m totals. | Audit A-02 asked that every dimension carry its value. The r2 report says "every dimension has a label"; that is false for the two outer remainder dimensions. The totals 19.00 m and 21.50 m appear only in the table, not on the drawing. The symmetric 1.25 m split is shown as fact and is not sourced (research 3.4 "lengths not found"). | Label both remainder dimensions "1.25 m" in WARN with "not sourced" in the sub-line. Add labelled overall dimensions "19.00 m" (grid) and "21.50 m" (box, sourced one source). Say "symmetric split assumed" or drop the split from the formula. |
| **V-M5** | Plan conventions across `context`–`handoff` (plan area approx. (157–583, 172–548)). Audit D-01 partly open. | The walls are outlines only, with no poché (`villa_plan` draws line-only walls). No section cut A–A links the plan to the column elevation in `properties` (`_column`, lines 431–456; the "1" badge is a view badge only). Audit D-01 asked for a cut, a level triangle and poché. Scale bar, north arrow, grid bubbles and solid piloti discs are now present. | Add a section mark A–A through E/3 with a head bubble, and draw ground walls with poché (solid, 0.2 m). A ±0.00 level triangle at the entrance is optional. |

### Minors

| Key | Location | Defect | Fix |
|---|---|---|---|
| **V-m1** | `same` and `handoff`, yellow trunk top (code line 762, y = 2.52), cyan "People read the drawing" route (line 746). EN and DE at approx. (362–1000, 128–136). | Trunk and people leader run about 5 px apart over about 300 px, so they read as one double line. | Move the trunk top to about 2.40 or start the trunk from the people leader. |
| **V-m2** | `chain` remainder leader, code line 540, at y = `at(0, 0.6)` ≈ −1.96; north arrow at line 507, y = −1.95. EN and DE approx. (583–655, 530–540). | The orange leader runs straight through the N arrow glyph. | Re-route to pass right of the arrow or move the arrow (e.g. to the top-right of the plan). |
| **V-m3** | `gauge`, lines 322–348. EN and DE at approx. (725–1212, 500–535). | No tick scale: no 0 m, 1 m … 5 m ticks, so the bar cannot be read in metres except through the labels. | Add ticks at each metre from 0 to 5 m with small numerals. |
| **V-m4** | `sensitive`, margin labels (line 302). EN and DE approx. (875–920, 400–422) and (1018–1060, 400–422). | "1.07" is the float result of `round(1.075, 2)` (model `_round`). Half-up gives 1.08. Printed values do not add up: 1.07 + 2.00 + 1.07 = 4.14, not 4.15. | Round half-up in `_round` (Decimal ROUND_HALF_UP) or show 1.08. |
| **V-m5** | Ø labels: bay `Ø 0.30 m` (line 285, WARN, `fits`/`clear`/`diameter`), properties `Ø 0.30 m` (line 448, `properties`). EN approx. bay (742–845, 300–320), properties (675–778, 205–232). | 0.30 m is assumed (research 2.12: not found; the slideshow gives radius 30 cm). Only the 0.60 m sensitivity has a "weak source" chip. The 0.30 m value carries WARN colour but no word. | Add the "assumed" chip next to the first Ø label in `diameter` and in `properties`. |
| **V-m6** | `chain` line 536 chip "sourced" for 21.50 m; `meaning` line 553 sub-line "teaching-model basis, not an IFC property" under a "sourced" chip. | Research fact 2.1 is medium, one source. The sub-line "one source" in the same row contradicts the "sourced" chip. The `meaning` sub-line is also inconsistent with "sourced". | Use "one source" chip (INFERRED tone) for box length. Rewrite the `meaning` sub-line to say what the basis means. |
| **V-m7** | `relations`, count note at line 717 (EN approx. (735–1190, 425–500)). | Floats below the table, away from the row it qualifies. Audit L-06 not done. | Put the count status into the "Defined by type" row sub-line and delete the floating paragraph. |
| **V-m8** | `drive` legend, "Distance driven 25.3 m" (lines 653–654). | Value without drawn meaning: no measure runs along the path. | Draw a progress ruler along the swept centreline that grows with `drive_t` and ends at the label. |
| **V-m9** | Plan circles: dashed turning circle at line 590 (5.50 m), glass arc `glass_end` at line 414 (3.50 m), R_turn dimension at line 417. EN `question` (215–435, 200–420) and `wall`. | The drawn geometry encodes the unsourced 5.50 m and 3.50 m at scale, with no "assumed size" mark on the drawing. Audit accepted drawn geometry; the rule "state the principle, not a radius" still asks for an honest mark. | Add a small "assumed size · radius not sourced" tag on the dashed circle and at the R_turn dimension. |
| **V-m10** | Typeset numbers: `_tex` (line 147) puts `\,` before the unit, so formulas read "4.75m", "0.30m", "4.45m" (`clear`, `fits`, `wall` with "4.75m − 0.30m = 4.45m"). DE "4,75m". Elsewhere "4.75 m". | Typography inconsistent with the rest of the clip. | Use the same thin-space convention everywhere, or `\mathrm{m}` with a full space. |
| **V-m11** | `relations`, sub-line "IfcGrid · axes E and 3" (line 716). EN and DE. | IfcGrid and IfcGridAxis are not covered in `research-ifc.md`. The audit says the name should be verified (N-08). | Verify against IFC 4.3.2.0 and add the fact to `research-ifc.md`, or drop the class name. |
| **V-m12** | `properties`, level label "+3.50" (line 695, EN approx. (786–850, 290–310)). | No unit and no basis at the point where the level is drawn. The relations row shows "assumed" but the drawing does not. | Write "+3.50 m · assumed" once on the level label. |
| **V-m13** | `properties` as a beat: height 3.25 m, Ø and load arrows are static (`_column`). | Audit D-02 asked for trackers. No value in this beat changes, so the "changing a value changes the drawing" rule does not bite. Minor only. | Add a height tracker only if a sensitivity step is wanted (NB-3 covers the sensitivity idea). |
| **V-m14** | Model note `piloti-diameter` (JSON about lines 26–30). Not on screen. | "no source gives them" is wrong: research 2.12 cites a student slideshow with radius 30 cm (weak source). The clip's "Ø: weak source" chip depends on that source. | Rewrite the note: "Ø 0.30 m assumed; one weak student slideshow gives radius 0.30 m (Ø 0.60 m)". |

## 3. Audit-r2 status

| Audit item | Status | Evidence |
|---|---|---|
| N-01 printed radii (blocker) | Resolved | No radius printed in `question`, `wall` or the car table. Glass formula is symbolic with "radii not sourced" chip. Drawn circle remains (V-m9). |
| N-02 piloti × 25 (blocker) | **Not resolved** | "× 25" removed, but the count is stated in the note and 25 rings are lit (V-B1). |
| A-01 glass radius drawn (blocker) | Resolved | `wall` draws centre dot, R_glass arc, c tick, b_car, R_turn dimension. |
| N-03 Ø sensitivity | Resolved | `sensitive` runs Ø 0.60 m with "Ø: weak source" chip. Model note still wrong (V-m14). |
| N-04 3.25 m basis | Resolved | Caveat "· assumed"; units missing (V-B3). |
| N-05 LoadBearing sourced | Partly | Screen says "inferred" but the basis is hard-coded and the model says "sourced" (V-M3). |
| N-07 colours and verdict | Resolved | Per-symbol colours in `clear` and `fits`; verdict "fits · assumed" in WARN. |
| A-02 clear-width label | Resolved | "4.45 m" labelled on the clear-width dimension. The chain's outer dimensions are a new instance (V-M4). |
| A-03 handoff leaders | Resolved | Elbow trunk, taps and GlobalId chip. Overlap noted (V-m1). |
| A-04 relation leaders | Partly | Storey, type, grid leaders drawn; slab leader lands in empty space (V-M1). |
| S-01 raw bare number | Resolved | `raw` shows only the number; ghost plan gone. |
| S-02 people arrow target | Resolved | Leader ends at the E/3 ring. |
| S-06 tint vs slab | Largely resolved | Storey is an outline, slab is hatched; both still cover the full footprint. |
| L-01 STEP contrast | Resolved | Grey lines readable at zoom. |
| D-01 plan conventions | Partly | Scale, north, level marks, solid piloti present. No poché, no section cut (V-M5). |
| D-02 moving values | Partly, minor | Spacing, Ø in bay, car, drive and gauge move. Properties values do not change (V-m13). |
| D-03 subtitle and norm chip | Resolved | "Villa Savoye · Ground floor" and "IFC 4.3 · ISO 16739-1:2024". |
| D-04 typeset formulas | Resolved | `s − Ø = w` colour-coded, substituted row present. Units missing (V-B3). |
| G-02 one IFC Name | Resolved | "Piloti E3" in EN and DE. |
| G-03 one minus | Resolved | Math minus U+2212 in all formulas checked. |
| G-04 heading case | Resolved | All-caps headings in both languages. |

Audit claim "every dimension has a label" is false (V-M4).

## 4. Checks that passed

- **Sync (4):** narration and visual agree in both languages for `raw` to `same`, with the exceptions V-M1 (`relations`) and V-M2 (`pinch`).
- **Arithmetic (2):** 4 × 4.75 = 19.00; 19.00 + 2 × 1.25 = 21.50; 4.75 − 0.30 = 4.45; 1.35 + 1.75 + 1.35 = 4.45; 1.75 + 2 × 0.50 = 2.75 ≤ 4.45; threshold 1.75 + 0.30 + 2 × 0.50 = 3.05 (model: 3.05); weak case 2.00 + 0.60 + 2 × 0.50 = 3.60 (model: 3.60); 4.75 − 0.60 = 4.15; 3.50 − 0.25 = 3.25; R_glass = 5.50 − 1.75 − 0.25 = 3.50 (model: curve_radius 3.5); path 25.28 m shown as 25.3 m. Only the 1.07 rounding fails (V-m4).
- **Model vs screen (2):** spacing 4.75 and initial 5.00 (both sourced, VS-2.10); car 1.75 × 4.60 (assumed); Ø 0.30 (assumed); height 3.25 (assumed). Pinch values 0.42 and 0.24 match `path_pinches`.
- **Research vs screen (2):** grid reduced from 5.00 to 4.75 for cost (VS-2.10, high). "Principle sourced, radii not sourced" matches research 5.1. "Ground floor ±0.00" and "Poissy" match research 10.7. Count shows "count open" (V-B1 is the exception).
- **IFC (2, research-ifc.md):** IFCCOLUMN with PredefinedType COLUMN; LoadBearing and IsExternal as Pset_ColumnCommon; IfcRelContainedInSpatialStructure, IfcRelDefinesByType (IfcColumnType), IfcRelConnectsElements, IfcSlab, IfcMaterial, IFC 4.3 / ISO 16739-1:2024. STEP: IFCCOLUMN 9 attributes in schema order; IFCPROPERTYSINGLEVALUE 4; IFCPROPERTYSET 5; IFCRELDEFINESBYPROPERTIES 6. Both GlobalIds are 22 characters from the IFC alphabet and identical in every frame that shows them.
- **German (5):** all 21 frames complete; decimal comma everywhere (4,75; 0,30; 1,25; 2,50; 1,35; 3,05; 3,60; 25,3; +3,50; ±0,00); minus U+2212; "belegt / angenommen / gefolgert / offen / passt" consistent with the chip words; Name "Piloti E3" identical in both.
- **Layout checker (tooling):** the 4 strict findings per language are false positives. Zoom confirmed the "5.00 m" label, the "1.35" margin labels and the "0.24 m" label are clean. The "H−", "H=", "H," and "Hcar" entries are measurement probes with no visible glyphs. The checker should ignore invisible probe text.
- **Depth (6):** the energy clips use a subtitle chip, a top-right standard chip ("DIN V 18599-2 · DIN 4108-2"), typeset formulas and a 0–1 gauge with threshold. This clip has all of those and more (plan, magnified bay, column, STEP). Parity achieved. Formulas are smaller than the energy clip's display formula (19.5–26 vs about 40); not a defect on its own.
- **Accepted by design:** `raw` shows the bare number 4.75. This is the point of the beat ("This number is data. Alone …"). If the owner reads "never a bare number" literally, it still needs a decision.

## 5. Files

- Report: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/verify-r2-information.md`
- Crops: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/verify-information/`
- Scratch scripts: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/verify-information/verify-information-crops.py` and `…/verify-information-model-check.py`
