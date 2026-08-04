# ☀️ Create Solar Radiation Cooling Video

## Beat plan

| Beat | Class | German subtitle | Teaches |
|------|-------|-----------------|---------|
| 1 | `Beat1_SolarIrradiance` | Direkte Sonnenstrahlung | `I_S,max`, orientation curves (Horiz/N/S/O/W) |
| 2 | `Beat2_FrameFactor` | Fensterfläche und Rahmenfaktor | `A_eff = A · F_F` |
| 3 | `Beat3_ShadingFactor` | Verschattungsfaktor | `I_reduziert = I_S,max · F_V` (facade section, unshaded → Raffstore) |
| 4 | `Beat4_GlassTransmittance` | Gesamtenergiedurchlassgrad | `g_tot = τ_e + q_i` |
| 5 | `Beat5_SolarCoolingLoad` | Solare Kühllast | `Q̇_S,tr = A · F_F · F_V · g_tot · I_S,max` |

Persistent chapter header `Kühllast mit Sonnenschutz` (Pastel Cyan) on every beat.

## Rendering technique notes

- **Occlusion by `z_index`, not by draw order.** Beats 2 and 3 teach "the frame / the blind
  physically blocks light". Rays are built at `z_index=1`, panes at `2`, masonry and frame at
  `3`, the Raffstore at `4`, and outgoing rays at `5`. `_build_window` therefore uses
  `fill_opacity=1.0` on the frame bands — a translucent frame would let blocked rays show
  through and invert the lesson.
- **Beat 3 is a vertical section, not a face-on view** (dev request). Face-on cannot show why
  a louvre blocks: that argument is entirely about the angle between the beam and the slat.
  The section carries it in two situations — unshaded (full beam through the glass, `F_V` = 1,0)
  then shaded (Raffstore drops in, beam reflects back outside, `F_V` = 0,15) — with the `F_V`
  marker sliding between them. The geometry is derived, not eyeballed: with a beam direction
  of `d = (0.75, -0.661)` the slats sit at 48°, which puts `d·n` at −0.999, i.e. all but
  perpendicular, so `r = d − 2(d·n)n` reflects back up-left toward the sun.
  Slat tilt sign matters: slats rise to the *inside* so their outer edge is lower. Mirror that
  and the beam reflects into the room, teaching the opposite of the truth.
- **The residual path starts at the slats, not at the glass.** Dimming the unshaded interior
  rays leaves a gap in the story — light appearing at the glass from nowhere. `residual`
  instead originates at each slat's inner tip and runs along `d` through the glass into the
  room, so `Restanteil` is one continuous path.
- **Reflection mirrors x.** Beat 4's pane is vertical, so the reflected direction is
  `(-dx, dy)`. An up-left reflection (the intuitive but wrong choice) lands almost on top of
  the incoming ray and reads as a single line.
- **No `MathTex`.** `_equation_row` composes equations from separate `Text` fragments keyed
  by name, so symbols stay addressable for highlighting without fragile glyph-index slicing.
- **No `Axes.add_numbers`.** CE renders those through LaTeX; Beat 1 places axis labels
  manually with `Text` at `axes.c2p(...)`.

## Verification

- All five beats smoke-rendered with `-ql` (854×480), frames inspected for overlap.
- `build_full_video.py` ran end-to-end: 5× `-qh` render → mux → compose →
  `Full_Scene4_Solar_Radiation.mp4` (~70 s, exit 0).

## Blocker: TTS

`generate_audio.py` fails for all five beats:

```
TTS failed (401) model='gpt-4o-mini-tts' voice='alloy' format='mp3':
Incorrect API key provided: sk-proj-…lmMA
```

The key lives in NowIGetIt's `.env`, outside this repo. The five English narrations are
finished and staged in `NARRATIONS`; re-run `📦build📽️solarradiation🔊audio` then
`📦build📽️solarradiation🎬video` once the key is valid. Same blocker as
`2026/08/02/CREATE-INTERNAL-GAINS-COOLING-VIDEO` (403 at the time).

## Side fixes

- `.vscode/launch.json` — `🛠️dev🎬manim scene_2` pointed at `scene_2.py Beat1_OfficeRoom`,
  which no longer exists; the file was renamed to `merged_scenes.py` with `SceneN` classes.
  Repointed to `merged_scenes.py Scene1`.
- `.gitignore` — Manim leaves `partial_movie_file_list.txt` under `partial_movie_files/`,
  the only render byproduct the `tutorial/**` media rules did not cover.
