"""Find uses of removed os-pack names through the os-kernel flat root (`store::NAME`, `semio_framework_os_kernel::NAME`, `dsl::NAME`) and brace imports from those roots."""
import re, sys
from orclib import *
files = []
for n in ("tracked", "untracked"):
    files += [x for x in open(G + "/orc-rs-%s.z" % n, "rb").read().decode().split("\0") if x]
names = set(sys.argv[1].split(","))
roots = sys.argv[2].split(",")
pat = re.compile(r"(?<![\w:])(" + "|".join(map(re.escape, roots)) + r")\s*::\s*(\{[^}]*\}|[A-Za-z_]\w*(?:::[A-Za-z_]\w*)?)")
for f in files:
    if f.startswith(".🧬semio/") or "/🗑️generated/" in f: continue
    try: t = open(ROOT + "/" + f, errors="replace").read()
    except Exception: continue
    if not any(r in t for r in roots): continue
    for m in pat.finditer(t):
        g = m.group(2)
        leafs = [x.strip() for x in g[1:-1].split(",")] if g.startswith("{") else [g]
        for lf in leafs:
            lf0 = re.split(r"\s+as\s+", lf)[0].strip().split("::")[0]
            if lf0 in names:
                print("%s:%d\t%s\t%s" % (f, t.count("\n", 0, m.start()) + 1, m.group(1), lf))
