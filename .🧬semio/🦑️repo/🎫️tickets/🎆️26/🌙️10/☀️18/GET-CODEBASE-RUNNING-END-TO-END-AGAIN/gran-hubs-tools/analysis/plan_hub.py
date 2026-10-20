"""📝 Facade repoint planner. For one hub (plugin | os-kernel | umbrella): every direct dependent's source files are rewritten by `repoint_core` to the producer crates that own the names; manifests are planned (hub edge replaced by the producer edges). Output: gran-hubs-tools/staged/plans/<hub>.json (per-file sha256 + exact edits, per-manifest edits) and 🗑️generated/gran-hubs/plan-<hub>.stats.json.
Run: `nice -n 10 python3 plan_hub.py <hub> [--only crate-name,...]` (reads only the hub's dependents; cwd independent)."""
from hubenv import *
import hashlib, re, time
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from repoint_core import rewrite_file, skeleton as sk_
from cratemodel import Model, World

hub = sys.argv[1]
only = set(sys.argv[sys.argv.index("--only") + 1].split(",")) if "--only" in sys.argv else None
HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
MAN = json.load(open(OUT + "/manifests.json"))
ROOTS = json.load(open(OUT + "/manifests-roots.json"))
CENSUS = json.load(open(OUT + "/census-%s.json" % hub))
ID = CENSUS["ident"]
FAC = {"semio_framework", "semio_framework_os_kernel", "semio_framework_plugin"}
world, I2L, I2D, LIB = make_world()
model = world.model_for(ID)
gated = set()
for x in MAN[HD]["deps"]:
    if x["kind"] in ("normal", "build") and (x["optional"] or x["target"]):
        for k in ("normal", "build"):
            for dn, t in C[HD]["deps"][k]:
                if dn == x["key"] and t in C:
                    gated.add(LIB[t])
pure_cache = {}


def mod_crates(node, seen=None):
    seen = seen if seen is not None else set()
    if id(node) in seen:
        return set()
    seen.add(id(node))
    if node.defs:
        return None
    out = set()
    for vis, p, al, it in node.uses:
        if not vis.startswith("pub"):
            continue
        h = p[0]
        if h in ("crate", "self", "super"):
            continue
        if h in node.children:
            r = mod_crates(node.children[h], seen)
            if r is None:
                return None
            out |= r
        elif h in I2L and h not in FAC:
            out.add(h)
    for nm, ch in node.children.items():
        if ch.vis.startswith("pub"):
            r = mod_crates(ch, seen)
            if r is None:
                return None
            out |= r
    return out


def pure_module(node):
    k = id(node)
    if k not in pure_cache:
        pure_cache[k] = mod_crates(node) is not None
    return pure_cache[k]


rcache = {}


def resolve(segs):
    key = tuple(segs)
    if key in rcache:
        return rcache[key]
    q = list(segs)
    suffix = []
    r = None
    while q:
        r = world.resolve(model, q)
        if r:
            break
        suffix.insert(0, q.pop())
    out = ("fail", "unresolved")
    if r and q:
        if r["kind"] == "external":
            out = ("fail", "external " + str(r.get("owner")))
        elif r["owner"] == ID and r["kind"] in ("def", "mod", "crate"):
            if r["kind"] == "mod" and not suffix:
                nd = [x for x in model.nodes if x.name == r["name"] and os.path.relpath(x.file, ROOT) == r["file"]]
                out = ("modalias", list(segs)) if nd and pure_module(nd[0]) else ("own",)
            else:
                out = ("own",)
        else:
            st = [s for s in r.get("steps", []) if s[0] not in FAC and s[0] != ID]
            if not st:
                out = ("fail", "no non-facade owner")
            else:
                ident = st[0][0]
                if ident in gated:
                    out = ("fail", "gated owner " + ident)
                else:
                    out = ("rewrite", ident, list(st[0][1:]) + suffix)
    rcache[key] = out
    return out


def target_roots(d):
    t = open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8").read()
    prod, test = [], []
    for m in re.finditer(r"^\[\[(bin|test|example|bench)\]\](.*?)(?=^\[|\Z)", t, re.S | re.M):
        pm = re.search(r'^\s*path\s*=\s*"([^"]+)"', m.group(2), re.M)
        if pm:
            (prod if m.group(1) == "bin" else test).append(os.path.normpath(os.path.join(ROOT, d, pm.group(1))))
    bm = re.search(r'^\s*build\s*=\s*"([^"]+)"', t, re.M)
    if bm:
        prod.append(os.path.normpath(os.path.join(ROOT, d, bm.group(1))))
    for sub in ("src/main.rs", "build.rs"):
        p = os.path.join(ROOT, d, sub)
        if os.path.exists(p):
            prod.append(p)
    return prod, test


