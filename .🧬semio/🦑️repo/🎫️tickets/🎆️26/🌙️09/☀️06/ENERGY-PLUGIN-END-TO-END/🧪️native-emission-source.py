"""📨️ Independently concatenate the shared native producer corpus with SQLite text ownership."""
import json
import sqlite3
from pathlib import Path
import jsonschema

root = next(p for p in Path(__file__).resolve().parents if (p / ".🧬semio").is_dir())
path = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🎟️capture/🧪️tests/🔣️.json"
law = json.loads(path.read_text())["producer"]
jsonschema.validate(law, {"type": "object", "required": ["cancelAfter", "wideRepeat", "verbText", "sameMetadataBacking", "maximumCopiedTextBytes"], "properties": {"cancelAfter": {"type": "array", "items": {"type": "integer", "minimum": 0}}, "wideRepeat": {"type": "integer", "minimum": 1}, "verbText": {"type": "string", "minLength": 1}, "sameMetadataBacking": {"const": True}, "maximumCopiedTextBytes": {"const": 64}}})
text = law["verbText"] * law["wideRepeat"]
chunks, current = [], ""
for char in text:
    if len((current + char).encode()) > law["maximumCopiedTextBytes"]:
        chunks.append(current)
        current = ""
    current += char
chunks.append(current)
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE chunks (position INTEGER PRIMARY KEY, text TEXT)")
    db.executemany("INSERT INTO chunks VALUES (?, ?)", enumerate(chunks))
    assert db.execute("SELECT group_concat(text, '') FROM (SELECT text FROM chunks ORDER BY position)").fetchone()[0] == text
    assert db.execute("SELECT max(length(cast(text AS BLOB))) FROM chunks").fetchone()[0] <= 64
print(f"Native emission producer: SQLite reconstructed {len(text.encode())} UTF-8 bytes in {len(chunks)} bounded chunks; JSON Schema passed")
