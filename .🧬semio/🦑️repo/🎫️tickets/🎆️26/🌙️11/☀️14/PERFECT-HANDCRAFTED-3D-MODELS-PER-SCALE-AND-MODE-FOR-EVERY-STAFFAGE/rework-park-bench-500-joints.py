"""🔩 furniture/park-bench: at 1:500 the six slat bolts of the structure list are joint pins of 40 mm, the thinnest drawn part of the backrest, instead of the 19 mm coach bolts of 1:50.

Each pin keeps its place and its span through slat and post (0 to 58.4 mm along the bolt axis); head, shank step and nut are below the scale. One-shot on `volumes/furniture/park-bench.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "furniture", "park-bench.json")
PIN = [[-0.00576, 0.02], [0.0584, 0.02]]
SENTENCE = {
    "en": "The six bolts of the slats are plain 40 mm joint pins through slat and post, as wide as the thinnest drawn part of the backrest; head and nut are below this scale.",
    "de": "Die sechs Schrauben der Latten sind glatte 40-mm-Gelenkstifte durch Latte und Pfosten, so breit wie das dünnste gezeichnete Teil der Rückenlehne; Kopf und Mutter liegen unter diesem Maßstab.",
}


def main():
    """▶️ Rewrites the three repeated back bolts of 1:500 and appends one decision."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]["500"]
    for solid in volumes["structure"]:
        if solid["id"] in ("bolt-back-1", "bolt-back-2", "bolt-back-3"):
            solid["profile"] = PIN
    if SENTENCE not in volumes["decisions"]:
        volumes["decisions"].append(SENTENCE)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
