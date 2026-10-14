#!/usr/bin/env python3
"""✂️ Cuts the artifact's io, import, export, simulation session and examples into crates (lane P1, slice S2d).

- `-io` (`🚪️io`): composition, io registry, model builder and analyzer.
- `-import` / `-export` (`🚪️io/📥️import`, `🚪️io/📤️export`): the epJSON, JSON, TXT and ZIP deserializers / serializers,
  mounted from new wiring roots as `epjson`, `json`, `txt`, `zip`.
- `-session` (`🧵️simulation-session`): the simulation tool-run definition and job.
- `-examples` (`📚️examples`): the bundled documents from a new wiring root; the per-example asset laws become the
  `example_assets` `[[test]]` (`📚️examples/🧪️tests/🔬️assets`) and read their committed DSL assets at run time.
The artifact root drops these subtrees and the io re-export facade; every reference names the owning crate.
"""
import importlib.util, json, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = f"{L.ART}/🦀️.rs"; IO = f"{L.ANY}/🚪️io"; EX = f"{L.ANY}/📚️examples"; SES = f"{L.ANY}/🧵️simulation-session"
root = L.rd(ROOT)
if "as session;" in root: sys.exit("already applied")
HEADER = "#![allow(async_fn_in_trait)]\n#![allow(clippy::too_many_arguments)]\n\n"
FORMATS = [("epjson", "⚡️epjson/🔖️25.2"), ("json", "🔣️json/🔖️rfc8259"), ("txt", "🔤️txt/🔖️utf-8"), ("zip", "🎒️zip/🔖️2.0")]

# 1. root: remove the io subtree, the examples tree, the session mount and the io facade
io_start = '                #[path = "."]\n                pub mod io {\n'
i = root.index(io_start); j = root.index("\n                }\n", i) + len("\n                }\n")
root = root[:i] + root[j:]
root, _ = L.take(root, '#[path = "."]\npub mod examples {\n', "\n}\n")
root = re.sub(r'(?:/// [^\n]*\n)*#\[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs"\]\npub mod energy_simulation_session;\n', "", root)
root = re.sub(r"(?m)^pub use crate::standards::v1::subsets::any::io::\{[^}]*\};\n", "", root)
root = L.replace_once(root, "extern crate semio_s_artifact_energy_model_diff as diff;\n",
                      "extern crate semio_s_artifact_energy_model_diff as diff;\nextern crate semio_s_artifact_energy_model_examples as examples;\nextern crate semio_s_artifact_energy_model_export as export;\nextern crate semio_s_artifact_energy_model_import as import;\nextern crate semio_s_artifact_energy_model_io as io;\nextern crate semio_s_artifact_energy_model_session as session;\n")
L.wr(ROOT, root)

# 2. crate roots
io_root = L.rd(f"{IO}/🦀️.rs"); d = io_root.index("\n\n") + 2
L.wr(f"{IO}/🦀️.rs", io_root[:d] + HEADER + "@@ALIASES@@\n\n" + io_root[d:])
for direction, folder, sub, what in (("import", "📥️import", "🧩️deserializers", "deserializers"), ("export", "📤️export", "🧵️serializers", "serializers")):
    mounts = "".join(f'#[path = "{sub}/🗿️artifacts/{p}/✳️any/🦀️.rs"]\npub mod {m};\n' for m, p in FORMATS)
    L.wr(f"{IO}/{folder}/🦀️.rs", f"//! 🚪️ Energy model {direction}: the epJSON, JSON, TXT and ZIP {what}.\n\n" + HEADER + "@@ALIASES@@\n\n" + mounts)
