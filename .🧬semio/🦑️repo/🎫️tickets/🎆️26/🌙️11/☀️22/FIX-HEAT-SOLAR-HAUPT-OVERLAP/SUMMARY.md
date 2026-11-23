# Fix Heat Solar Haupt Overlap

## Problem
`Beat8_Hauptgleichung` left free-factor tokens (`I_s`, `A`, `g_⊥`, `F_F`, `F_W`, `F_S`, `F_C`) painted over `Q_Speicher`, the mass legend, and the caption. `FadeOut` on nested `math_text`/`Text` does not clear Cairo glyph fill on leaves.

## Fix
1. Added `_wipe(scene, *mobs)` — zeros fill/stroke/opacity on every `family_members_with_points()` leaf, then `scene.remove`.
2. Assemble: `FadeOut(intro)` then `_wipe(tokens, labs, unit_head)` **before** `FadeIn(row)` (no parallel morph).
3. Store: `_wipe` again; clear progress bar updaters; compact solar pack (`scale(0.86)`); side legend with `fit_band(..., bottom=SAFE_BOTTOM + 0.55)`.
4. Tests: `HauptOverlapTest` requires `_wipe` / forbids token morph; runtime wipe opacity matches Manim `Text` leaf zeroing.

## Verify
- `big_ghost_blobs == 0` on assemble and mass frames (formula-sized saturated glyphs in free-token band).
- Side-by-side old vs fixed: ghosts gone; legend readable.
- Preview clip: `videos/preview/de/speaker/s/61/v/Beat8_Hauptgleichung.mp4`
- Full heating master not re-concatenated (disk ~1.3 Gi free).

## Files
- `🎓️teaching/…/☀️solar-gains/🐍️.py`
- `🎓️teaching/…/☀️solar-gains/🧪️tests/🐍️.py`
- `🎓️teaching/…/☀️solar-gains/🧪️tests/🔣️.cases.json`
