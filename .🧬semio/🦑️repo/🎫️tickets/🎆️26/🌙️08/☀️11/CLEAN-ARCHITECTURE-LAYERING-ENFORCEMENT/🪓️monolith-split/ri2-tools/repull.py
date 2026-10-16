#!/usr/bin/env python3
# 🔁️ Pulls extra planned moves into an already flipped crate and re-roots it: RI2_MOVES=<file> repull.py <crate>...
import sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import mv, flip as F
for suffix in sys.argv[1:]:
    mv.TOUCHED.clear()
    res, srcs = mv.pull(suffix, dry=False)
    st = F.load_state()
    mods = st["modules"].get(suffix) or F.module_list(suffix)
    by_file = {m["file"]: m for m in mods}
    for r in res:
        if r["file"] not in by_file:
            mods.append(r)
            st["pulled"].setdefault(suffix, []).append(r)
            by_file[r["file"]] = r
    for f in mv.TOUCHED:
        m = by_file.get(f)
        if m and not m.get("glob"):
            m["glob"] = True
            m["reexport"] = ["pub"]
            m["reexport_cfg"] = [""]
    st["modules"][suffix] = mods
    F.save_state(st)
    print(suffix, "new modules", [r["name"] for r in res], "touched", len(mv.TOUCHED), "deps", F.reroot(suffix))
