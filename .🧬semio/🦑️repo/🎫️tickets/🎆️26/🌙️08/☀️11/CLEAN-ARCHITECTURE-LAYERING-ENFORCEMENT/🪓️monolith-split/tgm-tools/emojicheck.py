import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *
have = {}
for l in read(F + "/🗑️generated/tgm-emoji-counts.txt").split("\n"):
    if l.strip():
        n, e = l.split(" ", 1)
        have[e.replace("️", "")] = int(n)
for a in sys.argv[1:]:
    e = a.replace("️", "")
    print(a, have.get(e, "NOT-USED"))
