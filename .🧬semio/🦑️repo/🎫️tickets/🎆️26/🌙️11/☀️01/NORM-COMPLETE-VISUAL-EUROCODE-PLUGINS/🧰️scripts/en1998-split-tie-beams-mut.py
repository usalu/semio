#!/usr/bin/env python3
"""🪓 Splits the en1998 `mut-tie-beams` crate (46 leaves, 139 files) into entity crates under the 32-file source boundary and rewires every consumer."""
import os, re
R = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
M = R + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
TB = M + "/🔗️tie-beams"
lib = open(TB + "/🦀️.rs", encoding="utf-8").read()
mounts = re.findall(r'(#\[path = "\.\./([^/]+)/🦀️\.rs"\]\npub mod (\w+);\n)', lib)
def group(mod):
    if "tie_beam" in mod: return "tie-beams"
    if "isolation_system" in mod: return "isolation-systems"
    if "pier_base" in mod: return "pier-bases"
    return "critical-regions"
crit = [m for m in mounts if group(m[2]) == "critical-regions"]
split = {"critical-regions": crit[:8], "critical-regions-more": crit[8:16], "critical-regions-rest": crit[16:]}
groups = {"tie-beams": [m for m in mounts if group(m[2]) == "tie-beams"], "isolation-systems": [m for m in mounts if group(m[2]) == "isolation-systems"], "pier-bases": [m for m in mounts if group(m[2]) == "pier-bases"]}
groups.update(split)
desc = {"tie-beams": "EN 1998 tie beam mutation leaves: span, section, steel and relative displacement", "isolation-systems": "EN 1998 isolation system mutation leaves", "pier-bases": "EN 1998 pier base mutation leaves", "critical-regions": "EN 1998 critical region mutation leaves (part 1)", "critical-regions-more": "EN 1998 critical region mutation leaves (part 2)", "critical-regions-rest": "EN 1998 critical region mutation leaves (part 3)"}
emoji = "🔗️"
head = lib.split("//#region 🪢️Mounts")[0]
tail = "//#endregion 🪢️Mounts" + lib.split("//#endregion 🪢️Mounts")[1]
cargo = open(TB + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf-8").read()
leaf2crate = {}
for g, ms in groups.items():
    for _, _, mod in ms: leaf2crate[mod] = g
for g, ms in groups.items():
    d = M + "/" + emoji + g
    os.makedirs(d + "/📦️packages/🦀️rust", exist_ok=True)
    h = head.replace(desc["tie-beams"], desc[g])
    open(d + "/🦀️.rs", "w", encoding="utf-8").write(h + "//#region 🪢️Mounts\n" + "".join(m[0] for m in ms) + tail)
    c = cargo.replace("mut-tie-beams", "mut-" + g).replace(desc["tie-beams"], desc[g])
    open(d + "/📦️packages/🦀️rust/Cargo.toml", "w", encoding="utf-8").write(c)
names = list(groups)
def crate(g): return "semio-s-artifact-norm-en1998-mut-" + g
def ident(g): return crate(g).replace("-", "_")
def sub_uses(t):
    return re.sub(r'semio_s_artifact_norm_en1998_mut_tie_beams::(\w+)', lambda m: ident(leaf2crate[m.group(1)]) + "::" + m.group(1), t)
for p in [M + "/🦀️.rs", R + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🔗️tie-beams/🦀️.rs"]:
    t = open(p, encoding="utf-8").read(); open(p, "w", encoding="utf-8").write(sub_uses(t))
def dep_lines(line, only):
    return "".join(line.replace("mut-tie-beams", "mut-" + g) + ("" if line.endswith("\n") else "\n") for g in only).rstrip("\n") + ("\n" if line.endswith("\n") else "")
for p, only in [(M + "/📦️packages/🦀️rust/Cargo.toml", names), (R + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🔗️tie-beams/📦️packages/🦀️rust/Cargo.toml", names)]:
    t = open(p, encoding="utf-8").read()
    out = []
    for line in t.split("\n"):
        if line.startswith("semio-s-artifact-norm-en1998-mut-tie-beams = "):
            out += [line.replace("mut-tie-beams", "mut-" + g) for g in only]
        elif line.startswith("test-support = ") and "mut-tie-beams/test-support" in line:
            line = line.replace('"semio-s-artifact-norm-en1998-mut-tie-beams/test-support"', ", ".join('"%s/test-support"' % crate(g) for g in only)); out.append(line)
        else: out.append(line)
    open(p, "w", encoding="utf-8").write("\n".join(out))
p = R + "/Cargo.toml"; t = open(p, encoding="utf-8").read()
mem = '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️tie-beams/📦️packages/🦀️rust",\n'
t = t.replace(mem, "".join(mem.replace("🔗️tie-beams", emoji + g) for g in names), 1)
wd = '[workspace.dependencies.semio-s-artifact-norm-en1998-mut-tie-beams]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️tie-beams/📦️packages/🦀️rust"\n'
assert wd in t
t = t.replace(wd, "\n".join(wd.replace("mut-tie-beams", "mut-" + g).replace("🔗️tie-beams", emoji + g) for g in names))
open(p, "w", encoding="utf-8").write(t)
p = R + "/📦️packages/🦀️rust/📜️script.ts"; t = open(p, encoding="utf-8").read()
t = t.replace('"semio-s-artifact-norm-en1998-mut-tie-beams"', ",".join('"%s"' % crate(g) for g in names), 1); open(p, "w", encoding="utf-8").write(t)
