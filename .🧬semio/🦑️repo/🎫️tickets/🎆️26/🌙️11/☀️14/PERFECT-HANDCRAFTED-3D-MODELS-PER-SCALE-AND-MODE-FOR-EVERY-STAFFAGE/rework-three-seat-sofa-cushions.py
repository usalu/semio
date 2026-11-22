"""🛋️ Three-seat sofa, author pass 1: the seat cushions get their domed crown above the top boxing seam (1:50 and 1:100).

The cushion keeps its envelope (top at 0.21 m above the deck, full plan size at the seam); above the seam the top panel
rises as a dome instead of a flat plateau with a rounded shoulder. Stations: height above the deck, inset from the full plan size.
Run from the ticket folder: python3 rework-three-seat-sofa-cushions.py, then rework-three-seat-sofa-frame.py (foam cores follow the body).
@see ./rework/assets/furniture--three-seat-sofa.md
"""
import json

SOURCE = "volumes/furniture/three-seat-sofa.json"
HALF_WIDTH_Z = 0.285
DOME = {
    "50": (0.17, [(0.180, 0.003), (0.189, 0.012), (0.196, 0.030), (0.202, 0.065), (0.206, 0.115), (0.2085, 0.180), (0.210, 0.235)]),
    "100": (0.175, [(0.190, 0.012), (0.200, 0.045), (0.207, 0.120), (0.210, 0.220)]),
}


def main():
    """✍️ Replace the stations above the seam of the three seat cushions."""
    doc = json.load(open(SOURCE, encoding="utf-8"))
    for scale, (seam, dome) in DOME.items():
        for body in doc["volumes"][scale]["body"]:
            if body["id"] in ("seat-1", "seat-2", "seat-3"):
                keep = [s for s in body["sections"] if s["at"][1] <= seam + 1e-9]
                full = keep[-1]
                top = [{**full, "at": [full["at"][0], y, full["at"][2]], "rx": round(full["rx"] - inset, 5), "ry": round(HALF_WIDTH_Z - inset, 5)} for y, inset in dome]
                body["sections"] = keep + top
    json.dump(doc, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("seat cushions domed", list(DOME))


main()
