# Heating Manim academic review

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable (`repo://goals`, `ticket_open`). Work logged here.

Read all Heating beats in:

- `tutorial/energy/demand/Heating/1_introduction/scene_1.py`
- `tutorial/energy/demand/Heating/2_conduction/scene_2.py`
- `tutorial/energy/demand/Heating/3_convection/scene_3.py`
- `tutorial/energy/demand/Heating/4_internal_heat_gain/scene_4.py`
- `tutorial/energy/demand/Heating/5_solar_heat_gain/scene_5.py`
- `tutorial/energy/demand/Heating/final_calculation/merged_scenes.py`
- `tutorial/energy/demand/Heating/full_heating_video.py`

Applied in Heating Manim sources (2026-09-20):

- Module 1 Beat 3 is surface convection (R_si/R_se), not a gap. Beat 5 no longer puts ventilation inside U.
- Series intro on-screen topic is Heizwärmebedarf. Playlist titles match TITLE_DE.
- Finale: Q_Verlust = (H_T + H_V) · G_t. Removed Φ_design × F_Klima.
- Φ vs Q separated on gain slides. η_h cuts demand, not load.
- c_Luft named volumenbezogene Wärmekapazität. g-value includes secondary heat.


Must-fix:

1. Module 1 treats ventilation through a gap as part of ISO 6946 U-value.
2. Series intro / playlist say Heizlast; finale is Heizwärmebedarf.
3. Φ_design × F_Klima is not annual Q_h.
4. Q_int = Φ_p+Φ_e+Φ_l unit mix; η_h described as cutting load.
5. c_Luft called specific heat capacity; solar Beat 8 says gains are not lost.
