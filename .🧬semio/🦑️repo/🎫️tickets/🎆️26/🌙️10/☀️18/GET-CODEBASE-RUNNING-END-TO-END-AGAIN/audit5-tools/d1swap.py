"""🔀 [audit5] Puts the pre-D1 bytes (backups land.py took at the G42 apply, first batch per file) into a MIRROR (`off`), or the current live bytes back (`on`); new D1 files stay (old code never imports them). usage: python3 -I d1swap.py <mirror-root> off|on"""
import json, os, shutil, sys
root, verb = os.path.abspath(sys.argv[1]), sys.argv[2]
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__))); LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
assert root != LIVE and "/sm/" in root, "mirror root required"
first = {}
for b in ["g-walk-walks", "g-gram2-row-table", "g-gram3-index-chunks", "g-gram3-churn", "g-gram4-shape-keys"]:
    d = f"{C}/backlog-tools/backup/{b}@live"
    for rel, v in json.load(open(d + "/manifest.json", encoding="utf8")).items():
        if v["file"] is not None and rel not in first: first[rel] = d + "/" + v["file"]
n = 0
for rel, backup in sorted(first.items()):
    target = f"{root}/{rel}"
    if not os.path.exists(os.path.dirname(target)): continue
    shutil.copyfile(backup if verb == "off" else f"{LIVE}/{rel}", target); n += 1
print(f"{verb}: {n} of {len(first)} files")
