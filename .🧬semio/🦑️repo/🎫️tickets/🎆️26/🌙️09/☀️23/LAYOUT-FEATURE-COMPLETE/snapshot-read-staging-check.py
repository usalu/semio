import json
import sqlite3
from pathlib import Path

path = Path("🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🪜️staging/🧪️tests/🔣️.json")
law = json.loads(path.read_text())

with sqlite3.connect(":memory:") as database:
    database.execute("CREATE TABLE snapshots(id INTEGER PRIMARY KEY, value TEXT NOT NULL)")
    database.execute("INSERT INTO snapshots VALUES (1, ?)", (law["source"],))
    for cut in law["cancelAfterTurns"]:
        database.execute("SAVEPOINT projection")
        database.execute("CREATE TEMP VIEW selected AS SELECT value FROM snapshots WHERE id = 1")
        if cut:
            assert database.execute("SELECT value FROM selected").fetchone() == (law["source"],)
        database.execute("ROLLBACK TO projection")
        database.execute("RELEASE projection")
        assert database.execute("SELECT value FROM snapshots WHERE id = 1").fetchone() == (law["source"],)
    assert database.execute("SELECT value FROM snapshots WHERE id = 1").fetchone() == (law["published"],)

print("SQLite verified the immutable source value across cancellation and publication")
