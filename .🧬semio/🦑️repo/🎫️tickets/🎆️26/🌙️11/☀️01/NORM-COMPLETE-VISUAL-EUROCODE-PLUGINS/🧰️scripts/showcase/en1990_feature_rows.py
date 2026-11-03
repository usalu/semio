"""Appends the road-bridge showcase rows (pass and fail, from the Python oracle JSON in 🗑️generated) to the en1990 check-outcome feature; idempotent (python3 -I en1990_feature_rows.py pass.json fail.json)."""
import json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact

FEATURE = artifact("⚖️en1990") / "🧪️tests/🚦️check-outcome-en1990-1/🥒️.feature"
MARK = "road-bridge-showcase-pass/"


def checks(path):
    return {c["id"]: c["utilization"] for c in json.load(open(path, encoding="utf-8"))["checks"]}


good, bad = checks(sys.argv[1]), checks(sys.argv[2])
rows = []
for key, value in ((k, v) for k, v in good.items() if not 0.97 <= v <= 1.03 or ".traffic-group" in k):
    if key.endswith("span-2") or ".span-2." in key:
        if not any(t in key for t in ("str.", "deflection.", "fat.damage")):
            continue
    rows.append(f"      | pass-rb-{len(rows) + 1:02d} | shared://🚦️check-outcome/road-bridge-showcase-pass/🔣️.json | {key} | pass | {value:.9g} | 1e-3 |")
for key, value in bad.items():
    if value > 1.05 and (".span-2" not in key or "fat.damage" in key):
        rows.append(f"      | fail-rb-{len(rows) + 1:02d} | shared://🚦️check-outcome/road-bridge-showcase-fail/🔣️.json | {key} | fail | {value:.9g} | 1e-3 |")
text = FEATURE.read_text(encoding="utf-8")
if MARK in text:
    print("rows already present")
    sys.exit(0)
FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf-8")
print(len(rows), "rows appended")
