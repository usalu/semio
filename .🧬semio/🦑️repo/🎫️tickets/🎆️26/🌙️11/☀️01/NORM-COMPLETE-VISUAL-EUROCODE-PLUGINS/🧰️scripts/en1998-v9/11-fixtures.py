"""en1998 v9 step 11: fixtures. wallStoreyId 1.0 -> "" (every storey), 2.0 -> "s2"; liquefiableLayer 0.0 -> false, positive -> true. python -I"""
import os, re
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import ANY
RULES = [
 (re.compile(r'("(?:new)?[wW]allStoreyId": ?)1\.0\b'), r'\1""'),
 (re.compile(r'("(?:new)?[wW]allStoreyId": ?)2\.0\b'), r'\1"s2"'),
 (re.compile(r'("(?:new)?[lL]iquefiableLayer": ?)0\.0\b'), r'\1false'),
 (re.compile(r'("(?:new)?[lL]iquefiableLayer": ?)1\.5\b'), r'\1true'),
]
files = changed = 0
for dp, dn, fs in os.walk(ANY):
    dn[:] = [d for d in dn if d not in ("node_modules", "target")]
    for f in fs:
        if not f.endswith(".json"): continue
        p = os.path.join(dp, f)
        t = open(p, encoding="utf-8").read(); n = t
        for r, s in RULES: n = r.sub(s, n)
        files += 1
        if n != t: open(p, "w", encoding="utf-8").write(n); changed += 1
print(files, changed)
