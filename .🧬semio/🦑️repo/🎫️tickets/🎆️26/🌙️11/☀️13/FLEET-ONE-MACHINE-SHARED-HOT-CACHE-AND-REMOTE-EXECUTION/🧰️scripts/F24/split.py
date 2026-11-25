import os, re, subprocess, sys
R = "/Users/ueli/Documents/semio"
DWG = R + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg"
OBJ_REL = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️objects"
OBJ = DWG + "/" + OBJ_REL
OLD = "semio-s-artifact-stdio-dwg-snapshot-objects"
DOM = {  # folder -> (suffix, internal deps)
 "🎛️metadata": ("metadata", []),
 "🏷️xrecord": ("xrecord", []),
 "🗃️tables": ("tables", []),
 "🔗️associativity": ("associativity", []),
 "📐️entities": ("entities", ["tables"]),
 "🖌️styles": ("styles", ["tables"]),
 "📏️constraints": ("constraints", ["associativity"]),
 "🧩️blocks": ("blocks", ["associativity"]),
}
MACRO_USERS = {"tables","associativity","xrecord","entities","constraints","styles"}
def pkg(s): return OLD + "-" + s
def lib(s): return pkg(s).replace("-", "_")
BASE = ["semio-framework-dsl-record","semio-framework-dsl-record-derive","semio-os-kernel","semio-framework-pack-json","semio-framework-schema","semio-framework-schema-composition","semio-framework-schema-state","semio-framework-value","semio-framework-value-native-owned","semio-framework-value-derive"]
EXT = """#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
"""
def cargo(s, deps, desc):
    d = "".join(f"{x} = {{ workspace = true }}\n" for x in deps)
    return f'''[package]
workspace = "../../../../../../../../../.."
name = "{pkg(s)}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
{d}
[dev-dependencies]
semio-os-kernel = {{ workspace = true, features = ["protocol-laws", "mutation-testing"] }}
'''
PFX = "use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::"
def rw_use(text):
    return re.sub(r"use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::([a-z]+)::", lambda m: f"use {lib(DOM_BY_MOD[m.group(1)])}::", text)
DOM_BY_MOD = {v[0]: v[0] for v in DOM.values()}
def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w").write(t)
# 1 domain crates
UNIT_TAIL = "\n"
for folder, (s, internal) in DOM.items():
    f = f"{OBJ}/{folder}/🦀️.rs"
    t = open(f).read()
    if "extern crate semio_os_kernel" in t or "macro_export" in t:
        print("already", s); continue
    t = rw_use(t)
    lines = t.split("\n")
    i = 0
    while lines[i].startswith("//!"): i += 1
    hd = "\n".join(lines[:i]); rest = "\n".join(lines[i:]).lstrip("\n")
    if s == "metadata":
        t2 = hd + "\n\n" + rest
        t2 = re.sub(r"^macro_rules! (dwg_\w+)", r"#[macro_export]\nmacro_rules! \1", t2, flags=re.M)
        t2 = re.sub(r"(?<![\w$:])(dwg_(?:ordinary_shape|controlled_shape|borrowed_shape|borrowed_metadata))!\(", r"$crate::\1!(", t2)
        wr(f, t2)
    else:
        used = [m for m in ("dwg_controlled_payloads", "dwg_metadata") if m + "!" in rest]
        imps = f"use {lib('metadata')}::{{{', '.join(sorted(used))}}};\n" if used else ""
        wr(f, hd + "\n\n" + EXT + "\n" + imps + rest)
    users = s in MACRO_USERS
    deps = [] if s == "metadata" else list(BASE)
    deps += [pkg(x) for x in internal + (["metadata"] if users else [])]
    wr(f"{OBJ}/{folder}/📦️packages/🦀️rust/Cargo.toml", cargo(s, deps, f"Stdio dwg snapshot object model: {s}"))
