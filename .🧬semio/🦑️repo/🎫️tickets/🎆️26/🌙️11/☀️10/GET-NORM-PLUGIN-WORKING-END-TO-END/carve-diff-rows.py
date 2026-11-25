"""🧱️ Moves the row patch, key and list-delta items of the en1993 diff schema into row-group crates and re-exports them from the facade.
usage: carve-diff-rows.py   (idempotent refusal if already carved)"""
import os, re, sys
fam = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993"
any_ = fam + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
diff = any_ + "/🧬️schema/🔺️diff"
src = open(diff + "/🦀️.rs", encoding="utf8").read()
assert "RowGroups" not in src
groups = {
    "members": ("🧑️", "Material Section Member LoadCase MemberAction".split()),
    "joints": ("🔩️", "Joint WeldGroup ParametricSection HollowJoint JointRow JointPlate FastenedJoint".split()),
    "fatigue-fire": ("🔥️", "FatigueDetail FireExposure BridgeFatigue FatigueStress".split()),
    "thin-walled": ("🥫️", "ColdFormedMember PlatedPanel SiloShell TensionComponent BuiltUpMember".split()),
    "structures": ("🗼️", "TowerLeg SteelPile CraneRunway Storey Restraint FoundationBlock Placement".split()),
    "frame": ("🕸️", "FrameNode FrameBar FrameAction FrameMemberBasis BucklingLength MaterialDuctility FormingGain".split()),
}
row_of_type = {"SteelMaterial": "Material", "SteelSection": "Section", "SteelMember": "Member", "LoadCase": "LoadCase", "MemberAction": "MemberAction", "SteelJoint": "Joint", "FatigueDetail": "FatigueDetail", "FireExposure": "FireExposure", "ColdFormedMember": "ColdFormedMember", "PlatedPanel": "PlatedPanel", "SiloShell": "SiloShell", "TensionComponent": "TensionComponent", "BridgeFatigue": "BridgeFatigue", "TowerLeg": "TowerLeg", "SteelPile": "SteelPile", "CraneRunway": "CraneRunway", "Storey": "Storey", "Restraint": "Restraint", "BuiltUpMember": "BuiltUpMember", "WeldGroup": "WeldGroup", "ParametricSection": "ParametricSection", "HollowJoint": "HollowJoint", "JointRow": "JointRow", "FrameNode": "FrameNode", "FrameBar": "FrameBar", "Placement": "Placement", "JointPlate": "JointPlate", "FoundationBlock": "FoundationBlock", "MaterialDuctility": "MaterialDuctility", "FormingGain": "FormingGain", "BucklingLength": "BucklingLength", "FatigueStress": "FatigueStress", "FrameMemberBasis": "FrameMemberBasis", "FastenedJoint": "FastenedJoint", "FrameAction": "FrameAction"}
group_of_row = {r: g for g, (_, rows) in groups.items() for r in rows}
lines = src.split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith("//#region 🔖️Rows")) + 1
end = next(i for i, l in enumerate(lines) if l.startswith("//#region 🔖️Diff"))
body = lines[start:end]
items, cur, depth, opened = [], [], 0, False
for l in body:
    cur.append(l)
    code = re.sub(r'"(?:\\.|[^"\\])*"', '""', re.sub(r"//.*", "", l))
    depth += code.count("{") - code.count("}")
    if "{" in code: opened = True
    if (opened and depth == 0) or (not opened and code.rstrip().endswith(";") and not code.lstrip().startswith(("#", "///"))):
        items.append("\n".join(cur).strip("\n")); cur, depth, opened = [], 0, False
tail = "\n".join(cur).strip()
assert not tail, tail[:200]
buckets = {g: [] for g in groups}
for it in items:
    m = re.search(r"semio_s_artifact_norm_en1993_model::(\w+)", it)
    if m:
        row = row_of_type[m.group(1)]
    else:
        k = re.search(r"En1993(\w+?)DeltaKey", it)
        row = k.group(1) if k.group(1) in group_of_row else "Steel" + k.group(1)
    buckets[group_of_row[row]].append(it)
