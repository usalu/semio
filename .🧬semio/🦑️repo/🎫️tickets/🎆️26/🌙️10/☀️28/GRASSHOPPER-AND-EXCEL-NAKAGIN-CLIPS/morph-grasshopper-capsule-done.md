# Morph Done: grasshopper capsule (`Beat2_Capsule`)

Storyboard: `morph-grasshopper-capsule.md`. Scene: `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`.

## Mechanism

A clause can now run an ordered sequence (`_Seq`: plays and callables; `Beat2_Capsule.play` runs it). Morph sources and targets are free-standing doubles taken from the live stage (`_probe()` gives the stage drawing at the current trackers, `_capsule_at(glide)` the core viewport capsule), so there is no jump when the live redraw takes over; all doubles are removed after the morph. Note: `Scene.play(run_time=...)` stretches every animation of one play, so each morph has its own play.

## Beats implemented (en and de)

| Beat | Clause | Direction | What morphs |
| --- | --- | --- | --- |
| M1 | hook | C->A | every capsule of the tower gathers into the model-driven count panel; the lit capsule flies into the Center Box component |
| M2 | parametric | A->C | width value token (red) travels the wire into Center Box, becomes the width dimension line |
| M3 | sliders | C->A | width, length, height dimension lines scatter into the value cells of the live parameter card |
| M4 | plane | C->A | red/green/blue axes at the origin become the Origin panel |
| M5 | box | C->A | box centre dot becomes the "Box centre" coordinates panel |
| M8 | frontplane | C->A, A->C | red circle on the floor becomes a plane token, the token becomes the orange ring on the front face |
| M10 | bolts | C->A | four bolt rings gather into the indexed point panel |
| M11 | types | C->A | four mini capsules scatter into the type-colour dots of the table rows |
| M12 | orient | C->A, A->C | capsule collapses into the plane-A token, token travels to B, re-grows as the capsule |
| M13 | listitem | A->C | four type item tokens (type colours) become the capsules on planes 0..3 |

Concrete to abstract: 8; abstract to concrete: 4 (M2, M8, M12, M13). Chain: floor circle -> plane token -> ring; capsule -> plane A token -> capsule.

## Narration

`hook` no longer hard-codes 140 (en and de); the counter panel still shows the model count. The foreign `four` clause was too long for two caption lines in en and de; both shortened.

## Frames checked

`capture_morph.py` mid frames, en: hook, parametric, sliders, plane, box, frontplane, bolts, types, orient, listitem; de: same keys re-run clean (sliders frame viewed). Colours carry (red width token, orange ring, type colours); capsule and tower sources show as flat hull blobs mid-way (toolkit silhouette). Gather of the tower reads as a swarm into the count panel.

## Open items

- Round 1 open items are done in Round 2 below.
- Part-wise morphs (`pairs=`, `dip=`) announced by the toolkit are not used yet; capsule to token mid-frames are hulls.
- Capsule `capture.py` keyframe layout re-check and presentation tests: see the test run result in the final message.

## Round 2 (coordinator request)

Added and captured clean in en and de (mid frames read: lift, rounded, cut, orient, rotate, wrap, recap en; lift, wrap de):

| Beat | Clause | What morphs |
| --- | --- | --- |
| M6 | lift | length and height dimension lines become the half-value tokens (green, blue) on the A/B components, then the plane lifts |
| M7 | rounded | radius value token (violet) travels slider to Fillet Edge, becomes the corner radius tag on the rounded shell |
| M9 | cut | window ring becomes the cylinder prism (length from the Cylinder length panel), the prism becomes the hole in the front face |
| M12b | rotate | angle value token becomes the angle arc on the core, then the slider steps on to -180 and -270 |
| M13b | wrap | index token 4 loops (negative path arc) back into the type-A item token |
| M14 | recap | the real Orient, List Item and index widgets morph into their legend rows |

