"""📚️ Cross-checks the durable authored-media fixture against SQLite transaction replay."""

import hashlib
import json
import sqlite3
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
rows = [(index, bytes([97 + index]) * fixture["bytesPerRow"]) for index in range(fixture["rows"])]
database = sqlite3.connect(":memory:")
database.executescript("create table edits(id primary key, actor); create table rows(edit references edits, ordinal, body, primary key(edit, ordinal));")
with database:
    database.execute("insert into edits values (?, ?)", (fixture["edit"], fixture["actor"]))
    database.executemany("insert into rows values (?, ?, ?)", [(fixture["edit"], ordinal, body) for ordinal, body in rows])
with database:
    database.execute("insert or ignore into edits values (?, ?)", (fixture["edit"], fixture["actor"]))
assert database.execute("select count(*) from edits").fetchone() == (fixture["commits"],)
replayed = database.execute("select ordinal, body from rows where edit in (select edit from rows where ordinal >= ?) order by ordinal", (fixture["catchupFrom"],)).fetchall()
assert replayed == rows
print("[DEBUG]", json.dumps({"commits": fixture["commits"], "rows": len(replayed), "sha256": hashlib.sha256(b"".join(body for _,body in replayed)).hexdigest()}))
