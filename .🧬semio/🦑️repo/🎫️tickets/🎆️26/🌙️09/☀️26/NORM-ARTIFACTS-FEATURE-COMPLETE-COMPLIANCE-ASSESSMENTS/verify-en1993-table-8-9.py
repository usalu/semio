"""🌉 Verify EN 1993-1-9 Table 8.9 against an independent SQLite calculation."""

import json
import math
from pathlib import Path
import sqlite3
import sys

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
direct = """
SELECT 1, :deck_open_range * :gamma_ff,
       (CASE WHEN :deck_open_thickness <= 12 THEN 80 ELSE 71 END) / :gamma_mf
WHERE :basis = 'confirmed' AND :deck_open_thickness > 0
UNION ALL
SELECT 2, :deck_open_frequent_direct, 1.5 * :fy
WHERE :basis = 'confirmed' AND :deck_open_thickness > 0
ORDER BY 1
"""
continuous = """
WITH valid(sigma, tau) AS (
    SELECT :deck_cross_direct, :deck_cross_shear
    WHERE :basis = 'confirmed' AND :deck_cross_equivalence > 0
)
SELECT 1, 0.5 * (sigma + sqrt(sigma * sigma + 4 * tau * tau))
          * :deck_cross_equivalence * :gamma_ff, 56 / :gamma_mf
FROM valid
UNION ALL SELECT 2, :deck_open_frequent_direct, 1.5 * :fy FROM valid
UNION ALL SELECT 3, :deck_cross_frequent_shear, 1.5 * :fy / sqrt(3) FROM valid
UNION ALL SELECT 4, :deck_between_utilization, 1.0 FROM valid
ORDER BY 1
"""
with sqlite3.connect(":memory:") as database:
    database.create_function("sqrt", 1, math.sqrt)
    for entry, basis, values, expected_rows in fixture["cases"]:
        query = direct if entry == "C93-19" else continuous
        actual_rows = database.execute(query, dict(values, basis=basis)).fetchall()
        assert len(actual_rows) == len(expected_rows), (entry, values)
        for (_, demand, capacity), expected in zip(actual_rows, expected_rows):
            assert math.isclose(demand, expected[0], abs_tol=1e-10), (entry, values)
            assert math.isclose(capacity, expected[1], abs_tol=1e-10), (entry, values)
            assert ("pass" if demand <= capacity else "fail") == expected[2], (entry, values)
print(f"SQLite {sqlite3.sqlite_version}: {len(fixture['cases'])} cases verified")
