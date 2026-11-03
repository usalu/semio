"""🪚 Crate-budget F: lift the generated spec-trace table (data only) of an artifact out of its checks crate: f_spec_trace.py <artifact dir name> <code>."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

art_dir, code = sys.argv[1], sys.argv[2]
ART = NORM + "/🗿️artifacts/" + art_dir
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
CHK = ANY + "/🧬️schema/💡️inferences/⚖️checks"
PKG = ANY + "/🧭️spec-trace/📦️packages/🦀️rust"
OLD, NEW = "semio-s-artifact-norm-%s-checks" % code, "semio-s-artifact-norm-%s-spec-trace" % code
assert os.path.exists(ANY + "/🧭️spec-trace/🦀️.rs")
if not os.path.exists(PKG + "/Cargo.toml"):
    wr(PKG + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN %s specification traceability table: every spec id with its clause, entity and realisation"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % (relp(ART, PKG), NEW, code[2:]))
register_workspace(ART, NEW, PKG)
add_dep(CHK + "/📦️packages/🦀️rust/Cargo.toml", NEW)
script_register(ART, OLD, NEW)
lock_register(OLD, NEW, extra_old_deps=[NEW], new_deps=["semio-s-plugin-norm-compliance"])
f = CHK + "/🦀️.rs"
s = rd(f)
m = '#[path = "../../../🧭️spec-trace/🦀️.rs"]\npub mod spec_trace;\n'
assert m in s
wr(f, s.replace(m, "pub use %s as spec_trace;\n" % NEW.replace("-", "_"), 1))
