"""✏️ RI3 anchored editor: applies (file, old, new, expected-count) replacements with a pre-edit snapshot in the scratchpad, asserting every anchor count."""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common_ri3 import *


def apply(spec, dry=False):
    by = {}
    for f, old, new, n in spec:
        by.setdefault(f, []).append((old, new, n))
    for f, items in by.items():
        text = open(f, encoding="utf8").read()
        orig = text
        for old, new, n in items:
            c = text.count(old)
            assert c == n, f"{f[-70:]}: anchor {old[:60]!r} count {c} != {n}"
            text = text.replace(old, new)
        if dry:
            print("dry ok", f[-70:])
            continue
        snap = PRE + "/" + os.path.relpath(f, REPO)
        if not os.path.exists(snap):
            os.makedirs(os.path.dirname(snap), exist_ok=True)
            shutil.copy2(f, snap)
        assert open(f, encoding="utf8").read() == orig, "file changed while editing: " + f
        open(f, "w", encoding="utf8").write(text)
        print("edited", f[-70:])
