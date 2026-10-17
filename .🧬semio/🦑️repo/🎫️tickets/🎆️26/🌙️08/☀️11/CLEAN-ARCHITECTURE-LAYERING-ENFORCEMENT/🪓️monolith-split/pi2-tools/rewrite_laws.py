#!/usr/bin/env python3
"""🎯️ Re-points the exact-law selectors and `--lib` filters of the facade `📜️script.ts` at the crates that now own the laws (companion test bins or the facade for the stay roots), using the test-path index.
usage: rewrite_laws.py [--apply]"""
import json, re, sys
from lib import *
import lawindex
import ed

SC = PLUG + "/" + PKGDIR + "/📜️script.ts"
idx = lawindex.build()
text = read(SC)
FACADE = "semio-framework-plugin"
GROUP = re.compile(r'\{\s*package:\s*"' + FACADE + r'(?:-tests-\d+)?",\s*(target:\s*\{[^}]*\}),\s*laws:\s*\[((?:\s*"[^"]*"\s*,?)*)\s*\]([^{}]*)\}')
report = []


def suffix(a, b):
    n = 0
    for x, y in zip(reversed(a.split("::")), reversed(b.split("::"))):
        if x != y:
            break
        n += 1
    return n


def pick(old):
    name = old.split("::")[-1]
    c = idx.get(name, [])
    if old in [r[1] for r in c] and len(c) > 1:
        c = [r for r in c if r[1] == old] or c
    if not c:
        report.append(("MISSING", old))
        return None
    best = max(c, key=lambda r: suffix(old, r[1]))
    if sum(1 for r in c if suffix(old, r[1]) == suffix(old, best[1])) > 1:
        report.append(("AMBIGUOUS", old, [r[:2] for r in c]))
    return best[0], best[1]


def regroup(m):
    old = re.findall(r'"([^"]+)"', m.group(2))
    by = {}
    for o in old:
        p = pick(o)
        if p:
            by.setdefault(p[0], []).append(p[1])
            report.append(("MOVE", o, p[0], p[1]))
    return ", ".join('{ package: "%s", %s, laws: [%s]%s}' % (pkg, m.group(1), ", ".join('"%s"' % l for l in laws), m.group(3)) for pkg, laws in by.items())


new = GROUP.sub(regroup, text)
CT = re.compile(r'runRepositoryCargoTests\(\["' + FACADE + r'"\](.*?)\["--lib",\s*"([^"]+)"')


def refilter(m):
    f = m.group(2)
    pk = sorted({p for v in idx.values() for p, l, _ in v if f in l})
    report.append(("FILTER", f, pk))
    return 'runRepositoryCargoTests([%s]%s["--lib","%s"' % (", ".join('"%s"' % p for p in pk) or '"%s"' % FACADE, m.group(1), f)


new = CT.sub(refilter, new)
CT2 = re.compile(r'runRepositoryCargoTests\(\["' + FACADE + r'"\],\s*(.*?)\["--offline","--lib","-E","test\(([^)]*)\)"')


def refilter2(m):
    f = m.group(2)
    pk = sorted({p for v in idx.values() for p, l, _ in v if f in l})
    report.append(("FILTER-E", f, pk))
    return 'runRepositoryCargoTests([%s],%s["--offline","--lib","-E","test(%s)"' % (", ".join('"%s"' % p for p in pk) or '"%s"' % FACADE, m.group(1), f)


new = CT2.sub(refilter2, new)
for r in report:
    print(r)
print("changed", new != text)
if "--apply" in sys.argv and not any(r[0] in ("MISSING", "AMBIGUOUS") for r in report):
    ed.put(SC, new)
    print("applied")
