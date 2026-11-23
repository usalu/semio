"""📐️ Declared region areas against the model: for every `volumes-thermal-coverage` finding of a measure log (HW-03/measure.ts) the `areaM2` of the region is set to the outer surface its thermal solids have at the finest scale that reports it; writes ops/<slug>.21-areas.json for apply.py. python3 areas.py <slug> <measure.log>"""
import json, os, re, sys

slug, log = sys.argv[1:3]
found = {}
for line in open(log, encoding="utf-8"):
    m = re.search(r'volumes-thermal-coverage volumes\.(\d+)\.thermal: The outer surface of region "([^"]+)" at 1:\d+ is ([\d.]+) m² against areaM2 ([\d.]+)', line)
    if m and (m.group(2) not in found or int(m.group(1)) < found[m.group(2)][0]):
        found[m.group(2)] = (int(m.group(1)), float(m.group(3)), float(m.group(4)))
ops = [{"op": "set", "path": ["thermal", "regions", f"id={region}", "areaM2"], "value": area, "was": was, "measuredAt": scale} for region, (scale, area, was) in found.items()]
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.21-areas.json")
if ops:
    json.dump(ops, open(out, "w", encoding="utf-8"), indent=1)
print(slug, [(o["path"][2][3:], o["was"], o["value"]) for o in ops])
