"""🚗 vehicles/city-car: brake discs of the coarse scales modelled for the smallest feature of their scale instead of the 12 mm plate of 1:50.

1:100: the plain rear discs are 15 mm thick (were 12). 1:200: 30 mm, as the front discs already are. 1:500: all four discs are one 75 mm brake body between the hub faces of the disc hat (-45 to +30 mm), radii unchanged (75 to 150 mm). One-shot on `volumes/vehicles/city-car.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "vehicles", "city-car.json")
SPAN = {"100": {"rl-disc": (-0.015, 0.0), "rr-disc": (-0.015, 0.0)}, "200": {"rl-disc": (-0.03, 0.0), "rr-disc": (-0.03, 0.0)},
        "500": {key: (-0.045, 0.03) for key in ("fl-disc", "fr-disc", "rl-disc", "rr-disc")}}
PLATE = [[-0.012, 0.15], [0, 0.15], [0, 0.075], [-0.012, 0.075], [-0.012, 0.15]]


def main():
    """▶️ Rewrites the profiles of the named discs and one sentence of the decisions."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    for scale, discs in SPAN.items():
        body = {solid["id"]: solid for solid in document["volumes"][scale]["body"]}
        for ident, (a, b) in discs.items():
            if body[ident]["profile"] != PLATE:
                raise SystemExit(f"{scale} {ident}: not the 12 mm plate (already applied?)")
            body[ident]["profile"] = [[a, 0.15], [b, 0.15], [b, 0.075], [a, 0.075], [a, 0.15]]
        document["volumes"][scale]["decisions"].append({
            "en": {"100": "The plain rear brake discs are 15 mm thick, the smallest feature of 1:100.", "200": "The rear brake discs are 30 mm thick like the front ones, the smallest feature of 1:200.", "500": "Each brake is one 75 mm body of disc and hat behind the wheel disc."}[scale],
            "de": {"100": "Die massiven hinteren Bremsscheiben sind 15 mm dick, das kleinste Merkmal von 1:100.", "200": "Die hinteren Bremsscheiben sind wie die vorderen 30 mm dick, das kleinste Merkmal von 1:200.", "500": "Jede Bremse ist ein 75 mm tiefer Körper aus Scheibe und Topf hinter der Radscheibe."}[scale]})
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False)
        f.write("\n")
    print("applied")


if __name__ == "__main__":
    main()
