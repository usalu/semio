"""QA9: widen the drinking-fountain 1:500 button (feature width 0.188 mm < 0.2 mm) from 0.025 to 0.04 m. Atomic write."""
import json, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️FW"))
from atomic_json import write_json_atomic

p = os.path.join(r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects\drinking-fountain", "🔣️.json")
doc = json.load(open(p, encoding="utf-8"))
rep = doc["asset"]["representations"]["500"]
for vp in ("elevation", "front", "plan"):
    for s in rep["geometry"][vp]:
        if s["id"] == "button" and s["kind"] == "rect":
            print(vp, s["x"], s["w"], s["y"], s["h"])
            if vp == "elevation":
                s["x"], s["w"] = -0.02, 0.04
d = rep["decisions"]["thinElements"]
add_en = " The button is drawn 0.04 m wide (instead of 0.025 m) so its feature width stays above 0.2 mm."
add_de = " Der Taster ist 0,04 m breit gezeichnet (statt 0,025 m), damit seine Merkmalsbreite über 0,2 mm bleibt."
if add_en not in d["en"]:
    d["en"] = d["en"].rstrip() + add_en
    d["de"] = d["de"].rstrip() + add_de
write_json_atomic(p, doc)
