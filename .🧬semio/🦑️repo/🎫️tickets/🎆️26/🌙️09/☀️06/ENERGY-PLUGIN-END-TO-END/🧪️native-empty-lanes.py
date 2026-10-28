"""🕳️ Validate the native empty-lane corpus through independent schema and SQLite projections."""
import json
import sqlite3
from pathlib import Path
import jsonschema
root = next(p for p in Path(__file__).resolve().parents if (p / ".🧬semio").is_dir())
law = json.loads((root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🕳️empty-lanes/genesis.json").read_text())
jsonschema.validate(law["state"], {"type": "object", "additionalProperties": False})
assert law["rejectEmptyBytes"]
with sqlite3.connect(":memory:") as db:
    assert json.loads(db.execute("SELECT json_object()").fetchone()[0]) == law["state"]
    ids = [db.execute("SELECT 'empty-' || ? || ':' || ?", (kind, "energy-empty")).fetchone()[0] for kind in ("config", "draft")]
    assert ids == ["empty-config:energy-empty", "empty-draft:energy-empty"]
    assert law["configSchema"] != law["draftSchema"]
print("Empty native lanes: JSON Schema and SQLite state/identity projections passed")
