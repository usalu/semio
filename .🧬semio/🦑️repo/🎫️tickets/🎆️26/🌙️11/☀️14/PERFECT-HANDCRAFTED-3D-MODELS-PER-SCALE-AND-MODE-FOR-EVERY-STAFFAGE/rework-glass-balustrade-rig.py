"""🦴 Re-rigs `components/glass-balustrade` by hand: the steel shoe becomes the fixed root bone, the laminate with its handrail the cantilever bone clamped at the bolt line, with the true masses, sections and loads of the 2.0 m panel.

Without arguments it writes the candidate document `🗑️generated/rework-glass-balustrade/🔣️.json`; with `--publish` it writes the volumes source and the catalog file (then run `volumes-insert.py`).
"""
import copy
import json
import pathlib
import sys

TICKET = pathlib.Path(__file__).parent
CATALOG = TICKET.parents[6] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog" / "🧱️components" / "glass-balustrade" / "🔣️.json"
SOURCE = TICKET / "volumes" / "components" / "glass-balustrade.json"
CANDIDATE = TICKET / "🗑️generated" / "rework-glass-balustrade" / "🔣️.json"

BOLT_LINE = 0.075
SHOE_HEIGHT = 0.125
RUN = 2.0
GLASS_KG = 2 * 1.96 * 1.055 * 0.010 * 2500 - 7 * 3.14159265 * 0.013 ** 2 * 0.020 * 2500
PVB_KG = 1.96 * 1.055 * 0.0015 * 1070
HANDRAIL_KG = 6.9
PANEL_KG = round(GLASS_KG + PVB_KG + HANDRAIL_KG, 1)
PANEL_COM = round(((GLASS_KG + PVB_KG) * (0.5525 - BOLT_LINE) + HANDRAIL_KG * (1.079 - BOLT_LINE)) / (GLASS_KG + PVB_KG + HANDRAIL_KG), 4)
SHOE_PARTS = [(0.3 * 0.015 * RUN * 7850, 0.0075), (2 * 0.015 * 0.11 * RUN * 7850, 0.07), (2 * 0.01 * 0.1 * 1.96 * 1150, 0.075), (0.85, 0.075), (0.5, 0.02), (1.13, 0.018)]
SHOE_KG = round(sum(kg for kg, _ in SHOE_PARTS), 1)
SHOE_COM = round(sum(kg * at for kg, at in SHOE_PARTS) / sum(kg for kg, _ in SHOE_PARTS), 4)
SHOE_SOLIDS = {"base-plate", "cheek-fixed", "cheek-clamp", "weld-outer", "weld-inner", "pad-fixed", "pad-clamp", "setting-block", "bolt-sleeve", "bolt-shank", "bolt-head", "bolt-nut", "bolt", "anchor-nut-fixed", "anchor-nut-clamp", "anchor-stud-fixed", "anchor-stud-clamp", "anchor-fixed", "anchor-clamp", "shoe"}
SHOE_MASS_PROFILE = {"50": [[0.02, 0.03], [0.05, 0.03]], "100": [[0.015, 0.03], [0.055, 0.03]], "200": [[0.01, 0.035], [0.07, 0.035]], "500": [[0.0, 0.04], [0.075, 0.04]]}


def lower(solid):
    kind = solid["kind"]
    if kind == "extrude" and solid.get("plane") == "zx":
        solid["profile"] = [[z, round(u - BOLT_LINE, 6)] for z, u in solid["profile"]]
        if "holes" in solid:
            solid["holes"] = [[[z, round(u - BOLT_LINE, 6)] for z, u in hole] for hole in solid["holes"]]
    elif kind == "extrude" and solid.get("plane") == "xy":
        solid["profile"] = [[round(u - BOLT_LINE, 6), v] for u, v in solid["profile"]]
        if "holes" in solid:
            solid["holes"] = [[[round(u - BOLT_LINE, 6), v] for u, v in hole] for hole in solid["holes"]]
    elif kind == "revolve":
        solid["origin"] = [round(solid["origin"][0] - BOLT_LINE, 6), solid["origin"][1], solid["origin"][2]]
    else:
        raise SystemExit(f"unhandled solid {solid['id']}")


