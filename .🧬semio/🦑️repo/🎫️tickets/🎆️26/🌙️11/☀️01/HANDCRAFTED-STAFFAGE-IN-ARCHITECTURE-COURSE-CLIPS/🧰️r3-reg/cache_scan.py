"""🗃️ Read-only scan of staffage cache key folders; classifies each key folder as complete or incomplete."""
import json
import os
from collections import Counter

BASE = "C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/staffage"
rows = []
for key in sorted(os.listdir(BASE)):
    kp = os.path.join(BASE, key)
    if not os.path.isdir(kp):
        continue
    subs = [s for s in os.listdir(kp) if os.path.isdir(os.path.join(kp, s))]
    top = [f for f in os.listdir(kp) if not os.path.isdir(os.path.join(kp, f))]
    empty_subs, tmp, locks, svg_total = [], [], [], 0
    for s in subs:
        sp = os.path.join(kp, s)
        entries = os.listdir(sp)
        svgs = [e for e in entries if e.endswith(".svg")]
        svg_total += len(svgs)
        if not svgs:
            empty_subs.append(s)
        tmp += [e for e in entries if e.startswith(".tmp")]
    locks = [f for f in top if f.startswith(".lock-")]
    tmp += [f for f in top if f.startswith(".tmp")]
    reasons = []
    if not subs:
        reasons.append("no-cast-folder")
    if empty_subs:
        reasons.append("empty-cast-folder:" + ",".join(empty_subs))
    if tmp:
        reasons.append("tmp-leftovers:%d" % len(tmp))
    rows.append({"key": key, "casts": sorted(subs), "svgs": svg_total, "locks": sorted(locks),
                 "incomplete": bool(reasons), "reasons": reasons})

incomplete = [r for r in rows if r["incomplete"]]
out = {"key_folders": len(rows), "incomplete": len(incomplete), "rows_incomplete": incomplete,
       "lock_files_in_complete": sum(len(r["locks"]) for r in rows if not r["incomplete"]),
       "cast_folder_counts": dict(Counter(len(r["casts"]) for r in rows))}
print(json.dumps(out, ensure_ascii=False, indent=1))
