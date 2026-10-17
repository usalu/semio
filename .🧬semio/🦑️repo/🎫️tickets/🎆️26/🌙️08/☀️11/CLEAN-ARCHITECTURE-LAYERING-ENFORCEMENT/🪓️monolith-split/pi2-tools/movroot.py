#!/usr/bin/env python3
"""🚚️ Moves one non-provider test root (its `#[path]` mount) from a companion bin to another one and regenerates both bins' dependency lists and docstrings; the bin closure budget is the reason.
usage: movroot.py <mod> <from NN> <to NN> [--apply]"""
import json, os, re, sys
from lib import *
import ed, tree as T
import binmanifest as BM
import manifest as MF

mod, a, b = sys.argv[1], f"tests-{int(sys.argv[2]):02d}", f"tests-{int(sys.argv[3]):02d}"
crates = json.load(open(TOOLS + "/crates.json"))
PROVIDERS = {"tests-01", "tests-02", "tests-03", "tests-04", "tests-05", "tests-06"}
rootp = lambda k: REPO + "/" + crates[k]["dir"] + "/🦀️.rs"
MOUNT = re.compile(r'(?:#\[cfg\([^\n]*\)\]\n)?#\[path = "[^"]+"\]\n(?:pub )?mod ' + re.escape(mod) + r";\n")
ra, rb = read(rootp(a)), read(rootp(b))
m = MOUNT.search(ra)
assert m and not MOUNT.search(rb), (mod, a, b)
mount = m.group(0)
others = "".join(read(f) for k in crates if k.startswith("tests-") for f, _, _ in T.tree(rootp(k)))
assert not re.search(r"semio_framework_plugin_tests_\d+::" + re.escape(mod) + r"\b", others), "someone imports the root"
na = ra.replace(mount, "")
nb = rb.rstrip("\n") + "\n" + mount


def redoc(text, K):
    mods = re.findall(r"pub mod ([A-Za-z0-9_]+);", text)
    return re.sub(r"hosts the test roots [^\n]*?\.\n", "hosts the test roots " + ", ".join("`" + x + "`" for x in mods) + ".\n", text, count=1)


na, nb = redoc(na, a), redoc(nb, b)
print("moves", mod, "from", a, "to", b)
if "--apply" in sys.argv:
    ed.put(rootp(a), na)
    ed.put(rootp(b), nb)
    for K in (a, b):
        texts = [read(rootp(K))] + [read(p) for p, parent, name in T.tree(rootp(K)) if parent is not None]
        man, rep = BM.plan(K, texts, provider=K in PROVIDERS)
        ed.put(MF.pkg_dir(K) + "/Cargo.toml", man)
        print(K, "manifest", rep["features"], rep["missing"])
    print("applied")