def lower_shape(item):
    if item["kind"] == "poly":
        item["points"] = [[round(u - BOLT_LINE, 6), v] for u, v in item["points"]]
    elif item["kind"] == "ellipse":
        item["cx"] = round(item["cx"] - BOLT_LINE, 6)
    elif item["kind"] == "rect":
        item["x"] = round(item["x"] - BOLT_LINE, 6)
    else:
        raise SystemExit(f"unhandled shape {item['id']} {item['kind']}")
    if "holes" in item or "solid" in item:
        raise SystemExit(f"unhandled shape member of {item['id']}")


def rebone(scale, volume):
    for solid in volume["body"]:
        if solid["id"] in SHOE_SOLIDS:
            solid["bone"] = "shoe"
        else:
            lower(solid)
    for solid in volume["thermal"]:
        if "of" not in solid:
            lower(solid)
    structure = []
    for solid in volume["structure"]:
        if solid.get("part") == "mass":
            solid["origin"] = [PANEL_COM, 0, 0]
            structure.append(solid)
            structure.append({"id": "mass-shoe", "kind": "revolve", "bone": "shoe", "material": "steel", "role": "body", "tone": 0.4, "profile": SHOE_MASS_PROFILE[scale], "axis": [1, 0, 0], "origin": [0, 0, 0], "member": "shoe", "part": "mass"})
            continue
        if solid["of"] in SHOE_SOLIDS:
            solid["member"] = "shoe"
        if solid["of"] in ("cheek-fixed", "cheek-clamp"):
            solid["part"] = "member"
            solid["id"] = "member-" + solid["of"]
        structure.append(solid)
        if solid["of"] == "shoe":
            structure.append({"id": "member-shoe", "of": "shoe", "member": "shoe", "part": "member"})
    volume["structure"] = structure
    volume["decisions"].append({
        "en": "Rig: the steel shoe is the fixed root bone on the floor; the laminate with its handrail is the cantilever bone, clamped at the bolt line 75 mm above the floor. The cheeks are the members of the shoe, the glass and the handrail those of the panel, and each bone carries its own mass marker (shoe %.1f kg, panel %.1f kg)." % (SHOE_KG, PANEL_KG),
        "de": "Rig: der Stahlschuh ist der feste Wurzelknochen auf dem Boden; das Verbundglas mit seinem Handlauf ist der Kragknochen, eingespannt in der Schraubenachse 75 mm über dem Boden. Die Wangen sind die Stäbe des Schuhs, Glas und Handlauf die des Feldes, und jeder Knochen trägt seine eigene Massenmarke (Schuh %.1f kg, Feld %.1f kg)." % (SHOE_KG, PANEL_KG),
    })


