"""🪚 Crate-budget F: lift the pure EN 1994 catalogue (formulas, continuous, encased, fire, fatigue, bridge, crack width) out of the checks crate into its own sibling crate."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ART = NORM + "/🗿️artifacts/🧩️en1994"
CHK = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
PKG = CHK + "/🧮️catalogue/📦️packages/🦀️rust"
OLD, NEW = "semio-s-artifact-norm-en1994-checks", "semio-s-artifact-norm-en1994-checks-catalogue"
DEPS = ["semio-s-plugin-norm-compliance", "semio-s-plugin-norm-mechanics"]

if not os.path.exists(PKG + "/Cargo.toml"):
    wr(PKG + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1994 catalogue formulas: SI functions, continuous beams, encased columns, fire, fatigue, bridge actions and crack width"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true }\nsemio-s-plugin-norm-mechanics = { workspace = true }\n\n[dev-dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true, features = ["compliance-testing"] }\n' % (relp(ART, PKG), NEW))
register_workspace(ART, NEW, PKG)
add_dep(CHK + "/📦️packages/🦀️rust/Cargo.toml", NEW)
script_register(ART, OLD, NEW)
lock_register(OLD, NEW, extra_old_deps=[NEW], new_deps=DEPS)

lib = CHK + "/🦀️.rs"
s = rd(lib)
mount = '#[path = "🧮️catalogue/🦀️.rs"]\npub mod catalogue;\n'
if mount in s:
    s = s.replace(mount, "pub use semio_s_artifact_norm_en1994_checks_catalogue as catalogue;\n", 1)
    wr(lib, s)
t = CHK + "/🧮️catalogue/🧪️tests/🦀️.rs"
s = rd(t)
old = '"../../../../../🔮️oracles/🧮️catalogue/🐍️.py"'
if old in s:
    wr(t, s.replace(old, '"../../../../../../🔮️oracles/🧮️catalogue/🐍️.py"'))
