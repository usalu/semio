"""🪚 Crate-budget I: mechanics fastening and geotechnics become the crate `-mechanics-applied` (root `🧱️applied`); the kernel re-exports both modules."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
M = NORM + "/🧮️mechanics"
P = "semio-s-plugin-norm-mechanics"
N = P + "-applied"
MODS = [("🔩️fastening", "fastening"), ("🌍️geotechnics", "geotechnics")]
root = rd(M + "/🦀️.rs")
if N.replace("-", "_") in root:
    sys.exit("done")
import glob
for d, _ in MODS:
    for f in glob.glob(M + "/" + d + "/**/*.rs", recursive=True):
        t = rd(f)
        n = re.sub(r"pub\((?:crate|super)\)", "pub", t)
        if n != t:
            wr(f, n)
    pat = '#[path = "%s/🦀️.rs"]\n' % d
    assert pat in root, d
    root = re.sub(re.escape(pat) + r"pub mod \w+;\n\n?", "", root)
root = root.replace("#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit", "pub use %s::{fastening, geotechnics};\n\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit" % N.replace("-", "_"), 1)
root = re.sub(r"\n{3,}", "\n\n", root)
wr(M + "/🦀️.rs", root)
wr(M + "/🧱️applied/🦀️.rs", "//! 🔩️ Mechanics applied kernels: fastenings and geotechnics (earth pressure, slope stability). See [`super`].\n\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % x for x in MODS))
pkg = M + "/🧱️applied/📦️packages/🦀️rust"
wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "Domain-neutral structural mechanics applied kernels: fastenings and geotechnics"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n' % (relp(NORM, pkg), N))
register_workspace(NORM, N, pkg)
add_dep(M + "/📦️packages/🦀️rust/Cargo.toml", N)
lock_register(P, N, new_deps=[])
