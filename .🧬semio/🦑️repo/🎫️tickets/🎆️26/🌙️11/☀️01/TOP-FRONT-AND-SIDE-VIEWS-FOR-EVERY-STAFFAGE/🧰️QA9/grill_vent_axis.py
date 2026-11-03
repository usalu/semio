"""QA9: the grill lid-vent bone gets rest angle 90 degrees (its own x axis is the vertical dome axis of the lid) so the lid dome can revolve about `lid-vent`
under the 'axis bone own rest direction' rule, without a helper bone. All bone-local data of lid-vent is rotated by -90 degrees so the drawing is unchanged. Atomic write."""
import json, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️FW"))
from atomic_json import write_json_atomic

PATHS = [
    r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects\charcoal-grill\🔣️.json",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "O1", "orig", "charcoal-grill.json"),
]
for p in PATHS:
    if not os.path.exists(p):
        continue
    doc = json.load(open(p, encoding="utf-8"))
    a = doc["asset"]
    bone = next(b for b in a["rig"]["bones"] if b["id"] == "lid-vent")
    if bone["angleDeg"] == 90:
        print("already")
        continue
    assert bone["angleDeg"] == 0
    bone["angleDeg"] = 90
    m = bone["mass"]
    m["comX"], m["comY"] = m["comY"], -m["comX"] + 0.0
    for an in a.get("anchors", []):
        if an.get("bone") == "lid-vent":
            an["x"], an["y"] = an["y"], -an["x"] + 0.0
    for rep in a["representations"].values():
        if rep["status"] != "authored":
            continue
        for s in rep["geometry"]["elevation"]:
            if s["bone"] != "lid-vent":
                continue
            if s["kind"] == "rect":
                x0, x1, y0, y1 = s["x"], s["x"] + s["w"], s["y"], s["y"] + s["h"]
                s["x"], s["y"], s["w"], s["h"] = round(y0, 6), round(-x1, 6), round(y1 - y0, 6), round(x1 - x0, 6)
            elif s["kind"] == "ellipse":
                s["cx"], s["cy"], s["rx"], s["ry"] = round(s["cy"], 6), round(-s["cx"], 6) + 0.0, s["ry"], s["rx"]
            else:
                raise ValueError(s["kind"])
    write_json_atomic(p, doc)
    print("rotated")
