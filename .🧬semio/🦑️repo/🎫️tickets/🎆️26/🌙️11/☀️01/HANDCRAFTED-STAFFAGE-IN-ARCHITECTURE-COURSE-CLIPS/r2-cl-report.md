# r2-cl: clip polish (OI-67, OI-112, OI-121, OI-132)

Rendered one scene at a time (ql, 15 fps, jobs 1; scripts in `T/🧰️cl/`), frames extracted with PyAV and read; generated output deleted.

## Fixed
- **OI-67** `E/🌬️ventilation/🎬️clip/📦️volume/🐍️.py` Beat1: house origin `O` moved from y -1.12 to -0.7, so the "10 m" dimension clears the formula box (frame at 60 s checked).
- **OI-112** `E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` Beat2: sun glyph plus dashed duct replaced by sun with rays, a solar collector glyph ("Solarkollektor"), a yellow flow line with animated particles into the regeneration coil ("Solarwärme → Heizregister"), a coil pulse and a "70 °C" readout above the coil. The coil sits in the exhaust lane before the wheel, so the picture now reads: sun, collector, coil, hot regeneration air, wheel. Total beat length unchanged. Narration en and de not changed: the text is factually correct for solar-assisted DEC. No audio touched.
- **OI-121** `E/🌬️ventilation/🎬️clip/❓️need/🐍️.py` `_room_occupants`: the prototype `_stand` motion is gone; spacing now uses `staffage_height(cast)` and the scene height formula. A shared helper was tried and dropped because the architecture test extracts a fixed function set.
- **OI-132** `E/🔥️heating/🎬️clip/🧱️conduction/🐍️.py` Beat4 intro: the ground line moved to y -0.9 (row now centred in the frame) and a yellow "1,75 m" height dimension added left of the person, on rh2's true-scale ground line.

## Tests
- `staffage_only` guard: passes (the solar collector is built from glyph primitives, not flagged).
- volume tests and cooling clip tests: pass. `ventilation 🧪️architecture`: 5 run, 1 failure `test_closeup_radiator_scales_and_clears_floor` (radiator height 0.568 vs 0.68, `natural` chapter/asset, not touched by me). `_stand` rooms test passes.

## Open
- The collector is a primitive glyph, not a handcrafted asset (none in the catalog); a handcrafted solar collector cast would be the long-term replacement.
