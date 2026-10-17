"""🔭 c4p25: prints every planned range with the first and last two lines (read only). usage: showplan.py [key...]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *
from plans import PLANS, ORDER
for key in (sys.argv[1:] or ORDER):
    ls = lines_of(read(f"{ORIG}/{key}.rs"))
    print(f"##### {key} ({len(ls)} lines)")
    for c in PLANS[key]["children"]:
        for r in c["ranges"]:
            if isinstance(r, list):
                print("  literal", r)
                continue
            a, b = r[0], r[1]
            print(f"  [{c['folder']}] {a}-{b} ({b - a + 1})  extra={ {k: v for k, v in c.items() if k in ('pre', 'post', 'macro_use')} }")
            for i in (a - 1, a, b - 2, b - 1):
                print(f"      {i + 1:5}: {ls[i][:110]}")
