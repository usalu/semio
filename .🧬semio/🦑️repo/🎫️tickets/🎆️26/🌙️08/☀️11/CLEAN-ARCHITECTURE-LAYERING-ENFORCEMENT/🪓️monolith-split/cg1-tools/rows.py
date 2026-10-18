# -*- coding: utf-8 -*-
import csv, sys
csv.field_size_limit(10**9)
rows = list(csv.DictReader(open(sys.argv[1], encoding="utf8")))
names = sys.argv[2:]
print("crates", len(rows), "oversized(lib|test)", sum(1 for r in rows if r["libOver"] == "true" or r["testOver"] == "true"))
print("name | libRatio(binding) | testRatio(binding) | test files/bytes/lines")
for r in rows:
    if any(r["name"] == n or (n.endswith("*") and r["name"].startswith(n[:-1])) for n in names):
        print(r["name"], "|", r["libRatio"], r["libBinding"], "|", r["testRatio"], r["testBinding"], "|", r["testFiles"], r["testBytes"], r["testLines"], "|", r["libFiles"], r["libBytes"], r["libLines"])
