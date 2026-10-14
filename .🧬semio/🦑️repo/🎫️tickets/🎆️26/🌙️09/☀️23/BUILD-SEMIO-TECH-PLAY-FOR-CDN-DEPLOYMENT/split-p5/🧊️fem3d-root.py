#!/usr/bin/env python3
"""🧊️ Decomposes the fem3d artifact root before the crate cut (P5, fem3d slice): the document entities move to
`🧬️schema/🏗️structure`, the mutation guards to `🧬️schema/🛡️guards`, the artifact kinds, dialect and pilot languages to
`🚪️io`; every root re-export is deleted and its consumers name the owner (via `🚚️relocate-root.py` maps).

usage: 🧊️fem3d-root.py
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
A = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
ANY = A + "/🏅️standards/🔖️1/🪆️subsets/🌐️any"
S = "standards::v1::subsets::any"
M2 = "semio_s_artifact_fem_2d_model::" + S + "::schema::structure"
rd = lambda p: open(p, encoding="utf-8").read()
def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf-8").write(s)
def cut(s, a, b):
    i = s.index(a); j = s.index(b, i) + len(b)
    return s[:i] + s[j:], s[i:j]

root = rd(A + "/🦀️.rs")
root, doc = cut(root, "// #region 🔖️Document", "// #endregion 🔖️Document\n")
root = root.replace('pub const FEM_3D_SCHEMA: &str = "fem.3d";\n', "", 1)
root = root.replace("use semio_framework_value_derive::{FromValue, ToValue};\nuse std::collections::BTreeMap;\n", "", 1)
doc = re.sub(r"(?:///[^\n]*\n)*pub use semio_s_artifact_fem_2d_model::[^\n]*FemDof;\n(?://[^\n]*\n)*", "", doc)
doc = re.sub(r"(?:///[^\n]*\n)*pub use semio_s_artifact_fem_2d_model::[^\n]*FemAnalysisSettings;\n", "", doc)
doc = re.sub(r"(?:///[^\n]*\n)*pub use semio_framework_os_kernel::Viewport3dOrbit;\n", "", doc)
vp = re.search(r'(?:///[^\n]*\n)*#\[path = "🪟️viewport/🦀️.rs"\]\npub mod viewport;\n', doc).group(0)
doc = doc.replace(vp, "")
doc = re.sub(r"(?:///[^\n]*\n)*pub use crate::standards::v1::subsets::any::schema::Fem3dArtifact;\n", "", doc)
doc = doc.replace("// #region 🔖️Document\n", "").replace("// #endregion 🔖️Document\n", "")
assert "pub use" not in doc, [l for l in doc.splitlines() if "pub use" in l]
structure = ("//! 🏗️ FEM 3D structural model entities: nodes, members, solids, materials, sections, supports, loads, load cases\n"
    "//! and combinations — the records every fem3d facet (snapshot, diff, mutations, io, editor) names.\n\n"
    "use " + M2 + "::FemDof;\nuse semio_framework_value_derive::{FromValue, ToValue};\nuse std::collections::BTreeMap;\n\n"
    'pub const FEM_3D_SCHEMA: &str = "fem.3d";\n\n' + doc.strip() + "\n")
wr(ANY + "/🧬️schema/🏗️structure/🦀️.rs", structure)
root = root.replace("// #region 🧪️Tests", vp + "\n// #region 🧪️Tests", 1)

root, kinds = cut(root, "// #region 🔖️ArtifactKind", "// #endregion 🔖️ArtifactKind\n")
dial = re.search(r"(?:///[^\n]*\n)*pub const FEM3D_DIALECT[^\n]*\n", root).group(0)
root = root.replace(dial, "", 1)
m = re.search(r"(?:///[^\n]*\n)*pub fn pilot_languages\(\)", root)
end = root.index("\n}\n", m.start()) + 3
langs = root[m.start():end]; root = root[:m.start()] + root[end:]
langs = re.sub(r"(?<![:\w])standards::v1::", "crate::standards::v1::", langs)
io = rd(ANY + "/🚪️io/🦀️.rs")
io = io.rstrip() + "\n\n//#region 🪪️ArtifactKind\nuse crate::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA;\n\n" + dial + "\n" + kinds.replace("// #region 🔖️ArtifactKind\n", "").replace("// #endregion 🔖️ArtifactKind\n", "") + "\n" + langs + "//#endregion 🪪️ArtifactKind\n"
wr(ANY + "/🚪️io/🦀️.rs", io)

for line in ["pub use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;\n", "pub use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;\n",
             "pub use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;\n"]:
    assert line in root, line; root = root.replace(line, "", 1)
root = re.sub(r"pub use crate::standards::v1::subsets::any::io::\{Fem3dBuilderConstruction[^\n]*\n", "", root)
anchor = '                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
assert anchor in root
root = root.replace(anchor, anchor + '                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🦀️.rs"]\n                    pub mod structure;\n                    #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🛡️guards/🦀️.rs"]\n                    pub mod guards;\n', 1)
wr(A + "/🦀️.rs", root)

mf = ANY + "/🧬️schema/🧬️mutations/🦀️.rs"; mu = rd(mf)
mu, guards = cut(mu, "//#region 🛡️Guards", "//#endregion 🛡️Guards\n")
gnames = re.findall(r"(?m)^pub fn (\w+)", guards)
guards = guards.replace("//#region 🛡️Guards\n", "").replace("//#endregion 🛡️Guards\n", "")
wr(ANY + "/🧬️schema/🛡️guards/🦀️.rs", "//! 🛡️ The payload validations, foreign-key resolutions and referential-integrity lookups every fem3d mutation kind\n//! shares — written once, below every kind that runs them.\n\n"
   "use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;\nuse crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;\n"
   "use crate::standards::v1::subsets::any::schema::structure::{element_id, load_id, FemCombination, FemElement, FemLoad, FemMaterial, FemNode, FemSection, FemSolid};\n"
   "use " + M2 + "::FemAnalysisSettings;\n\n" + guards.strip() + "\n")
wr(mf, mu)

names = re.findall(r"(?m)^pub (?:struct|enum|fn|const) (\w+)", structure)
cmap = {n: "crate::" + S + "::schema::structure" for n in names}
cmap.update({"FemDof": M2, "FemAnalysisSettings": M2, "Viewport3dOrbit": "semio_framework_os_kernel", "Fem3dArtifact": "crate::" + S + "::schema",
             "Fem3dDiff": "crate::" + S + "::schema::diff", "Fem3dMutation": "crate::" + S + "::schema::mutations", "Fem3dSnapshot": "crate::" + S + "::schema::snapshot"})
for n in ["Fem3dBuilderConstruction", "Fem3dParts", "Fem3dAnalyzerAnalysis", "Fem3dBuilderFacets", "Fem3dBuilder", "Fem3dAnalyzer", "Fem3dComposer", "FEM3D_DIALECT", "document_artifact_kind", "computation_artifact_kind", "pilot_languages"]:
    cmap[n] = "crate::" + S + "::io"
gmap = {n: "crate::" + S + "::schema::guards" for n in gnames}
json.dump(cmap, open(HERE + "/🚚️fem3d-root-map.json", "w"), ensure_ascii=False, indent=1)
json.dump(gmap, open(HERE + "/🚚️fem3d-guards-map.json", "w"), ensure_ascii=False, indent=1)
def rs(d):
    out = []
    for r, ds, fs in os.walk(d):
        ds[:] = [x for x in ds if x not in ("target", "node_modules", "🗑️generated")]
        out += [os.path.join(r, f) for f in fs if f.endswith(".rs")]
    return out
inner = rs(A)
outer = rs(REPO + "/🌎️hub/🧩️compositions/🏗️fem") + rs(REPO + "/✏️s/🔌️plugins/🏗️fem/🏭️bridge") + rs(REPO + "/✏️s/🔌️plugins/📕️norm")
tool = HERE + "/🚚️relocate-root.py"
subprocess.run([sys.executable, tool, HERE + "/🚚️fem3d-root-map.json", "crate"] + inner, check=True)
subprocess.run([sys.executable, tool, HERE + "/🚚️fem3d-guards-map.json", "crate::" + S + "::schema::mutations"] + inner, check=True)
subprocess.run([sys.executable, tool, HERE + "/🚚️fem3d-root-map.json", "semio_s_artifact_fem_3d"] + outer, check=True)
print("structure", len(names), "guards", len(gnames))
