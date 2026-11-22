"""📍️ Calculates the history preview's isolated and downstream poses in SQLite."""

from contextlib import closing
import json
from pathlib import Path
import sqlite3

root = Path(__file__).resolve().parent.parents[6]
fixtures = root / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️selection/✋️drag-selection"
selected = fixtures / "⏸️keeps-a-zero-offset"
downstream = fixtures / "⚠️skips-locked-ghost"
before = Path("📸️snapshot/⬅️before/🔣️.json")
mutation = Path("🦠️mutation/🔣️.json")
board = (selected / before).read_text(encoding="utf-8")
assert json.loads(board) == json.loads((downstream / before).read_text(encoding="utf-8"))
first = json.loads((selected / mutation).read_text(encoding="utf-8"))
second = json.loads((downstream / mutation).read_text(encoding="utf-8"))

with closing(sqlite3.connect(":memory:")) as database:
    query = """
        SELECT axis.value + COALESCE((
            SELECT SUM(delta.value)
            FROM json_each(?) AS operation, json_each(operation.value -> '$.offset') AS delta
            WHERE delta.key = axis.key AND 'object-b' IN (
                SELECT value FROM json_each(operation.value -> '$.targets')
            )
        ), 0)
        FROM json_each(? -> '$.objects') AS object, json_each(object.value -> '$.origin') AS axis
        WHERE object.value ->> '$.id' = 'object-b' ORDER BY axis.key
    """
    prefix = tuple(row[0] for row in database.execute(query, (json.dumps([first]), board)))
    head = tuple(row[0] for row in database.execute(query, (json.dumps([first, second]), board)))
assert prefix == (4, 0, 0)
assert head == (5, 1, 0)
print(f"SQLite preview prefix {prefix}; committed downstream head {head}")
