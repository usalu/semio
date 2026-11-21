"""🧭 Check the neutral picker visit order using SQLite's ordered projection."""

import json
import sqlite3
import sys
from pathlib import Path

rows = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))["idlePickerScans"]
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE instances (id INTEGER PRIMARY KEY, slot INTEGER UNIQUE)")
    for row in rows:
        db.execute("DELETE FROM instances")
        db.executemany("INSERT INTO instances VALUES (?1, ?1 % 1024)", [(instance,) for instance in row["instances"]])
        actual = [instance for (instance,) in db.execute("SELECT id FROM instances ORDER BY slot")]
        assert actual == row["visits"], (actual, row["visits"])
        print(json.dumps(actual))
