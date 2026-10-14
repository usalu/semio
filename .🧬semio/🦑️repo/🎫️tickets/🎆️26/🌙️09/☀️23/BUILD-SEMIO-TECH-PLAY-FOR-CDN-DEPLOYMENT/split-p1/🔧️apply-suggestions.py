#!/usr/bin/env python3
"""🔧️ Applies rustc's machine-applicable suggestions for the named lints to files under the given roots (lane P1).

Used after a split to drop the imports and qualifications the cut made redundant (`unused_imports`,
`unused_qualifications`). Edits are byte-offset replacements applied bottom-up per file; overlapping edits are skipped.
usage: 🔧️apply-suggestions.py <cargo json log> <lint,lint,...> <root>...
"""
import json, os, sys
log, lints, roots = sys.argv[1], set(sys.argv[2].split(",")), sys.argv[3:]
WS = "/Users/ueli/Documents/semio"
def absf(f):
    if os.path.isabs(f): return os.path.normpath(f)
    for base in [WS] + roots + [os.path.dirname(r) for r in roots]:
        c = os.path.normpath(os.path.join(base, f))
        if os.path.exists(c): return c
    return None
edits = {}
def walk(msg):
    for sp in msg.get("spans", []):
        if sp.get("suggestion_applicability") == "MachineApplicable" and sp.get("suggested_replacement") is not None:
            yield sp
    for c in msg.get("children", []): yield from walk(c)
for line in open(log, encoding="utf-8"):
    try: d = json.loads(line)
    except Exception: continue
    if d.get("reason") != "compiler-message": continue
    m = d["message"]; code = (m.get("code") or {}).get("code")
    if code not in lints: continue
    for sp in walk(m):
        f = absf(sp["file_name"])
        if not f or not any(f.startswith(r) for r in roots): continue
        edits.setdefault(f, set()).add((sp["byte_start"], sp["byte_end"], sp["suggested_replacement"]))
total = 0
for f, rows in edits.items():
    data = open(f, "rb").read(); last = len(data) + 1
    for a, b, rep in sorted(rows, reverse=True):
        if b > last: continue
        data = data[:a] + rep.encode() + data[b:]; last = a; total += 1
    open(f, "wb").write(data)
print("applied", total, "in", len(edits), "files")
