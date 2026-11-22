"""🧱 Two 1:500 structure solids left under the 75 mm feature by reworks that were cut off before publishing (2026-11-22 01:34): the silencer cross bearers of `machinery/diesel-generator` (60 mm deep, now 75 mm with the top face kept at 2.58 m under the saddles) and the base support disc of `vegetation/japanese-maple` (40 mm, now 75 mm below the ground plane). One-shot on the two volumes sources; publish each with `volumes-insert.py`."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def edit(path, ident, change):
    """✏️ Applies one stated change to one structure solid of 1:500 and writes the source back as it was formatted (one line)."""
    with open(path, encoding="utf8") as f:
        text = f.read()
    document = json.loads(text)
    solid = next(s for s in document["volumes"]["500"]["structure"] if s["id"] == ident)
    change(solid)
    with open(path, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=None if "\n" not in text.strip() else 2)
        f.write("\n")


def bearer(solid):
    """🔩 Cross bearer 100 x 75 mm, top at 2.58 m."""
    if solid["size"] != [0.1, 0.06, 2.44]:
        raise SystemExit("bearer: already applied")
    solid["size"] = [0.1, 0.075, 2.44]
    solid["transform"]["translation"] = [1.49, 2.5425, 0.0]


def disc(solid):
    """🌱 Support disc of 0.6 m radius, 75 mm deep."""
    if solid["profile"] != [[-0.04, 0.6], [0, 0.6]]:
        raise SystemExit("disc: already applied")
    solid["profile"] = [[-0.075, 0.6], [0, 0.6]]


if __name__ == "__main__":
    edit(os.path.join(HERE, "volumes", "machinery", "diesel-generator.json"), "member-silencer-bearer", bearer)
    edit(os.path.join(HERE, "volumes", "vegetation", "japanese-maple.json"), "support-base-support", disc)
    print("applied")
