"""🧵 Assembles the final staged plan of a facade repoint with the macro policy: raw source edits (plan_hub.py) + manifest edits with the hub's feature map, new macro crates from newcrates.json, crate-atomic with a fixpoint over shared files. Output: staged/plans/<tag>.json.
`python3 assemble2.py <hub> [--tag t] [--crates a,b]`"""
from hubenv import *
import hashlib, re, time
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from manifest_edit import plan_manifest, line_ops, apply_ops, hub_lines_of, append_to_section
from hubres import HubRes

hub = sys.argv[1]
tag = sys.argv[sys.argv.index("--tag") + 1] if "--tag" in sys.argv else hub
pick = set(sys.argv[sys.argv.index("--crates") + 1].split(",")) if "--crates" in sys.argv else None
raw = json.load(open(OUT + "/plan-%s.raw.json" % hub))
MAN = json.load(open(OUT + "/manifests.json"))
ROOTS = json.load(open(OUT + "/manifests-roots.json"))
CENS = json.load(open(OUT + "/census-%s.json" % hub))["deps"]
NEW = json.load(open(OUT + "/newcrates.json")) if os.path.exists(OUT + "/newcrates.json") else {}
OWN = json.load(open(OUT + "/own-map.json")) if os.path.exists(OUT + "/own-map.json") else {}
FIXED = set()
if os.path.exists(OUT + "/macros-elsewhere.json"):
    FIXED |= set(json.load(open(OUT + "/macros-elsewhere.json"))["fixed"])
for h, m in OWN.items():
    FIXED |= {k for k in m if not "::" in k}
world, I2L, I2D, LIB = make_world()
HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
HP = C[HD]["name"]
HR = HubRes(hub, {k: tuple(v) for k, v in OWN.get(hub, {}).items()}, {}, world, allow_gated=True)
HFEAT = MAN[HD]["features"]
KEY2PKG = {x["key"]: x["pkg"] for x in MAN[HD]["deps"]}
byname_dir = {c["name"]: d for d, c in C.items()}
for ident, n in NEW.items():
    byname_dir[n["name"]] = n["dir"]


def pkg_of(ident):
    return C[I2D[ident]]["name"] if ident in I2D else NEW[ident]["name"]


def dir_of_pkg(pkg):
    return byname_dir[pkg]


def sha(b):
    return hashlib.sha256(b).hexdigest()


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
                k = e[4:]
                if k not in KEY2PKG:
                    return f"external dependency {k}"
                out.setdefault(KEY2PKG[k], set())
            elif "/" in e:
                k, sub = e.split("/", 1)
                k = k.rstrip("?")
                if k not in KEY2PKG or not any(x["key"] == k and x["pkg"].startswith("semio-") for x in MAN[HD]["deps"]):
                    return f"external feature {e}"
                out.setdefault(KEY2PKG[k], set()).add(sub)
            else:
                r = go(e)
                if r:
                    return r
        # macro crates that mirror the feature
        for ident, n in NEW.items():
            if f in (n.get("features") or {}):
                out.setdefault(n["name"], set()).add(f)
        return None

    for f in features:
        r = go(f)
        if r:
            return None, r
    return out, None


crates = dict(raw["crates"])
if pick:
    crates = {d: c for d, c in crates.items() if c["name"] in pick}
