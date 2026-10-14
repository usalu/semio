#!/usr/bin/env python3
"""✂️ Cuts the energy mutation vocabulary into domain crates, the aggregate and a fixture-law crate (lane P1, slice S2c).

- Each leaf implements `MutationKind<EnergyModelSnapshot, Op>` for every `Op: Mutation<…, Diff = EnergyModelDiff> + From<…>`
  and its builder returns `Op: From<Payload>`, so no leaf names the aggregate (the enum↔leaf cycle is inverted).
- `-mut-<domain>` (`🧬️mutations/<domain>/🦀️.rs`, 47 crates): the domain's operations, re-exported at the domain root.
- `-mutations` (`🧬️mutations/🦀️.rs`): `EnergyModelMutation` over the domain payloads plus its own text/binary op codecs
  (`🚪️io/<representation>/🧬️mutations`, mounted here for the orphan rule).
- `-mutation-fixtures` (`🧬️mutations/🧪️tests/🔬️fixtures`): the fixture-law support (`Case`, the eight laws, the roster
  `KINDS`/`DIRECTORIES`, the codec probe corpus). Its `[[test]]` targets run every domain's committed vectors
  (`🧬️mutations/🧪️tests/🔬️<domain>/🦀️.rs`) and the aggregate laws (`🔬️aggregate-laws`). Committed vectors are read at run time.
usage: ✂️cut-mutations.py
"""
import importlib.util, json, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
MUT = f"{L.ANY}/🧬️schema/🧬️mutations"
TESTS = f"{MUT}/🧪️tests"
FIXT = f"{TESTS}/🔬️fixtures"
IO = f"{L.ANY}/🚪️io"
ROOT = f"{L.ART}/🦀️.rs"
cfg = L.load_domains()
if os.path.exists(f"{MUT}/🔋️model/🦀️.rs"): sys.exit("already applied")

# 0. leaf facts
leaves = []  # (slug, domain_dir, op_dir, kind, module, payload, builder)
dom_dir = {}
for emoji, slug, nouns, kinds in cfg["domains"]:
    d = emoji + slug; dom_dir[slug] = d
    for op in sorted(os.listdir(f"{MUT}/{d}")):
        src = L.rd(f"{MUT}/{d}/{op}/🦀️.rs")
        kind = json.loads(L.rd(f"{MUT}/{d}/{op}/🔣️.json"))["semanticKind"]
        payload = re.search(r"(?m)^pub struct (\w+)", src).group(1)
        builder = re.search(r"(?m)^pub fn (\w+)", src).group(1)
        leaves.append((slug, d, op, kind, kind.replace("-", "_"), payload, builder))
assert len(leaves) == 297
by_builder = {l[6]: l for l in leaves}; by_payload = {l[5]: l for l in leaves}
def dalias(slug): return L.domain_alias(slug)

