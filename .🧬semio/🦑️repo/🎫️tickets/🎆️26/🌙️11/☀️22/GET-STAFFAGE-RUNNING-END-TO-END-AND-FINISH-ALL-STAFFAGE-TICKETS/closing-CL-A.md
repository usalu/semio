FINAL (2026-11-26 16:15: `goat` torso junction and `red-squirrel` guard locks cured in the asset, both cases green, every proof green; the `goat` horn case stays red, it pins a rule allowance that was removed and no state of the asset can turn it green)

# 🐐️ Closing Lane CL-A: `goat` and `red-squirrel` Against the Detail Pins

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-A`, `M` = `U/🧰️matrix/MR`, `T14` = the 3D ticket `…/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`, `GEN` = `U/🗑️generated/matrix/CL-A` (deleted with this report; every number below was read there in a log of a run that ended by itself). No test, pin, fixture, engine or rule was edited.

## 1. Result

| Asset | Pin | Before | Now | Case |
| --- | --- | --- | --- | --- |
| `goat` | torso junction (`goatBarrel`, `🧊️model-skeleton`) | 240 failures of 240 expectations: `pelvic-barrel` → `abdominal-barrel` 46.100 mm, `abdominal-barrel` → `costal-barrel` 55.003 mm at every scale and pose (limit 0.01 mm), radii [0.136, 0.1655] against [0.124, 0.1555] and [0.146, 0.178] against [0.149, 0.1785] | 0 failures of 240: 0.000003 mm and 0.000004 mm, radii equal | "keeps both sections of each doe torso junction coincident through every named pose": 1 passed |
| `goat` | horn at 1:500 (`🧊️model-silhouette`) | steps 1 to 4 of the case ok, steps 5 to 7 red (0 findings where 1 is expected) | unchanged, not written (section 3) | "retains the measured horn stations …": 1 failed, as before |
| `red-squirrel` | guard locks (`squirrelBrush.guardLocks`, `🧊️model-skeleton`) | 36, 0, 0, 0 locks at 1:50, 1:100, 1:200, 1:500 (pin 36, 28, 18, 12) | 36, 28, 18, 12, each with its `tail-fur` surface; every tail bone keeps its member solid | "keeps the squirrel's tapered caudal chain and articulated brush together …": 1 passed |

Catalog files: `goat` `efb3968e2cac9733` (2,899,806 bytes) → `5a0aec3d2f9d90c1` (2,898,939 bytes), written 15:19; `red-squirrel` `fb001f2c619c4be9` (5,976,672 bytes) → `5929104548bbe3fd` (6,111,115 bytes), written 14:37.

## 2. Cause and cure

### `goat`, torso junction (`W/goat.ts`)

- Cause, read from the history of the file and from `T14/rework-goat-trunk.py`: when the trunk was re-cut by hand on 2026-11-22, each barrel was given one station of its neighbour (croup stations 0 to 7, loin starting again at croup station 6; loin stations 0 to 6, chest starting again at loin station 5). The solids overlap by one station, so the last section of one and the first of the next are two different sections. The gap never changed since (every commit from `01324745f80` on measures 46.1 and 55.0 mm); `T14/validator.md` already listed it as asset work.
- Cure: the croup ends in the first section of the loin, the loin in the first section of the chest, at every scale. Where the station before the last is the shared one, the doubled station is dropped (1:50 both junctions, 1:200 croup); else the last station becomes the shared one (1:100 and 1:500 both, 1:200 loin). The place is the neighbour's first section carried into the bone frame through `evaluate` (8 decimals), radii and exponent are the neighbour's. Sections: croup 8, 6, 6, 5 → 7, 6, 5, 5; loin 7, 5, 4, 3 → 6, 5, 4, 3. `lumbar` and `thorax` are not turned by any named pose, so the junction holds in all 15 poses.
- The drawings are NOT touched. A first trial (written 14:15, replaced 15:19) carried the new sections into the drawn lofts `croup`/`pelvic-barrel` and `abdominal-barrel` (24 shapes): it passed every proof (252 cells ok), but the sheet of 1:50 showed the shared section as a seam in the line view, a line across the rump in the plan and two ticks on the back line of the side elevation. A drawn loft that ends exactly where its neighbour starts draws its cap; the run of one station into the neighbour hides it. The final file keeps the drawn lofts as they were: `representations` and `anchors` equal to the backup (`../MV-3/same.py`), the six sheets (1:50, 1:100, 1:200, 1:500 `standing-alert`; 1:50 and 1:500 `browsing-upright`) byte-identical before and after.
- Guard before → now (IoU / largest distance; the solids lose the doubled wall, the drawings stay): 1:50 unchanged (0.9966/5.6, 0.9931/3.8, 0.9950/6.8 mm); 1:100 elevation 0.9930 → 0.9928, plan 0.9938 → 0.9934; 1:200 plan 0.9885 → 0.9880; 1:500 elevation 0.9849 → 0.9847, front 0.9733 → 0.9732, plan 0.9930 → 0.9922; distances and boxes unchanged.
- The 1:50 body decision says in English and German that croup, loin and chest meet in one shared section each.

### `red-squirrel`, guard locks (`W/squirrel.ts`, `W/brush-locks.json`)

- Cause: `T14/rework-red-squirrel-coarse-tail.py` (2026-11-22 17:15) took the single hair locks out of 1:100, 1:200 and 1:500 as "7 mm wide and far below the feature". They are `texture` lofts, which `volumes-feature` does not judge, and the pin counts them per scale.
- Cure: the 28, 18 and 12 locks of the state of 2026-11-22 13:42 (commit `01324745f80`) are back in the body list, each after the tail link of its bone, with its `tail-fur` surface in the thermal list (`th-brush-lock-<n>`). Checked before use: the six tail links and the rig of the tail are unchanged since that state, the 36 locks of 1:50 are unchanged, and every station of every coarse lock is a station of its 1:50 lock. Body 35, 35, 35 → 63, 53, 47 solids; thermal 37, 37, 37 → 65, 55, 49. The body decision of each scale states the new count and "six links with a few hair locks that carry the fringe of its outline" (English and German, the wording the removing script itself held for a scale that keeps locks).
- The drawings are unchanged (only `volumes` differs from the backup). Guard identical before and after (1:50 0.9794/2.4, 0.9530/5.2, 0.9215/9.9 mm; 1:100 0.9726/5.4, 0.9411/10.1, 0.9218/8.4 mm). Shaded body views rendered for 1:100, 1:200 and 1:500 (`W/looks.sh`), those of 1:100 and 1:500 opened: the locks sit on the surface of the brush along its edge, none floats.

### Sources of the 3D ticket

`T14/volumes/animals/goat.json` and `T14/volumes/animals/red-squirrel.json` equalled `asset.volumes` of the catalog before this lane (the chain of that ticket refuses or overwrites a catalog whose volumes differ). `W/sync-source.py` wrote the new volumes into both (second run: unchanged), so that chain reproduces the repair instead of undoing it. The scripts `rework-goat-trunk.py` (overlapping pick tables) and `rework-red-squirrel-coarse-tail.py` (removes the locks) of that ticket were not edited: running them again would bring the two defects back.

## 3. Open: `goat` horn at 1:500 (1 red case, not curable in the asset)

Measured with `W/pins.ts` (a soft copy of the case, every step counted); the full answer is in `requests.md` under `## closing — goat horn-n at 1:500 …`.

