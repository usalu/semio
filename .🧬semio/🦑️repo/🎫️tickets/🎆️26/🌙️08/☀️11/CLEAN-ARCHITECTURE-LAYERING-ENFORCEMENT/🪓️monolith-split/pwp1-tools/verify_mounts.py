#!/usr/bin/env python3
# 🧾️ Mount equivalence of the WP1 units: every module name the pre-WP1 hub 1 / root knew is still defined by the live hub / root under the same name, cfg attributes and (at least) the same visibility.
import re, sys
from lib import *
import modmap
from rs import blank

snap = {k: v for k, v in modmap.snapshot_mounts().items() if v[3] in ("🌱️wp1-foundation",) or k in ("wasip2", "window_transient")}
live_text = read(PLUG + "/🪝️mounts/🌱️wp1-foundation/🦀️.rs") + "\n" + read(PLUG + "/🦀️.rs")
clean = blank(live_text)
VIS_RANK = {"": 0, "pub(super)": 1, "pub(crate)": 2, "pub": 3}
rows, bad = [], 0
for name, (f, vis, attrs, hub) in sorted(snap.items()):
    pats = [
        r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?mod\s+" + re.escape(name) + r"\s*;",
        r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+[A-Za-z0-9_]+\s+as\s+" + re.escape(name) + r"\s*;",
        r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?use\s+[A-Za-z0-9_:]+::" + re.escape(name) + r"\s*;",
        r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?use\s+[A-Za-z0-9_:]+\s+as\s+" + re.escape(name) + r"\s*;",
    ]
    found = None
    for p in pats:
        m = re.search(p, clean)
        if m:
            found = (live_text[m.start(1):m.end(1)], (m.group(2) or "").strip())
            break
    if not found:
        rows.append(f"MISSING {name}")
        bad += 1
        continue
    old_cfg = sorted(re.findall(r"cfg\(.*?\)\s*\]", attrs.replace("\n", " ")))
    new_cfg = sorted(re.findall(r"cfg\(.*?\)\s*\]", found[0].replace("\n", " ")))
    vis_old = vis.strip()
    ok_cfg = old_cfg == new_cfg
    ok_vis = VIS_RANK.get(found[1], 0) >= VIS_RANK.get(vis_old, 0)
    state = "ok" if ok_cfg and ok_vis else "DIFF"
    if state != "ok":
        bad += 1
    rows.append(f"{state} {name} cfg {old_cfg} -> {new_cfg} vis '{vis_old}' -> '{found[1]}'")
print("\n".join(rows))
print("units", len(snap), "problems", bad)
