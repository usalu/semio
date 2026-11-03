"""🪚 Crate-budget H helpers: move whole child modules of a crate root into a new sibling crate (files stay in place), `pub(super)` becomes `pub`, the old root imports the new crate."""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *


def retarget_visibility(files):
    for f in files:
        t = rd(f)
        n = re.sub(r"pub\(super\)", "pub", t)
        if n != t:
            wr(f, n)



MOUNT = re.compile(r'((?:#\[cfg\([^\]]*\)\]\n)?)#\[path = "([^"]+)/🦀️\.rs"\]\n((?:pub )?mod \w+;\n(?:pub use [\w:{}, ]+;\n)?)\n?')


def lower_core(art, host_dir, parent, suffix, keep, desc, parent_toml):
    """🪚 The crate root `host_dir/🦀️.rs` moves wholesale into the lower crate `parent-suffix` (root `host_dir/🧱️<suffix>`, module files stay in place); the host keeps the mounts of `keep` and every `cfg` mount (tests), re-exports the lower crate and imports its modules."""
    new = parent + "-" + suffix
    root_f = host_dir + "/🦀️.rs"
    root = rd(root_f)
    if new.replace("-", "_") in root:
        return
    mounts = list(MOUNT.finditer(root))
    kept = [m for m in mounts if m.group(2) in keep or m.group(1)]
    low_mounts = [m for m in mounts if m not in kept]
    files = [f for m in low_mounts for f in glob.glob(host_dir + "/" + m.group(2) + "/**/*.rs", recursive=True)]
    for f in files:
        t = rd(f)
        n = t.replace("pub(super)", "pub").replace("pub(crate)", "pub")
        if n != t:
            wr(f, n)
    low = root
    for m in kept:
        low = low.replace(m.group(0), "", 1)
    low = re.sub(r'#\[path = "', '#[path = "../', low)
    low = re.sub(r"(?m)^mod (\w+);", r"pub mod \1;", low)
    low = re.sub(r"(?m)^use (?!super)", "pub use ", low)
    low = re.sub(r"(?m)^use (\w+)::\*;", r"pub use \1::*;", low)
    wr(host_dir + "/🧱️" + suffix + "/🦀️.rs", low)
    pubmods = [re.search(r"mod (\w+);", m.group(3)).group(1) for m in low_mounts]
    externs = "".join(l + "\n" for l in root.split("\n") if l.startswith("extern crate") or l.startswith("#![allow"))
    doc = root.split("\n")[0]
    top = doc + "\n\n" + externs + "\npub use %s::*;\nuse %s::{%s};\n\n" % (new.replace("-", "_"), new.replace("-", "_"), ", ".join(pubmods)) + "".join(m.group(0) for m in kept)
    wr(root_f, top.rstrip("\n") + "\n")
    t = rd(parent_toml)
    dev = t.split("[dev-dependencies]\n")[1].split("\n[")[0] + "\n" if "[dev-dependencies]" in t else ""
    deps = re.findall(r"(?m)^(semio-[\w-]+) = ", t.split("[dependencies]\n")[1].split("\n[")[0])
    new_crate(art, host_dir + "/🧱️" + suffix, new, desc, deps, parent, parent_toml, dev=dev)
    add_dep(parent_toml, new)
