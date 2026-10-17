"""🧾️ Files of the renderer engine folder added, removed or changed since the pre-edit snapshot, with line counts. usage: treediff.py [--list]"""
import collections, os, sys
from lib import *

def walk(root):
    out = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated", ".git")]
        for f in fn:
            p = os.path.join(dp, f)
            out[os.path.relpath(p, root)] = p
    return out

a, b = walk(SNAP + "/E"), walk(E)
added = sorted(set(b) - set(a))
removed = sorted(set(a) - set(b))
changed = []
for k in sorted(set(a) & set(b)):
    try:
        if open(a[k], "rb").read() != open(b[k], "rb").read():
            changed.append(k)
    except Exception:
        pass
print(f"added {len(added)} removed {len(removed)} changed {len(changed)}")
if "--list" in sys.argv:
    for tag, rows in (("A", added), ("D", removed), ("M", changed)):
        for r in rows:
            print(tag, r)
