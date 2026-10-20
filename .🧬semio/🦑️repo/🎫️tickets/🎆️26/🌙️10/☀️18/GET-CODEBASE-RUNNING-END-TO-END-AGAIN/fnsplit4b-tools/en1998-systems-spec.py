import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🧱️structural-system-checks/🦀️.rs"
c1 = child("🧭", "direction-and-base-shear", "Direction and base shear checks: the analysis direction and the seismic base shear of one structural system.")
c2 = child("🏬", "storey-forces-and-drift", "Storey force and drift checks: the accidental torsion, the storey shear demand, the drift and the second order effects of one structural system.")
c3 = child("🔗", "ductility-and-masonry", "Ductility and masonry checks: the material specific ductility detailing and the simple masonry wall ratio of one structural system.")
M = "semio_s_artifact_norm_en1998_model::"
ctx = {"document": "&En1998Snapshot", "annex": "AnnexChoice", "report": "&mut CheckReport", "gamma_i": "f64", "a_g": "f64", "s": "f64", "tb": "f64", "tc": "f64", "td": "f64", "building": "&" + M + "En1998Building",
       "bid": "&String", "bpath": "&String", "blabel": "&LocalizedCopy", "masses": "&Vec<f64>", "heights": "&Vec<f64>", "total_mass": "f64", "t1": "f64", "system": "&" + M + "En1998System"}
types = {n: "f64" for n in "q q_max s_e s_d lambda f_b plan_w e_ai delta_torsion remaining_v remaining_weight shear_above theta_lim".split()}
types.update({"spath": "String", "forces": "Vec<f64>", "forces_raw": "Vec<f64>", "gov_dc": "&String"})
modes = {k: "val" for k, v in {**ctx, **types}.items() if v in ("f64", "AnnexChoice")}
modes["spath"] = "ref"
spec = {"file": F, "function": "check_building_systems", "block_line": 26, "context": ctx, "types": types, "modes": modes, "keep": [], "phases": [
    ph("check_system_direction", "🧭 Checks that the analysis direction of the system is known and unique.", 27, 27, c1, drop_ins=["s"]),
    ph("check_system_base_shear", "🪨 Checks the behaviour factor, the design spectrum and the base shear of the system against its resistance.", 62, 134, c1),
    ph("check_storey_forces", "🏬 Checks the accidental torsion amplified storey forces and storey shears against the resistances.", 137, 185, c2, drop_ins=["s"]),
    ph("check_drift_and_second_order", "📐 Checks the interstorey drift and the second order sensitivity of every storey.", 243, 248, c2, drop_ins=["s"]),
    ph("check_member_ductility", "🔗 Checks the material specific ductility detailing of every member.", 313, 314, c3),
    ph("check_simple_masonry", "🧱 Checks the simple masonry wall area ratio.", 571, 571, c3),
]}
dump(spec, sys.argv[1])
