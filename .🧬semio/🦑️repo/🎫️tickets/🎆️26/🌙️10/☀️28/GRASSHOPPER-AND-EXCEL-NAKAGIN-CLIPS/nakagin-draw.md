# NAKAGIN-DRAW Report

Module: `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py` (mixin `NakaginDrawing`). Contract: `api.md` section 2, `Status: ready`.
Showcase: `nakagin-draw/showcase.py` (stills, en and de), `nakagin-draw/probe.py` (timings); PNGs in `🗑️generated/nakagin-draw/`.

## What exists

- Projection: BIM `View` (axon az/el, plan, four elevations) with a vectorised screen projection and back-face culling; per-capsule painter's algorithm keyed by horizontal depth, tiers podium, core, capital, capsules.
- `capsule_axon`: rounded-profile capsule from the model's `corner_radius` and `corner_segments`, three-plus tones, round window with frame and reflection, bolt points (ghosted when the back is hidden), dimension lines, hatches (`exposed` orange, `attached` green, any face), highlight tint.
- `capsule_views`: plan with hatched walls and window opening, front and side elevation, all dimensions, window offset dimension for type D.
- `plane_axes`: Rhino colours X red, Y green, Z blue, origin dot, faint XY square.
- `core_axon` (live `storeys`, `side`, `storey_height`), `capital_axon`, `podium_axon` (ribbon windows).
- `tower_axon` and `tower_elevation`: 2 podium, 2 cores, 2 capitals, 140 capsules, addressable `capsules[(b,t,s,f,i)]`; `lit` and `tint` by path prefix (a branch `(b,t,s,f)` lights both items) or predicate; `where`, `towers`, `storeys` for step-by-step build-up.

## Performance (measured)

Capsule 4 ms, with hatch 15 ms, with bolts 8 ms, with four dimensions 40 ms (labels cached per string; a new string costs about 40 ms once). Full tower 0.2 s, elevation 0.13 s.

## Design notes and traps

- `VGroup` is falsy when empty: `drawing or X` created a throw-away group once; use `is None`.
- Capsule local frame follows the model (x centred, back y=0, front y=length); dimension sign comes from "away from the drawing centre", not from fixed offsets.
- Unlit capsules are darkened (not transparent) so overlaps stay clean.
- German and English verified by stills (decimal comma, titles); no strings are authored in the module besides symbols (X, Y, Z, Ø, m).
- Blue for Z is a blend of the palette cyan and blue because the semio palette has no blue.
- No BIM files modified.
