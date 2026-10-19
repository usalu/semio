import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
DM = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx"
B = DM + "/🏅️standards/🔖️ecma-376/🪆️subsets"
BASE = B + "/🧱️base"
M = BASE + "/🧬️schema/🧬️mutations"
MPKG = M + "/📦️packages/🦀️rust"
OLD = "semio-s-artifact-stdio-docx-base-mutations"
NEWER = "semio-s-artifact-stdio-docx-base-edit-rules"
NEWXA = "semio-s-artifact-stdio-docx-base-xml-address"
npkg_er = M + "/🧭️edit-rules/📦️packages/🦀️rust"
for sub in ("📏️strict", "🔄️transitional", "🧱️base"):
    ed = B + "/" + sub + "/✏️editor/🦀️.rs"
    t = rd(ed)
    line = [l for l in t.split("\n") if l.startswith("use semio_s_artifact_stdio_docx_base_mutations::") and l.endswith("::mutations::edit_rules;")]
    if line:
        t = t.replace(line[0], "use semio_s_artifact_stdio_docx_base_edit_rules as edit_rules;", 1)
        wr(ed, t)
    mf = B + "/" + sub + "/📦️packages/🦀️rust/Cargo.toml"
    add_dep_like(mf, OLD, NEWER, npkg_er)
et = M + "/🧭️edit-rules/🧪️tests/🦀️.rs"
e = rd(et)
old_use = "use semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::edit_rules::*;\n"
if old_use in e:
    wr(et, e.replace(old_use, "use semio_s_artifact_stdio_docx_base_edit_rules::*;\n", 1))
add_dep_line(MPKG + "/Cargo.toml", "dependencies", NEWXA, '%s = { workspace = true }' % NEWXA)
print(run_owners(DM, True))
