# Brief — restyle one Energy Balance chapter with the Physical Fundamentals vocabulary

Repo: /Users/niloufarghandehariyoon/Developer/semio (bun/nx monorepo; this work is pure Manim/Python under `tutorial/`).
You own exactly ONE chapter file: `tutorial/energy/demand/EnergyBalance/<N_chapter>/scene_<N>.py`. Other agents edit the other chapters concurrently — do NOT touch their files. You MAY NOT edit `tutorial/manim_visuals.py` or `manim_fonts.py` (shared, owned by the lead). If you need a new shared helper, build it locally in your scene file under a `#region Shared visual motifs` and tell the lead in your report so it can be promoted.

## Goal (from the dev)
Revise the Energy Balance video so objects and animated effects use the Physical Fundamentals vocabulary instead of the ad-hoc drawings in the chapter: the house shape, human, lamp, sun, sun radiation and animated energy flow. ONLY the object visualisation and animation change — the explained content (NARRATION clauses, numbers, formulas' meaning, standards) stays. Everything must be animated well. The result must be as good as the Cooling restyle (`tutorial/energy/demand/Cooling/`, ticket notes in `.repo/🎫/26/10/09/RESTYLE-COOLING-WITH-FUNDAMENTALS-GLYPHS/notes.md`) — study `Cooling/6_lueftungssysteme/scene_6.py` and `Cooling/2_internal_gains/scene_2.py` as the reference for style and structure.

## Vocabulary (all in `tutorial/manim_visuals.py`, import from there)
house_section(center, scale), room_section, window_glyph/open_window, radiator, sun_glyph, moon_glyph, sun_rays/shine/beam_pulses/pulse_flashes (straight parallel rays, never wavy), person_glyph, seated_person_glyph, lamp_glyph, thermometer_glyph, clock_glyph, energy_tank, cloud_glyph, droplets, ripples (long-wave heat radiating from warm sources), flow_animation / paced_flow / animate_flow(s) (particle streams along smooth_path guides, flow_guides for the faint rails), meter/bind_meter (0–1 factor gauges), stacked_bar, labeled_axes, stat_card/card_grid/note_line/side_labels, chip, cross_mark, dim_arrow, color_ramp.
Typesetting: use `math_label`, `math_panel`, `math_readout`, `math_text`, `de_num` for every formula, unit, subscript, superscript and fraction (real fractions/roots, upright units via `\mathrm{}`), NOT `Text("Q_h")`-style fake subscripts and NOT `equation_row` with plain strings. Look at how the Cooling scenes call `math_panel([...])`.
`hold_for(self, NARRATION, key, used=..., during=fn)` keeps ripples/particles/rays moving while the subtitle is read — use `during=` so nothing freezes. `swap_caption` / `caption_bar` / `subtitle_text` stay as they are.

## The dev's rules (verbatim, German — apply them)
- Alles muss visuell sein (erklären, visualisieren, morphen etc meinen alle das gleiche). Ideen müssen ineinander morphen (Transform/ReplacementTransform between related visuals, e.g. a flow arrow becoming a bar segment becoming a formula term).
- Alle Faktoren sollen Animation bekommen von 0–1, viele fehlen noch (every factor such as η_h, e_p, f_p, g, F_F... gets a visible 0→1 animation — meter/ValueTracker/fill — not just a printed number).
- Alle Teilwerte der Gleichungen müssen Resultat von der physischen Repräsentation sein (Flächen, Faktoren, etc): every number/term in an equation must visibly come from the drawn physical picture (area of a drawn region, length of a bar, a gauge reading via `math_readout`), not appear from nowhere.
- Noch viel unwichtige und unverknüpfte Informationen anstelle von zusammenhängend und spannend: drop decorative/unrelated visuals; keep what connects to the through-line (see `EnergyBalance/README.md`). Do NOT change what is explained, but you may remove on-screen clutter that is not part of the NARRATION.
- Text übertrieben und unnötig wuchtig: keep on-screen labels short and sober (no dramatic taglines like "Architektur formt Raum"). Do not alter NARRATION strings except for removing such taglines if present.
- Architektur und Bauphysik sind nicht getrennt, sondern eine Einheit: the building drawing and the physics (flows, factors) live in the same picture.
- LaTeX muss richtig dargestellt werden (Formeln, Einheiten, Brüche, Sub-/Superscripts) — see typesetting above.

## Lessons from the Cooling review (the dev already complained about these — don't repeat)
- A person must be at true scale (≈70 % of the inner room height; 1.75 m in a 2.5 m room) and stand ON the floor slab's top edge, not float in the air or sink in the slab. Cooling has the helper `_stand(room_c, room_h, x)` in `Cooling/6_lueftungssysteme/scene_6.py` — copy that logic locally.
- Exterior shading slats tilt down toward the outside (outer end lower), on every window.
- No stray arrows/gimmick morph boxes that don't mean anything (e.g. a blue box morphing into a symbol). A morph must carry meaning.
- Heat/air flow must not run through people's bodies or labels; labels must not overlap anything; nothing sits on top of the subtitle bar or formula panel.

## Method (non-negotiable)
1. Read your chapter file fully and `EnergyBalance/README.md`. Keep every Beat class name, `NARRATION` list (keys and strings), `TITLE_DE`, and the `hold_for` clause structure so `vo_timing.json` stays valid; the full-video composer (`EnergyBalance/full_energy_balance_video.py`) imports your Beat classes by name.
2. Rewrite visuals. Use `#region`/`#endregion` (`//` style is for TS; in Python use `#region Name` / `#endregion`) to structure; no comments inside definitions; docstrings start with a unique fitting emoji; no new files except what's listed below; no legacy shims.
3. Verify EVERY beat: render with `LAYOUT_CHECK=1 .venv/bin/manim -ql -s --media_dir <your scratch dir> <scene file> <BeatClass>` (last frame) until the layout guard prints nothing for text overlaps. Also render each beat's intermediate states (at least one frame per NARRATION clause — e.g. use `-ql` full movie and extract frames with ffmpeg `-vf fps=1` or `select` into a contact sheet via PIL) and LOOK at them with the Read tool (images). Fix overlaps, off-screen items, wrong scale, flow through bodies, and anything not synced with its subtitle clause. "Done" means you looked at dense frames, not that it compiled. (Memory rule: scene work is only done after dense screenshot review + LAYOUT_CHECK=1 shows no overlaps.)
4. CPU is shared: render at `-ql` only (480p15), one manim process at a time. Do NOT run 1080p; the lead renders the final video. Do not run `git` commands that modify state (no commit/stash/checkout) — other people are working in the same files.
5. Scratch/temp files and contact sheets go in `/Users/niloufarghandehariyoon/Developer/semio/.repo/🎫/26/10/13/RESTYLE-ENERGYBALANCE-WITH-FUNDAMENTALS-GLYPHS/checks/` (sheets in `checks/sheets/ch<N>_*.png`; keep `[DEBUG] `-prefixed logs only temporarily). Do not create files outside that folder, except editing your scene file. Use your own `--media_dir` under the checks folder so you don't collide with others.
6. Do not claim something works unless you ran it. Report faithfully.

## Report back (concise)
Per beat: what the new visuals are, how each factor animates 0→1, which equation terms are tied to which physical element, what you verified (commands + which frames you inspected), anything unresolved, and any local helper that deserves promotion to manim_visuals.py.
