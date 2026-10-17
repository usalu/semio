#!/usr/bin/env python3
"""🔓️ Removes the own-line test gates (`#[cfg(test)]`, `#[cfg(any(test, feature = "artifact-app-testing"))]`) from the provider units of the facade test hosts (generic test-law functions that other test roots call), so a companion bin that hosts them can export them to its dependents. usage: degate.py [--apply]"""
import re, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import ed
GATE = re.compile(r'(?m)^[ \t]*#\[cfg\((?:test|any\(test, feature = "artifact-app-testing"\))\)\]\n')
H3 = PLUG + "/🪝️mounts/👥️wp3-owners/🧪️test-hosts"
PROVIDER_FILES = {
    H3 + "/🧵️retained-command/🦀️.rs": None,          # whole file
    H3 + "/🪆️child/🦀️.rs": "child_unit_2",            # one inline unit
    H3 + "/🏇️publication-mounted/🦀️.rs": None,
    PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs": None,
    PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⚖️testing-laws/🧱️unit-1/🦀️.rs": None,
}
out = {}
for f, unit in PROVIDER_FILES.items():
    t = read(f)
    if unit is None:
        n = len(GATE.findall(t))
        new = GATE.sub("", t)
    else:
        i = t.index("mod " + unit + " {")
        i0 = t.rfind("\n\n", 0, i) + 2
        head, tail = t[:i0], t[i0:]
        n = len(GATE.findall(tail))
        new = head + GATE.sub("", tail)
    new2 = re.sub(r'(?m)^#\[cfg\(test\)\] (use )', r'\1', new)
    if new2 != new:
        n += 1
        new = new2
    new = new.replace("pub(crate) mod mutation_fixture;", "pub mod mutation_fixture;")
    out[f] = new
    print(rel(f)[-70:], "gates removed:", n)
if "--apply" in sys.argv:
    for f, tx in out.items():
        ed.put(f, tx)
    print("applied")
