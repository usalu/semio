"""🪚 Crate-budget D helpers: create a sibling crate whose lib is an existing module file, register it everywhere."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *


def new_crate(art, crate_dir, name, desc, deps, parent_name, parent_toml, dev=""):
    pkg = crate_dir + "/📦️packages/🦀️rust"
    deps = sorted(set(deps))
    t = '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % (relp(art, pkg), name, desc)
    t += "".join('%s = { workspace = true }\n' % d for d in deps)
    if dev:
        t += "\n[dev-dependencies]\n" + dev
    wr(pkg + "/Cargo.toml", t)
    register_workspace(art, name, pkg)
    script_register(art, parent_name, name)
    lock_register(parent_name, name, new_deps=[d for d in deps])
    return pkg
