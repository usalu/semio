"""Scoped duplicate-mount scan (module-owner scan, inline-module aware): which Rust files are reachable from more than one package among the manifests under the scope directories.
usage: dupscan.py > report   (scope = the module trees touched by the otcp slice)"""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import modtree
R = "/Users/ueli/Documents/semio/"
O = "🧰️framework/🛍️products/💻️os/"
SCOPE = [O + "🔨️modules/🛎️services", O + "🔨️modules/📇️directory", O + "🔨️modules/📡️spr", O + "🔨️modules/🗣️dsl", O + "🎚️config", "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot", O + "🔨️modules/🎒️pack", O + "🔨️modules/🏪️store/🧩️composition/🗄️durable-group", O + "🔨️modules/🌿️vcs"]
INC = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def manifests():
    for s in SCOPE:
        for dp, dn, fn in os.walk(R + s):
            dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
            if "Cargo.toml" in fn:
                yield os.path.join(dp, "Cargo.toml")


def roots_of(m):
    t = open(m, encoding="utf-8").read()
    d = os.path.dirname(m)
    out = []
    lp = re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', t, re.M | re.S)
    if lp:
        out.append(os.path.normpath(os.path.join(d, lp.group(1))))
    elif os.path.exists(os.path.join(d, "src/lib.rs")):
        out.append(os.path.join(d, "src/lib.rs"))
    for sec in ("bin", "test"):
        for blk in re.finditer(r"^\[\[%s\]\](.*?)(?=^\[|\Z)" % sec, t, re.M | re.S):
            p = re.search(r'^path\s*=\s*"([^"]+)"', blk.group(1), re.M)
            if p:
                out.append(os.path.normpath(os.path.join(d, p.group(1))))
    return [x for x in out if os.path.exists(x)]


def reach(root):
    seen = set()
    queue = [root]
    while queue:
        f = queue.pop()
        if f in seen or not os.path.exists(f):
            continue
        seen.add(f)
        try:
            text = open(f, encoding="utf-8").read()
        except Exception:
            continue
        for mp, g, d, parent in modtree.tree(f):
            if g and g != "MISSING-PATH" and g not in seen:
                queue.append(g)
        for m in INC.finditer(text):
            queue.append(os.path.normpath(os.path.join(os.path.dirname(f), m.group(1))))
    return seen


owners = collections.defaultdict(set)
for m in manifests():
    name = re.search(r'^name = "([^"]+)"', open(m, encoding="utf-8").read(), re.M)
    pkg = name.group(1) if name else m
    for r in roots_of(m):
        for f in reach(r):
            if f.endswith(".rs"):
                owners[f].add(pkg)
multi = {f: sorted(p) for f, p in owners.items() if len(p) > 1}
print(f"packages scanned: {len(set(p for ps in owners.values() for p in ps))}, rust files reached: {len(owners)}, files reached by more than one package: {len(multi)}")
for f, p in sorted(multi.items()):
    print(f.replace(R, "").replace(O, "O/")[-150:], "|", ",".join(x.replace("semio-framework-", "") for x in p))
