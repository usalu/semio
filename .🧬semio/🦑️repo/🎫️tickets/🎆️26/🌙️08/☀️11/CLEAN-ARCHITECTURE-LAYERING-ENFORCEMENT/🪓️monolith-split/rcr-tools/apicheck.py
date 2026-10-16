"""Public-path check: the set of top-level public items (kind, name) and pub use lines of the original root equals the union over the new root and children; plus glob-collision scan (a top-level name defined in two files)."""
import collections, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC

def api(text):
    out = collections.Counter(); names = collections.Counter(); impls = collections.Counter()
    for it in top_items(text):
        if it["kind"] in ("fn", "struct", "enum", "trait", "type", "const", "static", "union") and it["name"]:
            names[(it["kind"], it["name"])] += 1
            if it["vis"] == "pub":
                out[(it["kind"], it["name"])] += 1
        if it["kind"] == "use" and it["vis"].startswith("pub") and not it["vis"].startswith("pub(super)"):
            out[("pub use", " ".join(t[1] for t in it["hdr"] if t[0] in ("id", "p")))] += 1
        if it["kind"] == "impl":
            impls[it["name"]] += 1
        if it["kind"] not in ("fn", "struct", "enum", "trait", "type", "const", "static", "union", "use", "impl", "mod", "macro") and it["hdr"]:
            names[("macro-call", it["kind"] + ":" + "".join(t[1] for t in it["hdr"][:12]))] += 1
    return out, names, impls

ok = True
for name, s in SPEC.items():
    t = TARGETS[name]
    base = f"{ROOT}/{t['pkgdir']}"
    o_api, o_names, o_impls = api(read(f"{F}/rcr-tools/orig/{name}.rs.txt"))
    n_api = collections.Counter(); n_names = collections.Counter(); n_impls = collections.Counter()
    where = collections.defaultdict(list)
    for f in ["🦀️.rs"] + [f"{c['folder']}/🦀️.rs" for c in s["children"]]:
        a, nm, im = api(read(f"{base}/{f}"))
        n_api += a; n_names += nm; n_impls += im
        for k in nm: where[k].append(f)
    mounts = {k for k in n_api if k[0] == "pub use" and ": : * ;" in k[1] and k[1].split()[2] in {c["mod"] for c in s["children"]}}
    n_api2 = collections.Counter({k: v for k, v in n_api.items() if k not in mounts})
    lost = o_api - n_api2; extra = n_api2 - o_api
    dup = [k for k, v in n_names.items() if v > 1 and k[0] != "macro-call"]
    impl_lost = o_impls - n_impls
    print(f"{name}: original public items {sum(o_api.values())}; new {sum(n_api2.values())}; lost {sum(lost.values())}; extra {sum(extra.values())}; top-level name defined twice {len(dup)}; impl blocks original {sum(o_impls.values())} new {sum(n_impls.values())} lost {sum(impl_lost.values())}")
    for k in list(lost)[:5]: print("   LOST", k)
    for k in list(extra)[:5]: print("   EXTRA", k)
    for k in dup[:5]: print("   DUP", k, where[k])
    if lost or extra or dup or impl_lost: ok = False
print("API CHECK", "OK" if ok else "FAIL")
