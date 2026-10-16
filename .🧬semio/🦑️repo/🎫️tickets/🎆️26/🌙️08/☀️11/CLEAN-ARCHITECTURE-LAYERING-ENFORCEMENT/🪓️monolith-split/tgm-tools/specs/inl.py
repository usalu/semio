import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tgmlib import *


def inline_range(name, mod):
    t = TARGETS[name]
    lines = read(os.path.join(snap_dir(name), t["root"])).split("\n")
    rx = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?mod\s+" + re.escape(mod) + r"\s*\{\s*$")
    starts = [i for i, l in enumerate(lines) if rx.match(l)]
    assert len(starts) == 1, (mod, starts)
    s = starts[0]
    for e in range(s + 1, len(lines)):
        if lines[e] == "}":
            return (s + 1, e + 1)
    raise AssertionError(mod)
