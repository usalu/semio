# Passive Natural Ventilation Focus

Ticket: `26/08/09/PASSIVE-NATURAL-VENT`

## Intent
Shift Scene 6 from mechanical HVAC taxonomy to **passive-house natural ventilation for comfort**, with mechanical systems only as reserve.

## New beat map
1. `Beat1_PassivhausIdee` — comfort first, envelope, natural levers, mechanics as backup
2. `Beat2_Fensterregeln` — window as adjustable valve (closed / partial / timed)
3. `Beat3_Querlueftung` — cross ventilation sized for comfort
4. `Beat4_Auftrieb` — buoyancy / shaft, Δp = h·g·(ρ_a−ρ_i) via equation_row
5. `Beat5_Nachtlueftung` — night purge of thermal mass
6. `Beat6_KomfortStrategie` — strategy ladder: natural → night → mechanical reserve

## Cleanup
- Removed MathTex-based mechanical beats (WRG, RLT classes, mixing/displacement, old summary)
- `generate_audio.py` / `build_full_video.py` / launch Beat1 command updated

## Follow-up
Re-run TTS + full video build for all 6 beats.
