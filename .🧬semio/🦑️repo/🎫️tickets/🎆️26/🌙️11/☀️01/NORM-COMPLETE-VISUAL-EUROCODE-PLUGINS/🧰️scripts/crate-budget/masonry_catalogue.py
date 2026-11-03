"""📇️ Moves the masonry catalogue dir into its own crate (whole files), moves the shared helpers into the formulas crate, rewires the facade, registers the crate."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

art = NORM + "/🗿️artifacts/🪨️en1996"
S = art + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
M = S + "/⚖️masonry"
C = S + "/📇️masonry-catalogue"
F = S + "/📐️masonry-formulas"
old, new = "semio-s-artifact-norm-en1996-masonry", "semio-s-artifact-norm-en1996-masonry-catalogue"
if not os.path.exists(C):
    shutil.move(M + "/📇️catalogue", C)
root = rd(M + "/🦀️.rs")
imports = re.findall(r"^use (semio_s_[\w:]+)::\{([^}]*)\};$", root, re.M)
text = ""
for dp, dn, fn in os.walk(C):
    for f in fn:
        if f.endswith(".rs"):
            text += rd(os.path.join(dp, f))
words = set(re.findall(r"\b\w+\b", text))
lines = []
for path, names in imports:
    if "model" in path or "compliance" in path:
        keep = [n.strip() for n in names.split(",") if n.strip() in words]
        lines.append("use %s::{%s};" % (path, ", ".join(keep)))
head = "\n".join(lines)
cs = rd(C + "/🦀️.rs")
if "use super::*;\n" in cs:
    cs = cs.replace("use super::*;\n", head + "\nuse semio_s_plugin_norm_compliance::incremental_evaluation::Scope;\nuse semio_s_artifact_norm_en1996_masonry_formulas::*;\nuse semio_s_artifact_norm_en1996_masonry_tables::*;\n", 1)
    cs = cs.replace("pub(super) use document_cases::", "pub use document_cases::", 1)
    cs += '\n/// 📂️ Directory of the catalogue sources, scanned by the superseded-form tests.\npub const SOURCE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");\n'
    wr(C + "/🦀️.rs", cs)
dc = C + "/🧮️document-cases/🦀️.rs"
s = rd(dc)
s = s.replace("pub(super) fn document_case_results", "pub fn document_case_results").replace("pub(super) fn document_cases", "pub fn document_cases")
wr(dc, s)
helpers = re.search(r"fn loc\(.*?(?=pub use semio_s_artifact_norm_en1996_masonry_tables)", root, re.S)
if helpers:
    h = helpers.group(0)
    pubh = re.sub(r"(?m)^fn ", "pub fn ", h)
    fr = rd(F + "/🦀️.rs")
    fr = fr.replace("use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation};", "use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId, DesignSituation, LocalizedCopy, SubjectRef};")
    fr = fr.replace("#[path = \"📏️", pubh + "#[path = \"📏️", 1)
    fr += '\n/// 📂️ Directory of the formula sources, scanned by the superseded-form tests.\npub const SOURCE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");\n'
    wr(F + "/🦀️.rs", fr)
    root = root.replace(h, "")
    root = root.replace('#[path = "📇️catalogue/🦀️.rs"]\nmod catalogue;\npub use catalogue::*;\n', "pub use semio_s_artifact_norm_en1996_masonry_catalogue::*;\n")
    wr(M + "/🦀️.rs", root)
pkg = C + "/📦️packages/🦀️rust"
wr(pkg + "/Cargo.toml", '[package]\nworkspace = "../../../../../../../.."\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1996 masonry verification catalogue"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-artifact-norm-en1996-masonry-formulas = { workspace = true }\nsemio-s-artifact-norm-en1996-masonry-tables = { workspace = true }\nsemio-s-artifact-norm-en1996-model = { workspace = true }\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % new)
register_workspace(art, new, pkg)
add_dep(M + "/📦️packages/🦀️rust/Cargo.toml", new)
script_register(art, old, new)
lock_register(old, new, new_deps=toml_deps(rd(pkg + "/Cargo.toml")))
print("ok")
