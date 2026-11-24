"""🪜️ Repair of `stair-flight` at 1:50: the handrail is drawn as a stroke of its own width (0.85 mm, 42.5 mm) from end to end of the tube; a stroke ends half its width beyond its last point, so the drawn rail stood 21 mm over both cut ends of the tube in the side elevation (21.4 mm) and over its top end in the front elevation (20.6 mm). The strokes end half a width earlier, so that their caps end where the tube ends. Idempotent: python3 fix_stair_flight.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "stair-flight"
CAP = 0.02125
text, doc = sc.load(ID)
sc.shape(doc, 50, "elevation", "handrail")["points"] = [[sc.r6(0.51164 + CAP), 1.06462], [sc.r6(5.65775 - CAP), 1.06462]]
for name in ("balustrade-l", "balustrade-r"):
    rail = sc.shape(doc, 50, "front", name)
    rail["points"] = [rail["points"][0], [rail["points"][1][0], sc.r6(3.986 - CAP)]]
sc.save(ID, text, doc)
