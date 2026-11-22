"""📏 Reads the 1:50 outer areas of the thermal regions from the validator and writes them to `author-areas.json`, so that `areaM2` is the honest outer area of the 1:50 model this author built (the asset is new; there is no earlier value to keep). Run from anywhere: `python3 author-areas.py`."""
import json
import pathlib
import re
import subprocess

HERE = pathlib.Path(__file__).parent
T = HERE.parents[3] / "☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN = sorted(pathlib.Path.home().glob("Library/Caches/semio/runtimes/bun/*/bun"))[0]
out = subprocess.run([str(BUN), "r3-validate.ts", "--file", str(HERE / "🔣️.json"), "--scale", "50", "--rules", "thermal-coverage", "--no-print"], cwd=T, capture_output=True, text=True).stdout
areas_file = HERE / "author-areas.json"
areas = json.loads(areas_file.read_text()) if areas_file.exists() else {}
for rid, a in re.findall(r'region "([a-z0-9-]+)" at 1:50 is ([0-9.]+) m²', out):
    areas[rid] = float(a)
areas_file.write_text(json.dumps(areas, indent=1) + "\n")
print("[DEBUG]", out.strip().splitlines()[-1][:200])
print("[DEBUG] areas", areas)
