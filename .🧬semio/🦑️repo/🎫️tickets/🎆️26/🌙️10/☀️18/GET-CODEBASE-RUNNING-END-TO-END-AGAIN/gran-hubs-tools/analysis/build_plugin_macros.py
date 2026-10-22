"""🧩 Plan `plugin-macros`: the exported macros of the plugin facade move into four macro crates (window, decl, reactor, facets); their expansions name only `$crate::__private::<producer>::..` (plus the few items they own); the plugin crate stops mounting them. `python3 build_plugin_macros.py` -> staged/plans/plugin-macros.json, own-map.json (plugin names -> new crate idents), newcrates.json."""
from hubenv import *
import re
sys.path.insert(0, os.path.normpath(T + "/../staged"))
from planlib import Plan, manifest_text, add_member, rel_to
from manifest_edit import add_deps
from macro_fix import fix_macros, private_module
from hubres import HubRes

world, I2L, I2D, LIB = make_world()
OVR = {"NoPluginApp": ("semio_framework_plugin_reactor_app", ["NoPluginApp"]), "__semio_dispatch_PluginApp": ("semio_framework_plugin_reactor_app", ["__semio_dispatch_PluginApp"])}
res = HubRes("plugin", overrides=OVR, world=world, allow_gated=True)
HUBFEAT = json.load(open(OUT + "/manifests.json"))[byname["semio-framework-plugin"]]["features"]  # no own map: macro-internal names resolve as own and stay `$crate::name`
P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
ROOTF = P + "/🦀️.rs"
WRAP = P + "/📦️packages/🦀️rust/🦀️.rs"
GEN = P + "/🧪️tests/🧬️generated-test-contracts/🦀️.rs"
plan = Plan("plugin-macros", "plugin", "semio-framework-plugin")
root = plan.text(ROOTF)
gen = plan.text(GEN)


def seg(text, start_pat, end_pat=None):
    i = text.index(start_pat)
    j = text.index(end_pat, i) if end_pat else len(text)
    return i, j


i1, j1 = seg(root, "#[macro_export]\nmacro_rules! register_plugin", "/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5) — `ArtifactChildren`/`DerivedArtifactSpec::Children`")
i2, j2 = seg(root, "/// 🪆️ Whether a subset owns snapshot/diff/mutation types", '#[cfg(test)]\n#[path = "🧪️tests/🔬️subset-macro/🦀️.rs"]')
facets_body = root[i1:j1].rstrip("\n") + "\n\n" + root[i2:j2].rstrip("\n") + "\n"
mounts_i, mounts_j = seg(root, "//#region 📣️FacadeMacroMounts", "//#endregion 📣️FacadeMacroMounts")
mounts_j += len("//#endregion 📣️FacadeMacroMounts\n")
gen_mount = '#[doc(hidden)]\n#[path = "🧪️tests/🧬️generated-test-contracts/🦀️.rs"]\nmod generated_test_contracts;\n'
assert root.count(gen_mount) == 1


def macro_block(name):
    i = gen.index("#[doc(hidden)]\n#[macro_export]\nmacro_rules! " + name)
    nxt = [m.start() for m in re.finditer(r"#\[doc\(hidden\)\]\n#\[macro_export\]\nmacro_rules! ", gen) if m.start() > i]
    return gen[i:(nxt[0] if nxt else len(gen))].rstrip("\n") + "\n"


