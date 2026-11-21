"""🚉 Verify the Table 8.8 suite with an independent SQLite calculation."""

import json
import math
from pathlib import Path
import sqlite3
import sys

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
query = """
WITH decision AS (
    SELECT
      CASE :entry
        WHEN 'C93-21' THEN CASE :closed_connection_detail
          WHEN 'cutout' THEN CASE WHEN :closed_web_thickness  > 0 AND :closed_web_thickness <= 12 THEN 80
                                 WHEN :closed_web_thickness > 12 THEN 71 END
          WHEN 'continuous' THEN CASE WHEN :closed_web_thickness  > 0 AND :closed_web_thickness <= 12 THEN 80
                                     WHEN :closed_web_thickness > 12 THEN 71 END
          WHEN 'separate' THEN 36
          WHEN 'backed' THEN 71
        END
        WHEN 'C93-22' THEN CASE :closed_cutout_method WHEN 'vierendeel' THEN 71 WHEN 'bridge-method' THEN 112 END
        WHEN 'C93-23' THEN CASE WHEN :closed_splice_thickness > 0 THEN
          (CASE :closed_splice_group WHEN 'ground' THEN 112 WHEN 'flat' THEN 90 WHEN 'as-welded' THEN 80 END)
          * pow(min(25.0 / :closed_splice_thickness, 1.0), 0.2)
        END
        WHEN 'C93-24' THEN CASE WHEN :closed_weld_throat > 0 AND :closed_weld_rib_thickness > 0 THEN
          CASE WHEN :closed_weld_type = 'recessed' AND :closed_weld_throat >= :closed_weld_rib_thickness
                    AND :closed_weld_gap <= 2 AND :closed_weld_root_face <= 2 THEN 71
               WHEN :closed_weld_gap <= 0.5 THEN 50 END
        END
      END AS category
    WHERE :closed_geometry = CASE WHEN :closed_connection_detail = 'backed'
                                  THEN 'backed-confirmed' ELSE 'confirmed' END
),
working AS (
    SELECT category, CASE category WHEN 71 THEN :closed_weld_rib_thickness ELSE :closed_weld_throat END AS weld_thickness
    FROM decision WHERE category IS NOT NULL
)
SELECT (CASE WHEN :entry = 'C93-24' THEN :closed_weld_moment * 6.0 / (weld_thickness * weld_thickness)
             ELSE :closed_range END) * :gamma_ff,
       category / :gamma_mf,
       CASE WHEN :entry = 'C93-24' THEN :closed_weld_frequent_moment * 6.0 / (weld_thickness * weld_thickness)
            ELSE :closed_frequent END,
       1.5 * :fy
FROM working
"""
with sqlite3.connect(":memory:") as database:
    database.create_function("pow", 2, math.pow)
    for entry, choices, values, expected in fixture["cases"]:
        parameters = {"entry": entry, **choices, **values}
        keys = ("closed_connection_detail", "closed_web_thickness", "closed_cutout_method",
                "closed_splice_thickness", "closed_splice_group", "closed_weld_type",
                "closed_weld_rib_thickness", "closed_weld_throat", "closed_weld_gap",
                "closed_weld_root_face", "closed_weld_moment", "closed_weld_frequent_moment",
                "closed_range", "closed_frequent")
        for key in keys:
            parameters.setdefault(key, None)
        actual = database.execute(query, parameters).fetchone()
        if actual is None:
            assert not expected, parameters
            continue
        assert len(expected) == 2, parameters
        for demand, capacity, row in zip(actual[::2], actual[1::2], expected):
            assert math.isclose(demand, row[0], abs_tol=1e-9), parameters
            assert math.isclose(capacity, row[1], abs_tol=1e-9), parameters
            assert ("pass" if demand <= capacity else "fail") == row[2], parameters
print(f"SQLite {sqlite3.sqlite_version}: {len(fixture['cases'])} cases verified")
