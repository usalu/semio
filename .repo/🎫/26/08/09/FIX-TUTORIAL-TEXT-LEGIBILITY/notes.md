# Fix Tutorial Text Legibility

Ticket: `26/08/09/FIX-TUTORIAL-TEXT-LEGIBILITY`

## Round 3 — extend to the remaining Heating videos ("do the same for heating videos")

Scope: `2_conduction/scene_2.py`, `3_convection/scene_3.py`, `4_internal_heat_gain/scene_4.py`,
`5_solar_heat_gain/scene_5.py`, `final_calculation/merged_scenes.py`.

**Font sizing was already inherited.** All four `scene_N.py` files pull `BODY_FONT_SIZE` /
`LABEL_FONT_SIZE` / `FORMULA_FONT_SIZE` from `manim_fonts.py`, so round 1's scale increase
already applies to them — verified no bespoke `font_size=<number>` literals in any of the four.
`final_calculation/merged_scenes.py` is a different, older file: every size is a hand-picked
literal (15–44), colors are ad hoc hex per class, and it predates `scene_title`/`equation_row`/
`formula_panel` entirely. Normalizing ~50 literals onto the shared scale in a file with
absolute-coordinate layout is a real migration (the skill's own reference.md already flags this
folder as "not yet migrated" legacy) — out of scope for a collision/legibility pass, so left
alone except for one objective bug: `UltimateEnergyBalance.construct()` was missing
`apply_scene_style(self)` entirely, meaning that scene would render on manim's default
background instead of the dark theme every other scene uses. Fixed and confirmed by render.

**Collision checker run against all four `scene_N.py` files.** Two more checker bugs surfaced
and were fixed before trusting results further:

* `set_stroke(width=0)` (the standard way to hide an edge — e.g. two adjacent rectangles
  merging into one on a colour change) renders nothing, but leaves `stroke_opacity` untouched.
  The checker still traced that invisible edge as a line-art obstacle. Split `_visible()` into a
  fill check and a proper `_stroke_opacity()` that zeros out when width is 0, and only outline
  candidates with actual drawn stroke (`_is_line_art`) are checked for text crossings — a filled
  card's own boundary is not a line a title visibly crosses just by sitting on the card.

**Real collisions found and fixed (13 total):**

| File | Fix |
|------|-----|
| `scene_2` Beat1 | Two rectangles merge to the same equilibrium colour but each kept its own stroke — the shared inner edge became an invisible-in-isolation but still-drawn line running through "WARM (GLEICHGEWICHT)". Replaced with one shared outer border, both original strokes dropped to 0. |
| `scene_3` Beat7 | "Kaltluft (−5°C)" label pulled back up onto a cube icon by a counteracting shift; "Warmluft (+21°C) entweicht!" anchored to a curved arrow's whole bounding box (reaching up to the curve's high tip) collided with the card header above it — re-anchored both to fixed, low points near each arrow's start. |
| `scene_4` Beat1 | "Personen"/"Licht" labels sat inside 0.8-unit window squares; this whole stage is uniformly scaled by `_fit_stage`, so a `next_to(..., buff=0.12)` that looked fine pre-scale still ran into the window's top edge post-scale — confirmed by replicating the exact scale in isolation before landing on buff=0.65. |
| `scene_5` Beat2 | The "G [W/m²]" formula label sat inside the sun-ray fan converging on the window; lifted above the fan. |
| `scene_5` Beat2/Beat3 captions | This file's own `FORMULA_EDGE_BUFF=1.0` put the formula panel's own bottom edge inside the caption zone below it (measured: box bottom ≈ −3.27 vs caption top ≈ −3.06) for any caption running close to the frame width — raised to 1.35. |
| `scene_5` Beat3 | "Sonnenstrahlung"/"Reflektiert" anchored to their diagonal rays' *center point* — clearance directly above one point doesn't clear the label's full width across a diagonal line. Re-anchored to each ray's own far endpoint instead, away from where both rays converge. |
| `scene_5` Beat5 | "Dringt in Wohnraum ein" (rendered ≈3.7 units wide) is literally wider than the beam wedge polygon it sits inside (≈2.7 units at its widest) — no position could avoid crossing an edge. Shortened the label to "Dringt ein" instead of continuing to reposition. |
| `scene_5` Beat6 | "Gespeicherte Wärme" sat inside a forest of wavy heat-rise lines; repositioning it chased the label into a *different* fixed-position label near the subtitle. Shortened the wavy lines (18 steps → 11) instead, so the label has real headroom regardless of exact buff. |

