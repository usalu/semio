"""🌵 vegetation/agave: the stem member is the heart of the rosette up to the leaf insertions, so every leaf core meets it (`volumes-connected` on the structure lists of 1:50, 1:100 and 1:200: leaf cores ended 2 to 25 mm outside the former 170 to 120 mm rod).

The 28 leaves insert on a cone of 100 mm radius at 50 mm height falling to 50 mm at 280 mm; their core members begin about 20 mm outside it. The heart is drawn 20 mm outside the insertion cone (240 mm at the base, 140 mm under the spear), inside the caudex body of every scale (260 mm, 1:500: 244 mm at 50 mm height). One-shot on `volumes/vegetation/agave.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "vegetation", "agave.json")
ROD = [[0, 0.085], [0.3, 0.06]]
HEART = [[0, 0.12], [0.05, 0.12], [0.28, 0.07], [0.3, 0.06]]
SENTENCE = {
    "en": "The stem member is the heart of the rosette, 20 mm outside the cone of the leaf insertions, so every leaf core is fixed in it.",
    "de": "Das Stammglied ist das Herz der Rosette, 20 mm außerhalb des Kegels der Blattansätze, sodass jeder Blattkern in ihm eingespannt ist.",
}


def main():
    """▶️ Replaces the profile of `m-caudex` at every scale and appends one decision."""
    with open(SOURCE, encoding="utf8") as f:
        text = f.read()
    document = json.loads(text)
    for scale in ("50", "100", "200", "500"):
        volumes = document["volumes"][scale]
        stem = next(solid for solid in volumes["structure"] if solid["id"] == "m-caudex")
        if stem["profile"] != ROD:
            raise SystemExit(f"{scale}: m-caudex is not the measured rod (already applied?)")
        stem["profile"] = HEART
        volumes["decisions"].append(SENTENCE)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=None if "\n" not in text.strip() else 1)
        f.write("\n")
    print("applied")


if __name__ == "__main__":
    main()
