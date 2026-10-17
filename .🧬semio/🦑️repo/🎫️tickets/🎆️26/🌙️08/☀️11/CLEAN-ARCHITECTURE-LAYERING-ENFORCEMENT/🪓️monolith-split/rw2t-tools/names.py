"""🧾 rw2t: test-name multiset of the engine folder; snapshot before, compare after."""
import sys, json, collections, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
from lib import *
import testnames
SNAPF = GEN + "/rw2t-testnames-before.json"
def snap():
    c = testnames.scan(E)
    json.dump([[k[0], k[1], v] for k, v in c.items()], open(SNAPF, "w"), ensure_ascii=False)
    print("snapshot", sum(c.values()), "tests in", len({k[0] for k in c}), "files")
def compare():
    a = collections.Counter({(r[0], r[1]): r[2] for r in json.load(open(SNAPF))})
    b = testnames.scan(E)
    na, nb = collections.Counter(), collections.Counter()
    for k, v in a.items(): na[k[1]] += v
    for k, v in b.items(): nb[k[1]] += v
    print("before", sum(a.values()), "after", sum(b.values()))
    print("by name: only before", sum((na - nb).values()), "only after", sum((nb - na).values()))
    for k in sorted(na - nb): print("  - name", k)
    for k in sorted(nb - na): print("  + name", k)
    fa, fb = a - b, b - a
    print("by (file,name): only before", sum(fa.values()), "only after", sum(fb.values()))
    for k, v in sorted(fa.items())[:40]: print("  - file", k)
    for k, v in sorted(fb.items())[:40]: print("  + file", k)
    print("IDENTICAL BY NAME" if not (na - nb) and not (nb - na) else "DIFFERENT BY NAME")
if __name__ == "__main__":
    {"snap": snap, "compare": compare}[sys.argv[1]]()
