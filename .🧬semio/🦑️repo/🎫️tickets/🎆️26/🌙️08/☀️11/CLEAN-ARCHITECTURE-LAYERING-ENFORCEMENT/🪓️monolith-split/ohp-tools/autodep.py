"""➕ Declares workspace dependencies that crate sources name but the manifest lacks (oracle E0433/E0432 "cannot find module or crate X").
Only when (a) X is an indexed workspace crate, (b) the user's workspace root has a [workspace.dependencies] entry for X, (c) adding the edge creates no normal-dependency cycle. Cycles and entries without a workspace row are reported, not edited.
usage: autodep.py <final.err>... [--apply]"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/pos")
os.environ["POS_PREFIX"] = "ohp"
from poslib import *
ROOT = "/Users/ueli/Documents/semio"
mans = Manifests(Source(None)); mans.build_index()
IDX = mans.by_name
BYU = {n.replace("-", "_"): n for n in IDX}
apply = "--apply" in sys.argv
files = [x for x in sys.argv[1:] if not x.startswith("--")]
pairs = collections.OrderedDict()
for p in [x for x in files if os.path.exists(x)]:
    for line in open(p, encoding="utf-8", errors="replace"):
        if not line.startswith("{"): continue
        try: d = json.loads(line)
        except Exception: continue
        if d.get("level") != "error": continue
        m = re.search(r"cannot find (?:module or crate|crate) `([A-Za-z0-9_]+)` in this scope|use of unresolved module or unlinked crate `([A-Za-z0-9_]+)`|use of undeclared crate or module `([A-Za-z0-9_]+)`|^unresolved import `([A-Za-z0-9_]+)`$|^unresolved import `([A-Za-z0-9_]+)::", d["message"])
        if not m: continue
        x = next(g for g in m.groups() if g)
        if x not in BYU or x == "semio_framework_plugin": continue
        pr = [s for s in d["spans"] if s["is_primary"]]
        if not pr: continue
        fm = re.match(r"^__m_([A-Za-z0-9_]+)/", pr[0]["file_name"])
        if not fm: continue
        pairs[(BYU.get(fm.group(1)), BYU[x])] = 1
def deps_of(pkg):
    m = mans.manifest(pkg)
    return [] if not m else [p for kind, name, p, f, o, r in mans.deps(m) if kind in ("dependencies", "build-dependencies")]
def reaches(src, dst):
    seen, st = set(), [src]
    while st:
        c = st.pop()
        if c == dst: return True
        if c in seen: continue
        seen.add(c); st += deps_of(c)
    return False
def declared(user, dep):
    m = mans.manifest(user)
    return any(p == dep for kind, name, p, f, o, r in mans.deps(m))
def ws_has(user, dep):
    mrel = IDX[user]
    t = open(os.path.join(ROOT, mrel), encoding="utf-8").read()
    wm = re.search(r'^workspace\s*=\s*"([^"]+)"', t, re.M)
    wdir = os.path.normpath(os.path.join(os.path.dirname(mrel), wm.group(1))) if wm else os.path.dirname(mrel)
    wp = os.path.join(ROOT, wdir, "Cargo.toml")
    if not os.path.exists(wp): return False
    wt = open(wp, encoding="utf-8").read()
    return bool(re.search(r"^\[workspace\.dependencies\." + re.escape(dep) + r"\]", wt, re.M) or re.search(r"^" + re.escape(dep) + r"\s*=", wt, re.M))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
done = collections.Counter()
for (user, dep) in pairs:
    if user is None or user == dep: continue
    if declared(user, dep): done["already-declared (oracle artifact)"] += 1; continue
    if reaches(dep, user): print(f"CYCLE  {user} -> {dep}"); done["cycle"] += 1; continue
    spec = None
    if not ws_has(user, dep):
        rel = os.path.relpath(os.path.dirname(os.path.join(ROOT, IDX[dep])), os.path.dirname(os.path.join(ROOT, IDX[user])))
        spec = '{ path = "' + rel + '" }'
        print(f"ADD-PATH {user} -> {dep}"); done["add by path"] += 1
    else:
        print(f"ADD    {user} -> {dep}"); done["add"] += 1
    if apply:
        import subprocess
        subprocess.run(["python3", os.path.dirname(os.path.abspath(__file__)) + "/adddep.py", os.path.join(ROOT, IDX[user]), dep] + ([spec] if spec else []), check=True)
print(dict(done), "APPLIED" if apply else "dry")
