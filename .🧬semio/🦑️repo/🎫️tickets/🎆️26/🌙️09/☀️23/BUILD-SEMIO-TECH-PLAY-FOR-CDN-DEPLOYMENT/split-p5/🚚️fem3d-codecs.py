#!/usr/bin/env python3
"""🚚️ Prepares the fem3d codecs for the crate cut (orphan rule, as `🚚️fem2d-codecs.py` did after the fem2d cut): the
`🚪️io/{📝️text,💾️binary,🪶️sqlite}` module files go, their leaves are mounted inline under the root's `io` skeleton so the
cut can give snapshot/diff codecs to the model crate and mutation codecs to the mutations crate; the full-stack SQLite
test is mounted at the root (aggregator); the schema/inference descriptors move to the subset declaration.

usage: 🚚️fem3d-codecs.py
"""
import os, re
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
ANYR = "🏅️standards/🔖️1/🪆️subsets/🌐️any"
ANY = A + "/" + ANYR
IO = ANY + "/🚪️io"
rd = lambda p: open(p, encoding="utf-8").read()
def wr(p, s): open(p, "w", encoding="utf-8").write(s)

io = rd(IO + "/🦀️.rs")
for m in ["binary", "text", "sqlite"]:
    io, n = re.subn(r'#\[path = "[^"]+"\]\npub mod ' + m + r';\n\n?', "", io, count=1); assert n == 1, m
wr(IO + "/🦀️.rs", io)
for d in ["📝️text", "💾️binary", "🪶️sqlite"]:
    os.remove(f"{IO}/{d}/🦀️.rs")
leaf = IO + "/🪶️sqlite/📸️snapshot/🦀️.rs"; s = rd(leaf)
s, n = re.subn(r'\n#\[cfg\(test\)\]\n#\[path = "🧪️tests/🦀️.rs"\]\nmod tests;\n', "\n", s); assert n == 1
wr(leaf, s)
t = IO + "/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"
wr(t, rd(t).replace("crate::standards::v1::subsets::any::io::sqlite::snapshot::Fem3dSnapshot", "crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot"))

ind = " " * 20
def leafmods(d, names):
    out = [f"{ind}#[path = \".\"]", f"{ind}pub mod {d[1]} {{"]
    for n, folder in names:
        out += [f'{ind}    #[path = "{ANYR}/🚪️io/{d[0]}/{folder}/🦀️.rs"]', f"{ind}    pub mod {n};"]
    return out + [f"{ind}}}"]
block = "\n".join(leafmods(("📝️text", "text"), [("snapshot", "📸️snapshot"), ("mutations", "🧬️mutations"), ("diff", "🔺️diff"), ("inferences", "💡️inferences")])
                  + leafmods(("💾️binary", "binary"), [("snapshot", "📸️snapshot"), ("mutations", "🧬️mutations"), ("diff", "🔺️diff"), ("inferences", "💡️inferences")])
                  + leafmods(("🪶️sqlite", "sqlite"), [("snapshot", "📸️snapshot")])) + "\n"
root = rd(A + "/🦀️.rs")
anchor = f'                    #[path = "{ANYR}/🚪️io/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
assert anchor in root
root = root.replace(anchor, anchor + block, 1)
root = root.replace("// #region 🧪️Tests\n", f'// #region 🧪️Tests\n/// 🪶️ The full-stack SQLite snapshot round trip through the declared artifact (editor and viewer included).\n#[cfg(test)]\n#[path = "{ANYR}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]\nmod sqlite_snapshot;\n', 1)
wr(A + "/🦀️.rs", root)

sch = ANY + "/🧬️schema/🦀️.rs"; s = rd(sch)
a = s.index("//#region 🔖️Descriptor"); b = s.index("//#endregion 🔖️Descriptor") + len("//#endregion 🔖️Descriptor\n")
desc = s[a:b]; wr(sch, s[:a] + s[b:])
inf = ANY + "/🧬️schema/💡️inferences/🦀️.rs"; s = rd(inf)
a = s.index("//#region 🔖️Descriptor"); b = s.index("//#endregion 🔖️Descriptor") + len("//#endregion 🔖️Descriptor\n")
idesc = s[a:b]; wr(inf, s[:a] + s[b:])
desc = re.sub(r'include_str!\("([^"]+)"\)', lambda m: f'include_str!("🧬️schema/{m.group(1)}")', desc)
idesc = re.sub(r'include_str!\("([^"]+)"\)', lambda m: f'include_str!("🧬️schema/💡️inferences/{m.group(1)}")', idesc)
idesc = idesc.replace("//#region 🔖️Descriptor", "//#region 🔖️InferenceDescriptor").replace("//#endregion 🔖️Descriptor", "//#endregion 🔖️InferenceDescriptor")
sub = ANY + "/🦀️.rs"; s = rd(sub)
s = s.replace("schema::inferences::fem3d_artifact_inference_descriptor()", "fem3d_artifact_inference_descriptor()").replace("schema::fem3d_artifact_schema_descriptor()", "fem3d_artifact_schema_descriptor()")
wr(sub, s.rstrip() + "\n\n" + desc + "\n" + idesc)
print("ok")
