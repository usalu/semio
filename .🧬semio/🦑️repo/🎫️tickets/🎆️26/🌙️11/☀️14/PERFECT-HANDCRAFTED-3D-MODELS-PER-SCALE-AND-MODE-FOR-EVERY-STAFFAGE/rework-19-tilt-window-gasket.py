"""🪟 Rework 19, tilt window: the sash no longer closes timber on timber. A kerf-mounted EPDM compression gasket 8 mm wide runs round the outer face of the sash where it meets the four stops 8 mm deep (1:50 only; under the smallest feature of the coarser scales). Material `epdm` after EN ISO 10456 is added to the asset."""
import sys

sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
import rework_19_lib as L

SLUG = "tilt-window"
OUTER = [[-0.384, 0.008], [0.384, 0.008], [0.384, 1.198], [-0.384, 1.198]]
INNER = [[-0.376, 0.016], [-0.376, 1.19], [0.376, 1.19], [0.376, 0.016]]
GASKET = {"id": "rebate-gasket", "kind": "extrude", "bone": "sash", "material": "epdm", "role": "detail", "tone": 0.15, "profile": OUTER, "holes": [INNER], "plane": "zx", "from": -0.0984, "to": -0.0904}
EPDM = {"id": "epdm", "title": {"en": "EPDM rebate gasket", "de": "EPDM-Falzdichtung"}, "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "source": "literature", "reference": "en-iso-10456", "conductivityWmK": 0.25, "specificHeatJKgK": 1000}
NOTE = {"en": "Between sash and stops sits a kerf-mounted EPDM compression gasket, one closed ring 8 mm wide in the outer sash face; it is the seal line of the closed window and shows on the sash as soon as it tilts.", "de": "Zwischen Flügel und Anschlägen sitzt eine eingenutete EPDM-Kompressionsdichtung, ein geschlossener, 8 mm breiter Ring in der äußeren Flügelfläche; sie ist die Dichtebene des geschlossenen Fensters und wird am Flügel sichtbar, sobald er kippt."}

if "--roots" in sys.argv:
    asset = L.load_asset(SLUG)
    if not any(m["id"] == "epdm" for m in asset["materials"]):
        asset["materials"].append(EPDM)
        L.write_members(SLUG, {"materials": asset["materials"]})
doc = L.load_source(SLUG)
for scale in ("50",):
    v = doc["volumes"][scale]
    if not any(x["id"] == "rebate-gasket" for x in v["body"]):
        k = next(n for n, x in enumerate(v["body"]) if x["id"] == "weather-bar")
        v["body"].insert(k + 1, dict(GASKET))
        k = next(n for n, x in enumerate(v["thermal"]) if x["id"] == "th-weather-bar")
        v["thermal"].insert(k + 1, {"id": "th-rebate-gasket", "of": "rebate-gasket", "region": "frame-timber"})
        v["decisions"].append(NOTE)
L.save_source(SLUG, doc)
