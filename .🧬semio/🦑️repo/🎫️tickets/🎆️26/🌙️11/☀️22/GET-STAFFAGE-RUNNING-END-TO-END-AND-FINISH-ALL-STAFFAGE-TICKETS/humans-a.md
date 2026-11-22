# 🧍 Finisher Report: Human Collection Wave, Workers HW-01 … HW-09

Finisher A of `☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (`K`), 2026-11-22 20:23 to 22:15. Scope: the 27 people assigned to HW-01 … HW-09. 23 were started and are finished; 4 were never started. Nothing was merged into the manifest.

## Result

- Every started person: `validateAsset` 0 issues, staffage check CLI `✓` with zero issues (9 samples per clip), default-pose sheets at 1:50, 1:100, 1:200 and 1:500 opened by the finisher, generator kept in `K/🧰️scripts/<worker>/`, row in `K/📥️manifest-rows/<id>.json` (`validateManifest` on a manifest copy with the row: 0 issues each), worker report `K/📝️HW-0x.md`.
- Every generator was run again in this session (`K/🧰️scripts/FIN-A/reproduce.sh`); the shipped catalog file of each of the 23 people is the output of its generator chain.
- Never started (no generator, no catalog file, not started now): `hotel-receptionist` (HW-02), `carer-pushing-wheelchair` (HW-06), `yoga-practitioner` (HW-07), `inline-skater` (HW-08).

## What stays imperfect

- **Silhouette guard (`🧪️tests/🧊️catalog-silhouette`)**: bounding box within one feature for all 23 and every scale. IoU below 0.92 in 17 views of 7 people (walking-toddler 5, person-stretching 5, schoolchild-with-backpack 2, e-scooter-rider 2, barista 1, tennis-player 1, person-pushing-bicycle 1; lowest 0.857, toddler top view 1:500). The largest boundary distance exceeds two smallest features in at least one view for 18 of 23 people; only standing-desk-worker, office-worker-carrying-laptop, doctor-with-clipboard, patient-with-iv-pole and hiker-with-trekking-poles meet it everywhere, and the test fails on the established base figure `worker-carrying-box` too (107 mm against 15 mm, measured with the test itself; section in `requests.md`). The full `catalog-silhouette` suite will therefore be red for these people. I did not replace outlines by traces: measured on `presenter-at-flipchart`, the worker's own trace repair meets the distance at 1:50 and raises 6 `lateral-ambiguous` validator issues.
- **`checkVolumes` (all rules, `HW-03/measure.ts`)**: 0 findings for 9 people, 1 to 14 findings for 14 (mostly `volumes-connected`, `volumes-thermal-coverage`, `volumes-structure-coverage`; counts per person below). Neither the validator nor the check CLI runs these rules; they are measured and listed in the worker reports, not repaired.
- **Design limits stated in the worker reports**: the barista is 1.70 m tall because a scale below 1 pushes base solids under the smallest feature; skin and hair tones of the HW-02 people come from the base figure; `doctor-with-clipboard` and `patient-with-iv-pole` have masculine German titles; `teenager-with-headphones` names its breathing idle `idle-scrolling`; stiff inverse-kinematics arms in `driving-blade` of the shovel worker.
- **Sheets**: I opened one joined sheet per person (four scales, side, front and top view of the default pose) at reduced size, plus full-size 1:50 sheets of shovel worker (two poses), jackhammer worker (before the fix), waiter and barista. Other poses, clip instants and the thermal and structural views were evaluated by the check CLI but not looked at.

## State before and after, with the final proof lines

Proof columns: validator issues; check CLI; silhouette guard (bounding box misses / views with IoU below 0.92 / views with boundary distance above two features, of 12); `checkVolumes` findings.

| Person | Worker | Found at 20:23 | Done by the finisher | Validator | Check CLI | Guard bbox / IoU / distance | checkVolumes |
|---|---|---|---|---|---|---|---|
| `standing-desk-worker` | HW-01 | catalog file, row, report; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ standing-desk-worker (1/1)` | 0 / 0 / 0 | 10 |
| `presenter-at-flipchart` | HW-01 | catalog file, row, report; valid, check ✓; unapplied silhouette repair ops of 18:10 | unchanged file; repair ops rejected (they raise 6 `lateral-ambiguous`), kept in `ops-rejected/`; row written; report | 0 | `✓ presenter-at-flipchart (1/1)` | 0 / 0 / 5 | 14 |
| `office-worker-carrying-laptop` | HW-01 | catalog file, row, report; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ office-worker-carrying-laptop (1/1)` | 0 / 0 / 0 | 2 |
| `waiter-with-tray` | HW-02 | catalog file, no row; valid, check ✓, tones inverted | tones corrected in the generator, regenerated; row written; report | 0 | `✓ waiter-with-tray (1/1)` | 0 / 0 / 5 | 0 |
| `barista` | HW-02 | half a generator, no catalog file | generator completed, asset authored at 1.70 m, written; row written; report | 0 | `✓ barista (1/1)` | 0 / 1 / 3 | 7 |
| `nurse-walking` | HW-03 | generator, no catalog file; its output 12 validator issues | hatch fixed, dimensions fitted, written; row written; report | 0 | `✓ nurse-walking (1/1)` | 0 / 0 / 2 | 5 |
| `doctor-with-clipboard` | HW-03 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ doctor-with-clipboard (1/1)` | 0 / 0 / 0 | 0 |
| `patient-with-iv-pole` | HW-03 | generator, no catalog file; its output 2 validator issues | 1:500 stand base fixed, written; row written; report | 0 | `✓ patient-with-iv-pole (1/1)` | 0 / 0 / 0 | 0 |
| `construction-worker-with-shovel` | HW-04 | generator, no catalog file; its output 5 validator issues, 2 print findings | shaft, resting pose, plan paint order, dimensions fixed, written; row written; report | 0 | `✓ construction-worker-with-shovel (1/1)` | 0 / 0 / 4 | 8 |
| `worker-with-jackhammer` | HW-04 | generator, no catalog file; its output 7 validator issues, breaker drawn apart (IoU 0.71) | rest placement of the breaker fixed (IoU 0.95 to 0.99), written; row written; report | 0 | `✓ worker-with-jackhammer (1/1)` | 0 / 0 / 4 | 8 |
| `surveyor-with-total-station` | HW-04 | generator, no catalog file; its output valid, check ✓ | written; row written; report | 0 | `✓ surveyor-with-total-station (1/1)` | 0 / 0 / 6 | 12 |
| `walking-toddler` | HW-05 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ walking-toddler (1/1)` | 0 / 5 / 1 | 6 |
| `schoolchild-with-backpack` | HW-05 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ schoolchild-with-backpack (1/1)` | 0 / 2 / 2 | 7 |
| `teenager-with-headphones` | HW-05 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ teenager-with-headphones (1/1)` | 0 / 0 / 3 | 0 |
| `elderly-person-knitting` | HW-06 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ elderly-person-knitting (1/1)` | 0 / 0 / 2 | 0 |
| `mobility-scooter-user` | HW-06 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ mobility-scooter-user (1/1)` | 0 / 0 / 2 | 0 |
| `tennis-player` | HW-07 | catalog file and row; valid, check ✓; generator newer than the file | regenerated from the current generator (arm and clavicle angles of 5 poses); row written; report | 0 | `✓ tennis-player (1/1)` | 0 / 1 / 3 | 0 |
| `basketball-player` | HW-07 | catalog file and row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ basketball-player (1/1)` | 0 / 0 / 6 | 2 |
| `hiker-with-trekking-poles` | HW-08 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ hiker-with-trekking-poles (1/1)` | 0 / 0 / 0 | 4 |
| `person-stretching` | HW-08 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ person-stretching (1/1)` | 0 / 5 / 4 | 1 |
| `e-scooter-rider` | HW-09 | catalog file, no row; valid, check ✓ | generator run again (identical file), measured; row written; report | 0 | `✓ e-scooter-rider (1/1)` | 0 / 2 / 5 | 0 |
| `traveller-with-rolling-suitcase` | HW-09 | catalog file, no row; valid, check ✓ | depth 0.712 to 0.723 m (fitted to the volume guard); row written; report | 0 | `✓ traveller-with-rolling-suitcase (1/1)` | 0 / 0 / 6 | 0 |
| `person-pushing-bicycle` | HW-09 | catalog file, no row; valid, check ✗ (3 print findings, 2 failing clip instants) | fork pen, chainstay, swing toe fixed in the generators, rebuilt; row written; report | 0 | `✓ person-pushing-bicycle (1/1)` | 0 / 1 / 8 | 13 |