fresh = macro_block("__semio_plugin_descriptor_fresh_test") + "\n" + macro_block("__semio_extension_descriptor_fresh_test")
subset_tests = macro_block("__semio_subset_owning_conformance_tests") + "\n" + macro_block("__semio_subset_derived_conformance_tests")
oi = root.index('#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\n#[doc(hidden)]\npub mod owned_abi {')
oj = root.index("pub use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleFailure", oi)
owned = root[oi:oj].rstrip("\n") + "\n"
owned = owned.replace("    pub use semio_framework_plugin_abi::owned_abi::*;\n\n", "")
owned = owned.replace("crate::reactor::original_actor_context::exports::", "semio_framework_plugin_reactor_context::exports::")
owned = owned.replace("pub use wasip2::component_export_anchor;", "pub use semio_framework_plugin_abi::wasip2::component_export_anchor;")
SPECS = [
    dict(key="window", name="semio-framework-plugin-macros-window", dir=P + "/📣️macros/🌱️wp1-window", lib=P + "/📣️macros/🌱️wp1-window/🦀️.rs", macros=["transient_root", "window_transient_owners"], own=set(), desc="Plugin window transient macros (transient_root!, window_transient_owners!); expansions name only private producer re-exports"),
    dict(key="decl", name="semio-framework-plugin-macros-decl", dir=P + "/📣️macros/📜️wp2-declaration", lib=P + "/📣️macros/📜️wp2-declaration/🦀️.rs", macros=["app_action_enum", "app_commands", "view_commands"], own=set(), desc="Plugin declaration macros (app_commands!, view_commands!, app_action_enum!); expansions name only private producer re-exports"),
    dict(key="reactor", name="semio-framework-plugin-macros-reactor", dir=P + "/📣️macros/⚛️wp4-reactor", lib=P + "/📣️macros/⚛️wp4-reactor/🦀️.rs", macros=["plugin_exports", "extension_exports", "__semio_actor_exports", "__semio_owned_core_exports", "__semio_plugin_actor_exports", "__semio_plugin_descriptor_fresh_test", "__semio_extension_descriptor_fresh_test"], own={"owned_abi::deallocate", "owned_abi", "component_export_anchor"}, desc="Plugin reactor export macros (plugin_exports!, extension_exports!); expansions name only private producer re-exports", extra=fresh + "\n" + owned),
    dict(key="facets", name="semio-framework-plugin-macros-facets", dir=P + "/📣️macros/🧬️facets", lib=P + "/📣️macros/🧬️facets/🦀️.rs", macros=["register_plugin", "derive_artifact_facets", "subset", "__semio_subset_owning_conformance_tests", "__semio_subset_derived_conformance_tests"], own={"SubsetKind", "SubsetKind::Owning", "SubsetKind::Derived"}, desc="Plugin artifact facet macros (derive_artifact_facets!, subset!, register_plugin!) and SubsetKind; expansions name only private producer re-exports",
         new_text="//! 🧬️ Artifact facet macros of the plugin SDK: `derive_artifact_facets!`, `subset!`, `register_plugin!` and the `SubsetKind` they expand against.\n\n" + facets_body + "\n" + subset_tests),
]
newcrates, ownmap, ws_rel = {}, {}, "../../../../../../../.."
explicit = lambda i: i in I2L
for s in SPECS:
    text = s.get("new_text") or plan.text(s["lib"])
    if s.get("extra"):
        text = text.rstrip("\n") + "\n\n" + s["extra"]
    own = set(s["macros"]) | s["own"]
    own_keep = lambda segs, own=own: "::".join(segs) in own or segs[0] in own
    ed, ids, err = fix_macros(text, res.resolve, set(), "crate", own_keep=own_keep, explicit=explicit)
    if err:
        print(s["key"], "ERRORS", err[:12])
        raise SystemExit(1)
    t2 = text
    for a, b, o, n in sorted(ed, reverse=True):
        t2 = t2[:a] + n + t2[b:]
    if s["key"] == "reactor":
        for m in re.finditer(r"semio_framework_[a-z0-9_]+(?=::)", owned):
            ids.add(m.group(0))
    mdir = s["dir"] + "/📦️packages/🦀️rust"
    gate_cfg, gated, feats = {}, [], {}
    for i in sorted(ids):
        g = res.gate.get(i)
        if not g:
            continue
        cfgs = []
        if g["target"]:
            cfgs.append(g["target"][4:-1])
        if g["optional"]:
            fl = g["features"]
            cfgs.append(f'feature = "{fl[0]}"' if len(fl) == 1 else "any(" + ", ".join(f'feature = "{f}"' for f in fl) + ")")
            for f in fl:
                key = [x for x in HUBFEAT[f] if x in ("dep:" + g["pkg"].replace("semio-framework-", "semio-framework-"),) or x == "dep:" + g["pkg"]]
                feats.setdefault(f, []).append("dep:" + g["pkg"])
                fw = [x for x in HUBFEAT[f] if x.startswith(g["pkg"].replace("semio-framework-", "") + "/") or x.startswith(g["pkg"] + "/")]
                for x in fw:
                    feats[f].append(g["pkg"] + "/" + x.split("/", 1)[1])
        gate_cfg[i] = cfgs[0] if len(cfgs) == 1 else "all(" + ", ".join(cfgs) + ")"
        gated.append((C[I2D[i]]["name"], rel_to(mdir, I2D[i]), g["target"][4:-1] if g["target"] and False else g["target"], g["optional"]))
    t2 = t2.rstrip("\n") + "\n" + private_module(ids, gate_cfg)
    deps = [(C[I2D[i]]["name"], rel_to(mdir, I2D[i])) for i in sorted(ids) if i in I2D and i not in gate_cfg]
    if s.get("new_text"):
        plan.create(s["lib"], t2)
    else:
        plan.put(s["lib"], t2, crate=s["name"])
    plan.create(mdir + "/Cargo.toml", manifest_text(s["name"], s["name"].replace("semio-framework-", ""), s["desc"], deps, ws_rel, "../../🦀️.rs", gated=gated, features=feats or None))
    ident = s["name"].replace("-", "_")
    newcrates[ident] = {"name": s["name"], "dir": mdir, "features": feats}
    for m in s["macros"]:
        if not m.startswith("__semio_"):
            ownmap[m] = [ident, [m]]
    if s["key"] == "facets":
        ownmap["SubsetKind"] = [ident, ["SubsetKind"]]
    if s["key"] == "reactor":
        ownmap["component_export_anchor"] = [ident, ["component_export_anchor"]]
        ownmap["owned_abi::deallocate"] = [ident, ["owned_abi", "deallocate"]]
    plan.notes.append(f"{s['name']}: {len(ed)} path edits, {len(ids)} producers")
    add_member(plan, "Cargo.toml", P + "/🚪️io/📦️packages/🦀️rust", mdir)
    add_member(plan, "🧰️framework/Cargo.toml", P[len("🧰️framework/"):] + "/🚪️io/📦️packages/🦀️rust", mdir[len("🧰️framework/"):])
