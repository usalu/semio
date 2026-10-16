"""Definition scan: names defined in the os-pack tree and in the framework pack tree (fn, struct, enum, trait, const, static, type, macro_rules; production files and test files apart).
Usage: defscan.py before|after   (before = frozen snapshot of the os-pack tree, after = the live tree)"""
import os, re, sys, collections
from orclib import *
SNAP = G + "/orc-work/snap/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack"
which = sys.argv[1]
osroot = SNAP if which == "before" else OS
pat = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(fn|struct|enum|trait|const|static|type|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)")
def scan(root, skip=()):
    d = collections.defaultdict(list)
    for r, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in skip and x != "🗑️generated"]
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(r, f)
            rel = os.path.relpath(p, root)
            test = "🧪️tests" in rel.split(os.sep) or "🧪️testing" in rel.split(os.sep)
            for i, l in enumerate(open(p, errors="replace"), 1):
                m = pat.match(l)
                if m: d[m.group(2)].append((m.group(1), rel, i, test))
    return d
a = scan(osroot)
b = scan(FW)
shared_prod = sorted(n for n in set(a) & set(b) if any(not t for *_, t in a[n]) and any(not t for *_, t in b[n]))
shared_all = sorted(set(a) & set(b))
print("os-pack names", len(a), "framework pack names", len(b))
print("shared names (production on both sides):", len(shared_prod))
print("shared names (any file):", len(shared_all))
for n in shared_prod:
    ao = [(k, p.replace("🌱️value/", "V/")[:70], i) for k, p, i, t in a[n] if not t]
    bo = [(k, p.replace("🌱️value/", "V/")[:60], i) for k, p, i, t in b[n] if not t]
    print("  %-40s os=%s fw=%d" % (n, ao[:2], len(bo)))