manifests, rejected = {}, {}
DEBUG = []
for d, c in list(crates.items()):
    bad = [m for m in CENS.get(d, {}).get("macros", []) if m not in FIXED]
    if bad:
        rejected[c["name"]] = "invokes a macro that still expands to hub paths: " + ",".join(bad[:3])
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
    declared = ROOTS.get(MAN[d]["wsroot"], {})
    prod, test, bad = set(), set(c["owners_test"]), None
    for i in c["owners_prod"]:
        g = HR.gate.get(i)
        if g and g["target"]:
            bad = f"target-gated owner {i}"
        elif g and g["optional"]:
            test.add(i)
        else:
            prod.add(i)
    for i in c["owners_test"]:
        g = HR.gate.get(i)
        if g and g["target"]:
            bad = f"target-gated owner {i}"
    if bad:
        rejected[c["name"]] = bad
        continue
    op = sorted({pkg_of(i) for i in prod})
    ot = sorted({pkg_of(i) for i in test} - set(op))

    def ws_declared(pkg, declared=declared):
        e = declared.get(pkg)
        return bool(e) and e.get("path") is not None and not e.get("package")

    def rel_of(pkg, d=d):
        return os.path.relpath(os.path.join(ROOT, dir_of_pkg(pkg)), os.path.join(ROOT, d)).replace(os.sep, "/")

    hn = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] in ("normal", "build")}
    hd = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] == "dev"}
    new, err = text, None
    hub_keep = []
    for key in keys:
        if c.get("hub_dev"):
            hl, hk = hub_lines_of(new, key)
            if hl:
                hub_keep += hl
        new, err = plan_manifest(new, key, op, ot, ws_declared, rel_of, hn, hd, feat_map)
        if err:
            break
    if not err and hub_keep:
        present = set(l.strip() for l in new.split("\n"))
        add = [l for l in hub_keep if l.strip() not in present]
        if add:
            new = append_to_section(new, "dev-dependencies", add)
    if err:
        rejected[c["name"]] = err
        continue
    if any(len(rel_of(p)) > 230 for p in op + ot):
        rejected[c["name"]] = "relative path longer than 230 characters"
        continue
    ops = line_ops(text, new)
    if apply_ops(text, ops) != new:
        rejected[c["name"]] = "manifest ops are not exact-once"
        if len(DEBUG) < 3:
            DEBUG.append({"crate": c["name"], "ops": ops, "old_tail": text[-400:], "text": text, "new": new, "new_diff_lines": [l for l in new.split("\n") if l not in text.split("\n")][:8]})
        continue
    manifests[d] = {"path": os.path.relpath(mp, ROOT), "sha256": sha(text.encode()), "new_text": new, "sha256_after": sha(new.encode()), "ops": ops, "crate": c["name"]}
rej_names = set(rejected)
changed = True
files = raw["files"]
ok_names = {c["name"] for c in crates.values()}
while changed:
    changed = False
    for rel, f in files.items():
        if any(cn in rej_names for cn in f["crates"]):
            for cn in f["crates"]:
                if cn in ok_names and cn not in rej_names:
                    rej_names.add(cn)
                    rejected[cn] = "shares a file with a rejected crate"
                    changed = True
ok_crates = {d: c for d, c in crates.items() if c["name"] not in rej_names}
ok_names = {c["name"] for c in ok_crates.values()}
out_files = []
for rel, f in sorted(files.items()):
    if not (set(f["crates"]) & ok_names) or any(cn in rej_names for cn in f["crates"]):
        continue
    old = open(os.path.join(ROOT, rel), "rb").read().decode("utf-8")
    new = old
    for a_, b_, o_, n_ in sorted(f["edits"], reverse=True):
        assert new[a_:b_] == o_, rel
        new = new[:a_] + n_ + new[b_:]

    def occ(sub):
        r, i = [], old.find(sub)
        while i >= 0:
            r.append(i)
            i = old.find(sub, i + 1)
        return r

    ed = []
    for a_, b_, o_, n_ in f["edits"]:
        oc = occ(o_)
        ed.append([a_, b_, o_, n_, oc.index(a_), len(oc), [old[max(0, a_ - 32):a_], old[b_:b_ + 32]]])
    out_files.append({"path": rel, "sha256": f["sha256"], "sha256_after": sha(new.encode()), "edits": ed, "crates": sorted(set(f["crates"]))})
out_manifests = [m for d, m in sorted(manifests.items()) if C[d]["name"] not in rej_names]
plan = {"hub": hub, "hub_package": HP, "tag": tag, "created": time.strftime("%Y-%m-%d %H:%M"),
        "stats": {"crates": len(ok_crates), "files": len(out_files), "manifests": len(out_manifests), "edits": sum(len(f["edits"]) for f in out_files), "rejected": len(rejected), "dependents": len(CENS)},
        "crates": {c["name"]: {"dir": d, "owners_prod": c["owners_prod"], "owners_test": c["owners_test"]} for d, c in ok_crates.items()}, "files": out_files, "manifests": out_manifests, "creates": [], "deletes": [], "notes": []}
pd = os.path.normpath(T + "/../staged/plans")
os.makedirs(pd, exist_ok=True)
json.dump(plan, open(pd + "/%s.json" % tag, "w"), ensure_ascii=False)
json.dump(DEBUG, open(OUT + "/plan-%s.debug.json" % tag, "w"), ensure_ascii=False, indent=1)
json.dump(rejected, open(OUT + "/plan-%s.rejected.json" % tag, "w"), ensure_ascii=False, indent=1)
print(json.dumps(plan["stats"]))
rc = collections.Counter(re.sub(r"[^ ]*\.rs: ", "", v)[:70] for v in rejected.values())
print(rc.most_common(14))
