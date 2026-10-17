#!/usr/bin/env python3
"""🔢️ Gives the routes of the new companion bins order keys that no other route uses (identical in both launch files)."""
import re, sys, os, collections
from decimal import Decimal
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import ed
FILES = [REPO + "/.vscode/launch.json", REPO + "/.vscode/🧩️launch.seed.jsonc"]
NEW = [8, 9] + list(range(22, 30))
texts = {f: read(f) for f in FILES}
BLOCK = re.compile(r'(\{\s*"name": "🦀️ semio-framework-plugin-tests-(\d+) (🏗️build|🔎️check|🧪️test) (🛠️prototype|🚀️release)",.*?"order":\s*)([0-9.]+)', re.S)
used = collections.Counter()
for f, t in texts.items():
    for m in re.finditer(r'"order":\s*([0-9.]+)', t):
        used[m.group(1)] += 1
mine = {}
for f, t in texts.items():
    for m in BLOCK.finditer(t):
        n = int(m.group(2))
        if n in NEW:
            mine[(n, m.group(3), m.group(4))] = m.group(5)
taken = {o for o, c in used.items()}
for v in mine.values():
    taken.discard(v) if used[v] <= 2 else None
others = {o for f, t in texts.items() for o in re.findall(r'"order":\s*([0-9.]+)', t)}
mineset = set(mine.values())
# orders used by anything other than my own routes
foreign = collections.Counter()
for f, t in texts.items():
    spans = [(m.start(), m.end()) for m in BLOCK.finditer(t) if int(m.group(2)) in NEW]
    for m in re.finditer(r'"order":\s*([0-9.]+)', t):
        if not any(a <= m.start(5 if False else 0) < b for a, b in spans):
            foreign[m.group(1)] += 1
base = {"🏗️build": Decimal("900.3017"), "🔎️check": Decimal("922.0637"), "🧪️test": Decimal("903.6029")}
alloc = {}
cursor = {k: v for k, v in base.items()}
def free(d):
    s = format(d.normalize(), "f")
    return s not in foreign
for n in NEW:
    for tgt in ("🏗️build", "🔎️check", "🧪️test"):
        for conf in ("🛠️prototype", "🚀️release"):
            while not free(cursor[tgt]):
                cursor[tgt] += Decimal("0.00001")
            alloc[(n, tgt, conf)] = format(cursor[tgt].normalize(), "f")
            foreign[alloc[(n, tgt, conf)]] += 1
            cursor[tgt] += Decimal("0.00001")
for f, t in texts.items():
    def rep(m):
        n = int(m.group(2))
        if n in NEW:
            return m.group(1) + alloc[(n, m.group(3), m.group(4))]
        return m.group(0)
    new = BLOCK.sub(rep, t)
    print(f[-30:], "changed" if new != t else "same")
    if "--apply" in sys.argv and new != t:
        assert read(f) == t, "a peer edited " + f
        ed.put(f, new)
print({k: v for k, v in list(alloc.items())[:6]})
