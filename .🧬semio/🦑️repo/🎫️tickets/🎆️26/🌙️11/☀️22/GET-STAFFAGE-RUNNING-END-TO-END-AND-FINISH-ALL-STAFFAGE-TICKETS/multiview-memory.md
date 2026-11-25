IN WORK

# 🧠️ Multiview Matrix: Memory, Speed and the Final Run (lane MM, updated 2026-11-25 20:03)

Paths: `R` = repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, scripts `U/🧰️matrix/MM/`, tool output `U/🗑️generated/matrix/MM/`. No catalog asset, fixture or allowlist was touched. Evidence rule: a figure is "measured" only when it comes from a log that ran to its end; everything else says "not run" or "claimed".

## 1. What stands (measured before 17:48, logs in `🗑️generated/matrix/MM`)

- `T/📊️matrix.ts`, `T/🗃️cells.ts`, `T/🧰️.ts`: no path loads the catalog; the generator keeps one head per asset and merges asset by asset (merge over 375 assets without records: 500 MB, 24 s, `f-heads.log`).
- Check CLI with `--asset`/`--category` and `T/🖨️proofs.ts` read one document at a time (check CLI `european-beech`: 757 MB, was 3,679 MB).
- Engine speed-up (`🧰️matrix/MM/engine.patch`, landed by MX-2 at 17:48): cells byte-identical on `finned-heat-exchanger`, `boxwood-ball`, `privet-hedge`, `jogger` (209,397 bytes, SHA-256 `e5f8e89a266d85b0`); `jogger` 3014 s to 172 s, peak 320 MB (`n3/jogger.log`).
- The older interrupt-and-resume run (`f-resume.log`, 10:04, old engine) cut after one record, exit 130, resumed and merged: 504 cells, 0 fail.

## 2. Measured since the takeover (20:03)

| Item | State |
| --- | --- |
| `saguaro-cactus`, `📊️matrix.ts` on the landed engine, request of 800 MB (`g-saguaro.log`, ended 20:33) | **measured**: done in 5,292 s wall (2,171 processor s) on the loaded host, peak 466 MB (`time -l` 488,488,960 bytes; the `peak 466 MB` of the progress line equals it, so the unit of `maxRSS` is verified), 252 of 252 cells ok. Old engine: 2.3 processor hours, cut. The document was edited by a fixer at 19:09 during the run (digest `967740d3` now, `1b1afabf` run), so the record is stale for the merge; time and memory stand |
| `atlas-cedar` and `olive-tree`, identity against the older records (`MU/matrix-trees/cells/atlas-cedar.json`, `MV-2/matrix-olive-tree/cells/olive-tree.json`, both with the digest of the document now in the catalog) and peak (`/usr/bin/time -l`) | queued at 19:03 (`g-trees.log`, 2500 MB), waiting 3,500 s at 20:03: **not run** |
| Interrupt and resume of the generator on the landed engine (`resume-proof.py`, `g-resume.log`, 800 MB request, 20:33, all 14 claims ok) | **measured**: SIGINT after the first record exits 130, leaves one record and no merge; SIGKILL of the generator while `privet-hedge` runs: the orphaned asset process ends by itself (next scale entry) and leaves no record; the restart skips the finished asset and exits 0; the cells of both assets equal a straight run byte for byte and the cells of `matrix.json` have the same SHA-256 (`117921d74ba7dbe2`). Peaks printed: `finned-heat-exchanger` 263 MB, `privet-hedge` 453 MB. Cells of `finned-heat-exchanger` also identical to the record of 09:52 (old engine) and to `n3/finned.json` (52,561 bytes, SHA-256 `f4f59d7a734b5337`) |
| Records of the other fixers since 17:48 (`harvest.py`, 111 assets) | seconds per category in section 3; they carry no peaks |

## 3. Seconds on the landed engine (wall time on the loaded host, load 20 to 30 on 10 cores)

