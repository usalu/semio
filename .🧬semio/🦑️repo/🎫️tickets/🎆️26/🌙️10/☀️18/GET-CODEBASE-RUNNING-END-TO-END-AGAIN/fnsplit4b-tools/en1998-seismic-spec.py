import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🦀️.rs"
c1 = child("🏢", "building-regularity-and-base-shear-checks", "Building regularity and base shear checks: the building mass, period, regularity and lateral force method checks of the full seismic check.")
c2 = child("🧱", "structural-system-checks", "Structural system checks: the per system checks of every building of the full seismic check.")
M = "semio_s_artifact_norm_en1998_model::"
ctx = {"document": "&En1998Snapshot", "annex": "AnnexChoice", "report": "CheckReport", "low_seismicity": "bool", "gamma_i": "f64", "a_g": "f64", "s": "f64", "tb": "f64", "tc": "f64", "td": "f64", "building": "&" + M + "En1998Building"}
types = {"bid": "&String", "bpath": "String", "blabel": "LocalizedCopy", "total_height": "f64", "n_storeys": "usize", "masses": "Vec<f64>", "heights": "Vec<f64>", "total_mass": "f64", "t1": "f64", "lfm_ok": "bool", "systems": "&Vec<" + M + "En1998System>"}
modes = {k: "val" for k, v in {**ctx, **types}.items() if v in ("f64", "bool", "usize", "AnnexChoice")}
modes["report"] = "mut"
modes["bpath"] = "ref"
spec = {"file": F, "function": "check_full_seismic", "block_line": 107, "context": ctx, "types": types, "modes": modes, "keep": [108, 109, 110, 112, 382], "phases": [
    ph("check_building_base_shear", "🏢 Checks the building masses, the fundamental period, the regularity and the lateral force method of one building.", 122, 364, c1),
    ph("check_building_systems", "🧱 Checks every structural system of one building.", 394, 394, c2),
]}
dump(spec, sys.argv[1])
