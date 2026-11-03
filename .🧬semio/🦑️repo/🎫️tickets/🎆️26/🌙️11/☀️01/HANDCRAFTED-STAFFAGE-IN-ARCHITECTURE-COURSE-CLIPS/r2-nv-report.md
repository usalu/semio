# R2 narration consistency (agent nv)

Scope: narration and subtitle strings (en and de) of the energy clips, optimization, BIM (information, exchange, consistency) and excel (energy, program) clips touched in this ticket, compared with the staffage casts each beat now draws. Visuals were not changed.

Abbreviations: E = `🎓️teaching/🏛️architecture/⚡️energy`.

## Fixed

1. `E/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:355-356` (Beat2 "power"). The narration compared 80 W with "a bright bulb" / "eine helle Birne". No bulb is drawn; the beat shows a seated-person anchor (`watt_anchor(PHI_P_W, compare="seated")`, 80 of 100 W). New text: "roughly four fifths of a seated person's full output" / "rund vier Fünftel einer sitzenden Person". This is the one "Glühbirne"-type mismatch left in the scope.
2. `E/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:183-184` (Beat1 "interior"). "desks, screens, chairs" / "Tische, Bildschirme, Stühle" named screens that are not drawn (the workstation is a height-adjustable desk and an office chair). New text: "desks and chairs" / "Tische und Stühle".
3. `E/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:141` (docstring of `_workstation`, not narration). It said "desk with its monitor" and "the monitor's heat point". Changed to "desk and a swivel chair" and "the desk's heat point".

Timing: the speech modules read NARRATION from the scene files. The TTS audio and timing manifest for the two changed clauses (heating internal-gains "power", cooling internal-gains "interior", DE and EN) must be re-synthesised with each speech module's synth step. I did not run synthesis.

## Checked and consistent

- Watt anchors: the table in `E/🏗️authoring/🎨️visuals/🐍️.py:220-238` only knows laptop, toaster, heater, person, seated and sun. Bulb, monitor and vacuum are gone from the code, and no narration names them as comparisons.
- `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:571-597, 681-719, 901-906`: "Staubsauger" = `physics-fundamentals-vacuum`; "Wasserkocher" = `physics-fundamentals-kettle` (label matches); "Dusche (Durchlauferhitzer)" = shower. Scale list "Teelicht / ruhender Mensch / Staubsauger" is text only.
- `E/🧲️physics/.../🐍️.py:1324-1338`: "lamp / incandescent bulb" is the drawn pendant lamp. Fine.
- `E/🔥️heating/🎬️clip/🧱️conduction/🐍️.py:834-847, 863-864`: toaster, person and tealight are all drawn in Beat4.
- `E/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:246-247, 281-282`: laptop and person anchors are drawn; the title "Gerät" for the laptop anchor is generic, kept.
- `E/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:423-424`: laptop and server rack are drawn.
- `E/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:541-566`: laptop, server rack and lamp are drawn.
- `E/📊️demand/🎬️clip/♨️system-losses/🐍️.py`: radiator, tank, boiler and heat pump are drawn.
- `E/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:380-396, 888-910`: "at the meter" matches the gas and electricity meter casts.
- `E/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:539-552`: thermostat and radiator are drawn.
- `E/🌬️ventilation/🎬️clip/📦️volume/🐍️.py:705-708`: kitchen hood and operating table are drawn.
- `E/🌬️ventilation/🎬️clip/❓️need/🐍️.py:218-237`: four occupant casts are drawn.
- `E/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:248-251, 706-720`: duct fan, MVHR and pendulum fan are drawn.
- `E/📊️demand/🎬️clip/🔁️recap/🐍️.py:416-420, 889-905`: desk, laptop and lamp are drawn.
- `E/📊️demand/🎬️clip/🏷️certificate/🐍️.py:513-523, 603-638`: heat pump is a labelled bar, not a cast; the label matches.
- Solar radiation: "Bestrahlungsstärke" with the sun glyph, consistent.
- `🏗️bim/.../📊️information`: "Auto" = car cast. `🔌️exchange`: "Personen und Möbel" = imposed-person cast. `🔗️consistency`: "eine Person prüft / entscheidet" = reviewer and decider casts.
- Excel `📋️program`: "eine Person" = occupant cast.
- Optimization: narration names windows, insulation and overhang only, which are building parts. No device nouns.

## Judgment cases (not changed)

- J1, `E/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:325` vs narration 249-250. The badge is `watt_anchor(100, compare="person", title="Körperwärme")`, which draws a standing person glyph and labels it "stehende Person". The narration says "A single seated adult" / "Eine sitzende Person", and the beat's figures are seated. Options: (a) `compare="seated"` changes the glyph, so it counts as a visual change; (b) drop "seated" from the narration. I left both alone.
- J2, `E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:146-147`: "before any fan runs" / "bevor ein Ventilator läuft". No fan is drawn; the only device-like cast is the occupant with clip "fanning" (`🧍️staffage` manifest). Kept.
- J3, `E/📊️demand/🎬️clip/♨️system-losses/🐍️.py:539-540`: "underfloor heating" / "Fußbodenheizung" is named as a comparison and is not drawn anywhere. It was never a cast, so this is not a migration effect. Kept.
- J4, `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:889-891, 957`: "house meter" / "Hauszähler". The meter is an abstract window box labelled "Zähler", not a meter cast. The text matches the label. Kept.
- J5, outside the clip scope: `E/📖️concepts/📝️.md:345` says "Ruhende Person ≈ 80–100 W thermisch (≈ Glühbirne)". It is a concept doc, not clip narration. Not touched.

## Tests

- `T/m9_tests.sh` with the cooling clip tests and the cooling internal-gains tests: 7 tests, OK. Log in `T/🗑️generated/🧰️nv/tests-cooling.txt`.
- `T/m9_tests.sh` with `🏗️authoring.🎬️clip.🧪️tests`, which covers physics: 31 tests, OK (1 skipped). Log in `T/🗑️generated/🧰️nv/tests-authoring.txt`.
- Visuals `🧪️tests/🐍️`: 7 tests, OK.
- Visuals `🧪️tests/🐍️staffage_only`: not run. `E/🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json` is malformed at line 92 (a raw newline inside the `"sources"` key). Another agent is editing that file right now. I did not touch it. Re-run the guard once it is fixed.
- Visuals `🐍️scenes` and `🐍️integration`: not run. `scenes` ran past the 580 s timeout.
- No renders were made. Only text changed.

## Still open

- Re-synthesise the TTS and timing for the two changed clauses (see Fixed).
- Re-run `staffage_only` after the other agent fixes `🔣️staffage_only.json`.
- Decide J1 (glyph or narration) and J2 to J4.
