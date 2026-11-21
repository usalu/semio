"""🚋 Verify the runway fatigue fixture independently against EN 1993-1-9 Table 8.10 and §8."""

import json
import math
from pathlib import Path
import sqlite3
import sys

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
query = """
WITH categories(detail, required_location, category) AS (
    VALUES ('rolled', 'web', 160), ('full-tee', 'web', 71),
           ('partial-tee', 'throat', 36), ('fillet', 'throat', 36),
           ('t-full', 'web', 71), ('t-partial', 'throat', 36),
           ('t-fillet', 'throat', 36)
),
check_values AS (
    SELECT :equivalentRangeMpa * :gammaFf AS fatigue_demand,
           category / (1.0 * :gammaMf) AS fatigue_resistance,
           :frequentRangeMpa AS frequent_demand,
           1.5 * :yieldMpa AS frequent_limit
    FROM categories
    WHERE detail = :detail AND required_location = :location AND :basis = 'confirmed'
)
SELECT fatigue_demand, fatigue_resistance,
       CASE WHEN fatigue_demand <= fatigue_resistance THEN 'pass' ELSE 'fail' END,
       CASE WHEN frequent_demand <= frequent_limit THEN 'pass' ELSE 'fail' END
FROM check_values
"""
with sqlite3.connect(":memory:") as database:
    for row in fixture["cases"]:
        assert len(row) == len(fixture["columns"]), row
        case = dict(zip(fixture["columns"], row))
        result = database.execute(query, case).fetchone()
        expected = row[-4:]
        if result is None:
            assert expected == [None, None, "not-applicable", "not-applicable"], case
        else:
            assert math.isclose(result[0], expected[0], abs_tol=1e-10), case
            assert math.isclose(result[1], expected[1], abs_tol=1e-10), case
            assert list(result[2:]) == expected[2:], case
print(f"SQLite {sqlite3.sqlite_version}: {len(fixture['cases'])} cases verified")
