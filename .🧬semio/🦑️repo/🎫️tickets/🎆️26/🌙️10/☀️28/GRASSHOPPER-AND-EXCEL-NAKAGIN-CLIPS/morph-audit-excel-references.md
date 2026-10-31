# Morph audit: excel references (`Beat2_References`)

Scene: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py` (last edit 2026-10-29 16:12:38). Plan: `morph-excel-references.md`. Built report: `morph-excel-references-done.md`.
Frame cutoff: 15:12:38 (scene minus 60 min). Every frame on disk is newer, so none are stale.
Frames: `🗑️generated/morph/📊️excel/Beat2_References/{en,de}/` (index.md + `<key>-<n>-mid.png`, `<key>-end.png`).
Coverage: en has only `mixed2` frames (5 PNG). de has `f4, slide, repair, mixed2, fill2d, stuck, pertype, deleted, recap`. Frames for hook, pointer, shift, lights, back, total, fixed, absolute, sheet, threed, span, guard, names and mixed (count) are not on disk in either language. The done file says round 1 frames were read, but those folders were overwritten (en index 16:27, de index 16:43). Per the brief these are reported as missing; code lines are cited so the gap can be closed by a capture.
The `en/media/v/MorphCapture.mp4` video was not used (brief: no capture, PNG frames only).

## 1. Planned beats (M1-M14)

Status key: BUILT = recognisable object in motion, identity and colour readable, origin visible. WEAK = blob, crossfade, unreadable, collision. MISSING = no frame on disk (code line given).

| beat | clause | direction | status | frame evidence / note |
| --- | --- | --- | --- | --- |
| M1 width dim -> B2 chip | hook | C->A | MISSING | no hook frame; code `present_references` line 304 `morph(dimension_token, cell_token)` |
| M2 B2 ticks, capsules widen, D3/D4/D7/D9 flash | hook | A->C | MISSING | no frame; code line 308 is `set_value` only, no morph of dependants |
| M3 B2/B3 -> width/length dimensions, `=B2*B3` split | pointer | A->C | MISSING | no frame; no split in code (lines 317-325) |
| M4 four hatched faces -> D3/D4/D7/D9 area chips | pointer | C->A | MISSING | no frame; code line 322 `scatter` present |
| M5 `=D3*E3` retexts to `=D4*E4` | shift | A->A | MISSING | no frame; code line 343 `retext` present |
| M6 F3..F9 values -> lit face tints | lights | A->C | MISSING | no morph in code (line 352 fades cursor only) |
| M7 seven U·A tokens gather into F10 | total | A->C | MISSING | no frame; code line 379 `gather` present |
| M8 D10 -> `$D$10`, `D$10`, `$D10` with padlocks | f4 | A->A | WEAK | `de/f4-03-mid.png`: padlocks on the card rows, but the live reference token does not move; reads as a card fade with locks |
| M9 seven tethers G3..G9 -> D10, loss numbers onto faces | absolute | A->C | MISSING | no frame; code line 416 is FadeIn/FadeOut only, no tethers |
| M10a slide: wrong values become red faces | slide | A->C | WEAK | `de/slide-03-mid.png`: red shards in G3..G9 and at the capsule, no ghost or tether to origin; shards overlap the G column text. `de/slide-04-mid.png` is a crossfade between slide and fixed windows |
| M10b repair: corrected loss chips onto faces, `retext` to `$D$10` | repair | A->C | WEAK | `de/repair-03-mid.png`, `de/repair-04-mid.png`: yellow outlined pills on the capsule, no value text on the pills; G column values visible |
| M11 U-values down, dT across, one mixed formula, bars | mixed2 | C->A, A->C | WEAK | `en/mixed2-03..06-mid.png`, `de/mixed2-03..06-mid.png`: three pills (en 04, de 04) with no identity; faces do not become row labels; bars never morph. Leftover "Lock the input" card still on the right in en/de `mixed2-03` |
| M12a lit capsules gather into COUNTIFS count | mixed | C->A | MISSING | no `mixed` frames on disk. Nearest: `de/mixed2-05-mid.png`, `de/mixed2-06-mid.png` show lit capsules and the count, but no gather in progress; code line 542 `gather` |
| M12b count splits back into capsules for next cell | fill2d | A->C | WEAK | `de/fill2d-03-mid.png`: lit capsules already in place, no split chip visible in flight; code line 552 `split` |
| M13a plan floor face -> area chip, `travel` to Program C4 | sheet | C->A | MISSING | no sheet frames; code line 572 `morph` with `pairs=` |
| M13b stuck: wrong chips shatter onto B10..B12 | stuck | C->A | WEAK | `de/stuck-03-mid.png`: shards tiny, sit on C5..C7 cells and the thumbnail. `de/stuck-04-mid.png`: crossfade with type pills over the sheet tab bar and the Summe row |
| M13c four type chips fly from tabs to formula rows | pertype | C->A | WEAK | `de/pertype-03-mid.png`: pills cover the Typ C and Typ D tabs and the Summe row, labels unreadable. `de/pertype-04-mid.png`: the four type capsules appear by FadeIn (see section 2) |
| M14a four capsules -> tab chips -> B13 values -> sum | threed | C->A, A->C | MISSING | no threed frame; values are FadeIn in code line 612 before `gather` on line 613 |
| M14b span: Type B chip leaves bracket, sum shrinks | span | A->A | MISSING | no frame; code line 624 `gather` only |
| M14c Type C chip + value shatter into `#REF!`, sum shrinks | deleted | A->C | WEAK | `de/deleted-03-mid.png`: teal pill covers the `- 680,8` formula text in the thumbnail, not a shatter. `de/deleted-05-mid.png` is a crossfade to the guard card |
| M14d `$D$10` -> `#REF!`, padlock breaks | guard | A->A | MISSING | no frame; code line 686 `retext` |
| M14e `Climate!$B$2` -> `theta_i` pills | names | A->A | MISSING | no frame; code line 711 `retext` |
| M14f six formulas fly into the legend rows | recap | C->A | WEAK | `de/recap-03-mid.png`: labels "Absolut", "Gemischt", "Blatt" are readable, but they fly across the sheet window and the card |

