import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏗️project-checks/🦀️.rs"
ctx = {"doc": "&En1997Snapshot", "report": "&mut CheckReport", "approach": "DesignApproach", "annex": "AnnexChoice", "part1": "&str", "part2": "&str", "part1054": "&str", "gwl": "f64", "gk": "u8",
       "footing": "&SpreadFoundation", "layer": "SoilLayer", "overburden": "f64", "phi_char": "f64", "path_w": "String", "label": "LocalizedCopy", "gamma_below": "f64", "phi_use": "f64",
       "foot_gov_u": "f64", "foot_gov_sit": "String", "foot_gov_check": "String"}
gov = [["> foot_gov_u {", "> *foot_gov_u {"], ["foot_gov_u = ", "*foot_gov_u = "], ["foot_gov_sit = ", "*foot_gov_sit = "], ["foot_gov_check = ", "*foot_gov_check = "]]
s = phspec.build(F, "check_footings", [
 ("check_footing_load_cases", "Checks the bearing, sliding, eccentricity and settlement of every load case of one spread foundation and tracks the governing one.", 295, 295,
   {"modes": {"foot_gov_u": "mut", "foot_gov_sit": "mut", "foot_gov_check": "mut"}, "rewrite": gov}),
 ("check_footing_uplift_and_bounds", "Checks the uplift and the dimension bounds of one spread foundation.", 482, 482),
 ("check_footing_governing_situation", "Reports the governing situation of one spread foundation.", 558, 581),
], types=ctx, modes={"layer": "ref", "label": "ref", "path_w": "ref", "gamma_below": "val", "phi_use": "val", "foot_gov_u": "val", "foot_gov_sit": "ref", "foot_gov_check": "ref"}, keep="auto", nth=0, block_line=269, context=ctx)
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
