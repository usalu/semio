import os, shutil
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
D = ROOT + "/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit"
root = D + "/🦀️.rs"
child_dir = D + "/🎯️element-picks"
child = child_dir + "/🦀️.rs"
assert not os.path.exists(child)
text = open(root, encoding="utf-8").read()
shutil.copy(root, F + "/lwr-tools/orig/mesh-edit.rs")
lines = text.split("\n")
s = next(i for i, l in enumerate(lines) if l.startswith("fn edges("))
e = next(i for i, l in enumerate(lines) if l.startswith("fn bevel("))
block = lines[s:e]
assert sum(1 for l in lines if l.startswith("fn edges(") or l.startswith("fn vertices(") or l.startswith("fn faces(")) == 3
assert block[-1] == "" and len(block) == 12, len(block)
block = [("pub(super) " + l) if l.startswith("fn ") else l for l in block]
imp_picked = "use semio_s_artifact_procedural_3d_mesh_support::picked;"
imp_ids_old = "use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId, WeldMode};"
imp_ids_new = "use semio_framework_3d::mesh::{FaceId, WeldMode};"
for a in (imp_picked, imp_ids_old):
    assert text.count(a) == 1
out = []
for i, l in enumerate(lines):
    if s <= i < e or l == imp_picked:
        continue
    out.append(imp_ids_new if l == imp_ids_old else l)
j = out.index(imp_ids_new)
out[j + 1:j + 1] = ["", '#[path = "🎯️element-picks/🦀️.rs"]', "mod element_picks;", "use element_picks::{edges, faces, vertices};"]
head = [
    "//! 🎯️ Mesh-edit element picks: the selection ports of the `mesh.edit` computes, validated against the input mesh (see [`picked`]).",
    "",
    "use semio_framework_3d::mesh::{EdgeId, FaceId, VertexId};",
    "use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;",
    "use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;",
    "use semio_s_artifact_procedural_3d_mesh_support::{picked, Element};",
    "",
]
os.makedirs(child_dir)
open(child, "w", encoding="utf-8").write("\n".join(head + block).rstrip("\n") + "\n")
open(root + ".lwr-tmp", "w", encoding="utf-8").write("\n".join(out))
os.replace(root + ".lwr-tmp", root)
print("ok", len(lines), len(out))
