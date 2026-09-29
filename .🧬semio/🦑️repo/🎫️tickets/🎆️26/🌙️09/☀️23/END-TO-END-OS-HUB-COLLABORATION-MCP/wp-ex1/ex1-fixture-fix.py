#!/usr/bin/env python3
"""🩹️ EX1: corrects a plugin's example catalog from a law capture — every `its row is {…}` a failing law printed replaces
the row of its app (or is added); rows whose app no editor claims (duplicates of a corrected app id) are reported.
usage: python3 ex1-fixture-fix.py <fixture.json> <capture.txt>"""
import json, re, sys
fixture_path, capture = sys.argv[1], sys.argv[2]
fixture = json.load(open(fixture_path, encoding="utf-8"))
text = open(capture, encoding="utf-8", errors="replace").read()
rows = {}
for m in re.finditer(r"its row is (\{\"app\": .*?\]\})", text):
    row = json.loads(m.group(1))
    rows[row["app"]] = row
apps = fixture["apps"]
for app, row in rows.items():
    for index, existing in enumerate(apps):
        if existing["app"] == app:
            apps[index] = row
            break
    else:
        apps.append(row)
    print("row", app, [e["id"] + ":" + e["document"] for e in row["examples"]])
seen = {}
for row in apps:
    seen.setdefault(row["app"], 0)
    seen[row["app"]] += 1
print("duplicates", [app for app, n in seen.items() if n > 1], "apps", len(apps))
json.dump(fixture, open(fixture_path, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
open(fixture_path, "a", encoding="utf-8").write("\n")
