"""📐️ Measures the outer area of every thermal region per scale with the product's own rule: writes a probe copy of the candidate with every `areaM2` set to 100 m² into the ticket's generated folder and reads the measured areas from the findings of `r3-validate.ts`."""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.normpath(os.path.join(HERE, "..", "..", "..", "..", "☀️14", "PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"))
PROBE = os.path.join(HERE, "🗑️generated", "used-accordion")
os.makedirs(PROBE, exist_ok=True)
doc = json.load(open(os.path.join(HERE, "🔣️.json"), encoding="utf-8"))
for r in doc["asset"]["thermal"]["regions"]:
    r["areaM2"] = 100
path = os.path.join(PROBE, "🔣️.json")
json.dump(doc, open(path, "w", encoding="utf-8"), ensure_ascii=False)
bun = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
for scale in sys.argv[1:] or ["50", "100", "200"]:
    out = subprocess.run([bun, "r3-validate.ts", "--file", path, "--scale", scale, "--rules", "thermal-coverage", "--no-print"], cwd=T, capture_output=True, text=True).stdout
    found = re.findall(r'region "([a-z-]+)" at 1:\d+ is ([0-9.]+) m²', out) or re.findall(r"is ([0-9.]+) m² against .*", out)
    print(f"[DEBUG] 1:{scale}", found if found else out[-600:])