def sha(b):
    return hashlib.sha256(b).hexdigest()


plan_files, crates, blocked = {}, {}, collections.Counter()
reasons = collections.defaultdict(list)
t0 = time.time()
deps = sorted(CENSUS["deps"])
for n, d in enumerate(deps):
    v = CENSUS["deps"][d]
    c = C[d]
    if only and c["name"] not in only:
        continue
    cls_none = not v["prod"] and not v["test"] and not v["macros"]
    if not v["prod"] and not v["test"]:
        continue
    keys = [dn for k in ("normal", "build", "dev") for dn, t in c["deps"][k] if t == HD]
    idents = {k.replace("-", "_") for k in keys}
    try:
        lib = prod_tree(libfile(d, c))
    except Exception:
        lib = []
    pr, te = target_roots(d)
    pf = set(lib)
    for r in pr:
        pf |= set(module_tree(r))
    tf = set()
    for r in te:
        tf |= set(module_tree(r))
    # lib modules gated by cfg(test) belong to the test side
    for f in module_tree(libfile(d, c)):
        if f not in pf:
            tf.add(f)
    texts = {}
    for f in sorted(pf | tf):
        try:
            texts[f] = open(f, "rb").read().decode("utf-8")
        except Exception:
            pass
    # crate-wide `extern crate hub as alias;`
    for f, t in texts.items():
        if any(i in t for i in idents):
            for m in re.finditer(r"extern\s+crate\s+(%s)\s+as\s+(%s)\s*;" % ("|".join(map(re.escape, idents)), "[A-Za-z_][A-Za-z0-9_]*"), t):
                idents.add(m.group(2))
    results, errors, owners_p, owners_t, own = {}, [], set(), set(), 0
    for f, t in texts.items():
        if not any(i in t for i in idents):
            continue
        r = rewrite_file(t, idents, resolve)
        if r.errors:
            errors += [f"{os.path.relpath(f, ROOT)[-60:]}: {e}" for e in r.errors[:3]]
            continue
        own += r.own_refs
        if r.edits:
            results[f] = r
            ((owners_t if (f in tf and f not in pf) or "🧪️" in f else owners_p)).update(r.owners)
    if errors or own:
        blocked["own" if own and not errors else "error"] += 1
        reasons[c["name"]] = errors[:3] or ["own definitions used: %d refs" % own]
        continue
    if not results:
        blocked["no-edit"] += 1
        continue
    hubfeat = [x for x in MAN[d]["deps"] if x["pkg"] == C[HD]["name"] and (x["features"] or x["optional"] or not x["defaultFeatures"])]
    if hubfeat:
        blocked["hub-edge-has-features"] += 1
        reasons[c["name"]] = ["hub edge has features/optional"]
        continue
    crates[d] = {"name": c["name"], "owners_prod": sorted(owners_p), "owners_test": sorted(owners_t - owners_p), "files": [os.path.relpath(f, ROOT) for f in results]}
    for f, r in results.items():
        rel = os.path.relpath(f, ROOT)
        old = texts[f].encode("utf-8")
        plan_files.setdefault(rel, {"sha256": sha(old), "edits": [[a, b, texts[f][a:b], new] for a, b, new in r.edit_list], "crates": []})["crates"].append(c["name"])
    if n % 300 == 0:
        print(hub, n, len(deps), round(time.time() - t0), flush=True)
# a file mounted by several crates must be rewritten identically; crates sharing a file are planned together (already identical text edits)
stats = {"hub": hub, "dependents": len(deps), "planned_crates": len(crates), "planned_files": len(plan_files), "blocked": dict(blocked), "edits": sum(len(f["edits"]) for f in plan_files.values())}
json.dump({"stats": stats, "crates": crates, "files": plan_files}, open(OUT + "/plan-%s.raw.json" % hub, "w"), ensure_ascii=False)
json.dump({"stats": stats, "reasons": dict(list(reasons.items())[:400])}, open(OUT + "/plan-%s.stats.json" % hub, "w"), ensure_ascii=False, indent=1)
print(json.dumps(stats))
rc = collections.Counter()
for k, v in reasons.items():
    rc[re.sub(r"[^:]*: ", "", v[0])[:70]] += 1
print(rc.most_common(15))
