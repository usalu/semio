"""🪟️ Validate original-row publication fixtures against SQLite transactions."""
import json
import sqlite3
from pathlib import Path
import jsonschema

root = Path(__file__).resolve().parents[7]
domain = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🗃️history-ledger"
fixture = json.loads((domain / "replacements.json").read_text())
jsonschema.validate(fixture, json.loads((domain / "replacements-schema.json").read_text()))
for case in fixture["cases"]:
    db = sqlite3.connect(":memory:")
    db.execute("create table rows (ordinal integer primary key, value integer not null)")
    db.executemany("insert into rows values (?,?)", enumerate(case["before"]))
    db.commit()
    displaced = []
    for rank, value in case["replacements"]:
        old = db.execute("select value from rows where ordinal=?", (rank,)).fetchone()[0]
        db.execute("update rows set value=? where ordinal=?", (value, rank))
        displaced.append(old if case["commit"] else value)
    if case["commit"]:
        db.commit()
    else:
        db.rollback()
    assert [r[0] for r in db.execute("select value from rows order by ordinal")] == case["after"]
    assert displaced == case["displaced"]
    db.close()
print("PASS: JSON Schema and SQLite original-row publication/abort fixtures")
