"""🪚 Moves the en1999 checks `📏️parts` directory into its own crate `en1999-check-parts`; the checks root re-exports it."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ART = NORM + "/🗿️artifacts/🪶️en1999"
C = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
OLD = "semio-s-artifact-norm-en1999-checks"
NEW = "semio-s-artifact-norm-en1999-check-parts"
pr = C + "/📏️parts/🦀️.rs"
s = rd(pr)
if "use crate::CheckResult;\n" in s:
    s = s.replace("use crate::CheckResult;\n", "use semio_s_artifact_norm_en1999_model::{AluminiumConnection, AluminiumShell, ColdFormedSheet, FatigueDetail, SupportCondition};\nuse semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult};\n", 1)
    wr(pr, s)
root = C + "/🦀️.rs"
r = rd(root)
old = '#[path = "📏️parts/🦀️.rs"]\nmod parts;\npub use parts::*;\n'
if old in r:
    wr(root, r.replace(old, "pub use semio_s_artifact_norm_en1999_check_parts::*;\n", 1))
pkg = C + "/📏️parts/📦️packages/🦀️rust"
wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1999 check parts: national annex parameters, EN 1990 combinations and the pure clause functions of EN 1999-1-1 to 1-5"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-artifact-norm-en1999-fatigue-math = { workspace = true }\nsemio-s-artifact-norm-en1999-model = { workspace = true }\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % (relp(ART, pkg), NEW))
register_workspace(ART, NEW, pkg)
ct = C + "/📦️packages/🦀️rust/Cargo.toml"
t = rd(ct)
t = t.replace("semio-s-artifact-norm-en1999-fatigue-math = { workspace = true }\n", "semio-s-artifact-norm-en1999-check-parts = { workspace = true }\n", 1) if "fatigue-math" in t else t
wr(ct, t)
script_register(ART, OLD, NEW)
lock_register(OLD, NEW, new_deps=["semio-s-artifact-norm-en1999-fatigue-math", "semio-s-artifact-norm-en1999-model", "semio-s-plugin-norm-compliance"], drop_old_deps=["semio-s-artifact-norm-en1999-fatigue-math"])
