"""🪟 Hand-authored volume lists of `components/glass-balustrade` for 1:100, 1:200 and 1:500 and the structure fix of 1:50.

Every number is chosen by hand from the reference table in `rework/assets/components--glass-balustrade.md`.
Frame of the bone `panel`: x = height, y = across the glass (thickness), z = along the run (2.0 m).
"""
import json
import math
import pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes" / "components" / "glass-balustrade.json"
RAIL_X, RAIL_R, GLASS_HALF = 1.079, 0.0212, 0.01075
STEEL, GLASS = "steel", "glass"


def arc(start_deg, end_deg, step_deg):
    count = round(abs(end_deg - start_deg) / step_deg)
    return [[round(RAIL_X + RAIL_R * math.cos(math.radians(start_deg + (end_deg - start_deg) * i / count)), 6),
             round(RAIL_R * math.sin(math.radians(start_deg + (end_deg - start_deg) * i / count)), 6)] for i in range(count + 1)]


def solid(identifier, kind, material, role, tone, **fields):
    return {"id": identifier, "kind": kind, "bone": "panel", "material": material, "role": role, "tone": tone, **fields}


def base_plate():
    return solid("base-plate", "extrude", STEEL, "body", 0.32, plane="yz",
                 profile=[[-0.15, -1.0], [0.15, -1.0], [0.15, 1.0], [-0.15, 1.0]], **{"from": 0, "to": 0.015})


def jaw(identifier, near, far):
    return solid(identifier, "extrude", STEEL, "body", 0.38, plane="zx",
                 profile=[[-1.0, 0.015], [1.0, 0.015], [1.0, 0.125], [-1.0, 0.125]], **{"from": near, "to": far})


def laminate(sheet):
    part = solid("laminate", "extrude", GLASS, "glazing", 0.92, plane="zx",
                 profile=[[-0.98, 0.025], [0.98, 0.025], [0.98, 1.08], [-0.98, 1.08]], **{"from": -GLASS_HALF, "to": GLASS_HALF})
    if sheet:
        part["sheet"] = True
    return part


def clamp_bolt():
    return solid("bolt", "revolve", STEEL, "detail", 0.22, axis=[0, 1, 0], origin=[0.075, 0, -0.9],
                 profile=[[-0.04325, 0.0], [-0.04325, 0.011], [-0.03575, 0.011], [-0.03575, 0.006], [0.03575, 0.006],
                          [0.03575, 0.011], [0.04655, 0.011], [0.04655, 0.0]],
                 repeat={"count": 7, "translation": [0, 0, 0.3]})


def anchor(identifier, lateral):
    return solid(identifier, "revolve", STEEL, "detail", 0.22, axis=[1, 0, 0], origin=[0, lateral, -0.8],
                 profile=[[0.0, 0.0], [0.0, 0.006], [0.015, 0.006], [0.015, 0.011], [0.0258, 0.011], [0.0258, 0.006],
                          [0.034, 0.006], [0.034, 0.0]],
                 repeat={"count": 5, "translation": [0, 0, 0.4]})


def slotted_rail():
    slot_deg = math.degrees(math.asin(GLASS_HALF / RAIL_R))
    outline = arc(180 - slot_deg, -180 + slot_deg, 15.0)
    return solid("handrail-tube", "extrude", STEEL, "body", 0.45, plane="xy",
                 profile=outline + [[1.08, -GLASS_HALF], [1.08, GLASS_HALF]], **{"from": -1.0, "to": 1.0})


def round_rail(slender):
    part = solid("handrail-tube", "revolve", STEEL, "body", 0.45, axis=[0, 0, 1], origin=[RAIL_X, 0, -1.0],
                 profile=[[0.0, 0.0], [0.0, RAIL_R], [2.0, RAIL_R], [2.0, 0.0]])
    if slender:
        part["slender"] = True
    return part


def shoe():
    return solid("shoe", "extrude", STEEL, "body", 0.35, plane="xy",
                 profile=[[0.0, -0.15], [0.015, -0.15], [0.015, -0.03575], [0.125, -0.03575], [0.125, 0.03575],
                          [0.015, 0.03575], [0.015, 0.15], [0.0, 0.15]], **{"from": -1.0, "to": 1.0})


def mass(half, radius):
    return {"id": "mass-panel", "kind": "revolve", "bone": "panel", "material": GLASS, "role": "body", "tone": 0.5,
            "profile": [[-half, radius], [half, radius]], "axis": [0, 1, 0], "origin": [0.55, 0, 0], "member": "panel", "part": "mass"}


def thermal(pairs):
    return [{"id": "th-" + identifier, "of": identifier, "region": region} for identifier, region in pairs]


