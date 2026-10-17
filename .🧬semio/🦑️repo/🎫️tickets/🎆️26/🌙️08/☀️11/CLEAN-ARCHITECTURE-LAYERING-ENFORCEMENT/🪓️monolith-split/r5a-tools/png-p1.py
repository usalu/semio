import os, shutil
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png"
S = A + "/🏅️standards/🔖️1.2/🪆️subsets/✳️any"
IO = S + "/🚪️io"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
P = "crate::standards::v1_2::subsets::any::io"
def snap(path, tag):
    dst = f"{ORIG}/png-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
def rw(path, fn, tag):
    snap(path, tag)
    t = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(fn(t))
io_path = IO + "/🦀️.rs"
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
i0 = find("//#region 🧩️ArtifactItems5"); i1 = find("//#endregion 🧩️ArtifactItems5")
blocks = [(c0, c1), (r0, r1), (k0, k1), (a0, a1), (f0, f1), (i0, i1)]
parts = [lines[a:b+1] for a, b in blocks]
rest = [l for i, l in enumerate(lines) if not any(a <= i <= b for a, b in blocks)]
fac = "\n\n".join("\n".join(p) for p in parts)
for old, new in [(f"use {P}::PngAnalyzer;", f"use {P}::facets::PngAnalyzer;"), (f"use {P}::PngComposer as PngRawAnyComposer;", f"use {P}::facets::PngComposer as PngRawAnyComposer;")]:
    assert fac.count(old) == 1, old
    fac = fac.replace(old, new)
assert "super::" not in fac
fac = fac.replace('include_bytes!("💾️binary/📸️snapshot/📡️.protocol.semio")', 'include_bytes!("../💾️binary/📸️snapshot/📡️.protocol.semio")')
os.makedirs(IO + "/🎹️facets", exist_ok=True)
open(IO + "/🎹️facets/🦀️.rs", "w", encoding="utf-8").write("//! 🎹️ Artifact facets of stdio.png (1.2/✳️any): builder, analyzer and composer, the composer registry and the native codec factory over the codec of the parent `io` module and the schema's snapshot, diff and mutation types.\n" + fac + "\n")
out = "\n".join(rest)
for old in ['#[path = "💾️binary/🦀️.rs"]\npub mod binary;\n\n', '#[path = "📝️text/🦀️.rs"]\npub mod text;\n\n']:
    assert out.count(old) == 1, old
    out = out.replace(old, "")
open(io_path, "w", encoding="utf-8").write(out)
rw(S + "/✏️editor/🦀️.rs", lambda t: t.replace(f"{P}::io_registry::entries()", f"{P}::facets::io_registry::entries()"), "editor.rs")
rw(S + "/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs", lambda t: t.replace(f"{P}::io_registry as v1_2", f"{P}::facets::io_registry as v1_2"), "inferences-tests-mirror.rs")
rw(A + "/🦀️.rs", lambda t: t.replace(f"{P}::{{PngBuilderConstruction,", f"{P}::facets::{{PngBuilderConstruction,"), "root.rs")
M = S + "/🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs"
def mroot(t):
    t = t.replace(f"{P}::artifact_items_5::{{native_codec}}", f"{P}::facets::artifact_items_5::{{native_codec}}")
    anchor = '                    #[path = "../../../../🚪️io/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
    assert t.count(anchor) == 1
    def wrap(name, rel):
        return f'                    #[path = "."]\n                    pub mod {name} {{\n                        #[path = "../../../../🚪️io/{rel}/🦀️.rs"]\n                        mod component;\n                        pub use component::*;\n                    }}\n'
    return t.replace(anchor, anchor + wrap("facets", "🎹️facets") + wrap("binary", "💾️binary") + wrap("text", "📝️text"))
rw(M, mroot, "mutations-root.rs")
print("ok")
