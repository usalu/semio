"""📏 Measures the outer area of every thermal region of the 1:50 model with the product's own rule (volumes-thermal-coverage) and writes `author-areas.json`, which `author-slide-tower.py` reads as `areaM2`.

It writes a probe document whose regions all declare 1000 m2, so the rule reports the measured outer surface of each region, then rebuilds the candidate.

@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-validate.ts — the validator that measures
"""
import glob
import json
import os
import re
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.normpath(os.path.join(HERE, "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
PROBE_DIR = os.path.join(HERE, "🗑️generated", "used-slide-tower")
AREAS = os.path.join(HERE, "author-areas.json")


def main():
    """🧪 Probe, parse, write."""
    if os.path.exists(AREAS):
        os.remove(AREAS)
    subprocess.run(["python3", os.path.join(HERE, "author-slide-tower.py")], check=True, capture_output=True)
    doc = json.load(open(os.path.join(HERE, "🔣️.json")))
    for r in doc["asset"]["thermal"]["regions"]:
        r["areaM2"] = 1000.0
    os.makedirs(PROBE_DIR, exist_ok=True)
    probe = os.path.join(PROBE_DIR, "🔣️.json")
    json.dump(doc, open(probe, "w"), ensure_ascii=False)
    out = subprocess.run([BUN, "r3-validate.ts", "--file", probe, "--scale", "50", "--rules", "thermal-coverage", "--no-print"], cwd=TOOLS, capture_output=True, text=True).stdout
    found = {m.group(1): float(m.group(2)) for m in re.finditer(r'region "([a-z0-9-]+)" at 1:50 is ([0-9.]+) m²', out)}
    print("[DEBUG] measured outer areas at 1:50:", found)
    json.dump({k: round(v, 3) for k, v in found.items()}, open(AREAS, "w"), indent=1)
    subprocess.run(["python3", os.path.join(HERE, "author-slide-tower.py")], check=True, capture_output=True)


if __name__ == "__main__":
    main()