def structure(rows, marker):
    out = []
    for part, identifier in rows:
        row = {"id": part + "-" + identifier, "of": identifier, "member": "panel", "part": part}
        if part == "support":
            row["anchor"] = "base-clamp"
        out.append(row)
    return out + [marker]


DECISIONS = {
    "50": [
        {"en": "Smallest feature 7.5 mm: every real part is its own solid. The 300 x 15 mm base plate with ten 14 mm anchor holes, the welded fixed cheek and the loose clamping cheek (15 x 110 mm), two 10 mm elastomer linings, two setting blocks, the laminate as two drilled 10 mm panes with the 1.5 mm PVB interlayer, seven sleeved M12 through bolts with hexagon head and nut at 300 mm, two rows of five M12 anchor studs with nuts at 400 mm, and the slotted 42.4 x 2 mm tube with its U gasket and two domed end caps.",
         "de": "Kleinstes Merkmal 7,5 mm: jedes wirkliche Teil ist ein eigener Körper. Die Fußplatte 300 x 15 mm mit zehn Ankerlöchern 14 mm, die angeschweißte feste Wange und die lose Klemmwange (15 x 110 mm), zwei Elastomerlagen 10 mm, zwei Klotzungen, das Verbundglas als zwei gebohrte 10-mm-Scheiben mit 1,5 mm PVB-Folie, sieben Durchgangsschrauben M12 mit Hülse, Sechskantkopf und Mutter im Abstand von 300 mm, zwei Reihen mit je fünf Ankerbolzen M12 mit Muttern im Abstand von 400 mm und das geschlitzte Rohr 42,4 x 2 mm mit U-Dichtung und zwei gewölbten Endkappen."},
        {"en": "Thermal: the shoe with linings and fasteners is the clamp region, the laminate the glass region, the tube is closed to a bar so that its inside does not count as outer surface. Structure: handrail, gasket, the two panes coupled by the interlayer, linings and cheeks as the clamping couple, through bolts, welds, base plate as the fixed support and the anchors into the floor.",
         "de": "Thermik: der Schuh mit Zwischenlagen und Verbindungsmitteln ist die Klemmregion, das Verbundglas die Glasregion, das Rohr ist zum Stab geschlossen, damit sein Inneres nicht als Außenfläche zählt. Tragwerk: Handlauf, Dichtung, die beiden über die Folie gekoppelten Scheiben, Zwischenlagen und Wangen als Klemmkräftepaar, Durchgangsschrauben, Schweißnähte, Fußplatte als Einspannung und die Anker in den Boden."}],
    "100": [
        {"en": "Smallest feature 15 mm: the 15 mm base plate stays a plate without the 14 mm holes; each cheek is merged with its 10 mm lining and its weld into one 25 mm clamping jaw; the two panes and the interlayer are one 21.5 mm laminate; each through bolt is one turned solid with 22 mm head and nut, each anchor one turned stud with its nut; tube, gasket and end caps are one round 42.4 mm bar with the glass slot. Setting blocks, sleeves and bolt holes are hidden inside and left out.",
         "de": "Kleinstes Merkmal 15 mm: die 15-mm-Fußplatte bleibt eine Platte ohne die 14-mm-Löcher; jede Wange ist mit ihrer 10-mm-Zwischenlage und ihrer Naht zu einer 25-mm-Klemmbacke verschmolzen; die beiden Scheiben und die Folie sind ein Verbundglas von 21,5 mm; jede Durchgangsschraube ist ein Drehkörper mit Kopf und Mutter von 22 mm, jeder Anker ein gedrehter Bolzen mit Mutter; Rohr, Dichtung und Endkappen sind ein runder Stab 42,4 mm mit dem Glasschlitz. Klotzungen, Hülsen und Bohrungen liegen verdeckt und entfallen."},
        {"en": "Thermal and structure use the same solids: jaws, bolts, anchors and plate are the clamp region and the clamping joint on the fixed support, laminate and handrail are the two members.",
         "de": "Thermik und Tragwerk nutzen dieselben Körper: Backen, Schrauben, Anker und Platte sind die Klemmregion und der Klemmanschluss auf der Einspannung, Verbundglas und Handlauf sind die beiden Stäbe."}],
    "200": [
        {"en": "Smallest feature 30 mm: the shoe is one inverted-T section (300 x 15 mm flange, 71.5 x 110 mm upstand) drawn in one piece along the 2.0 m run; the laminate is a declared sheet at its true 21.5 mm; the handrail is a solid round 42.4 mm bar without the 21.5 mm slot. Bolts, anchors and nuts (22 mm) are below the feature and left out.",
         "de": "Kleinstes Merkmal 30 mm: der Schuh ist ein umgekehrtes T-Profil (Flansch 300 x 15 mm, Steg 71,5 x 110 mm) in einem Stück über die 2,0 m Länge; das Verbundglas ist eine ausgewiesene Scheibe in wahrer Dicke 21,5 mm; der Handlauf ist ein voller Rundstab 42,4 mm ohne den 21,5-mm-Schlitz. Schrauben, Anker und Muttern (22 mm) liegen unter dem Merkmal und entfallen."},
        {"en": "Thermal: shoe, glass and handrail are the three regions. Structure: the handrail bar on the glass cantilever, clamped in the shoe, which is the fixed support.",
         "de": "Thermik: Schuh, Glas und Handlauf sind die drei Regionen. Tragwerk: der Handlaufstab auf dem Glaskragarm, eingespannt im Schuh, der die Einspannung ist."}],
    "500": [
        {"en": "Smallest feature 75 mm: three solids. The shoe as one inverted-T section, the laminate as a declared sheet at its true 21.5 mm and the handrail as a declared slender round bar at its true 42.4 mm; nothing is thickened, so the balustrade keeps its real transparency and slenderness in a model at this scale.",
         "de": "Kleinstes Merkmal 75 mm: drei Körper. Der Schuh als ein umgekehrtes T-Profil, das Verbundglas als ausgewiesene Scheibe in wahrer Dicke 21,5 mm und der Handlauf als ausgewiesener schlanker Rundstab in wahrer Dicke 42,4 mm; nichts ist verdickt, damit die Brüstung im Modell dieses Maßstabs ihre wirkliche Durchsicht und Schlankheit behält."},
        {"en": "Minimal: shoe, laminate and handrail are the three parts a glass balustrade consists of, so the lists equal those of 1:200 in count; they are the three thermal regions and, in the structure, the handrail on the glass cantilever clamped in the shoe as the fixed support.",
         "de": "Minimal: Schuh, Verbundglas und Handlauf sind die drei Teile, aus denen eine Glasbrüstung besteht, daher gleichen die Listen in der Anzahl denen von 1:200; sie sind die drei thermischen Regionen und im Tragwerk der Handlauf auf dem Glaskragarm, eingespannt im Schuh als Einspannung."}],
}


