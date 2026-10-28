"""🗂️ Validate native object receiving semantics with independent SQLite ownership-neutral fixtures."""
import json
import sqlite3
from pathlib import Path
import jsonschema

root = next(parent for parent in Path(__file__).resolve().parents if (parent / ".🧬semio").is_dir())
fixture = root / "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🗂️map"
law = json.loads((fixture / "🔣️.json").read_text())
jsonschema.Draft202012Validator(json.loads((fixture / "🧬️.json").read_text())).validate(law)
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE entries (key TEXT PRIMARY KEY COLLATE BINARY, value TEXT NOT NULL)")
    for case in law["cases"]:
        db.execute("DELETE FROM entries")
        db.executemany("INSERT INTO entries VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value", case["entries"])
        assert [list(row) for row in db.execute("SELECT key, value FROM entries ORDER BY key COLLATE BINARY")] == case["expected"]
print(f"Native map fixture: {len(law['cases'])} SQLite and JSON Schema cases passed")

metadata_root = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest"
metadata = json.loads((metadata_root / "metadata.json").read_text())
view_schema = root / "🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json"
jsonschema.Draft202012Validator(json.loads(view_schema.read_text())).validate(metadata["view"])
with sqlite3.connect(":memory:") as db:
    row = db.execute("SELECT json_extract(?, '$.locale'), json_extract(?, '$.activeUtilityByWindowId.energy-window'), json_extract(?, '$.toolRunTraceCursorByWindowId.energy-window.run')", [json.dumps(metadata["view"])] * 3).fetchone()
    assert row == ("de", "inspect", 17)
print("Native metadata fixture: JSON Schema and SQLite projections passed")


alias = json.loads((root / "🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧬️alias.json").read_text())
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE aliases (key TEXT PRIMARY KEY, value TEXT)")
    db.executemany("INSERT INTO aliases VALUES (?, ?)", alias["entries"])
    assert dict(db.execute("SELECT key,value FROM aliases")) == dict(alias["entries"])
    assert alias["sharedRoot"] and alias["aliasCapacityBytes"] == 0
print("Immutable map alias fixture: independent SQLite values passed; native tests prove pointer sharing and retirement")

surfaces = json.loads((root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/native.json").read_text())
with sqlite3.connect(":memory:") as db:
    for case in surfaces["cases"]:
        if case["role"] == "section":
            assert case["preserve"]
            continue
        roster = json.dumps(case["roster"])
        selected = db.execute("SELECT value FROM json_each(?) WHERE value=?", (roster, case["target"])).fetchone() if case["role"] == "window" else None
        assert (selected[0] if selected else None) == case["expected"]
print("Surface role fixture: independent SQLite roster projection passed")
