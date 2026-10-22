"""📝 Facade repoint planner. For one hub (plugin | os-kernel | umbrella): every direct dependent's source files are rewritten by `repoint_core` to the producer crates that own the names; manifests are planned (hub edge replaced by the producer edges). Output: gran-hubs-tools/staged/plans/<hub>.json (per-file sha256 + exact edits, per-manifest edits) and 🗑️generated/gran-hubs/plan-<hub>.stats.json.
Run: `nice -n 10 python3 plan_hub.py <hub> [--only crate-name,...]` (reads only the hub's dependents; cwd independent)."""
from hubenv import *
import hashlib, re, time
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from repoint_core import rewrite_file, skeleton as sk_
from macro_fix import macro_bodies
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
FIXEDM = set()
if os.path.exists(OUT + "/macros-elsewhere.json"):
    FIXEDM |= set(json.load(open(OUT + "/macros-elsewhere.json"))["fixed"])
OWN = json.load(open(OUT + "/own-map.json")).get(hub, {}) if os.path.exists(OUT + "/own-map.json") else {}
OVRD = json.load(open(OUT + "/overrides.json")).get(hub, {}) if os.path.exists(OUT + "/overrides.json") else {}
from hubres import HubRes
HR = HubRes(hub, {k: tuple(v) for k, v in OWN.items()}, {k: tuple(v) for k, v in OVRD.items()}, world, allow_gated=True)
model = HR.model
resolve = HR.resolve


BARE_OK = {i for i in idents_all()} if False else {i for i in ('protocol',) if i in I2D}


def skips(t):
    if 'macro_rules!' not in t or not FIXEDM:
        return ()
    sk = sk_(t)
    return [(a, b) for n, a, b in macro_bodies(t, sk) if n in FIXEDM and 'macro_export' in t[max(0, a - 200):a]]


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
    REAL = set(idents)
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
    results, errors, owners_p, owners_t, own, skipped = {}, [], set(), set(), 0, 0
    declared_alias = bool(idents - {k.replace("-", "_") for k in keys})
    for f, t in texts.items():
        if not f.endswith(".rs"):
            continue
        if not any(i in t for i in idents):
            continue
        is_test = ("🧪️" in f) or (f in tf and f not in pf)
        r = rewrite_file(t, idents, resolve, skip_spans=skips(t), bare_ok=BARE_OK, real_idents=REAL)
        if r.errors or (is_test and r.own_refs):
            if is_test:
                skipped += 1
                continue
            errors += [f"{os.path.relpath(f, ROOT)[-60:]}: {e}" for e in r.errors[:3]]
            continue
        own += r.own_refs
        if r.edits:
            results[f] = r
            (owners_t if is_test else owners_p).update(r.owners)
    if skipped and declared_alias and not errors and not own:
        results, owners_p, owners_t = {}, set(), set()
        for f, t in texts.items():
            if not f.endswith(".rs") or not any(i in t for i in idents):
                continue
            is_test = ("🧪️" in f) or (f in tf and f not in pf)
            r = rewrite_file(t, idents, resolve, alias_cfg_test=True, skip_spans=skips(t), bare_ok=BARE_OK, real_idents=REAL)
            if r.errors or (is_test and r.own_refs):
                if is_test:
                    continue
                errors += [f"{os.path.relpath(f, ROOT)[-60:]}: {e}" for e in r.errors[:3]]
                continue
            own += r.own_refs
            if r.edits:
                results[f] = r
                (owners_t if is_test else owners_p).update(r.owners)
    if errors or own:
        blocked["own" if own and not errors else "error"] += 1
        reasons[c["name"]] = errors[:3] or ["own definitions used: %d refs" % own]
        continue
    if not results and not skipped:
        blocked["no-edit"] += 1
        continue
    crates[d] = {"name": c["name"], "owners_prod": sorted(owners_p), "owners_test": sorted(owners_t - owners_p), "hub_dev": bool(skipped), "files": [os.path.relpath(f, ROOT) for f in results]}
    for f, r in results.items():
        rel = os.path.relpath(f, ROOT)
        old = texts[f].encode("utf-8")
        plan_files.setdefault(rel, {"sha256": sha(old), "edits": [[a, b, texts[f][a:b], new] for a, b, new in r.edit_list], "crates": []})["crates"].append(c["name"])
    if n % 300 == 0:
        print(hub, n, len(deps), round(time.time() - t0), flush=True)
# a file mounted by several crates must be rewritten identically; crates sharing a file are planned together (already identical text edits)
stats = {"hub": hub, "dependents": len(deps), "planned_crates": len(crates), "planned_files": len(plan_files), "blocked": dict(blocked), "edits": sum(len(f["edits"]) for f in plan_files.values())}
json.dump({"stats": stats, "crates": crates, "files": plan_files}, open(OUT + "/plan-%s.raw.json" % hub, "w"), ensure_ascii=False)
json.dump({"stats": stats, "reasons": dict(list(reasons.items())[:5000])}, open(OUT + "/plan-%s.stats.json" % hub, "w"), ensure_ascii=False, indent=1)
print(json.dumps(stats))
rc = collections.Counter()
for k, v in reasons.items():
    rc[re.sub(r"[^:]*: ", "", v[0])[:70]] += 1
print(rc.most_common(15))