Two lessons that came up repeatedly this round, worth keeping in mind for any beat with a
`_fit_stage`/uniform-group-scale call: (1) a `next_to(..., buff=X)` computed against raw,
pre-scale coordinates can still collide post-scale — the only reliable check is the actual
rendered/checked output, not hand arithmetic; (2) nudging a label's position back and forth
against one obstacle can just walk it into a second one — when that happens twice, the fix is
usually to shrink or move the *obstacle* (shorter label text, shorter decorative lines), not to
keep hunting for an ever-smaller gap between two things that were never going to both fit.

### Verification

`check_layout.py` (fixed) over all four `scene_N.py` files: **0 hard findings**, confirmed with
a full re-sweep after each fix. Remaining "near" flags sampled and are the same class of
intentional pairing documented in round 1/2 (stacked caption lines, formula-panel operators,
arrow tips at their target). Six tiny (~0.005–0.24 area) self-overlaps in `scene_4` Beat5 and
`scene_5` Beat8 are pixel-perfect duplicate glyphs during a formula-token merge animation
(a moved copy lands exactly on the row's own copy one frame before the redundant one is
removed) — confirmed invisible by direct render inspection, left as-is.

`final_calculation/merged_scenes.py` was **not** run through `check_layout.py` — its classes
don't follow the `Beat*` naming the checker filters on, and more fundamentally its font/colour
scale isn't the shared one this whole pass is measuring against. Flagged to the user as a
separate migration, not silently done or silently skipped.

Reported: text too small in the finished videos, and some text colliding with other
objects. Both applied to `Cooling/6_lueftungssysteme` and `Heating/1_introduction`.

## 1. Type scale — measured, not eyeballed

A Text's cap height is ≈ `0.0126 × font_size` units against the 8-unit frame, so the old
scale rendered:

| | old | % frame | new | % frame |
|--|--|--|--|--|
| TITLE | 30 | 4.7 % | **34** | 5.3 % |
| SUBTITLE | 20 | 3.2 % | **23** | 3.6 % |
| CAPTION | 22 | 3.5 % | **25** | 4.0 % |
| FORMULA | 26 | 4.1 % | **30** | 4.7 % |
| BODY | 16 | **2.5 %** | **20** | 3.2 % |
| LABEL | 14 | **2.2 %** | **17** | 2.7 % |

Body and label were the complaint and were the two furthest below the ≈ 3 % that stays
readable while a diagram moves. Changed once in `manim_fonts.py`, which is the single
source every scene draws from, so both topics and every future one pick it up.

## 2. A layout checker instead of eyeballing frames

`check_layout.py` renders each `Beat*` with `dry_run` at 3 fps and snapshots the scene at
every animation boundary — the moments a viewer actually reads — reporting text-on-text
overlap, text crossed by an outline (room edge, axis, arrow, bracket), and text leaving the
frame. Mid-animation crossings are ignored on purpose.

Two checker bugs found and fixed while trusting it:

* a line-count guard counted glyphs, not lines, for single-line captions (`Text` vs `VGroup`)
* every `text crossed by Dot` report was a **false positive** from degenerate zero-size
  mobjects manim leaves inside grouped shapes. Confirmed by dumping coordinates: the real
  token was 0.31 units clear while three width-0.0 `Dot`s sat inside the label. Now filtered
  by `MIN_EXTENT`.

## 3. Real collisions fixed

| Where | Collision |
|-------|-----------|
| Cooling Beat5 | `≈ 4 K` label ran into the second night band → moved to the left of the bracket |
| Heating Beat5 | ISO 6946 chip above the section crossed the beat subtitle → moved into the free centre slot of the temperature row and shortened |
| Heating Beat7 | `λ = 2,1` sat on the temperature gradient, which runs corner-to-corner through the layer centre → dropped low in the layer |
| both, 13 captions | the larger caption size pushed hand-broken lines over `CAPTION_MAX_WIDTH`; each half re-wrapped, giving 3-line bars whose background overlapped the formula panel and the route strip |

Captions are now handled structurally rather than by hand: `caption_bar` word-wraps at
`CAPTION_MAX_WIDTH` at fixed size (never shrinking), all 39 hand `\n` breaks in `scene_1.py`
were removed so the wrapper balances the lines, and `CAPTION_MAX_LINES = 2` warns if a clause
ever needs a third line — that is a text-length problem, not a layout one, because a third
line reaches y ≈ -2.1 and collides with content.

## 4. Better air exchange (Heating Beat 3)

Requested separately. The beat drew static `convection_stream` ribbons plus one dot, which
read as "air could move here". Now:

* the opening is tall enough to have an upper and a lower half, which is the physics — with
  a single opening warm air leaves through the top and cold air enters through the bottom
