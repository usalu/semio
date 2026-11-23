"""🔥 appliances/hot-water-cylinder: the welds of the six connection stubs are modelled for the coarse scales, and the structure lists are one load path.

1:100, 1:200, 1:500: each stub weld was the 8.4 mm fillet bead of 1:50 (a torus of 27 or 33 mm mean diameter). It is a weld collar now: a 15 mm disc of the outer bead diameter (34.3 mm, hot outlet 40.3 mm) around the stub at the shell, the joint a drawing of these scales can mean.
All scales: the two legs of the charging coil that pass through the tank wall are members (the coil hung 27 mm from the next member).
1:50: the dial thermometer gets its 12 mm stem through the insulation onto the tank and its case becomes a member, so the needle pivot is carried (it floated 45 mm from every member).
One-shot on `volumes/appliances/hot-water-cylinder.json`; publish with `volumes-insert.py`.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "appliances", "hot-water-cylinder.json")
DECISIONS = {
    "50": {"en": "The dial thermometer sits on a 12 mm stem that crosses the insulation to the tank; stem and case are members that carry the needle pivot. The two coil legs through the tank wall are members.", "de": "Das Zeigerthermometer sitzt auf einem 12-mm-Schaft, der die Dämmung bis zum Speicher quert; Schaft und Gehäuse sind Glieder, die das Zeigerlager tragen. Die zwei Schenkel der Heizwendel durch die Speicherwand sind Glieder."},
    "coarse": {"en": "Each stub weld is a 15 mm weld collar of the bead diameter around the stub at the shell; the 8.4 mm fillet bead is left to 1:50. The two coil legs through the tank wall are members.", "de": "Jede Stutzennaht ist ein 15-mm-Schweißkragen im Durchmesser der Naht um den Stutzen am Mantel; die 8,4-mm-Kehlnaht bleibt dem Maßstab 1:50. Die zwei Schenkel der Heizwendel durch die Speicherwand sind Glieder."},
}


def collar(profile):
    """⭕ The weld collar that replaces a bead: 15 mm along the stub, centred on the bead, out to its outer radius."""
    along = [point[0] for point in profile]
    middle = round((min(along) + max(along)) / 2, 5)
    radius = max(point[1] for point in profile)
    return [[round(middle - 0.0075, 5), 0], [round(middle - 0.0075, 5), radius], [round(middle + 0.0075, 5), radius], [round(middle + 0.0075, 5), 0]]


def add(solids, after, solid):
    """📌 Inserts a solid behind another one unless it is there."""
    if all(item["id"] != solid["id"] for item in solids):
        solids.insert(next(i for i, item in enumerate(solids) if item["id"] == after) + 1, solid)


def main():
    """▶️ Applies the joins to every scale."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]
    for scale in ("50", "100", "200", "500"):
        v = volumes[scale]
        if scale != "50":
            for solid in v["structure"]:
                if solid["id"].startswith("weld-pipe-") and len(solid["profile"]) == 7:
                    solid["profile"] = collar(solid["profile"])
        if any(solid["id"] == "coil-legs" for solid in v["body"]):
            add(v["structure"], "member-charging-coil", {"id": "member-coil-legs", "of": "coil-legs", "member": "connections", "part": "member"})
        sentence = DECISIONS["50" if scale == "50" else "coarse"]
        if sentence not in v["decisions"]:
            v["decisions"].append(sentence)
    fine = volumes["50"]
    bezel = next(solid for solid in fine["body"] if solid["id"] == "dial-bezel")
    stem = {"id": "dial-stem", "kind": "revolve", "bone": "zone-2", "material": bezel["material"], "role": "detail", "tone": bezel["tone"], "profile": [[0, 0], [0, 0.006], [0.05, 0.006], [0.05, 0]], "axis": [0, 0, -1], "origin": [0.0, 0.165, -0.237]}
    add(fine["body"], "dial-bezel", stem)
    add(fine["structure"], "needle-pivot", {"id": "dial-case-member", "of": "dial-bezel", "member": "zone-2", "part": "member"})
    add(fine["structure"], "dial-case-member", {"id": "dial-stem-member", "of": "dial-stem", "member": "zone-2", "part": "member"})
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
