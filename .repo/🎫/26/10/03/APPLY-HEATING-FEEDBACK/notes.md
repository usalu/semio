# Apply Heating Review Feedback

Goal: `R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS`

Repo MCP was unavailable (`CONNECTION_CLOSED`), so `repo://goals`, `ticket_open` and `ticket_close` could not be called. Ticket created and logged here by hand, same as `APPLY-PHYSICAL-FUNDAMENTALS-FEEDBACK`.

Dev instruction: apply only the supervisor's Heating feedback, keep everything else unchanged. Work spec with the verbatim feedback, the rules and the shared numbers: [spec.md](spec.md).

## Shared changes

- `tutorial/manim_visuals.py` — additive `#region Typeset readouts`: `de_num`, `place_math`, `math_label`, `math_readout`, `math_panel` (the PF scene's private `_de/_m/_readout/_panel` as shared helpers); `\Phi \Psi \psi \chi \ldots` in the math symbol table; `watt_anchor` ratio uses a decimal comma.
- `checks/math_probe.py` — typesetting probe for the Heating notations.
- `tutorial/energy/demand/Heating/full_heating_video.py` — playlist gains `Beat4_WattVergleich`, `Beat5_Gebaeudehuelle`, `Beat6_Waermebruecken` (Modul 2), `Beat7_KippOderStoss`, `Beat8_Lueftungssysteme` (Modul 3), `AnlagenVerluste` (final).
- `tutorial/energy/demand/KONZEPTE_UND_FORMELN.md` — Heating sections: U per m², H = U·A, Ψ / ΔU_WB, watt table, V_e vs V, n scale, Stoß vs Kipp, F_sh, annual chain, η_h as area ratio, Anlagenverluste.

## Feedback checklist

Regeln / Allgemein
- [x] Visuell + morphen: readouts fly into formula panels, tiles → A, wall → toaster/person/tealights, green area → η_h, examples → n scale, paper line → Ψ
- [x] Faktoren 0→1 animiert: every value counts from 0 (`math_readout` / local `_live`); F_f 1→0,70, g 0→0,60, F_sh 1→0, η_WRG 0→1→0,8, η_h 0→0,90
- [x] Teilwerte aus physischer Repräsentation: A from dimension arrows, V from inset section polygons, F_f from glass area, g from counted ray packets, F_sh from sunlit glass height, η_h from chart areas, G_t from Δθ day area, losses from pipe length / store / radiator
- [x] Unverknüpftes / wuchtiger Text: "Sieb", "Geschichte", "journeys", "Jetzt drehen wir um", ALL CAPS, "entweicht!", "(Sehr Hoch)" removed; thermometer replaced by the utilisation chart
- [x] Architektur + Bauphysik: bricks instead of HEISS/KALT blocks, one example house (10 × 8 m, 6/8,5 m) through Modul 2, 3, 5 and the final balance
- [x] LaTeX: all formulas, symbols and units typeset (sub/sup, fractions, dot accents, Φ/Ψ/η); subtitles and captions free of `_`
Heizen
- [x] Transmission animiert — Modul 2 `Beat5_Gebaeudehuelle` particle flows per element, Φ_i bars stack to 2 311 W; final `ReviewingHeatLosses` wall/window flows
- [x] U je 1 m², Dicke steckt in U — Modul 2 `Beat3_UWertUndGradient` 1 m² tile with d → R → U, tiles copy to 25 m², H = U·A
- [x] Wattvergleich — Modul 2 `Beat4_WattVergleich`: 715 / 120 / 55 W → 0,7 Toaster / 1,2 Mensch / 1,8 Teelicht (link to PF power scale)
- [x] Wärmebrücken — Modul 2 `Beat6_Waermebruecken`: paper line außen/Mitte/innen, corner 2× vs gap, Ψ_e −0,08 / Ψ_i +0,11, Ψ_i − Ψ_e = U·2d, Balkonplatte 4 W/K, ΔU_WB 0,10/0,05
- [x] Netto ≠ Brutto — Modul 3 `Beat2_Innenvolumen`: V_e 580 m³ from outer outline, construction removed, V ≈ 440 m³ ≈ 0,76 · V_e
- [x] Luftwechsel animiert + Skala — Modul 3 `Beat3_Luftwechselrate`: fresh air replaces room air at rate n, examples Lager ≈ 0 … OP-Saal ≈ 20 morph onto a log scale
- [x] Kipplüftung — Modul 3 `Beat7_KippOderStoss`: 5 min Stoß 100 % / 340 Wh vs Kipp 8 % after 5 min, 60 min per exchange, + radiator 250 Wh + Laibung 190 Wh ≈ 780 Wh
- [x] Wintersonne — Modul 5 `Beat4_SaisonaleWinkel`: parallel rays, `_sun_patch`, patch on floor/lower back wall, upper back corner unlit
- [x] Verschattungsfaktor 1→0 — Modul 5 `Beat5_Verschattung`: Raffstore lowers F_sh 1,00 → 0,00; overhang winter 0,80 / summer 0,05 from geometry
- [x] Ausnutzungsgrad — final `Scene4`: day chart, green used / red surplus, η_h = A_nutz/A_Gewinn = 0,90, heavy mass 0,97, green area morphs into η_h
- [x] Übergangsverluste — final `AnlagenVerluste`: pipes in cold cellar, store, radiator at outer wall, flue; Q_h 11 500 → Q_E ≈ 13 940 kWh/a (DIN V 18599-5)

## Numbers

Shared numbers in [spec.md](spec.md). Deviations: gedämmt/hochgedämmt wall 120 W / 55 W (from the rounded U shown on screen); summer F_sh 0,05 instead of 0,2 (a single overhang cannot give winter 0,80 and summer 0,2 at 20°/60°).

## Verification

- Every beat of Modul 1–5 and every final scene rendered at `-ql` with `LAYOUT_CHECK=1` (each into its own media dir — parallel renders into one dir corrupt the text cache): 0 `[LAYOUT]` lines, 0 exceptions. Logs in `checks/`.
- Contact sheets reviewed for all modules (`/tmp/heat_m1 … /tmp/heat_final/sheets`).
- Caption check: every German caption ≤ 2 lines, no underscores (`checks/*caption_check*`).
- `full_heating_video.py` imports and lists 42 beats across 6 sections.
- Not done: the 1080p60 full-series render (`🛠️dev🎬manim heating full`) and VO re-synthesis — module `generate_audio.py` lists follow the new beat order.

## Follow-ups

- Modul 1 and 4 carry a local cached `_live` readout next to the shared `math_readout`; candidate for one shared cached helper.
- PF `scene_1.py` still has its private `_de/_m/_readout/_panel` copies of the new shared helpers.

## Files

Updated
- `tutorial/manim_visuals.py`
- `tutorial/energy/demand/Heating/1_introduction/scene_1.py`
- `tutorial/energy/demand/Heating/2_conduction/scene_2.py`, `generate_audio.py`, `build_full_video.py`
- `tutorial/energy/demand/Heating/3_convection/scene_3.py`, `generate_audio.py`, `build_full_video.py`
- `tutorial/energy/demand/Heating/4_internal_heat_gain/scene_4.py`
- `tutorial/energy/demand/Heating/5_solar_heat_gain/scene_5.py`
- `tutorial/energy/demand/Heating/final_calculation/merged_scenes.py`
- `tutorial/energy/demand/Heating/full_heating_video.py`
- `tutorial/energy/demand/KONZEPTE_UND_FORMELN.md`

Created (ticket)
- `spec.md`, `notes.md`, `ticket.json`, `checks/*` (render scripts, caption checks, logs, before-copies)

## Reopened 2026-10-03 — Δθ alignment

- Modul 1 `Beat1_DreiWegeDerWaerme`: the live Δθ readout was anchored on the static label's left baseline marker, so `edge="center"` pushed it right of the wall. Now anchored on the wall centre and the baseline of „innen/außen“; `_section` places the static Δθ the same way. Re-rendered at `-ql` with `LAYOUT_CHECK=1`: 0 issues, Δθ centred under the wall.

## Reopened 2026-10-03 — realistic Leitung / Konvektion / Strahlung (Modul 1 Beats 1–5)

Dev request: the wall graphic and the animations of the three heat paths were not realistic or understandable — regenerate them.

- New shared section `_stage()`: heated room (radiator, standing occupant, floor/ceiling slabs) | plastered brick wall with mortar joints | winter night (snow, moon, stars); labels „innen · Δθ · außen“ on one baseline, Δθ centred under the wall. `_tint_wall` colours the masonry warm → cold.
- Beat1: heat particles cross room → wall → night while the wall takes its gradient; Δθ counts 20 → 18 K.
- Beat2 Leitung: dashed marker morphs into a magnifier; bonded brick particles vibrate, the vibration front runs warm → cold (colour and amplitude), free air particles on both sides; one particle stays in its ring.
- Beat3 Konvektion: real room air loop — rises at the radiator, ceiling run, cools and sinks at the cold wall, returns on the floor (colour by path position); wind sweeps the facade outside; R_si / R_se at the hand-over arrows. Old mid-height „Außenluft sickert ein“ removed (that is infiltration, not surface convection).
- Beat4 Strahlung: IR rays radiator → wall and outer wall → night sky with travelling pulses; thought experiment removes the room air, convection loop is crossed out, radiation continues.
- Beat5: the three ways in series (`Q̇_c,i + Q̇_r,i = Q̇_k = Q̇_c,e + Q̇_r,e`) instead of the physically wrong sum `Q̇_k + Q̇_c + Q̇_r`; stepped temperature profile 20 → 16,3 → 1,1 → 0 °C from U = 1,43, R_si = 0,13, R_se = 0,04.
- Narration of Beats 1–5 adapted to the new pictures (VO must be regenerated for Modul 1).
- Verification: `checks/m1_render_realistic.sh` — all five beats at `-ql` with `LAYOUT_CHECK=1`: 0 layout issues, 0 tracebacks (`checks/m1r_*.log`); contact sheets reviewed; all captions ≤ 2 lines. Pre-change copy: `checks/m1_scene_1_before_realistic.py.txt`; building blocks: `checks/m1_realistic_block.py.txt`, `checks/m1_realistic_beats.py.txt`.
- `KONZEPTE_UND_FORMELN.md` Modul 1: convection/radiation definitions and the series formula corrected.

## Reopened 2026-10-03 — neutral bricks, friendlier occupant

- Bricks drawn without material colour (`BRICK_FILL` neutral dark, light mortar joints); lens wall band neutral too. Temperature tint kept as physics overlay but lighter (35 %, 22 % where it only hints).
- `_occupant()` redrawn: round head with hair, eyes, cheeks and smile, sweater torso, arms with hands, trousers, shoes.
- Re-rendered Beats 1–5 (`checks/m1_render_realistic.sh`): 0 layout issues, 0 tracebacks. Still probe: `checks/m1_stage_probe.py`.

## Reopened 2026-10-03 — minimal occupant

- `_occupant()` now uses the same minimal head-and-body outline glyph as the other tutorial scenes (PF, Cooling, EnergyBalance), scaled ×2.6 to stand on the floor; the detailed figure (face, hair, sweater, hands) is gone. Beat4 air particles keep a wider gap around it.
- Re-rendered Beats 1–5: 0 layout issues, 0 tracebacks.

## Reopened 2026-10-03 — DIN chip collision in Beat5

- The „DIN EN ISO 6946“ chip sat on the wall base and against the formula panel. It is now the standard top-right `_din_ref` corner citation, faded in and pulsed on the „standard“ clause (FadeIn and Indicate in separate plays — Indicate restores its start state). Re-rendered Beat5: 0 layout issues.

## Reopened 2026-10-04 — straight radiation rays, simpler Modul 2 Beat3

- Radiation is drawn as straight lines everywhere in Heating: new shared `radiation_ray` (straight `Line`, round caps) in `manim_visuals.py`; Modul 1 `_ir_rays` and every `solar_wave_ray` in Modul 5 (sun rays, g-value packets, Beat1/2) switched. Re-rendered Modul 1 Beat4–5 and Modul 5 Beat1–5: 0 layout issues, 0 tracebacks.
- Modul 2 `Beat3_UWertUndGradient` decluttered on dev request: only the symbols U and A on screen — no stacked numeric equations (U = 1/4,13 …, Q̇ = 0,24 · 1 m² · …, d → R → U chain, H readouts). Tile shows U and d (d flies into U), the wall is 25 tiles marked U with a framed A; U and A fly into `H = U · A`, Δθ from the profile flies into `Q̇ = U · A · Δθ`. Narration numbers removed accordingly. Re-rendered: 0 layout issues. Before-copy: `checks/m2_scene_2_before_beat3_simplify.py.txt`.

## Reopened 2026-10-04 — calm transmission arrows in Modul 2 Beat5

- `Beat5_Gebaeudehuelle`: the scattered particle streams around the house were too busy. Replaced by one outward arrow per element (roof slopes, wall, windows, door, floor), stroke width ∝ Φ_i; each element's arrows grow while its table row counts, and one light pulse runs along all arrows while the bars stack into Φ_T = 2 311 W. Re-rendered: 0 layout issues. Before-copy: `checks/m2_scene_2_before_beat5_arrows.py.txt`.

## Reopened 2026-10-04 — 3D net volume in Modul 3 Beat2

- `Beat2_Innenvolumen`: ground line under the house removed (confused the 3D box). Net air volume V drawn as two orange prisms (front face one wall thickness behind the facade, back face one wall thickness before the rear, connecting side faces and edges) that fill from the floor; the gross box keeps its 8 m depth — the inner depth d_i counts on its own tracker. Re-rendered: 0 layout issues. Before-copy: `checks/m3_scene_3_before_beat2_prism.py.txt`.

## Reopened 2026-10-04 — Modul 3 Beat3 spacing and pacing

- n scale raised (`_n_scale(0.6, -0.62, 2.0, zero_drop=0.4)`, label floor −1.15) so its „≈ 0“ end no longer crowds the formula panel.
- „60 min / 20 = 3 min“ now waits 1 s after the operating room lands on the scale, appears over 1,5 s and is pulsed once. Re-rendered: 0 layout issues.

## Reopened 2026-10-04 — Δθ alignment in Modul 3 Beat5

- `Beat5_Lueftungsverlust`: the live „Δθ = θ_i − θ_e = 20 K“ readout was right-aligned to the brace while „Temperaturdifferenz“ sat at another x; both are now centred on x = −1,5. Re-rendered: 0 layout issues.

## Reopened 2026-10-04 — room corners in Modul 3 Beat7

- `_vent_room`: floor and ceiling slabs started 0,06 right of the back wall's outer face, so the wall stuck out; slabs now start flush with it and the back wall spans the slab thickness for clean corners. Re-rendered `Beat7_KippOderStoss`: 0 layout issues.

## Reopened 2026-10-04 — Φ_p label, section-host fix

- Modul 4 `Beat2_PersonenPhiP`: live „Φ_p = 80 W“ was anchored on the floor line's left end and sat on the house wall; now right-aligned beside the person's glow inside the room. Re-rendered: 0 layout issues.
- Modul 3 full section render (`Heating_03_Convection`) crashed: section hosts only copy `NARRATION` etc., but `Beat3` read `self.USES` and `Beat7` its room/energy constants as class attributes. Moved them to module level (`AIR_CHANGE_USES`, `ROOM_A … DT_LAIBUNG`); no other Heating beat uses class constants or helper methods.

## Reopened 2026-10-04 — Modul 5 Beat2 symbols only, pastel palette

- `Beat2_BestrahlungUndFlaeche` decluttered: only the symbols G (sun), A (opening), F_f (glass) on the drawing; no numeric readouts, dimension values or chained equations; G, A, F_f fly into `Φ = G · A · F_f [W]`. Narration without numbers.
- Sun redrawn as a soft glowing disc with four evenly spaced, semi-transparent parallel rays onto the window.
- Module palette softened (pastel): `COLOR_G #F7E3A1`, `COLOR_A #9FD3F2`, `COLOR_WIN #A8E6EF`, `COLOR_FF #F4B183`, `COLOR_GOLD #F3D98B` — applies to all Modul 5 beats.
- Re-rendered Beat2: 0 layout issues. Before-copy: `checks/m5_scene_5_before_beat2_pastel.py.txt`.
- Rendered `Heating_04_InternalGains` at -ql (2 min 23 s, no errors).

## Reopened 2026-10-04 — Modul 5 Beat3 rebuilt

- `Beat3_GWert` rebuilt on dev request (animation and colours not good): double-glazing section with an energy-flow (Sankey) diagram — 100 % solar band from a soft sun, 20 % reflected curving away, 30 % taken up by the glass (panes glow), 50 % transmitted; the warm glass then releases 10 % inward and 20 % outward. A brace collects transmitted + inward heat → g ≈ 0,6, and g flies into `Φ = G · A · F_f · g [W]`. Band widths are proportional to the shares (`SHARE_R/T/IN/OUT`, `G_VALUE = SHARE_T + SHARE_IN`); the ten-packet tally and all numeric equations are gone. Pastel `COLOR_GVAL #A8DDB5`, new `COLOR_REFL`, `COLOR_GLASS_HEAT`. Cites DIN EN 410. Re-rendered: 0 layout issues. Before-copy: `checks/m5_scene_5_before_beat3_sankey.py.txt`.

Status: closed 2026-10-04 (manually — repo MCP unavailable).
