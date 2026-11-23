# Scale Lighting Lamp To Room

## Problem
Beleuchtungswärme pendant used `B4_LAMP_UNIT = 3.2` in a room of height 3.3 (~2.4× true room_metre), so the fixture filled the section. Cooling internal-gains lamps used `×1.6` / `×1.4` over room scale.

## Fix
- Heating Beat4: `B4_LAMP_UNIT = room_metre(B4_ROOM_H)`.
- Cooling lighting scenes: `room_metre(room["h"])` without oversize multipliers.

## Tests
Language-agnostic cases forbidding the old units and requiring `room_metre`.
