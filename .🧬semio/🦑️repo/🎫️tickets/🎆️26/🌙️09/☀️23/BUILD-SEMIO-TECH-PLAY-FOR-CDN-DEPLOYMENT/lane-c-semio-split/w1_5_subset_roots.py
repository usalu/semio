#!/usr/bin/env python3
"""🪆️ W1 step 5 (split plan 1e): every subset gets its own lib-root file `🪆️subsets/<s>/🦀️.rs` holding its module tree, its
retirement impls and its member-owner invocation; the monolith root mounts those files."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *

root = read(ROOT_RS)
lines = root.split("\n")

a = root.index("retire_leaf!(\n")
b = root.index("crate::semio_member_owner!(subsets::animation")
retire_region = root[a:b]
leaf_body = retire_region[len("retire_leaf!(\n"):retire_region.index(");\n")]
leaves = [l.strip().rstrip(",") for l in leaf_body.split("\n") if l.strip()]
structs = re.findall(r"retire_struct!\(([^;]*)\);", retire_region)
owner_lines = re.findall(r"crate::semio_member_owner!\(subsets::([a-z]+)::[^\n]*\);\n", root)

per = {n: [] for n in NAMES}
base_leaves = []
for leaf in leaves:
    m = re.match(r"([a-z]+)::(\w+)$", leaf)
    if m:
        per[m.group(1)].append("semio_framework_value::artifact_retire_leaf!(schema::snapshot::%s);" % m.group(2))
    else:
        base_leaves.append(leaf)
for st in structs:
    m = re.match(r"([a-z]+)::(.*)$", st)
    if m and m.group(1) in per:
        per[m.group(1)].append("semio_framework_value::artifact_retire_struct!(schema::snapshot::%s);" % m.group(2))
    elif st.startswith("subsets::drawing::schema::diff::NodePath"):
        per["drawing"].append("semio_framework_value::artifact_retire_struct!(schema::diff::%s);" % st.split("diff::", 1)[1])
    elif st.startswith("document_mutation::DocBlockPath"):
        per["document"].append("semio_framework_value::artifact_retire_struct!(schema::mutations::%s);" % st.split("::", 1)[1])
    else:
        raise SystemExit("unclassified retire_struct " + st)
assert sorted(base_leaves) == sorted(["SemioPoint2", "SemioPoint3", "SemioQuaternion", "SemioRgba", "SemioTransform", "SemioUv"]), base_leaves

start = lines.index("        pub mod subsets {")
def find_block(name):
    for i in range(start, len(lines)):
        if lines[i] == "            pub mod %s {" % name and lines[i - 1] == '            #[path = "."]':
            return i - 1, block_end(lines, i)
    raise SystemExit("no block " + name)

DOC = "//! 🪆️ The `%s` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner\n//! implementations of its own types.\n\n"
for name in NAMES:
    s0, s1 = find_block(name)
    body = lines[s0 + 2:s1]
    emoji = EMOJI[name]
    out = []
    for l in body:
        l = l[16:] if l.startswith(" " * 16) else l.lstrip()
        l = l.replace('#[path = "🏅️standards/🔖️v1/🪆️subsets/%s/' % emoji, '#[path = "')
        out.append(l)
    assert not any("🏅️standards" in l for l in out), name
    cap = name.capitalize()
    open_ = ", open = io::binary::snapshot::SemioFlowSnapshotDecode" if name == "flow" else ""
    text = DOC % name + "\n".join(out) + "\n\n" + "\n".join(per[name]) + "\n\ncrate::semio_member_owner!(schema::snapshot::Semio%sSnapshot, schema::mutations::Semio%sMutation%s);\n" % (cap, cap, open_)
    write(SUBSETS + "/" + emoji + "/🦀️.rs", text)
    lines[s0:s1 + 1] = ['            #[path = "🏅️standards/🔖️v1/🪆️subsets/%s/🦀️.rs"]' % emoji, "            pub mod %s;" % name]

root = "\n".join(lines)
a = root.index("retire_leaf!(\n")
b = root.index("crate::semio_member_owner!(subsets::animation")
e = root.index("\n", root.index("crate::semio_member_owner!(subsets::video")) + 1
root = root[:a] + root[e:]
root = root.replace("use semio_framework_value::retirement::RetireOwned;\nuse semio_framework_value::{artifact_retire_leaf as retire_leaf, artifact_retire_struct as retire_struct};\n\nuse crate::base_foundation::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform, SemioUv};\nuse subsets::{document::schema::mutations as document_mutation};\n", "")
root = root.replace("\nuse subsets::{\n    animation::schema::snapshot as animation,", "\n#[cfg(test)]\nuse subsets::{\n    animation::schema::snapshot as animation,")
write(ROOT_RS, root)

found = SUBSETS + "/✉️base/🦀️.rs"
f = read(found).rstrip("\n") + "\n\nsemio_framework_value::artifact_retire_leaf!(schema::geometry::SemioPoint2, schema::geometry::SemioPoint3, schema::geometry::SemioQuaternion, schema::geometry::SemioRgba, schema::geometry::SemioTransform, schema::geometry::SemioUv);\n"
write(found, f)
squeeze_blank_runs([ROOT_RS])
print({n: len(per[n]) for n in NAMES})
