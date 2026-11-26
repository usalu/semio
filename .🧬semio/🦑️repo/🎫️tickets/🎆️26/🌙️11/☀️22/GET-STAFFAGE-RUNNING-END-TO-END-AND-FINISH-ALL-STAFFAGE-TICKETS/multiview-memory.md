FINAL

# 🧠️ Multiview Matrix: Memory, Speed and the Plan for the Final Run (lane MM, FINAL 2026-11-26 01:00)

Paths: `R` = repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, scripts `U/🧰️matrix/MM/`. No catalog asset, fixture, allowlist, engine file or test was edited by this lane; the only code changed is `T/📊️matrix.ts` (section 4) and the lane's own scripts. Every figure below was read from a log that ran to its end; the logs of `U/🗑️generated/matrix/MM` were deleted with this report, the numbers stand here and in `📋️takeover-brief.md`.

## 1. Memory repair (before 17:48)

- `T/📊️matrix.ts`, `T/🗃️cells.ts`, `T/🧰️.ts`: no path loads the catalog; the generator keeps one head per asset and merges asset by asset (merge over 375 assets without records: 500 MB, 24 s).
- Check CLI with `--asset`/`--category` and `T/🖨️proofs.ts` read one document at a time (check CLI `european-beech`: 757 MB, was 3,679 MB).
- Engine speed-up (`🧰️matrix/MM/engine.patch`, landed by MX-2 at 17:48).

## 2. Identity of the cells on the landed engine (byte for byte)

| Asset (kind) | Against | Result |
| --- | --- | --- |
| `finned-heat-exchanger` (rigid) | record of 09:52 and the cells of 17:28 | identical, 52,561 bytes, SHA-256 `f4f59d7a734b5337` (also in the 20:33 run) |
| `boxwood-ball`, `privet-hedge` (shrubs) | base engine | identical, 46,051 bytes each |
| `jogger` (person) | record of 09:52 | identical, 209,397 bytes, `e5f8e89a266d85b0` |
| `atlas-cedar` (tree) | `MU/matrix-trees` record of 17:34 (same document digest `cf8c44eb`) | **identical**, 252 cells, 45,799 bytes, `dba025047006e1ce` |
| `olive-tree` (tree) | record of 10:04 (document edited since, so another digest) | **identical** cells, 252 |

Kinds covered: rigid, person, shrub, two trees. `saguaro-cactus`, `cyclist`, `dancer` and `small-leaved-lime` have no older record on the same document; they ran to their end (section 3), no identity claim.

## 3. Measured on the landed engine (`📊️matrix.ts --jobs 1`, peak by `/usr/bin/time -l` and by the new `peak <n> MB` of the progress line, which agree)

| Asset | Wall seconds (host at load 20 to 30) | Peak MB | Cells |
| --- | --- | --- | --- |
| `atlas-cedar` | 350 (376 on 17:34) | 975 | 252 ok |
| `olive-tree` | 254 (331 at 10:04) | 770 | 84 ok, 168 `group-drift` |
| `small-leaved-lime` (23 MB document, the largest) | 2,778 | 935 | 252 `group-drift` |
| `saguaro-cactus` | 5,292 (2,171 processor s; old engine 2.3 processor hours, cut) | 466 | 252 ok |
| `cyclist` | 4,090 (2,241 processor s) | 511 | 252 fail on the document of before 23:17 (`sync-bone-forward` 168, `sync-bone-height` 126, `anchor-landmark` 119, `sync-height` 56, `sync-bone-lateral` 42): a verdict on the old document |
| `dancer` | 1,580 | 329 | 14 ok, 238 `group-drift` |
| `privet-hedge` / `finned-heat-exchanger` / `jogger` | 40 to 65 / 17 / 172 | 587 / 263 / 320 | ok |

A tree's peak does not follow the size of its document (466 to 975 MB). `cyclist`, `saguaro-cactus` and `dancer` were edited by their owners while they ran (digest changed), so their records are stale for a merge; time and memory stand. 217 further records of the other fixers since 17:48 (`🧰️matrix/MM/harvest.tsv`): median seconds people 353, trees 361, furniture 27, components 20, appliances 9.

Proof sheets (`T/🖨️proofs.ts`, one category per process): `machinery` 16 assets 239 s, 898 MB; `furniture` 39 assets 224 s, 653 MB; `animals` 14 assets 168 s, 632 MB; four trees (`saguaro-cactus`, `atlas-cedar`, `olive-tree`, `small-leaved-lime`) 292 s, 857 MB. `humans` and the whole of `vegetation` were not run as sheets: expected about 15 to 30 minutes and 1.5 hours.

Type check of the package including the edited `📊️matrix.ts` (`tsc --noEmit -p`, 1,230 MB): exit 0, 0 errors.

## 4. The generator: interrupt, resume, kill (`resume-proof.py`, 14 of 14 claims ok at 20:33)

Changes in `T/📊️matrix.ts`: the progress line of an ended asset carries `peak <n> MB`; `--progress <file>` lets requests side by side in one run folder write their own progress; an asset process whose generator vanished (SIGKILL, crash) ends at its next scale entry and leaves no record. The test is language-agnostic and runs `finned-heat-exchanger` and `privet-hedge` straight and cut:

- SIGINT after the first record: exit 130, one record, no merge, no asset process left.
- Restart, then SIGKILL of the generator while `privet-hedge` runs: the orphaned asset process ends by itself, no record of it.
- Restart: the finished asset is skipped, exit 0, the cells of both assets equal the straight run byte for byte, the cells of `matrix.json` have the same SHA-256 (`117921d74ba7dbe2`).

## 5. Reservations (in `📋️takeover-brief.md`: "Reservation figures of MM" and "Vitest with a name filter")

800 MB matrix of a person, animal or rigid asset; **1300 MB any tree or shrub** (was 2500; highest peak 975 MB plus 30 %); check CLI 1000 MB; suite `🔭️multiview-coverage` 3600 MB; vitest with a name filter 1600 MB `🧊️catalog-silhouette`, 2500 MB `🧊️three-views`, 1950 MB `🫁️breathing-catalog`, 1900 MB `⚡️asset-power` and `asset-validation` (not measured, assumed), 700 MB `🌡️catalog-physics` (from the fixers' own `time -l` logs); proof sheets 1500 MB per category; `tsc` 1300 MB. Request note: `## multiview MM — vitest reservation …` in `requests.md`.

## 6. The final full matrix over all 375 assets (do NOT start before every batch is FINAL)

Command, from anywhere: `STAFFAGE_LANGUAGE=en sh U/🧰️matrix/MM/full.sh <run folder> --lanes 3` (`--retry-failed`, `--minutes <n>`, `--assets a,b` optional; German progress lines with `de`).

- **Plan** (`plan.ts`, 1000 MB request, 9.6 s, 699 MB; dry run on the 375 assets: 16 requests): trees 66 assets in 11 requests of two at a time (2700 MB each, per-asset limit 6 h), people and animals 89 assets in 3 requests of three at a time (1900 MB), rigid 219 assets in one request of four at a time (1500 MB) and one single asset (1400 MB). Heaviest class first, so the longest assets start first.
- **Lanes:** `--lanes` workers (default 3) take the plan lines one by one; each line is one `repo capacity run` of `📊️matrix.ts --shard 0/1 --progress <own file>`. At most three requests of the run are queued; up to nine processes run (about 6 GB reserved) on a quiet host. Request logs: `<run folder>/lanes/<round>-<line>.log`.
- **Resume:** the rerun skips every asset whose record is current (version, id, document digest); a cut (Ctrl-C, SIGTERM, reboot, killed lane, killed generator) loses only the assets that were running. Rounds repeat until nothing is left or a round finishes no asset; named failures (`asset-timeout`, `asset-crashed`) keep their record and are rerun by `--retry-failed`. The merge is a request of 700 MB and writes `matrix.json` and `summary.json`. A document edited during the run makes its record stale and the asset runs again in the next round.
- **Self-test:** `full.sh` was run on `finned-heat-exchanger` and `privet-hedge` in a scratch folder (`--lanes 1`): plan, two requests, round two found nothing left, merge: 504 cells, 483 ok, 21 unavailable, 0 fail, 2 done.
- **Forecast** (`forecast.py` with `harvest.tsv`, `estimate.jsonl`, `measured.tsv`; measured assets by their seconds, the others by the old-engine estimate times the ratio of their category): 26.8 single-process hours on the loaded host (people 4.5 h, trees 13.3 h, animals 5.0 h, all rigid categories 4.0 h). Wall time on that host: 8.9 h with 3 processes, 6.7 h with 4, 4.5 h with 6, 3.4 h with 8; the longest asset (`saguaro-cactus`, 88 min) bounds it. On a quiet host (load factor up to 2.5) **expect 2 to 4 hours for 6 to 9 processes**; humans' ratio rests on few measured assets and animals ran slower than estimated (`roe-deer` forecast 81 min), so treat the upper bound as likely.
- **Proof sheets afterwards** (frozen catalog, not beside the matrix): `sh U/🧰️matrix/MM/proofs.sh <sheet folder>`; nine requests (`humans animals vegetation furniture appliances objects vehicles machinery components`), 1500 MB each, resumable per category (a category with its five files is skipped), expected 1.5 to 3 hours in all.

## 7. Files (repo-relative to `U` unless marked)

Changed: `R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/📊️matrix.ts`; `📋️takeover-brief.md` (two reservation sections appended); `requests.md` (one note appended); this report. Created in `🧰️matrix/MM/`: `resume-proof.py`, `timed.sh`, `peak.sh`, `harvest.py`, `forecast.py`, `proofs.sh`, `chain-final.sh`, `chain-proofs.sh`, `harvest.tsv`, `measured.tsv`, `estimate.jsonl`; rewritten: `full.sh` (lanes), `plan.ts` (classes). Removed: `🗑️generated/matrix/MM`.

## 8. Open

- `asset-validation` has no measured peak (1900 MB assumed).
- The proof sheets of `humans` and the whole of `vegetation` have no timing; the full set was not run (it belongs to the frozen catalog).
- The `cyclist`, `dancer`, `saguaro-cactus` records are stale for the final merge; the final run recomputes them on the frozen documents.
- The `group-drift` cells (`olive-tree`, `small-leaved-lime`, `dancer`) and the `cyclist` findings belong to the allowlist owner and to the people's fixers, not to the generator.
