"""🧱️ Moves the row types (structs, enums and their impls) of the en1993 snapshot schema into row-group crates and re-exports them from the model facade."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from parse_items import split_items, head
fam = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
snap = fam + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot"
text = open(snap + "/🦀️.rs", encoding="utf8").read()
assert "🧱️RowGroups" not in text
groups = {
 "materials": ("🧱️", "MaterialKind SteelMaterial SectionKind SteelSection WarpingSupport HollowFabrication StiffenerShape ParametricShape ParametricSection StressRelief".split()),
 "members": ("🧑️", "SteelMemberType LoadApplication MomentDiagram AnalysisKind StainlessForming TorsionLoad SteelMember ChordShape BuiltUpKind BuiltUpMember RestraintSupport RestraintDistribution Restraint".split()),
 "actions": ("⚖️", "DesignAction ActionKind LoadCase MemberAction ForceAction JointForceAction FrameActionKind MemberForceBasis FrameAction FrameMemberBasis".split()),
 "joints": ("🔩️", "StiffnessClass JointKind JointCategory SteelJoint WeldPattern WeldGroup JointRowType JointRow HollowJoint FastenerKind FastenedJoint BoltGroupMethod JointPlate".split()),
 "fatigue-fire": ("🔥️", "FatigueBand FatigueMethod FatigueConsequence FatigueDetail FireRating FireExposure BridgeFatigue FatigueStress".split()),
 "structures": ("🗼️", "FormingProcess ColdFormedMember PatchLoadType PlatedPanel FabricationToleranceClass SiloShell TensionComponent LatticeKind TowerLeg SheetPileEnvironment SteelPile Storey CraneRunway FoundationBlock Placement FormingGain".split()),
 "frame": ("🕸️", "FrameNodeSupport FrameNode FrameBarRelease FrameBar NodeARestraint NodeBRestraint FrameSway BucklingPlane BucklingLength MaterialDuctility".split()),
}
g_of = {n: g for g, (_, ns) in groups.items() for n in ns}
cname = lambda g: f"semio-s-artifact-norm-en1993-model-rows-{g}"
items = split_items(text)
def owner(it):
    h = head(it)
    m = re.match(r"pub (?:struct|enum) (\w+)", h) or re.match(r"impl(?:<[^>]*>)? (?:[\w:<>]+ for )?(\w+)", h)
    return m.group(1) if m else None
buckets = {g: [] for g in groups}
root = []
for it in items:
    o = owner(it)
    if o in g_of:
        lines = it.split("\n")
        keep = [l for l in lines if l.startswith("//#")]
        body = [l for l in lines if not l.startswith("//#")]
        buckets[g_of[o]].append("\n".join(body).strip("\n"))
        if keep: root.append("\n".join(keep))
    else:
        root.append(it)
deps = {g: set() for g in groups}
for g, its in buckets.items():
    for it in its:
        for n, og in g_of.items():
            if og != g and re.search(r"\b%s\b" % n, it): deps[g].add(og)
cargo_src = open(snap + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
dep_lines = re.search(r"\[dependencies\]\n(.*?)\n\n", cargo_src, re.S).group(1).split("\n")
skip = ("model-sqlite-ddl", "model-proto", "model-json-schema", "model-graphql", "model-typescript", "app-surface", "semio-plugin ")
fam_deps = [l for l in dep_lines if not any(s in l for s in skip)]
head_block = '''#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
'''
def reg(name, rel):
    cargo = fam + "/Cargo.toml"; t = open(cargo, encoding="utf8").read()
    if f'"{rel}"' not in t:
        i = t.index("members = [\n") + len("members = [\n"); t = t[:i] + f'    "{rel}",\n' + t[i:]
    if f"[workspace.dependencies.{name}]" not in t:
        t = t.rstrip("\n") + f'\n\n[workspace.dependencies.{name}]\npath = "{rel}"\n'
    open(cargo, "w", encoding="utf8").write(t)
    s = fam + "/📦️packages/🦀️rust/📜️script.ts"; b = open(s, encoding="utf8").read()
    if f'"{name}"' not in b: b = b.replace('{packages:["', '{packages:["' + name + '","', 1); open(s, "w", encoding="utf8").write(b)
for g, (emoji, ns) in groups.items():
    d = f"{snap}/🧱️rows/{emoji}{g}"; man = f"{d}/📦️packages/🦀️rust"; os.makedirs(man, exist_ok=True)
    body = "\n\n".join(buckets[g])
    uses = []
    if re.search(r"\b(AnnexChoice|ReliabilityClass)\b", body): uses.append("use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};")
    for dg in sorted(deps[g]): uses.append(f"use {cname(dg).replace('-', '_')}::*;")
    open(f"{d}/🦀️.rs", "w", encoding="utf8").write(f"//! 🧱️ EN 1993 model rows ({g}): {len([n for n in ns])} row and vocabulary types of the snapshot with their display and key helpers.\n\n" + head_block + "\n" + "\n".join(uses) + "\n\n" + body + "\n")
    lines = []
    for l in fam_deps:
        m = re.search(r'path = "([^"]+)"', l)
        if m:
            tgt = os.path.normpath(os.path.join(snap + "/📦️packages/🦀️rust", m.group(1)))
            l = l.replace(m.group(1), os.path.relpath(tgt, man))
        lines.append(l)
    lines += [f"{cname(dg)} = {{ workspace = true }}" for dg in sorted(deps[g])]
    feat = '["dep:serde", "semio-s-plugin-norm-compliance/test-support"' + "".join(f', "{cname(dg)}/test-support"' for dg in sorted(deps[g])) + "]"
    open(f"{man}/Cargo.toml", "w", encoding="utf8").write(f'''[package]
workspace = "{os.path.relpath(fam, man)}"
name = "{cname(g)}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "EN 1993 model rows ({g})"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
''' + "\n".join(lines) + f'''

[features]
test-support = {feat}
''')
    reg(cname(g), os.path.relpath(man, fam))
mount = "//#region 🧱️RowGroups\n" + "\n".join(f"pub use {cname(g).replace('-', '_')}::*;" for g in groups) + "\n//#endregion 🧱️RowGroups\n"
out = "\n".join(root)
i = out.index("use framework_schema::ArtifactSchema;") + len("use framework_schema::ArtifactSchema;")
out = out[:i] + "\n\n" + mount + out[i:]
open(snap + "/🦀️.rs", "w", encoding="utf8").write(out)
cp = snap + "/📦️packages/🦀️rust/Cargo.toml"; c = open(cp, encoding="utf8").read()
c = c.replace("[dependencies]\n", "[dependencies]\n" + "".join(f"{cname(g)} = {{ workspace = true }}\n" for g in groups), 1)
c = c.replace('test-support = ["dep:serde", ', 'test-support = ["dep:serde", ' + ", ".join(f'"{cname(g)}/test-support"' for g in groups) + ", ", 1)
open(cp, "w", encoding="utf8").write(c)
print({g: len(v) for g, v in buckets.items()}, "root items", len(root))
