IN WORK (not final). Fourth fixer of the batch (takeover 2026-11-25 19:00). Clean with all proofs so far: 2 of 13 (`domestic-cat`, `mallard-duck`).

# 🐾️ Multiview Fixer MA: the 13 Animals

Batch `🧰️matrix/batches/MA.tsv`. Paths as in `📋️multiview-brief.md` (`R`, `S`, `T`, `U`); scripts in `U/🧰️matrix/MA/`, base tools of `U/🧰️matrix/MR/` (read only); logs in `U/🗑️generated/matrix/MA/` (`$GEN`) until the FINAL report.

## 1. Method

The chain of an asset is ONE step after its generators: `loft.ts <id> --ops loft/<id>.json --write` (third fixer, adopted). A part that a pose turns becomes a `loft` shape (D54, README "Loft shapes") in the front and the plan, with the v range of the drawn side elevation and the lateral range of the body solids of the bone at every station; the side elevation keeps its drawn outline unless the part leaves the sagittal plane (`"side": true`: wings, ears). A second run changes nothing. The earlier chains (`fit.ts` with `ops/`, `nest.ts` with `nest/`: single solids and nested prisms) are superseded: they left 4 to 1832 hard findings per asset in the gate over clips and transitions (logs `$GEN/guard/<id>.gate.log` of 11:38 to 14:55).

Helpers added in this takeover: `left.py` (takes up parts the proposal left flat), `thin.py` with the op member `slim` of `loft.ts` (a drawn loft no thicker than the thinnest solid of its bone, rule `volumes-feature`), `lsettle.py` (keeps the loft ops the silhouette guard accepts), `lsum.sh` (state from the logs), `rebase.py` (see section 4), `lfinish.sh` (write and prove one asset in one grant of the queue).

Proof per asset (`lfinish.sh`, logs `$GEN/proof/<id>-*.log`, block `<id>-finish.txt`): apply twice, catalog file equals the proven trial copy, `T/🔎️scan.ts`, `MR/gate.ts --twins` (poses, clip instants, transitions), `MR/measure.ts` (silhouette guard, `validateAsset`, three-views sweep, before and after), check CLI, `MC/prints.ts`, `T/📊️matrix.ts --assets <id>` to its end. The six vitest suites with the name filter run in one 2500 MB grant (requested 19:40).

## 2. Assets

| Asset | State | Scan hard before → after | Gate hard (all subjects) | Guard, validate, sweep | Check CLI | Prints | Matrix |
|---|---|---|---|---|---|---|---|
| `domestic-cat` | CLEAN, written 19:35 | 323 → 0 | 0 | OK, 0, 0 (as before) | ✓ | every authored scale | 231 cells: 210 ok, 21 unavailable, 0 fail (ended 20:32) |
| `mallard-duck` | CLEAN, written 20:40 | 78 → 0 | 0 | OK, 0, 0 (as before) | ✓ | every authored scale | 231 cells: 210 ok, 21 unavailable, 0 fail (ended 21:11) |
| `pig` | catalog = hybrid chain of 12:07 (second fixer), to be replaced | 342 → 2 | 4 | OK, 0, 0 | ✓ | not run | 252 cells: 98 ok, 28 fail (`sync-bone-forward` ears 28 mm) |
| `sheep` | catalog = hybrid chain of 12:30, to be replaced | 344 → 0 | 4 | OK, 0, 0 | ✓ | not run | 252 cells: 77 ok, 28 fail |
| `hedgehog` | catalog = hybrid chain of 12:48, to be replaced | 47 → 1 | 14 | OK, 0, 0 | ✓ | not run | 210 cells: 56 ok, 84 fail |
| the other 8 | catalog unchanged; loft trial copies in work | dairy-cow 342, goat 115, herring-gull 204, horse 139, labrador-dog 569, pigeon 164, red-fox 798, roe-deer 1012 | trial copies: herring-gull 813, pigeon 688, roe-deer 23 (19:02 to 19:24), others not run | | | | not run |

`domestic-cat`: 73 loft ops (`loft/domestic-cat.json`), all applied; sheets of 1:50, 1:100, 1:200 in `standing-alert`, `sitting-tail-wrapped`, `stretch-deep` opened before and after: side unchanged, front and plan are the same outlines with smooth sections (the sitting front lost its lumps). Vitest suites: pending (lane `jobs-v`).

`mallard-duck`: 67 loft ops (10 parts the proposal left flat taken up with `left.py`: feet and scapulars; `slim` 9.9 mm on the two shanks at 1:100, the thinness of `joint-shank-*`); sheets of 1:50 and 1:200 in `standing` and `upending` opened before and after: side and front as before, the plan wings are smooth bodies instead of stacked rectangles.

## 3. Files

Updated: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/domestic-cat/🔣️.json` (and `pig`, `sheep`, `hedgehog` by the second fixer). Created or updated in `U/🧰️matrix/MA/`: `loft.ts`, `loft/*.json`, `lcycle.sh`, `lfinish.sh`, `left.py`, `thin.py`, `edit.py`, `lsettle.py`, `lsettle.sh`, `lsum.sh`, `rebase.py`, `worker.sh`, `job.sh`, `where.ts`, `shift.ts` and the earlier tools (`fit.ts`, `nest.ts`, `ops/`, `nest/`, `turns/`, stage scripts).

## 4. Notes

- 19:09: another worker rewrote `asset.tags` of every catalog document (strings became `{ id, title }`). `rebase.py` spliced that member byte for byte into the backup and the trial copies, so a write of this batch does not undo it; the backup of 2026-11-24 stays in `$GEN/backup-1124`.
- The matrix of an animal takes about 45 minutes on the loaded host (1:50 entry alone 20 minutes), peak below 350 MB.

## 5. Requests

None filed yet.