# 1. leaves: generic over the aggregate
for slug, d, op, kind, module, payload, builder in leaves:
    comp_p, inv_p, diff_p = f"{MUT}/{d}/{op}/🦀️.rs", f"{MUT}/{d}/{op}/↩️inverse/🦀️.rs", f"{MUT}/{d}/{op}/🔺️diff/🦀️.rs"
    inv = L.rd(inv_p)
    called = sorted(set(re.findall(r"\bvocabulary::([a-z_0-9]+)\(", L.strip(inv))))
    bounds = [f"From<super::{payload}>"]
    for b in called:
        if b == builder: continue
        o = by_builder[b]
        bounds.append(f"From<crate::{o[5]}>" if o[0] == slug else f"From<crate::{dalias(o[0])}::{o[5]}>")
        if o[0] != slug: inv = re.sub(r"\bvocabulary::" + b + r"\(", f"crate::{dalias(o[0])}::{b}(", inv)
    inv = L.replace_once(inv, "use crate::mutations as vocabulary;\n", "use crate as vocabulary;\n")
    inv = L.replace_once(inv, "use crate::mutations::EnergyModelMutation;\n", "")
    inv = L.replace_once(inv, f"pub fn inverse(payload: &super::{payload}, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {{",
                         f"pub fn inverse<Op>(payload: &super::{payload}, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>\nwhere\n    Op: {' + '.join(bounds)},\n{{")
    if "use crate as vocabulary;" in inv and "vocabulary::" not in L.strip(inv.replace("use crate as vocabulary;", "")):
        inv = inv.replace("use crate as vocabulary;\n", "")
    L.wr(inv_p, inv)
    dtext = L.rd(diff_p)
    if "use crate::mutations as vocabulary;" in dtext: L.wr(diff_p, dtext.replace("use crate::mutations as vocabulary;\n", "use crate as vocabulary;\n"))
    comp = L.rd(comp_p)
    comp = L.replace_once(comp, "use crate::mutations::EnergyModelMutation;\n", "")
    m = re.search(r"pub fn " + builder + r"\(([^)]*)\) -> EnergyModelMutation \{\n    EnergyModelMutation::\w+\((\w+ \{[^}]*\})\)\n\}", comp)
    comp = comp[:m.start()] + f"pub fn {builder}<Op: From<{payload}>>({m.group(1)}) -> Op {{\n    {m.group(2)}.into()\n}}" + comp[m.end():]
    comp_bounds = [b.replace("super::", "") for b in bounds]
    comp = L.replace_once(comp, f"impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for {payload} {{",
                          f"impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for {payload}\nwhere\n    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + {' + '.join(comp_bounds)},\n{{")
    comp = L.replace_once(comp, "-> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError>", "-> Result<Vec<Op>, semio_framework_value::ValueError>")
    comp = comp.replace("/// 🏗️ Builder — wraps the payload in its dispatch variant.", "/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.")
    L.wr(comp_p, comp)

# 2. domain roots
agg_p = f"{MUT}/🦀️.rs"; agg = L.rd(agg_p)
agg, link_roles = L.take(agg, "//#region 🔖️LinkRoles\n", "//#endregion 🔖️LinkRoles\n")
HEADER = "#![allow(async_fn_in_trait)]\n#![allow(clippy::too_many_arguments)]\n\n"
for emoji, slug, nouns, kinds in cfg["domains"]:
    d = dom_dir[slug]; mine = [l for l in leaves if l[0] == slug]
    mounts = []
    for _, _, op, kind, module, payload, builder in mine:
        mounts.append(f'#[path = "."]\npub mod {module} {{\n    #[path = "{op}/🦀️.rs"]\n    mod component;\n    pub use component::*;\n'
                      f'    #[path = "{op}/🔺️diff/🦀️.rs"]\n    pub mod diff;\n    #[path = "{op}/↩️inverse/🦀️.rs"]\n    pub mod inverse;\n}}\n')
    reexports = "".join(f"pub use {l[4]}::{{{l[6]}, {l[5]}}};\n" for l in mine)
    names = ", ".join(f"`{l[3]}`" for l in mine)
    doc = f"//! {emoji} Energy model mutation domain `{slug}`: {names}.\n\n"
    extra = ("\n" + link_roles) if slug == "model" else ""
    L.wr(f"{MUT}/{d}/🦀️.rs", doc + HEADER + "@@ALIASES@@\n\nuse crate::diff::EnergyModelDiff;\nuse crate::snapshot::EnergyModelSnapshot;\n" + extra + "\n" + "\n".join(mounts) + "\n" + reexports)
LEAF_RULES = [(r"\bcrate\s*::\s*mutations\s*::\s*(WEATHER_LINK_ROLE|REFERENCED_MODEL_LINK_ROLE)\b", r"crate::\1")]

# 3. aggregate
agg, _ = L.take(agg, "//#region 🔖️Reexports\n", "//#endregion 🔖️Reexports\n")
agg, kinds_block = L.take(agg, "/// 🏷️ Direct semantic roster exported for the language-neutral test adapter", "];\n")
agg, dirs_block = L.take(agg, "/// 🗂️ `(semanticKind, domain/operation owner directory)` for every declared kind", "];\n")
agg, bridge = L.take(agg, "//#region 🌉️TestBridge\n", "//#endregion 🌉️TestBridge\n")
agg, probes = L.take(agg, "//#region 🧵️WireProbes\n", "//#endregion 🧵️WireProbes\n")
agg, laws = L.take(agg, "//#region 🧰️Fixtures\n", "//#endregion 🧪️StructuralCorrespondence\n")
def variant(m):
    name, inner = m.group(1), m.group(2)
    if inner in by_payload: return f"    {name}(crate::{dalias(by_payload[inner][0])}::{inner}),"
    return m.group(0)
