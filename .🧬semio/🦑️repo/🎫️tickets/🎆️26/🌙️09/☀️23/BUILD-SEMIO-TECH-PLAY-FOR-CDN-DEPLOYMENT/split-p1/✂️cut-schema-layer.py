#!/usr/bin/env python3
"""✂️ Cuts the energy model artifact's schema layer into four crates (lane P1, slice S2b).

- `-snapshot` (`🧬️schema/📸️snapshot`): `EnergyModelSnapshot`, the composition it is built from (child types, value/table
  converters, structure overview, read lease, snapshot construction, genesis children, entity id minting, whole-document
  load, the schema ids and `MODEL_DIALECT`, all moved down from the artifact root), and the snapshot's text, binary and
  sqlite codecs (`🚪️io/<representation>/📸️snapshot`, mounted here because they implement the snapshot's own traits).
- `-diff` (`🧬️schema/🔺️diff`): `EnergyModelDiff` with its patches, its text/binary codecs and its five facet leaves.
- `-inferences` (`🧬️schema/💡️inferences`): the inference schema, `🗃️entries` and its text/binary codecs.
- `-schema` (`🧬️schema`): `EnergyModelArtifact` and the artifact schema descriptor.
Every other artifact file is rewritten from the monolith's module paths to the crate aliases.
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = f"{L.ART}/🦀️.rs"
SNAP, DIFF, INF, SCH = f"{L.ANY}/🧬️schema/📸️snapshot", f"{L.ANY}/🧬️schema/🔺️diff", f"{L.ANY}/🧬️schema/💡️inferences", f"{L.ANY}/🧬️schema"
IO = f"{L.ANY}/🚪️io"
root = L.rd(ROOT)
if "extern crate semio_s_artifact_energy_model_snapshot" in root: sys.exit("already applied")

# 1. composition block and facades out of the artifact root
root = L.replace_once(root, "pub use crate::schema::EnergyModelArtifact;\n\n", "")
root, composition = L.take(root, "/// 🔖️ Document schema / DSL envelope id.\n", "//#endregion ♻️WholeDocumentLoad\n")
uses_line = re.search(r"(?m)^use \{semio_framework_plugin::ArtifactKindSpec[^\n]*\n", root).group(0)
root, shims = L.take(root, "// ---- Shims: keep pre-migration module paths resolving for external callers ----\n", "pub use crate::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;\n")
mutations_shim = re.search(r"pub mod mutations \{.*?\n\}\n", shims, re.S).group(0)
root = root.replace("\n\n\n\n#[path = \".\"]\npub mod examples", "\n\n" + mutations_shim + "pub use crate::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;\n\n#[path = \".\"]\npub mod examples") if "\n\n\n\n#[path = \".\"]\npub mod examples" in root else root.replace("#[path = \".\"]\npub mod examples", mutations_shim + "pub use crate::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;\n\n#[path = \".\"]\npub mod examples", 1)
# mount tree: the schema node keeps only its mutations subtree
node_start = '                #[path = "."]\n                pub mod schema {\n                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n'
mut_start = '                    #[path = "."]\n                    pub mod mutations {\n'
assert root.count(node_start) == 1 and root.count(mut_start) == 1
i = root.index(node_start); j = root.index(mut_start)
root = root[:i] + '                #[path = "."]\n                pub mod schema {\n' + root[j:]
# 2. io wiring: the snapshot/diff/inferences codecs leave the representation wiring files
for rep in ("📝️text", "💾️binary"):
    p = f"{IO}/{rep}/🦀️.rs"; t = L.rd(p)
    for facet, name in (("📸️snapshot", "snapshot"), ("🔺️diff", "diff"), ("💡️inferences", "inferences")):
        t = L.replace_once(t, f'#[path = "{facet}/🦀️.rs"]\npub mod {name};\n', "")
    L.wr(p, t)
io = L.rd(f"{IO}/🦀️.rs")
io = re.sub(r'(?:\n[^\n]*\n)?#\[path = "🪶️sqlite/🦀️.rs"\]\npub mod sqlite;\n', "\n", io, count=1) if '#[path = "🪶️sqlite/🦀️.rs"]' in io else io
L.wr(f"{IO}/🦀️.rs", io)
os.remove(f"{IO}/🪶️sqlite/🦀️.rs")

HEADER = ["#![allow(async_fn_in_trait)]", "#![allow(clippy::too_many_arguments)]", ""]
MOVED = set(re.findall(r"(?m)^pub (?:const|fn|struct|type|enum) ([A-Za-z_][A-Za-z0-9_]*)", composition))
MOVED |= {"EnergyModelSnapshot"}

# 3. crate roots
snap = L.rd(f"{SNAP}/🦀️.rs")
snap = L.replace_once(snap, "use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};\n", "")
doc_end = snap.index("\n\n") + 2
snap_body = snap[doc_end:]
snap_doc = "//! 🧬️ EnergyModel snapshot — the persisted artifact-lane fields, the composed `structure`/`zones` children and the\n//! value/table bridge they are built from, snapshot construction and whole-document load, and the snapshot's own text,\n//! binary and sqlite codecs.\n\n"
mounts = ('\n//#region 🚪️Representations\n#[path = "../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]\npub mod text;\n#[path = "../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]\npub mod binary;\n'
          '#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]\npub mod sqlite;\n//#endregion 🚪️Representations\n')
snap_text = snap_doc + "\n".join(HEADER) + "@@ALIASES@@\n\n" + uses_line + "use crate::model::{EntityId, Model};\n" + snap_body.rstrip("\n") + "\n\n" + composition.rstrip("\n") + "\n" + mounts
diff = L.rd(f"{DIFF}/🦀️.rs")
diff_mounts = '\n//#region 🚪️Representations\n#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]\npub mod text;\n#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]\npub mod binary;\n//#endregion 🚪️Representations\n'
facet = ('\n//#region 🧬️Facet\n/// 🧬️ The diff facet\'s five handcrafted language leaves, owned here beside the diff they describe; the artifact schema\n'
         '/// descriptor (`semio_s_artifact_energy_model_schema`) assembles them.\npub const FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {\n'
         '    rust: include_str!("🦀️.rs"),\n    typescript: include_str!("🟦️.ts"),\n    graphql: include_str!("🔗️.graphql"),\n    json_schema: include_str!("🔣️.json"),\n    proto: include_str!("🛰️.proto"),\n};\n//#endregion 🧬️Facet\n')
d_doc_end = diff.index("\n\n") + 2
diff_text = diff[:d_doc_end] + "\n".join(HEADER) + "@@ALIASES@@\n\n" + diff[d_doc_end:].rstrip("\n") + "\n" + facet + diff_mounts
inf = L.rd(f"{INF}/🦀️.rs")
inf = L.replace_once(inf, "pub use super::entries::EnergyModelEntries;", '#[path = "🗃️entries/🦀️.rs"]\npub mod entries;\n\npub use entries::EnergyModelEntries;')
i_doc_end = inf.index("\n\n") + 2
inf_mounts = '\n//#region 🚪️Representations\n#[path = "../../🚪️io/📝️text/💡️inferences/🦀️.rs"]\npub mod text;\n#[path = "../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]\npub mod binary;\n//#endregion 🚪️Representations\n'
inf_text = inf[:i_doc_end] + "\n".join(HEADER) + "@@ALIASES@@\n\n" + inf[i_doc_end:].rstrip("\n") + "\n" + inf_mounts
sch = L.rd(f"{SCH}/🦀️.rs")
sch = L.replace_once(sch, "use crate::{EnergyModelSnapshot, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_ARTIFACT_SCHEMA_ID, ENERGY_MODEL_DOCUMENT_SCHEMA};\n", "use crate::snapshot::{EnergyModelSnapshot, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_ARTIFACT_SCHEMA_ID, ENERGY_MODEL_DOCUMENT_SCHEMA};\n")
sch, diff_leaves = L.take(sch, "        diff: semio_framework_schema_registry::FacetLeaves {\n", "        },\n")
sch = L.replace_once(sch, "        mutations: semio_framework_schema_registry::FacetLeaves {\n", "        diff: crate::diff::FACET,\n        mutations: semio_framework_schema_registry::FacetLeaves {\n")
s_doc_end = sch.index("\n\n") + 2
sch_text = sch[:s_doc_end] + "\n".join(HEADER) + "@@ALIASES@@\n\n" + sch[s_doc_end:]

# 4. path rewrites
D = r"crate\s*::\s*standards\s*::\s*v1\s*::\s*subsets\s*::\s*any\s*::\s*"
COMMON = [
    (D + r"schema\s*::\s*snapshot\s*::\s*", "crate::snapshot::"), (D + r"schema\s*::\s*diff\s*::\s*", "crate::diff::"),
    (D + r"schema\s*::\s*inferences\s*::\s*entries\s*::\s*", "crate::inferences::entries::"), (D + r"schema\s*::\s*inferences\s*::\s*", "crate::inferences::"),
    (D + r"io\s*::\s*(text|binary|sqlite)\s*::\s*snapshot\s*::\s*", r"crate::snapshot::\1::"), (D + r"io\s*::\s*(text|binary)\s*::\s*diff\s*::\s*", r"crate::diff::\1::"),
    (D + r"io\s*::\s*(text|binary)\s*::\s*inferences\s*::\s*", r"crate::inferences::\1::"),
    (D + r"schema\s*::\s*(?!mutations\b)", "crate::schema::"),
    (r"\bcrate\s*::\s*schema\s*::\s*(snapshot|diff|inferences)\s*::", r"crate::\1::"), (r"\bcrate\s*::\s*(diff|snapshot)\s*::\s*schema\s*::", r"crate::\1::"),
    (r"\bcrate\s*::\s*EnergyModelSnapshot\b", "crate::snapshot::EnergyModelSnapshot"), (r"\bcrate\s*::\s*EnergyModelDiff\b", "crate::diff::EnergyModelDiff"),
    (r"\bcrate\s*::\s*EnergyModelArtifact\b", "crate::schema::EnergyModelArtifact"),
]
moved_alt = "|".join(sorted((re.escape(n) for n in MOVED if n != "EnergyModelSnapshot"), key=len, reverse=True))
COMMON.append((r"\bcrate\s*::\s*(" + moved_alt + r")\b", r"crate::snapshot::\1"))
def split_root_groups(text):
    def fix(m):
        items = [x.strip() for x in m.group(1).split(",") if x.strip()]
        moved = [x for x in items if re.match(r"[A-Za-z_]\w*", x).group(0) in MOVED]
        rest = [x for x in items if x not in moved]
        out = []
        if rest: out.append("use crate::{" + ", ".join(rest) + "};")
        if moved: out.append("use crate::snapshot::{" + ", ".join(moved) + "};")
        return "\n".join(out)
    return re.sub(r"use crate::\{([^{}]*)\};", fix, text)
OWN = {
    "snapshot": [(D + r"schema\s*::\s*snapshot\s*::\s*", "crate::"), (D + r"io\s*::\s*(text|binary|sqlite)\s*::\s*snapshot\s*::\s*", r"crate::\1::"),
                 (r"\bcrate\s*::\s*snapshot\s*::\s*", "crate::"), (r"\bcrate\s*::\s*schema\s*::\s*snapshot\s*::\s*", "crate::")],
    "diff": [(D + r"schema\s*::\s*diff\s*::\s*", "crate::"), (D + r"io\s*::\s*(text|binary)\s*::\s*diff\s*::\s*", r"crate::\1::"),
             (r"\bcrate\s*::\s*diff\s*::\s*", "crate::"), (r"\bcrate\s*::\s*EnergyModelDiff\b", "crate::EnergyModelDiff")],
    "inferences": [(D + r"schema\s*::\s*inferences\s*::\s*entries\s*::\s*", "crate::entries::"), (D + r"schema\s*::\s*inferences\s*::\s*", "crate::"),
                   (D + r"io\s*::\s*(text|binary)\s*::\s*inferences\s*::\s*", r"crate::\1::"), (r"\bcrate\s*::\s*inferences\s*::\s*", "crate::")],
    "schema": [(D + r"schema\s*::\s*(?!mutations\b)", "crate::"), (r"\bcrate\s*::\s*schema\s*::\s*", "crate::")],
}
roots = {"snapshot": (SNAP, snap_text), "diff": (DIFF, diff_text), "inferences": (INF, inf_text), "schema": (SCH, sch_text)}
for alias, (owner, text) in roots.items():
    L.wr(f"{owner}/🦀️.rs", text)
crate_files = {}
for alias, (owner, _) in roots.items():
    files = L.closure_files(f"{owner}/🦀️.rs")
    tests = []
    for f in files:
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*\n\s*(?:pub\s+)?mod\s+\w*tests?\w*\s*;', L.rd(f)):
            tests.append(os.path.normpath(os.path.join(os.path.dirname(f), m.group(1))))
    crate_files[alias] = sorted(set(files + tests))
all_artifact = []
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if f.endswith(".rs"): all_artifact.append(os.path.join(r, f))
owned = {f: a for a, fs in crate_files.items() for f in fs}
for f in all_artifact:
    t = orig = L.rd(f); alias = owned.get(f)
    rules = (OWN[alias] + [r for r in COMMON if not r[1].startswith("crate::" + alias)]) if alias else COMMON
    if f == ROOT: t = L.rewrite_code(t, [(r"(?<![\w:])standards\s*::\s*v1\s*::", "crate::standards::v1::")])
    t2 = L.rewrite_code(t, rules)
    if alias != "snapshot": t2 = split_root_groups(t2)
    if t2 != orig: L.wr(f, t2)
# root names the moved items it uses
root = L.rd(ROOT)
used = sorted(n for n in MOVED if re.search(r"(?<![\w:])" + re.escape(n) + r"\b", L.strip(root)))
root = root.replace("extern crate semio_s_plugin_energy_epw as epw;\n", "extern crate semio_s_plugin_energy_epw as epw;\nextern crate semio_s_artifact_energy_model_diff as diff;\nextern crate semio_s_artifact_energy_model_inferences as inferences;\nextern crate semio_s_artifact_energy_model_schema as schema;\nextern crate semio_s_artifact_energy_model_snapshot as snapshot;\n", 1)
root = root.replace(uses_line, uses_line + ("use crate::snapshot::{" + ", ".join(used) + "};\n" if used else ""), 1)
L.wr(ROOT, root)
# 5. alias blocks, manifests, workspace
DESC = {"snapshot": "Energy model snapshot: persisted fields, composed children, construction and the snapshot codecs",
        "diff": "Energy model diff: the sparse per-entity delta, its patches, codecs and facet leaves",
        "inferences": "Energy model inferences: the inference schema, entries and their codecs",
        "schema": "Energy model schema: EnergyModelArtifact and the artifact schema descriptor"}
for alias, (owner, _) in roots.items():
    files = crate_files[alias]
    lib_files = [f for f in files if "🧪️tests" not in f]
    test_files = [f for f in files if "🧪️tests" in f]
    used_l, deps = L.deps_for(lib_files, L.ALIASES[alias])
    used_t, dev = L.deps_for(test_files, L.ALIASES[alias])
    used_l.discard(alias); used_t.discard(alias)
    text = L.rd(f"{owner}/🦀️.rs")
    block = L.alias_block(used_l)
    test_only = sorted(used_t - used_l - {"geometry", "pack"})
    if test_only: block += "\n" + "\n".join(f"#[cfg(test)]\nextern crate {L.ALIASES[a].replace('-', '_')} as {a};" for a in test_only)
    L.wr(f"{owner}/🦀️.rs", text.replace("@@ALIASES@@", block))
    dev_features = {"semio-framework-os-kernel": ["protocol-laws", "mutation-testing"], "semio-framework-plugin": ["artifact-app-testing"]}
    dev = (dev - deps) | ({"semio-framework-async-macros"} if any("async_test" in L.rd(f) for f in test_files) else set())
    L.manifest(owner, L.ALIASES[alias], DESC[alias], deps, sorted(dev), dev_features)
    L.ensure_workspace_rows(deps | dev)
    L.add_energy_workspace_row(L.ALIASES[alias], owner)
    L.add_workspace_member(owner)
    print(alias, "deps", sorted(deps), "dev", sorted(dev))
man_p = f"{L.ART}/📦️packages/🦀️rust/Cargo.toml"; m = L.rd(man_p)
m = m.replace("semio-s-plugin-energy-epw = { workspace = true }\n", "semio-s-plugin-energy-epw = { workspace = true }\n" + "".join(f"{L.ALIASES[a]} = {{ workspace = true }}\n" for a in ("diff", "inferences", "schema", "snapshot")), 1)
L.wr(man_p, m)
print("moved root items:", sorted(MOVED))
