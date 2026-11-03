"""🪚 EN 1992 checks: the generated input vocabulary becomes `-checks-case-inputs`."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from d_lib import *
ART = NORM + "/🗿️artifacts/🏛️en1992"
C = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
P = "semio-s-artifact-norm-en1992-checks"
N = P + "-case-inputs"
toml = C + "/📦️packages/🦀️rust/Cargo.toml"
reg = C + "/🗄️registry/🦀️.rs"
s = rd(reg)
old = '#[path = "📚️inputs/🦀️.rs"]\npub mod case_inputs;'
if old in s:
    new_crate(ART, C + "/🗄️registry/📚️inputs", N, "EN 1992 catalogue case input vocabulary", ["semio-s-plugin-norm-compliance"], P, toml)
    add_dep(toml, N)
    wr(reg, s.replace(old, "pub use %s as case_inputs;" % N.replace("-", "_")))
    f = C + "/🗄️registry/📚️inputs/🦀️.rs"
    wr(f, rd(f).replace(" See [`super`].", "", 1))
