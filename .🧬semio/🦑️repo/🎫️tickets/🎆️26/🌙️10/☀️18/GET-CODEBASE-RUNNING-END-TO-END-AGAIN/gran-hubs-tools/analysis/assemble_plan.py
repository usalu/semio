"""🧵 Assembles the final staged plan of a facade repoint: raw source edits (plan_hub.py) + manifest edits (manifest_edit.py), crate-atomic with a fixpoint over files shared by several crates. Output: staged/plans/<hub>.json. `python3 assemble_plan.py <hub> [--limit N] [--crates a,b]`"""
from hubenv import *
import hashlib, re, time
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from manifest_edit import plan_manifest, line_ops, apply_ops

hub = sys.argv[1]
limit = int(sys.argv[sys.argv.index("--limit") + 1]) if "--limit" in sys.argv else None
pick = set(sys.argv[sys.argv.index("--crates") + 1].split(",")) if "--crates" in sys.argv else None
tag = sys.argv[sys.argv.index("--tag") + 1] if "--tag" in sys.argv else hub
raw = json.load(open(OUT + "/plan-%s.raw.json" % hub))
MAN = json.load(open(OUT + "/manifests.json"))
ROOTS = json.load(open(OUT + "/manifests-roots.json"))
_, I2L, I2D, LIB = make_world()
HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
HP = C[HD]["name"]


def sha(b):
    return hashlib.sha256(b).hexdigest()


crates = dict(raw["crates"])
if pick:
    crates = {d: c for d, c in crates.items() if c["name"] in pick}
if limit:
    crates = dict(list(crates.items())[:limit])
manifests, rejected = {}, {}
CENS = json.load(open(OUT + "/census-%s.json" % hub))["deps"]
for d, c in list(crates.items()):
    if CENS.get(d, {}).get("macros"):
        rejected[c["name"]] = "invokes a macro that expands to hub paths: " + ",".join(CENS[d]["macros"][:3])
        del crates[d]
for d, c in list(crates.items()):
    mp = os.path.join(ROOT, d, "Cargo.toml")
    text = open(mp, "rb").read().decode("utf-8")
    if "\r" in text:
        rejected[c["name"]] = "CRLF manifest"
        continue
    keys = sorted({x["key"] for x in MAN[d]["deps"] if x["pkg"] == HP and x["kind"] in ("normal", "build", "dev")})
    if not keys:
        rejected[c["name"]] = "hub key not in manifest"
        continue
    wsroot = MAN[d]["wsroot"]
    declared = ROOTS.get(wsroot, {})
    op = sorted({C[I2D[i]]["name"] for i in c["owners_prod"]})
    ot = sorted({C[I2D[i]]["name"] for i in c["owners_test"]})
    byname_dir = {C[x]["name"]: x for x in C}

    def ws_declared(pkg, declared=declared):
        e = declared.get(pkg)
        return bool(e) and e.get("path") is not None and not e.get("package")

    def rel_of(pkg, d=d):
        return os.path.relpath(byname_dir[pkg], d).replace(os.sep, "/")

    hn = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] in ("normal", "build")}
    hd = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] == "dev"}
    new = text
    err = None
    for key in keys:
        new, err = plan_manifest(new, key, op, ot, ws_declared, rel_of, hn, hd)
        if err:
            break
    if err:
        rejected[c["name"]] = err
        continue
    if any(len(rel_of(p)) > 200 for p in op + ot):
        rejected[c["name"]] = "relative path longer than 200 characters"
    ops = line_ops(text, new)
    if apply_ops(text, ops) != new:
        rejected[c["name"]] = "manifest ops are not exact-once"
        continue
    manifests[d] = {"path": os.path.relpath(mp, ROOT), "sha256": sha(text.encode()), "new_text": new, "sha256_after": sha(new.encode()), "ops": ops, "crate": c["name"]}
rej_names = set(rejected)
changed = True
files = raw["files"]
while changed:
    changed = False
    for rel, f in files.items():
        if any(cn in rej_names for cn in f["crates"]):
            for cn in f["crates"]:
                if cn not in rej_names and cn in {c["name"] for c in crates.values()}:
                    rej_names.add(cn)
                    rejected[cn] = "shares a file with a rejected crate"
                    changed = True
ok_crates = {d: c for d, c in crates.items() if c["name"] not in rej_names}
out_files = []
for rel, f in sorted(files.items()):
    if not any(cn in {c["name"] for c in ok_crates.values()} for cn in f["crates"]):
        continue
    if any(cn in rej_names for cn in f["crates"]):
        continue
    old = open(os.path.join(ROOT, rel), "rb").read().decode("utf-8")
    new = old
    for a, b, o, n in sorted(f["edits"], reverse=True):
        assert new[a:b] == o, rel
        new = new[:a] + n + new[b:]
    def occ(sub):
        r, i = [], old.find(sub)
        while i >= 0:
            r.append(i); i = old.find(sub, i + 1)
        return r
    ed = []
    for a_, b_, o_, n_ in f["edits"]:
        oc = occ(o_)
        ed.append([a_, b_, o_, n_, oc.index(a_), len(oc)])
    out_files.append({"path": rel, "sha256": f["sha256"], "sha256_after": sha(new.encode()), "edits": ed, "crates": sorted(set(f["crates"]))})
out_manifests = [m for d, m in sorted(manifests.items()) if C[d]["name"] not in rej_names]
plan = {"hub": hub, "hub_package": HP, "tag": tag, "created": time.strftime("%Y-%m-%d %H:%M"), "stats": {"crates": len(ok_crates), "files": len(out_files), "manifests": len(out_manifests), "edits": sum(len(f["edits"]) for f in out_files), "rejected": len(rejected)},
        "crates": {c["name"]: {"dir": d, "owners_prod": c["owners_prod"], "owners_test": c["owners_test"]} for d, c in ok_crates.items()}, "files": out_files, "manifests": out_manifests}
pd = os.path.normpath(T + "/../staged/plans")
os.makedirs(pd, exist_ok=True)
json.dump(plan, open(pd + "/%s.json" % tag, "w"), ensure_ascii=False)
json.dump(rejected, open(OUT + "/plan-%s.rejected.json" % tag, "w"), ensure_ascii=False, indent=1)
print(json.dumps(plan["stats"]))
rc = collections.Counter(re.sub(r"[^ ]*\.rs: ", "", v)[:60] for v in rejected.values())
print(rc.most_common(10))
