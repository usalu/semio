"""🧾️ Condenses the gate logs of a run (🗑️generated/REP-B/<dir>/<slug>.log as gate.sh writes them) into record/<dir>.json: per person the failing silhouette checks (scale, view, IoU, largest boundary distance in mm), the failing box axes and the `checkVolumes` findings per rule.
python3 summarize.py <dir>"""
import collections
import glob
import json
import os
import re
import sys

K = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
name = sys.argv[1]
out = {}
for path in sorted(glob.glob(f"{K}/🗑️generated/REP-B/{name}/*.log")):
    slug = os.path.basename(path)[:-4]
    sil, box, rules = [], [], collections.Counter()
    for line in open(path, encoding="utf8"):
        m = re.match(r"\S+ 1:(\d+) bbox (.*?)  (elevation .*)$", line)
        if m:
            scale = int(m.group(1))
            for k, part in enumerate(m.group(2).split()):
                if part.endswith("!"):
                    box.append([scale, ["widthM", "heightM", "depthM"][k], part[:-1]])
            for view, iou, h, bad in re.findall(r"(elevation|front|plan) ([\d.]+)/(\d+)mm(!?)", m.group(3)):
                limit = scale * 0.3
                if float(iou) < 0.92 or float(h) > limit:
                    sil.append([scale, view, float(iou), int(h)])
        m = re.match(r"\s+(volumes-[a-z-]+) ", line)
        if m and m.group(1) != "volumes-silhouette":
            rules[m.group(1)] += 1
    out[slug] = {"silhouette": sil, "box": box, "rules": dict(rules)}
os.makedirs(f"{K}/🧰️scripts/REP-B/record", exist_ok=True)
with open(f"{K}/🧰️scripts/REP-B/record/{name}.json", "w", encoding="utf8") as f:
    json.dump(out, f, indent=1)
print(name, len(out), "people;", sum(1 for v in out.values() if v["silhouette"]), "with failing silhouette checks;", sum(len(v["silhouette"]) for v in out.values()), "checks;", sum(sum(v["rules"].values()) for v in out.values()), "volume findings")
