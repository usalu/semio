"""🦶 Round 5, appliances/desktop-monitor: the four 2 mm rubber pads are below the 7.5 mm feature of 1:50 and below the printable 0.2 mm at the 1:20 end of its band (0.19 mm), so the catalog was refused by the print check. At 1:50 they merge into the base plate in 2D (shapes removed, plate drawn down to the desk) and in 3D (pad solids and their thermal references removed, plate loft down to the desk, the steel core, its mass and the contact pad at the 7.5 mm feature that the pad drawing no longer lowers); the decisions say so.

Usage: python3 r5-desktop-monitor-pads.py candidate   writes 🗑️generated/r5-desktop-monitor/cand/🔣️.json (catalog + source volumes)
       python3 r5-desktop-monitor-pads.py apply       writes the candidate's representations into the catalog file and its volumes into the ticket source
@see ./r5-brief.md
@see ./r3-desktop-monitor-apply.py
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/desktop-monitor/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "desktop-monitor.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r5-desktop-monitor", "cand", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])

PADS_2D = {"foot-front", "foot-rear", "foot-left", "foot-right"}
PADS_3D = {"foot-front", "foot-rear", "th-foot-front", "th-foot-rear"}
BASE_STRUCTURE = {
    "member-base-core": {"size": [0.17, 0.0075, 0.22], "transform": {"translation": [0.105, 0.00575, 0.0]}},
    "mass-base": {"radii": [0.02, 0.00375, 0.02], "transform": {"translation": [0.105, 0.00575, 0.0]}},
    "support-base-contact": {"size": [0.19, 0.0075, 0.2], "transform": {"translation": [0.105, 0.00375, 0.0]}},
}
TEXT_2D = [
    ("the base plate with its bevelled top on two rubber pads,", "the base plate with its bevelled top on the desk,"),
    ("die Fußplatte mit abgeschrägter Oberkante auf zwei Gummifüßen,", "die Fußplatte mit abgeschrägter Oberkante auf dem Tisch,"),
    ("the plate on its two pads.", "the plate on the desk."),
    ("die Platte auf ihren zwei Füßen.", "die Platte auf dem Tisch."),
    ("the bracket, the pads and the power light.", "the bracket and the power light."),
    ("der Halter, die Füße und die Betriebsleuchte.", "der Halter und die Betriebsleuchte."),
    ("The 2 mm rubber pads are drawn at their true height (0.04 mm on paper);", "The four 2 mm rubber pads (0.04 mm on paper, 0.1 mm at 1:20) are below the 7.5 mm feature and merge into the base plate, which is drawn down to the desk;"),
    ("Die 2 mm hohen Gummifüße sind in wahrer Höhe gezeichnet (0,04 mm auf Papier);", "Die vier 2 mm hohen Gummifüße (0,04 mm auf Papier, 0,1 mm bei 1:20) liegen unter dem Merkmal von 7,5 mm und gehen in der Fußplatte auf, die bis auf den Tisch gezeichnet ist;"),
    ("glass 0.1, pads 0.15;", "glass 0.1;"),
    ("Glas 0,1, Füße 0,15;", "Glas 0,1;"),
]
TEXT_3D = [
    ("with a bevelled top edge on four 22 mm rubber pads.", "with a bevelled top edge, drawn down to the desk: the four 2 mm rubber pads are below the 7.5 mm feature and merge into it."),
    ("mit abgeschrägter Oberkante auf vier 22-mm-Gummifüßen.", "mit abgeschrägter Oberkante, bis auf den Tisch geführt: die vier 2 mm hohen Gummifüße liegen unter dem Merkmal von 7,5 mm und gehen in ihr auf."),
    ("one contact pad under the four feet.", "one contact pad under the plate."),
    ("ein Kontaktpolster unter den vier Füßen.", "ein Kontaktpolster unter der Platte."),
]


def swap(value, pairs):
    """🔁 Every listed sentence part replaced in every string of a decision tree; a part that is missing is reported."""
    raw = json.dumps(value, ensure_ascii=False)
    for old, new in pairs:
        if old in raw:
            raw = raw.replace(old, new)
        elif new not in raw:
            print("[r5] text not found:", old[:60])
    return json.loads(raw)


def candidate():
    """🧪 The catalog asset with the 1:50 pads merged in 2D and in the source volumes."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    asset = doc["asset"]
    rep = asset["representations"]["50"]
    for view, shapes in rep["geometry"].items():
        rep["geometry"][view] = [s for s in shapes if s["id"] not in PADS_2D]
        for s in rep["geometry"][view]:
            if s["id"] == "base-plate" and view in ("elevation", "front"):
                s["points"] = [[x, 0.0 if y == 0.002 else y] for x, y in s["points"]]
    rep["decisions"] = swap(rep["decisions"], TEXT_2D)
    volumes = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    v50 = volumes["50"]
    for key in ("body", "thermal"):
        v50[key] = [s for s in v50[key] if s["id"] not in PADS_3D]
    for s in v50["body"]:
        if s["id"] == "base-plate" and s["sections"][0]["at"][1] == 0.002:
            s["sections"][0]["at"][1] = 0.0
    for s in v50["structure"]:
        if s["id"] in BASE_STRUCTURE:
            s.update(BASE_STRUCTURE[s["id"]])
    v50["decisions"] = swap(v50["decisions"], TEXT_3D)
    asset["volumes"] = volumes
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("[r5] candidate", CANDIDATE)


def apply():
    """📝 The candidate's representations into the catalog file (its own formatting, member order kept) and its volumes into the ticket source."""
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    start, end = member_range(text, "    ", "representations")
    text = text[:start] + "    " + json.dumps("representations") + ": " + json.dumps(want["representations"], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text[end:]
    if json.loads(text)["asset"]["representations"] != want["representations"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    source = json.load(open(SOURCE, encoding="utf-8"))
    source["volumes"] = want["volumes"]
    open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, indent=1, ensure_ascii=False) + "\n")
    print("[r5] written representations and source volumes")


{"candidate": candidate, "apply": apply}[sys.argv[1]]()