def rerig(asset):
    if any(bone["id"] == "shoe" for bone in asset["rig"]["bones"]):
        raise SystemExit("already re-rigged")
    panel = asset["rig"]["bones"][0]
    shoe = {
        "id": "shoe", "parent": None,
        "title": {"en": "Steel shoe (base plate, welded and loose cheek, fixed on the floor)", "de": "Stahlschuh (Fußplatte, angeschweißte und lose Wange, fest auf dem Boden)"},
        "x": 0, "y": 0, "angleDeg": 90, "lengthM": SHOE_HEIGHT, "joint": {"type": "fixed"},
        "mass": {"kg": SHOE_KG, "comX": SHOE_COM, "comY": 0, "distribution": "point", "source": "authored-estimate"},
        "section": {"areaM2": 0.03, "secondMomentM4": 5.625e-07, "extremeFibreM": 0.0075, "material": "steel", "title": {"en": "Welded cheek 15 mm over the 2.0 m run", "de": "Angeschweißte Wange 15 mm über 2,0 m Länge"}},
    }
    panel.update({
        "parent": "shoe", "x": BOLT_LINE, "y": 0, "angleDeg": 0, "lengthM": round(1.1 - BOLT_LINE, 4),
        "title": {"en": "Laminated glass panel with handrail (cantilever from the bolt line of the clamp)", "de": "Verbundglasfeld mit Handlauf (Kragarm ab der Schraubenachse der Klemmung)"},
        "mass": {"kg": PANEL_KG, "comX": PANEL_COM, "comY": 0, "distribution": "uniform", "source": "authored-estimate"},
        "section": {"areaM2": 0.043, "secondMomentM4": 1.656396e-06, "extremeFibreM": 0.01075, "material": "glass", "title": {"en": "21.5 mm laminate over the 2.0 m run", "de": "21,5 mm Verbundglas über 2,0 m Länge"}},
    })
    asset["rig"]["root"] = "shoe"
    asset["rig"]["bones"] = [shoe, panel]
    asset["dimensions"]["massKg"] = round(SHOE_KG + PANEL_KG, 1)
    for anchor in asset["anchors"]:
        if anchor["id"] == "base-clamp":
            anchor["bone"] = "shoe"
        else:
            anchor["x"] = round(anchor["x"] - BOLT_LINE, 4)
    for pose in asset["poses"]:
        for load in pose["loads"]:
            if load["id"] == "handrail-line":
                load["fxN"] = 1000 * RUN
                load["title"] = {"en": "Handrail line load 1.0 kN/m x 2.0 m", "de": "Handlauf-Linienlast 1,0 kN/m x 2,0 m"}
            if load["id"] == "wind-panel":
                load["distributed"].update({"fromM": round(SHOE_HEIGHT - BOLT_LINE, 4), "toM": round(1.1 - BOLT_LINE, 4), "wxNPerM": 800 * RUN})
                load["title"] = {"en": "Wind pressure 0.8 kN/m2 x 2.0 m", "de": "Winddruck 0,8 kN/m² x 2,0 m"}
        if pose["id"] == "handrail-line-load":
            pose["description"] = {"en": "A horizontal line load of 1.0 kN/m at handrail height, 2.0 kN over the 2.0 m panel.", "de": "Horizontale Linienlast 1,0 kN/m in Handlaufhöhe, 2,0 kN über das 2,0 m lange Feld."}
        if pose["id"] == "wind-pressure":
            pose["description"] = {"en": "Wind pressure of 0.8 kN/m2 on the glass above the shoe, 1.6 kN per metre of height over the 2.0 m panel, pushing outward.", "de": "Winddruck 0,8 kN/m² auf das Glas über dem Schuh, 1,6 kN je Meter Höhe über das 2,0 m lange Feld, nach außen wirkend."}
    asset["structure"]["assumptions"] = [
        {"en": "Whole 2.0 m panel: the laminate (E = 70 GPa, 21.5 mm by 2.0 m) is a cantilever clamped at the bolt line of the shoe, 75 mm above the floor; the shoe carries the clamping moment through the welded cheek into the base plate, which is the fixed support on its anchors. The handrail load is the resultant of the line load at 1.1 m.", "de": "Ganzes Feld von 2,0 m: das Verbundglas (E = 70 GPa, 21,5 mm mal 2,0 m) ist ein Kragarm, eingespannt in der Schraubenachse des Schuhs 75 mm über dem Boden; der Schuh leitet das Einspannmoment über die angeschweißte Wange in die Fußplatte, die auf ihren Ankern die Einspannung ist. Die Handlauflast ist die Resultierende der Linienlast in 1,1 m Höhe."},
        {"en": "Masses from the modelled parts: laminate 106.5 kg and handrail 6.9 kg on the panel, base plate 70.7 kg, cheeks 51.8 kg, linings and fasteners 7.0 kg on the shoe.", "de": "Massen aus den modellierten Teilen: Verbundglas 106,5 kg und Handlauf 6,9 kg am Feld, Fußplatte 70,7 kg, Wangen 51,8 kg, Zwischenlagen und Verbindungsmittel 7,0 kg am Schuh."},
        asset["structure"]["assumptions"][1],
    ]
    for representation in asset["representations"].values():
        for view, shapes in representation.get("geometry", {}).items():
            for item in shapes:
                if item.get("region") == "clamp-steel":
                    item["bone"] = "shoe"
                elif view == "elevation":
                    lower_shape(item)
    for symbol in asset["symbols"]:
        for shapes in symbol["geometry"].values():
            for item in shapes:
                item["bone"] = "shoe"


def main():
    document = json.loads(CATALOG.read_text())
    volumes = json.loads(SOURCE.read_text())
    rerig(document["asset"])
    for scale, volume in volumes["volumes"].items():
        rebone(scale, volume)
    if "--publish" in sys.argv:
        SOURCE.write_text(json.dumps(volumes, ensure_ascii=False, indent=2) + "\n")
        CATALOG.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
        return
    document["asset"]["volumes"] = copy.deepcopy(volumes["volumes"])
    CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
    CANDIDATE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print(f"shoe {SHOE_KG} kg at {SHOE_COM}, panel {PANEL_KG} kg at {PANEL_COM}")


main()