Per-scale print lines of the check CLI (min stroke, gap, feature) and the per-view guard numbers of every person are in the worker reports `K/📝️HW-0x.md` (section "Proof measured by the finisher") and in `K/🧰️scripts/FIN-A/proof.json`.

## Decisions taken without asking

- The proof bar is the one of the order (validator 0, check CLI ✓); the silhouette guard and `checkVolumes` were measured for all 23 and only real defects found through them were repaired (jackhammer breaker placement, bounding boxes through fitted dimensions, barista sleeves and hands).
- `dimensions` of barista, nurse, the three HW-04 people and the traveller are fitted to the volume guard and the drawing guard of every scale at once (`FIN-A/fit_dims.py`) instead of copied from the 1:50 volume.
- `tone` is lightness: the HW-02 generators stated darkness; both people were re-toned.
- `tennis-player` was regenerated from its current generator although the older file was valid, so that generator and catalog agree.
- The barista was finished from the half generator (started counts), at 1.70 m instead of 1.63 m.

## Shared-code requests (open, in `requests.md` of this ticket)

- `STAFFAGE-HUMAN-COLLECTION-WAVE — catalog-silhouette: the Hausdorff gate fails for an established human, so the people derived from it cannot pass it`.
- `STAFFAGE-HUMAN-COLLECTION-WAVE — unionPolygons: second measurement of the edge-on prism failure (humans/person-pushing-bicycle)`.
- Observed, not filed as a section: `vehicles/city-bicycle` is `✗` in the check CLI today (`volumes-feature` of axles, nuts, rims at 1:100 and 1:200); it is outside this ticket.

