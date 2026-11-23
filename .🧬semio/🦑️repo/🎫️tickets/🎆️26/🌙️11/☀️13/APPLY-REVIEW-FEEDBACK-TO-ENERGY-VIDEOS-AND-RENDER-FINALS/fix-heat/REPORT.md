# fix-heat: polish pass, German heating video (speaker and spectator 1080p30)

Method: 4 s sweep of the old speaker final (468 frames, 52 contact sheets plus 24 larger sheets, `fix-heat/sheets.py`, `sheets2.py`, `frame.sh`), held frames at 1280x720 for every lead; numbers recomputed (2311 W table, H_T 115,6, psi values, 768 W to 232 W, 16000 - 4500 = 11500, Q_E 14100, U/Phi of the three walls). After the fixes the complete final was re-rendered (`render.sh`, `--profile final --jobs 3`) and the fixed spots were checked on held 1280x720 frames of the new speaker final (`🗑️generated/fix-heat/new`).

## Root cause of most defects

A morph source built as a fresh `VGroup` of children that are already top-level in the scene is never removed (Manim `Scene.remove` does not expand families), so the old pieces stayed behind the new ones.

## Defects (all REAL, verified on held frames)

| Time (old) | Defect | Fix | File |
|---|---|---|---|
| 10:30-11:00 | theta_1/theta_2 readouts left over after the morph to "Gleichgewicht", then "Daemmschicht" label ran into them | `self.remove(left_read, right_read)` after the morph | conduction |
| 16:30-17:00 | H_T formula drawn twice (blurred) at the flat-rate stage | `self.remove(*p_det[0].submobjects)` after the morph | conduction |
| 24:50 | factor-token row (I_s A g F...) left behind the Q_Speicher formula | `self.remove(*tokens, ...)` | solar-gains |
| 26:56-27:08 | Q_l formula overlapped by ghosts of Q_l, H_T, H_V, arrow | remove all children of `initial_eq` and `multiplier_group` | calculation |
| 27:12-27:52 | grey subtitle "Freie Waermegewinne (DIN V 4108-6:2003-06)" clipped at the left frame edge, held 40 s | `aligned_edge=LEFT` | calculation |
| 19:00 and 20:16 | two-line captions covered the formula box (Q_i,p row, q_i row) | captions shortened to one line, en and de | internal-gains |
| 5:40 | reference line ran into the title chip; R_si/R_se box text was the cryptic wording of feedback F21 | chip "Drei Wege, ein Waermestrom"; box "R_si innen, R_se aussen: Konvektion und Strahlung in einem Wert"; caption reworded | fundamentals |

Folders are `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/<folder>/🐍️.py`.

## Transient (no change)

- psi formula box fading in at 15:58; chapter-boundary UI overlays 20:24-20:30; count-ups (2241 W, 1083 W, Q_l 400 kWh/a) while running, held values agree; sparse stage at 11:48-12:00 (lone R_T) is by design.

## Checked and fine

- Second law wording, light/EM wave link, R_si/R_se box, thermometers 39/1 C and 29/11 C visibly different, "dies ist die Temperaturdifferenz", F_C, degree days with theta_i reference line, roadmap first (with Phi, Delta-theta, F_C). Captions at most two lines everywhere.
- Not separately swept: the thermal-bridge "U = 0,24 W/m2K" line and the solar-scene g placement were not looked at in the new final.

## Verification

- `python -m unittest ...🧱️conduction.🧪️tests.🐍️` (SEMIO_TEST_MANIM=1): 2 tests OK. It is the only heating test module; the pinned caption fixture does not contain the changed strings.
- New speaker final (01:50): chip and ref clear, equilibrium scene clean, Daemmschicht label clear, H_T formula single, Q_Speicher row clean, Q_l formula clean, subtitle left-aligned, q_i caption one line above its box.

## Leftovers (outside heating, not edited)

1. Physics thermal-mass beat reused in this video (about 23:58, `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`): "10 m2 . 0,133 kWh = 1,4 kWh" lacks "/m2" on 0,133 kWh.
2. Speech was not re-synthesised (silent track); shortened captions only change subtitle text.
