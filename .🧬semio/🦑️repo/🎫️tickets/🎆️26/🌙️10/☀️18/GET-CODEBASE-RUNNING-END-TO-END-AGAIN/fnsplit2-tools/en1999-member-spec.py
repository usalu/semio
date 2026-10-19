import sys, glob
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = glob.glob(ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️member-checks/🦀️.rs")[0]
c1 = child("🧭", "classification-and-axial-checks", "Classification and axial checks: the section classification and the tension, compression and combined axial checks of the member checks.")
c2 = child("🔧", "bending-and-shear-checks", "Bending and shear checks: the bending about both axes and the shear about both axes of the member checks.")
c3 = child("🏛", "buckling-and-interaction-checks", "Buckling and interaction checks: the flexural, torsional and lateral torsional buckling and the interaction of the member checks.")
c4 = child("📉", "serviceability-checks", "Serviceability checks: the quasi permanent deflection and the characteristic and frequent stresses of the member checks.")
types = {"params": "na_de::AnnexParams", "alloy": "part_1_1::AlloyProps", "class": "part_1_1::SectionClass", "has_weld": "bool", "el_dig": "String", "act_dig": "String", "sup_dig": "String", "out": "Vec<CheckResult>", "part": "&str", "action_id": "&str", "combo": "&'static str", "gov": "part_en1990::GoverningEffects",
         "path_n": "String", "path_m": "String", "path_mz": "String", "path_vy": "String", "eps": "f64", "sls_qp": "part_en1990::GoverningEffects", "sls_char": "part_en1990::GoverningEffects"}
for n in "a_gross i_y i_z i_t i_w a_eff wel_eff sigma_lim alpha lambda_0 n_ed v_y v_z m_y m_z k_sup l_cr_y l_cr_z l_cr_t l_lt n_rd m_rd v_rd_b m_rd_z lb_y lb_z lb_t_slend n_b_rd n_b_rd_y n_b_rd_z m_b_rd chi chi_y chi_z".split():
    types[n] = "f64"
modes = {k: "val" for k, v in types.items() if v in ("f64", "bool")}
modes["out"] = "mut"
modes["annex"] = "val"
keep = [82, 83, 104, 105, 106, 107, 108, 109, 110, 111, 112, 114, 115, 116, 117, 118, 119] + list(range(162, 166)) + [166, 171, 172, 173, 174, 809]
spec = {"file": F, "function": "check_member", "types": types, "modes": modes, "keep": keep, "phases": [
    ph("check_classification", "🧭 Classifies every plate element of the section, one check per element.", 122, 123, c1),
    ph("check_tension_and_compression", "⬍ Checks the tension and compression resistance of the cross section.", 177, 204, c1),
    ph("check_bending_y", "🔧 Checks the bending resistance about the strong axis.", 221, 248, c2),
    ph("check_shear_z", "✂ Checks the shear resistance with the web buckling reduction.", 265, 284, c2),
    ph("check_bending_z", "🪛 Checks the bending resistance about the weak axis.", 301, 327, c2),
    ph("check_shear_y", "🪚 Checks the shear resistance of the weak axis.", 344, 353, c2),
    ph("check_combined_axial_bending", "🔀 Checks the simplified combined axial and bending utilization.", 370, 379, c1),
    ph("check_flexural_buckling", "🏛 Checks the flexural buckling resistance about both axes.", 396, 473, c3),
    ph("check_slenderness_limits", "📏 Checks the slenderness about the strong axis and the torsional slenderness and reports the critical lengths.", 489, 546, c3),
    ph("check_torsional_buckling", "🌀 Checks the torsional and flexural torsional buckling resistance.", 563, 580, c3),
    ph("check_lateral_torsional_buckling", "🦋 Checks the lateral torsional buckling resistance.", 597, 622, c3),
    ph("check_buckling_interaction", "🧲 Checks the interaction of compression and bending buckling.", 639, 656, c3),
    ph("check_deflection", "📉 Checks the quasi permanent deflection against the span limit.", 673, 714, c4),
    ph("check_characteristic_stress", "🧪 Checks the elastic stress of the characteristic combination.", 729, 753, c4),
    ph("check_frequent_stress", "🔁 Checks the elastic stress of the frequent combination.", 769, 793, c4),
]}
dump(spec, sys.argv[1])
