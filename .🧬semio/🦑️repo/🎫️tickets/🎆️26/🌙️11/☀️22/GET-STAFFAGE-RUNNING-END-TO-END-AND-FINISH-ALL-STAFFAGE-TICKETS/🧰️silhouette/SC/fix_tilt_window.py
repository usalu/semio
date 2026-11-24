"""🪟️ Repair of `tilt-window`: the front elevation of 1:200 drew the 50 mm frame and the sash with a 12 mm ring of nothing between them, where the body has the 30 mm stops (62 mm from the next drawn edge); the frame members are drawn with their stops, 80 mm wide, as they are seen from ahead. Idempotent: python3 fix_tilt_window.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "tilt-window"
text, doc = sc.load(ID)
sc.shape(doc, 200, "front", "sill").update({"y": 0.0, "h": 0.08})
sc.shape(doc, 200, "front", "head").update({"y": 1.244, "h": 0.08})
sc.shape(doc, 200, "front", "jamb-left").update({"x": 0.37, "w": 0.08})
sc.shape(doc, 200, "front", "jamb-right").update({"x": -0.45, "w": 0.08})
sc.save(ID, text, doc)
