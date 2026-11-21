"""💪 Round 5 roe deer: the 1:500 shoulder-and-upper-arm mass `near-fore-upper` is drawn by hand as a five-point outline in its bone frame (the traced outline carries a rear bump whose visible width falls under the 0.2 mm print feature); front and plan take the same outline through the shape's extruded solid.

Usage: python3 r5-roe-deer-upper500.py   (edits 🗑️generated/r3-roe-deer/🔣️.json in place)
"""
import json

W = "🗑️generated/r3-roe-deer/🔣️.json"
OUTLINE = [[0.17109, -0.0524], [0.17699, 0.04365], [-0.04845, 0.03773], [-0.04214, -0.05487], [0.0789, -0.07681]]
doc = json.load(open(W, encoding="utf-8"))
for view in ("elevation", "front", "plan"):
    shape = next(s for s in doc["asset"]["representations"]["500"]["geometry"][view] if s["id"] == "near-fore-upper")
    shape["points"] = OUTLINE
json.dump(doc, open(W, "w", encoding="utf-8"), ensure_ascii=False)
print("near-fore-upper 1:500 drawn by hand in three views")
