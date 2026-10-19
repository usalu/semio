import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from leafize import *
APPLY = "--apply" in sys.argv
X = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
M = X + "/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations"
AGG = M + "/🦀️.rs"
LEAVES = {  # dir: (module, struct, inverse siblings (module,struct))
 "➕insert-list-item": ("insert_list_item", "InsertListItem", [("remove_list_item", "RemoveListItem")]),
 "➖remove-list-item": ("remove_list_item", "RemoveListItem", [("insert_list_item", "InsertListItem")]),
 "✖️remove-map-entry": ("remove_map_entry", "RemoveMapEntry", [("set_map_entry", "SetMapEntry")]),
 "✂️remove-node": ("remove_node", "RemoveNode", [("set_node", "SetNode")]),
 "🗝️set-map-entry": ("set_map_entry", "SetMapEntry", [("remove_map_entry", "RemoveMapEntry")]),
 "🧷set-node": ("set_node", "SetNode", [("remove_node", "RemoveNode")]),
 "🔁set-value": ("set_value", "SetValue", []),
}
MODS = [v[0] for v in LEAVES.values()]
B = "semio_s_artifact_stdio_semio_base::schema::triples::"
V = "semio_s_artifact_stdio_semio_value_model::schema::"
IMP = {
 "IndexAdded": B + "IndexAdded", "NamedModified": B + "NamedModified", "NamedTripleDiff": B + "NamedTripleDiff",
 "NamedAdded": V + "diff::NamedAdded", "SemioValueDiff": V + "diff::SemioValueDiff", "SemioValueTreeDiff": V + "diff::SemioValueTreeDiff",
 "SemioValue": V + "snapshot::SemioValue", "SemioValueEntry": V + "snapshot::SemioValueEntry", "SemioValueNode": V + "snapshot::SemioValueNode",
 "SemioValueSnapshot": V + "snapshot::SemioValueSnapshot", "ValueId": V + "snapshot::ValueId",
 "resolve": "crate::path::resolve", "diff_at_path": "crate::path::diff_at_path", "SemioValuePath": "crate::path::SemioValuePath",
}
import nsmap
def imports_for(text, extra=()):
    ids = nsmap.free_idents(text)
    names = sorted(n for n in IMP if n in ids)
    return "".join("use %s;\n" % IMP[n] for n in names)

NEWNAME = "semio-s-artifact-stdio-semio-value-mutation-leaves"
ND = M + "/🍃️leaves"
PATHF = M + "/🧭️path-vocabulary/🦀️.rs"
agg = rd(AGG)
ra = agg.index("//#region 🔖️SemioValuePath"); rb = agg.index("//#endregion 🔖️SemioValuePath") + len("//#endregion 🔖️SemioValuePath") + 1
da = agg.index("//#region 🔖️DiffAtPath"); db = agg.index("//#endregion 🔖️DiffAtPath") + len("//#endregion 🔖️DiffAtPath") + 1
path_region = agg[ra:rb]; diff_region = agg[da:db]
path_file = '''//! 🧭️ The semio value mutation path vocabulary shared by the leaves and the aggregate: path segments, read-only navigation and the nested modified-chain a leaf diff is lowered into.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueDiff;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;

''' + path_region + "\n" + diff_region
path_file = path_file.replace("\nfn resolve<", "\npub fn resolve<").replace("\nfn diff_at_path(", "\npub fn diff_at_path(").replace("\nfn wrap_at_path(", "\nfn wrap_at_path(")
out = {}
out[PATHF] = path_file
root = '''//! 🍃️ The seven semio value mutation leaves with their diffs and inverses, generic over the dispatching aggregate, held below the aggregate crate so it stays inside the source gate.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "../🧭️path-vocabulary/🦀️.rs"]
pub mod path;

'''
for d, (mod, st, sib) in sorted(LEAVES.items(), key=lambda kv: kv[1][0]):
    root += '#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (d, mod)
