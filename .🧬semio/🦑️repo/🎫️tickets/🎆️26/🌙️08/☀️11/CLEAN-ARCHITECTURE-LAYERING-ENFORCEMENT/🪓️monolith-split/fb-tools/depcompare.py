"""🔎 FB: compares the crate heads a crate's source closure names with the dependencies its manifest declares (normal / dev), lib and test closures separately."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from depscan import closure, heads, STD, ROOT

E = f"{ROOT}/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine"
CRATES = {
    "engine": ("📦️packages/🦀️rust", "🦀️.rs"),
    "kernel-contract": ("🧬️kernel-contract/📦️packages/🦀️rust", "🧬️kernel-contract/🦀️.rs"),
    "mesh-io": ("📦️mesh-io/📦️packages/🦀️rust", "📦️mesh-io/🦀️.rs"),
    "shape-value": ("🪅️shape-value/📦️packages/🦀️rust", "🪅️shape-value/🦀️.rs"),
    "tests": ("🧪️tests/📦️packages/🦀️rust", "🧪️tests/🦀️.rs"),
}
KNOWN_EXT = {"parry3d", "serde_json", "serde", "bytemuck", "num_rational"}


def deps_of(manifest):
    t = Bun_like(manifest)
    return t


def Bun_like(path):
    sect, normal, dev = None, set(), set()
    for line in open(path, encoding="utf-8"):
        s = line.strip()
        if s.startswith("["):
            sect = s
            continue
        if sect in ("[dependencies]", "[dev-dependencies]") and "=" in s and not s.startswith("#"):
            name = s.split("=")[0].strip().split(".")[0].strip()
            (normal if sect == "[dependencies]" else dev).add(name.replace("-", "_"))
    return normal, dev


for key, (pkg, lib) in CRATES.items():
    normal, dev = deps_of(f"{E}/{pkg}/Cargo.toml")
    files = closure(f"{E}/{lib}")
    h, local = heads(files)
    named = {k for k in h if (k.startswith("semio_") or k in KNOWN_EXT) and k not in local}
    extra_cfg_test = {}
    print(f"== {key}: {len(files)} files")
    print("   named but undeclared :", sorted(named - normal - dev))
    print("   declared normal unused:", sorted(normal - named))
    print("   declared dev unused   :", sorted(dev - named))
    print("   named only via dev    :", sorted((named & dev) - normal))
