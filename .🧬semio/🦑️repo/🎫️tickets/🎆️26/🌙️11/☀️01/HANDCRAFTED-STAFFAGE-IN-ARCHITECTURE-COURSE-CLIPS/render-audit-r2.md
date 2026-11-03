# Render audit R2: ventilation clips

Method: each scene rendered alone (`--jobs` n/a, direct manim, ql, 15 fps, de, speaker) via `r2_render.sh`/`r2_chain.sh`; frames cut with `r2_frames.py`/`r2_zoom.py` (PyAV) and looked at. Frame PNGs lived in `🗑️generated/r2/f` and were deleted with the folder. Series dir `V` = `🌬️ventilation/🎬️clip`.

| Scene | Verdict | Frames (fraction of clip) | Fix |
|---|---|---|---|
| natural Beat1_Fensterregeln | fixed | .15 .50 .90 | label `innen 26 °C` overlapped head and plume. Occupant moved from room_c+1.3 to +0.85, label to +2.0 (re-rendered, clean at .90) |
| natural Beat2_StossOderKipp | ok | .15 .50 | none. Radiators are tiny and low-contrast (see A11) |
| natural Beat3_Querlueftung | ok, no staffage | .50 | none |
| natural Beat4_Auftrieb | ok | .20 .50 .85 | none. Occupant grounded, plume visible, thermal reads on #001117 |
| natural Beat5_Nachtlueftung | ok, no staffage (diagram only) | .20 .50 .85 | none |
| natural Beat6_GrenzenDerFreienLueftung | fixed | .20 .50 .85 | occupant stood in the wall and its plume crossed the right wall. x moved -1.65 to -2.0 (re-rendered at .50, clear). Note: orange tag "ungenutzt" starts about 2 px from the left frame edge (pre-existing, not changed) |
| handoff Beat1_Zusammenfassung | ok, no people | .15 .50 .90 | none |
| handoff Beat2_UebergabeHeizungKuehlung | ok, house section only | .10 .50 .90 | none |
| volume Beat1_Innenvolumen | ok, building section only | .20 .80 | none |
| volume Beat2_Luftwechselrate | ok | .30 .70 .98 + zoom crops of the use rooms | crates, office worker with desk and chair, blackboard class with three seated pupils, cook under hood, op-room table with stand and occupant all handcrafted staffage. Final op-room tweak seen only in morph frames (Create transition); table, light stand and occupant at x .42 are clear of the frame. Kitchen hood is small and the cook stands at the right wall (design limit of the mini room) |
| volume Beat3_Volumenstrom | ok | .30 .70 .84 .99 | four breathing occupants, equal size, grounded; label "4 Personen" is dimmed by design |
| need Beat1_Lueftungsbedarf | ok | .20 .50 .80 | four mirrored breathing occupants, plumes rise as grey pathlines. Last frame is mid-transition (formula morph), as authored |
| heat-flow Beat1_Luftwaerme | ok | .20 .50 .80 | fan heater is a small warm thermal cast |
| mechanical Beat3_Anlagenvergleich | ok | .20 .50 .80 | occupants equal scale, grounded, breathing animation plays |

Animation: occupant pathline plumes and breathing change between frames in all rendered rooms.

## Ad-hoc leftovers (not fixed, per coordinator)
- Fans: `_fan` in mechanical (WRG Zentral / Pendellüfter icons in Beat3, tiny) goes to M9.
- Radiators: natural Beat2 Stoss/Kipp radiators are about 8 px, white-grey and nearly invisible on #001117; A11's front viewpoint should also enlarge them.
- Remaining non-staffage symbols are diagram only (window apertures, room sections, clocks, gauges, thermometer).

## Tests
- `🧪️architecture` 5 ok, `📦️volume/🧪️tests` 2 ok, visuals `scenes.py` occupants and moisture tests 2 ok.
- Edited file: `V/🪟️natural/🐍️.py` (three one-line position changes).
