"""Map the monolith-era fem 2d verification-law names to the current crate/target/module paths. usage: laws_map.py <out.json>"""
import sys, os, re, json, collections
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
from modtree import walk, crate_root, Node
F2 = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
FIX = F2 + "/📦️packages/🦀️rust/🧫️fixtures/⚖️verification-laws/🔣️.json"
laws = json.load(open(ROOT + "/" + FIX, encoding="utf8"))
fn_re = re.compile(r'#\[(?:semio_framework_async_macros::async_test|test)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]+)')
def members(ws):
    t = rd(ws)
    m = re.search(r'(?ms)^members\s*=\s*\[(.*?)\]', t)
    return re.findall(r'"([^"]+)"', m.group(1))
roots = [F2 + "/Cargo.toml"]
file_fns = {}     # abs file -> set(fn)
mounts = collections.defaultdict(list)   # abs file -> [(package, kind, target, modpath)]
fn_files = collections.defaultdict(set)
def scan(pkg, kind, tname, root_abs):
    nodes = []
    walk(root_abs, os.path.dirname(root_abs), (), nodes)
    files = {root_abs: ()}
    for n in nodes:
        if n.file and n.file not in files:
            files[n.file] = n.modpath
    for f, mp in files.items():
        if not f.endswith(".rs") or not os.path.exists(f): continue
        if f not in file_fns:
            file_fns[f] = set(fn_re.findall(open(f, encoding="utf8").read()))
            for fn in file_fns[f]: fn_files[fn].add(f)
        if file_fns[f]:
            mounts[f].append((pkg, kind, tname, "::".join(mp)))
for ws in roots:
    wsdir = os.path.dirname(ws)
    for mem in members(ws):
        mf = wsdir + "/" + mem + "/Cargo.toml"
        if not os.path.exists(ROOT + "/" + mf): continue
        t = rd(mf)
        pkg = re.search(r'^name = "([^"]+)"', t, re.M).group(1)
        d = os.path.dirname(mf)
        libp = re.search(r'(?ms)^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', t)
        lib = os.path.normpath(os.path.join(ROOT, d, libp.group(1) if libp else "src/lib.rs"))
        if os.path.exists(lib):
            libname = re.search(r'(?ms)^\[lib\][^\[]*?^name\s*=\s*"([^"]+)"', t)
            scan(pkg, "lib", libname.group(1) if libname else pkg.replace("-", "_"), lib)
        for m in re.finditer(r'(?ms)^\[\[test\]\]\n(.*?)(?=^\[|\Z)', t):
            nm = re.search(r'name\s*=\s*"([^"]+)"', m.group(1)); pp = re.search(r'path\s*=\s*"([^"]+)"', m.group(1))
            if nm and pp:
                scan(pkg, "test", nm.group(1), os.path.normpath(os.path.join(ROOT, d, pp.group(1))))
GENERIC = {"editor", "fem2d", "tests", "unit_tests", "component", "standards", "v1", "subsets", "any", "unit", "context", "schema", "io", "text", "binary"}
def slugs(path):
    out = set()
    for seg in path.split("/"):
        s = re.sub(r'[^\x00-\x7f]', '', seg).strip("-_. ").replace("-", "_").lower()
        if s: out.add(s)
    return out
def score(old, f, mount=None):
    parts = old.split("::")
    oseg = [x for x in parts[:-1] if x not in GENERIC]
    sl = slugs(f)
    sc = sum(1 for x in oseg if x in sl) * 10 - len(f) / 1000.0
    if mount is not None:
        last = mount[3].split("::")[-1] if mount[3] else ""
        for i, seg in enumerate(parts[:-1]):
            if seg.startswith("tests_") and i > 0:
                want = parts[i - 1] + "_" + seg[len("tests_"):]
                if last == want or want.startswith(last) or last.startswith(want):
                    sc += 50
        if mount[3] and mount[3].split("::")[-1] in parts[:-1]:
            sc += 5
    return sc
groups = collections.OrderedDict()
unresolved = []
for seg, names in laws.items():
    g = collections.OrderedDict()
    for old in names:
        fn = old.split("::")[-1]
        fs = fn_files.get(fn, set())
        if not fs:
            unresolved.append(old); continue
        best = max(((f, m) for f in sorted(fs) for m in mounts[f]), key=lambda fm: (score(old, fm[0], fm[1]), fm[1][1] == "lib"))
        f, (pkg, kind, tname, mp) = best
        new = (mp + "::" if mp else "") + fn
        g.setdefault((pkg, kind, tname), []).append(new)
    groups[seg] = [{"package": k[0], "target": ({"kind": "lib", "name": k[2]} if k[1] == "lib" else {"kind": "test", "name": k[2]}), "laws": v} for k, v in g.items()]
json.dump({"groups": groups, "unresolved": unresolved}, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print({k: (len(v), sum(len(x["laws"]) for x in v), sum(len(set(x["laws"])) for x in v)) for k, v in groups.items()}, "unresolved", len(unresolved))
print({(g["package"], g["target"]["name"]): len(g["laws"]) for g in groups["native"]})
print(unresolved[:15])
