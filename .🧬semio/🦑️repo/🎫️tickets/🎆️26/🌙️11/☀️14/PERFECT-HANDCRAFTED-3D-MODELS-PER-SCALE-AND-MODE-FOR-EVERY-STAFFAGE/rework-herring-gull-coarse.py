"""🪶 Herring gull, second round pass 1, coarse scales: two tapered humeral blades at 1:500, rounded occiput and a neck that ends inside the head at 1:200 and 1:500.

Usage: python3 rework-herring-gull-coarse.py   edits volumes/animals/herring-gull.json in place (publish with volumes-insert.py afterwards)
"""
import copy
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/animals/herring-gull.json")
VANE = [0, 0.70710678, 0, 0.70710678]
BLADES = {2: 0.112, 4: 0.122}
BLADE_PROFILE = [(0, 0.05), (0.14, 0.7), (0.32, 1), (0.62, 0.8), (0.88, 0.42), (1, 0.05)]
BLADE_WIDTH = 0.078
BLADE_THICKNESS = 0.0012
BLADE_DROP = 0.02
NECK_END = 0.068
NECK_END_RADIUS = 0.011
OCCIPUT = (0.5, 0.006, 0.9)


def blade(name, side, k, length):
    sections = [{"at": [round(length * f, 5), BLADE_DROP, 0], "rx": BLADE_THICKNESS / 2, "ry": round(max(BLADE_WIDTH / 2 * g, BLADE_THICKNESS / 2), 5), "exponent": 2, "rotation": VANE} for f, g in BLADE_PROFILE]
    return {"id": f"{name}-tertial-{k}", "kind": "loft", "bone": f"t{k}-{side}", "material": "wing-feathers", "role": "body", "tone": 0.62, "sections": sections, "capStart": True, "capEnd": True, "sheet": True}


def main():
    source = json.load(open(SOURCE, encoding="utf8"))
    lists = source["volumes"]["500"]
    if any(solid["id"] == "near-tertial-3" for solid in lists["body"]):
        for name, side in (("near", "n"), ("far", "f")):
            index = next(i for i, solid in enumerate(lists["body"]) if solid["id"] == f"{name}-tertial-3")
            lists["body"][index:index + 1] = [blade(name, side, k, length) for k, length in BLADES.items()]
            index = next(i for i, entry in enumerate(lists["thermal"]) if entry["id"] == f"th-{name}-tertial-3")
            pattern = lists["thermal"][index]
            entries = []
            for k in BLADES:
                entry = copy.deepcopy(pattern)
                entry["id"], entry["of"] = f"th-{name}-tertial-{k}", f"{name}-tertial-{k}"
                entries.append(entry)
            lists["thermal"][index:index + 1] = entries
        lists["decisions"][0]["en"] = lists["decisions"][0]["en"].replace("One humeral blade 80 mm wide per wing on the middle humeral feather bone closes", "Two tapered humeral blades 78 mm wide per wing on the second and fourth humeral feather bones close")
        lists["decisions"][0]["de"] = lists["decisions"][0]["de"].replace("Eine 80 mm breite Schirmfederfläche je Flügel auf dem mittleren Schirmfederknochen schließt", "Zwei zugespitzte, 78 mm breite Schirmfederflächen je Flügel auf dem zweiten und vierten Schirmfederknochen schließen")
    for scale in ("200", "500"):
        for list_name in ("body", "thermal"):
            for solid in source["volumes"][scale][list_name]:
                if solid.get("kind") != "loft":
                    continue
                sections = solid["sections"]
                if solid["bone"] == "neck-2" and sections[-1]["at"][0] > NECK_END:
                    kept = [section for section in sections if section["at"][0] < NECK_END - 0.004]
                    last = copy.deepcopy(sections[-1])
                    last["at"][0], last["rx"], last["ry"] = NECK_END, min(last["rx"], NECK_END_RADIUS), min(last["ry"], NECK_END_RADIUS)
                    solid["sections"] = kept + [last]
                    print("[DEBUG] neck end", scale, list_name, solid["id"], [(s["at"][0], s["rx"]) for s in solid["sections"]])
                if solid["bone"] == "head" and solid["material"] != "bill-keratin" and sections[0]["rx"] >= 0.009 and sections[0]["at"][0] < 0:
                    shrink, step, swell = OCCIPUT
                    first, second = sections[0], sections[1]
                    ring = copy.deepcopy(first)
                    fraction = step / (second["at"][0] - first["at"][0])
                    ring["at"] = [round(first["at"][i] + (second["at"][i] - first["at"][i]) * fraction, 5) for i in range(3)]
                    ring["rx"], ring["ry"] = round(max(first["rx"] * swell, first["rx"] + (second["rx"] - first["rx"]) * fraction), 5), round(max(first["ry"] * swell, first["ry"] + (second["ry"] - first["ry"]) * fraction), 5)
                    first["rx"], first["ry"] = round(first["rx"] * shrink, 5), round(first["ry"] * shrink, 5)
                    if fraction < 0.8:
                        sections.insert(1, ring)
                    print("[DEBUG] occiput", scale, list_name, solid["id"], [(s["at"][0], s["rx"], s["ry"]) for s in sections])
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")


if __name__ == "__main__":
    main()