out[ND + "/🦀️.rs"] = root
for d, (mod, st, sib) in LEAVES.items():
    f = M + "/" + d + "/🦀️.rs"
    t = rd(f)
    assert "use super::*;\n" in t
    froms = ["crate::%s::%s" % (m, s) for m, s in sib]
    t2, x = generic_impls(t, "SemioValueSnapshot", "SemioValueTreeDiff", "SemioValueMutation", froms)
    assert x == st, (x, st)
    imp = imports_for(t2)
    t2 = t2.replace("use super::*;\n", imp, 1)
    t2 = re.sub(r'\(\) -> usize', '() -> usize', t2)
    out[f] = t2
    df = M + "/" + d + "/🔺️diff/🦀️.rs"
    dt = rd(df)
    assert "use super::super::*;\n" in dt
    out[df] = dt.replace("use super::super::*;\n", imports_for(dt), 1)
    inf = M + "/" + d + "/↩️inverse/🦀️.rs"
    it = rd(inf)
    assert "use super::super::*;\n" in it
    it2 = re.sub(r'SemioValueMutation::[A-Za-z]+\(', 'Op::from(', it)
    for m in MODS:
        it2 = re.sub(r'(?<![A-Za-z0-9_:])' + m + r'::', 'crate::' + m + '::', it2)
    used = sorted(set(re.findall(r'Op::from\(crate::([a-z_]+)::([A-Za-z]+)', it2)))
    it2 = it2.replace("pub fn inverse(payload: &super::%s, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {" % st,
                      "pub fn inverse<Op>(payload: &super::%s, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>\nwhere\n    Op: %s,\n{" % (st, " + ".join("From<crate::%s::%s>" % (m, s_) for m, s_ in used)))
    it2 = it2.replace("use super::super::*;\n", imports_for(it2), 1)
    out[inf] = it2
# aggregator
a = agg.replace(path_region, "").replace(diff_region, "")
blk = ''
for d, (mod, st, sib) in LEAVES.items():
    pm = '#[path = "%s/🦀️.rs"]\npub mod %s;\n' % (d, mod)
    assert pm in a, mod
    a = a.replace(pm, "", 1)
a = a.replace("//#region 🔖️Mutations\n", "//#region 🔖️Mutations\npub use semio_s_artifact_stdio_semio_value_mutation_leaves::path::{SemioValuePath, SemioValuePathSegment};\npub use semio_s_artifact_stdio_semio_value_mutation_leaves::{insert_list_item, remove_list_item, remove_map_entry, remove_node, set_map_entry, set_node, set_value};\n", 1)
for imp in ["use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;\n", "use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;\n", "use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;\n", "use semio_s_artifact_stdio_semio_value_model::schema::diff::NamedAdded;\n", "use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueDiff;\n"]:
    assert imp in a, imp
    a = a.replace(imp, "", 1)
out[AGG] = a
for p, t in sorted(out.items()):
    print("OUT", p[-80:], len(t))
if not APPLY:
    for p in (M + "/➕insert-list-item/🦀️.rs", M + "/➕insert-list-item/↩️inverse/🦀️.rs", M + "/🗝️set-map-entry/↩️inverse/🦀️.rs", ND + "/🦀️.rs"):
        print("=====", p[-60:]); print(out[p])
    sys.exit(0)
for p, t in out.items():
    wr(p, t)
npkg = register_crate(X + "/Cargo.toml", "semio-s-artifact-stdio-semio-value-mutations", M + "/📦️packages/🦀️rust", NEWNAME, ND, "Stdio semio value mutation leaves generic over the dispatching aggregate", "../../🦀️.rs")
mf = M + "/📦️packages/🦀️rust/Cargo.toml"
add_dep_line(mf, "dependencies", NEWNAME, '%s = { path = "%s" }' % (NEWNAME, os.path.relpath(os.path.join(ROOT, npkg), os.path.join(ROOT, os.path.dirname(mf)))))
print(run_owners(X, True))
