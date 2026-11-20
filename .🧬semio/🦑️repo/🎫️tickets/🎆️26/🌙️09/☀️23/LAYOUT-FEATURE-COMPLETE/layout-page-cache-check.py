import json
import sqlite3
from pathlib import Path

root = next(Path("✏️s").glob("🔌️plugins/📏️layout/**/🪡️apply/🗃️page-cache"))
fixture = json.loads((root / "🧪️tests/🔣️.json").read_text())
with sqlite3.connect(":memory:") as database:
    for case in fixture["cases"]:
        layers = database.execute(
            "SELECT value FROM json_each(?) ORDER BY key",
            (json.dumps(case["layers"]),),
        ).fetchall()
        layer_ids = [row[0] for row in layers]
        object_ids = [
            [
                row[0]
                for row in database.execute(
                    """
                    SELECT json_extract(value, '$.id')
                    FROM json_each(?)
                    WHERE json_extract(value, '$.layer') = ?
                    ORDER BY key
                    """,
                    (json.dumps(case["frames"]), layer),
                )
            ]
            for layer in layer_ids
        ]
        assert {"layerIds": layer_ids, "objectIds": object_ids} == case["expected"]
print(f"SQLite verified all {len(fixture['cases'])} Layout page cache fixtures")
