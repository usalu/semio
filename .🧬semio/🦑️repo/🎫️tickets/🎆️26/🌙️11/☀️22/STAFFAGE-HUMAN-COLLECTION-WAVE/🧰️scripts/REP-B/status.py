"""🚦️ Reads the last line of every greedy repair log (🗑️generated/REP-B/auto/<slug>-<scale>.log) and prints per person which scales and views still miss the guard (IoU below 0.92 or a largest boundary distance above two smallest features, 0.3 mm on paper), and the list of person and scale pairs to run again.
python3 status.py"""
import glob
import os
import re
import sys

G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/REP-B/" + (sys.argv[1] if len(sys.argv) > 1 else "auto")
people = {}
for path in sorted(glob.glob(f"{G}/*.log")):
    slug, scale = os.path.basename(path)[:-4].rsplit("-", 1)
    lines = open(path, encoding="utf8").read().strip().split("\n")
    last = lines[-1] if lines else ""
    views = re.findall(r"([efp]):([\d.]+)/([\d.]+)mm", last) if "final" in last else None
    issues = next((l for l in reversed(lines) if "] issues " in l), "")
    m = re.search(r"issues (\d+) (?:\(base|new \(inherited) (\d+)\)", issues)
    people.setdefault(slug, {})[int(scale)] = (views, m.groups() if m else None)
again, green = [], 0
for slug, scales in people.items():
    bad = []
    for scale in sorted(scales):
        views, issues = scales[scale]
        if views is None:
            bad.append(f"1:{scale} unfinished")
            again.append(f"{slug}-{scale}")
            continue
        miss = [f"{v}:{iou}/{h}" for v, iou, h in views if float(iou) < 0.92 or float(h) > scale * 0.3 + 1e-9]
        if miss:
            bad.append(f"1:{scale} " + " ".join(miss))
            again.append(f"{slug}-{scale}")
        if issues and issues[0] != "0":
            bad.append(f"1:{scale} validator {issues[0]} (base {issues[1]})")
    green += not bad
    print(f"{slug}: {'green' if not bad else '; '.join(bad)}")
print(f"{green} of {len(people)} people green in the repair logs; again: {' '.join(again)}")
