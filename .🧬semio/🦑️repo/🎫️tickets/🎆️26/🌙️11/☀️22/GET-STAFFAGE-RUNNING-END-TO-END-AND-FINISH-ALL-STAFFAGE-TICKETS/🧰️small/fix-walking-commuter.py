"""🚶️ Applies the redraw of `wc_redraw.py` (left arm with hand and fingers, both feet, both legs) to the catalog document of the walking commuter, rewriting only the spans of the changed shapes; `wc-rest.json` holds the rest frames of the bones (`🧰️three-views/rest-frames.ts`). Run once: python3 fix-walking-commuter.py"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "🧰️three-views"))
import tv2_doc
import wc_redraw

doc = tv2_doc.Doc(tv2_doc.catalog_file("walking-commuter"))
if any("solid" in shape for shape in doc.get("asset", "representations", "50", "geometry", "plan")):
    raise SystemExit("already applied")
shapes = wc_redraw.redraw(doc.get("asset"), json.load(open(os.path.join(HERE, "wc-rest.json"), encoding="utf8")), ["arm", "feet", "legs"])
for (scale, view, shape_id), shape in shapes.items():
    base = ("asset", "representations", str(scale), "geometry", view)
    ids = [item["id"] for item in doc.get(*base)]
    if shape_id in ids:
        doc.set(base + (ids.index(shape_id),), shape)
    else:
        doc.insert(base, ids.index(wc_redraw.FAR[shape_id]), shape)
doc.save()
print(f"[DEBUG] walking-commuter: {len(shapes)} shapes redrawn")
