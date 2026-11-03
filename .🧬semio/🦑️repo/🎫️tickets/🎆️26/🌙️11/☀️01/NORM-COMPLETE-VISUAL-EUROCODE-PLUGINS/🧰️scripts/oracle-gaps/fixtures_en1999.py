"""🧫 Writes the EN 1999 check-outcome fixtures `edge-stiffener-pass` (the showcase, whose slender beam has 8 x 3 mm lips on 23.5 x 3 mm outstands) and `edge-stiffener-fail` (outstands of 60 x 1.2 mm, so the lip is a soft strut on a weak strip)."""
import json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
for name, weak in (("edge-stiffener-pass", False), ("edge-stiffener-fail", True)):
    d = json.load(open(root / "🎪️showcase/🔣️.json", encoding="utf8"))
    if weak:
        for section in d["sections"]:
            for el in section["elements"]:
                if section["id"] == "sec-slender" and el["id"] == "flange-top":
                    el.update(width=0.06, thickness=0.0012)
    out = root / "🚦️check-outcome" / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