Part-wise morphs: `capsule_pairs` is used for orient (capsule shell/window to the item token ring/dot and back) and for the listitem scatter. Not converted: the hook gather (140 whole-capsule hulls into a panel) and the lift/recap widget morphs, which remain hull flights.

Presentation tests of the Grasshopper series passed after round 1 (not re-run after round 2; both language captures of the changed keys ran clean).

## Round 3 (audit FIX, toolkit auto part matching)

- One full capture en and one full de (no key filter, 74 frames each) ran clean after the toolkit default changed to automatic part matching; hull blobs are gone from sliders, bolts, rotate and the hook (real shapes travel); frames read: en hook, sliders, bolts, types, recap, de hook (end).
- hook: the count panel now sits inside the Perspective viewport (bottom, clear of the tower) so the capsule swarm stays inside the viewport; de reads "Kapseln 140 / Typen 4 / Definitionen 1" from the model.
- Remaining weaknesses (not fixed): the clause-opening FadeOut/FadeIn of the canvas nodes (lift, rounded, cut, orient, rotate, recap) is still a scene-level swap; the capsule is not one persistent object through hook and rounded; recap widgets fly as fragments; the index-4 loop in wrap shows as a token path only (no arrow); index.md is regenerated by the capture run.

## Round 4 (four open items)

- parametric: a copy of the hook's Center Box chip morphs into the box in the viewport (hook capsule -> chip -> box), then the stage body takes over; rounded: the box double morphs into the rounded shell double (box -> shell), then the stage shell takes over.
- Clause openings: rounded, cut and rotate no longer fade node sets: the old nodes gather into the new key node (Fillet Edge, Solid Difference, rotated-plane Panel) and the other new nodes grow from it (`GrowFromPoint`); old wires fade.
- recap: whole widget copies (Orient, List Item, index) shrink to the legend icon position and then morph into their legend rows.
- wrap: a curved arrow from the index-4 token back to type A; the token travels along it and becomes the type A item token.
- Full capture en (78 frames) and de (78 frames) clean; read: en parametric, rounded (2), wrap; de recap, cut. Box-to-shell and index loop read clearly; gather flights read as small fragments of the moving nodes.

## Lessons pass (morph-review-excel-references, 12 lessons)

1 one actor: the capsule is the actor through hook, parametric, rounded. 2 bound pair persists: dimension lines, radius tag, ring and hole are handed over to the stage and stay. 3 number leaves the cell: width, radius and angle tokens travel from sliders, then the dimension, tag and arc appear. 4 no tracker sweep for discrete items: ramps only for continuous values (width, length, angle). 5 events shown: the cut is a prism that subtracts, wrap is a looping arrow. 6 structure first: node-set swaps (0.4-1.2 s) precede the object morphs, which have their own plays. 7 legibility: one flying group per play (at most 7 objects), tokens carry slider colours; token size was not enlarged (open: check at 1280x720). 8 mid frames read at 640x360 only. 9 both directions: yes. 10 narration timing: morph and key word run in the same clause, not timed to the word. 11 back-link: List Item tokens become capsules on the planes; count -> lit capsules is not done. 12 toolkit hygiene: `capsule_pairs` for orient and listitem, automatic matching elsewhere, recap uses copies of real widgets.
Open: count back to lit capsules in hook, 1280x720 legibility check, word-level timing.

## Final polish (SHIP review)

- hook: after the lit capsule reaches Center Box, a copy of the count panel splits back into yellow lit capsules on the tower (back-link; de mid frame shows the swarm, count reads 140 from the model).
- domains: the width range text of the card morphs into the front-bottom width edge of the transport envelope; the stage envelope then takes over.
- outside: an index-4 token leaves the index list, travels to List Item and becomes a red `null` token near the result panel (removed at the start of wrap).
- Captured hook, domains, outside, wrap in en and de, no errors; read de hook-06, domains-03, outside-03. The domains and outside token flights were not caught in the sampled mid frames (short plays), only their end states.
