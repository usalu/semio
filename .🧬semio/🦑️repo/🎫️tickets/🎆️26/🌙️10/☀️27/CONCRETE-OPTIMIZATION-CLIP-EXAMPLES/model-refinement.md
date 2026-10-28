# Pareto Refinement Now Equals The Search Optimum

`pareto()["refinement"]` is computed from `surface()["minimum"]["design"]` (dense 41 × 41 grid, south, 16 cm): no typed numbers. `🔣️.json` only names the `member`; the schema no longer has a refinement `design`.

| Item | Value |
| --- | --- |
| Refinement design | south 180°, WWR 66 %, overhang 0.99 m, insulation 16 cm |
| Refinement cost | 21,260.16 CHF (21,260 CHF) |
| Refinement electricity | 494.84 kWh/a |
| Dominated front member (`member`) | `w50-t30-d60` (WWR 50 %, d 0.6 m, t 30 cm): 21,552 CHF, 519.83 kWh/a (the lowest-electricity front member) |
| Δ CHF | −291.84 CHF (cheaper) |
| Δ kWh/a | −24.99 kWh/a (less electricity) |
| `dominates` | `w40-t30-d60` (21,384 CHF, 520.08 kWh/a: −123.84 CHF, −25.24 kWh/a), `w50-t30-d60` |

Other front members stay non-dominated (the refinement costs more than every front member up to `w50-t20-d60` at 20,832 CHF). Pareto `note` now says 6 window ratios (en and de).

Files: `🧮️model/🔣️.json`, `🧮️model/🧬️schema/🔣️.schema.json`, `🧮️model/🐍️.py` (`_pareto`), `🧪️cases/🔣️.json`, `🧪️tests/🐍️.py` (refinement test re-derives the argmin of the dense grid with NumPy and the dominated front members), `model.md`, `model-tools/build_schema.py`, `model-tools/build_report.py`.

Note: the algorithms clip's earlier "≈ 498 kWh/a at ≈ 20,873 CHF" is an algorithm end point; the exact surface optimum is 494.84 kWh/a at 21,260 CHF.
