#!/usr/bin/env python3
"""🧪️ Lists the test roots of the facade (`cfg(test)` closure files whose mounting parent is in the lib closure) with their directory prefixes, source and embedded sizes. usage: tops.py <out.json>"""
import collections, json, subprocess, sys
from lib import *
import tree as T
root = PLUG + "/" + PKGDIR + "/🦀️.rs"
order = T.tree(root)
parent = {f: p for f, p, n in order}
name = {f: n for f, p, n in order}
libset = set()
for line in open(GEN + "/pi2w7-r8-closure-lib.tsv"):
    if not line.startswith("TOTAL"):
        libset.add(REPO + "/" + line.split("\t")[3].strip())
def top_of(f):
    while parent.get(f) is not None and parent[f] not in libset:
        f = parent[f]
    return f
members = collections.OrderedDict()
for f, p, n in order:
    if f not in libset:
        members.setdefault(top_of(f), []).append(f)
out = []
for t, fs in members.items():
    dirs = sorted({"/".join(rel(f).split("/")[:2]) if not rel(f).startswith("🪝️mounts/") else "/".join(rel(f).split("/")[:4]) for f in fs})
    out.append(dict(top=rel(t), mod=name[t], parent=rel(parent[t]), files=[rel(f) for f in fs], dirs=dirs, bytes=sum(len(open(f, encoding="utf8").read().encode()) for f in fs), lines=0))
json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print(len(out))
