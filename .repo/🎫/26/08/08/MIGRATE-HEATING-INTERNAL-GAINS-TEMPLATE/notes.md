## Internal heat gains migration
- Source: `4_internal_heat_gain/merged_scenes.py` (Scene1–3, LightingHeat, Scene5; skip FullInternalHeatGainVideo)
- Target: `4_internal_heat_gain/scene_4.py` Beat1–5 on generate-manim-tutorial template
- TITLE: Modul 4: Interne Wärmegewinne
- Accent locals: COLOR_PEOPLE / COLOR_EQUIP / COLOR_LIGHT (pink/cyan/yellow continuity)
- formula_panel with SI units (W, W/m²); watt_anchor on person power (~90 W)
- German caption_bar + hold_for sync
- Animations preserved; content shifted up for formula/caption zones
- generate_audio.py / build_full_video.py / manim.cfg added
- Smoke: all 5 beats OK at 480p15 (`smoke_render.log`) — Rendered ×5, exit 0
- Note: initially written under mistyped `4-internal_heat_gain/`; moved into `4_internal_heat_gain/` to match `2_conduction` / `3_convection`

### Beats
1. Beat1_WinterInterneGewinne
2. Beat2_PersonenPhiP
3. Beat3_GeraetePhiE
4. Beat4_BeleuchtungPhiL
5. Beat5_SummeUndDichte

### Files worked on
- created: `tutorial/energy/demand/Heating/4_internal_heat_gain/scene_4.py`
- created: `generate_audio.py`, `build_full_video.py`, `manim.cfg`
- kept: `merged_scenes.py`
- ticket: notes.md, ticket.json, smoke_render.log
