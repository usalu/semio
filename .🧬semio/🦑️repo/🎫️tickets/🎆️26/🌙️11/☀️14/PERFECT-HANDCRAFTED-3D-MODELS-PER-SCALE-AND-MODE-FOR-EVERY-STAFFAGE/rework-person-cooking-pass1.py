"""🍳 person-cooking, pass 1 of the second round: girdles on the lateral rest placement plus the hand corrections of the joints that did not meet.

Runs `rework-person-cooking-girdle.py` (clavicles, hip bones, re-bound girdle solids) on the catalog asset and the volumes source, then closes by hand what the sheets and `volumes-connected` showed open: drawstrings on the sweatshirt, sacroiliac joints, radiocarpal joints, femoral shafts into their condyles, atlas into the skull base, hob plate as the member that carries the pot.

Usage: python3 rework-person-cooking-pass1.py            candidate `🗑️generated/rework-person-cooking/person-cooking/🔣️.json`
       python3 rework-person-cooking-pass1.py --apply    writes the catalog file and the volumes source
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("girdle", os.path.join(T, "rework-person-cooking-girdle.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)

DRAWSTRING_Y = {-0.06921: -0.06571, -0.09332: -0.08982, -0.12442: -0.12092, -0.15073: -0.14723}
DRAWSTRING_TIP_Y = {-0.15334: -0.14984}
SACRUM_RX = {"50": {0.056: 0.061, 0.048: 0.053}, "100": {0.054: 0.062}}
SACRUM_TAIL = {"50": {2: [0.01538, 0.072, 0], 3: [-0.012, 0.082, 0], 4: [-0.028, 0.08, 0]}, "100": {1: [-0.022, 0.082, 0]}, "200": {1: [-0.022, 0.082, 0]}, "500": {1: [-0.022, 0.082, 0]}}
RIB = 0.0065
RIBS_200 = [
    ("rib-1", 0.39, [(0.354, 0.02, 0.092), (0.318, -0.06, 0.062)], (0.307, None, 0.02)),
    ("rib-2", 0.308, [(0.263, 0.02, 0.108), (0.218, -0.06, 0.072)], (0.24, -0.104, 0.02)),
    ("rib-3", 0.23, [(0.192, 0.02, 0.11), (0.16, -0.055, 0.076)], (0.229, -0.082, 0.046)),
]
RIBS_500 = [
    ("rib-1", 0.385, [(0.346, 0.02, 0.095), (0.308, -0.06, 0.064)], (0.3, None, 0.02)),
    ("rib-2", 0.25, [(0.21, 0.02, 0.11), (0.175, -0.055, 0.077)], (0.232, -0.084, 0.044)),
]
SHAFTS = {
    "spine": [0.023, 0.023, 0.02, 0.016, 0.014, 0.012],
    "femur": [0.021, 0.016, 0.013, 0.024],
    "tibia": [0.028, 0.0135, 0.016],
    "humerus": [0.021, 0.011, 0.02],
    "forearm-bones": [0.012, 0.011, 0.015],
}
ENDS_500 = {"femur": 0.416, "humerus": 0.3135}
SLEEVE_CAP_X = {-0.02778: -0.0208, -0.02496: -0.018, -0.02: -0.0135}
HOB_MEMBER = {"id": "member-hob-plate", "of": "hob-plate", "member": "counter", "part": "member"}


def chisel(volumes):
    for scale, entry in volumes.items():
        every = [solid for name in ("body", "thermal", "structure") for solid in entry[name]]
        solids = {solid["id"]: solid for solid in every}
        for solid in every:
            stem = solid["id"].removeprefix("th-")
            if stem in ("drawstring-l", "drawstring-r") and "sections" in solid:
                for section in solid["sections"]:
                    section["at"][1] = DRAWSTRING_Y.get(section["at"][1], section["at"][1])
            if stem in ("drawstring-tip-l", "drawstring-tip-r") and "transform" in solid:
                at = solid["transform"]["translation"]
                at[1] = DRAWSTRING_TIP_Y.get(at[1], at[1])
        if scale in SACRUM_RX:
            for section in solids["sacrum"]["sections"]:
                section["rx"] = SACRUM_RX[scale].get(section["rx"], section["rx"])
        if scale == "50":
            for s, sign in (("l", 1), ("r", -1)):
                solids[f"radius-{s}"]["sections"][-1]["at"] = [0.2495, 0.004, sign * 0.008]
                solids[f"femur-{s}"]["sections"][-1]["at"][0] = 0.392
                for section in solids[f"sleeve-{s}"]["sections"][:3]:
                    section["at"][0] = SLEEVE_CAP_X.get(section["at"][0], section["at"][0])
        if scale == "100":
            solids["cervical-spine"]["sections"][-1]["at"] = [0.113, -0.002, 0]
        for index, at in SACRUM_TAIL[scale].items():
            solids["sacrum"]["sections"][index]["at"] = list(at)
        if scale in ("200", "500"):
            G.HOOPS[scale] = (RIB, 0.016, None)
            for stem, radii in SHAFTS.items():
                for name in ([stem] if stem == "spine" else [f"{stem}-l", f"{stem}-r"]):
                    solid = solids[name]
                    assert len(solid["sections"]) == len(radii), (scale, name)
                    for section, radius in zip(solid["sections"], radii):
                        section["rx"] = section["ry"] = radius
                    solid["slender"] = True
                    if scale == "500" and stem in ENDS_500:
                        solid["sections"][-1]["at"][0] = ENDS_500[stem]
            for s, sign in (("l", 1), ("r", -1)):
                for stem, x0, middle, front in (RIBS_200 if scale == "200" else RIBS_500):
                    if f"{stem}-{s}" not in solids:
                        at = next(i for i, x in enumerate(entry["structure"]) if x["id"] == f"rib-1-{s}")
                        solids[f"{stem}-{s}"] = {**solids[f"rib-1-{s}"], "id": f"{stem}-{s}"}
                        entry["structure"].insert(at + 1, solids[f"{stem}-{s}"])
                    solids[f"{stem}-{s}"]["sections"] = G.hoop(scale, x0, middle, front, sign)
                    solids[f"{stem}-{s}"]["slender"] = True
                if scale == "200":
                    girdle = solids[f"shoulder-girdle-{s}"]
                    for section in girdle["sections"]:
                        section["rx"] = section["ry"] = 0.008
                    girdle["slender"] = True
        if scale in ("50", "100", "200") and "member-hob-plate" not in solids and "hob-plate" in solids:
            at = next(i for i, x in enumerate(entry["structure"]) if x["id"] == "member-worktop")
            entry["structure"].insert(at + 1, dict(HOB_MEMBER))


def main():
    with open(G.CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    document = json.loads(text)
    with open(G.SOURCE, encoding="utf-8") as handle:
        source = json.load(handle)
    sets = [document["asset"]["volumes"], source["volumes"]]
    G.convert(document["asset"], sets)
    for volumes in sets:
        chisel(volumes)
    if "--apply" in sys.argv:
        indent = 2 if text.startswith("{\n  \"") else None
        with open(G.CATALOG, "w", encoding="utf-8") as handle:
            handle.write(json.dumps(document, ensure_ascii=False, indent=indent) + ("\n" if text.endswith("\n") else ""))
        with open(G.SOURCE, "w", encoding="utf-8") as handle:
            json.dump(source, handle, ensure_ascii=False)
        print("applied")
    else:
        os.makedirs(os.path.dirname(G.CANDIDATE), exist_ok=True)
        with open(G.CANDIDATE, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False)
        print(G.CANDIDATE)


if __name__ == "__main__":
    main()