# 2 root
rp0 = f"{OBJ}/📦️packages/🦀️rust/Cargo.toml"
f = f"{OBJ}/🦀️.rs"
t = open(f).read()
if "#[macro_use]" in t:
    t = re.sub(r"#\[macro_use\]\n(#\[path = \"[^\"]+\"\]\npub mod \w+;\n)+", "", t) if False else t
    t = re.sub(r"(#\[macro_use\]\n)?#\[path = \"[^\"]+/🦀️\.rs\"\]\npub mod \w+;\n", "", t)
    t = rw_use(t)
    t = t.replace("use semio_framework_value::{ValueError, ValueRefusalKind};\n", EXT + "\nuse semio_framework_value::{ValueError, ValueRefusalKind};\nuse " + lib("metadata") + "::{dwg_controlled_payloads, dwg_metadata};\n", 1)
    wr(f, t)
    t = open(rp0).read()
    wr(rp0, t.replace('path = "🦀️.rs"', 'path = "../../🦀️.rs"'))
    os.remove(f"{OBJ}/📦️packages/🦀️rust/🦀️.rs")
rp = f"{OBJ}/📦️packages/🦀️rust/Cargo.toml"
t = open(rp).read()
if pkg("tables") not in t:
    t = t.replace("[dependencies]\n", "[dependencies]\n" + "".join(f"{pkg(v[0])} = {{ workspace = true }}\n" for v in DOM.values()), 1)
    wr(rp, t)
# 3 workspace
wp = DWG + "/Cargo.toml"
t = open(wp).read()
if pkg("tables") not in t:
    mem = f'    "{OBJ_REL}/📦️packages/🦀️rust",\n'
    new = "".join(f'    "{OBJ_REL}/{k}/📦️packages/🦀️rust",\n' for k in DOM)
    assert mem in t
    t = t.replace(mem, new + mem, 1)
    old = f'[workspace.dependencies.{OLD}]\npath = "{OBJ_REL}/📦️packages/🦀️rust"\n'
    assert old in t
    nd = "".join(f'[workspace.dependencies.{pkg(v[0])}]\npath = "{OBJ_REL}/{k}/📦️packages/🦀️rust"\n\n' for k, v in DOM.items())
    t = t.replace(old, nd + old, 1)
    wr(wp, t)
# 4 consumers
FULL = "semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::"
files = subprocess.run(["git","grep","-lI","dwg_snapshot_objects","--","*.rs"],cwd=R,capture_output=True,text=True).stdout.split("\n")
files = [x for x in files if x and "🎫️tickets" not in x]
need = {}
def crate_of(path):
    d = os.path.dirname(path)
    while d:
        if os.path.exists(f"{R}/{d}/📦️packages/🦀️rust/Cargo.toml"): return f"{d}/📦️packages/🦀️rust/Cargo.toml"
        d = os.path.dirname(d)
for x in files:
    p = R + "/" + x
    t = open(p).read()
    c = crate_of(x)
    n = need.setdefault(c, set())
    for m in re.finditer(re.escape(FULL) + r"(?:([a-z]+)::)?([A-Za-z0-9_]+)", t):
        if m.group(1) in DOM_BY_MOD: n.add(m.group(1))
        else: n.add("")
    t = re.sub(re.escape(FULL) + "(" + "|".join(DOM_BY_MOD) + r")::", lambda m: lib(m.group(1)) + "::", t)
    t = t.replace(FULL, "semio_s_artifact_stdio_dwg_snapshot_objects::")
    t = re.sub(r"semio_s_artifact_stdio_dwg_snapshot_objects::(?=standards)", "semio_s_artifact_stdio_dwg_snapshot_objects::", t)
    wr(p, t)
print("crates", len(need))
for c, n in need.items():
    p = R + "/" + c
    t = open(p).read()
    pat = re.compile(r"^" + re.escape(OLD) + r" = (\{[^}]*\}|\"[^\"]*\")\n", re.M)
    if not pat.search(t):
        print("NO DEP LINE", c); continue
    def rep(m):
        return "".join(f"{OLD if k == '' else pkg(k)} = {m.group(1)}\n" for k in sorted(n))
    t = pat.sub(rep, t)
    wr(p, t)
    print(c.split("🪆️subsets/✳️any/")[-1], sorted(n))
