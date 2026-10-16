#!/usr/bin/env python3
# 🔗️ First-party renderer crates named in a carved crate's code (including tests) against its manifest (dev-dependencies counted separately); --apply adds the missing path dependencies.
import json, os, re, sys
from common import *
import modtree
from rs import blank

carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
ALL = {PKG_PREFIX + s: s for s in list(CRATES)}
apply = "--apply" in sys.argv
rc = 0
for c in carved:
    mp = T + "/" + CRATES[c] + "/📦️packages/🦀️rust/Cargo.toml"
    mt = read(mp)
    files = modtree.tree(T + "/" + CRATES[c] + "/🦀️.rs")
    code = "\n".join(blank(read(f)) for f in files if "/🧪️tests/" not in f)
    tests = "\n".join(blank(read(f)) for f in files if "/🧪️tests/" in f)
    declared = set(re.findall(r"(?m)^(" + re.escape(PKG_PREFIX) + r"[a-z0-9\-]+) *=", mt))
    named = {k for k in ALL if lib_name(ALL[k]) in code}
    named_tests = {k for k in ALL if lib_name(ALL[k]) in tests} - named
    missing = sorted(named - declared - {PKG_PREFIX + c})
    missing_dev = sorted(named_tests - declared - {PKG_PREFIX + c})
    unused = sorted(d for d in declared if d in ALL and lib_name(ALL[d]) not in code and lib_name(ALL[d]) not in tests)
    if missing or missing_dev:
        rc = 1
    print(c, "missing", [m[len(PKG_PREFIX):] for m in missing], "missing(dev)", [m[len(PKG_PREFIX):] for m in missing_dev], "declared-unused", [u[len(PKG_PREFIX):] for u in unused])
    if apply and (missing or missing_dev):
        add = ""
        for m in missing:
            add += f'{m} = {{ path = "../../../{CRATES[ALL[m]]}/📦️packages/🦀️rust" }}\n'
        dev = "".join(f'{m} = {{ path = "../../../{CRATES[ALL[m]]}/📦️packages/🦀️rust" }}\n' for m in missing_dev)
        if add:
            mt = mt.replace("[dependencies]\n", "[dependencies]\n" + add, 1)
        if dev:
            mt = mt.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + dev, 1) if "[dev-dependencies]\n" in mt else mt.rstrip("\n") + "\n\n[dev-dependencies]\n" + dev
        write(mp, mt)
sys.exit(rc)
