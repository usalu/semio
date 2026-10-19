import os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
from mkcomp import rebase
write = "--write" in sys.argv
WS = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx"
SCH = WS + "/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema"
MOD = SCH + "/📦️packages/🦀️rust"
D = SCH + "/🏷️vocabulary/📦️packages/🦀️rust"
NAME = "semio-s-artifact-stdio-pptx-base-vocabulary"
IDENT = NAME.replace("-", "_")
modman = rd(MOD + "/Cargo.toml")
def dep(line_name):
    m = re.search(r"(?m)^%s = .*$" % re.escape(line_name), modman)
    assert m, line_name
    return rebase(m.group(0), MOD, D)
deps = [dep("semio-framework-value"), "semio-s-artifact-stdio-xml = { workspace = true }", "semio-s-artifact-stdio-zip = { workspace = true }"]
text = rd(SCH + "/🏷️vocabulary/🦀️.rs") + rd(SCH + "/⚠️refusal/🦀️.rs") + rd(SCH + "/📸️snapshot/🧭️transform/🦀️.rs")
assert "crate::" not in text and "super::" not in text
man = '''[package]
workspace = "../../../../../../../.."
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "🏷️ The pptx ecma-376 base vocabulary: namespaces, part names, XML helpers, refusals and the signed64 transform scalar."

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
%s
''' % (NAME, "\n".join(deps))
root = '''//! 🏷️ The pptx ecma-376 base vocabulary: namespaces, part names, XML helpers, refusals and the signed64 transform scalar.

#[path = "../../🦀️.rs"]
pub mod vocabulary;
#[path = "../../../⚠️refusal/🦀️.rs"]
pub mod refusal;
#[path = "../../../📸️snapshot/🧭️transform/🦀️.rs"]
pub mod transform;
'''
edits = {}
sp = SCH + "/🦀️.rs"; t = rd(sp)
old = '#[path="🏷️vocabulary/🦀️.rs"]\npub mod vocabulary;\n#[path="⚠️refusal/🦀️.rs"]\npub mod refusal;\n'
assert old in t
edits[sp] = t.replace(old, "pub use %s::{refusal, vocabulary};\n" % IDENT, 1)
np = SCH + "/📸️snapshot/🦀️.rs"; t = rd(np)
old = '#[path = "🧭️transform/🦀️.rs"]\nmod transform;\n'
assert old in t
edits[np] = t.replace(old, "use %s::transform;\n" % IDENT, 1)
edits[MOD + "/Cargo.toml"] = modman.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % NAME, 1)
for p, t in edits.items(): print("edit", p[len(R) + 1:][-60:])
print(man); print(root)
if not write: sys.exit(0)
for p, t in edits.items(): wr(p, t)
os.makedirs(D, exist_ok=True); assert not os.path.exists(D + "/Cargo.toml")
open(D + "/Cargo.toml", "w", encoding="utf8").write(man); open(D + "/🦀️.rs", "w", encoding="utf8").write(root)
add_member(WS, D, MOD)
print("ws dep", add_ws_dep(WS, NAME, D, "semio-s-artifact-stdio-pptx-base-model"))
add_owner(NAME, D)
print("done")