## Left for the umbrella coordinator

- Merge the 23 rows of this scope (and finisher B's) into `🗂️catalog/🔣️.json`, collections, full test suites, site build, closing the ticket.

## Files

Created (catalog, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json`): `barista`, `nurse-walking`, `patient-with-iv-pole`, `construction-worker-with-shovel`, `worker-with-jackhammer`, `surveyor-with-total-station`.

Updated (catalog, same path pattern): `waiter-with-tray`, `tennis-player`, `traveller-with-rolling-suitcase`, `person-pushing-bicycle`. Rewritten by their generators with identical content: the other 13 people of the table.

Ticket `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE`:

- Created: `K/📥️manifest-rows/<id>.json` for the 18 people without a row; `K/📝️HW-02.md` … `K/📝️HW-09.md`; `K/🧰️scripts/FIN-A/` (`env.sh`, `measure.sh`, `gate.sh`, `sheet.sh`, `proof.sh`, `reproduce.sh`, `montage.ts`, `feature.ts`, `union.ts`, `volbox.ts`, `extents.py`, `fit_dims.py`, `summary.py`, `build_proof.py`, `report.py`, `distinctions.json`, `notes.json`, `proof.json`).
- Updated: `K/📥️manifest-rows/` of `standing-desk-worker`, `presenter-at-flipchart`, `office-worker-carrying-laptop`, `tennis-player`, `basketball-player` (rewritten from the evaluated assets, same values); `K/📝️HW-01.md` (tests and previews sections); `K/🗂️work-board.md` (rows HW-01 … HW-09 only); `K/🧰️scripts/HW-02/barista.py`, `waiter.py`; `K/🧰️scripts/HW-03/nurse.ts`, `patient.ts`; `K/🧰️scripts/HW-04/lib.py`, `shovel.py`, `jackhammer.py`, `texts_shovel.py`; `K/🧰️scripts/HW-09/bicycle_fixes.py`, `bicycle_texts.py`, `poses_person_pushing_bicycle.ts`.
- Moved: `K/🧰️scripts/HW-01/ops/presenter-at-flipchart-{50,100,200,500}.json` to `K/🧰️scripts/HW-01/ops-rejected/`.
- Removed: `K/🗑️generated/` subfolders `FIN-A`, `HW-01`, `HW-02`, `HW-04` (drafts, logs, sheets, backups of this session).

Umbrella ticket: created `humans-a.md` (this file); updated `requests.md` (two sections appended).
