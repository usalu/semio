## Migrate Heating Modul 5 — Solarer Wärmegewinn
- Source: `5_solar_heat_gain/merged_scenes.py` (Scene1–7 + Seasonal/ThermalMass; skip FullSolarHeatGainVideo)
- Target: `scene_5.py` Beat1–8 on generate-manim-tutorial template
- TITLE: Modul 5: Solarer Wärmegewinn
- formula_panel + German caption_bar + hold_for; `_fit_stage` + FORMULA_EDGE_BUFF=1.2
- generate_audio.py / build_full_video.py / manim.cfg added
- Smoke: all 8 beats Rendered at -ql (smoke_render_1.log + smoke_render_2.log)

### Beats
1. Beat1_VerlustZuGewinn
2. Beat2_BestrahlungUndFlaeche
3. Beat3_GWert
4. Beat4_SaisonaleWinkel
5. Beat5_Verschattung
6. Beat6_Waermespeicherung
7. Beat7_SpeichermasseFormel
8. Beat8_Hauptgleichung

### Files
- created: scene_5.py, generate_audio.py, build_full_video.py, manim.cfg
- kept: merged_scenes.py
