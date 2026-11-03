# Migration M5 report: ventilation clips

Scope: `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/**`.

## Done

- `person_glyph`, `seated_person_glyph`, `radiator(...)` removed from `⚙️mechanical`, `❓️need`, `📦️volume`, `🪟️natural`, `🌡️heat-flow` (imports removed too; `lamp_glyph` import dropped from volume). `🤝️handoff` and the series file had none.
- Casts added to `V/🧍️staffage/🔣️.json` (all `ventilation-<clip>-*`, `theme: dark`):
  - occupants: `need-occupant`, `need-occupant-mirrored` (alternate, rates 0.9..1.1), `mechanical-occupant`, `natural-occupant`, `volume-occupant` (standing-adult, clip idle-breathing, thermal).
  - `volume-office-worker`, `volume-student` (seated-office-worker typing), `volume-cook` (person-cooking stirring), `volume-surgical-lamp` (pendant-lamp on).
  - `natural-radiator-on` / `-off` (panel-radiator rated / off, thermal, mirrored so the wall sits on the exterior side), `heat-flow-heater` (electric-fan-heater heating, thermal).
- Breathing occupants (all casts with a person) use `"transport": ["pathlines"]`. I tried `parcels` and `streamlines` first. Parcels render as large orange blobs that cover the room; streamlines are ambient flow. Pathlines give a small grey exhale trail rising from the face, which fits the CO2 narrative. The coordinator's note was right that transport is exposed in the cast exporter.
- `_stand` helpers now return `staffage_motion` anchored on the slab top; the radiator in `🪟️natural` is two static thermal casts cross-faded by `_power` (`_remember`/`_level` keep authored opacities).
- `Indicate(person)` replaced with a scale pulse (Indicate would recolour the thermal figure). `TransformFromCopy(person, seg)` in volume uses `_still(person)` (a frame-frozen copy), because the frame-swap updater broke the morph (broadcast error).
- `_room_occupants` uses a body width (0.2 x height) instead of the group width, which now includes plumes.

## Kept on purpose

- `respiration_parts` is not used by any ventilation clip, nothing to replace.
- Room sections, wall apertures and animated sashes (`window_glyph`, `open_window`, `window_opening`) stay: they are diagrammatic building sections with the sash animation the narration relies on. No `tilt-window` cast used because no window is depicted as a free-standing object.
- Operating table (two lines) and the kitchen hood outline in the volume mini rooms, the blackboard rectangle in the classroom, storage crates: diagram symbols, not staffage assets (no catalog asset).

## Verification

- Tests: `🧪️architecture` (5, ok) and `📦️volume/🧪️tests` (2, ok; needs `SEMIO_TEACHING_OUTPUT_ROOT`). Test tolerances changed: staffage group bounds include plumes and a 5 cm sole margin, so occupant height is asserted as a lower bound, bottom within 6 percent, radiator height/clearance within tolerance; width assertion of the old radiator glyph dropped (`width_m` removed from cases).
- Rendered (l, 15 fps, de, speaker) and looked at frames: need (Beat1, plumes rising in the room), volume (Luftwechselrate use rooms, Volumenstrom people morph), heat-flow (Luftwaerme heater), natural (Stoss/Kipp radiators, Fensterregeln occupant), mechanical (Anlagenvergleich). The final op-room tweak (lamp 0.5 tall, occupant x 0.42) was checked on a still of `_use_glyph`, not in a full render. Natural beats 4 to 6 and handoff were not individually looked at.

## For the coordinator

- Parallel renders of the same chapters clash on the shared output dir; one segment failed with FileNotFoundError until the duplicate run was killed.
- `repo tools exec ffmpeg` is not provisioned here; I used the imageio ffmpeg in the teaching python cache.
