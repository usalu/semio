# Cooling Manim academic review

Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`

Repo MCP was unavailable (`repo://goals`, `ticket_open`). Work logged here.

Read all Cooling beats in:

- `tutorial/energy/demand/Cooling/1_heating_vs_cooling/scene_1.py`
- `tutorial/energy/demand/Cooling/2_internal_gains/scene_2.py`
- `tutorial/energy/demand/Cooling/3_transmission_humidity/scene_3.py`
- `tutorial/energy/demand/Cooling/4_solar_radiation/scene_4.py`
- `tutorial/energy/demand/Cooling/5_systemauslegung/scene_5.py`
- `tutorial/energy/demand/Cooling/6_lueftungssysteme/scene_6.py`
- `tutorial/energy/demand/Cooling/full_cooling_video.py`
- `tutorial/intro/intro_scene.py`
- `tutorial/energy/demand/Cooling/README.md`

Applied in Cooling Manim sources (2026-09-20):

- Series identity stays Kühllast (VDI 2078 design power), not annual Kühlenergiebedarf.
- Teil 1 winter gains cut Heizwärmebedarf (DIN V 18599), not EN 12831 Heizlast.
- Teil 2–5: Kühlbedarf wording → Kühllast. Internal-load DIN chips → VDI 2078.
- Opaque summer load uses Δθ_eq (sol-air). Dark roof absorbs, mass stores.
- Q̇_S,tr is solar load through glazing, not opaque transmission.
- Teil 5 sizes cooled AHU supply air (18 °C), not outdoor air. ρ_a ≈ 1,2 kg/m³.
- Teil 6 title is Lüftungssysteme. WRG uses η, not Φ. Cross-vent does not replace a chiller.
- Teil 6 is not Passivhaus: PHI requires MVHR. Beat 1 is last-first summer comfort (DIN 4108-2); night purge is not a PH-only idea.

Must-fix:

1. Winter gains called Heizlast; EN outro said demand, DE said load.
2. Beat8 / leftover beats mixed Kühlbedarf into a load series.
3. Teil 5 said cool outdoor air while the diagram uses 18 °C treated supply.
4. Q̇_S,tr narrated as “transmission cooling load”.
5. Φ used both as heat flow and as heat-recovery efficiency.

Ticket close: repo MCP `ticket_close` unavailable. Sources patched 2026-09-20. VO not re-synthed.
