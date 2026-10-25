#!/usr/bin/env python3
"""usage: oracle-flip.py FILE ID...  -- sets productionReachable false and drops productionDebt of the named oracle entries (text-level, keeps formatting)."""
import json, re, sys
path, ids = sys.argv[1], sys.argv[2:]
t = open(path, encoding="utf8").read()
for oid in ids:
    i = t.index(f'"id": "{oid}"')
    m = re.compile(r'^( *)"productionReachable": true,\n( *)"productionDebt": \{\n', re.M).search(t, i)
    nxt = t.find('"id": "', i + 10)
    assert m and (nxt == -1 or m.start() < nxt), f"no debt block for {oid}"
    indent = m.group(2)
    end = t.index(f"\n{indent}}},\n", m.end()) + len(f"\n{indent}}},\n")
    t = t[:m.start()] + f'{m.group(1)}"productionReachable": false,\n' + t[end:]
json.loads(t)
open(path, "w", encoding="utf8").write(t)
print("flipped", ids)
