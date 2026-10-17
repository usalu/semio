import os, re, shutil, sys
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg"
D = A + "/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"
B = A + "/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
def snap(path, tag):
    dst = f"{ORIG}/jpg-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
io_path = D + "/🚪️io/🦀️.rs"
snap(io_path, "io-root.rs")
text = open(io_path, encoding="utf-8").read()
lines = text.split("\n")
def find(prefix, start=0):
    for i in range(start, len(lines)):
        if lines[i].startswith(prefix): return i
    raise SystemExit("missing " + prefix)
# regions to move
c0 = find("//#region 🎹️DerivedComposition"); c1 = find("//#endregion 🎹️DerivedComposition")
r0 = find("//#region 🚪️DerivedIoRegistry"); r1 = find("//#endregion 🚪️DerivedIoRegistry")
k0 = find("pub mod derived_construction {"); k1 = find("pub use derived_construction::*;")
a0 = find("pub mod derived_analysis {"); a1 = find("pub use derived_analysis::*;")
f0 = find("semio_framework_plugin::derive_artifact_facets!("); f1 = find(");", f0)
composition = lines[c0:c1+1]; registry = lines[r0:r1+1]; construction = lines[k0:k1+1]; analysis = lines[a0:a1+1]; facets = lines[f0:f1+1]
rest = []
for i, l in enumerate(lines):
    if c0 <= i <= c1 or r0 <= i <= r1 or k0 <= i <= k1 or a0 <= i <= a1 or f0 <= i <= f1: continue
    rest.append(l)
fa = "\n".join(analysis)
assert fa.count("super::inspect_jpg_native_header") == 1 and fa.count("super::JpgNativeObservations") == 1
fa = fa.replace("super::inspect_jpg_native_header", "crate::standards::v_jfif_1_01::subsets::document::io::inspect_jpg_native_header")
fa = fa.replace("super::JpgNativeObservations", "crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations::JpgNativeObservations")
fc = "\n".join(composition)
anchor = "use crate::standards::v_jfif_1_01::subsets::document::io::JpgAnalyzer;"
assert fc.count(anchor) == 1
fc = fc.replace(anchor, "use crate::standards::v_jfif_1_01::subsets::document::io::facets::JpgAnalyzer;")
new = "//! 🎹️ Artifact facets of stdio.jpg (jfif-1.01/🧾️document): the builder, analyzer and composer over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.\n" + fc + "\n\n" + "\n".join(construction) + "\n\n" + fa + "\n\n" + "\n".join(facets) + "\n"
os.makedirs(D + "/🚪️io/🎹️facets", exist_ok=True)
open(D + "/🚪️io/🎹️facets/🦀️.rs", "w", encoding="utf-8").write(new)
out = "\n".join(rest)
mount_anchor = "#[path = \"💾️binary/🦀️.rs\"]\npub mod binary;"
assert out.count(mount_anchor) == 1
out = out.replace(mount_anchor, "#[path = \"🎹️facets/🦀️.rs\"]\npub mod facets;\n\n" + mount_anchor)
open(io_path, "w", encoding="utf-8").write(out)
# registry -> baseline io
b_path = B + "/🚪️io/🦀️.rs"
snap(b_path, "baseline-io.rs")
bt = open(b_path, encoding="utf-8").read()
reg = "\n".join(registry)
a1_ = "use crate::standards::v_jfif_1_01::subsets::baseline::io::JpgBaselineComposer;"
a2_ = "use crate::standards::v_jfif_1_01::subsets::document::io::JpgComposer as JpgRawAnyComposer;"
assert reg.count(a1_) == 1 and reg.count(a2_) == 1
reg = reg.replace(a1_, "use super::JpgBaselineComposer;").replace(a2_, "use crate::standards::v_jfif_1_01::subsets::document::io::facets::JpgComposer as JpgRawAnyComposer;")
bt = bt.rstrip("\n") + "\n\n" + reg + "\n"
for old, newv in [("document::io::JpgComposer as JpgAnyComposer", "document::io::facets::JpgComposer as JpgAnyComposer"), ("document::io::JpgBuilder as JpgAnyBuilder", "document::io::facets::JpgBuilder as JpgAnyBuilder"), ("document::io::JpgAnalyzer as JpgAnyAnalyzer", "document::io::facets::JpgAnalyzer as JpgAnyAnalyzer"), ("document::io::JpgParts;", "document::io::facets::JpgParts;"), ("`crate::standards::v_jfif_1_01::subsets::document::io::io_registry::entries()`", "`crate::standards::v_jfif_1_01::subsets::baseline::io::io_registry::entries()`")]:
    assert bt.count(old) == 1, old
    bt = bt.replace(old, newv)
open(b_path, "w", encoding="utf-8").write(bt)
# root
r_path = A + "/🦀️.rs"
snap(r_path, "root.rs")
rt = open(r_path, encoding="utf-8").read()
for old, newv in [(".composers(standards::v_jfif_1_01::subsets::document::io::io_registry::entries())", ".composers(standards::v_jfif_1_01::subsets::baseline::io::io_registry::entries())"), ("use crate::standards::v_jfif_1_01::subsets::document::io::io_registry as v_jfif_1_01;", "use crate::standards::v_jfif_1_01::subsets::baseline::io::io_registry as v_jfif_1_01;"), ("pub use crate::standards::v_jfif_1_01::subsets::document::io::{JpgBuilderConstruction, JpgParts, JpgAnalyzerAnalysis, JpgBuilderFacets, JpgBuilder, JpgAnalyzer, JpgComposer};", "pub use crate::standards::v_jfif_1_01::subsets::document::io::facets::{JpgBuilderConstruction, JpgParts, JpgAnalyzerAnalysis, JpgBuilderFacets, JpgBuilder, JpgAnalyzer, JpgComposer};")]:
    assert rt.count(old) == 1, old
    rt = rt.replace(old, newv)
open(r_path, "w", encoding="utf-8").write(rt)
print("ok", len(composition), len(registry), len(construction), len(analysis), len(facets))
