"""🧹 Dead-edge plan of one hub: dependents whose source trees (lib, bin, test, example, bench, build roots) never name the hub crate and invoke none of its macro emitters lose the manifest line (feature-carrying, optional and renamed entries are kept). Output staged/plans/<hub>-dead.json (manifests only). `python3 plan_dead.py <hub>`"""
from hubenv import *
import hashlib, time
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from manifest_edit import plan_manifest, line_ops, apply_ops
from classify import classify
hub = sys.argv[1]
HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
HP = C[HD]["name"]
MAN = json.load(open(OUT + "/manifests.json"))
byname_dir = {c["name"]: d for d, c in C.items()}
cl = classify(hub)
HFEAT = MAN[HD]["features"]
KEY2PKG = {x["key"]: x["pkg"] for x in MAN[HD]["deps"]}


def feat_map(features):
    out, seen = {}, set()

    def go(f):
        if f in seen:
            return None
        seen.add(f)
        if f not in HFEAT:
            return f"unknown hub feature {f}"
        for e in HFEAT[f]:
            if e.startswith("dep:"):
                if e[4:] not in KEY2PKG:
                    return f"external dependency {e[4:]}"
                out.setdefault(KEY2PKG[e[4:]], set())
            elif "/" in e:
                k, sub = e.split("/", 1)
                k = k.rstrip("?")
                if k not in KEY2PKG or not KEY2PKG[k].startswith("semio-"):
                    return f"external feature {e}"
                out.setdefault(KEY2PKG[k], set()).add(sub)
            else:
                r = go(e)
                if r:
                    return r
        return None

    for f in features:
        r = go(f)
        if r:
            return None, r
    return out, None

def sha(b): return hashlib.sha256(b).hexdigest()
out, rej = [], collections.Counter()
for d, c in sorted(cl.items()):
    if c != "none": continue
    mp = os.path.join(ROOT, d, "Cargo.toml")
    text = open(mp, "rb").read().decode("utf-8")
    keys = sorted({x["key"] for x in MAN[d]["deps"] if x["pkg"] == HP and x["kind"] in ("normal", "build", "dev")})
    new, err = text, None
    for k in keys:
        hn = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] in ("normal", "build")}
        hd = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] == "dev"}
        new, err = plan_manifest(new, k, [], [], lambda p: False, lambda p, d=d: os.path.relpath(os.path.join(ROOT, byname_dir[p]), os.path.join(ROOT, d)).replace(os.sep, "/"), hn, hd, feat_map)
        if err: break
    if err or not keys:
        rej[err or "no key"] += 1; continue
    ops = line_ops(text, new)
    if apply_ops(text, ops) != new:
        rej["ops not exact-once"] += 1; continue
    out.append({"path": os.path.relpath(mp, ROOT), "sha256": sha(text.encode()), "new_text": new, "sha256_after": sha(new.encode()), "ops": ops, "crate": C[d]["name"]})
plan = {"hub": hub, "hub_package": HP, "tag": hub + "-dead", "created": time.strftime("%Y-%m-%d %H:%M"), "stats": {"manifests": len(out), "rejected": sum(rej.values())}, "crates": {m["crate"]: {} for m in out}, "files": [], "manifests": out}
json.dump(plan, open(os.path.normpath(T + "/../staged/plans/%s-dead.json" % hub), "w"), ensure_ascii=False)
print(hub, plan["stats"], dict(rej))
