"""🧩 Plan `os-kernel-macros`: the two os-kernel macro files become macro crates whose expansions name only `$crate::__private::<producer>::..`; os-kernel stops mounting them. `python3 build_oskernel_macros.py` -> staged/plans/os-kernel-macros.json, own-map.json (os-kernel macro names -> new crate idents) and newcrates.json."""
from hubenv import *
import re
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from planlib import Plan, manifest_text, add_member, rel_to
from macro_fix import fix_macros, private_module, macro_bodies
from hubres import HubRes
world, I2L, I2D, LIB = make_world()
res = HubRes("os-kernel", world=world)
MAN = json.load(open(OUT + "/manifests.json"))
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
SPECS = [
    dict(name="semio-framework-os-store-space-members-macro", id="os-store-space-members-macro", dir=STORE + "/🎲️space-members-macro", macros=["space_members"], desc="The space_members! generator of per-plugin space member and member factory enums; expansions name only its private producer re-exports"),
    dict(name="semio-framework-os-store-config-record-macros", id="os-store-config-record-macros", dir=STORE + "/🧶️config-record-macros", macros=["field_set_mutations", "config_record", "config_diff"], desc="The config_record!, config_diff! and field_set_mutations! generators; expansions name only its private producer re-exports"),
]
plan = Plan("os-kernel-macros", "os-kernel", "semio-framework-os-kernel")
newcrates, ownmap = {}, {}
explicit = re.compile(r"(?<![A-Za-z0-9_$])::?(semio_framework_[a-z0-9_]+)::")
for s in SPECS:
    lib = s["dir"] + "/🦀️.rs"
    t = plan.text(lib)
    ed, ids, err = fix_macros(t, res.resolve, set(), "crate", own_keep=lambda segs: segs[0] in s["macros"], explicit=lambda i: i in I2L)
    if err: raise SystemExit(str(err))
    sk = t
    t2 = t
    for a, b, o, n in sorted(ed, reverse=True): t2 = t2[:a] + n + t2[b:]
    t2 = t2.rstrip("\n") + "\n" + private_module(ids)
    plan.put(lib, t2, crate=s["name"])
    mdir = s["dir"] + "/📦️packages/🦀️rust"
    deps = [(C[I2D[i]]["name"], rel_to(mdir, I2D[i])) for i in sorted(ids)]
    plan.create(mdir + "/Cargo.toml", manifest_text(s["name"], s["id"], s["desc"], deps, "x", "../../🦀️.rs").replace('workspace = "x"', 'workspace = "../../../../../../../.."'))
    ident = s["name"].replace("-", "_")
    newcrates[ident] = {"name": s["name"], "dir": mdir}
    for m in s["macros"]:
        ownmap[m] = [ident, [m]]
    plan.notes.append(f"{s['name']}: {len(ed)} path edits, producers {sorted(i.replace('semio_framework_', '') for i in ids)}")
    # member registration (root workspace members + framework workspace exclude)
    add_member(plan, "Cargo.toml", STORE + "/🔄️sync/📦️packages/🦀️rust", mdir)
    add_member(plan, "🧰️framework/Cargo.toml", STORE[len("🧰️framework/"):] + "/🔄️sync/📦️packages/🦀️rust", mdir[len("🧰️framework/"):])
# os-kernel stops mounting the macro files
store = STORE + "/🦀️.rs"
t = plan.text(store)
for blk in ('#[macro_use]\n#[path = "🧶️config-record-macros/🦀️.rs"]\nmod config_record_macros;\n', '#[macro_use]\n#[path = "🎲️space-members-macro/🦀️.rs"]\nmod space_members_macro;\n'):
    if t.count(blk) != 1: raise SystemExit("mount block not found once: " + blk[:60])
    t = t.replace(blk, "")
plan.put(store, t, crate="semio-framework-os-kernel")
stats = plan.write(os.path.normpath(T + "/../staged/plans"))
nc = json.load(open(OUT + "/newcrates.json")) if os.path.exists(OUT + "/newcrates.json") else {}
nc.update(newcrates); json.dump(nc, open(OUT + "/newcrates.json", "w"), indent=1)
om = json.load(open(OUT + "/own-map.json")) if os.path.exists(OUT + "/own-map.json") else {}
om.setdefault("os-kernel", {}).update(ownmap); json.dump(om, open(OUT + "/own-map.json", "w"), indent=1)
print(stats); print("\n".join(plan.notes))
