# 🔍 Audit: excel references (`Beat2_References`)

Scope: scene `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py`, storyboard, scene report, plan row, `research-excel.md`, `research-nakagin.md`, Nakagin model `🏢️nakagin/🔣️.json`, workbook model `📊️excel/🎬️clip/🧱️model/🔣️.json`, engine values (`nk.slots()`), and all 40 keyframes (20 clauses, en and de) with `layout.md`.

## Verdict

Not publishable yet. One blocker: in the German frames the formulas and chips name sheets (`'Typ A'`, `Klima`, `Energie`) that the tab strip does not show (`Type A`, `Climate`, `Energy`). Seven majors follow: the `lights` clause contradicts its drawing, `###` and truncated `#BEZUG!` labels hide the taught values, parentheses render as square brackets in every frame, and two card labels and one overlap are wrong. All numbers, physics and Excel behaviour check out against the engine. Narration is 20 clauses, each at most two caption lines, and "teaching model" is said once per language.

## Defects per frame

| frame/key | lang | severity | defect | fix suggestion |
| --- | --- | --- | --- | --- |
| sheet, stuck, threed, span, names, deleted, guard, recap, fixed, f4 | de | blocker | Tab strip shows `Program`, `Type A`–`D`, `Climate`, `Energy`. Formula bar, chips and cards show `'Typ A'!B9`, `'Typ A'!B10`, `SUMME['Typ A:Typ D'!B13]`, `Klima!$B$2`, `'Typ C'!B9`, `Klima!B2`. The model defines `title.de` = Programm, Typ A–D, Klima, Energie, so the engine knows the German names but the tab strip ignores them. A German viewer cannot find the sheet the formula names. Same frame: chips read "Typ A", tabs read "Type B". | Render the tab strip with the localised title in DE (`Programm`, `Typ A`–`D`, `Klima`, `Energie`). Formulas, chips and tabs then use one name. Check the threed and span frames after the change. |
| lights | en, de | major | Row mode highlights only the face of the current row; earlier faces go dark. The narration says "every surface lights up ... as its U·A value appears" (cumulative). At F8 only the front/window face is lit. The back wall (row 9) is never lit, and F9 already shows `1,88 W/K` in the relative frames, before the fill. | Highlight all faces up to the current row, as the loss mode does (`index + 3 <= row`). Say "six surfaces", or fill F9 with a sentence explaining the attached factor. |
| stuck | en, de | major | Program C6 shows `###`. `'Type A'!B11` (30.9 W/K) inherits the m² format and overflows column C. The teaching value (wrong number) is unreadable at the moment it matters. | Widen Program column C, or give the copied cell a readable format. |
| deleted | de | major | Error labels truncated: `#BEZ…` in F4:F8, `#BE…` in C6 and row 8. The taught label is `#BEZUG!`; the English frame shows `#REF!` untruncated. | Widen columns C and F for the German layout and re-capture. |
| guard | de | major | Card tag `#BEZUG!` runs into the `=` of `='Typ C'!B9`. | Give the DE tag more width, or place it above the formula line. |
| repair | en, de | major | The corrected formula `=F4*$D$10` sits in G4 but its card tag reads `F4`. The first card row (`=F4*D11`) has no tag. | Tag both rows with their cell (`G4`), e.g. "G4 before" and "G4 after". |
| slide | en, de | major | The wrong values (5 W, 6 W, 1 W, 0 W) appear with no correct value beside them. The correct losses (100, 120, 120, 59, 62, 38 W) are not on screen, so the contrast the clause needs is missing. | Show the expected value in the card (e.g. "G4 should be 120 W") or keep the correct column as a ghost next to the wrong one. |
| all 40 frames | en, de | major | Parentheses render as square brackets in the toolkit font: unit `W/[m²·K]`, `=COUNTIFS[…]`, `=SUMME[…]`, `=F3*[theta_i-theta_e]`, card and recap formulas. Viewers will read these as different syntax. | Verify the font's `(` and `)` glyphs (or substitute a font that renders them), then re-capture all frames. |
| fixed, f4 | en, de | minor (known) | Formula bar shows `-` as `_`: `=Klima!B2_Klima!B3`. Listed, not dwelt on. | Covered by the toolkit font fix above. |
| hook | en, de | minor | Window cuts row 11 (`Transmission H_T`) at its bottom edge. Cell texts truncated: `Einzelwohnung mit Bett, Na…`, `Rechte Seitenwa…`, `Rückwand zum K…`. | Show rows 1–10 fully, or widen columns C and D. |
| all frames | en, de | minor | Top ~125 px (about 18 %) of every frame is empty above the window; the window sits low-left. | Move the window up or enlarge it to fill the stage between title and caption. |
| mixed | en, de | minor | Formula bar text runs past the window's right edge (to x≈850 of 1280). The separator after `$M$141` is hard to see. | Wrap or shorten the bar view; keep the editor inside the window. |
| names, absolute, repair, slide, threed | en, de | minor | Cell editors (G3, G4, B16) extend past the window edge; the names editor overlaps the names card (x 757–885, y 270–295). | Clip editors to the window width; move the names card right. |
| sheet | en, de | minor | Program header `Anz…` / `Cou…` truncated; thumbnail title "Nakagin.xlsx" overlapped by the search pill. | Widen column B; drop the search pill in the thumbnail. |
| deleted | en, de | minor | Warning icon hides the count `35` in B6 (Type C row). | Move the error indicator or widen B. |
| span | en, de | minor | Tab strip scrolled so `Type A` (the span start) is off screen; the span is never bracketed. | Show all tabs, and bracket the span from the first to the last sheet. |
| fill2d | en, de | minor | Towers are not labelled A/B on the drawing. Only the last cell's type and tower light up, and no count appears (narration says "counts those capsules"). | Label the towers; show the count (e.g. 14, 22) in the active cell or card. |
| f4 | en, de | minor | Tags `F4 ×0` … `F4 ×3` are cryptic. | Use "1st F4" … "4th F4", or label with the form (`D10`, `$D$10`, `D$10`, `$D10`). |
| recap | de | minor | The 3D row's rule text auto-shrinks ("dieselbe Zelle auf allen Registern dazwischen") while the others do not. The caption breaks as "Blatt-" / "und 3D-Bezüge". The Mixed example `=$A11*B$10` is not clause 9's `COUNTIFS`. | Set the rule texts to one size and wrap by width; use the clause-9 formula or say "illustrative". |
| sheet, stuck, deleted | en, de | minor | Totals `1223.6 m²` / `1223,6 m²` have no thousands separator (rule: en `1,223.6`, de `1.223,6`). | Use a grouped number format for Program totals. |
| all frames | en, de | minor | "von oben" / "von unten" labels are muted on the dark stage, near the 16 px floor. | Raise the contrast of the view labels. |
| stuck, names, slide, repair, absolute | en, de | minor | Card captions start in lower case: "the sheet name is text…", "der Blattname…", "die Eingabe …", "gleicher Wert, lesbar". | Capitalise the caption starts. |

