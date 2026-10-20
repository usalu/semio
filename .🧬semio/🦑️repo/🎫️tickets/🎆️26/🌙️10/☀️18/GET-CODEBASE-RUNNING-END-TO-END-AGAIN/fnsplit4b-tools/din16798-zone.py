import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏠️zone-evaluation/🦀️.rs"
M = "semio_s_artifact_norm_din16798_model::"
names = open("/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/part1-names.txt").read()
types = {"out": "Vec<CheckResult>", "cat": "ComfortCategory", "usage": "OccupancyType", "pollution": "PollutionClass", "method": "VentMethod", "path": "dyn Fn(&str) -> String",
         "linked": "Option<&" + M + "VentSystemDocument>", "linked_name": "&str", "linked_type": "Option<VentSystemType>"}
modes = {"out": "mut", "cat": "val", "usage": "val", "pollution": "val", "method": "val", "linked_type": "val", "path": "ref", "linked": "val"}
s = phspec.build(F, "evaluate_zone", [
 ("check_zone_ventilation_link", "Checks the ventilation system a zone links to.", 17, 17),
 ("check_zone_temperatures", "Checks the operative and the adaptive temperature of a zone against its comfort category.", 62, 108),
 ("check_zone_clothing_insulation", "Checks the clothing insulation and the seasonal comfort bands of a zone.", 110, 112),
 ("check_zone_air_quality", "Checks the ventilation method, the outdoor air flow and the carbon dioxide level of a zone.", 209, 285),
 ("check_zone_humidity_light_noise_and_draught", "Checks the humidity, illuminance, noise and draught of a zone.", 287, 356, {"allow_control": [346]}),
], types=types, modes=modes, keep="auto", prelude_ifs=[{"regex": r"\b(%s)\b" % names, "line": "use part_1::*;"}], out=sys.argv[1])
