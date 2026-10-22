#!/usr/bin/env python3
"""Rewrites expected English error messages of the lawcov-cli corpora to the catalog keys the code throws since i18n-core (message `<code> <key> {values}`). usage: fix-messages.py <summary.txt> <logdir> [--write]"""
import json, os, re, sys
summary, logdir = sys.argv[1], sys.argv[2]
write = "--write" in sys.argv
changed = {}
for line in open(summary, encoding="utf8"):
    m = re.match(r"(\d+) rc=1 .*?s (\S.*)$", line.rstrip("\n"))
    if not m: continue
    n, test = m.group(1), m.group(2)
    log = open(os.path.join(logdir, "%03d.log" % int(n)), encoding="utf8").read()
    pairs = re.findall(r'Expected substring: (".*?")\nReceived message: (".*?")\n', log)
    unit = os.path.dirname(test)
    for exp, rec in pairs:
        e = json.loads(exp); r = json.loads(rec)
        parts = r.split(" ")
        if len(parts) < 2 or not re.match(r"^[a-z][a-z0-9.-]+$", parts[1]): continue
        key = parts[1]
        for root, _, names in os.walk(unit):
            for name in names:
                if not name.endswith((".json", ".ts", ".py")): continue
                p = os.path.join(root, name)
                s = open(p, encoding="utf8").read()
                j = json.dumps(e, ensure_ascii=False)
                if j in s:
                    changed.setdefault(p, []).append((e, key))
                    if write: s = s.replace(j, json.dumps(key)); open(p, "w", encoding="utf8").write(s)
for p, rows in changed.items(): print(len(rows), p[-70:], sorted({(a[:40], b) for a, b in rows})[:2])