- The case was written on 2026-11-21 against the allowance "a coarser solid may keep the exact dimensions of its 1:50 counterpart" in `featureRule` (`physicalSources`, `retainedStations`). That allowance was removed on purpose on 2026-11-22 (`T14/validator.md`, item 3 and line 70: "`volumes-feature` judges every scale on its own"); the horn was thickened to 13.8 mm the same day and the case has been red since.
- On the present rule `transform.translation` and `material` do not enter the judgement of a body solid, so "0 findings as authored" and "1 finding after a translation of 10 mm" cannot both hold.

| Horn at 1:500 (limit of bone `head`: 26.42 mm, the thinnest drawing) | stations that are 1:50 stations | as authored (case: 0) | station 1 thinned (1) | translated (1) | other material (1) |
| --- | --- | --- | --- | --- | --- |
| catalog (13.8 mm constant, smallest extent 27.6 mm) | 2 of 5 | 0 | 0 red | 0 red | 0 red |
| measured stations 0, 3, 6, 11, 15 (`goat.ts --horn measured`, 23.0 mm) | 5 of 5 | 1 red | 1 | 1 | 1 |

The measured horn is a new finding of `volumes-feature` ("23.0 mm thin, below 26.4 mm", both horns), so it was not written. The owner of the rule and of the case decides: the allowance returns for solids that keep 1:50 stations (then `goat.ts --horn measured` writes the horn the case wants), or the case goes with the allowance.

## 4. Proofs of the files in the catalog

