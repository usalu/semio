"""Longest repo-relative path (UTF-8 bytes) among the files under the renderer engine folder that are new or differ from the snapshot."""
import os, sys
from lib import *
rows = []
for dp, dn, fn in os.walk(E):
    dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
    for f in fn:
        p = os.path.join(dp, f)
        r = os.path.relpath(p, REPO)
        snap = SNAP + "/E/" + os.path.relpath(p, E)
        if not os.path.exists(snap) or open(snap, "rb").read() != open(p, "rb").read():
            rows.append((len(r.encode("utf8")), r))
rows.sort(reverse=True)
print(len(rows), "new or changed files; longest:", rows[0][0] if rows else 0)
for n, r in rows[:5]:
    print(n, r[-100:])
