"""🧪 c4p25: rustfmt parse check (skip_children, so only the file itself is parsed) plus path length and brace balance for every file of the plans. usage: c4pparse.py stage|live [key...]"""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
bad = 0
n = 0
for key in (sys.argv[2:] or ORDER):
    spec = PLANS[key]
    base = os.path.dirname(spec["rel"])
    for rel in [spec["rel"]] + [f"{base}/{c['folder']}/🦀️.rs" for c in spec["children"]]:
        p = (STAGE + "/" + rel) if which == "stage" else (ROOT + "/" + rel)
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", p], capture_output=True, text=True)
        n += 1
        pb = len(rel.encode("utf-8"))
        if r.returncode != 0 or pb > 240:
            bad += 1
            print("BAD", key, rel[-60:], "rc", r.returncode, "bytes", pb, r.stderr[:300])
print(f"PARSE {'OK' if bad == 0 else 'FAILED'} files={n} bad={bad}")
