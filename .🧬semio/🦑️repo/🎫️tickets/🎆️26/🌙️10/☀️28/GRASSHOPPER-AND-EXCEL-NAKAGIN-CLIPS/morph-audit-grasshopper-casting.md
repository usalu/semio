# Morph audit: grasshopper casting (`Beat4_Casting`)

Verdict: **FIX**

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` (`present_casting`, lines 608-809).
Plan: `morph-grasshopper-casting.md`. Done: `morph-grasshopper-casting-done.md`.
Frames: `🗑️generated/morph/🦗️grasshopper/Beat4_Casting/{en,de}/<key>-03-mid.png`, `<key>-04-mid.png`, `<key>-end.png`.

Note: `index.md` (en and de) lists only locale, order, matrix_fill and recap. It is stale or from a partial capture, so it cannot be used to confirm animation types for the other clauses. Frames for `mismatch` and `ties` morph do not exist (see below).

## 1. Planned beats: BUILT / WEAK / MISSING

Frame paths are relative to `🗑️generated/morph/🦗️grasshopper/Beat4_Casting/en/` unless marked de.

| # | Clause | Status | Evidence |
| --- | --- | --- | --- |
| 1 | types | WEAK | `types-03-mid.png`: scatter sources are tiny marks on the tower; new title "Every parameter has a type" overlaps the Plane/Number chips; old "Width 2.50 m / slot plane / world plane" pills still visible. |
| 2 | mismatch | MISSING | No morph, no tether. Code (line 639) is a plain `FadeIn(ghost)`. No `mismatch-*.png` exists. |
| 3 | text_number | WEAK | `text_number-03-mid.png`: Text chip "2.5" is a small badge, no visible travel. `text_number-04-mid.png`: teal crude polygon hull with window ring. Plan's dimension-line target is not built. |
| 4 | storeys | WEAK | `storeys-03-mid.png`: old canvas title "Number into Integer" overlaps ghost labels. `storeys-04-mid.png` (also de): tan hull sits left of the tower body, not on the lit storeys. Plan numbers 10.6/11 are superseded by 8.6/9 (intentional, see done file). |
| 5 | ties | MISSING | No morph: line 672 fades the column out and the number line in. `ties-03-mid.png` shows a static number line with arrows, no dot-to-ring morph. |
| 6 | deliberate | WEAK | `deliberate-03-mid.png`: eleventh-storey hull offset left of tower. `deliberate-04-mid.png`: no red crossed tick readable. |
| 7 | point_plane | BUILT | `point_plane-03-mid.png`: gold origin dot on slot capsule. `point_plane-04-mid.png`: "Slot origin" gold chip at Plane input, origin readable. No tether (note only). |
| 8 | wrong_way | WEAK | `wrong_way-03-mid.png`, `wrong_way-04-mid.png` (also de): red translucent square with N, 90° and W tags on top of the window. Red capsule not readable. |
| 9 | line_plane | BUILT | `line_plane-03-mid.png`: blue Z stroke on capsule top edge. `line_plane-04-mid.png`: Z, X, Y axes solid on capsule. Stroke is thin; plan's dashed "guessed" X/Y not built. |
| 10 | explicit | WEAK | `explicit-03-mid.png`: ghost remnants of previous canvas ("Plane at", "Orient") overlap the new canvas and the title. `explicit-04-mid.png`: chips gone, stray red translucent hexagons left of capsule, no plane square visible. |
| 11 | domain | WEAK | `domain-03-mid.png`: "4.0 m" chip on canvas, no length edge on capsule. `domain-04-mid.png`: gold bar floats on the ground beside the capsule, not along the length edge; no cut ticks on capsule. |
| 12 | geometry | WEAK | `geometry-03-mid.png`: capsule body becomes a flat gold blob; rings not visible. `geometry-04-mid.png`: "Circle → Curve" and "Box → Brep" chips are pale grey on a pale sheet (low contrast). |
| 13 | bool_int | WEAK | `bool_int-03-mid.png`: chip "1" on canvas, no travel. `bool_int-04-mid.png`: green ring appears on window while chip "1" stays put. |
| 14 | int_bool | WEAK | `int_bool-03-mid.png`: "True" chip stays on canvas; overlapping ghost labels "0", "3", "Window". `int_bool-04-mid.png`: orange ring appears, chip does not travel. |
| 15 | locale | WEAK | `locale-03-mid.png`, `locale-04-mid.png` (also de): comma-to-"!" token renders as a tiny pale-red circle with a dash-like glyph; ghost canvas overlaps the title. |
| 16 | fail / order / fix | WEAK | `fail-03-mid.png`: ghost capsule still intact. `fail-04-mid.png`: pale grey "null" blob, capsule gone (crossfade look). `order-03-mid.png` (also de): ladder drawn over remnant "Center Box" and error bubble, texts collide. `fix-03-mid.png`: green "2.5" chip over the input pins. `fix-04-mid.png`: teal crude hull with ring, not a capsule silhouette. Chain break: `fix` creates a new chip "2.5" instead of morphing the Value List item. |
| 17 | matrix_fill | WEAK | `matrix_fill-03-mid.png`: several travelling pills have no text; one grey blob in mid-air above Plane/Curve. |
| 18 | recap | WEAK | `recap-03-mid.png`: overlapping translucent red and green hulls around capsule and habit cards; cells not identifiable. |

Totals: BUILT 2 (point_plane, line_plane), WEAK 14, MISSING 2 (mismatch, ties).

## 2. Remaining fade-swaps (where a morph would show identity)

- `mismatch` (line 639): ghost capsule FadeIn and canvas FadeOut. Should tether the Panel chip to the ghost.
- `ties` (line 672): storey column FadeOut and number line FadeIn. Should morph the lit storey band into the dot on the number line.
- `wrong_way`, `line_plane`, `explicit`, `domain` (lines 694, 703, 712, 720): the same capsule is faded out and a new one faded in each clause. Identity of the capsule is lost.
- `deliberate` (line 683): whole tower and canvas faded instead of the tower being carried.
- `matrix` step (line 796): the capsule is faded out and the matrix faded in. A morph of the capsule outline into the matrix would keep identity.
- `order` (line 784): canvas12 faded out while the ladder fades in.
- `bool_int`, `int_bool`, `fix`: the capsule fades in after the morph instead of being the morph target.

## 3. Direction and chains

- Concrete to abstract (C→A) present: types, point_plane, deliberate, geometry, fail, matrix_fill, locale (failure).
- Abstract to concrete (A→C) present: text_number, storeys, wrong_way, domain, bool_int, int_bool, fix, recap.
- Both directions occur.
- Chain of one object across clauses:
  - Type A token: continuous through fail and order (blank becomes entry). Broken at fix, where a new chip "2.5" is created.
  - Slot origin dot: becomes the origin chip at point_plane, then is removed at line_plane. Not reused in wrong_way.
  - Capsule: re-created by fade at every clause from wrong_way to domain. Not a chain.
  - Matrix cells: seen tokens to cells to recap gather. Continuous.

## 4. Collisions and clutter

- Overlapping titles during transitions: types-03, storeys-03, bool_int-03, int_bool-03, explicit-03, locale-03 (en and de).
- Ghost remnants from the previous canvas drawn over the new canvas: types-03, explicit-03, order-03 (en and de), locale-03.
- Tower-hull offset left of the tower body: storeys-04 (en and de), deliberate-03.
- Pale or low-contrast text: geometry-04 chips, matrix_fill-03 pills, locale-04 token glyph.

## 5. Fix list (exact)

1. Line 639 (`mismatch`): replace the bare FadeIn with a dashed tether from the Panel chip "2.5" to the ghost capsule, and add a `mismatch` frame.
2. Line 672 (`ties`): morph the lit storey band (`column`) into the number-line dot; drop the FadeOut of `column`.
3. `_band` (lines 184-192): fix the hull registration so it sits on the Tower B body (storeys-04, deliberate-03).
4. Lines 692, 698, 707, 720: keep one capsule object across point_plane, wrong_way, line_plane, explicit and domain; morph between them instead of FadeOut/FadeIn.
5. Line 694 and `_stage` line 382: raise the red capsule opacity from 0.5 and remove the N, 90° and W tags until the morph completes.
6. Line 767 (`stop` token) and line 479 (geometry chips): use a visible "!" at size and INK text on filled chips.
7. Lines 741-743 and 753-756 (`one`, `true_chip`): make the chip travel along a path to the ring and remove it on arrival.
8. Line 779 (`fail`): replace the pale grey blob with a part-wise morph of ghost11 into the red `null` token.
9. Line 784 (`order`): FadeOut canvas12 (and its Center Box) before the ladder FadeIn.
10. Lines 791-794 (`fix`): morph the Value List item into chip3, and build the stand-in from the same silhouette as `_standin`.
11. Line 800 (`matrix_fill`): keep token text visible in flight; remove the stray grey blob source.
12. Line 809 (`recap`): fade the cells before the gather, or scale them down, to stop translucent hull overlap.
13. Titles: `_gone` the old title in the same step as the new title (types, storeys, explicit, locale, int_bool).
14. Regenerate `index.md` in en and de so it lists every clause (out of scope for this audit; capture not run).