Median and longest: people 353 s (`sleeping-person` 2,648 s, `person-pushing-bicycle` 2,178 s), trees and shrubs 361 s (`banana-plant` 1,855 s), machinery `excavator` 902 s, furniture 27 s (114 s), components 20 s (84 s), appliances 9 s (72 s), `red-squirrel` 392 s. `harvest.tsv` columns: asset, category, document bytes, seconds, record bytes, time, digest state, folder.

## 4. Reservations (written into `📋️takeover-brief.md`, "Reservation figures of MM", 20:05)

800 MB person, animal or rigid asset; 1000 MB tree below 1.5 MB document; 1600 MB tree of 1.5 to 6 MB; 2500 MB tree above 6 MB (until measured); check CLI 1000 MB; suite 3600 MB. `📊️matrix.ts` now prints `peak <n> MB` per ended asset, so the final run measures every asset.

## 5. The final run over all 375 assets (plan, not started)

Forecast (`forecast.py`: harvested assets by their measured seconds, the others by the processor seconds of the old engine, `f-estimate.jsonl`, times the ratio of their category; objects and vehicles have no measured asset yet and use the overall ratio): **26.6 single-process hours** on the loaded host; humans 14.3 h (of which `cyclist` forecast 300 min, `dancer` 57 min, the largest uncertainty: not measured on the landed engine), vegetation 9.2 h (`saguaro-cactus` measured: 88 min wall, 36 processor min), all rigid categories 3.2 h together. With 3 processes at a time 8.9 h, with 4 6.6 h, with 6 or more 5.0 h (the longest asset bounds it). A quiet host is faster by up to the load factor (up to 2.5x), so the expected wall time is **2.5 to 5 hours with 6 to 8 processes**.

Script `U/🧰️matrix/MM/full.sh` (not run): `STAFFAGE_LANGUAGE=en sh full.sh <run folder> [--lanes 3] [--retry-failed] [--minutes 30] [--assets a,b]`.

- `plan.ts` (800 MB request) reads the catalog one document at a time, leaves out assets with a current record and writes one line per request, heaviest class first: big tree (2600 MB, 1 at a time), mid tree (1600 MB per process, 2), small tree (1000 MB, 3), people and animals (600 MB, 3), rigid (350 MB, 4); a request holds at most `--minutes` of estimated work.
- `--lanes` workers (default 3) take the lines one by one, each line is one `repo capacity run` request of `📊️matrix.ts --shard 0/1 --progress <own file>`; at most `--lanes` requests of the run are in the queue and up to 9 processes run (3 lanes x 3 processes) on a quiet host; the log of every request is `<run folder>/lanes/<round>-<line>.log`.
- Resume: the rerun skips every asset whose record is current (version, id and digest of the document); a cut loses the assets that were running. Rounds repeat until nothing is left or a round finishes no asset; `--retry-failed` reruns named failures in the first round. The merge is a request of its own (700 MB) and writes `matrix.json` and `summary.json`.
- Proof sheets: `U/🧰️matrix/MM/proofs.sh <sheet folder>` runs `T/🖨️proofs.ts --categories <c>` once per category (9 requests, 1500 MB each, resumable per category) after the matrix, on the frozen catalog; memory and seconds per category are **not measured yet**.
- Rules for the day: start only when every batch is FINAL; do not start a second copy on the run folder; with other fixers gone the host has about 14 GB above its floor.

## 6. Code changed by MM since the interim report

`T/📊️matrix.ts`: peak per asset in the progress line, `--progress <file>`, an asset process ends at its next scale entry when its generator vanished (test: `resume-proof.py`, 14 of 14 claims ok). The unit of `maxRSS` is verified against `time -l` (466 MB both). New in `🧰️matrix/MM`: `resume-proof.py`, `timed.sh`, `peak.sh`, `harvest.py`, `forecast.py`, `proofs.sh`, `plan.ts` (new classes), `full.sh` (lanes).

## 7. Still to do before FINAL

Peaks and identity of `atlas-cedar` and `olive-tree` (queued since 19:03); `cyclist`, `dancer`, `small-leaved-lime` seconds and peaks; proof-sheet timing; dry run of `plan.ts`; check of the type contract of the `📊️matrix.ts` edits.
