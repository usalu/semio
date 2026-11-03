"""🧪 Moves the en1993-scene unit tests that reach the eigen-modes and detail-sketches internals next to that code in the members crate."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

B = NORM + "/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any"
F, M = B + "/🎬️scene", B + "/🎬️scene-members"
tp = F + "/🧪️tests/🔬️unit/🦀️.rs"
s = rd(tp)
a = s.index("fn strut()")
b = s.index("#[test]\nfn members_are_coloured_per_station")
eig = s[a:b]
c = s.index("#[test]\nfn the_effective_width_of_the_compliant_panel")
d = s.index("#[test]\nfn the_detail_sketches_exist_in_3d")
det = s[c:d]
if not os.path.exists(M + "/🌀️eigen-modes/🧪️tests/🔬️unit/🦀️.rs"):
    e2 = eig.replace("eigen::", "").replace("kit::", "crate::kit::").replace("model::", "crate::model::").replace("scene::Vec3", "crate::scene::Vec3").replace("norm::python_oracle", "crate::norm::python_oracle").replace("crate::crate::", "crate::")
    wr(M + "/🌀️eigen-modes/🧪️tests/🔬️unit/🦀️.rs", "use super::*;\nuse crate::model::En1993Snapshot;\n\nfn compliant() -> En1993Snapshot {\n    En1993Snapshot::compliant_heb240_frame()\n}\n\n" + e2.rstrip("\n") + "\n")
    d2 = det.replace("detail_sketches::", "").replace("model::SteelJoint", "crate::model::SteelJoint")
    wr(M + "/🧩️detail-sketches/🧪️tests/🔬️unit/🦀️.rs", "use super::*;\nuse crate::model::En1993Snapshot;\n\nfn compliant() -> En1993Snapshot {\n    En1993Snapshot::compliant_heb240_frame()\n}\n\n" + d2.rstrip("\n") + "\n")
    for m in ("🌀️eigen-modes", "🧩️detail-sketches"):
        p = M + "/" + m + "/🦀️.rs"
        wr(p, rd(p).rstrip("\n") + '\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod unit;\n')
    os.makedirs(M + "/🔮️oracles", exist_ok=True)
    os.rename(F + "/🔮️oracles/🌀️modes.py", M + "/🔮️oracles/🌀️modes.py")
    wr(tp, s[:a] + s[b:c] + s[d:])
root = F + "/🦀️.rs"
r = rd(root)
r = r.replace("use scene_members::{kit, members, eigen, tracks, joints, detail_sketches};", "use scene_members::{kit, tracks, detail_sketches};")
wr(root, r)
