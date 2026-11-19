# Beat5 Radiation Layout Fixes

Scene: `Beat5_Strahlung` in physics fundamentals.

## Changes

1. **Formula panel lower** — `_panel(..., edge_buff=1.2)` (default 1.7) so the red Q̇s box sits farther from the room slab.
2. **Longer reflections** — `REFLECT_LEN = 1.45` for glass bounce, exterior Raffstore bounce, and interior bounce (was ~0.4–0.55).
3. **Reflection animation** — exterior: morph incoming rays, then `Create` bounced rays, then pulse along `[start, stop, back]`. Interior: same Create for bounce after incoming.
4. **Visible Raffstore lamellae** — `_blind` lightens thermal fill (`opacity=0.4`) and overlays teal stroke lines for every lamella face.
5. **Full-window sun rays** — entry y-span uses ±0.02 inset (was ±0.12); blinds span `win_hi+0.04` … `win_lo-0.02` for outside and inside.

## Verify

Re-run Manim Sideview on `Beat5_Strahlung` (shim writes under `.sideview/`).

## Caption / formula overlap (follow-up)

`edge_buff=1.2` pushed the formula into the caption band. Restored the default formula panel buff (`1.7`) and lifted the Beat5 room (`center y 0.25→0.55`, `h 2.6→2.4`) so the slab clears the formula without invading the subtitle.

## Raffstore glyph + steady shade rays (follow-up)

Staffage cast was window-clipped to ~0.2×0.3 and only short teal face strokes remained (looked like a dashed vertical line).

- `_blind` is now a didactic side-elevation glyph: teal head/foot/rails + orange tilted lamella polygons spanning the full window.
- Summer/inner shade radiation no longer uses staggered `_beam_pulses` (which flickered). Beams stay drawn; one simultaneous `ShowPassingFlash` runs over the steady paths.
Reflections on the Raffstore now run back toward the sun (`stop - sun_dir * REFLECT_LEN`) so rejection reads clearly.

## Precise slat hits (follow-up)

Shade rays no longer reuse glass-aimed `starts`. `_shade_beams` aims **one ray per lamella** at the sun-facing face center, then reflects with the lamella normal — used for both exterior and interior Raffstore.

Formula panel uses `edge_buff=2.05` so tall summer captions stay clear of the equation box.

## Label color

Compare tag `besser` / `better` uses `P_BLUE`.

## Raffstore head + true rejection (follow-up)

- Teal frame is **only the head box** on top of the orange lamellae (no side/foot rectangle).
- Through-room beams, shaft, patch and Low-E remnant are faded before shade rays.
- Each shade ray stops on the sunward face of a lamella and **bounces back toward the sun** (`-sun_dir`).
- Formula panel fades during the Beat5 summer caption so it no longer covers the subtitle.


## Specular outdoor bounce (follow-up)

`-sun_dir` made reflected beams **collinear** with the incoming rays, so the reflection looked missing/false.

`_shade_beams` now:
1. Specular-bounces with `_mirror(sun_dir, lamella angle)`.
2. Forces the bounce **up and left** (outdoors) so it is visually distinct from the incident ray.
3. Keeps hit points on the sunward third of each lamella.
