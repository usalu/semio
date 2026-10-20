#!/usr/bin/env python3
"""💍 Ring-law check without the test runner: every crate-boundaries row of the fixture must name no package that the crate's manifest declares under [dependencies].

usage: check_rings.py <root>
"""
import json, os, re, sys
root = sys.argv[1]
fx = json.load(open(root + "/🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json", encoding="utf-8"))
os_dir = root + "/🧰️framework/🛍️products/💻️os"
bad = rows = 0
for row in fx["crates"]:
    p = os.path.normpath(os.path.join(os_dir, row["manifest"]))
    if not os.path.isfile(p):
        continue
    rows += 1
    t = open(p, encoding="utf-8").read()
    m = re.search(r"^\[dependencies\](.*?)(?=^\[|\Z)", t, re.S | re.M)
    deps = set(re.findall(r"^([A-Za-z0-9_\-]+)\s*(?:=|\.)", m.group(1), re.M)) if m else set()
    for f in row["forbiddenDependencies"]:
        if f in deps:
            bad += 1
            print("VIOLATION", row["package"], "->", f)
print(json.dumps({"rowsChecked": rows, "violations": bad}))
sys.exit(1 if bad else 0)
