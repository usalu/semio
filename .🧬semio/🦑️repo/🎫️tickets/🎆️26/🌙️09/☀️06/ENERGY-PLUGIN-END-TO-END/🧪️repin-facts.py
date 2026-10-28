"""📌️ Validate canonical checkpoint reidentification against SQLite JSON mutation."""
import json
import sqlite3
from pathlib import Path
import jsonschema

root = Path(__file__).resolve().parents[7]
domain = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original"
fixture = json.loads((domain / "repin-facts.json").read_text())
jsonschema.validate(fixture, json.loads((domain / "repin-facts-schema.json").read_text()))
with sqlite3.connect(":memory:") as db:
    encoded = db.execute("select json_set(?, '$.id', ?)", (json.dumps(fixture["checkpoint"]), fixture["checkpointId"])).fetchone()[0]
    assert json.loads(encoded) == fixture["expected"]
print("PASS: JSON Schema and SQLite canonical checkpoint repin metadata")

fixture = json.loads((domain / "repin-alternative.json").read_text())
jsonschema.validate(fixture, json.loads((domain / "repin-alternative-schema.json").read_text()))
with sqlite3.connect(":memory:") as db:
    original = fixture["alternative"]
    db.execute("create table refs (ordinal integer primary key, checkpoint text not null)")
    db.executemany("insert into refs values (?,?)", enumerate(original["checkpointIds"]))
    db.execute("update refs set checkpoint=? where checkpoint=?", (fixture["replacement"], fixture["checkpointId"]))
    changed = {**original, "checkpointIds": [row[0] for row in db.execute("select checkpoint from refs order by ordinal")]}
    assert changed == fixture["expected"]
print("PASS: JSON Schema and SQLite original alternative reference reidentification")
