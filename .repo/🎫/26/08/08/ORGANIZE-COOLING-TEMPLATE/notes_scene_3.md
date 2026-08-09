# Notes — Scene 3 Transmission & Humidity Template Migration

Ticket: `26/08/08/ORGANIZE-COOLING-TEMPLATE`

## Classes

- `Beat1_TransmissionOpaque` — house + sun → morph to `Q̇_T = U · A · ΔT_eq [W]`
- `Beat2_TimeLag` — thermal mass / evening peak; persists formula, highlights `ΔT_eq`
- `Beat3_VentilationHeat` — section house airflow → `Q̇_L = Q̇_sens + Q̇_lat [W]`
- `Beat4_SensibleVsLatent` — split screen; one formula panel at a time with unit lines
  - Sensible: `ρ_a [kg/m³] · c_p,a [kJ/(kg·K)] · ΔΘ [K] · q_v,R [m³/s]`
  - Latent: `ρ_a [kg/m³] · r [kJ/kg] · Δx [kg/kg] · q_v,R [m³/s]`

## Files

- Updated `scene_3.py` — full five-rule template
- Updated `generate_audio.py` — imports `NARRATION` via `narration_text`
- `build_full_video.py` — class names already matched

## Smoke

- `manim -ql` `Beat1_TransmissionOpaque` ✓
- `manim -ql -s` `Beat4_SensibleVsLatent` ✓