agg = re.sub(r"(?m)^    (\w+)\((\w+)\),$", variant, agg)
agg = agg.replace("use crate::diff::EnergyModelDiff;\nuse crate::snapshot::EnergyModelSnapshot;\n", "use crate::diff::EnergyModelDiff;\nuse crate::snapshot::EnergyModelSnapshot;\n", 1)
agg_doc_end = agg.index("\n\n") + 2
agg_mounts = ('\n//#region 🚪️Representations\n#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]\npub mod text;\n#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]\npub mod binary;\n'
              '//#endregion 🚪️Representations\n')
agg = agg[:agg_doc_end] + HEADER + "@@ALIASES@@\n\n" + agg[agg_doc_end:].rstrip("\n") + "\n" + agg_mounts
agg = agg.replace("so a new kind costs one enum variant and one\n//! re-export line here", "so a new kind costs one enum variant here")
L.wr(agg_p, agg)
tm_p = f"{IO}/📝️text/🧬️mutations/🦀️.rs"; tm = L.rd(tm_p)
tm = re.sub(r"(?m)^use crate::standards::v1::subsets::any::schema::mutations::[^\n]*\n", "", tm)
tm = tm.replace("pub use crate::mutations::EnergyModelMutation;\n", "use crate::EnergyModelMutation;\n")
tm = tm.replace("use crate::diff::EnergyModelDiff;\n", "").replace("use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};\n", "")
L.wr(tm_p, tm)
bm_p = f"{IO}/💾️binary/🧬️mutations/🦀️.rs"; bm = L.rd(bm_p)
bm = bm.replace("use crate::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;\n", "use crate::EnergyModelMutation;\n")
bm = bm.replace("#[cfg(test)] use super::*;\npub use crate::mutations::EnergyModelMutation;\n", "use crate::EnergyModelMutation;\n")
bm, bm_test = L.take(bm, "#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;\n", "")
L.wr(bm_p, bm)
for rep in ("📝️text", "💾️binary"):
    p = f"{IO}/{rep}/🦀️.rs"; t = L.rd(p)
    t = L.replace_once(t, '#[path = "🧬️mutations/🦀️.rs"]\npub mod mutations;\n', "")
    assert "mod " not in L.strip(t), p
    os.remove(p)
io = L.rd(f"{IO}/🦀️.rs")
io = re.sub(r'(?:///[^\n]*\n)*#\[path = "💾️binary/🦀️.rs"\]\npub mod binary;\n', "", io)
io = re.sub(r'(?:///[^\n]*\n)*#\[path = "📝️text/🦀️.rs"\]\npub mod text;\n', "", io)
L.wr(f"{IO}/🦀️.rs", io)

