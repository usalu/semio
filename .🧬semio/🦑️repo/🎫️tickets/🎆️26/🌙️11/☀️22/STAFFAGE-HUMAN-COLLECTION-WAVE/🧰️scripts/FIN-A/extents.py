"""📐️ Prints the default-pose extents the validator measures in every drawing of an asset file (height, forward width, lateral depth per scale and view) by validating a copy with deliberately wrong dimensions: python3 extents.py <slug|file.json>..."""
import json
import os
import re
import subprocess
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
BUN = [os.path.join(d, "bun") for d in sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*")))][0]


def extents(path):
    doc = json.load(open(path, encoding="utf-8"))
    doc["asset"]["dimensions"].update({"heightM": 99, "widthM": 99, "depthM": 99})
    os.makedirs(T + "/🗑️generated/FIN-A", exist_ok=True)
    tmp = T + "/🗑️generated/FIN-A/[DEBUG]-extents.json"
    json.dump(doc, open(tmp, "w", encoding="utf-8"), ensure_ascii=False)
    out = subprocess.run([BUN, "validate.ts", "--file", tmp], capture_output=True, text=True, cwd=T + "/🧰️scripts/HW-01").stdout
    os.remove(tmp)
    rows = {}
    for m in re.finditer(r"(extent-[a-z-]+): The default-pose [a-z ]+ ([\d.]+) m of the 1:(\d+)", out):
        rows.setdefault(m.group(1), {})[m.group(3)] = float(m.group(2))
    return rows


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        path = arg if arg.endswith(".json") else f"{CAT}/{arg}/🔣️.json"
        print(os.path.basename(os.path.dirname(path)) if not arg.endswith(".json") else os.path.basename(path))
        for code, by in sorted(extents(path).items()):
            print(f"   {code:22s}", " ".join(f"1:{s} {by.get(s, float('nan')):.3f}" for s in ("500", "200", "100", "50")), f"| spread {max(by.values()) - min(by.values()):.3f}")
