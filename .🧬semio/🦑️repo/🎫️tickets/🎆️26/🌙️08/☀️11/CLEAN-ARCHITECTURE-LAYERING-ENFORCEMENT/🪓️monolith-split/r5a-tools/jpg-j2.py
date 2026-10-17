import os, re, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg"
D = A + "/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
def snap(path, tag):
    dst = f"{ORIG}/jpg-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
def rw(path, fn, tag):
    snap(path, tag)
    t = open(path, encoding="utf-8").read()
    n = fn(t)
    open(path, "w", encoding="utf-8").write(n)
# 1. demo_jpg_snapshot: schema root -> snapshot
root_p = D + "/🧬️schema/🦀️.rs"
snap_p = D + "/🧬️schema/📸️snapshot/🦀️.rs"
state = {}
def cut_demo(t):
    s = t.index("/// 🧪️ P2-FG2: the demo `JpgSnapshot`")
    e = t.index("//#endregion 🔖️DocumentHelpers")
    block = t[s:e]
    assert block.count("pub(crate) fn demo_jpg_snapshot()") == 1
    state["block"] = block
    t = t[:s] + t[e:]
    assert t.count("#[cfg(test)] use crate::JpgSnapshot;\n") == 1
    return t.replace("#[cfg(test)] use crate::JpgSnapshot;\n", "")
rw(root_p, cut_demo, "schema-root.rs")
block = state["block"].replace("#[cfg(test)]\n", "", 1).replace("pub(crate) fn demo_jpg_snapshot()", "pub fn demo_jpg_snapshot()")
assert "#[cfg(test)]" not in block
def add_demo(t):
    return t.rstrip("\n") + "\n\n" + block.rstrip("\n") + "\n"
rw(snap_p, add_demo, "snapshot.rs")
# 2. io tests split
io_tests = D + "/🚪️io/🧪️tests/🔬️unit/🦀️.rs"
def split_tests(t):
    lines = t.split("\n")
    start = next(i for i, l in enumerate(lines) if l.startswith("/// 🧬️ Publishes the actual compiled JPG factory"))
    nxt = next(i for i, l in enumerate(lines) if i > start and l == "#[test]" and lines[i + 1].startswith("fn controlled_components_preserve_gray"))
    reg0 = next(i for i, l in enumerate(lines) if l.startswith("//#region 🔖️ConformanceLaws"))
    reg1 = next(i for i, l in enumerate(lines) if l.startswith("//#endregion 🔖️ConformanceLaws"))
    moved = lines[start:nxt] + [""] + lines[reg0:reg1 + 1]
    keep = lines[:start] + lines[nxt:reg0] + lines[reg1 + 1:]
    header = ["//! 🧪️ Conformance and publication laws of stdio.jpg (jfif-1.01/🧾️document): grammar and protocol parseability, recognition of real printed output, protocol walks over real bytes, fixture honesty and the native-codec publication receipt.",
              "use pack::snapshot::ArtifactSqliteSnapshot as _;",
              "use crate::JpgSnapshot;",
              "use crate::schema::snapshot::demo_jpg_snapshot;",
              "use crate::standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg, JpgEncodeOptions};",
              ""]
    body = "\n".join(moved)
    body = body.replace('include_str!("../', 'include_str!("../../').replace('include_bytes!("../', 'include_bytes!("../../')
    body = body.replace("    use super::*;\n    use crate::schema::{diff, snapshot};", "    use super::*;\n    use crate::schema::{diff, snapshot};")
    state["moved"] = "\n".join(header) + body + "\n"
    out = "\n".join(keep)
    out = out.replace("use crate::schema::demo_jpg_snapshot;", "use crate::schema::snapshot::demo_jpg_snapshot;", 1)
    return out
rw(io_tests, split_tests, "io-tests.rs")
os.makedirs(D + "/🚪️io/🎹️facets/🧪️tests/🔬️conformance", exist_ok=True)
open(D + "/🚪️io/🎹️facets/🧪️tests/🔬️conformance/🦀️.rs", "w", encoding="utf-8").write(state["moved"])
# mount tests in facets
fac = D + "/🚪️io/🎹️facets/🦀️.rs"
def mount(t):
    return t.rstrip("\n") + "\n\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️conformance/🦀️.rs\"]\nmod tests;\n"
rw(fac, mount, "facets.rs")
# 3. editor tests
ed = D + "/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
def ed_fix(t):
    assert t.count("crate::schema::demo_jpg_snapshot()") == 1
    return t.replace("crate::schema::demo_jpg_snapshot()", "crate::schema::snapshot::demo_jpg_snapshot()")
rw(ed, ed_fix, "editor-tests.rs")
print("ok")