# 4. fixtures crate
fx = L.rd(f"{FIXT}/🦀️.rs")
fx = L.replace_once(fx, "use super::EnergyModelMutation;\n", "use crate::mutations::EnergyModelMutation;\n")
fx = L.replace_once(fx, """pub struct Case {
    pub kind: &'static str,
    pub directory: &'static str,
    pub before: &'static str,
    pub after: &'static str,
    pub mutation: &'static str,
    pub diff: &'static str,
    pub outcome: &'static str,
    pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
}
""", """pub struct Case {
    pub kind: &'static str,
    pub directory: &'static str,
    pub before: String,
    pub after: String,
    pub mutation: String,
    pub diff: String,
    pub outcome: String,
    pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
}

impl Case {
    /// 🧫️ Reads one committed `(before, mutation, after, diff, outcome)` vector from its fixture bundle at run time; a leaf
    /// not yet materialized reads empty documents, which every law then refuses by name.
    pub fn load(kind: &'static str, directory: &'static str, scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation)) -> Self {
        let bundle = fixture_root().join(directory);
        let read = |leaf: &str| std::fs::read_to_string(bundle.join(leaf)).unwrap_or_default();
        Case {
            kind,
            directory,
            before: read("📸️snapshot/⬅️before/🔣️.json"),
            after: read("📸️snapshot/➡️after/🔣️.json"),
            mutation: read("🦠️mutation/🔣️.json"),
            diff: read("🔺️diff/🔣️.json"),
            outcome: read("🎯️outcome/🔣️.json"),
            scenario,
        }
    }
}

/// 📁️ The committed `🧫️fixtures/🧬️mutations/<domain>/<operation>/<case>` tree, resolved from this crate's manifest.
pub fn fixture_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations")
}
""")
fx = re.sub(r"\bcase\.(before|after|mutation|diff|outcome)\b(?!\s*[:=])", r"case.\1.as_str()", fx)
fx = re.sub(r'std::path::Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\("[^"]*🧫️fixtures/🧬️mutations"\)\.join\(case\.directory\)', "fixture_root().join(case.directory)", fx)
probe_body = re.search(r"#\[cfg\(test\)\]\n(pub fn wire_probes\(\) -> Vec<EnergyModelMutation> \{.*?\n\})", probes, re.S).group(1)
probe_doc = "/// 🧵️ One representative value per declared variant — the codec round-trip corpus, so wire coverage\n/// grows with the enum instead of with a hand-maintained list of tests.\n"
used_builders = sorted(set(re.findall(r"\b([a-z_0-9]+)\(", probe_body)) & set(by_builder))
imports = {}
for b in used_builders: imports.setdefault(by_builder[b][0], []).append(b)
probe_uses = "".join(f"use crate::{dalias(s)}::{{{', '.join(sorted(bs))}}};\n" for s, bs in sorted(imports.items()))
fx_doc = "//! 🧰️ Energy mutation fixture laws: the committed-vector case shape and the eight laws every vector is held to, the\n//! roster the language-neutral adapter mirrors, and the codec probe corpus. Its `[[test]]` targets run them.\n\n"
fx = fx_doc + HEADER + "@@ALIASES@@\n\n" + probe_uses + fx.rstrip("\n") + "\n\n//#region 🔖️Roster\n" + kinds_block + "\n" + dirs_block + "//#endregion 🔖️Roster\n\n//#region 🧵️WireProbes\n" + probe_doc + probe_body + "\n//#endregion 🧵️WireProbes\n"
L.wr(f"{FIXT}/🦀️.rs", fx)
# leaf fixture tests: committed vectors read at run time
for slug, d, op, kind, module, payload, builder in leaves:
    tdir = f"{MUT}/{d}/{op}/🧪️tests"
    for case in sorted(os.listdir(tdir)):
        p = f"{tdir}/{case}/🦀️.rs"; t = L.rd(p)
        t = re.sub(r'(?m)^const (BEFORE|AFTER|MUTATION|DIFF|OUTCOME): &str = include_str!\("[^"]*"\);\n', "", t)
        t = re.sub(r'Case \{ kind: "([^"]+)", directory: "[^"]+", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario \}',
                   lambda m: f'Case::load("{m.group(1)}", "{d}/{op}/{case}", scenario)', t)
        t = t.replace("use crate::mutations::fixtures::", "use crate::fixtures::").replace("crate::mutations::fixtures::", "crate::fixtures::")
        t = L.rewrite_code(t, LEAF_RULES)
        L.wr(p, t)
# per-domain test roots
targets = []
for emoji, slug, nouns, kinds in cfg["domains"]:
    d = dom_dir[slug]; mods = []
    for _, _, op, kind, module, payload, builder in [l for l in leaves if l[0] == slug]:
        cases = sorted(os.listdir(f"{MUT}/{d}/{op}/🧪️tests"))
        body = "".join(f'    #[path = "../../{d}/{op}/🧪️tests/{c}/🦀️.rs"]\n    mod tests_{re.sub(r"[^a-z0-9]+", "_", re.sub(r"^[^a-z]+", "", c)).strip("_")};\n' for c in cases)
        mods.append(f'#[path = "."]\nmod {module} {{\n    pub use crate::{dalias(slug)}::{module}::*;\n{body}}}\n')
    root_dir = f"{TESTS}/🔬️{slug}"
    L.wr(f"{root_dir}/🦀️.rs", f"//! 🧪️ The `{slug}` domain's committed vectors, each held to the fixture laws through the aggregate.\n\n@@ALIASES@@\n\n" + "\n".join(mods))
    targets.append((slug.replace("-", "_") + "_vectors", f"../../../🔬️{slug}/🦀️.rs", root_dir))
