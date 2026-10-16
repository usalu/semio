"""List every consumer occurrence of os-pack-provided names (tracked + untracked .rs, ticket and generated folders excluded)."""
import re, sys, os, collections
from orclib import *
files = []
for n in ("tracked", "untracked"):
    files += [x for x in open(G + "/orc-rs-%s.z" % n, "rb").read().decode().split("\0") if x]
def skip(f):
    return f.startswith(".🧬semio/") or "/🗑️generated/" in f or f.startswith("🗑️generated")
names = sys.argv[1].split(",")
pat = re.compile(r"\b(" + "|".join(re.escape(n) for n in names) + r")\b")
for f in files:
    if skip(f): continue
    p = ROOT + "/" + f
    try: t = open(p, errors="replace").read()
    except Exception: continue
    if "os_pack" not in t and "pack_rt" not in t: continue
    for i, l in enumerate(t.split("\n"), 1):
        if ("os_pack" in l or "pack_rt" in l) and pat.search(l):
            print("%s:%d: %s" % (f, i, l.strip()[:200]))
