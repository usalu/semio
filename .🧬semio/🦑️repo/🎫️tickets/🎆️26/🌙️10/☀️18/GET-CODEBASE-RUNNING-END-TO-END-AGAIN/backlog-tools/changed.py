"""📋 [backlog] repo-relative FILES that differ between two mirrors (new, changed, removed), walking both trees (no node_modules/.cache/.git), size+mtime first then bytes. usage: changed.py <mirror> [base=blbase]"""
import os, sys, filecmp
home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm")
mir, base = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else "blbase")
SKIP = {"node_modules", ".cache", ".git", ".tmp", ".xplat3-backup"}
def walk(root):
    out = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            p = os.path.join(dp, f)
            try: st = os.lstat(p)
            except OSError: continue
            out[os.path.relpath(p, root)] = (st.st_size, st.st_mtime_ns)
    return out
a, b = walk(f"{home}/{base}"), walk(f"{home}/{mir}")
for rel in sorted(set(a) | set(b)):
    if rel not in a or rel not in b or (a[rel] != b[rel] and not filecmp.cmp(f"{home}/{base}/{rel}", f"{home}/{mir}/{rel}", shallow=False)): print(rel)
