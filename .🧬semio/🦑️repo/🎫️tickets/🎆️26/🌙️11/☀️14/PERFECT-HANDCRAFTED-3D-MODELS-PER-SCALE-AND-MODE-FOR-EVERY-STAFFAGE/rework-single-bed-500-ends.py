"""🛏️ single-bed 1:500: head and foot end stand on their two posts instead of closing the 140 mm clear height under the lower rail.

Hand-chosen numbers (m): posts 60 wide at z = +-0.445..0.505, lower edge of both end panels at 0.14 (underside of the side rails),
head end 0.95 high, foot end 0.55 high, both 60 thick. The front drawings get the same notch. Run, then `volumes-insert.py`.
"""
import glob
import json

T = __file__.rsplit("/", 1)[0]
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/single-bed/*.json")[0]
SOURCE = T + "/volumes/furniture/single-bed.json"


def end(solid_id, height, x0):
    """🪵 One end of the bed as an inverted U in the (y, z) plane, 60 mm thick along x."""
    profile = [[0, 0.505], [height, 0.505], [height, -0.505], [0, -0.505], [0, -0.445], [0.14, -0.445], [0.14, 0.445], [0, 0.445]]
    return {"id": solid_id, "kind": "extrude", "bone": "frame", "material": "pine-timber", "role": "body", "tone": 0.5, "profile": profile, "plane": "yz", "from": x0, "to": round(x0 + 0.06, 3)}


def notch(shape, height):
    """📐 The front drawing of one end with the same clear height between the posts."""
    for key in ("x", "y", "w", "h"):
        del shape[key]
    shape["kind"] = "poly"
    shape["points"] = [[-0.505, 0], [-0.445, 0], [-0.445, 0.14], [0.445, 0.14], [0.445, 0], [0.505, 0], [0.505, height], [-0.505, height]]
    shape["closed"] = True


def main():
    source = json.load(open(SOURCE))
    scale = source["volumes"]["500"]
    body = scale["body"]
    assert [s["id"] for s in body[:2]] == ["head", "foot"] and body[0]["kind"] == "box"
    body[0], body[1] = end("head", 0.95, 0.0), end("foot", 0.55, 2.06)
    scale["decisions"][0] = {
        "en": "Body, authored anew for 75 mm: head end and foot end as one panel each at their true heights of 0.95 and 0.55 m, standing on their two posts with the 140 mm clear height under the lower rail left open; side rails and slatted base merged into one tray on the centre leg; the mattress as three plain blocks on their bones. The model is minimal: nothing smaller can be left out without losing the bed.",
        "de": "Körper, für 75 mm neu gebaut: Kopf- und Fußteil je eine Platte in wahrer Höhe von 0,95 und 0,55 m, die auf ihren beiden Pfosten steht und die lichte Höhe von 140 mm unter dem unteren Riegel offen lässt; Seitenholme und Lattenrost zu einer Wanne auf dem Mittelfuß verschmolzen; die Matratze als drei glatte Blöcke auf ihren Knochen. Das Modell ist minimal: kleiner geht es nicht, ohne das Bett zu verlieren.",
    }
    document = json.load(open(CATALOG))
    asset = document["asset"]
    asset.pop("volumes")
    front = {s["id"]: s for s in asset["representations"]["500"]["geometry"]["front"]}
    notch(front["head"], 0.95)
    notch(front["foot"], 0.55)
    open(SOURCE, "w").write(json.dumps(source, ensure_ascii=False, indent=1))
    open(CATALOG, "w").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("1:500 ends written")


if __name__ == "__main__":
    main()