| Proof | `goat` `5a0aec3d2f9d90c1` | `red-squirrel` `5929104548bbe3fd` |
| --- | --- | --- |
| Light proof on the scratch copy BEFORE the write (gate over every subject, gate over the poses, guard, `validateAsset`, sweep, check CLI `--file`) | PASS 14:58 (first trial: PASS 14:04) | PASS 14:09 |
| Tool twice on the catalog file | second run leaves the digest | second run leaves the digest |
| Catalog file against the trial copy | equal (bytes) | equal (bytes) |
| Pins (`W/pins.ts`) | junction 0 failures of 240; horn as in section 3 | brush 0 failures |
| Fixer's gate `M/gate.ts --soft` (poses, clip instants, transitions; 164 subjects per scale for the goat) | exit 0, hard findings: 0 | exit 0, hard findings: 0 |
| Authors' scan `T/🔎️scan.ts` | 1 clean, 0 hard findings | 1 clean, 0 hard findings |
| Prints (`../MC/prints.ts`) | every authored scale prints | every authored scale prints |
| Silhouette guard, `validateAsset`, sweep (`M/measure.ts`) | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings |
| Check CLI `--asset` | `✓`, 4 suitability lines | `✓`, 2 suitability lines |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>`, to its end | 252 cells: 252 ok, 0 fail (1543 s; first trial 252 ok, 1084 s) | 210 cells: 168 reported group drift, 42 unavailable, 0 fail (223 s; as lane MU recorded before) |
| `🧊️catalog-silhouette` | 1 passed, 384 skipped | 1 passed, 384 skipped |
| `🧊️three-views` | 2 passed, 773 skipped | 2 passed, 773 skipped |
| `🌡️catalog-physics` | 3 passed, 837 skipped | 3 passed, 837 skipped |
| `✅️asset-validation` | 10 passed, 75 skipped | 10 passed, 75 skipped |
| `🫁️breathing-catalog`, `⚡️asset-power` | no test of this name (290 and 24 skipped) | same |
| `🧊️model-skeleton`, the red case by name | "doe torso junction": 1 passed, 26 skipped | "tapered caudal chain": 1 passed, 26 skipped |
| `🧊️model-silhouette`, "retains the measured horn stations" | 1 failed, 23 skipped (`expected [] to have a length of 1`, the step after thinning) | — |

Not run: the other cases of `🧊️model-skeleton` and `🧊️model-silhouette` (not of these two assets), the full suites and the full matrix (lane MZ; the lines `CL-A rewrote …` are in `requests.md` under the closing section of the nine assets, with the digests: `goat` 14:15 first trial, 15:19 final, 15:32 the proof run with the same bytes; `red-squirrel` 14:37).

## 5. Queue

Every gate, guard, check, matrix and suite ran through `repo capacity run` (`W/q.sh`), at most two requests at a time. Peaks (`/usr/bin/time -l`, largest process): light proof and full proof with the matrix of an animal 426 to 467 MB (800 MB reserved); the six suites with the pinned cases 2496 MB and 2893 MB (reserve 3300 MB, not 2500). The JSON tools and `pins.ts` (86 MB, seconds) also ran outside a grant while the queue admitted nothing. From 13:45 to 14:04 the queue admitted nothing because two grants reserved 8600 MB and held under 1 GB; note in `requests.md` (`## closing CL-A — the memory queue admits nothing …`).

## 6. Files

Updated in the catalog: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json` (`volumes` only), `…/🐕️animals/red-squirrel/🔣️.json` (`volumes` only).

Updated in the 3D ticket: `T14/volumes/animals/goat.json`, `T14/volumes/animals/red-squirrel.json` (section 2).

Updated: `requests.md` (answer under the horn section, the rewrite lines, one queue note).

Created in `U/🧰️matrix/CL-A/`: `goat.ts`, `squirrel.ts`, `brush-locks.json` (the repair, idempotent; chain of an asset: its earlier chain, then `tool.sh <id> <file> <file>`), `pins.ts` (the three pins without the test runner), `tool.sh`, `stage1.sh`, `light.sh`, `stage2.sh`, `prove.sh`, `tell.py`, `stage3.sh`, `suites.sh`, `sheets.sh`, `looks.sh`, `sync-source.py`, `q.sh`, `env.sh`, `backup-sha256.txt` (digests of the two files before the lane; the backups went with `GEN`). Report: `closing-CL-A.md`.

Removed: `U/🗑️generated/matrix/CL-A`.
