"""🪚 Crate-budget H: the EN 1998 structural system checks become the crate `-inferences-structural-system` (root = the module file); the full seismic check imports `check_building_systems`."""
import glob, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences"
P = "semio-s-artifact-norm-en1998-inferences"
N = P + "-structural-system"
S = D + "/🌍️full-seismic-check/🧱️structural-system-checks"
parent = D + "/🌍️full-seismic-check/🦀️.rs"
r = rd(parent)
if N.replace("-", "_") in r:
    sys.exit("done")
retarget_visibility(glob.glob(S + "/**/*.rs", recursive=True))
root = rd(S + "/🦀️.rs")
imports = "use semio_s_artifact_norm_en1998_model::En1998Snapshot;\nuse semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};\nuse semio_s_artifact_norm_en1998_schema::{na_de, part_1, wall_plan};\nuse semio_s_artifact_norm_en1998_inferences_details::seismic_helpers::*;\n"
assert "use super::*;\n" in root
wr(S + "/🦀️.rs", root.replace("use super::*;\n", imports, 1))
old = '#[path = "🧱️structural-system-checks/🦀️.rs"]\nmod structural_system_checks;\nuse structural_system_checks::{check_building_systems};\n'
assert old in r
wr(parent, r.replace(old, "use %s::check_building_systems;\n" % N.replace("-", "_")))
old_toml = D + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, S, N, "EN 1998 structural system checks of the full seismic check", ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance", P + "-details"], P, old_toml)
add_dep(old_toml, N)
