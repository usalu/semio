"""✂️ Splits the en1999 `collection-rows` crate (43 files, over the 32-file cap) into `collection-rows` (material, section, connection, shell) and `scenario-rows` (fire scenario, fatigue detail, cold-formed sheet). Idempotent."""
import os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "leaf-kinds"))
from common import *

ROOT = NORM + "/🗿️artifacts/🪶️en1999"
A = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = A + "/🧬️schema/🧬️mutations"
MOVED = ["fire_scenario", "fatigue_detail", "cold_formed_sheet"]
MODS = [v + "_" + n for n in MOVED for v in ("insert", "remove")]
OLD, NEW = "semio_s_artifact_norm_en1999_mut_collection_rows", "semio_s_artifact_norm_en1999_mut_scenario_rows"
OLDC, NEWC = "semio-s-artifact-norm-en1999-mut-collection-rows", "semio-s-artifact-norm-en1999-mut-scenario-rows"


def w(p, t):
    open(p, "w", encoding="utf-8").write(t)


def main():
    src = M + "/🗂️collection-rows/🦀️.rs"
    s = rd(src)
    blocks = re.findall(r'#\[path = "[^"]+"\]\npub mod (\w+);\n', s)
    if not any(m in blocks for m in MODS):
        print("nothing to do")
        return
    moved = "".join(b for b in re.findall(r'(#\[path = "[^"]+"\]\npub mod \w+;\n)', s) if re.search(r"pub mod (%s);" % "|".join(MODS), b))
    kept = s
    for b in re.findall(r'(#\[path = "[^"]+"\]\npub mod \w+;\n)', s):
        if re.search(r"pub mod (%s);" % "|".join(MODS), b):
            kept = kept.replace(b, "")
    kept = re.sub(r"^//! .*\n", "//! 🗂️ EN 1999 collection mutation leaves: insert and remove material, section, connection and shell.\n", kept, 1)
    head = s[: s.index("//#region 🪢️Mounts")]
    head = re.sub(r"^//! .*\n", "//! 🗂️ EN 1999 scenario mutation leaves: insert and remove fire scenario, fatigue detail and cold-formed sheet.\n", head, 1)
    d = M + "/🗂️scenario-rows"
    os.makedirs(d + "/📦️packages/🦀️rust", exist_ok=True)
    w(d + "/🦀️.rs", head + "//#region 🪢️Mounts\n" + moved + "//#endregion 🪢️Mounts\n")
    w(src, kept)
    t = rd(M + "/🗂️collection-rows/📦️packages/🦀️rust/Cargo.toml")
    w(d + "/📦️packages/🦀️rust/Cargo.toml", t.replace(OLDC, NEWC).replace("mutation leaves: collection-rows", "mutation leaves: scenario-rows"))
    rel = os.path.relpath(d + "/📦️packages/🦀️rust", ROOT)
    c = rd(ROOT + "/Cargo.toml")
    c = c.replace('members = [\n', 'members = [\n    "%s",\n' % rel, 1)
    c = c.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (NEWC, rel), 1)
    w(ROOT + "/Cargo.toml", c)
    sp = ROOT + "/📦️packages/🦀️rust/📜️script.ts"
    t = rd(sp)
    w(sp, t.replace('"%s",' % OLDC, '"%s","%s",' % (OLDC, NEWC), 1))
    for pkg in [M + "/📦️packages/🦀️rust/Cargo.toml", A + "/🚪️io/📦️packages/🦀️rust/Cargo.toml", A + "/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"]:
        t = rd(pkg)
        line = "%s = { workspace = true }\n" % OLDC
        w(pkg, t.replace(line, line + "%s = { workspace = true }\n" % NEWC, 1))
    p = M + "/🦀️.rs"
    t = rd(p)
    for m in MODS:
        t = t.replace("use %s::%s;\n" % (OLD, m), "use %s::%s;\n" % (NEW, m))
    w(p, t)
    p = A + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    t = rd(p)
    m = re.search(r"use %s::\{([^}]*)\};\n" % OLD, t)
    names = [x.strip() for x in m.group(1).split(",")]
    a = [x for x in names if x not in MODS]
    b = [x for x in names if x in MODS]
    w(p, t[: m.start()] + "use %s::{%s};\nuse %s::{%s};\n" % (OLD, ", ".join(a), NEW, ", ".join(b)) + t[m.end():])
    print("split", len(MODS), "kinds")


main()
