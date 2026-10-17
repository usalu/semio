import os, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff"
S = A + "/🏅️standards/🔖️6.0/🪆️subsets"
D = S + "/🧾️document"; B = S + "/🧱️baseline"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
IO = "crate::standards::v6_0::subsets::document::io"
def snap(path, tag):
    dst = f"{ORIG}/tiff-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
def rw(path, fn, tag):
    snap(path, tag)
    t = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(fn(t))
io_path = D + "/🚪️io/🦀️.rs"
snap(io_path, "io-root.rs")
lines = open(io_path, encoding="utf-8").read().split("\n")
def find(prefix, start=0):
    for i in range(start, len(lines)):
        if lines[i].startswith(prefix): return i
    raise SystemExit("missing " + prefix)
c0 = find("//#region 🎹️DerivedComposition"); c1 = find("//#endregion 🎹️DerivedComposition")
r0 = find("//#region 🚪️DerivedIoRegistry"); r1 = find("//#endregion 🚪️DerivedIoRegistry")
k0 = find("pub mod derived_construction {"); k1 = find("pub use derived_construction::*;")
a0 = find("pub mod derived_analysis {"); a1 = find("pub use derived_analysis::*;")
f0 = find("semio_framework_plugin::derive_artifact_facets!("); f1 = find(");", f0)
composition = lines[c0:c1+1]; registry = lines[r0:r1+1]; construction = lines[k0:k1+1]; analysis = lines[a0:a1+1]; facets = lines[f0:f1+1]
rest = [l for i, l in enumerate(lines) if not (c0 <= i <= c1 or r0 <= i <= r1 or k0 <= i <= k1 or a0 <= i <= a1 or f0 <= i <= f1)]
fc = "\n".join(composition)
anchor = f"use {IO}::TiffAnalyzer;"
assert fc.count(anchor) == 1
fc = fc.replace(anchor, f"use {IO}::facets::TiffAnalyzer;")
fa = "\n".join(analysis)
assert "super::" not in fa and "super::" not in "\n".join(construction)
new = "//! 🎹️ Artifact facets of stdio.tiff (6.0/🧾️document): the builder, analyzer and composer over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.\n" + fc + "\n\n" + "\n".join(construction) + "\n\n" + fa + "\n\n" + "\n".join(facets) + "\n"
os.makedirs(D + "/🚪️io/🎹️facets", exist_ok=True)
open(D + "/🚪️io/🎹️facets/🦀️.rs", "w", encoding="utf-8").write(new)
out = "\n".join(rest)
for old in ['#[path = "💾️binary/🦀️.rs"]\npub mod binary;\n\n', '#[path = "📝️text/🦀️.rs"]\npub mod text;\n\n', '#[path = "🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n\n']:
    assert out.count(old) == 1, old
    out = out.replace(old, "")
open(io_path, "w", encoding="utf-8").write(out)
b_path = B + "/🚪️io/🦀️.rs"
snap(b_path, "baseline-io.rs")
bt = open(b_path, encoding="utf-8").read()
reg = "\n".join(registry)
a1_ = "use crate::standards::v6_0::subsets::baseline::io::TiffBaselineComposer;"
a2_ = "use crate::standards::v6_0::subsets::document::io::TiffComposer as TiffRawAnyComposer;"
assert reg.count(a1_) == 1 and reg.count(a2_) == 1
reg = reg.replace(a1_, "use super::TiffBaselineComposer;").replace(a2_, f"use {IO}::facets::TiffComposer as TiffRawAnyComposer;")
bt = bt.rstrip("\n") + "\n\n" + reg + "\n"
for old, newv in [("document::io::TiffComposer as TiffAnyComposer", "document::io::facets::TiffComposer as TiffAnyComposer"), ("document::io::TiffBuilder as TiffAnyBuilder", "document::io::facets::TiffBuilder as TiffAnyBuilder"), ("document::io::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts}", "document::io::facets::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts}")]:
    if old not in bt: print("WARN missing", old); continue
    assert bt.count(old) == 1, old
    bt = bt.replace(old, newv)
open(b_path, "w", encoding="utf-8").write(bt)
r_path = A + "/🦀️.rs"
snap(r_path, "root.rs")
rt = open(r_path, encoding="utf-8").read()
for old, newv in [(".composers(standards::v6_0::subsets::document::io::io_registry::entries())", ".composers(standards::v6_0::subsets::baseline::io::io_registry::entries())"), ("use crate::standards::v6_0::subsets::document::io::io_registry as v6_0;", "use crate::standards::v6_0::subsets::baseline::io::io_registry as v6_0;"), ("pub use crate::standards::v6_0::subsets::document::io::{TiffBuilderConstruction, TiffParts, TiffAnalyzerAnalysis, TiffBuilderFacets, TiffBuilder, TiffAnalyzer, TiffComposer};", "pub use crate::standards::v6_0::subsets::document::io::facets::{TiffBuilderConstruction, TiffParts, TiffAnalyzerAnalysis, TiffBuilderFacets, TiffBuilder, TiffAnalyzer, TiffComposer};")]:
    assert rt.count(old) == 1, old
    rt = rt.replace(old, newv)
P = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/"
def wrap(indent, name, rel, extra=""):
    i = " " * indent
    return f'{i}#[path = "."]\n{i}pub mod {name} {{\n{i}    #[path = "{P}{rel}/🦀️.rs"]\n{i}    mod component;\n{i}    pub use component::*;\n{extra}{i}}}\n'
def group(name, rel, kids, indent=20):
    i = " " * indent
    body = "".join(wrap(indent + 4, m, f"{rel}/{k}") for k, m in kids)
    return f'{i}#[path = "."]\n{i}pub mod {name} {{\n{body}{i}}}\n'
kids4 = [("📸️snapshot", "snapshot"), ("🧬️mutations", "mutations"), ("🔺️diff", "diff"), ("💡️inferences", "inferences")]
block = wrap(20, "facets", "🎹️facets") + group("binary", "💾️binary", kids4) + group("text", "📝️text", kids4) + group("sqlite", "🪶️sqlite", [("📸️snapshot", "snapshot")])
anchor = f'                    #[path = "{P}🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
assert rt.count(anchor) == 1
rt = rt.replace(anchor, anchor + block)
open(r_path, "w", encoding="utf-8").write(rt)
for rel in ["💾️binary", "📝️text", "🪶️sqlite"]:
    f = f"{D}/🚪️io/{rel}/🦀️.rs"
    shutil.copy(f, f"{ORIG}/tiff-io-{rel}-root.rs"); os.remove(f)
print("ok")
