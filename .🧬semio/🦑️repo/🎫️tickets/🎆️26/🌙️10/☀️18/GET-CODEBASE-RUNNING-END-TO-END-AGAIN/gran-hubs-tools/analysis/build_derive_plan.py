"""🧬 Plan `os-kernel-derive`: the derive macros of `os-kernel-dsl-derive-expand` stop emitting `::semio_framework_os_kernel::..` and emit the producer crate paths instead; every crate that uses those derives gains the producer dependencies. `python3 build_derive_plan.py` -> staged/plans/os-kernel-derive.json, derive-users.json."""
from hubenv import *
import re
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from planlib import Plan, rel_to
from manifest_edit import add_deps
from repoint_core import skeleton
from hubres import HubRes

world, I2L, I2D, LIB = make_world()
OWN = json.load(open(OUT + "/own-map.json")).get("os-kernel", {})
res = HubRes("os-kernel", {k: tuple(v) for k, v in OWN.items()}, {}, world, allow_gated=True)
d = byname["semio-framework-os-kernel-dsl-derive-expand"]
plan = Plan("os-kernel-derive", "os-kernel", "semio-framework-os-kernel")
HUBID = "semio_framework_os_kernel"
PAT = re.compile(r"::" + HUBID + r"((?:::[A-Za-z_][A-Za-z0-9_]*)+)")
idents, errors, nedits = set(), [], 0
for f in module_tree(libfile(d, C[d])):
    if not isprod(f):
        continue
    rel = os.path.relpath(f, ROOT)
    t = open(f, encoding="utf-8").read()
    if HUBID not in t:
        continue
    sk = skeleton(t)
    edits = []
    for m in PAT.finditer(sk):
        segs = [x for x in m.group(1).split("::") if x]
        q = list(segs)
        r = None
        while q:
            r = res.resolve(q)
            if r[0] != "fail":
                break
            q.pop()
        if not q or r[0] != "rewrite":
            errors.append(f"{rel[-50:]}: {'::'.join(segs)}: {r}")
            continue
        tail = segs[len(q):]
        edits.append((m.start(), m.end(), "::" + r[1] + "".join("::" + x for x in list(r[2]) + tail)))
        idents.add(r[1])
    new = t
    for a, b, n in sorted(edits, reverse=True):
        new = new[:a] + n + new[b:]
    nedits += len(edits)
    plan.put(rel, new, crate="semio-framework-os-kernel-dsl-derive-expand")
if errors:
    print("ERRORS", len(errors)); print("\n".join(errors[:15])); raise SystemExit(1)
# users: os-kernel importers whose trees use the derives + the derive crate's direct dependents
DER = re.compile(r"derive\s*\([^)]*\b(CompositeMutation|Mutations|DslArtifact)\b|\b(diff_text|diff_binary)\s*!")
cs = json.load(open(OUT + "/census-os-kernel.json"))["deps"]
users = set(rev[byname["semio-framework-os-kernel-dsl-derive"]])
for dd in cs:
    try:
        for f in module_tree(libfile(dd, C[dd])):
            if DER.search(open(f, encoding="utf-8", errors="ignore").read()):
                users.add(dd)
                break
    except Exception:
        pass
MAN = json.load(open(OUT + "/manifests.json")); ROOTS = json.load(open(OUT + "/manifests-roots.json"))
names = sorted(C[I2D[i]]["name"] for i in idents if i in I2D)
for u in sorted(users):
    if C[u]["name"] in names:
        continue
    mp = os.path.join(u, "Cargo.toml")
    declared = ROOTS.get(MAN[u]["wsroot"], {})
    have = {x["pkg"] for x in MAN[u]["deps"] if x["kind"] in ("normal", "build")}
    nt, e = add_deps(plan.text(mp), set(names) - {C[u]["name"]}, lambda p, declared=declared: bool(declared.get(p, {}).get("path")) and not declared[p].get("package"), lambda p, u=u: os.path.relpath(os.path.join(ROOT, byname[p]), os.path.join(ROOT, u)).replace(os.sep, "/"), have=have)
    plan.put(mp, nt, kind="manifest", crate=C[u]["name"])
stats = plan.write(os.path.normpath(T + "/../staged/plans"), {"producers": names, "users": len(users)})
json.dump(sorted(C[u]["name"] for u in users), open(OUT + "/derive-users.json", "w"))
json.dump(sorted(idents), open(OUT + "/derive-idents.json", "w"))
print(stats, "emitted path edits", nedits, "producers", len(names), "users", len(users))
