"""🧩 Plan `macros-elsewhere`: exported macros defined OUTSIDE the hubs whose bodies name hub paths (macro-emit.json) are rewritten to `$crate::__private::<producer>::..`; the defining crate gets one `__private` module at its lib root end and the producer dependency lines. `python3 build_macro_plan.py [--tag t] [--dry]` -> staged/plans/<tag>.json + macros-elsewhere.json (macro name list, `fixed`)."""
from hubenv import *
import re
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from planlib import Plan
from manifest_edit import add_deps
from macro_fix import fix_macros, private_module
from hubres import HubRes
tag = sys.argv[sys.argv.index("--tag") + 1] if "--tag" in sys.argv else "macros-elsewhere"
OWN = json.load(open(OUT + "/own-map.json")) if os.path.exists(OUT + "/own-map.json") else {}
OVR = json.load(open(OUT + "/overrides.json")) if os.path.exists(OUT + "/overrides.json") else {}
world, I2L, I2D, LIB = make_world()
HUBS_ = {"umbrella": "semio_framework", "plugin": "semio_framework_plugin", "os-kernel": "semio_framework_os_kernel"}
res = {h: HubRes(h, {k: tuple(v) for k, v in OWN.get(h, {}).items()}, {k: tuple(v) for k, v in OVR.get(h, {}).items()}, world, allow_gated=True) for h in HUBS_}
me = json.load(open(OUT + "/macro-emit.json"))
targets = collections.defaultdict(lambda: collections.defaultdict(set))
for h, lst in me.items():
    if h not in HUBS_: continue
    for crate, mac, f in lst:
        if mac == "<proc-macro>": continue
        d = byname[crate]
        if crate in ("semio-framework-plugin", "semio-framework-os-kernel"): continue
        targets[d][h].add(mac)
NEW = json.load(open(OUT + "/newcrates.json")) if os.path.exists(OUT + "/newcrates.json") else {}
NEWD = {n["name"]: n["dir"] for n in NEW.values()}
MAN = json.load(open(OUT + "/manifests.json"))
ROOTS = json.load(open(OUT + "/manifests-roots.json"))
plan = Plan(tag, "macros", "-")
fixed, report = [], {}
for d, hubs in sorted(targets.items()):
    c = C[d]
    lib = libfile(d, c)
    edits_by_file, idents, errors = collections.defaultdict(list), set(), []
    macs = set().union(*hubs.values())
    for f in prod_tree(lib):
        t = open(f, encoding="utf-8").read()
        if "macro_rules!" not in t: continue
        for h, names in hubs.items():
            ed, ids, er = fix_macros(t, res[h].resolve, {HUBS_[h]}, "ident", only=names)
            edits_by_file[f] += ed; idents |= ids; errors += er
    if errors:
        report[c["name"]] = errors[:4]; continue
    if not idents:
        report[c["name"]] = ["no hub path in the macro bodies"]; continue
    rel_lib = os.path.relpath(lib, ROOT)
    for f, ed in edits_by_file.items():
        if not ed: continue
        t = open(f, encoding="utf-8").read()
        for a, b, o, n in sorted(ed, reverse=True): t = t[:a] + n + t[b:]
        rel = os.path.relpath(f, ROOT)
        plan.put(rel, t, crate=c["name"])
    lt = plan.text(rel_lib)
    if "mod __private" in lt:
        report[c["name"]] = ["crate already has __private"]; continue
    plan.put(rel_lib, lt.rstrip("\n") + "\n" + private_module(idents), crate=c["name"])
    mp = os.path.join(c["dir"], "Cargo.toml")
    mt = plan.text(mp)
    wsroot = MAN[d]["wsroot"]; declared = ROOTS.get(wsroot, {})
    names = {(C[I2D[i]]["name"] if i in I2D else NEW[i]["name"]) for i in idents}
    have = {x["pkg"] for x in MAN[d]["deps"] if x["kind"] in ("normal", "build")}
    nt, err = add_deps(mt, names, lambda p: bool(declared.get(p, {}).get("path")) and not declared[p].get("package"), lambda p: os.path.relpath(byname.get(p) or NEWD[p], d).replace(os.sep, "/"), have=have)
    plan.put(mp, nt, kind="manifest", crate=c["name"])
    fixed += sorted(macs)
    report[c["name"]] = ["fixed " + ",".join(sorted(macs)) + " -> " + ",".join(sorted(i.replace("semio_framework_", "") for i in idents))]
stats = plan.write(os.path.normpath(T + "/../staged/plans"), {"fixed_macros": sorted(set(fixed))})
json.dump({"fixed": sorted(set(fixed)), "report": report}, open(OUT + "/macros-elsewhere.json", "w"), indent=1)
print(stats)
for k, v in report.items(): print(" ", k, v)
