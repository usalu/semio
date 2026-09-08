#!/usr/bin/env python3
"""🛂️ Builds the row-115 rename plan: per-case schema authorities to the taxonomy `test-fixture-schema-authority` name."""
import json, os, sys

LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

def plan(report):
    dirs, files = [], []
    for line in open(report, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        row = json.loads(line)
        path = row.get("path", "")
        if not path.startswith(LIB + "/"):
            continue
        if row["code"] == "fixture-defines-schema":
            source = os.path.dirname(path)
            assert os.path.basename(source) == "🧬️schema", source
            dirs.append((source, os.path.join(os.path.dirname(source), "🛂️schema")))
        elif row["code"] == "placement-retired-location":
            files.append((path, os.path.join(os.path.dirname(path), "🛂️schema.json")))
    return sorted(set(dirs)), sorted(set(files))

if __name__ == "__main__":
    d, f = plan(sys.argv[1])
    for source, target in d + f:
        print(f"{source}\t{target}")
    print(f"# {len(d)} directories, {len(f)} files", file=sys.stderr)
