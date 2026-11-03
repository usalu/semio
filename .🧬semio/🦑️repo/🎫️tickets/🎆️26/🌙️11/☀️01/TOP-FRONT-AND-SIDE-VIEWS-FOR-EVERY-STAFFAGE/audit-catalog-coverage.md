# Staffage catalog coverage audit

Script: `🧰️AUDIT/coverage_audit.py`. Schema: `🧬️schema/🔣️.json` (semio.staffage/v2). Data: `🗂️catalog/<category>/<id>/🔣️.json`. Raw rows: `🗑️generated/AUDIT/coverage.json` (deleted after review).

## Totals

- Catalog JSON files read: 152 (asset documents 149, other documents 3), parse errors 0.
- Scale representations: 596 (authored 534, unavailable 62).
- Authored representations with elevation, front, plan and both-language decisions complete: 534 of 534.
- Gaps: 0.

Gaps by kind: json 0, document 0, category 0, representation 0, status 0, geometry 0, decisions 0, viewpoints 0, manifest 0.

## Per category

| category | assets | reps | authored | unavailable | elevation ok | front ok | plan ok | decisions complete | complete | gaps |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| animals | 14 | 56 | 47 | 9 | 47 | 47 | 47 | 47 | 47 | 0 |
| appliances | 24 | 96 | 70 | 26 | 70 | 70 | 70 | 70 | 70 | 0 |
| components | 15 | 60 | 56 | 4 | 56 | 56 | 56 | 56 | 56 | 0 |
| furniture | 12 | 48 | 47 | 1 | 47 | 47 | 47 | 47 | 47 | 0 |
| humans | 28 | 112 | 106 | 6 | 106 | 106 | 106 | 106 | 106 | 0 |
| machinery | 16 | 64 | 55 | 9 | 55 | 55 | 55 | 55 | 55 | 0 |
| objects | 15 | 60 | 56 | 4 | 56 | 56 | 56 | 56 | 56 | 0 |
| vegetation | 16 | 64 | 63 | 1 | 63 | 63 | 63 | 63 | 63 | 0 |
| vehicles | 9 | 36 | 34 | 2 | 34 | 34 | 34 | 34 | 34 | 0 |

## Gap list

| category | asset | scale | kind | missing item |
|---|---|---:|---|---|
| - | - | - | - | none |