laws_dir = f"{TESTS}/🔬️aggregate-laws"
L.wr(f"{laws_dir}/🦀️.rs", "//! 🎯️ Laws over the whole aggregate: middle-row inverses, structural correspondence of the roster with the leaves\n"
     "//! and the oracle catalog, and the binary op codec round trip over the probe corpus.\n\n@@ALIASES@@\n\n"
     "use crate::fixtures::{wire_probes, DIRECTORIES, KINDS};\nuse crate::mutations::binary::{decode_op, encode_op};\nuse crate::mutations::EnergyModelMutation;\n\n"
     '#[path = "../🔬️middle-row-laws/🦀️.rs"]\nmod middle_row_laws;\n#[path = "../🔬️structural-correspondence/🦀️.rs"]\nmod structural_correspondence;\n'
     '#[path = "../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]\nmod binary_codec;\n')
targets.append(("aggregate_laws", "../../../🔬️aggregate-laws/🦀️.rs", laws_dir))
bt_p = f"{IO}/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"; bt = L.rd(bt_p)
bt = bt.replace("use crate::standards::v1::subsets::any::io::binary::mutations::*;\n", "use super::*;\n").replace("crate::mutations::wire_probes()", "wire_probes()")
L.wr(bt_p, bt)
sc_p = f"{TESTS}/🔬️structural-correspondence/🦀️.rs"; sc = L.rd(sc_p)
sc = sc.replace('std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")', 'std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")')
L.wr(sc_p, sc)

# 5. builder/payload references everywhere else name the owning domain crate
def domain_refs(text):
    def fix(m):
        name = m.group(2)
        o = by_builder.get(name) or by_payload.get(name)
        return f"crate::{dalias(o[0])}::{name}" if o else m.group(0)
    text = L.rewrite_code(text, [(r"\bcrate\s*::\s*standards\s*::\s*v1\s*::\s*subsets\s*::\s*any\s*::\s*schema\s*::\s*mutations\s*::\s*", "crate::mutations::")])
    s = L.strip(text); edits = []
    for m in re.finditer(r"(?<![\w:])(crate::mutations::|mutations::|v::|vocabulary::)([A-Za-z_][A-Za-z0-9_]*)\b", s):
        o = by_builder.get(m.group(2)) or by_payload.get(m.group(2))
        if o: edits.append((m.start(), m.end(), f"crate::{dalias(o[0])}::{m.group(2)}"))
    for a, b, r in reversed(edits): text = text[:a] + r + text[b:]
    return text
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(r, f)
        if p.startswith(MUT + "/") and not p.startswith(TESTS + "/") and "/🧪️tests/" not in p and p != agg_p: continue
        t = L.rd(p); t2 = domain_refs(t)
        t2 = t2.replace("use crate::mutations as v;\n", "").replace("use crate::mutations;\n", "")
        if t2 != t: L.wr(p, t2)

# 6. artifact root: the mutations subtree leaves the mount tree
root = L.rd(ROOT)
root, _ = L.take(root, '                #[path = "."]\n                pub mod schema {\n                    #[path = "."]\n                    pub mod mutations {\n', "\n                }\n", True)
root, _ = L.take(root, "pub mod mutations {\n    pub use crate::standards::v1::subsets::any::schema::mutations::*;\n", "}\n")
root = L.replace_once(root, "pub use crate::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;\n", "")
dom_aliases = "".join(f"extern crate {L.ALIASES[dalias(s)].replace('-', '_')} as {dalias(s)};\n" for _, s, _, _ in cfg["domains"])
root = L.replace_once(root, "extern crate semio_s_artifact_energy_model_inferences as inferences;\n",
                      "extern crate semio_s_artifact_energy_model_inferences as inferences;\nextern crate semio_s_artifact_energy_model_mutations as mutations;\n" + dom_aliases)
L.wr(ROOT, root)
json.dump({"targets": targets}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "mutation-targets.json"), "w"), ensure_ascii=False, indent=1)
print("leaves", len(leaves), "domains", len(cfg["domains"]), "targets", len(targets))
