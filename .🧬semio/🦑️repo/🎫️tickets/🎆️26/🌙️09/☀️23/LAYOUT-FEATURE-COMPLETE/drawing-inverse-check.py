import json
import sqlite3
from pathlib import Path

root = next(Path("✏️s").glob("🔌️plugins/🖍️draw/**/✳️any/🧬️schema"))
snapshot = json.loads((root / "✅️validation/🧪️.json").read_text())["base"]
law = json.loads(
    (root / "🧬️mutations/🧳️retained/↩️inverse/🔣️.json").read_text()
)["visibilityLaw"]
authored = law["authored"]

with sqlite3.connect(":memory:") as database:
    (inverse,) = database.execute(
        """
        WITH RECURSIVE layers(node) AS (
            SELECT value FROM json_each(?, '$.layers')
            UNION ALL
            SELECT child.value FROM layers JOIN json_each(layers.node, '$.children') AS child
            WHERE child.type = 'object'
        )
        SELECT json_set(?, '$.visible', json(IIF(json_extract(node, '$.visible')=1, 'true', 'false')))
        FROM layers
        WHERE json_extract(node, '$.id') = ?
        """,
        (json.dumps(snapshot), json.dumps(authored), authored["layerId"]),
    ).fetchone()
assert [json.loads(inverse)] == law["inverse"]
assert json.loads(inverse)["visible"] is not authored["visible"]
print("SQLite verified the original visibility and exact inverse command")