Count: BUILT 0. WEAK 9 (f4, slide, repair, mixed2, fill2d, stuck, pertype, deleted, recap). MISSING 11 (hook x2, pointer x2, shift, lights, total, absolute, mixed, sheet, threed, span, guard, names). Several MISSING beats have code in the scene; the report must not count them as built until frames show them.

## 2. Fade-swaps and cuts that should be morphs (index.md)

- `pertype-04` (de): FadeOut x6 and FadeIn x7. The four type capsules (teal, orange, green, red, 681 W) arrive by FadeIn beside the Energy sheet. This is the concrete end of the type chain (M14); it should be a morph from the tab chips.
- `threed` (code 612): the four B13 values are FadeIn before the gather. Identity is lost at the handover from capsule to value.
- `mixed2-03` (en and de, index): FadeOut of the ten cell chips (code 501), then the chart FadeIn. The U-value faces should morph into row labels and the cell values into bars.
- `mixed2-05` (en and de): FadeOut and FadeIn of the chart; bars are ValueTracker grow, not a morph of the cell token.
- `stuck-04` and `deleted-05` (de): FadeOut x10 or x5 with FadeIn. The wrong values fade out instead of morphing into the Type A cells B10..B12, and the `#REF!` chip fades out instead of morphing into the 3D-sum.
- `f4-03` (de): LaggedStart of card rows, no retext of the live token (M8).
- `absolute` (code 416): FadeIn of frames and locks, with no tethers (M9).
- `lights` (code 352): no morph at all between F values and face tints (M6).
- Window swaps (relative, fixed, repair, fill2d, slide, pertype, sheet, guard, names): acceptable as window changes, since the plan allows them, but the object morphs on them are missing (see section 1).

## 3. Directions and identity chains

- Concrete -> abstract: present in code (pointer faces, sheet floor, mixed count, pertype tabs). Frames show it only in slide (shards), repair (pills onto faces), stuck (shards), recap (labels).
- Abstract -> concrete: present in code (hook dimension to B2 is the only one with a morph of a dimension). Frames show it only in repair (pills onto capsule faces).
- Both directions in each clause: no clause shows both on a frame.
- Width chain (dimension -> B2 -> `=B2*B3` -> D3 -> roof -> bar): no frames; cannot be judged.
- Surface chain (face -> area chip -> F value -> tint): broken at lights (no morph) and at absolute (no tethers).
- Type chain (type capsule -> tab chip -> `'Type C'!` chip -> B13 -> shard): broken at pertype-04 and threed (FadeIn of capsules and values). The Type C shard in deleted is a pill, not a shard of the chip. Recap's type colours are readable on the labels only.

## 4. Collisions and clutter

- `slide-03` (de): shards overlap the G column text and sit on the sheet window edge.
- `stuck-04` and `pertype-03` (de): pills cover the sheet tab bar (Typ C, Typ D) and the Summe row of the Program sheet.
- `deleted-03` (de): teal pill covers the formula text in the thumbnail window.
- `recap-03` (de): term chips fly across the sheet window, the formula card and the Name Manager card at the same time.
- `mixed2-03` (en and de): the "Die Eingabe sperren" / "Lock the input" card from repair stays on the right while the two-way table is shown. This is clutter from the previous clause.
- `repair-03`, `repair-04` (de): yellow pills sit over the capsule top view and on the G column values.

## 5. Verdict

FIX. No beat is BUILT on disk. 9 beats are WEAK and 11 are MISSING (frames or morph code). Fix list:

1. Recapture frames: en for every clause except mixed2, de for hook through names. Rerun the capture, since the round 1 frames were overwritten.
2. `lights` (line 352): replace the FadeIn of the cursor with a morph of F3..F9 values into the face tints (M6).
3. `absolute` (line 416): add the seven tethers from G3..G9 to D10 and the loss-value morphs onto faces (M9).
4. `mixed2` (line 501): morph the U-value faces into row labels and the cell values into bars; remove the leftover "Lock the input" card before the flux shows.
5. `threed` / `pertype` (lines 601-613): morph the four type capsules into tab chips and then into the B13 values; remove the FadeIn of the capsules and values.
6. `slide` / `stuck` (lines 437-438, 586-587): give the shards a ghost at the origin cell and keep them off the G column text and the sheet tab bar.
7. `recap` (line 737): fade the sheet window before the scatter so the term chips do not cross it; keep labels readable.
8. `deleted` (line 676): move the crack to the tab, so the teal pill does not cover the formula text in the thumbnail.
9. `fill2d` (line 552) and `mixed` (line 542): keep the split/gather chip visible in flight; confirm with frames.
10. `f4` (lines 402-404): retext the live reference token itself, not only the card rows.
