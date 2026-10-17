"""👻️ Test files of the renderer engine folder that no package root reaches, and test files reached only by the top crate's lib-test unit.
usage: orphans.py"""
import collections, re
from lib import *
import dupscan as D

owners = collections.defaultdict(set)
for m in D.manifests():
    mm = re.search(r'^name = "([^"]+)"', read(m), re.M)
    name = mm.group(1) if mm else m
    for r in D.roots_of(m):
        for f in D.reach(r):
            if f.endswith(".rs"):
                owners[f].add(name)
TEST = re.compile(r"#\[\s*(?:[\w]+::)*(?:async_)?test(?:_case)?\s*(?:\([^\]]*\))?\s*\]")
orph, top_only = [], []
for dp, dn, fn in os.walk(E):
    dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated", ".git")]
    for f in fn:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(dp, f)
        if not TEST.search(blank(read(p))):
            continue
        o = owners.get(p, set())
        if not o:
            orph.append(p)
        elif o == {"semio-framework-os-renderer-wgpu"}:
            top_only.append(p)
print("test files", sum(1 for _ in orph) , "orphans:", len(orph))
for p in orph:
    print("  ORPHAN", rel(p)[-110:])
print("reached only by the top crate:", len(top_only))
for p in top_only:
    print("  TOP", rel(p)[-110:])
