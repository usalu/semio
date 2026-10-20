import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️part2/🦀️.rs"
M = "semio_s_artifact_norm_iso16757_model::part_2::"
ctx = {"geometry": "&" + M + "GeometryCatalogue", "id": "&String", "obj": "&" + M + "GeometryObject", "report": "&mut CheckReport",
       "graph_fail": "u32", "volume_fail": "u32", "port_fail": "u32", "space_fail": "u32", "binding_fail": "u32", "objects_ok": "u32"}
modes = {k: "mut" for k in ("graph_fail", "volume_fail", "port_fail", "space_fail", "binding_fail", "objects_ok")}
inc = lambda *names: {"rewrite": [[n + " += 1", "*" + n + " += 1"] for n in names]}
s = phspec.build(F, "check_part_2", [
 ("check_geometry_graph_validity", "Validates the geometry graph of one object and counts a failure.", 36, 38, inc("graph_fail")),
 ("check_geometry_shape_and_volume", "Checks the bounding box, volume, space and graph of one geometry object.", 67, 67, inc("volume_fail", "objects_ok", "space_fail", "graph_fail")),
 ("check_geometry_ports_and_bindings", "Checks the ports and the parameter bindings of one geometry object.", 260, 317, inc("port_fail", "binding_fail")),
], types=ctx, modes=modes, nth=0, out=None, block_line=35, context=ctx)
import json
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
