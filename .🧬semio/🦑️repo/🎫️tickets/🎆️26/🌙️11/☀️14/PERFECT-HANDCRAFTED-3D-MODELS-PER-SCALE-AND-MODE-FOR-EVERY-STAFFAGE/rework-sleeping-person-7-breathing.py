"""🌬️ The breathing key of humans/sleeping-person keeps the bent elbow of the supine pose: 71.22° less the half degree the hand lifts.

Usage: python3 rework-sleeping-person-7-breathing.py   (re-reads the catalog document and writes only clips[breathing].keys[2.4 s].bones.forearm-r)
"""
import json

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
LIFT_DEG = -0.5


def main():
    text = open(CATALOG).read()
    document = json.loads(text)
    asset = document["asset"]
    rest = next(p for p in asset["poses"] if p["id"] == "supine")["bones"]["forearm-r"]["angleDeg"]
    key = next(k for k in next(c for c in asset["clips"] if c["id"] == "breathing")["keys"] if k["t"] == 2.4)
    key["bones"]["forearm-r"] = {"angleDeg": round(rest + LIFT_DEG, 2)}
    first = text.find("\n")
    indent = len(text[first + 1:]) - len(text[first + 1:].lstrip(" "))
    with open(CATALOG, "w") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=indent)
        handle.write("\n" if text.endswith("\n") else "")
    print("forearm-r at 2.4 s:", key["bones"]["forearm-r"])


main()
