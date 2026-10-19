import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📋️member-evaluation/🦀️.rs"
c1 = child("📐", "cover-and-constitutive-checks", "Cover and constitutive checks: the durability cover and the constitutive relation checks of the member evaluation.")
c2 = child("🏗", "resistance-and-serviceability-checks", "Resistance and serviceability checks: the ultimate resistances, the crack width, the reinforcement limits, the deflection and the serviceability stress limits of the member evaluation.")
c3 = child("🧱", "slenderness-prestress-and-ductility-checks", "Slenderness, prestress and ductility checks: the column slenderness, the prestress limits and the reinforcement ductility of the member evaluation.")
c4 = child("🔥", "fire-anchorage-bridge-and-liquid-checks", "Fire, anchorage, bridge and liquid checks: the fire resistance, the anchorage and laps, the bridge stress and the liquid retaining tightness of the member evaluation.")
types = {"out": "Vec<CheckResult>", "annex": "AnnexChoice", "concrete": "&semio_s_artifact_norm_en1992_model::ConcreteGrade", "reinf": "&semio_s_artifact_norm_en1992_model::ReinforcementGrade", "params": "na_de::AnnexParams", "effects": "Vec<DesignEffects>",
         "uls": "Option<&DesignEffects>", "acc": "Option<&DesignEffects>", "sls_qp": "Option<&DesignEffects>", "sls_char": "Option<&DesignEffects>"}
for n in "f_ck f_yk e_s eps_cu2 n_parabola f_ck_cube b d h a_s rho".split():
    types[n] = "f64"
modes = {k: "val" for k in "annex uls acc sls_qp sls_char f_ck f_yk e_s eps_cu2 n_parabola f_ck_cube b d h a_s rho".split()}
modes["out"] = "mut"
keep = [7, 8, 9, 26] + list(range(43, 55)) + [106, 107, 108, 109, 110, 860]
spec = {"file": F, "function": "evaluate_member", "types": types, "modes": modes, "keep": keep, "phases": [
    ph("check_cover", "📐 Checks the concrete cover against the durability minimum c_nom.", 57, 82, c1),
    ph("check_constitutive", "🧪 Checks the constitutive relation of the concrete grade against Table 3.1.", 85, 85, c1),
    ph("check_ultimate_resistances", "🏗 Checks the ultimate resistances of the governing ultimate situation.", 112, 112, c2, drop_ins=["params"]),
    ph("check_crack_width", "🪓 Checks the crack width of the quasi permanent situation.", 332, 332, c2),
    ph("check_reinforcement_limits", "🔩 Checks the minimum and maximum tension reinforcement.", 371, 413, c2),
    ph("check_deflection", "📉 Checks the span to depth deflection limit.", 416, 416, c2),
    ph("check_sls_stress_limits", "⚖ Checks the serviceability stress limits of the characteristic and quasi permanent situations.", 666, 714, c2),
    ph("check_column_slenderness", "🏛 Checks the column slenderness and the nominal curvature.", 445, 445, c3),
    ph("check_prestress", "🎻 Checks the prestress steel and the stress limits.", 491, 491, c3),
    ph("check_ductility", "🧵 Checks the reinforcement ductility class against Annex C.", 598, 598, c3),
    ph("check_fire", "🔥 Checks the fire resistance minimum dimensions and axis distances.", 623, 623, c4),
    ph("check_anchorage_and_laps", "⚓ Checks the anchorage lengths and the lap lengths of the longitudinal layers.", 737, 737, c4),
    ph("check_bridge", "🌉 Checks the bridge stress limits and the fatigue stress range.", 794, 794, c4),
    ph("check_liquid_retaining", "💧 Checks the tightness crack limit of liquid retaining members.", 830, 830, c4),
]}
dump(spec, sys.argv[1])
