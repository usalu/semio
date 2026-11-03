"""🔢 List the quantitative leaves (W, kg, L, m, minutes, °C, MET) of the catalog assets named on the command line."""
import json
import re
import sys
from pathlib import Path

CATALOG = Path(sys.argv[1])
WANTED = set(sys.argv[2:])
SKIP = re.compile(r"rig\.|representations|anchors|materials|hatches|geometry|layers|bones|penMm|spacingMm")
KEEP = re.compile(r"dimensions\.|thermal\.body\.(massKg|basalMetabolismW|metWM2|skinAreaM2)|baseMet|activityMet|heatW|powerW|Watt|WM2|volumeL|tidalVolume|volumeFlow|temperatureRiseK|vapourKgPerKg|massKg|mass\.kg|emitters\.\d+\.(diameterM|volumeFlow|temperatureRise)|heightM|widthM|depthM|Litre|LM2H|latent|phaseChange|operating|flow|Flow|water|Water")
for path in sorted(CATALOG.rglob("🔣️.json")):
    doc = json.loads(path.read_text(encoding="utf-8"))
    asset = doc.get("asset", {})
    if asset.get("id") not in WANTED:
        continue
    print("==", asset["id"], "|", str(path.relative_to(CATALOG)).replace("\\", "/"))
    print("  title:", asset.get("title", {}).get("en"), "/", asset.get("title", {}).get("de"))
    leaves = []

    def walk(node, trail):
        if isinstance(node, dict):
            for key, value in node.items():
                walk(value, trail + [key])
        elif isinstance(node, list):
            for index, value in enumerate(node):
                walk(value, trail + [str(index)])
        elif isinstance(node, (int, float)) and not isinstance(node, bool):
            path = ".".join(trail)
            if SKIP.search(path):
                return
            if KEEP.search(path):
                leaves.append(path + " = " + repr(node))

    walk({k: v for k, v in doc.items() if k not in ("$schema", "version")}, [])
    for leaf in leaves:
        print("   ", leaf)
