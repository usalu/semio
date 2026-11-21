"""🍞 Round 5 toaster: keeps the hand-stated lists of `r3-toaster-model.py` and restates by hand what the round-5 look found: the 1:200 slots 1 mm wider than the 30 mm slices on each side (they were coplanar and flickered), the 1:200 slot-air outlet as four flush patches in the open slot ends instead of one plate across deck and bread, the 1:500 housing as five rounded sections instead of four chamfered ones and its slot air as two end patches; writes `volumes/appliances/toaster.json`."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model3", os.path.join(HERE, "r3-toaster-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
K = M.K

HOUSING_500 = [0.0, 0.15, 0.168, 0.181, K.TOP]


def end_patches(sid, half_x, length, depth, at_z, pairs):
    """💨 Flush outlet patches (D2) at both ends of a slot opening: one solid, repeated across x and, when pairs, across the two slots."""
    s = K.box(sid, "base", M.DARK, "detail", 0.4, (length, 0.001, depth), (-half_x, K.TOP - 0.0004, at_z), tags=["slot-air"], sectionFaces=M.ALL_FACES, repeat={"count": 2, "translation": [K.r5(2 * half_x), 0, 0]})
    return M.th(s, "top-plate", "plate")


def scale200():
    body, thermal, structure = M.scale200()
    body[1] = K.extrude("top-plate", "base", M.DARK, "body", 0.4, "zx", K.deck_outline(6), 0.158, K.TOP + 0.0005, holes=[K.slot(-1, 0.001, 2), K.slot(1, 0.001, 2)])
    thermal = [x for x in thermal if x["id"] != "th-slot-air"]
    gap = (2 * K.SLOT_HX - 0.104) / 2
    thermal.append(end_patches("th-slot-air-front", K.SLOT_HX - gap / 2, gap, K.SLOT_W, -K.SLOT_Z, False))
    thermal.append(end_patches("th-slot-air-rear", K.SLOT_HX - gap / 2, gap, K.SLOT_W, K.SLOT_Z, False))
    return body, thermal, structure


def scale500():
    body, thermal, structure = M.scale500()
    rows = [(y, K.hx(max(y, 0.04)), K.hz(max(y, 0.04))) for y in HOUSING_500]
    body[0] = K.stack("casing", "base", M.CHROME, "body", 0.82, rows)
    thermal = [x for x in thermal if x["id"] != "th-slot-air"]
    thermal.append(end_patches("th-slot-air", 0.058, 0.012, 0.09, 0.0, False))
    return body, thermal, structure


def main():
    texts = json.load(open(os.path.join(HERE, "r3-toaster-texts.json"), encoding="utf-8"))["volumes"]
    texts.update(json.load(open(os.path.join(HERE, "r5-toaster-texts.json"), encoding="utf-8"))["volumes"])
    volumes = {}
    for den, make in (("50", M.scale50), ("100", M.scale100), ("200", scale200), ("500", scale500)):
        body, thermal, structure = make()
        volumes[den] = {"status": "authored", "denominator": int(den), "body": body, "thermal": thermal, "structure": structure, "decisions": texts[den]}
        print(den, len(body), len(thermal), len(structure))
    with open(os.path.join(HERE, "volumes", "appliances", "toaster.json"), "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")


if __name__ == "__main__":
    main()
