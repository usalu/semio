"""Which non-pub production definitions do the test files of a crate name? Read-only.
usage: usage.py <crate-id>   (reads 🗑️generated/otcp-tree-<id>.txt for the module tree)"""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
from defs import scan, struct_fields, IDENT
from stubgen import mask
from splitter import tokenize, member_refs, KEYWORDS

R = "/Users/ueli/Documents/semio/"
F = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
crate = sys.argv[1]
rows = []
for line in open(F + f"🗑️generated/otcp-tree-{crate}.txt", encoding="utf8"):
    parts = line.rstrip("\n").split(" | ")
    if len(parts) < 5: continue
    rows.append(parts)
import subprocess
# re-run modtree for absolute paths
sys.path.insert(0, os.path.dirname(__file__))
import modtree


def lib_of(crate):
    manifests = open(F + "🗑️generated/otcp-manifests.txt", encoding="utf8").read().strip().split(";")
    for m in manifests:
        txt = open(R + m, encoding="utf8").read()
        nm = re.search(r'^name\s*=\s*"([^"]+)"', txt, re.M).group(1)
        if nm == "semio-framework-" + crate:
            lp = re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', txt, re.M | re.S)
            return os.path.normpath(os.path.join(os.path.dirname(R + m), lp.group(1) if lp else "src/lib.rs"))


lib = lib_of(crate)
nodes = modtree.tree(lib)
files = {lib: ()}
test_files = set()
test_mount = {}
for mp, f, d, parent in nodes:
    if f and f != "MISSING-PATH" and os.path.exists(f):
        files[f] = mp
        under_test = any("cfg(test)" in a.replace(" ", "") for a in d["attrs"])
        if under_test or (mp[:-1] and tuple(mp[:-1]) in test_mount):
            test_files.add(f)
            test_mount[mp] = f
prod_files = [f for f in files if f not in test_files]
index_items = collections.defaultdict(list)
index_methods = collections.defaultdict(list)
index_fields = collections.defaultdict(list)
for f in prod_files:
    t = open(f, encoding="utf-8").read()
    s = scan(t)
    m = s["m"]
    for it in s["items"]:
        if it.get("name") and it["kind"] != "mod":
            index_items[it["name"]].append((f, it["vis"], it["kind"], it["ctx"]))
        if it["kind"] in ("struct", "union"):
            for (nm, vis, vp, np_) in struct_fields(m, it, t):
                index_fields[nm].append((f, it.get("name"), vis))
    for mem in s["members"]:
        if mem["kind"] == "fn" and mem.get("name") and "trait" not in mem["ctx"]:
            index_methods[mem["name"]].append((f, mem["ctx"], mem["vis"]))
        elif mem.get("name") and "trait" not in mem["ctx"]:
            index_methods[mem["name"]].append((f, mem["ctx"], mem["vis"]))
rep = collections.defaultdict(lambda: collections.defaultdict(set))
for f in sorted(test_files):
    t = open(f, encoding="utf-8").read()
    first, allt = tokenize(t)
    fields, methods = member_refs(t)
    own = scan(t)
    own_names = {x.get("name") for x in own["items"] if x.get("name")} | {x.get("name") for x in own["members"] if x.get("name")}
    short = f.replace(R, "").replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "M/")
    for name in sorted(allt):
        if name in own_names or name in KEYWORDS: continue
        for (pf, vis, kind, ctx) in index_items.get(name, []):
            if vis != "pub":
                rep[short]["item"].add(f"{name}[{kind},{vis or 'priv'}]@{pf.split('/')[-2]}")
    for name in sorted(methods):
        if name in own_names: continue
        for (pf, ctx, vis) in index_methods.get(name, []):
            if vis != "pub":
                rep[short]["method"].add(f"{name}[{ctx},{vis or 'priv'}]@{pf.split('/')[-2]}")
    for name in sorted(fields):
        if name in own_names: continue
        for (pf, st, vis) in index_fields.get(name, []):
            if vis != "pub":
                rep[short]["field"].add(f"{st}.{name}[{vis or 'priv'}]@{pf.split('/')[-2]}")
for f, d in rep.items():
    print("##", f[-100:])
    for k in ("item", "method", "field"):
        if d.get(k): print("  ", k, sorted(d[k]))