ses = L.rd(f"{SES}/🦀️.rs"); d = ses.index("\n\n") + 2
L.wr(f"{SES}/🦀️.rs", ses[:d] + HEADER + "@@ALIASES@@\n\n" + ses[d:])
examples = sorted(e for e in os.listdir(EX) if os.path.isdir(f"{EX}/{e}") and e != "🧪️tests")
def module(e): return re.sub(r"[^a-z0-9]+", "_", re.sub(r"^[^a-z0-9]+", "", e.lower())).strip("_")
ex_mounts = "".join(f'#[path = "{e}/🦀️.rs"]\npub mod {module(e)};\n' for e in examples)
L.wr(f"{EX}/🦀️.rs", "//! 📚️ Energy model examples: the BESTEST cases and the demo as bundled documents.\n\n" + HEADER + "@@ALIASES@@\n\n" + ex_mounts
     + '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️bestest-catalog/🦀️.rs"]\nmod bestest_catalog_tests;\n')
asset_mods = []
for e in examples:
    tp = f"{EX}/{e}/🧪️tests/🧩️example/🦀️.rs"
    if not os.path.exists(tp): continue
    t = L.rd(tp)
    t = re.sub(r'include_str!\("\.\./\.\./\.\./\.\./🖼️assets/([^"]+)"\)', r'&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🖼️assets/\1")).expect("committed example asset")', t)
    L.wr(tp, t)
    asset_mods.append(f'#[path = "."]\nmod {module(e)} {{\n    pub use crate::examples::{module(e)}::*;\n    #[path = "../../{e}/🧪️tests/🧩️example/🦀️.rs"]\n    mod example;\n}}\n')
L.wr(f"{EX}/🧪️tests/🔬️assets/🦀️.rs", "//! 🧪️ Each bundled example's committed DSL asset decodes to exactly the model the example claims.\n\n@@ALIASES@@\n\n" + "\n".join(asset_mods))
cat_p = f"{EX}/🧪️tests/🔬️bestest-catalog/🦀️.rs"; cat = L.rd(cat_p)
cat = cat.replace('std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")', 'std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")')
L.wr(cat_p, cat)

# 3. references
D = r"crate\s*::\s*standards\s*::\s*v1\s*::\s*subsets\s*::\s*any\s*::\s*"
RULES = [
    (D + r"io\s*::\s*import\s*::\s*deserializers\s*::\s*artifacts\s*::\s*(\w+)\s*::\s*\w+\s*::\s*any\s*::\s*", r"crate::import::\1::"),
    (D + r"io\s*::\s*export\s*::\s*serializers\s*::\s*artifacts\s*::\s*(\w+)\s*::\s*\w+\s*::\s*any\s*::\s*", r"crate::export::\1::"),
    (D + r"io\s*::\s*", "crate::io::"),
    (r"\bcrate\s*::\s*energy_simulation_session\s*::\s*", "crate::session::"),
]
OWN = {"io": f"{IO}/🦀️.rs", "import": f"{IO}/📥️import/🦀️.rs", "export": f"{IO}/📤️export/🦀️.rs", "session": f"{SES}/🦀️.rs", "examples": f"{EX}/🦀️.rs"}
owned = {}
for alias, r in OWN.items():
    for f in L.closure_files(r): owned.setdefault(f, alias)
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(r, f); t = L.rd(p); alias = owned.get(p)
        t2 = L.rewrite_code(t, RULES)
        if alias: t2 = L.rewrite_code(t2, [(r"\bcrate\s*::\s*" + alias + r"\s*::\s*", "crate::")])
        if t2 != t: L.wr(p, t2)
DESC = {"io": "Energy model io: composition, io registry, model builder and analyzer",
        "import": "Energy model import: epJSON, JSON, TXT and ZIP deserializers",
        "export": "Energy model export: epJSON, JSON, TXT and ZIP serializers",
        "session": "Energy model simulation session: the tool-run definition and the run job",
        "examples": "Energy model examples: the BESTEST cases and the demo as bundled documents"}
spec_rows = [{"owner": os.path.dirname(r), "package": L.ALIASES[a], "alias": a, "description": DESC[a]} for a, r in OWN.items()]
spec_rows[-1]["tests"] = [["example_assets", f"{EX}/🧪️tests/🔬️assets/🦀️.rs"]]
json.dump(spec_rows, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️io-session-examples-crates.json"), "w"), ensure_ascii=False, indent=1)
print("ok", len(examples), "examples")
