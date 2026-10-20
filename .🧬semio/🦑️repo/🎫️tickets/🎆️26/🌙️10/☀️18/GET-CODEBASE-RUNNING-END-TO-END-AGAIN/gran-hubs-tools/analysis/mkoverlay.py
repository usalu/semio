"""🧪 Builds the pre/post overlays for the oracle differential of a facade repoint plan: a sample of planned importer crates (greedy cover of the producer crates they gain), the hub and all producers they name; copies the module-tree files and manifests of the family into /private/tmp/gran-hubs/overlay/{pre,post}, applies the plan to `post` (only the sampled crates) and writes the family spec. `python3 mkoverlay.py <tag> <hub> [N]`"""
from hubenv import *
import shutil, subprocess
tag, hub = sys.argv[1], sys.argv[2]
N = int(sys.argv[3]) if len(sys.argv) > 3 else 14
plan = json.load(open(os.path.normpath(T + "/../staged/plans/%s.json" % tag)))
world, I2L, I2D, LIB = make_world()
HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
O = "/private/tmp/gran-hubs"
def files_of(d):
    c = C[d]
    fs = set(module_tree(libfile(d, c)))
    t = open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8").read()
    for m in re.finditer(r'^\s*path\s*=\s*"([^"]+\.rs)"', t, re.M):
        fs |= set(module_tree(os.path.normpath(os.path.join(ROOT, d, m.group(1)))))
    return fs
import re
cands = []
for name, c in plan["crates"].items():
    d = c["dir"]
    n = len(files_of(d))
    if n <= 40:
        cands.append((name, d, n, {I2D[i] for i in c["owners_prod"] + c["owners_test"]}))
cands.sort(key=lambda x: (x[2], x[0]))
chosen, covered = [], set()
for _ in range(N):
    best = max(cands, key=lambda x: len(x[3] - covered), default=None)
    if not best or not (best[3] - covered):
        break
    chosen.append(best); covered |= best[3]; cands.remove(best)
fam_dirs = {HD} | covered | {d for _, d, _, _ in chosen}
print("importers", [c[0] for c in chosen]); print("producers", len(covered), "family crates", len(fam_dirs))
shutil.rmtree(O + "/overlay", ignore_errors=True)
for side in ("pre", "post"):
    for d in fam_dirs:
        for f in files_of(d) | {os.path.join(ROOT, d, "Cargo.toml")}:
            if not os.path.isfile(f): continue
            dst = os.path.join(O, "overlay", side, os.path.relpath(f, ROOT))
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            if not os.path.exists(dst): shutil.copy2(f, dst)
os.makedirs(O + "/fam", exist_ok=True)
json.dump({"name": hub + "fam", "crates": sorted(C[d]["name"] for d in fam_dirs)}, open(O + "/fam/%s.json" % tag, "w"))
json.dump([c[0] for c in chosen], open(O + "/fam/%s.importers.json" % tag, "w"))
r = subprocess.run([sys.executable, os.path.normpath(T + "/../staged/apply.py"), "--plan", tag, "--root", O + "/overlay/post", "--write", "--ledger", O + "/ledger-" + tag, "--only-crates", ",".join(c[0] for c in chosen)], capture_output=True, text=True)
print(r.stdout[-600:], r.stderr[-300:])
