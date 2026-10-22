"""🧪 Overlay + family specs for the oracle differential of macro plans: pre family = hub + producers + sample importers (original sources), post family = new macro crates + producers + the same importers (plans applied). `python3 mkoverlay2.py <tag> <hubpkg> <plan,plan,..> <macro-plan> <importer,importer,..>` -> /private/tmp/gran-hubs/overlay-<tag>/{pre,post}, fam/<tag>-pre.json, fam/<tag>-post.json"""
from hubenv import *
import shutil, subprocess, re
tag, hubpkg, plantags, macroplan, imps = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5].split(",")
sys.path.insert(0, os.path.normpath(T + "/../staged"))
import apply as A
O = "/private/tmp/gran-hubs"
plans = {t: json.load(open(os.path.normpath(T + "/../staged/plans/%s.json" % t))) for t in plantags.split(",")}
creates = {c["path"]: c["content"] for p in plans.values() for c in p.get("creates", [])}
newcr = {}
for rel, content in creates.items():
    if rel.endswith("Cargo.toml"):
        nm = re.search(r'^name\s*=\s*"([^"]+)"', content, re.M).group(1)
        newcr[nm] = os.path.dirname(rel)
hub = byname[hubpkg]
producers = set()
for nm, d in newcr.items():
    for m in re.finditer(r'^([a-z0-9-]+) = \{ path = "([^"]+)"', creates[d + "/Cargo.toml"], re.M):
        producers.add(m.group(1))
for t, p in plans.items():
    for c in imps:
        if c in p["crates"]:
            pass
imp_dirs = {c: byname[c] for c in imps}
census = json.load(open(OUT + "/census-%s.json" % ("plugin" if "plugin" in hubpkg else "os-kernel")))
# producers of the importers: the idents the plan resolved (owners) by package name
byident = {}
world, I2L, I2D, LIB = make_world()
for t, p in plans.items():
    for c in imps:
        if c in p["crates"]:
            for i in p["crates"][c]["owners_prod"] + p["crates"][c]["owners_test"]:
                if i in I2D: producers.add(C[I2D[i]]["name"])
fam_pre_dirs = {hub} | {byname[n] for n in producers if n in byname} | set(imp_dirs.values())
fam_post_dirs = {byname[n] for n in producers if n in byname} | set(imp_dirs.values())
def files_of(d):
    c = C[d]; fs = set(module_tree(libfile(d, c)))
    t = open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8").read()
    for m in re.finditer(r'^\s*path\s*=\s*"([^"]+\.rs)"', t, re.M): fs |= set(module_tree(os.path.normpath(os.path.join(ROOT, d, m.group(1)))))
    return fs
base = O + "/overlay-" + tag
shutil.rmtree(base, ignore_errors=True)
for side, dirs in (("pre", fam_pre_dirs), ("post", fam_post_dirs | {hub})):
    for d in dirs:
        for f in files_of(d) | {os.path.join(ROOT, d, "Cargo.toml")}:
            if os.path.isfile(f):
                dst = os.path.join(base, side, os.path.relpath(f, ROOT)); os.makedirs(os.path.dirname(dst), exist_ok=True)
                if not os.path.exists(dst): shutil.copy2(f, dst)
# macro crate sources that exist today (hub-mounted files) also belong to pre/post trees
only = ",".join(list(imps) + [n for n in newcr] + [hubpkg])
r = subprocess.run([sys.executable, os.path.normpath(T + "/../staged/apply.py"), "--plan", plantags, "--root", base + "/post", "--write", "--ledger", base + "/ledger", "--only-crates", only], capture_output=True, text=True)
print(r.stdout[-500:], r.stderr[-300:])
os.makedirs(O + "/fam", exist_ok=True)
json.dump({"name": tag + "pre", "crates": sorted(C[d]["name"] for d in fam_pre_dirs)}, open(O + "/fam/%s-pre.json" % tag, "w"))
json.dump({"name": tag + "post", "crates": sorted([C[d]["name"] for d in fam_post_dirs] + list(newcr))}, open(O + "/fam/%s-post.json" % tag, "w"))
print("pre", len(fam_pre_dirs), "post", len(fam_post_dirs) + len(newcr), "producers", len(producers))
