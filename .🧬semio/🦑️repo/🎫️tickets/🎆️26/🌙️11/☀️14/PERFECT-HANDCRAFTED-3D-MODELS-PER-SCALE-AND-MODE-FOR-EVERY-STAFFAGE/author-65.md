# Author 65 — slender parts at true size (INCOMPLETE)

Status: tooling built and one asset trialled; no source under `volumes/` was changed and nothing was re-spliced. The full list of about 55 assets is open.

Tools (this folder): `author-65-scan.ts` (scan, `bun author-65-scan.ts [asset]`), `author-65-fix.py` (restore 1:50 geometry plus `slender`), `author-65-run.py` (product check before/after, writes only changes that make no measurement worse, scale by scale; skips sources modified in the last 20 minutes). Scan output `🗑️generated/author-65/scan2.json`.

## playground-swing (trial, nothing written)
| scale | part | before | restored | result |
|---|---|---|---|---|
| 1:200 | chains | 70 mm | 11 mm | elevation IoU 0.869 -> 0.823 |
| 1:200 | braces | 26 mm | 40 mm | with chains 0.801 |
| 1:500 | legs, braces, chains | 90 / 90 / 150 mm | 60 / 40 / 11 mm | front 0.965 -> 0.573, plan 0.813 -> 0.776, elevation 0.982 -> 0.889 |
| 1:100 | legs flagged `slender` | - | - | elevation 0.972 -> 0.873 |
Blocked by the validator (request section 65 in `tool-requests.md`), so the asset stays as it was.

## Named cases
- office-swivel-chair column (ratio 9), bar-stool ring (closed), bicycle-stand hoop (curved): cannot be `slender` today; request filed, models unchanged.
- pendant-lamp cord, stepladder rails: not processed. vacuum-cleaner: left to author 57.
- Not done: the remaining scan list, render sheets, splices.
