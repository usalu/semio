"""Find every path/use occurrence `...os_pack::<X>` or `...os_pack::{..X..}` where X is in a given name set, with the preceding path prefix and the file/line (tracked + untracked .rs, ticket + generated folders excluded)."""
import re, sys, collections
from orclib import *
files = []
for n in ("tracked", "untracked"):
    files += [x for x in open(G + "/orc-rs-%s.z" % n, "rb").read().decode().split("\0") if x]
def skip(f):
    return f.startswith(".🧬semio/") or "/🗑️generated/" in f or f.startswith("🗑️generated")
names = set(sys.argv[1].split(","))
pat = re.compile(r"((?:[A-Za-z_][A-Za-z0-9_]*::)*)(?:semio_framework_os_pack|os_pack)\s*::\s*(\{[^}]*\}|[A-Za-z_][A-Za-z0-9_]*(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)*)")
out = []
for f in files:
    if skip(f): continue
    try: t = open(ROOT + "/" + f, errors="replace").read()
    except Exception: continue
    if "os_pack" not in t: continue
    for m in pat.finditer(t):
        pre, g = m.group(1), m.group(2)
        line = t.count("\n", 0, m.start()) + 1
        leafs = []
        if g.startswith("{"):
            for x in g[1:-1].split(","):
                x = x.strip()
                if x: leafs.append(re.split(r"\s+as\s+", x)[0].strip())
        else: leafs.append(re.sub(r"\s+", "", g))
        for lf in leafs:
            first = lf.split("::")[0]
            if first in names:
                out.append((f, line, pre, lf))
for o in out: print("%s:%d\t%s\t%s" % o)
print(len(out), "hits in", len(set(o[0] for o in out)), "files", file=sys.stderr)
