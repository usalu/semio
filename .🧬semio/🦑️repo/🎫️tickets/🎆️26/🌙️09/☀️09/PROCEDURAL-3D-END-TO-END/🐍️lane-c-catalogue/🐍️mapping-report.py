import json, os, sys

ROOT = "/Users/ueli/Documents/semio"
T = f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
G = f"{T}/🗑️generated/lane-c/"
D = f"{ROOT}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue"

mapping = json.load(open(G + "mapping.json", encoding="utf8"))
old_in = json.load(open(G + "old-inputs.json", encoding="utf8"))
old_out = json.load(open(G + "old-outputs.json", encoding="utf8"))

mesh_in = {
    "construct": ["data"], "box": ["width", "height", "depth"], "plane": ["width", "depth"], "sphere": ["radius", "subdivisions"],
    "cylinder": ["radius", "height", "segments"], "cone": ["radius", "height", "segments"], "fromBrep": ["geometry", "deflection"],
    "toBrep": ["mesh", "tolerance"], "translate": ["mesh", "offset"], "transform": ["mesh", "matrix"], "rotate": ["mesh", "axis", "angle"],
    "scale": ["mesh", "factor"], "moveVertices": ["mesh", "vertices", "offset"], "translateComponents": ["mesh", "mode", "selection", "offset"],
    "rotateComponents": ["mesh", "mode", "selection", "pivot", "center", "axis", "angle"], "scaleComponents": ["mesh", "mode", "selection", "pivot", "center", "factor"],
    "bevel": ["mesh", "edges", "amount", "segments"], "dissolveEdges": ["mesh", "edges"], "dissolveVertices": ["mesh", "selection"],
    "mergeVertices": ["mesh", "selection", "mode", "tolerance"], "moveProportional": ["mesh", "selection", "center", "offset", "radius"],
    "snapVertices": ["mesh", "selection", "grid"], "mirror": ["mesh", "axis", "tolerance"], "decimate": ["mesh", "ratio"], "mergeCoplanar": ["mesh"],
    "loopCut": ["mesh", "edges", "cuts"], "knifeCut": ["mesh", "face", "start", "end"], "extrude": ["mesh", "faces", "distance"], "inset": ["mesh", "faces", "amount"],
    "subdivide": ["mesh", "faces"], "flip": ["mesh", "faces"], "deleteFaces": ["mesh", "faces"], "triangulate": ["mesh"], "weld": ["mesh", "tolerance"],
    "orient": ["mesh"], "fillHoles": ["mesh"], "inspectVertex": ["mesh", "index"], "inspectEdge": ["mesh", "index"], "inspectFace": ["mesh", "index"],
    "analyze": ["mesh"], "exportObj": ["mesh"], "exportJson": ["mesh"],
}
analyze_out = ["vertices", "faces", "edges", "triangles", "boundaryEdges", "nonManifoldEdges", "inconsistentEdges", "degenerateTriangles", "area", "volume", "minimum", "maximum"]
mesh_out = {"toBrep": ["geometry"], "analyze": analyze_out, "inspectVertex": ["point"], "inspectEdge": ["start", "end", "length"], "inspectFace": ["vertices", "normal", "center"], "exportObj": ["text"], "exportJson": ["text"]}

new_in, new_out = {}, {}
for name in os.listdir(D):
    if name.startswith("🔣️") and name != "🔣️.json":
        for kind in json.load(open(os.path.join(D, name), encoding="utf8"))["kinds"]:
            new_in[kind["id"]] = [p["name"] for p in kind["inputs"]]
            new_out[kind["id"]] = [p["name"] for p in kind["outputs"]]


def diff(old, new):
    if old == new:
        return ""
    removed = [name for name in old if name not in new]
    added = [name for name in new if name not in old]
    if not removed and not added:
        return ""
    if not removed:
        return "added " + ", ".join(f"`{n}`" for n in added)
    if not added:
        return "removed " + ", ".join(f"`{n}`" for n in removed)
    if len(removed) == len(added):
        return ", ".join(f"`{a}` -> `{b}`" for a, b in zip(removed, added))
    return "removed " + ", ".join(f"`{n}`" for n in removed) + "; added " + ", ".join(f"`{n}`" for n in added)


rows = []
for entry in mapping:
    was, new = entry["was"], entry["id"]
    if not was:
        continue
    if was.startswith("brep.mesh."):
        op = was.split(".")[-1]
        oi, oo = mesh_in[op], mesh_out.get(op, ["meshOut"])
    else:
        oi, oo = old_in[was], old_out.get(was) or []
    parts = []
    di = diff(oi, new_in[new])
    if di:
        parts.append("in: " + di)
    do = diff(oo, new_out[new]) if oo else ""
    if do:
        parts.append("out: " + do)
    if was == "brep.bool.fuse" or was == "brep.bool.cut" or was == "brep.bool.intersect":
        parts.append("out: `solid` -> `shape`")
    if was == "brep.brep":
        parts.append("out: `V,E,F,S,SE,SF,SI` -> `vertices, edges, faces, shells, selectedEdges, selectedFaces` (sourceIndex dropped)")
    if entry["note"]:
        parts.append(entry["note"])
    rows.append(f"| `{was}` | `{new}` | {'; '.join(parts)} |")

open(G + "mapping-table.md", "w", encoding="utf8").write("\n".join(rows) + "\n")
print(len(rows))
