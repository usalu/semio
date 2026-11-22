"""📣 Independently assert the shell notices a mounted case recorded in its console report."""
import argparse
import json
from pathlib import Path

cli = argparse.ArgumentParser()
cli.add_argument("report", type=Path)
cli.add_argument("required")
cli.add_argument("--absent", action="append", default=[])
args = cli.parse_args()
rows = json.loads(args.report.read_text(encoding="utf8"))
assert isinstance(rows, list) and all(isinstance(row, str) for row in rows)
assert any(row.startswith(args.required + "|") for row in rows), f"missing notice {args.required}"
for name in args.absent:
    assert not any(name in row for row in rows), f"unexpected {name}"
print(f"[DEBUG] {args.report.name} recorded {args.required} without {len(args.absent)} forbidden marker(s)")
