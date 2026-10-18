import sys, os, re, json
sys.path.insert(0, "/Users/ueli/.cache/hdr3/tools")
from h3reg import *
P = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
MODEL = P + "/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model"
mir_old = MODEL + "/📦️packages/🦀️rust/🪞️mirror/🦀️.rs"
mt = rd(mir_old)
write = "--write" in sys.argv
NODE = MODEL + "/🧩️profiles"
mir_new = NODE + "/🪞️mirror/🦀️.rs"
# extract blocks
ed = re.search(r'#\[path = "\."\]\npub mod editor \{\n(?:.*\n)*\}\n\Z', mt)
assert ed, "editor block"
e_blk = re.search(r'            #\[path = "\."\]\n            pub mod e \{\n(?:.*\n)*?            \}\n', mt)
h_blk = re.search(r'            #\[path = "\."\]\n            pub mod h \{\n(?:.*\n)*?            \}\n', mt)
assert e_blk and h_blk
def rebase(block):
    def sub(m):
        absf = os.path.normpath(os.path.join(os.path.dirname(mir_old), m.group(1)))
        return '#[path = "%s"]' % os.path.relpath(absf, os.path.dirname(mir_new))
    return re.sub(r'#\[path = "((?!\.")[^"]+)"\]', sub, block)
new_mirror = ('//! 🪞️ The mirrored editor profile schemas and the extra-profile io owners of the typed PDF model.\n\n'
  '#[path = "."]\npub mod standards {\n    #[path = "."]\n    pub mod v1_7 {\n        #[path = "."]\n        pub mod subsets {\n'
  + rebase(e_blk.group(0)) + rebase(h_blk.group(0)) + '        }\n    }\n}\n' + rebase(ed.group(0)))
root_text = '//! 🧩️ profiles of `semio-s-artifact-stdio-pdf-model` — the editor profile schemas and the extra-profile io owners, carved out of the model crate.\n\n#[path = "🪞️mirror/🦀️.rs"]\nmod mirror;\npub use mirror::*;\n'
print(new_mirror[:1800])
if not write: sys.exit(0)
pkgdir = NODE + "/" + PKG
os.makedirs(os.path.dirname(mir_new), exist_ok=True); os.makedirs(pkgdir, exist_ok=True)
open(NODE + "/🦀️.rs", "w").write(root_text); open(mir_new, "w").write(new_mirror)
oman = MODEL + "/" + PKG + "/Cargo.toml"
wsrel = os.path.relpath(P, pkgdir)
open(pkgdir + "/Cargo.toml", "w").write('[package]\nworkspace = "%s"\nname = "semio-s-artifact-stdio-pdf-model-profiles"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Component crate of semio-s-artifact-stdio-pdf: model profiles"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % wsrel)
# old mirror
t2 = mt.replace(ed.group(0), "").replace(e_blk.group(0), "").replace(h_blk.group(0), "")
wr(mir_old, t2)
add_member(P, pkgdir, MODEL + "/" + PKG)
add_owner("semio-s-artifact-stdio-pdf-model-profiles", pkgdir)
# root package dependency
rm = P + "/Cargo.toml"
t = rd(rm)
line = [l for l in t.split("\n") if l.startswith("semio-s-artifact-stdio-pdf-model = {") and "features" not in l][0]
new_line = line.replace("semio-s-artifact-stdio-pdf-model", "semio-s-artifact-stdio-pdf-model-profiles", 1).replace("📐️model/📦️packages/🦀️rust", "📐️model/🧩️profiles/📦️packages/🦀️rust")
t = t.replace(line + "\n", line + "\n" + new_line + "\n", 1)
wr(rm, t)
# root crate source
rs = P + "/🦀️.rs"
s = rd(rs)
for a in ("editor::pdf14a", "editor::pdf14::", "editor::pdf14x", "editor::pdf17a", "subsets::e::io", "subsets::h::io"):
    pass
s2 = re.sub(r"semio_s_artifact_stdio_pdf_model::(editor::pdf1\w+::(?:presence|config);)", r"semio_s_artifact_stdio_pdf_model_profiles::\1", s)
s2 = re.sub(r"semio_s_artifact_stdio_pdf_model::(standards::v1_7::subsets::[eh]::io::)", r"semio_s_artifact_stdio_pdf_model_profiles::\1", s2)
print("root edits:", sum(1 for a, b in zip(s.split("\n"), s2.split("\n")) if a != b))
wr(rs, s2)
