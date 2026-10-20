import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏗️project-checks/🦀️.rs"
phspec.build(F, "check_project", [
 ("check_soil_layers", "Checks the strength parameters and the id uniqueness of every soil layer.", 57, 57),
 ("check_footings", "Checks the bearing, sliding, eccentricity and settlement of every spread foundation.", 218, 218),
], types={"part1": "&str", "part2": "&str", "annex": "AnnexChoice", "report": "CheckReport", "approach": "DesignApproach", "gwl": "f64", "gk": "u8", "part1054": "&str", "design_situations": "Vec<String>"}, modes={"report": "mut", "annex": "val", "approach": "val", "gwl": "val", "gk": "val", "design_situations": "ref"}, keep="auto", out=sys.argv[1])