## Fact findings

Verified (no change needed):
- Excel names in German frames: `ZÄHLENWENNS` (COUNTIFS), `SUMME` (SUM), `#BEZUG!` (#REF!), `;` separators and decimal comma, `'…'!` quoting. Consistent with `research-excel.md` §4–§6, which marks these `[K]`/`[V]`.
- F4 cycle D10 → $D$10 → D$10 → $D10 matches Excel's order (research §2.1 marks it `[CHECK]`; behaviour is standard Excel).
- 3D sum: `SUM('Type A:Type D'!B13)` adds one cell per sheet. Deleting Type C shrinks it silently; moving Type B past D removes it. Both match research §3.2 and the 3D notes.
- Engine values: width 2.5, length 4.0, height 2.5, interior 2.3 × 3.8 × 2.1, window Ø 1.3 (Type C 1.6), U roof 0.5, floor 0.6, wall 0.6, window 2.33, ΔT 20 K, n 0.5, b 0.5. All match the Nakagin model.
- Counts A 36, B 35, C 34, D 35 match `nk.slots()` (sum 140). `research-nakagin.md` §3.2 (40/36/44/20) is superseded by the model, so the clip is right to use the model.
- Physics: D3 = 2.5 × 4.0 = 10.0 m²; front wall 2.5 × 2.5 − 1.33 = 4.9 m² (shown `4,9`); H_T 30.92 W/K; H_V 3.12 W/K; heating load per type 681 W (A, B, D) and 704 W (C), so Σ 2,747 W; without B 2,066 W; without C 2,042 W (both match the frames). Loss = U·A × 20 K: 100, 120, 120, 120, 59, 62, 38 W.
- Mixed table: B11 … E12 sum to 36, 35, 34, 35 per type and 54/86 per tower, matching Program B4:B7.
- Narration: "teaching model" once per language (hook). The Nakagin history clauses are not in this clip.

Issues:
- de, shift: missing comma. "Eine Zeile tiefer kopiert wird aus …" should be "Eine Zeile tiefer kopiert, wird aus …".
- de, fill2d: "In einer Bewegung nach rechts und unten ausgefüllt:" is clumsy. Suggest "Nach rechts und unten in einem Zug ausgefüllt: …".
- de, sheet: "Anführungszeichen" for the sheet-name apostrophe; use "Apostrophe". The EN clause also says "Program reads type A's net floor area"; the DE clause omits it, so the two languages carry different content.
- de, deleted: "Blatt der Art C löschen" lacks the article. Suggest "Das Blatt der Art C löschen".
- de, slide: "Dollarzeichen vergessen" → "Das Dollarzeichen vergessen".
- en/de, recap: "a name reads well" / "Namen lesen sich gut" are awkward. Suggest "a name makes a reference readable" / "Namen machen Bezüge lesbar".
- en/de, names and hook: the workbook labels are θi / θe, while the narration says theta_i / theta_e (names kept as ASCII in the formula). Consider θi / θe in the narration.
- en/de, fixed: D10 is called "input" but holds a formula (`=Climate!B2-Climate!B3`). Say "design temperature difference cell".
- de, Program labels `Typ A`–`D` (A4:A7, B10:E10) stay English on the tabs. Known, listed only.
- Not verified: the German error tooltip ("Die Formel verweist auf eine nicht mehr vorhandene Zelle."). Mark `[CHECK]` in the German UI before filming.

## Didactic findings

- Arc holds: hook → pointer → relative → shift → lights → fixed → F4 → absolute → mixed → fill2d → sheet → stuck → threed → span → names → slide → repair → deleted → guard → recap. Misconceptions 3, 4 and 9 and the quoted-sheet-name point are all addressed. Pointer uses the smallest example (`D3 = B2*B3`).
- Gaps:
  - lights: the drawing does not accumulate (above).
  - slide/repair: no correct value on screen next to the wrong one.
  - stuck: the clause says "each type needs its own sheet name", but the storyboard's fix (edit each copy to B, C, D) is not shown.
  - names: the storyboard's Name Manager / name-box step is not shown; only the formula text swaps.
  - relative: the back-wall row F9 is pre-filled with a different formula (`×attached factor`), which the clause ("one formula per surface") does not explain.
  - mixed: the two-way table uses the full 140-row COUNTIFS. A 2 × 2 example would introduce the idea on the smallest case.
- Plan row deviations (`plan.md` video row): absolute reference `$B$1` (interior temperature) became `$D$10` (design ΔT), accepted in the scene report. 3D example `'Type A:Type C'!B12` became `'Type A:Type D'!B13` (heating load, a better teaching value). "Copied formula with shift arrows" became Δ-text tags, with no arrows on the tokens.

## Side effects

- The engine check imported the Nakagin and workbook modules from the shell. This may have written `__pycache__` bytecode under the teaching folders. No source file was edited and no git command was run.