def main():
    document = json.loads(SOURCE.read_text())
    volumes = document["volumes"]

    fine = volumes["50"]
    present = {row["id"] for row in fine["structure"]}
    extra = [("joint-gasket", "handrail-gasket"), ("joint-interlayer", "interlayer"), ("joint-bolt-sleeve", "bolt-sleeve")]
    marker = [row for row in fine["structure"] if row["part"] == "mass"]
    fine["structure"] = [row for row in fine["structure"] if row["part"] != "mass"] + [
        {"id": identifier, "of": of, "member": "panel", "part": "joint"} for identifier, of in extra if identifier not in present] + marker
    fine["decisions"] = DECISIONS["50"]

    volumes["100"].update({
        "body": [base_plate(), jaw("cheek-fixed", -0.03575, -GLASS_HALF), jaw("cheek-clamp", GLASS_HALF, 0.03575), laminate(False),
                 clamp_bolt(), anchor("anchor-fixed", -0.11), anchor("anchor-clamp", 0.11), slotted_rail()],
        "thermal": thermal([("base-plate", "clamp-steel"), ("cheek-fixed", "clamp-steel"), ("cheek-clamp", "clamp-steel"),
                            ("bolt", "clamp-steel"), ("anchor-fixed", "clamp-steel"), ("anchor-clamp", "clamp-steel"),
                            ("laminate", "panel-glass"), ("handrail-tube", "handrail-steel")]),
        "structure": structure([("member", "laminate"), ("member", "handrail-tube"), ("joint", "cheek-fixed"), ("joint", "cheek-clamp"),
                                ("joint", "bolt"), ("joint", "anchor-fixed"), ("joint", "anchor-clamp"), ("support", "base-plate")],
                               mass(0.02075, 0.075)),
        "decisions": DECISIONS["100"]})

    for scale, slender, marker_size in (("200", False, (0.03075, 0.09)), ("500", True, (0.04, 0.12))):
        volumes[scale].update({
            "body": [shoe(), laminate(True), round_rail(slender)],
            "thermal": thermal([("shoe", "clamp-steel"), ("laminate", "panel-glass"), ("handrail-tube", "handrail-steel")]),
            "structure": structure([("member", "laminate"), ("member", "handrail-tube"), ("support", "shoe")], mass(*marker_size)),
            "decisions": DECISIONS[scale]})

    SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
