#!/usr/bin/env python3
# 📦️ Embedded fixtures above 40 KB are read at run time from CARGO_MANIFEST_DIR (plan 11.2 rule 3): rewrites `include_str!("<rel>")` of test files whose target is that large.
# usage: runtime_fixtures.py <crate> [--apply]
import os, re, sys
from lib import *
import wp1files
from rs import blank
from crates_wp1 import ROOT

LIMIT = 40 * 1024
RX = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')

crate = sys.argv[1]
apply = "--apply" in sys.argv
root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
pkg = PLUG + "/" + ROOT[crate] + "/" + PKGDIR
prod, test = wp1files.tree_split(root)
for f in prod + test:
    text = read(f)
    clean = blank(text)
    new, last, n = [], 0, 0
    for m in RX.finditer(text):
        if clean[m.start():m.start() + 11] != "include_str":
            continue
        target = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
        if not os.path.exists(target) or os.path.getsize(target) <= LIMIT:
            continue
        rel = os.path.relpath(target, pkg)
        new.append(text[last:m.start()])
        new.append(f'&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/{rel}")).expect("fixture read at run time")')
        last = m.end()
        n += 1
    if n:
        print(f.replace(PLUG + "/", ""), n, "large fixture(s)")
        if apply:
            new.append(text[last:])
            write(f, "".join(new))
