# R2 hp report: staffage cache, cast export, motion check

Agent `hp`, round 2 (cache and performance). Scripts are in `🧰️hp/`: `stale_keys.py`, `export_manifest.py`, `motion_scan.py`.

## 1. Staffage cache (`.🧬semio/🎓️teaching/⚡️cache/staffage/`)

Keys are recomputed by `stale_keys.py`, which mirrors `cast_key` in `V/🧍️staffage/🐍️.py` (entry JSON, catalog file, environment presets when present, and the exporter source digest). The same function produced the existing folder names of the current casts (81 current key folders contained their cast folders), so the mirror matches the loader.

| | Before cleanup | After cleanup |
|---|---|---|
| Total size | 2.42 GB (2 426 676 999 bytes incl. temporaries) | 0.34 GB |
| Entries | 452 | 85 (84 current, 1 in-flight temporary) |
| Keys produced by the manifest | 92 (143 casts share keys where entries are identical) | 92 |
| Stale key folders | 367 (2.08 GB) | 0 |
| Stale temporaries (`.tmp-*`, older than 1 h) | 3 | 0 |

- Deleted: 370 entries (367 stale key folders and 3 old `.tmp-*` folders). One `.tmp-*` folder was skipped because it was younger than 1 hour (an export in progress). Nothing younger than 15 minutes was stale when deleted.
- Left in place: 84 current key folders, with 3 incomplete (other agents were still rendering casts into them when the cache was read). 10 manifest casts had no folder yet.
- Note: the deletion ran while other agents were changing sources. Any stale key that another agent later needs is recreated by `ensure_casts`.

## 2. Cast export with `repo tools exec --light` (143 casts)

One exporter process per cast, run as `repo tools exec --light bun -- 🟦️.ts --manifest … --ids <cast> --out <tmp> --lang en`. Peak memory is the `[peak]` line the exporter prints (maxRSS of its own process). Frame count is the length of `frames` in each cast's index. Output went to the ticket generated folder and was deleted afterwards.

- All 143 casts exported with exit 0. Total wall 1848 s (about 13 s per cast on average; most time is the `repo` and bun start-up).
- Max peak per cast: 1206 MB (`heating-internal-gains-server-rack`), then 757 MB (`demand-primary-energy-tree`). Typical casts are 100 to 250 MB.
- Slowest casts: `demand-primary-energy-tree` 127 s (1 frame, 757 MB), `demand-recap-desk` 36 s, `ventilation-need-occupant-mirrored` 30 s (75 frames).
- The P1 report gave about 370 MB for the five worst casts. Two casts now exceed 700 MB in a single process (see the open point below).

Flags:

- **600-frame cap:** 25 casts sit exactly at 600 frames (the cap, not above it). None exceed it. They are: `heating-internal-gains-server-rack`, `heating-internal-gains-pendant-lamp-warm-up`, `physics-fundamentals-vacuum`, `physics-fundamentals-pot`, `physics-fundamentals-kettle`, `physics-fundamentals-shower`, `cooling-summer-strategy-heater`, `cooling-heating-comparison-toaster`, `cooling-heating-comparison-laptop`, `cooling-heating-comparison-cooler`, `shared-watt-laptop`, `shared-watt-toaster`, `shared-watt-heater`, `ventilation-mechanical-pendulum-fan`, `demand-system-losses-boiler`, `demand-primary-energy-boiler`, `demand-practice-boiler`, `demand-system-losses-tank`, `heating-calculation-boiler`, `heating-calculation-tank`, `heating-conduction-toaster`, `heating-conduction-radiator`, `heating-calculation-radiator`, `heating-calculation-radiator-front`, `heating-calculation-radiator-mirrored`.
- **Frame SVGs above 200 KiB (total per cast):** 73 casts. The total is the sum of all SVG files of a cast, and most casts are 200 KiB or more because of many frames. Per single frame, only 3 are above 200 KiB: `demand-primary-energy-tree` (288 KiB), `ventilation-volume-student` (231 KiB) and `ventilation-volume-office-worker` (231 KiB). If the 200 KB limit is meant per frame, these three are the ones to fix. If it is meant per cast, the list is long and points at the frame count, not one bad frame. The full per-cast list is in the TSV, which the generated folder held before deletion; the numbers above are complete.
- **Peak memory above 400 MB:** `heating-internal-gains-server-rack` (1206 MB) and `demand-primary-energy-tree` (757 MB).

Open point: the peaks of the two casts above are not explained by the frame count (server-rack has 600 frames, tree has 1 frame and a large single SVG). Someone should look at the cast's geometry before the next full export. This is for the performance owner and was not changed here.

## 3. One `staffage_motion` per visible device per scene

Static scan with `motion_scan.py` over all clip, visuals and staffage modules (36 scenes call `staffage_motion`). The scan follows same-module helper calls with their argument text. Each scene that reuses a cast id was checked by reading the call sites.

Result: no scene creates two motions for the same visible device at the same time.

Same cast id used more than once in one scene, all checked and all distinct devices or separate phases:
- `summer-strategy` `Beat1_LastZuerstSenken`: `_person` (occupant) and `_blinds` (blind per window, two windows) use the variable name `cast`; the casts differ.
- `mechanical` `Beat3_Anlagenvergleich`: `panel()` three times, each at its own `px` (three ventilation panels).
- `volume` `Beat2_Luftwechselrate`: `_use_glyph` per use, one room per use, shown in sequence within a key group; occupant appears in the `passiv` and `op` rooms.
- `primary-energy` `Beat3_Rechenbeispiel`: `_plant_panel` for gas and for wp at `px` -6.85 and 0.1.
- `primary-energy` `Beat4_Referenzgebaeude`: `_furnish` for design and reference houses.
- `practice` `Beat3_SerienAbschluss`: `_talker` for landlord (x 2.70) and tenant (x 3.50).
- `conduction` `Beat4_WattVergleich`: the intro glyphs are faded out (line 878) before the device glyphs are placed (line 952).

Observations (not violations):
- `ventilation/❓️need` `Beat1_Lueftungsbedarf`: `_room_occupants` creates a `prototype = _stand(...)` only to read `prototype.height`. It is never added to the scene, so it is an invisible motion. `staffage_height("ventilation-need-occupant")` gives the same value without building a motion.
- Static counts undercount loops. The scan shows one call site for a loop that makes several motions (e.g. the four occupants in `_room_occupants`). The duplicate check above was done on call sites.

## Tests and deletions

- No code or manifest was edited. The new scripts are in `🧰️hp/`.
- `🗑️generated/hp/` (export output, logs, stale JSON reports) is deleted at the end of this run, as AGENTS.md requires.
- Not run: vitest and Python unit tests (not needed for measurement-only work).
