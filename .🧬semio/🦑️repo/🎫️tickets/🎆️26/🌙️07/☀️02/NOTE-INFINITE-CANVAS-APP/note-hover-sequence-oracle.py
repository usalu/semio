"""👆 Validate hover changes with SQLite's adjacent-target comparison."""
import json
from pathlib import Path
import sqlite3
from sys import argv

fixture = json.loads(Path(argv[1]).read_text())
cases = {entry["id"]: entry["args"]["targets"] for entry in fixture["cases"] if entry["action"] == "interactionHover"}
law = fixture["hoverSequence"]
with sqlite3.connect(":memory:") as db:
    db.execute("create table input (ordinal integer, sample text, target text)")
    db.executemany("insert into input values (?, ?, ?)", ((i, sample, cases[sample]) for i, sample in enumerate(law["inputs"])))
    actual = [sample for (sample,) in db.execute("""
        select sample from (
          select ordinal, sample, target, lag(target) over (order by ordinal) as previous from input
        ) where target is not previous order by ordinal
    """)]
assert actual == law["published"], (actual, law["published"])
print("[DEBUG] SQLite hover transitions", actual)
