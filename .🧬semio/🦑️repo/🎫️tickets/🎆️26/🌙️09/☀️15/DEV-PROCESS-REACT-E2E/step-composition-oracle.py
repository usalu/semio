"""🔬️ Validate the neutral Process3d step scenario with SQLite."""
import json
import sqlite3
import sys
from pathlib import Path

scenario = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
with sqlite3.connect(":memory:") as database:
    database.execute("CREATE TABLE steps (position INTEGER, kind TEXT, label TEXT, posX REAL, radius REAL)")
    database.executemany(
        "INSERT INTO steps VALUES (?, ?, NULL, NULL, NULL)",
        enumerate(scenario["steps"]),
    )
    for edit in scenario["edits"]:
        field = {"label": "label", "posX": "posX"}[edit["field"]]
        database.execute(f"UPDATE steps SET {field}=? WHERE kind='cut'", (edit["value"],))
    database.execute("UPDATE steps SET radius=? WHERE kind='drill'", (scenario["drillRadius"],))
    move = scenario["move"]
    order = [
        kind
        for (kind,) in database.execute(
            """
            WITH stationary AS (
                SELECT kind, ROW_NUMBER() OVER (ORDER BY position) - 1 AS slot
                FROM steps WHERE kind <> :measure
            )
            SELECT kind FROM (
                SELECT kind, slot, 1 AS tie FROM stationary
                UNION ALL SELECT :measure, :index, 0
            ) ORDER BY slot, tie
            """,
            move,
        )
    ]
    label, x = database.execute("SELECT label, posX FROM steps WHERE kind='cut'").fetchone()
    (radius,) = database.execute("SELECT radius FROM steps WHERE kind='drill'").fetchone()
    stock = scenario["stockResize"]
    (volume,) = database.execute("SELECT ? * ? * ?", (stock["width"], stock["depth"], stock["height"])).fetchone()
    assert abs(volume - stock["volume"]) < 1e-12
result = {"order": order, "label": label, "positionX": x, "drillRadius": radius}
assert result == scenario["expected"], result
print(json.dumps({**result, "stockVolume": volume}))
