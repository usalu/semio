from pathlib import Path

TEXT = """# fix-phys: visual polish pass, German physics video (speaker and spectator 1080p30)

Scene file: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (only file edited). Lines refer to the state after the edits.

## Method

- Rendered the complete German preview (speaker, 854x480) into `🗑️generated/fix-phys/out`, extracted a frame every 4 s with the imageio-ffmpeg binary (273 frames), built 31 contact sheets (`🗑️generated/fix-phys/sheets`, 9 frames each, time stamp top right) and looked at every sheet. Leads were checked on held frames of this render and of the old final (`🗑️generated/fix-phys/old`).
- After the fixes: second complete preview render (speaker) with the targeted frames viewed, then the complete final render (`--profile final --jobs 3`, default root), a second 4 s sweep of the new speaker final (`🗑️generated/fix-phys/sheets2`) and 1080p frames of the fixed spots (`🗑️generated/fix-phys/fin`).
- The audit indices are shifted against the current render; the new render is 18:12.4 long like the audited one but the audit frame numbers differ from times (index x 20 s), so leads were matched by content.

## Leads: real or transient

| Lead | Verdict | What / where | Verified by eye |
|---|---|---|---|
| ~9:40 JAZ / VDI 4650 block overlaps "außen"/"innen" | REAL (9:48 to 10:00 in the sheet: the reference line ran through both tags and "Strom") | `jaz` moved to y 3.05 and `jaz_ref` to y 2.7 (lines 1915 and 1921), both still right-aligned at x 6.6, clear of the title chip and the tags (tags at y 2.15). Also dropped the doubled term: "Jahresarbeitszahl JAZ ≈ 3" / "Seasonal performance factor SPF ≈ 3" (the term is introduced in the caption), line 1915 | Preview and final frame at 9:56 (`fin/f_596.png`) |
| ~13:20 thermal mass chart: "24" collides with "t in h" | REAL (also "30" next to "θ in °C" on the humidity chart at 15:00 to 16:00, same cause) | `_axes` got `beside` (line 159 to 170): the x axis name sits right of the arrow tip instead of under it; used for the thermal-mass chart `cax` (line 2494) and the humidity chart `aax` (line 2855) | `fin` and preview frames at 13:32 and 15:24 |
| ~16:40 frame with only caption/header | TRANSIENT. At 16:40 and the neighbouring frames the scene is complete (winter window left, summer cooling coil right; the particles flow in and out). The empty frames in the sweep are the 1 s cross-fades: 17:56 (summary box fading into the reference card) and 0:04 (UI intro to roadmap). Nothing is held empty | `z2/y_1000.png`, `z2/tail.png` (one frame per second 17:50 to 18:03) |
| Beat1 shower icon | REAL | The cast crop window was a leftover (a stick plus a small square, 1 px wide at 480p). Now the whole shower fixture (rail, hand shower, hose) is fitted into the same 0.6 high slot centre (4.8, 1.95) with no window (line 725), so the slot test (`beside the vacuum and kettle`) stays green. The unused window arguments are gone | Final 1:36 to 1:44 (`sheets2/s_02.png`): thin rail with hose, readable, no empty crop |
| Beat2 shower "drop lines still run to y=-0.95" | PARTLY. -0.95 is the floor line and the bottom of the readouts, so ending there is right. The real defect was the source: the nozzle (0.15, 1.42 m) was the lower end of the handle, the drops started mid-rail 0.3 units below the head. Now `SHOWER_NOZZLE` is the head face (0.15, 1.65), line 510, so the head is at y 1.7, the rail ends just above the floor and the drops leave the head and land on the floor line | Preview frames at 4:36 and 4:40 and final sheet `sheets2/s_07.png` |
| `test_physics_scene_constructor_bounds` radiator 0.6812 vs 0.7384 +- 0.0222 | The SCENE is right, the EXPECTATION is stale (shared test). The scene draws `shared-radiator-winter` (elevation view, window crop 0 to 0.25 m) in a room 2.6 high: the visible radiator is 0.655 m real, which is 0.65 m (`radiator_height_m` in `🔣️integration.json`) at 2.6/2.5 = 1.04 per metre = 0.676 (actual 0.6812, 0.7 % off). The test compares with the height of `shared-radiator-front-winter` (0.71 m, the full front cast, a different cast and not the one drawn) times the metre = 0.7384 | Numbers computed with `fix-phys/heights.py`; not a visual change |
| Opening overview cards | By design, reported only: the UI recording at 0:00 to 0:04 shows six cards. Energie (text card with the reference list), Building Information Modelling (rendered frame), Grasshopper (rendered frame), Excel (rendered frame), Optimierung (rendered frame) show content; **Parametrik shows the placeholder "Noch keine Videos"** (empty card). The same UI recording plays again after the references card at 18:08 | `sheets/s_00.png`, `sheets2/s_00.png` |

## Further defects found in the sweep and fixed

- Heat pump (Beat4) thermometer read "-0 °C" at 9:40 (a float of -1e-15 after the animation back to 0): `t_read` now rounds (line 1892).
- Heat pump stack labels (Endenergie / Umweltwärme / Nutzwärme / the live Q_E label) touched the bar frame; moved 0.1 further left (lines 1854, 1855, 1860, 1905).
- Thermal-mass reference "DIN EN ISO 13786 ... Wärmespeicherung" ended 0.2 from the right frame edge; now right-aligned at 6.6 (line 2605).

## Checked and fine (no change)

- Captions: at most two lines in all 273 frames, no raw `$` or TeX. Title chip never collides with a reference chip. Numbers agree: 0,7 kW x 1,43 h = 1 kWh; shower 20 kW x 0,05 h; 30 kg x 4,19 kJ/(kg K) x 28 K = 3520 kJ = 0,98 kWh; COP 4 / 3,0 / 2,3 and Q_E 1 / 1,33 / 1,74 kWh; g 0,60 x 1,92 m2 x 500 W/m2 = 576 W; 240 kg, 480 kJ = 0,13 kWh, 20 m2 = 2,7 kWh = 3,9 h; phase shift 15 h to 21 h = 6 h; 419 kJ + 2257 kJ = 2676 kJ, ratio 5,4; 20 degC / 50 % = 7,2 g/kg, dew point 9 degC.
- Animated counters pass through other values (Q_h 9550 to 15000 kWh/a, Q_E 1,34 to 1,33 kWh, A 1,91 to 1,92 m2) while they run; the held values agree.
- At 0:48 to 1:04 and 6:44 the stage is sparse (one formula) by design; no overlap.

## Verification

- `uv run ... python -m unittest ...🧲️fundamentals.🧪️tests.🐍️` (SEMIO_TEACHING_PROFILE=preview, LANGUAGE=en, TEST_MANIM=1): 13 tests OK (the 5 shower icon errors of the tests report are gone, the 3 slot failures I caused on the way were fixed by returning to the declared slot).
- `test_physics_scene_constructor_bounds` still fails, expectation stale (see above, shared test, not edited).
- Final: speaker `Full_Physical_Fundamentals_speaker_de_1080p30.mp4` 18:12.4, 1920x1080, 30 fps; spectator `Full_Physical_Fundamentals_spectator_de_1080p30.mp4` 18:12.4 also ready (the run printed both "Ready:" lines; spectator spot-checked at 1:40 and 9:56).

## Shared follow-up (outside `🧲️physics/`, NOT done)

1. `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py` line 112: replace `expected = self.shared.staffage_height("shared-radiator-front-winter") * metre` with `expected = CASES["physical"]["radiator_height_m"] * metre` (the drawn radiator is the elevation cast cropped to the real 0,65 m; tolerance 3 % stays: 0.676 expected, 0.6812 actual).
2. The same `_axes` last-tick vs axis-name collision can exist in other families' charts that put the name under the arrow tip (this one is physics-local).
3. Overview: the "Parametrik" card shows "Noch keine Videos" (presentation code, intentionally untouched).
4. Speech was not re-synthesised (the audio track of all finals is the 2 kb/s silent track); narration text was not changed, en and de are untouched.

## Remains

- Preview and final frame checks were done on the German speaker video; the English video was not rendered, but all edits are language-neutral or changed in both languages (JAZ/SPF wording).
"""

Path(__file__).with_name("REPORT.md").write_text(TEXT, encoding="utf-8")
