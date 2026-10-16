"""Definition scan on staged/installed files: every private top-level item, field or inherent method whose name is used by another file must already carry a visibility (dry run of the widening planner = 0 further edits)."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
import split

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
total = 0
for name, s in SPEC.items():
    t = TARGETS[name]
    base = f"{W}/{name}/stage/{t['pkgdir']}" if which == "stage" else f"{ROOT}/{t['pkgdir']}"
    texts = {"<root>": read(f"{base}/🦀️.rs")}
    for c in s["children"]:
        texts[c["folder"]] = read(f"{base}/{c['folder']}/🦀️.rs")
    tests = f"{ROOT}/{t['tree']}/🧪️tests/🔬️unit/🦀️.rs"
    if os.path.exists(tests):
        texts["<tests>"] = read(tests)
    ids = {k: split.file_ids(v) for k, v in texts.items()}
    n = 0
    for c in s["children"]:
        f = c["folder"]
        others = [v for k, v in ids.items() if k != f]
        edits = split.plan_widen(texts[f], others)
        for e in edits:
            print("  PENDING", name, f, e[2])
        n += len(edits)
    total += n
    print(f"{name}: further widenings proposed = {n}")
print("TOTAL pending", total)
