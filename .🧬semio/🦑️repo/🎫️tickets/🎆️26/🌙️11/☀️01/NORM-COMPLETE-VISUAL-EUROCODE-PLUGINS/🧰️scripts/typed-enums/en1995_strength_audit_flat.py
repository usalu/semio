#!/usr/bin/env python3
"""Audit of en1995 `strengthClass` values in the flattened fixture tree (PATH-BUDGET layout): maps every flat file to its former deep path, reads the committed version (HEAD, else index; read-only git) and compares the ordered strengthClass/newValue tokens case-insensitively.
Prints files without a committed version and files that differ. Usage: python3 -I en1995_strength_audit_flat.py"""
import os, re, subprocess
REPO = "/home/deck/semio"
ROOT = REPO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
MAP = {"after": "📸️snapshot/➡️after", "before": "📸️snapshot/⬅️before", "diff": "🔺️diff", "mutation": "🦠️mutation", "outcome": "🎯️outcome"}
RX = re.compile(r'"(?:strengthClass)":\s*"([^"]*)"')
def show(spec):
    r = subprocess.run(["git", "-C", REPO, "show", spec], capture_output=True)
    return r.stdout.decode("utf-8", "replace") if r.returncode == 0 else None
none, bad, ok = [], [], 0
for base, _, files in os.walk(ROOT + "/🧬️mutations"):
    for f in files:
        p = os.path.join(base, f)
        t = open(p, encoding="utf-8").read()
        cur = [v.lower() for v in RX.findall(t)]
        if not cur:
            continue
        m = re.match(r"🔣️\.(\w+)\.json$", f)
        rel = os.path.relpath(base, REPO) + "/" + MAP[m.group(1)] + "/🔣️.json"
        h = show("HEAD:" + rel) or show(":" + rel)
        if h is None:
            none.append((os.path.relpath(p, ROOT), cur))
        elif [v.lower() for v in RX.findall(h)] != cur:
            bad.append((os.path.relpath(p, ROOT), [v.lower() for v in RX.findall(h)][:6], cur[:6]))
        else:
            ok += 1
print("identical to committed:", ok, "| no committed version:", len(none), "| differs:", len(bad))
for r, a, b in bad: print("  diff:", r, a, "->", b)
from collections import Counter
print("uncommitted value profile:", Counter(tuple(sorted(set(c))) for _, c in none))
for r, c in none: print("  none:", r.replace("🧬️mutations/", ""), sorted(set(c)))
