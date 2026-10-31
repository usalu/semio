# Morph Audit: grasshopper capsule (`Beat2_Capsule`)

Brief: `morph-audit-brief.md`. Plan: `morph-grasshopper-capsule.md`. Done: `morph-grasshopper-capsule-done.md`. Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py` (mtime 2026-10-29 16:25). Frames: `🗑️generated/morph/🦗️grasshopper/Beat2_Capsule/{en,de}/`.

Frame age cutoff: 15:25 (scene mtime minus 60 min). Stale frames are listed, not judged.

- Stale (not judged): en `hook-01/03/04/05/end`, `parametric-*`, `sliders-04/05/end`, `plane-04/05/end`, `box-04/05/end`, `frontplane-04/05/06/end`, `bolts-04/05/end`, `types-04/05/end`. All de frames are newer than the cutoff and were judged.
- Missing: no beat frames are missing. `index.md` (en) has no rows for hook, parametric, sliders, plane, box, frontplane, bolts, types or listitem, and `index.md` (de) has no rows for hook through types. The index is out of date with the frames on disk.

## 1. Beat verdicts (mid frame cited)

| Beat | Clause | Dir | Verdict | Evidence |
| --- | --- | --- | --- | --- |
| M1 | hook | C->A | WEAK | de `hook-04-mid.png`: capsules become a dark hull swarm that spills past the Definition group into the perspective pane. Count panel readable at end only (`hook-end.png`; verify the count text, it reads 148 at this scale). |
| M2 | parametric | A->C | WEAK | de `parametric-05-mid.png`: red "2.50 m" token readable on the wire. `parametric-06-mid.png`: the token becomes a dark red hull blob, not a dimension line. |
| M3 | sliders | C->A | WEAK | de `sliders-05-mid.png`: teal and blue hull blobs, no readable dimension lines. Card rows readable at `sliders-end.png`. |
| M4 | plane | C->A | WEAK | de `plane-05-mid.png`: grey hull blob next to the origin panel. Panel readable at end. |
| M5 | box | C->A | WEAK | de `box-04-mid.png`: box absent, centre dot faded. `box-05-mid.png`: ghost box, no growth from the plane. |
| M6 | lift | C->A, A->C | BUILT (partial) | en and de `lift-05-mid.png`: green "2.00 m" and blue "1.25 m" tokens readable on A/B, colour carried. No visible tether to the dimension line. |
| M7 | rounded | A->C | WEAK | en and de `rounded-05-mid.png`: violet "0.25 m" token at the Fillet Edge radius input, readable. `rounded-06-mid.png`: box to shell has no visible intermediate, reads as a crossfade. |
| M8 | frontplane | C->A, A->C | BUILT (partial) | de `frontplane-05-mid.png`: orange ring on the front face, colour carried. `frontplane-04-mid.png`: floor-plane label and red circle pale. |
| M9 | cut | A->C | BUILT | en and de `cut-04/05-mid.png`: orange ring on the front face, `cut-06-mid.png`: dark hole in the face. Prism stage faint. |
| M10 | bolts | C->A | WEAK | de `bolts-05-mid.png`: grey hull blob crossing the canvas, no ring shapes. `bolts-end.png`: indexed points 0..3 readable. |
| M11 | types | C->A, A->C | WEAK | de `types-04-mid.png`: mini capsules present, no morph into row dots. `types-05-mid.png`: table just fades in. |
| M12 | orient | C->A, A->C | WEAK | en and de `orient-05-mid.png`: only a small dot, no collapse to plane-A token. `orient-07-mid.png`: capsule readable at plane B (re-grow). |
| M12b | rotate | A->C | WEAK | en `rotate-04/05-mid.png`: grey hull blob at the core, leftover blob at lower left of the perspective pane. Angle pill "-90"/"-180" readable at `rotate-05/06`. |
| M13 | listitem | A->C | WEAK | en and de `listitem-05-mid.png`: teal/orange hulls in flight, colour readable, shapes blocky. |
| M13b | wrap | A->C | BUILT (partial) | en and de `wrap-06-mid.png`: type-coloured capsules on planes 0..7, readable with index tags. The index 4 loop arrow is not visible in `wrap-04/05`. |
| M14 | recap | C->A | WEAK | en and de `recap-04-mid.png`: dark grey hull blobs over the canvas. `recap-05-mid.png`: icons readable, but they appear without a visible morph. |

Counts: BUILT 3 (cut, plus partial lift, frontplane, wrap). WEAK 11. MISSING 0.

## 2. Remaining fade-swaps and cuts

- Every clause still opens with FadeOut then FadeIn (`index.md` rows n=1, n=2) for lift, rounded, cut, orient, rotate, wrap, recap, and listitem in de. These are scene-level swaps between concrete and abstract stages, not object morphs.
- Plan says sliders, types and recap had fade-swaps replaced. The frames show fade-ins in `types-05` and `recap-05`.
- `recap` row 6 is FadeOut, which removes the icons without a morph back.

## 3. Direction and chains

- Both directions occur: C->A in hook, sliders, plane, box, bolts, types, recap and listitem. A->C in parametric, lift, rounded, frontplane, cut, orient re-grow, wrap and rotate.
- Chain of one object: the window ring (frontplane -> cut) holds identity best. The capsule chain (hook -> box -> shell -> plane A -> capsule at B) is broken at hook, rounded and orient.

## 4. Collisions and clutter

- `hook-04` (de): hull swarm crosses the Definition group boundary into the perspective pane.
- `sliders-05` (de): teal and blue blobs over the perspective pane and the live card; blue blob left on the box.
- `bolts-05` (de): grey blob over the Merge components and canvas.
- `rotate-04/05` (en): grey-blue blob left on the canvas, not cleared.
- `recap-04` (en and de): dark hull blobs over the Orient and List Item nodes.
- `types-05` (de): red capsule overlaps the table.

## Verdict: FIX

Fix list:
1. hook: replace the whole-capsule hull swarm with small silhouettes gathered into the count panel; keep the lit capsule as a ghost; clip to the canvas.
2. parametric: morph the width token into the dimension line, not a hull (de-06).
3. sliders: replace the teal and blue hulls with value tokens that become the dimension lines (de-05).
4. box and plane: make the box grow from the plane visible (box-04 is empty); replace the origin hull with the three colour axes (plane-05).
5. bolts: replace the grey hull with four ring shapes that travel to the point panel (bolts-05).
6. types: make the mini capsules morph into the row dots; do not let the table fade in (types-04/05).
7. rotate: replace the arc hull with the arc; clear the leftover blob (rotate-04/05, en).
8. recap: replace the dark hulls with the real objects morphing into the icons (recap-04, en and de).
9. orient: show the capsule collapsing into the plane-A token and travelling to B (orient-05).
10. listitem: use part morphs (tokens into capsules), not hulls (listitem-05, en and de).
11. Remove the clause-start FadeOut/FadeIn pairs where a morph can carry identity, and make the index-4 loop arrow visible in wrap.
12. Regenerate `index.md` (en and de) so it lists every key on disk, including listitem in en.
