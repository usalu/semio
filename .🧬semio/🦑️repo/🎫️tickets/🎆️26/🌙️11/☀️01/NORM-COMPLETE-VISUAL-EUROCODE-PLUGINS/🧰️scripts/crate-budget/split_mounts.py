"""🪚 Moves whole kind mounts of a mutation leaf crate into a new sibling crate; the old crate re-exports them (`pub use new::{mods}`)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

ARTS = NORM + "/🗿️artifacts/"
MOUNT = re.compile(r'#\[path = "\.\./([^"]+)/🦀️\.rs"\]\npub mod (\w+);\n')


def split(art, old_dir, new_dir, slug, mods, doc):
    art_root = ARTS + art
    M = art_root + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
    root_f = M + "/" + old_dir + "/🦀️.rs"
    root = rd(root_f)
    ms = MOUNT.findall(root)
    have = {m for _, m in ms}
    assert set(mods) <= have, set(mods) - have
    old_toml = M + "/" + old_dir + "/📦️packages/🦀️rust/Cargo.toml"
    old = re.search(r'name = "([^"]+)"', rd(old_toml)).group(1)
    new = old.split("-mut-")[0] + "-mut-" + slug
    if os.path.exists(M + "/" + new_dir + "/🦀️.rs"):
        print("exists", new)
        return
    pre = root[root.index("#![allow"): root.index("//#region 🪢️Mounts")]
    mv = [(d, m) for d, m in ms if m in mods]
    body = "//! %s\n\n" % doc + pre + "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in mv) + "//#endregion 🪢️Mounts\n"
    pkg = M + "/" + new_dir + "/📦️packages/🦀️rust"
    desc = re.search(r'description = "([^"]+)"', rd(old_toml)).group(1) + " (" + slug.replace("-", " ") + ")"
    drop = tuple(re.findall(r"^(semio-s-artifact-norm-[\w-]+-mut-[\w-]+) =", rd(old_toml), re.M))
    wr(pkg + "/Cargo.toml", manifest(new, art_root, pkg, desc, old_toml, drop))
    wr(M + "/" + new_dir + "/🦀️.rs", body)
    for d, m in mv:
        root = root.replace('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (d, m), "")
    root = root.rstrip("\n") + "\n\npub use %s::{%s};\n" % (new.replace("-", "_"), ", ".join(m for _, m in mv))
    wr(root_f, root)
    register_workspace(art_root, new, pkg)
    add_dep(old_toml, new)
    script_register(art_root, old, new)
    lock_register(old, new, new_deps=toml_deps(rd(pkg + "/Cargo.toml")))
    print("split", old, "->", new, len(mv))


if __name__ == "__main__":
    split(*sys.argv[1:5], sys.argv[5].split(","), sys.argv[6])
