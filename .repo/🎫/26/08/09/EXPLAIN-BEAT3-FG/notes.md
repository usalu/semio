# Explain Beat3 Lighting Parameters

Ticket: `26/08/09/EXPLAIN-BEAT3-FG`

## Done

In `Beat3_DevicesLighting` (`scene_2.py`):

- Extended `NARRATION` with `pl` (P_Licht = installed luminaire power) and `fg` (f_g = coincidence factor / share running at once).
- After revealing `Q̇_Licht = Σ P_Licht · f_g`, ring each symbol with `highlight_param` and `hold_for`, same pattern as `P_el` / `f_N` for devices.
- Longer beat is intentional — timing comes from the new English clauses via `hold_for`.

## Follow-up

Re-run `generate_audio.py` (or at least beat 3) so TTS includes the new clauses in `beat_3_audio.mp3`.
