"""🪚 Crate-budget H: the `EDIT_RULES` table of an artifact mutations crate (data only) becomes the crate `-mutations-edit-rules` (root `🧭️edit-rules/📚️table`); the leaf edits keep importing it. `h_edit_rules.py <artifact dir> <code>`"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
art_dir, code = sys.argv[1], sys.argv[2]
ART = NORM + "/🗿️artifacts/" + art_dir
M = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
P = "semio-s-artifact-norm-%s-mutations" % code
N = P + "-edit-rules"
f = M + "/🧭️edit-rules/🦀️.rs"
t = rd(f)
mark = "//#region 🧭️LeafEdits"
if N.replace("-", "_") in t:
    sys.exit("done")
i = t.index(mark)
table = t[:i].rstrip("\n") + "\n"
table = table.replace("//! 🧭️ The `setField`", "//! 🧭️ The table of the `setField`", 1)
wr(M + "/🧭️edit-rules/📚️table/🦀️.rs", table)
wr(f, "//! 🧭️ The leaf edits of the %s editor: the typed rule table is the crate `%s`. See [`%s`].\n\npub use %s::EDIT_RULES;\n\n" % (code.upper(), N, N.replace("-", "_"), N.replace("-", "_")) + t[i:])
toml = M + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, M + "/🧭️edit-rules/📚️table", N, "%s editor rule table: which value-tree path raises which mutation kind" % code.upper(), ["semio-s-plugin-norm-app-surface"], P, toml)
add_dep(toml, N)
