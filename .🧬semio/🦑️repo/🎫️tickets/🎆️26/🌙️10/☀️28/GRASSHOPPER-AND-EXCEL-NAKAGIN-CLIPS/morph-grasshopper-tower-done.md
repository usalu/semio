# Morph Done: Grasshopper Tower (Beat3_Tower)

Scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`. Storyboard: `morph-grasshopper-tower.md`. Implemented with `🔀️morph` (`morph`, `scatter`, `gather`, `split`, `token`, `silhouette`), small diffs on top of the codex remodel. The `stage()` helper got a `via={"drawing"|"note"|"network": [animations]}` slot that replaces the fade of that slot by morphs, plus `done=` for adoption of loose targets; helpers `shapes`, `sils`, `adopt`, `settle`.

## Beats implemented (one per clause)

| Clause | Direction | Morph |
| --- | --- | --- |
| hook | concrete to abstract | 140 capsule outlines of both towers gather into the number badge `140` |
| parts | concrete to abstract | core, 140 capsules, capital and podium outlines gather into their coloured group badges (Planes badge fades in) |
| core | concrete to concrete | the core of the tower flies into the live parametric core drawing |
| shaft | concrete to concrete | the core box morphs into the detailed shaft assembly |
| series | concrete to concrete | the shaft morphs into the tower with one plane per level |
| facades | abstract to concrete | the level plane splits into the four door-port frames |
| axes | abstract to concrete | one frame morphs into the X, Y, Z axes |
| items | abstract to concrete | the axes split into plane tokens placed on the candidate capsules of one storey |
| orient | abstract to concrete | the plane tokens fly into the capsule outlines of the first storey; the tower arrives |
| types | abstract to concrete | neutral outlines recolour into the type colours (colour carries over) |
| count | concrete to abstract | the typed capsules gather into the live sum token (follows the slider) |
| mistake | abstract to concrete | candidate index tokens fly into the phantom capsules |
| fix | concrete to abstract | the phantoms fly into red Cull Index tokens next to the sum |
| capital | concrete to concrete | the capital of the tower morphs into the parametric capital drawing |
| ontop | concrete to concrete | the capital morphs into the rising tower |
| podium | concrete to concrete | the tower morphs into the podium envelope |
| why | concrete to concrete | the podium morphs into the whole building |
| towers | concrete to abstract | A (teal) and B (orange) capsules gather into number badges 76 and 64 |
| total | abstract to concrete | the badges split back into the capsules of both towers |
| swap | concrete to concrete | one type B and one type C capsule of the tower morph into the two cutaway programs |
| recap | abstract to concrete | the five group badges split into the core, capsule, capital and podium outlines of the tower |

Counts: concrete to abstract 5 (hook, parts, count, fix, towers), abstract to concrete 7 (facades, axes, items, orient, types, mistake, total, recap, 8 counting both), chained (capsules to badge 76/64 and back to capsules; capsules to Cull Index tokens). All numbers come from the model (`total`, `totals()`, engine lengths, `masks`).

## Narration changes (three-line captions at the caption bar, en and de, transcripts updated by hand because `sync-transcripts.py` fails on f-string narration of another scene)

facades (en, de shortened), swap (en, de shortened), de only: orient, mistake, why. Counts in the narration (76, 64, 140) match the model. `gh-tower/captions.py` checks every clause of both languages (0 too long after the edits).

## Frames checked (mid frames, `🗑️generated/morph/🦗️grasshopper/Beat3_Tower/{en,de}`)

en: hook, parts, core, shaft, series, facades, axes, items, orient, types, count, mistake, fix, capital, ontop, podium, towers, total, swap, recap. de: core, series, facades, mistake, fix, capital, count, orient, types, towers, total, swap, recap captured; fix, swap and recap read.

## Final state

After the toolkit changed to part-matching morphs, the key morph clauses (hook, parts, core, orient, types, count, mistake, fix, towers, total, swap, recap) were re-captured in en and de (`🗑️generated/morph/…/{en,de}`); capsules now travel as real shapes. Labels, badges and number tokens were moved from the row under the canvas (hidden by the caption bar) into the bottom of the canvas frame (y = -2.3) and drawn in the dark text colour. Layout keyframes (`capture.py`, both languages) were captured before this move; hook, parts, types, count, mistake, fix, towers, total, recap were re-captured in both languages (strict findings 17 in 4 of 9 frames en, 17 in 3 of 7 de, all slider/grip text). The presentation contract tests were not run (a full run takes too long while the capture jobs of other agents load the machine).

## Open items

- Mid frames of the large detail swaps (shaft, series, podium, why) read as a blurred outline blob before the details arrive; they are identity-preserving but not elegant. Candidates: part-wise morphs and `capsule_pairs` of the toolkit.
- The narrated label row at y = -2.88 sits under the caption bar in the present layout; the new tokens sit at y = -2.3 inside the canvas frame. The old labels are hidden by captions (not changed here, existing layout).
- Capture of one run takes about 30 minutes (live canvases, 11 to 20 s per slider frame).
- `gh-tower/captions.py` replaced a file of the same name from an earlier round.

## Second round (coordinator message: full recapture, blobs, label retext, slider morphs)

- Full captures en and de with the part-matching toolkit (`🗑️generated/morph/…/{en,de}`, all 22 keys read at least at the mid frame). Hull blobs are gone for the capsule, core and capital morphs.
- Large detail swaps (shaft, series, podium, why, ontop) were still passing through a big grey box (hull of the whole tower). Replaced by part lifts: series morphs the shaft into the core part of the new tower while the tower fades in; podium morphs the real Rectangle component of the canvas (the footprint) into the podium drawing; why morphs the podium into the podium part of the whole building; ontop morphs the capital drawing into the head of the rising tower. Mid frames re-read in en and de: real shapes travel, no blob.
- Label crossfade between clauses is now `retext` (note slot, plain labels only). Mid frames show glyph soup for about a second when the two sentences differ completely; the end state is clean.
- Slider morphs: capital (height and overhang number tokens fly from the sliders of the canvas to the drawing and fade after the clause) and ontop (the Capsule storeys slider value flies to the tower top). `nowire` has no concrete counterpart (the visible canvas has no slider in that view), so it stays a slider sweep.
- Caption lengths: series (de) and ontop (en, de) were three lines after the codex rewrite; shortened (numbers kept), transcripts edited; `gh-tower/captions.py` reports 0 too long.

## Lessons pass (general lessons 1 to 12 of `morph-review-excel-references.md`)

1. One actor: every morph moves the capsule, part, component or number itself; the cards only follow. OK.
2. Persist the bound pair: cull tokens, the sum token, group badges and number badges 76 and 64 stay; capital and storey tokens persist for the whole clause and fade after it (they would be stale after the slider sweep that follows). OK with that note.
3. Move the number: sum and badge tokens leave the capsules (gather); the slider tokens carry the value. OK.
4. Tracker sweeps: the count, nowire, capital and ontop sweeps stay as `ValueTracker` redraws because the drawing is a continuous build of the tower and the sum is a live label; the discrete counting is carried by the gather into the token at the start. Kept deliberately (not converted to per-level gather loops).
5. Actual event: the culled capsules fly into the Cull Index tokens (deletion as removal into the list). OK.
6. Structure before morph: window and card swaps run concurrently with the morph in one clause; the canvas fade is the only crossfade. Open.
7. Legibility: number tokens use size 1.1 to 1.5; the 140 and 76/64 streams are many small flyers by nature (gather of the real capsules). Open for the stream count.
8. Frames at final size: keyframes read at 1280 by `capture.py`; the morph mids are 640 wide.
9. Both directions: C to A (hook, parts, count, fix, towers) and A to C (facades, axes, items, orient, types, mistake, total, recap, podium, ontop), chained capsule to badge to capsule.
10. Narration match: the morphs start with the clause; the count morph ends before "sum" is read. Not timed to key words.
11. Back-link: `total` splits the 76 and 64 badges back into the capsules, `recap` splits the group badges into the tower parts.
12. Toolkit hygiene: default `parts` mode, no `path` tokens, recap uses copies of real mobjects.

## Final polish round

- Narration numbers are now derived from the model with f-strings (`gh-tower/derive.py`): total, per-tower counts (`totals()`), podium roof (`podium_height()`), storey pitch, capital heights and core-roof heights of A and B. The capital figures were 11.36 m (A) and 10.83 m (B); the 10.90 m seen in an old frame was stale (the capital slider reads 12.45 m after the sweep, 10.83 x 1.15). The German transcripts already carry the same values. `sync-transcripts.py` cannot parse f-strings; transcripts stay hand-synced.
- Note collisions: the cull tokens of `mistake` moved to the right of the label (x >= 0.6, y = -2.3); `stage` default is 2 s, so crossfades between rows are shorter. Sequential fade-then-fade (`Succession`) was tested and does not leave its targets in the scene, so it was not used; the overlap that remains is the transient 2 s crossfade of the old and new row.
- swap: one capsule changes program: the type B capsule of the tower morphs into the cutaway drawing of type C (tint and window change, the porthole stays).
- planes badge: born from a plane token over the tower in `parts`, and the badge splits back into a plane token in `recap`.
- nowire: a storey value token flies from the tower top towards the formula row while the tracker sweeps.
- One full capture en and de (`🗑️generated/morph/…/{en,de}`, 22 keys each, no errors); changed frames read (parts, nowire, swap, mistake de, capital).
