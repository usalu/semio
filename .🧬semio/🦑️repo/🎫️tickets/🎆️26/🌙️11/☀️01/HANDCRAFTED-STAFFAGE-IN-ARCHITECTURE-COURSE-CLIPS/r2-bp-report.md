# r2-bp: breathing and plume transport layers

Scripts in `T/🧰️bp/` (`tr.ts` transport numbers per cast, `sheetdir.py`/`sheetscale.py` contact sheets, `mkvar.py` variant manifests, `patch_*.py` edits, `pytest.sh`). Looked at: first, middle, last frames of all transport casts, true-scale (100 px figure) strips, and need Beat1 rendered (ql, 15 fps) at 12/24/40 s before and after.

## Findings (before)
- Transport casts: `shared-person-pathlines` (merged by xc: need, natural, volume, mechanical), `ventilation-need-occupant-mirrored`, `ventilation-volume-{office-worker,student,cook}`. No cast uses parcels, streamlines or condensation; correct: exhaled 34 degC air mixing with 21-22 degC room air never saturates, the cooling casts have no transport, shower/kettle vapour is drawn by scene code.
- Loop pop: the 5 s loops started from empty air (pathlines grew from nothing and vanished at every wrap).
- Bounds: pathlines of 30 s life blew the cast bounds up (office worker maxX 1.11 to 2.04 m, maxY 1.46 to 2.17 m). `_Motion` measured the group by those bounds, so `get_top()`, `get_x()`, `height` (need Beat1 `heads`, `xs`, `prototype.height`, natural `stuffy_spots`) pointed at the plume, not the head.
- Hairline 0.08 mm pathlines are 0.2 px wide at 480p: invisible until thickened; thickened at 30 s life they became a grey mass.
- Plume leaned 30 degrees above a bowed head (emitter direction = crown bone angle, 60 deg when seated) and drifted at 0.1 m/s with the preset wind.
- Nose jet exit 4.3 m/s peak (9.5 mm nozzle, 0.7 cm2); real nostrils about 1.5-2 cm2, 1-2 m/s. Plume rise 0.09-0.14 m/s.
- Breath 14 bpm x 0.4 L (standing) did not divide the 5 s loop; seated 15 bpm vs 4.8 s; cook 16 bpm vs 2 s.

## Fixes
- Exporter (`S/🔨️modules/📤️export/🎭️cast`): new cast fields `transportAgeS` (parcel/pathline life; frames sample absolute time from that offset so frame 0 is a developed field) and `transportStrokeMm`; index gets `figureBoundsM` for transport casts (schema json and TS twin, rules: positive, needs transport, age needs clip). Depiction option `transportStrokeMm` (`🔗️contracts`, `💨️transport-view`).
- Python `_Motion` (`V/🧍️staffage/🐍️.py`): bounding box is the figure (probes at `figureBoundsM`) when the index has it; the air layer no longer moves top, centre, height. Verified in need Beat1.
- Solver (`💨️transport/🔥️sources`): plume emitters rise vertically (down when colder), not along the bone; docstring updated.
- Assets (`standing-adult`, `seated-office-worker`, `person-cooking`): resting pattern 12 bpm x 0.5 L (standing; one breath per 5 s loop), seated 12.5 bpm (one per 4.8 s), cook 15 bpm (one per 4 s); nostrils 15 mm, mouth 20 mm (peak exit 1-2 m/s), nose direction 45 deg below the head axis; plume diameter 0.16/0.14/0.16 m (rise 0.15-0.25 m/s above the head).
- New preset `office-still` (22 degC, 0.02 m/s) in `S/🗂️catalog/🌤️environments`, used by all five transport casts so plumes rise instead of leaning.
- Manifest: ages 3.5/3/3.5 s, stroke 0.1 mm, cook clip `stir-and-taste` (4 s = one breath at 15 bpm).
- Result: figure bounds equal the figure; frames 0/mid/last all populated; need Beat1 at 480p shows light grey plumes over heads and breath jets between faces, no clutter over the bodies.
- Fixture vectors (3 invalid cases) and one export test (age warm-up, 0.3 stroke, figure bounds) added to `🧪️tests/🎭️cast-export` and its fixture.

## Verified
- Cast-export vitest: all mine pass; 2 failures (`parses the contract invocation`, `refuses ... missing language`) come from another agent removing `--lang` from the exporter.
- Transport and respiration vitest: 4 files, 47 passed. Catalog check of the 3 assets: pass.
- Python staffage_only/scenes/main: 38 tests, 1 failure + 7 errors, not mine: `unknown --lang` (the same exporter change vs the Python caller) and `test_thermal_span_every_clip...` for physics fundamentals (2 spans). `uv run` is unusable (pyet/pandas resolution), I ran the cache python.

## Open
- The `--lang` removal must be completed in `V/🧍️staffage/🐍️.py` by its owner.
- Laptop vent jet still reaches about 0.5 m along the desk (physical: 1.1 L/s, +10 K).
- natural/mechanical/volume scenes with `shared-person-pathlines` were not re-rendered; only need Beat1.
