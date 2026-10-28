# Ventilation Architectural Integration

The authored chapter sections retain their outer coordinates and slab datum. Their interiors use the shared room-section geometry with left/right/both apertures declared before drawing. Window glyphs are inserted into those apertures, and standing occupants preserve the 1.75 m / 2.5 m scale ratio. The physical volume-house model provides closed construction polygons rather than overlaid masking rectangles. Close-up radiators use 1.1 m by 0.6 m dimensions and 0.12 m floor clearance.

Language-neutral cases are in the ventilation clip architecture tests; Manim supplies independent geometry bounds and closed-path checks. Tests are added before modifying chapter helpers.

## Implementation and Verification

- Natural, need, mechanical and volume chapter room helpers delegate to shared solid room sections while preserving existing outside dimensions and floor datums. Explicit aperture side, height and centre replace opaque overlay masks.
- Natural window controls use the shared projected opening model, including continuously adjustable casements and bottom-hinged tilt. Opposite facade sashes project inward on the correct side; changing the effective opening keeps the structural hole intact.
- Volume house construction comes directly from its physical metre-based wall, roof and slab polygons, with real cuts for all four windows. No background masking rectangle covers continuous wall lines.
- Close-up ventilation emitters are 1.1 m wide and 0.6 m high, with 0.12 m floor clearance. Their geometry scales together with the room; the heat-flow symbolic heater retains its previous on-screen dimensions explicitly.
- Standing occupants remain 70% of clear room height and touch the slab top. The low stack inlet was raised 0.10 scene units so its sill clears the floor slab.

TDD: initial cases failed against the old helpers. The final portable case suite passes all five tests, including four chapter subcases. Manim bounds, filled-path checks and actual frame rendering provide third-party comparison. All five edited chapter sources parse successfully.

A 1280×720 representative frame was rendered and visually inspected. The slabs and walls connect, apertures stay open, left/right sashes project inward, silhouettes are connected, and radiators are floor-anchored. A two-second 15 fps animation of the same geometry was generated in the ticket generated folder. The preview and smoke input scripts remain at the ticket root; generated media is temporary.

Concurrent theme edits briefly introduced a malformed call in natural ventilation. Its editor repaired the keyword before the next parse, and those edits were preserved.

## Authored Files

- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`
- `🌬️ventilation/🎬️clip/❓️need/🐍️.py`
- `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`
- `🌬️ventilation/🎬️clip/📦️volume/🐍️.py`
- `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py`
- `🌬️ventilation/🎬️clip/🧪️architecture/🐍️.py`
- `🌬️ventilation/🎬️clip/🧪️architecture/🔣️.cases.json`
