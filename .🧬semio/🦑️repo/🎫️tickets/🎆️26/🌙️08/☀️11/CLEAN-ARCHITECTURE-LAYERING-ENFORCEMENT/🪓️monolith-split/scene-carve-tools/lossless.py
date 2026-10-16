"""🧾 Losslessness: compares the non-blank line multisets of every Rust file of the baseline scene tree and of the carved one."""
import os, collections, sys
from lib import *

def lines(root):
    c = collections.Counter()
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                for l in rd(os.path.join(dp, f)).split("\n"):
                    if l.strip():
                        c[l.rstrip()] += 1
    return c

b = lines(BASE)
n = lines(REPO + "/" + SCENE_REL)
gone = b - n
added = n - b
print("baseline lines", sum(b.values()), "new lines", sum(n.values()), "lost", sum(gone.values()), "added", sum(added.values()))
if "-v" in sys.argv:
    print("--- LOST"); [print(c, l[:170]) for l, c in sorted(gone.items())]
    print("--- ADDED"); [print(c, l[:170]) for l, c in sorted(added.items())]
