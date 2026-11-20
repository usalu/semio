import json
import re
import sqlite3
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
server = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))["definitions"]["Server"]["const"]
connection = sqlite3.connect(":memory:")
connection.execute("CREATE TABLE high_water (dimension TEXT PRIMARY KEY, value INTEGER)")
expected_steps = 0

def mark(dimension, value, baseline_counts):
    prior = connection.execute("SELECT value FROM high_water WHERE dimension = ?", (dimension,)).fetchone()
    connection.execute(
        "INSERT INTO high_water VALUES (?, ?) ON CONFLICT (dimension) DO UPDATE SET value = MAX(value, excluded.value)",
        (dimension, value),
    )
    return (baseline_counts and value > 0) if prior is None else value > prior[0]

for case in fixture["cases"]:
    connection.execute("DELETE FROM high_water")
    last = 0
    for step in case["steps"]:
        line = step["line"]
        advances = []
        if line.startswith("[script] {"):
            event = json.loads(line.removeprefix("[script] "))
            prefix, kind, phase, done, total, *running = event["activity"]["resource"].split(":")
            if prefix == "build" and kind == "graph-phase" and int(done) <= int(total):
                advances.append(mark("phase:" + phase, int(done), True))
                if running and phase == "inference":
                    for token in running[2].split(","):
                        name, _kind, _age, rows = token.split("|")
                        advances.append(mark("owner:" + name, int(rows), False))
        elif line.startswith("[owners] ") and (" done:" in line or " fertig:" in line):
            done, total, present, fetched, here, *_ = map(int, re.findall("[0-9]+", line))
            if done <= total and done == present + fetched + here:
                advances.append(mark("ledger", done, True))
        if any(advances):
            last = step["at"]
        elapsed, lifetime = step["at"] - last, step["at"]
        verdict = "too-long" if lifetime >= server["inferMaximumMilliseconds"] else (
            "stalled" if elapsed >= server["inferStallMilliseconds"] else "go"
        )
        assert (elapsed, verdict) == (step["silent"], step["verdict"]), (case["name"], step["label"], elapsed, verdict)
        expected_steps += 1

print(f"SQLite {sqlite3.sqlite_version} agrees with {expected_steps} neutral replay steps.")
