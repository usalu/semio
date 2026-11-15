# heat-b report (heating: solar gains, internal gains, calculation), recorded by the coordinator

Verified with direct manim renders (German preview profile) + ffmpeg frames; the pipeline
render was not run. Not re-rendered after the last tweaks: F_S colour, Scene2 right-hand
column positions, Beat3 leader line.

- F26: notation checked against DIN V 4108-6:2003-06 (din.de table of contents; formulas as
  restated in IWU Loga 2004). Balance Q_h = Q_l − η·Q_g, Q_l = Q_T + Q_V, Q_g = Q_S + Q_i opens
  the solar chapter. Loads Q̇_S, Q̇_i, Q̇_i,p/e/l, Q̇_l, Q̇_T, Q̇_V (= n·V·ρc_p·Δθ). Solar:
  I_s·A·g⊥·F_F·F_W·F_S·F_C. F_Gt added. Clauses cited: 6.3 internal gains, 6.4 solar gains,
  6.5 η, Annex B.4 F_C, Annex D.2 heating-period method. Units `[Q̇] = W` / fraction bars; units on
  every number; narration `$…$`.
- F27: g-value scene simplified to sun ray, glazing, g⊥ = 0,60 with leader at the glazing; master
  equation keeps the DIN chain; second line 768 W at the opening → about 232 W in the room.
- F28: blind F_C = 0,15, roof overhang F_S = 0,8, F_W = 0,9 added (258 W → 232 W).
- F29: year of daily θ_e, indoor line θ_i = 20 °C, dashed heating limit 15 °C, shaded area =
  G_t20/15 ≈ 3 500 Kd/a (212 heating days, stylised cold climate); F_Gt = 24 h/d · G_t = 84 kKh/a;
  Q_l = (H_T + H_V)·F_Gt ≈ 16 000 kWh/a. Running example owns the limit and the profile.

## Answer to the reviewer: "Inkonsistenz der DIN?"
No inconsistency inside the DIN. `F_C` is the German notation (DIN V 4108-6 Annex B.4,
DIN 4108-2), sun-protection factor = g_tot/g. `F_sh` is ISO notation (ISO 13790,
EN ISO 52016-1: F_sh,ob obstacles, F_sh,gl movable shading), roughly F_S and F_C.
Confirmed only: EN ISO 52016-1 Annex F (EPB Center case study), F_C = g_tot/g (Warema).
The ISO 13790 equation and the exact correspondence were NOT checked against the standard text.

## Not verified
G20/15 definition from secondary sources only (VDI 3807 Blatt 1 not read).

## Checks
Running-example tests pass (+2 new). Narration guard scan clean for the three files; the
guard-mechanics test fails (`file.py:41 [d en]`), owner: speech agent. `staffage_only` exit 0.
`visuals…tests.🐍️scenes::test_heating_closeup_retains_real_dimensions` fails (staffage heights,
not touched). Quiz and README not edited (no quiz questions on these chapters).

## Files (`🎓️teaching/🏛️architecture/⚡️energy/`)
`🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`, `👥️internal-gains/🐍️.py`, `🧮️calculation/🐍️.py`,
`🏗️authoring/📏️running-example/{🔣️.json,🐍️.py,🧪️tests/🐍️.py}`, `🏗️authoring/🎨️visuals/🐍️.py`
(`\perp`, `\sum`), `📖️concepts/📝️.md`. Frames: `🗑️generated/heat-b-out/direct/<Scene>/`.
