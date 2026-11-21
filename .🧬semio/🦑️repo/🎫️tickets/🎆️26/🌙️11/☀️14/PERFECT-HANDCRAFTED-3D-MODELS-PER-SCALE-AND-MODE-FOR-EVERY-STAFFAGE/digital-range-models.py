"""🖊️ Draw the narrow connections and screen edge legibly in the 1:200 print band."""
import json
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
lib = run_path(str(ticket / "r5-range-gaps-lib.py"))
cases = ["laptop", "gas-meter", "mvhr-unit"]
for name in cases:
    path = Path(lib["asset_path"]("appliances/" + name))
    raw = path.read_text()
    doc = json.loads(raw)
    band = doc["asset"]["representations"]["200"]
    band["validRange"]["maxDenominator"] = 350
    for view, shapes in band["geometry"].items():
        for shape in shapes:
            part = shape["id"]
            if name == "laptop" and part == "lid":
                shape["penMm"] = .20
            if name == "gas-meter" and part == "counter":
                shape["penMm"] = .15
            if name == "gas-meter" and view == "elevation" and part in ("stub-1","stub-2"):
                shape["w"] = .032
                shape["x"] = -.080 if part == "stub-1" else .048
                shape["penMm"] = .12
            if name == "mvhr-unit" and part.startswith("spigot-"):
                shape["penMm"] = .10
                if view == "plan":
                    cx=(max(p[0] for p in shape["points"])+min(p[0] for p in shape["points"]))/2
                    cy=(max(p[1] for p in shape["points"])+min(p[1] for p in shape["points"]))/2
                    radius=max(abs(x-cx) for x,y in shape["points"])
                    shape["points"]=[[round(cx+(x-cx)*.02925/radius,6),round(cy+(y-cy)*.02925/radius,6)] for x,y in shape["points"]]
                elif view == "elevation":
                    shape["y"] += (shape["h"]-.0585)/2
                    shape["h"] = .0585
                else:
                    shape["x"] += (shape["w"]-.0585)/2
                    shape["w"] = .0585
    p=ticket / "🗑️generated" / ("digital-band-" + name + ".json")
    p.write_text(json.dumps(doc,ensure_ascii=False))
