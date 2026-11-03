# mv report (round 2): move to machinery and honest sorption wheel

## 1. Move
- `plate-heat-exchanger` and `sorption-wheel` moved with plain file moves from `S/🗂️catalog/🔌️appliances` to `S/🗂️catalog/🏗️machinery` (machinery now 14 assets).
- `asset.category` set to `machinery` in both asset JSONs; the catalog root manifest `🗂️catalog/🔣️.json` entries set to `machinery` (re-read before editing: fx2 had already added `transport` to plate-heat-exchanger analytics, kept).
- No other references to the category or folder path exist: the cast manifest `V/🧍️staffage/🔣️.json` carries only asset ids (lines 99, 100), tests and fixtures hold no per-asset paths, the React overview maps category ids to icons only. No alias.

## 2. Sorption wheel, physical model (`S/🗂️catalog/🏗️machinery/sorption-wheel/🔣️.json`; input script `T/🧰️mv/sorption.py`, one-shot, not idempotent)
Reference air 21 degC, 7.8 g/kg.
- Process stream 0.4 m3/s (2.1 m/s over 0.19 m2): moisture -3.4 g/kg releases 0.48 kg/s x 3.4 g/kg x 2.8 MJ/kg = 4.6 kW; with about 0.9 kW carried over by the matrix (14 kg x 920 J/kgK x 20 K / 300 s) the dry air leaves +11 K at 4.4 g/kg. Vent emitter `process-air`.
- Regeneration stream 0.14 m3/s (a third, 0.064 m2), coil heated to 70 degC (7.9 kW), takes 4.6 kW desorption plus 0.9 kW matrix, leaves +33 K at 17.8 g/kg. Vent emitter `regen-air`. Thermal COP 4.6/7.9 = 0.6.
- Two operating keys replace `run`: `rotation` (0, 6, 12 rev/h) and `regeneration` (coil off, about 45 degC, 70 degC). Emitters scale with `regeneration`; the half level is the usual linear approximation.
- Region heats are now only surface losses at air-set temperatures (disc 0.19 m2, 18 W; regen face 0.064 m2, 28 W; hub bearing 1.5 W instead of 4 W). Solved surface temperatures at `rated`: disc 31.5, regen 58.8, hub 37.9 degC (previously the hub reached 60 degC).
- Anchors `process-collar`, `regen-collar`; poses `off`, `slow` (6 rev/h, 45 degC), `rated` bind both emitters; provenance, assumptions, summary rewritten (en and de), solar wording replaced by the heating coil; references `ashrae-2020-equipment` and `henning-2007` added; manifest analytics gained `transport`.
- Clip `spin` is real time: 300 s, one turn per 5 minutes (12 rev/h), loop, fps 1. `start-and-run` gets tracks for both keys (coil ramps over 60 s) and two events.
- Scene `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`: constant `WHEEL_TIMELAPSE = 30.0` (one turn in 10 s) replaces `wheel.rate = 1.0`.

## 3. Verification
- Full catalog check: `✓ manifest` and 147 assets, 0 failures (148 check marks); plate-heat-exchanger and sorption-wheel pass.
- Staffage vitest (`q2-vitest-alias.config.ts`): 51 of 53 files pass, 1070 tests pass, 11 skipped, 2 fail, both unrelated to my assets: `🌡️thermal-span` (laptop surface value 46.7 vs 58.3, laptop edited by ha2) and `🫕️phase-change` (electric-kettle plateau balance 1.7e-5 vs 1.9e-6, kettle asset is untracked/new). Open for the owners.
- React vitest: 17 of 17 files, 107 pass, 1 skipped.
- Visuals tests: `🐍️` plus `staffage_only` 27 of 27 OK. Integration, scenes, icons, transforms, sections, section_consumers: 48 run, 17 errors, all from concurrent foreign edits (`measure/🟦️.ts:408` syntax error while another agent edited it, unknown cast `demand-certificate-radiator`, `_Path` NameError in a scene, `Beat1_SolarIrradiance` caption). None touch the moved assets.
- Rendered `Beat2_SorptionsKuehlung` (ql, 15 fps, de) and looked at frames: wheel with hot sector, plate exchanger, heater coil and flows read correctly; the first attempt failed twice because other agents were mid-edit (`rawFile` in the cast exporter, a half-written scene line); the third run was clean.

## 4. Open
- Narration of the scene still says "Solarwärme regeneriert das Rad" with a sun glyph feeding the coil; consistent with a solar-fed coil but the coil is the heat source in the asset.
- The face-on view cannot show the process and regeneration plumes; they appear only in views that draw emitters.
- Generated output deleted; scripts kept in `T/🧰️mv/`.
