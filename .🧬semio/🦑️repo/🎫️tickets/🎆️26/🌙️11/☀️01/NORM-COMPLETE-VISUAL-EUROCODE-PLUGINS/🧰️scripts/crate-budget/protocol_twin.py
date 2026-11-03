"""📡️ Moves the binary wire-protocol text of a mutations crate into a twin crate (`LEAF`), mounted by the codec through a const alias. Usage: protocol_twin.py ARTIFACT-DIR."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *


def run(art):
    root = NORM + "/🗿️artifacts/" + art
    any_ = root + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    bdir = any_ + "/🚪️io/💾️binary/🧬️mutations"
    toml = any_ + "/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml"
    parent = re.search(r'name = "([^"]+)"', rd(toml)).group(1)
    f = bdir + "/🦀️.rs"
    s = rd(f)
    inc = 'include_str!("📡️.protocol.semio")'
    if inc not in s:
        print("skip", parent)
        return
    cname = parent + "-protocol"
    short = parent.split("norm-")[1].split("-")[0].upper().replace("EN", "EN ")
    d = bdir + "/📡️protocol-twin"
    pkg = d + "/📦️packages/🦀️rust"
    wr(d + "/🦀️.rs", '//! 📡️ %s mutations binary wire-protocol twin leaf, kept next to the codec that reads it.\n\n/// 📡️ The wire protocol of the mutation records.\npub const LEAF: &str = include_str!("../📡️.protocol.semio");\n' % short)
    wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s mutations binary protocol twin"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n' % (relp(root, pkg), cname, short))
    register_workspace(root, cname, pkg)
    add_dep(toml, cname)
    script_register(root, parent, cname)
    lock_register(parent, cname, new_deps=[])
    wr(f, s.replace(inc, cname.replace("-", "_") + "::LEAF"))
    print("protocol twin", parent)


run(sys.argv[1])
