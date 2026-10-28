"""🪪️ SQLite concatenation independently verifies native canonical transition identity fixtures."""
import json
import sqlite3
from pathlib import Path
import jsonschema
root = Path(__file__).resolve().parents[7]
fixture = next(root.glob("🧰*/🛍*/💻*/🔨*/🏪*/⚙*/🌿*/📜*/ids.json"))
law = json.loads(fixture.read_text())
jsonschema.validate(law, json.loads(fixture.with_name("ids.schema.json").read_text()))
connection = sqlite3.connect(":memory:")
for case in law["cases"]:
    for width in law["widths"]:
        prefix = "x" * width
        actual, = connection.execute("select ? || case when ? is null then '' else '#' || cast(? as text) end", (prefix + case["source"], case["suffix"], case["suffix"])).fetchone()
        assert actual == prefix + case["expected"]
print("[DEBUG] Native transition identity fixture matches JSON Schema and SQLite concatenation")

batch = json.loads(fixture.with_name("batch.json").read_text())
jsonschema.validate(batch, json.loads(fixture.with_name("batch.schema.json").read_text()))
for mode in batch["modes"]:
    applies, = connection.execute("select ? in ('overwrite', 'trunk', 'alternative')", (mode,)).fetchone()
    assert bool(applies) == batch["expected"]["appliesToHead"]
print("[DEBUG] Native transition batch fixture matches JSON Schema and SQLite scope selection")

existing = json.loads(fixture.with_name("existing.json").read_text())
jsonschema.validate(existing, json.loads(fixture.with_name("existing.schema.json").read_text()))
connection.execute("create table phases (position integer, events integer, applies integer, head real)")
connection.executemany("insert into phases values (?,?,?,?)", [(0,3,1,existing["firstHead"]),(1,2,1,existing["secondHead"]),(2,1,0,existing["inactiveReplacement"]),(3,1,0,existing["inactiveReplacement"])])
counts = [row[0] for row in connection.execute("select sum(events) over (order by position) from phases order by position")]
assert counts == existing["expectedTransitions"]
last_head, = connection.execute("select head from phases where applies = 1 order by position desc limit 1").fetchone()
assert last_head == existing["secondHead"]
print("[DEBUG] Existing native checkpoint and inactive scopes match independent SQLite event and head selection")
