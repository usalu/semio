# Final Morph Review: Grasshopper Tower (Beat3_Tower)

Scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py` (line numbers below refer to it). Judged from code intent plus the en mid frames (`🗑️generated/morph/🦗️grasshopper/Beat3_Tower/en`: hook, orient, count, mistake, fix, capital, towers, recap read). Frames are 640x360; the part-matching toolkit is in use (capsules travel as real shapes).

## Per clause

| Clause | Verdict | Why |
| --- | --- | --- |
| hook | GOOD | 140 real capsule outlines of both towers stream into the `140` number token (L242); frame shows the stream leaving the tower. |
| parts | GOOD | core, capsules, capital and podium outlines gather into their coloured group badges (L248); planes badge only fades in (no source), minor. |
| core | ACCEPTABLE | real core silhouette morphs into the live parametric core (L266); but the "side x side x storeys x height" label is plain text, the sliders `side`/`sh` are not tied to the drawing by a token. |
| shaft | ACCEPTABLE | core box to detailed shaft (L271); concrete to concrete, no abstract partner (envelope vs assembly idea is only in the label). |
| series | ACCEPTABLE | shaft morphs into the core part of the level tower, planes fade in with it (L281); the abstract "Series elevation list" on the canvas never becomes the slab planes. |
| facades | GOOD | a level plane splits into the four frame axes (L289), identity transfer plane to frames is visible. |
| axes | GOOD | one frame morphs into the X/Y/Z axes (L295). |
| items | GOOD | axes split into plane tokens that sit on the candidate capsules of one storey (L302). |
| orient | GOOD | plane tokens scatter into the first-storey capsule outlines (L306): abstract plane becomes concrete capsule, the intended chapter. Mid frame shows glyph soup in the label row (retext of the previous note), distracting. |
| types | GOOD | neutral outlines recolour into the four type colours (L310); legend appears at the same time. Colour carry-over is exactly the hero idea. |
| count | ACCEPTABLE | typed capsules gather into the live sum token (L320), then the slider sweep is a tracker redraw (L321-323). Gather happens once, not per level; the Sigma label overlaps the legend in the frame. |
| nowire | WEAK | pure `ValueTracker` sweep (L322); no morph, and the narration (wires unchanged, height changes) has no concrete token. Acceptable only as a short hold. |
| mistake | GOOD | index tokens fly into phantom capsules (L339); frame shows orange phantoms arriving; the Sigma label collides with the tokens at y -2.3. |
| fix | GOOD | phantoms shrink into red Cull Index tokens (L346), the deletion event is shown. Label overlaps token row (frame: "Unoccupied" and "Sigma documented" stacked). |
| capital | GOOD | the real capital silhouette morphs into the parametric capital; slider number tokens fly from the canvas sliders to the drawing (L361). Frame shows previous labels still overlapped. |
| ontop | ACCEPTABLE | capital morphs into the rising tower, storey token flies from slider to tower top (L371); then a tracker sweep (L373). |
| podium | GOOD | the canvas Rectangle component (footprint) morphs into the podium drawing (L378), a true abstract to concrete move. |
| why | ACCEPTABLE | podium morphs into the podium part of the whole building (L382); concrete to concrete, "named assemblies" idea only in the label. |
| towers | GOOD | A (teal) and B (orange) capsules gather into the 76 and 64 tokens (L392); colours match tower tints. Overlap of the old note "Shared named..." with the new one is visible in the frame. |
| total | GOOD | 76 and 64 split back into the real capsules of both towers (L399): the back-link exists. |
| swap | ACCEPTABLE | one B and one C capsule of the tower morph into the cutaway programs (L407); no visible "switch" actor, the program change B to C is shown as two separate objects rather than one capsule changing. |
| recap | ACCEPTABLE | the five group badges split into core, capsule, capital and podium outlines (L412); planes has no source. Frame shows large grey slabs flying in, legible only as "parts return", not as a clean mapping. |

Tally: GOOD 11, ACCEPTABLE 8, WEAK 1 (nowire). Both directions are present and chained (capsules to 140 to groups to colours to sum to Cull Index tokens to 76/64 and back; badges back to tower parts), the hero identity (capsule, type colour) is carried across clauses.

## Verdict: SHIP with minor polish

The video is genuinely morph-driven: every explanatory clause that has an abstract counterpart (number, plane, index, group, type, slider value) transfers identity to or from a real capsule or part. Remaining issues are label hygiene and a few tracker sweeps, not missing morphs. Highest-impact polish (optional, at most five):

1. Label collisions: notes at y = -2.3 overlap token rows and the previous label in mistake, fix, capital, towers (`label()` L230, `stage` retext L198-200). Move the token rows to y -2.0 or fade the previous note before the retext and tokens start.
2. count/ontop: replace the sweep (L321, L373) with 3-4 short gather cycles per level (level capsules into the Sigma token), then keep the tracker only for the final reveal.
3. nowire (L322): add the Sigma token flying back to the slider (`morph(growing_token, slider)`), or hold without the sweep; at minimum shorten it.
4. swap (L407): morph the B capsule into the C capsule on the tower (same object, program colour changes) before the cutaways, so the switch has an actor; keep the porthole ring as the persistent mark.
5. recap (L412) and parts (L249): give the planes badge a source (the plane axes tokens or a few capsules' planes) so all five parts have a morph, and keep the flying slabs smaller.

Also note: the capital frame shows 10.90 m while narration says 10.83 m; the frames pre-date the narration fix, recapture to confirm.
