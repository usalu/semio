# Final morph review: grasshopper capsule (`Beat2_Capsule`)

Judged from the scene code (1098 lines, step calls and `_Seq` chains), storyboard, done note (rounds 1-4), plan, lessons, and en mid frames (hook, listitem, orient, types, recap). Frames are 640x360, so code intent was weighed too.

## Per clause

| clause | rating | why |
| --- | --- | --- |
| hook | GOOD | Tower capsule crowd gathers into the count panel, the lit capsule morphs into the Center Box chip (C to A, one actor). Weak: frame shows the gather barely begun; count does not return to lit capsules (lesson 11). |
| parametric | GOOD | Chip morphs into the viewport box, then width token travels the wire and becomes the width dimension line; hook capsule to chip to box to dimension is a real chain. |
| sliders | GOOD | Width, length, height dimension lines scatter into the live parameter-card cells, colours kept. |
| plane | GOOD | Base axes morph into the Origin panel; persists as a panel. |
| box | GOOD | Centre dot morphs into the "Box centre" coordinates panel; both ends persist. |
| lift | ACCEPTABLE | Dimension lines become half-value tokens on A/B, but flights are small and the node-set opening is a FadeIn swap. |
| live | WEAK | FadeOut/FadeIn plus succession only; nothing changes form. Live redraw is a continuous quantity, so tolerable. |
| rounded | GOOD | Nodes gather into Fillet Edge, box double morphs into the shell, violet radius token becomes the radius tag. Clear identity transfer. |
| wrongplane | WEAK | Pure node swap, wrong circle/bubble fade in; setup beat, no morph. |
| frontplane | GOOD | Floor circle becomes plane token, token becomes the front-face ring (C to A to C chain). |
| cut | GOOD | Window ring becomes the prism, prism becomes the hole; the subtraction is shown as an event. |
| bolts | GOOD | Four bolt rings gather into the indexed point panel with real shapes. |
| domains | WEAK | Card, azimuth and envelope driven by trackers and fades; the domain range never morphs from or into the capsule. |
| limit | WEAK | Ramp plus red box and request appear; acceptable as a state demo but no abstract/concrete morph. |
| types | GOOD | Four mini capsules scatter into the type-colour rows of the table (frame: type capsules, coloured dots, red type D). |
| four | WEAK | Type tag and trackers only; a caption beat. |
| planes | ACCEPTABLE | Stage swap to the planes glide; plane tokens do not come from the capsule. Setup beat. |
| orient | GOOD | Capsule shell and window pair-morph into the plane-A token, travel to B and regrow as the capsule (capsule_pairs); the frame shows base/target planes and translation on the core. |
| rotate | GOOD | Angle token becomes the arc on the core, nodes gather into the rotated-plane Panel. |
| listitem | GOOD | Type item tokens become the capsules on planes 0..3 (frame: teal and orange capsules with plane tags mid-flight). Best A to C beat. |
| outside | WEAK | Counter to 8 plus a notice fade; the out-of-range index is not shown as a token leaving the list. |
| wrap | GOOD | Index-4 token follows a curved loop arrow and becomes the type A item token. |
| recap | GOOD | Real widget copies shrink and morph into legend rows while the lit capsules wear their index tags (frame confirms rows and eight capsules); strong back-link to the building. |

## Verdict: SHIP

The video is morph-driven where ideas are explained: 14 of 23 clauses carry real identity transfers in both directions (C to A: hook, sliders, plane, box, bolts, types; A to C: parametric, rounded, cut, listitem; chained: frontplane, orient, wrap). The WEAK clauses are setup or caption beats (wrongplane, live, four, planes, outside) where the idea is a state, not an object.

Minor polish (optional, all low-impact):
1. hook: add the reverse morph of the count back into lit tower capsules (`present_capsule` hook `_Seq`, ~line 666), lesson 11.
2. outside: let the index-8 token leave the List Item list and fade at the boundary instead of the counter only (~line 1048).
3. domains: morph the 0-to-width range bar from the width slider into the capsule envelope (~line 932).
4. 1280x720 legibility check of the tokens (lift half-value tokens, bolts panel) is still open from round 4.
5. Clause-opening node-set swaps (lift, live, wrongplane) are still FadeOut/FadeIn.

Summary: SHIP; 14 GOOD morph clauses, both directions chained; five caption/setup clauses WEAK by nature; main gap is the missing count-to-capsules back-link in hook; 1280x720 token legibility unchecked.
