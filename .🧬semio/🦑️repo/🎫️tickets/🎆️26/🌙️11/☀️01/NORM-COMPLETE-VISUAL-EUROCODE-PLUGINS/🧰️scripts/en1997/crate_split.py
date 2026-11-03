import sys, os, re, shutil
sys.path.insert(0, ".")
from spec import *
from rows import *

GROUPS = {
    "rows": ("🧱️rows", "EN 1997 row mutation leaves (layers, footings, piles, retaining walls, slopes)", ["layer", "footing", "pile", "retaining-wall", "slope"]),
    "structures": ("⚓️structures", "EN 1997 structure mutation leaves (uplift cases, ground anchors, embedded walls)", ["uplift-case", "anchor", "embedded-wall"]),
    "investigation": ("🔬️investigation", "EN 1997 investigation mutation leaves (pile groups, laboratory tests, layer water)", ["pile-group", "lab-test", "layer-water"]),
}
SLUG_GROUP = {slug: g for g, (_, _, slugs) in GROUPS.items() for slug in slugs}
NK = newkinds()
def group_of(r): return SLUG_GROUP[r.slug]
def crate_of(r): return "semio_s_artifact_norm_en1997_mut_" + group_of(r)

HEADER = '''//! 🧱 {desc}.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
{mounts}//#endregion 🪢️Mounts
'''
tmpl = open(MUT + "/🏗️foundations/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
for g, (dirn, desc, slugs) in GROUPS.items():
    base = f"{MUT}/{dirn}"
    kinds = sorted([(r, v) for r, v in NK if group_of(r) == g], key=lambda x: dirname(*x))
    mounts = "".join(f'#[path = "../{dirname(r, v)}/🦀️.rs"]\npub mod {mod(r, v)};\n' for r, v in kinds)
    os.makedirs(base + "/📦️packages/🦀️rust", exist_ok=True)
    open(base + "/🦀️.rs", "w", encoding="utf8").write(HEADER.format(desc=desc, mounts=mounts))
    toml = tmpl.replace("semio-s-artifact-norm-en1997-mut-foundations", f"semio-s-artifact-norm-en1997-mut-{g}").replace("EN 1997 foundations mutation leaves", desc)
    open(base + "/📦️packages/🦀️rust/Cargo.toml", "w", encoding="utf8").write(toml)

# strip moved mounts out of foundations and ground
for gdir in ("🏗️foundations", "🪨️ground"):
    p = f"{MUT}/{gdir}/🦀️.rs"
    s = open(p, encoding="utf8").read()
    for r, v in NK:
        s = s.replace(f'#[path = "../{dirname(r, v)}/🦀️.rs"]\npub mod {mod(r, v)};\n', "")
    open(p, "w", encoding="utf8").write(s)

# root Cargo.toml
p = ROOT + "/Cargo.toml"
s = open(p, encoding="utf8").read()
if "mut-rows" not in s:
    s = s.replace('    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️ground/📦️packages/🦀️rust",\n', '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️ground/📦️packages/🦀️rust",\n' + "".join(f'    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{d}/📦️packages/🦀️rust",\n' for d, _, _ in GROUPS.values()), 1)
    dep = "".join(f'[workspace.dependencies.semio-s-artifact-norm-en1997-mut-{g}]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{d}/📦️packages/🦀️rust"\n\n' for g, (d, _, _) in GROUPS.items())
    s = s.replace("[workspace.dependencies.semio-s-artifact-norm-en1997-mut-ground]", dep + "[workspace.dependencies.semio-s-artifact-norm-en1997-mut-ground]", 1)
    open(p, "w", encoding="utf8").write(s)
p = ROOT + "/📦️packages/🦀️rust/📜️script.ts"
s = open(p, encoding="utf8").read()
if "mut-rows" not in s:
    s = s.replace('"semio-s-artifact-norm-en1997-mut-ground",', '"semio-s-artifact-norm-en1997-mut-ground","semio-s-artifact-norm-en1997-mut-investigation","semio-s-artifact-norm-en1997-mut-rows","semio-s-artifact-norm-en1997-mut-structures",', 1)
    open(p, "w", encoding="utf8").write(s)
# dependents
for cp in (MUT + "/📦️packages/🦀️rust/Cargo.toml", S + "/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml", S + "/🚪️io/📦️packages/🦀️rust/Cargo.toml"):
    s = open(cp, encoding="utf8").read()
    if "mut-rows" in s: continue
    line = "semio-s-artifact-norm-en1997-mut-foundations = { workspace = true }\n"
    assert line in s, cp
    s = s.replace(line, line + "".join(f"semio-s-artifact-norm-en1997-mut-{g} = {{ workspace = true }}\n" for g in GROUPS), 1)
    open(cp, "w", encoding="utf8").write(s)

# use paths
def retarget(path, pattern_fn):
    s = open(path, encoding="utf8").read()
    for r, v in NK:
        old_g = r.group
        m = mod(r, v)
        s = s.replace(f"use semio_s_artifact_norm_en1997_mut_{old_g}::{m};", f"use semio_s_artifact_norm_en1997_mut_{group_of(r)}::{m};")
        s = re.sub(rf"(use semio_s_artifact_norm_en1997_mut_){old_g}(::\{{[^}}]*)\b{m}\b", lambda mo: mo.group(0), s)
    open(path, "w", encoding="utf8").write(s)
retarget(MUT + "/🦀️.rs", None)
retarget(S + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs", None)

# text codec: grouped imports
p = S + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = open(p, encoding="utf8").read()
for g_old in ("foundations", "ground"):
    mods_old = [mod(r, v) for r, v in NK if r.group == g_old]
    for m in mods_old:
        s = s.replace(f", {m}", "")
new_imports = ""
for g in GROUPS:
    names = [mod(r, v) for r, v in NK if group_of(r) == g]
    new_imports += f"use semio_s_artifact_norm_en1997_mut_{g}::{{{', '.join(names)}}};\n"
anchor = "use semio_s_artifact_norm_en1997_model::{"
if "mut_rows" not in s:
    s = s.replace(anchor, new_imports + anchor, 1)
open(p, "w", encoding="utf8").write(s)
print("split ok")
