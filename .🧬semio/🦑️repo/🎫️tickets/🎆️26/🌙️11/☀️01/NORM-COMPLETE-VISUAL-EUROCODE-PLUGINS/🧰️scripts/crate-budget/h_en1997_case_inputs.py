"""🪚 Crate-budget H: lift the generated EN 1997 case input vocabulary (data only) out of the checks crate."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ART = NORM + "/🗿️artifacts/🌍️en1997"
CHK = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
LEAF = CHK + "/📇️catalogue/📚️inputs"
PKG = LEAF + "/📦️packages/🦀️rust"
OLD, NEW = "semio-s-artifact-norm-en1997-checks-catalogue", "semio-s-artifact-norm-en1997-checks-case-inputs"

if not os.path.exists(PKG + "/Cargo.toml"):
    wr(PKG + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1997 verification catalogue input vocabulary: unit, bilingual label and choices of every case input"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % (relp(ART, PKG), NEW))
register_workspace(ART, NEW, PKG)
add_dep(CHK + "/🧬️catalogue-core/📦️packages/🦀️rust/Cargo.toml", NEW)
script_register(ART, OLD, NEW)
lock_register(OLD, NEW, extra_old_deps=[NEW], new_deps=["semio-s-plugin-norm-compliance"])
f = CHK + "/📇️catalogue/🦀️.rs"
s = rd(f)
m = '#[path = "📚️inputs/🦀️.rs"]\npub mod case_inputs;\n'
if m in s:
    wr(f, s.replace(m, "pub use semio_s_artifact_norm_en1997_checks_case_inputs as case_inputs;\n", 1))
