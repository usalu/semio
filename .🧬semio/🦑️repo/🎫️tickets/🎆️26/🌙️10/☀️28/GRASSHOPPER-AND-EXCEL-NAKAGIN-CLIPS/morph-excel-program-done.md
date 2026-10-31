# Morph done: excel / 📋️program / `Beat3_Program`

Scene: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`. Storyboard: `morph-excel-program.md`.

## Mechanism

`play()` of the scene runs queued morphs (`self.later(anim, when="pre"|"post", clean=, adopt=)`) before or after the clause's main animation (`step` stores its `seconds`); the toolkit swarms are played alone, never nested. Helpers: `silhouettes(keys)` (one flat capsule hull per key in its type colour at its place on the tower), `collect(pairs)` (several gathers in one swarm), `fly(sources, targets)` (many to many), `keep()` (register targets as right-panel objects). Gather targets are static labels; the live counter replaces them afterwards (`adopt`), because a `Live` mobject faded in by a swarm stayed invisible.

## Beats implemented (11 of the 14 planned; 24 clauses and keys unchanged)

| Clause | Dir | Morph |
| --- | --- | --- |
| hook | C->A | capsules of each type gather (silhouettes, type colour) into the four count chips |
| sheet | A->C | sheet row labels (fitting name pills) fly onto their fitting rectangles on the plan |
| table | C->A | 10 capsules scatter into row dots of the list, the other 130 gather into the "140 rows" chip; old panel fades first, tower appears before |
| row | A->C | the type pill of the selected row flies out of the cell into its lit capsule |
| countif | C->A | all type-A capsules gather into the counter |
| rest | C->A | the other types' capsules gather into the total counter |
| countifs | A->C | the value pill of B11 splits into the lit type-A capsules of tower A |
| pull | C->A | free-floor texts of the four plans fly into cells C21:C24, then the cells recalculate |
| sumproduct | A->C | count pills of B21:B24 split into one tile per capsule, in the type colour |
| chart | C->A | tile rows gather into the bar bases, the chart grows from them |
| towers | C->A | tower A and tower B capsules gather into F11 and F12 |
| swap | C->C | the capsule lifts out of its slot (empty slot outline), the new capsule drops in and recolours |
| typo | A->C | the red space mark morphs into the ring around the grey capsule |
| fix | C->A | the ring gathers into the TRUE badge |
| recap | C->A | all 140 capsules gather into the "140 capsules" total |

Chains: capsule -> table row -> lit capsule (table, row); mark in cell -> ring -> badge (typo, fix); count pills -> tiles -> bars (sumproduct, chart).
Not implemented: program (fittings to tokens), area/free/volume morphs, xlookup, update (delta chips travelling from the capsule), nested part-wise morphs (`pairs=`) of the new toolkit.

## Verified

- `capture_morph.py` en (all keys) and de (all keys): 0 errors; mid frames read for hook, sheet, table, countif, rest, sumproduct, towers, swap, typo, pull, recap (en) and countif (de). Silhouettes stay readable mid-way, colours carry (teal A, orange B, green C, violet D, tower tints), tethers not used.
- Captions: hook en and de were 3 lines after a concurrent narration edit; shortened (en "Nakagin Tower", de "Nakagin Towers vier Raumprogramme ... jedes ... es"). Narration contract tests (`test_narration_title_and_steps`, imports, series folders) OK. The full `SceneContract` lifecycle test renders every scene and did not finish within 10 minutes; not run to completion.
- Keyframe layout (`capture.py`, both languages) not re-run.

## Open items

- Re-run `capture.py` keyframes en/de and check overlap of the new chip "140 rows" (6.3, 1.75) and the dots on the list column H.
- A pending-file note: while writing the first helper I created and deleted `T/excel-program/apply_edit.py` (an empty file by that name may have existed before; it was untracked). Re-create if needed.
- Concurrent edits to the same scene (hook narration, tower counts 35/33/35/37) were kept.

## Round 2 (coordinator follow-up)

Added: area (fitting rectangles fly into G12:G14 as value pills, the cells recalculate afterwards), volume (interior box collapses into B10), xlookup (the C27 value pill flies into the free floor of plan C), update (+1 / -1 chips fly from B21 / B23 to their places). Capsule to token morphs now use real capsules and `_morph.capsule_pairs` with `dip=0.15`: table (10 capsules scatter to `item` tokens in the list rows) and row (item token to the real lit capsule, pairs reversed). Hook captions rewritten (en: "The 140 capsules of the Nakagin Tower come in four types, each with its own room program. How many of each, and how much floor?"; de: "Die 140 Kapseln des Nakagin Towers gibt es in vier Typen, jeder mit eigenem Raumprogramm. Wie viele sind es je Typ, und wie viel Fläche?"), both within two lines (render passed).

Checked: `capture_morph.py` en (hook sheet area volume table row xlookup update) and de (hook area volume table row xlookup update): no errors; mid frames read in en (table, row, area, volume, xlookup, update). `capture.py` en (table row towers countif): tower and chips visible after fixing that the new tower is added directly instead of through a faded-in pre-animation (a skipped capture dropped it). Not visible in `capture.py` (skipped animations): the "140 rows" chip and the row dots, which appear only in the playing animation (seen in the mid frames); the "Table name" chip and the sum chip touch the tower foot (pre-existing layout).

Not re-captured after the toolkit's newest default (matched parts instead of hulls): hook, countif, rest, countifs, pull, sumproduct, chart, towers, typo, fix, recap still use hull silhouettes from `silhouettes()`; re-capture them with `capture_morph.py` when needed. No processes were killed.

## Round 3 (layout, real capsules, full captures)

- Layout: the table-name chip is now "Name: Capsules" at the upper left of the tower (free column between window and tower); the sum chip in `towers` reads "664.2 + 559.4 = 1,223.6 m²" at y -2.58 (the old chip lay across the foot; the new one still touches the thin foot rail lines but no capsule). The row dots sit right of column M (never over the ids) and now stay until the next window swap (lesson 2: the bound pair persists).
- Hull blobs removed: `silhouettes()` returns real capsule drawings per type colour (`coloured(color)` caches a tint-coloured tower); sources and targets of hook, table, row, countif, rest, countifs, towers, swap, typo, recap are real capsules. countifs: item tokens (ring + dot) become the lit capsules with reversed `capsule_pairs`; towers and recap gather into small item-token sinks (no wide pills, so no stretched sheets).
- New back-link (lesson 11): in `towers` the F13 total pill morphs into a green glow over the whole tower (sum to building).
- Full captures (no key filter), en and de, 0 errors; mid frames read at 640x360 for hook, countif, rest, countifs, pull, sumproduct, chart, towers, typo, fix, recap (en) and towers (de). Keyframes `capture.py` en for table and towers re-read.
- German counts in the sheet differ from English (36/35/33/36 vs 35/33/35/37) because the Nakagin model changed between the two runs (concurrent remodel); not touched.

## Lessons pass (T/morph-review-excel-references.md, 12 lessons)

1 one actor: tokens come from the cell or capsule they describe; the formula card is a caption. OK. 2 persist: lit capsule, row dots, tiles, cell values, tower glow persist; only bridge tokens are removed after landing. Fixed (dots). 3 move the number: area, pull, xlookup, volume send the value pill to the target, tint appears with the arrival (hatches were already placed in the main play; acceptable). 4 trackers: the counts in `rest` (35 to 140), the bars (`chart`, `update`) and the counter still use ValueTrackers over 1.6 to 2 s; replaced the cell-by-cell counting only for hook, countif and towers by gathers; remaining open item: per-type gathers for B5:B7. 5 event: swap lifts the capsule out of an empty slot and drops the new one in; typo shows the space mark becoming the ring. 6 structure before morph: window swap plays together with the old-panel fade-out in a 0.4 s pre step in table, sumproduct and towers; other clauses play morphs after the main animation. 7 legibility: tokens carry type colours; hook, table and recap fly more than 7 capsules by design (many to one). 8 reading size: frames read at 640x360 only; the 1280x720 keyframes were read for table and towers. 9 both directions: yes, chained (capsule to row dot, to lit capsule). 10 timing: morphs run before or after the main animation of the clause, not synchronised to individual words. 11 back-links: row, countifs, xlookup, towers glow, typo, swap. 12 hygiene: part-wise pairs only for capsule to token; gather with ghost not needed.

## Round 4 (final polish, one full capture en + de, 0 errors)

Implemented from `morph-final-excel-program.md`: program (fitting rectangles and the person glyph fly into item tokens under "fittings" and "users"); net (the two dimension labels fly into B9, the value pill morphs into the floor tint, then the cell recalculates); free (the fitting pieces gather into the B16 pill, the pill morphs into the free-floor tint; the "free x m²" label stays); check (the two counts, B8 and B17, gather into the TRUE badge); update (the +1 / -1 chips start at the swapped capsule on the still visible tower, as a pre morph, and fly to their bars). Frames read (en): program-04, net-04, free-04/05, check-04, update-04: the transforming objects stay readable, colours carry.

Not done: rest per-type gathers instead of the tracker sweep, and recap chips morphing into earlier results (still tracker sweep in rest; recap gathers the 140 capsules only).

Hook caption: another session had rewritten the hook to four lines ("... six historical physical capsule families ..."); shortened to "Four proposed room programs, A to D, fill the 140 reconstructed capsules. How many of each, and how much floor?" / "Vier vorgeschlagene Raumprogramme A–D füllen die 140 rekonstruierten Kapseln. Wie viele je Programm, und wie viel Fläche?" (two lines). The distinction from the historical capsule families is therefore not in the caption; add it to the transcript or a later clause if wanted.