new_root = root
for a, b in sorted([(i1, j1), (i2, j2), (mounts_i, mounts_j)], reverse=True):
    new_root = new_root[:a] + new_root[b:]
new_root = new_root.replace(gen_mount, "")
plan.put(ROOTF, new_root, crate="semio-framework-plugin")
plan.delete(GEN)
wrap = plan.text(WRAP)
anchor = '#[path = "../../🦀️.rs"]\npub mod component;'
assert wrap.count(anchor) == 1
plan.put(WRAP, wrap.replace(anchor, "#[cfg(test)]\n#[macro_use]\nextern crate semio_framework_plugin_macros_facets;\n\n" + anchor), crate="semio-framework-plugin")
pm = P + "/📦️packages/🦀️rust/Cargo.toml"
d = byname["semio-framework-plugin"]
nt, e = add_deps(plan.text(pm), {"semio-framework-plugin-macros-facets"}, lambda p: False, lambda p: rel_to(d, newcrates["semio_framework_plugin_macros_facets"]["dir"]), section="dev-dependencies")
plan.put(pm, nt, kind="manifest")
stats = plan.write(os.path.normpath(T + "/../staged/plans"))
nc = json.load(open(OUT + "/newcrates.json")) if os.path.exists(OUT + "/newcrates.json") else {}
nc.update(newcrates)
json.dump(nc, open(OUT + "/newcrates.json", "w"), indent=1)
om = json.load(open(OUT + "/own-map.json")) if os.path.exists(OUT + "/own-map.json") else {}
om.setdefault("plugin", {}).update(ownmap)
json.dump(om, open(OUT + "/own-map.json", "w"), indent=1)
print(stats)
print("\n".join(plan.notes))