assert sum(len(v) for v in buckets.values()) == len(items)
header = '''#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

'''
cargo_src = open(diff + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
mounts, deps, feats = [], [], []
for g, (emoji, rows) in groups.items():
    d = f"{diff}/🧱️rows/{emoji}{g}"
    man = f"{d}/📦️packages/🦀️rust"
    os.makedirs(man, exist_ok=True)
    name = f"semio-s-artifact-norm-en1993-diff-rows-{g}"
    open(f"{d}/🦀️.rs", "w", encoding="utf8").write(f"//! 🧱️ EN 1993 diff rows ({g}): the sparse field patch, key extractor and keyed row delta of {len(rows)} document row lists.\n\n" + header + "\n\n".join(buckets[g]) + "\n")
    ws = os.path.relpath(fam, man)
    open(f"{man}/Cargo.toml", "w", encoding="utf8").write(f'''[package]
workspace = "{ws}"
name = "{name}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "EN 1993 diff rows ({g})"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
serde = {{ workspace = true, optional = true }}
semio-os-kernel = {{ workspace = true }}
semio-framework-schema = {{ workspace = true }}
semio-framework-value-derive = {{ workspace = true }}
semio-s-artifact-norm-en1993-model = {{ workspace = true }}

[features]
test-support = ["dep:serde", "semio-s-artifact-norm-en1993-model/test-support"]
''')
    rel = os.path.relpath(man, fam)
    mounts.append(f"pub use {name.replace('-', '_')}::*;")
    deps.append(name)
    # register
    cargo = fam + "/Cargo.toml"; t = open(cargo, encoding="utf8").read()
    if f'"{rel}"' not in t:
        i = t.index("members = [\n") + len("members = [\n"); t = t[:i] + f'    "{rel}",\n' + t[i:]
    if f"[workspace.dependencies.{name}]" not in t:
        t = t.rstrip("\n") + f'\n\n[workspace.dependencies.{name}]\npath = "{rel}"\n'
    open(cargo, "w", encoding="utf8").write(t)
    s = fam + "/📦️packages/🦀️rust/📜️script.ts"; b = open(s, encoding="utf8").read()
    if f'"{name}"' not in b: b = b.replace('{packages:["', '{packages:["' + name + '","', 1); open(s, "w", encoding="utf8").write(b)
lines[start:end]
items, cur, depth, opened = [], [], 0, False
for l in body:
    cur.append(l)
    code = re.sub(r'"(?:\\.|[^"\\])*"', '""', re.sub(r"//.*", "", l))
    depth += code.count("{") - code.count("}")
    if "{" in code: opened = True
    if (opened and depth == 0) or (not opened and code.rstrip().endswith(";") and not code.lstrip().startswith(("#", "///"))):
        items.append("\n".join(cur).strip("\n")); cur, depth, opened = [], 0, False
tail = "\n".join(cur).strip()
assert not tail, tail[:200]
buckets = {g: [] for g in groups}
for it in items:
    m = re.search(r"semio_s_artifact_norm_en1993_model::(\w+)", it)
    if m:
        row = row_of_type[m.group(1)]
    else:
        k = re.search(r"En1993(\w+?)DeltaKey", it)
        row = k.group(1) if k.group(1) in group_of_row else "Steel" + k.group(1)
    buckets[group_of_row[row]].append(it)
assert sum(len(v) for v in buckets.values()) == len(items)
header = '''#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

'''
cargo_src = open(diff + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
mounts, deps, feats = [], [], []
for g, (emoji, rows) in groups.items():
    d = f"{diff}/🧱️rows/{emoji}{g}"
    man = f"{d}/📦️packages/🦀️rust"
    os.makedirs(man, exist_ok=True)
    name = f"semio-s-artifact-norm-en1993-diff-rows-{g}"
    open(f"{d}/🦀️.rs", "w", encoding="utf8").write(f"//! 🧱️ EN 1993 diff rows ({g}): the sparse field patch, key extractor and keyed row delta of {len(rows)} document row lists.\n\n" + header + "\n\n".join(buckets[g]) + "\n")
    ws = os.path.relpath(fam, man)
    open(f"{man}/Cargo.toml", "w", encoding="utf8").write(f'''[package]
workspace = "{ws}"
name = "{name}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "EN 1993 diff rows ({g})"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
serde = {{ workspace = true, optional = true }}
semio-os-kernel = {{ workspace = true }}
semio-framework-schema = {{ workspace = true }}
semio-framework-value-derive = {{ workspace = true }}
semio-s-artifact-norm-en1993-model = {{ workspace = true }}

[features]
test-support = ["dep:serde", "semio-s-artifact-norm-en1993-model/test-support"]
''')
    rel = os.path.relpath(man, fam)
    mounts.append(f"pub use {name.replace('-', '_')}::*;")
    deps.append(name)
    # register
    cargo = fam + "/Cargo.toml"; t = open(cargo, encoding="utf8").read()
    if f'"{rel}"' not in t:
        i = t.index("members = [\n") + len("members = [\n"); t = t[:i] + f'    "{rel}",\n' + t[i:]
    if f"[workspace.dependencies.{name}]" not in t:
        t = t.rstrip("\n") + f'\n\n[workspace.dependencies.{name}]\npath = "{rel}"\n'
    open(cargo, "w", encoding="utf8").write(t)
    s = fam + "/📦️packages/🦀️rust/📜️script.ts"; b = open(s, encoding="utf8").read()
    if f'"{name}"' not in b: b = b.replace('{packages:["', '{packages:["' + name + '","', 1); open(s, "w", encoding="utf8").write(b)
new_body = ["", "//#region 🧱️RowGroups", "// rows_carved: the patch, key and delta items live in the row-group crates below.", *mounts, "//#endregion 🧱️RowGroups", ""]
new_body[2] = "// rows_carved"
lines[start:end] = ["", "//#region 🧱️RowGroups", *mounts, "//#endregion 🧱️RowGroups", ""]
open(diff + "/🦀️.rs", "w", encoding="utf8").write("\n".join(lines))
cp = diff + "/📦️packages/🦀️rust/Cargo.toml"; c = open(cp, encoding="utf8").read()
c = c.replace("[dependencies]\n", "[dependencies]\n" + "".join(f"{d} = {{ workspace = true }}\n" for d in deps), 1)
c = c.replace('test-support = ["dep:serde", ', 'test-support = ["dep:serde", ' + ", ".join(f'"{d}/test-support"' for d in deps) + ", ", 1)
c = c.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + "".join(f'{d} = {{ workspace = true, features = ["test-support"] }}\n' for d in deps), 1)
open(cp, "w", encoding="utf8").write(c)
print({g: len(v) for g, v in buckets.items()})
