# 🌳 Final Sweep: Grasshopper Data Trees (Beat5_DataTrees)

Series: `🦗️grasshopper`. Scene: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py`, class `Beat5_DataTrees`.
Frames: `🗑️generated/frames/🦗️grasshopper/Beat5_DataTrees/{en,de}/`. Read-only sweep, nothing edited except this report.

## Verdict: FIX (1 item)

## Freshness

- Scene file last written 2026-10-29 04:04:32.
- All 24 narrated PNGs per language are dated 04:04 to 04:09 on the same day, so they match the current narration.
- Extra file `stats.png` (en 23:08, de 23:15 on 2026-10-28) is not in the narration. It is stale and orphaned, so it does not affect the clip. Delete it from the frames folder at the next cleanup.

## Clause-by-clause review

Clause order is the `NARRATION` order in the scene file. Each frame was read in both languages.

| # | Clause | en | de | Notes |
|---|---|---|---|---|
| 1 | hook | OK | OK | |
| 2 | flat | OK | OK | |
| 3 | tree | OK | OK | |
| 4 | branch | OK | OK | |
| 5 | path | OK | OK | |
| 6 | item | OK | OK | |
| 7 | panel | OK | OK | Bare "70" in the panel is labelled by the header "70 branches". Not flagged. |
| 8 | flatten_one | OK | OK | |
| 9 | flatten | OK | OK | |
| 10 | graft_one | OK | OK | |
| 11 | graft | OK | OK | |
| 12 | simplify_one | OK | OK | |
| 13 | simplify | OK | OK | |
| 14 | mapper_example | OK | OK | See check below. |
| 15 | mapper | OK | OK | |
| 16 | perstorey | OK | OK | |
| 17 | pertower | OK | OK | |
| 18 | flip | OK | OK | |
| 19 | match | OK | OK | |
| 20 | mistake | OK | OK | Chip "no error" is intentional; caption "classic mistake" is not contradicted. |
| 21 | why | OK | OK | |
| 22 | fix | OK | OK | |
| 23 | result | OK | OK | |
| 24 | recap | OK | **FIX** | See item 1. |

## Fixes

1. **de / recap** (`de/recap.png`): the row labels read "Datenbaum (Tree)", "Ast (Branch)", "Pfad (Path)", "Element (Item)". These are English words in the German video. Fix: drop the English in parentheses, so the labels read "Datenbaum", "Ast", "Pfad", "Element". Keep "Index", "Flatten", "Graft", "Simplify", "Path Mapper", "Flip Matrix", which are real Grasshopper UI names. The German caption "Zehn Begriffe, eine Adresse" stays as it is.

## Checks not counted as defects

- **mapper_example** (en and de): the fourth overlay letter is struck through. The code sets it to `D` in pink (`zip("ABCD", ...)`), so it should read D, matching the caption "D is 1. D, the facade, is dropped." At full resolution it looked close to B. I could not zoom because PIL is not installed here. Recommend one 2x look before final render. Not counted as a fix.
- **Tree Statistics** (panel, de): the English component name appears in the German caption. This is a real Grasshopper UI name, so it is allowed under the brief. Not flagged.

## Numbers checked against the 🏢nakagin model

Computed with `facade_storeys()` from the model:

- Tower A: 9 capsule storeys, 3 facades each, 27 branches, 54 capsules.
- Tower B: 11 capsule storeys, 4 facades on the first 10 storeys and 3 on the top storey, 43 branches, 86 capsules.
- Total: 70 branches, 140 items, 20 storeys.

Every frame and caption number matches: 70 branches, 140 items, 54/86 capsules, 20 branches in the mapper, "storey 4: 8 capsules", "storey 10: 6 capsules", "storey 9: 4 facades". No wrong numbers.

## Summary

- Verdict FIX: one item, the German recap labels "Datenbaum (Tree)" and similar.
- All 24 clauses read in both languages. Layout is clean and captions wrap cleanly. Captions match pictures.
- Numbers match the Nakagin model in every frame and caption.
- Open check: the struck letter in mapper_example should be confirmed as D at 2x.
- Housekeeping: `stats.png` in both language folders is stale and orphaned (23:08 and 23:15 on 2026-10-28).
