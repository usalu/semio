import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️member-checks/🦀️.rs"
c1 = child("🏷", "section-class-checks", "Section class checks: the section classification and the plastic analysis admission of one member of the full steel structure check.")
c2 = child("🧮", "action-resistance-checks", "Action and resistance checks: the ultimate action rows against the member resistances and the serviceability deflection of one member of the full steel structure check.")
ctx = {"document": "&En1993Snapshot", "annex": "AnnexChoice", "report": "CheckReport", "params": "AnnexParams", "member": "&SteelMember"}
types = {"section": "&SteelSection", "material": "&SteelMaterial", "kind": "String", "class": "u8", "combinations": "Vec<CombinedEffects>"}
for n in "a_res w_y w_z iy iz lambda_y lambda_z chi_y chi_z n_b_rd_y n_b_rd_z n_b_rd m_cr lambda_lt chi_lt m_b_rd".split():
    types[n] = "f64"
types["curve_y"] = types["curve_z"] = types["curve_lt"] = "part_1_1::BucklingCurve"
modes = {k: "val" for k, v in {**types, **ctx}.items() if v in ("f64", "u8", "BucklingCurve", "AnnexChoice", "AnnexParams")}
modes["report"] = "mut"
keep = [41, 42, 43, 44] + [101, 102, 109, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 129, 130, 131, 132, 133, 135, 136]
spec = {"file": F, "function": "check_full_steel_structure", "block_line": 40, "context": ctx, "types": types, "modes": modes, "keep": keep, "phases": [
    ph("check_section_class", "🏷 Checks the section class of the member and the admission of the plastic analysis.", 46, 75, c1),
    ph("check_uls_actions", "🧮 Checks every governing ultimate action row of the member against the axial, bending, shear, buckling and interaction resistances.", 154, 155, c2),
    ph("check_sls_deflection", "📐 Checks the deflection of the characteristic combination against the member limit.", 564, 564, c2),
]}
dump(spec, sys.argv[1])
