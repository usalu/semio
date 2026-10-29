# Verify: grasshopper / trees (`Beat5_DataTrees`)

Scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py`. Fresh frames captured (en, de; 22 each) into `🗑️generated/frames/🦗️grasshopper/Beat5_DataTrees/{en,de}/`. Both runs completed; the en run's final console print crashed on cp1252 encoding after all frames were written, so de was run with UTF-8 stdout. Layout reports: en 6 strict findings in 3 frames (mapper, perstorey, pertower: stroke through the Path Mapper chip, line through T), de the same 6. Zoom crops are in `🗑️generated/zoom/`.

## Verdict

**NOT READY.** The blocker from the audit is fixed and most majors are fixed. Open: the smallest-example rule for Flatten, Graft and Simplify (D1), a wire that crosses the Path Mapper T input in the fix step (new), and two unverified facts (F1 notation, F2 Tree Statistics output names).

## Audit blockers and majors

| audit item | frame | status | evidence |
| --- | --- | --- | --- |
| Path Mapper count line contradicts result (blocker) | mapper, perstorey | FIXED | Count reads `20 branches · 140 items` (en) and `20 Äste · 140 Elemente` (de); diagram leaf level is storey; perstorey chips show 8 and 6. |
| Canvas at x = 5 px, Capsules nub cut (major) | all canvas frames | FIXED | Canvas starts at about x = 28; the left nub is fully visible in hook and flat. |
| Graft Tree `T` unwired (major) | fix | FIXED | Colour panel wired into Graft Tree `T` in both languages. |
| Path Mapper `M` wired from Capsules (major) | fix | FIXED | Zoom shows Capsules dashed wire into `T` only; the rule wire enters `M`. See new finding N1 for the crossing. |
| Simplify dangling apex (major) | simplify | FIXED | Tower nodes are the roots; no apex point. |
| de Panel `teal`/`orange` next to `türkis` (major) | match, mistake, result, rule, fix | FIXED | de Panel reads `türkis` / `orange`; chips match. |
| Colour meanings collide (major) | pertower, perstorey, stats, mistake, match | MOSTLY FIXED | Storey is now green; Tower B is purple (pertower) or orange (match, result); "no error" chip is neutral. Lit highlight is still yellow in several frames, which is a deliberate highlight, not a clash. |
| perstorey: only top storey lit, "eight" never shown (major) | perstorey | FIXED | Chip `B, storey 4: 8 capsules` (de `Geschoss 4: 8 Kapseln`) beside `storey 10: 6 capsules`. The lit 8-capsule storey is small and partly hidden behind Tower A; readable but weak. |
| Level labels disappear on diagram frames (major) | flatten, simplify | PARTLY | graft, perstorey, mapper, pertower, stats now keep labels. flatten shows only Capsules and Branch; simplify drops Building (acceptable, building removed). |
| Flatten and Graft on 140 items, no smallest example (major, D1) | flatten, graft | NOT FIXED | Still `1 branch · 140 items` and `140 branches · 140 items`. |
| Simplify on 70 branches, no one-branch example (major, D1) | simplify | NOT FIXED | Still `70 branches · 140 items`. |
| Count check after mapping (D3) | mapper, perstorey | PARTLY | Counts correct; no explicit `20 × 6/8 = 140` check line. |

All audit blockers are fixed. Remaining audit majors: D1 (smallest example) and the level-label item (partly).

## Audit minors (short)

- FIXED: stats chips name tower B (`B, storey 9: 4 facades`); panel label `Param Viewer: 70 branches`; pertower Tower A now lit; rule chip darkened.
- NOT FIXED: `why` path columns `{0;1;4;2}` vs `{0}` not shown; `flip` left-then-right sequence absent and tree nodes white; `simplify` example `{1;4;2}` not in panel; `mapper` D not struck through; `mistake` silent-wrong tag not drawn; `rule` check badge missing; `recap` tower thumbnail missing; `stats` P/L/C labels and wire to the 70 box unchanged (F2).
- PARTLY: `mistake` chip is neutral now, tag still not on the component.
- Likely FIXED, not zoomed: `path` plate no longer covers the Capsules stub.
- German: FIXED `hook` ("welche Kapsel sitzt wo"), `panel` ("seinem Pfad"), `mapper` ("wird zu"), `fix` ("mit Graft Tree verteilen"), `why` ("zeigt, warum"), `mistake` ("Die linken Kapseln ... in beiden Türmen"), `rule` ("stillen Fehlern"), `recap` ("Datenbaum"). None outstanding.

## New findings

- **N1 (major, overlap).** `fix` en and de: the rule wire from `{A;B;C;D}(i)->{A;B;C}(i)` curves down past the Path Mapper T input and crosses the dashed Capsules wire at the T nub (zoom `🗑️generated/zoom/en-fix-zoom.png`). It reads as if the rule feeds T. Reroute the rule wire so it enters M from the side away from T.
- **N2 (minor).** The strict layout report flags a stroke through the Path Mapper chip in mapper, perstorey and pertower (both languages). Visually the rule wire runs along the component's top edge. Same fix as N1 would clear it.

## Still open facts (check before filming)

- **F1 (check).** `{A;B;C;D}(i)` path-mapper notation is teaching notation, not in `research-grasshopper.md` §3.3. Say so on screen or show the component's numeric expression. Not verified against McNeel docs here (no network).
- **F2 (check).** Tree Statistics outputs labelled `P`, `L`, `C`; the wire to the `70` box leaves from the `P` region. Confirm which output is the branch count.
- **F3 (check).** Input labels `F`, `D`, `P`, `G`/`M` and "Param Viewer" vs "Parameter Viewer" are unverified.

## Facts re-checked

Model values on screen are consistent: 140 capsules; 70 branches / 140 items; mapped 20 branches (9 + 11 storeys) with 6 or 8 capsules each; Tower A 54, Tower B 86; B storey 10 has 3 facades, B storey 4 has 4 facades; `{0;1;4;2}` = building, Tower B, storey 4, South (2). Flatten 1 × 140, Graft 140 × 1, Simplify 70 × 2 (path `{0;0;0}` shown), Flip 70 × 2 → 2 × 70 all match the recap.

## Required before READY

1. Add the one-branch example for Flatten, Graft and Simplify (D1), or justify the 140-item version in the caption.
2. Reroute the rule wire in `fix` so it does not cross the T input (N1).
3. Resolve F1 and F2 against the component docs, then either change the frames or state the notation on screen.
4. Minor list at the author's discretion: `why` path columns, `flip` left-then-right, `simplify` `{1;4;2}` example, `mapper` strike on D, `mistake` tag, `rule` check mark, `recap` tower thumbnail.
