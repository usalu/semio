# Full Layout Pass On Ventilation Need Beat

Scene: `Beat1_Lueftungsbedarf` personen + jahreszeit.

## Fixes
1. Room shifted up/left (`[-3.15, 1.35]`, `5.0×2.2`) so the right column is free.
2. Rate chips sit `0.28` below the slab (height measured from the label) — no floor collision.
3. `4 ×` / `30 m³/h` / `=120` wipe in the same play as the formula panel (`FadeOut(count_strip)` + `remove`) — no leftovers above the formula.
4. Winter/Sommer compact stack placed from room-right and formula-top with gap checks so they cannot sit on the room or the formula.
