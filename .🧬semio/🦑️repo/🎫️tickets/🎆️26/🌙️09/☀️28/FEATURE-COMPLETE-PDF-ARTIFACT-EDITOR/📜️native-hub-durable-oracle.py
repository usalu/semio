"""📚️ Cross-check the authored-media fixture after closing and reopening SQLite."""

import contextlib
import hashlib
import json
import sqlite3
import sys
import tempfile
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
rows = [(index, bytes([97 + index]) * fixture["bytesPerRow"]) for index in range(fixture["rows"])]
generated = Path(__file__).resolve().parent / "🗑️generated"
with tempfile.TemporaryDirectory(prefix="sqlite-original-oracle-", dir=generated) as scratch:
    path = Path(scratch) / "original.sqlite3"
    with contextlib.closing(sqlite3.connect(path)) as database:
        assert database.execute("pragma journal_mode=wal").fetchone() == ("wal",)
        database.execute("pragma synchronous=full")
        database.executescript(
            "create table edits(id primary key, actor);"
            "create table rows(edit references edits, ordinal, body, primary key(edit, ordinal));"
        )
        with database:
            database.execute("insert into edits values (?, ?)", (fixture["edit"], fixture["actor"]))
            database.executemany("insert into rows values (?, ?, ?)", [(fixture["edit"], ordinal, body) for ordinal, body in rows])
        with database:
            database.execute("insert or ignore into edits values (?, ?)", (fixture["edit"], fixture["actor"]))

    with path.open("rb") as source:
        assert source.read(16) == b"SQLite format 3\0"
    with contextlib.closing(sqlite3.connect(path)) as database:
        assert database.execute("select count(*) from edits").fetchone() == (fixture["commits"],)
        replayed = database.execute(
            "select ordinal, body from rows where edit in (select edit from rows where ordinal >= ?) order by ordinal",
            (fixture["catchupFrom"],),
        ).fetchall()
        assert replayed == rows

print("[DEBUG]", json.dumps({
    "commits": fixture["commits"],
    "rows": len(replayed),
    "sha256": hashlib.sha256(b"".join(body for _, body in replayed)).hexdigest(),
}))
