import os, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg"
D = A + "/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
IO = "crate::standards::v_jfif_1_01::subsets::document::io"
def snap(path, tag):
    dst = f"{ORIG}/jpg-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
def rw(path, fn, tag):
    snap(path, tag)
    t = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(fn(t))
# 1 widen codec internals that the decoded-components child reaches through the io root
for child in ["🔎️header-and-decode-entry", "🔡️huffman-and-bit-io", "✍️encoder", "🧮️transforms-and-tables", "🪜️step-decoder"]:
    rw(f"{D}/🚪️io/{child}/🦀️.rs", lambda t: t.replace("pub(super)", "pub(crate)"), f"codec-{child}.rs")
snapshot_file = D + "/🚪️io/💾️binary/📸️snapshot/🦀️.rs"
def snap_fn(t):
    for old in ['#[path = "🧮️decoded-components/🦀️.rs"]\npub mod decoded_components;\n\n', '#[path = "🎛️encode-options/🦀️.rs"]\npub mod encode_options;\n\n']:
        assert t.count(old) == 1
        t = t.replace(old, "")
    return t
rw(snapshot_file, snap_fn, "binary-snapshot.rs")
rw(D + "/🚪️io/💾️binary/📸️snapshot/🧮️decoded-components/🦀️.rs", lambda t: t.replace("use super::super::super::{JpgByteSource,JpgStepDecoder,parse_jpg_header};", f"use {IO}::{{JpgByteSource,JpgStepDecoder,parse_jpg_header}};"), "decoded.rs")
rw(D + "/🚪️io/💾️binary/📸️snapshot/🎛️encode-options/🦀️.rs", lambda t: t.replace("super::observations::JpgFrameHeader", f"{IO}::binary::snapshot::observations::JpgFrameHeader"), "encode-options.rs")
# 2 io root: drop the four mounts
def io_fn(t):
    old = '#[path = "🎹️facets/🦀️.rs"]\npub mod facets;\n\n#[path = "💾️binary/🦀️.rs"]\npub mod binary;\n\n#[path = "📝️text/🦀️.rs"]\npub mod text;\n\n#[path = "🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n\n'
    assert t.count(old) == 1
    return t.replace(old, "")
rw(D + "/🚪️io/🦀️.rs", io_fn, "io-root-j5.rs")
# 3 root nest
P = "🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/"
def wrap(indent, name, rel, extra=""):
    i = " " * indent
    return f'{i}#[path = "."]\n{i}pub mod {name} {{\n{i}    #[path = "{P}{rel}/🦀️.rs"]\n{i}    mod component;\n{i}    pub use component::*;\n{extra}{i}}}\n'
def rep(rep_indent, name, rel, extra_fn=None):
    return wrap(rep_indent, name, rel)
def snapshot_wrapper(indent, rel):
    i = " " * indent
    extra = (f'{i}    #[path = "{P}{rel}/🧮️decoded-components/🦀️.rs"]\n{i}    pub mod decoded_components;\n'
             f'{i}    #[path = "{P}{rel}/🎛️encode-options/🦀️.rs"]\n{i}    pub mod encode_options;\n')
    return wrap(indent, "snapshot", rel, extra)
def group(name, rel, kids, indent=20):
    i = " " * indent
    body = ""
    for k, m in kids:
        if m == "snapshot" and name == "binary":
            body += snapshot_wrapper(indent + 4, f"{rel}/{k}")
        else:
            body += wrap(indent + 4, m, f"{rel}/{k}")
    return f'{i}#[path = "."]\n{i}pub mod {name} {{\n{body}{i}}}\n'
kids4 = [("📸️snapshot", "snapshot"), ("🧬️mutations", "mutations"), ("🔺️diff", "diff"), ("💡️inferences", "inferences")]
block = (wrap(20, "facets", "🎹️facets") + group("binary", "💾️binary", kids4) + group("text", "📝️text", kids4) + group("sqlite", "🪶️sqlite", [("📸️snapshot", "snapshot")]))
def root_fn(t):
    anchor = '                    #[path = "🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
    assert t.count(anchor) == 1
    return t.replace(anchor, anchor + block)
rw(A + "/🦀️.rs", root_fn, "root-j5.rs")
for rel in ["💾️binary", "📝️text", "🪶️sqlite"]:
    f = f"{D}/🚪️io/{rel}/🦀️.rs"
    shutil.copy(f, f"{ORIG}/jpg-io-{rel}-root.rs"); os.remove(f)
print("ok")
