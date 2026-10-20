import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️part2/🦀️.rs"
M = "semio_s_artifact_norm_iso16757_model::part_2::"
ctx = {"doc": "&Iso16757Snapshot", "report": "&mut CheckReport", "id": "&String", "obj": "&" + M + "GeometryObject", "EXPECTED": "&[(&str, &[&str])]"}
s = phspec.build(F, "check_ports_primitives_and_spaces", [
 ("check_object_id_and_spaces", "Checks the id of one geometry object and the ids and envelopes of its spaces.", 533, 572),
 ("check_object_ports", "Checks the media, types and primitive parameters of the ports of one geometry object.", 634, 634),
], types=ctx, modes={}, nth=0, block_line=532, context=ctx)
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