* two path bundles, warm-out and cold-in, run as **continuous looped particle streams** in
  both directions at once, labelled `warme Luft raus` / `kalte Luft rein`
* the tagged parcel now rides the warm route while the exchange keeps running behind it

`smooth_path` / `flow_guides` / `animate_flow` were promoted from `scene_6.py` into
`manim_visuals.py` for this, and `scene_6.py` was switched onto the shared versions —
the local copies were deleted rather than left to drift (1868 chars removed).

## Verification

`check_layout.py` over all 18 beats: **0 findings, 0 caption warnings** in both files.
All 18 render clean at `-ql`. Frames spot-checked for the type change and the new flow.

Skill docs updated with the new scale and the note that raising it widens every label.

## Follow-up (unchanged)

Audio still needs generating for both topics — Cooling 6 beats 7–9 have none and the rest
are stale; Heating 1 has none at all.

## Round 2 — user reported collisions persisting, subtitles too big

The user watched the actual rendered videos (not just the checker's "0 findings" summary)
and still saw text touching other elements, plus asked for the subtitle/caption size to come
down slightly. Two separate problems, both real.

### Subtitle/caption size

Trimmed back down — they read a touch large next to the rest of the scale after round 1.
Body/label/title/formula unchanged (those were the genuinely-too-small ones):

| | round 1 | round 2 |
|--|--|--|
| SUBTITLE | 23 (3.6%) | **21** (3.3%) |
| CAPTION | 25 (4.0%) | **23** (3.6%) |

### The checker had a real blind spot

`OUTLINE_TYPES` only listed named shape classes (`Line`, `Rectangle`, `Polygon`, `Circle`,
`Arc`, `Arrow`) — it never saw the raw `VMobject` curves that `radiation_waves`,
`convection_stream`, and the promoted `flow_guides`/`animate_flow` are built from. Every wavy
line or particle-stream path in both files was invisible to it, which is exactly the class of
"real" collision (label crossing an animated flow line) the user was seeing. Added `VMobject`
to `OUTLINE_TYPES`, plus a soft "near" pass (`NEAR_PAD`) for tight-but-not-crossing spacing to
verify by eye rather than only hard crossings.

That surfaced a second, self-inflicted bug: `Text` is itself an `SVGMobject`, so
`get_family()` walks into its own glyph paths (`VMobjectFromSVGPath`), which now matched the
generic `VMobject` outline type — every label was reported as colliding with its own letters
(300+ false "findings" on the first run). Fixed by collecting each `Text`'s glyph-path ids
first and excluding them from the outline set. Also added a bounding-box pre-filter before the
expensive point-by-point curve sampling (only run it on pairs whose boxes are already close) —
without it the widened check was too slow to finish a single beat.

### Real collisions found and fixed after the widened check

| Where | Collision | Fix |
|-------|-----------|-----|
| Heating Beat6 | Layer-stack labels (`_layer`) sat directly under columns as narrow as 0.26 units — "Außenputz" alone measured 1.20 units wide, ~4.6× its column, guaranteed to overlap neighbours | Dropped the per-column label entirely; added `_layer_legend` — a swatch-and-name key placed below the stack, sized independently of column width |
| Heating Beat3 | `Öffnung` label sat where the warm-air flow lane bulges through the opening (smooth-path interpolation overshoots past its control points near a sharp turn) | Moved above `wall_upper`, clear of every lane's vertical range |
| Heating Beat3 | `ein Luftpaket` label, centred above the parcel via `next_to`, spawned right at the room's left wall and ran past that edge | Replaced with a fixed callout position in the room's open interior — first tried shifting right, which only moved it into the flow curve instead; a fixed spot clear of both was the reliable fix |
| Cooling Beat7 | The `−`/`+`/`=` pressure signs were centred on each panel at `FORMULA_FONT_SIZE`, i.e. directly on the panel's vertical centreline where the animated flow path also runs | Moved below the room (in the gap before the `Unterdruck`/`Überdruck`/`ausgeglichen` tag row) and down to `BODY_FONT_SIZE` — still clearly legible, no longer the panel's largest element for a one-character glyph |

### Verification

Both files re-run through the fixed, fast checker: **0 hard findings** across all 18 beats
(Heating: 0/9, Cooling: 0/9). Remaining "near" flags (dozens per beat) were sampled across
both files and are overwhelmingly intentional close pairings the checker can't distinguish
from bugs: a legend swatch beside its own label, an arrow tip touching the box it points at,
axis tick labels beside their axis, formula operators inside their own tightly-fit panel
border, and the two lines of one caption sitting close by design. Every fixed beat re-rendered
and spot-checked by eye.
