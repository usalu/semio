"""⏱️ Phase table of every process profile in a product log (`SEMIO_REPO_PROFILE=json` writes one JSON line per process on exit): `python3 -I profsum.py <log> [min-ms]`.

Streams the log line by line; prints per process its total, boot and every phase of at least `min-ms` (default 50) with start, duration and detail, plus the graph-memo and Bazel-plan marks."""
import json, sys

LOG = sys.argv[1]
FLOOR = float(sys.argv[2]) if len(sys.argv) > 2 else 50.0
MARKS = ("graph.memo", "graph.memo-seal", "bazel.plan", "dependencies.vouch", "source.served", "boot.loaded", "boot.main")

with open(LOG, encoding="utf8", errors="replace") as stream:
    for line in stream:
        start = line.find('{"pid"')
        if start < 0:
            continue
        try:
            record = json.loads(line[start:])
        except ValueError:
            continue
        rows = record.get("rows", [])
        print(f'PID {record.get("pid")} total {record.get("total", 0):.0f} ms covered {record.get("covered", 0):.0f}')
        for row in rows:
            if row["duration"] >= FLOOR or row["phase"] in MARKS:
                print(f'  {"  " * row.get("depth", 0)}{row["phase"]:<30} start {row["start"]:8.0f} dur {row["duration"]:8.0f} {row.get("detail", "")}')
