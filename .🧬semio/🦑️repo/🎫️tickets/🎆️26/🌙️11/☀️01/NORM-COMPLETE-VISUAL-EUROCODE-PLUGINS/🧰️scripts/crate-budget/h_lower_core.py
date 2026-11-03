"""🪚 Crate-budget H: `h_lower_core.py <artifact dir> <crate dir below ✳️any> <parent crate> <suffix> <kept module dir>...` moves a crate root wholesale into the lower crate `<parent>-<suffix>`; the kept module dirs and the test mounts stay in the host."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
art, sub, parent, suffix = sys.argv[1:5]
ART = NORM + "/🗿️artifacts/" + art
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/" + sub
lower_core(ART, D, parent, suffix, set(sys.argv[5:]), "%s core: every module except %s" % (parent.split("norm-")[1], ", ".join(sys.argv[5:])), D + "/📦️packages/🦀️rust/Cargo.toml")
