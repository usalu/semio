# 🧍️ Multiview Fixer MH-5: Twelve People

Fixer report, batch `🧰️matrix/batches/MH-5.tsv`, begun 2026-11-24 and taken over on 2026-11-25 02:20 by a second fixer after the first stopped at 00:43. Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-5/`. **Status of this file: IN WORK, not final; section 0 is the state measured at the takeover, section 1 the state per asset from the logs of the latest runs.**

## 0. State at the takeover (measured 2026-11-25 02:22 to 02:35)

- Catalog files against the backup of 2026-11-24 13:39 and against the last copy this batch landed (`md5`): five are byte-identical to the backup (`runner-with-blade-prosthesis`, `seated-office-worker`, `seated-reader`, `skateboarder`, `sleeping-person`: never written), seven are byte-identical to the landed copy (`presenter-at-flipchart`, `schoolchild-with-backpack`, `seated-guitarist`, `standing-desk-worker`, `standing-phone-user`, `standing-violinist`, `surveyor-with-total-station`). No file was found half-written.
- Official scan of all twelve now (`T/🔎️scan.ts --language en --quiet`, one run, 02:24 to 02:34): 5 clean (`presenter-at-flipchart`, `seated-guitarist`, `standing-desk-worker`, `standing-phone-user`, `standing-violinist`), 7 with hard findings: `runner-with-blade-prosthesis` 165, `schoolchild-with-backpack` 2, `seated-office-worker` 25, `seated-reader` 50, `skateboarder` 227, `sleeping-person` 58, `surveyor-with-total-station` 2 (529 in all).
- Proofs that count (run to the end, outside 22:00 to 23:00 of 2026-11-24, on the file that is in the catalog now): `presenter-at-flipchart` (18:38 to 19:21, with matrix), `standing-violinist` (19:32 to 20:04, with matrix), `standing-desk-worker` (20:25 to 20:41, with matrix), `seated-guitarist` (00:06 to 00:09, without matrix), `surveyor-with-total-station` (00:24 to 00:26, gate 56 hard, without matrix). Proofs that do NOT count and are repeated: `standing-phone-user` (22:05, and the file was landed again at 23:02), `schoolchild-with-backpack` (22:24, landed again at 22:49), the matrix of `seated-guitarist` (in flight at 21:59, no result).
- Searches (`auto.py`): `runner-with-blade-prosthesis` and `seated-office-worker` ended at 00:42 with ops files that were never applied to the catalog; `skateboarder` was in flight at 00:43 (counts as not done); `seated-reader` and `sleeping-person` were never searched.


## 1. Result per asset

STATE-TABLE

## 2. What was wrong

Every hard finding of the twelve people is one shape whose three pictures are not three views of one solid (causes D, F and G of `multiview-diagnosis.md` 5.6), in three forms:

1. **Plates of the silhouette rounds.** Arms, hands, necks and heads of the finer bands are drawn as `extruded` shapes 0.8 to 20 mm thick whose outline is the silhouette of the part in the default pose, each view in another plane of the bone (elevation swept `w`, front swept `v` or `u`, plan swept `v`). In the default pose each view is right; in every other pose the three plates turn into three different pictures (`presenter-at-flipchart` 1:50 `forearm-l` in `writing`: elevation y [0.907, 1.194], front [1.031, 1.078], 125 mm).
2. **Hidden length.** A front or plan outline runs further into the neighbouring part than the elevation twin (front neck of `standing-violinist`: 56 mm, the defect `multiview-MR.md` found on `standing-adult`).
3. **Flat shapes on bones that turn** (the coarse bands: `sil-body`, shoes, forearms at 1:500).

The silhouette guard pins all three views of the default pose to twice the feature size (15 mm at 1:50) and most of these people stand exactly at that limit, so the plain remedy (one solid, copied into three views) is refused by the guard for a part whose default-pose silhouette is not the silhouette of a simple solid: `presenter-at-flipchart` 1:50, the revolved sleeve copied into the front gives a boundary distance of 27.3 mm (limit 15.0). Every repair therefore has to be measured against the guard, shape by shape.

## 3. Method and tools (`U/🧰️matrix/MH-5/`)

| Script | Purpose |
|---|---|
| `solid.ts` | The model: the body every drawn shape implies (the placement rules of `evaluate`: flat elevation plate, flat front or plan plate at the middle depth of its elevation twin, `extruded` sweeps u, v, w, `revolved` about the own bone) as a rigid point cloud in the bone frame, and the bone frames of every subject of the matrix plus the 49 probe instants of every clip and denser transitions, evaluated on a copy without shapes. |
| `mgate.ts` | The model gate: the hard tier and the twin tier of the matrix from the model, in 2 s per person instead of 3 to 15 min. Measured against the real gate on `presenter-at-flipchart` before the repair: the same bones and the same worst deltas to the millimetre (125, 62, 53, 48, 53, 64, 156 mm). The real gate stays the proof. |
| `fit.ts` | `../MR/fit.ts` plus the ops this batch needed: `sync` (flat shapes become prisms: a flat ELEVATION shape becomes its outline swept laterally, the op MR lacked; then the spans and profile ranges of the three shapes are fitted together against the extents the matrix compares over all subjects, while no shape may move in its own view in the default pose by more than `slack`), `clip` (cut hidden length to the range of the elevation twin), `scale-profile` (a body of revolution a few per cent slimmer), `copy-scale` (the geometry of the neighbouring band), `anchor-split` (D45: one anchor per physical contact, references in poses, clips and events followed), `measure`, and `--try` (many candidates in one process). |
| `sil.ts` | The silhouette guard of many variants of one asset in one process (the posed volume is built once per scale). |
| `auto.py` | The search: per shape the model gate names, a fixed list of candidates from the least to the most invasive; a candidate passes when the model disagreement is below 0.93 × tolerance and the guard of that scale is still `OK`; kept ops go to `ops/<id>.json`, as `set` ops with the fitted geometry where applying the op twice would move it again. Copies are only taken from a real solid, never from a plate thinner than 30 mm. |
| `freeze.py`, `trial.sh`, `variants.sh`, `anchors.sh` | Hand-picked candidates: trial on a copy with guard and anchor findings, kept as `set` ops. |
| `land.sh` | check on a copy (fit twice, model gate, `measure.ts` of backup and copy), write to the catalog (refused when the file changed on disk), then `../MR/prove.sh` with the matrix. Stops before writing when the copy has a hard pair, the guard fails or `validateAsset` or the three-views sweep differ from the backup. |
| `q.sh` | One heavy process of this fixer at a time (`lockf`). |
| `rows.py`, `scan-table.py`, `status.sh`, `wait.sh`, `sheets.sh`, `dump.ts`, `shapes.py`, `show.py`, `gate-all.sh`, `sil.sh`, `try.sh`, `auto-all.sh` | Report rows, summaries, sheets, inspection. |

The chain of an asset: its generator and silhouette ops (unchanged), then `bun fit.ts <id> --ops ops/<id>.json --write` from `T`. A second run reports `0 changed something` for every landed asset.

DETAIL-SECTIONS

## 5. Files

FILES

## 6. Open

OPEN
