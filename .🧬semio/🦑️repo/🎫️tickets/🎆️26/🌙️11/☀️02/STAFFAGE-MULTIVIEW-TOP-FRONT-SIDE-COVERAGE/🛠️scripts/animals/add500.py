import json, copy


def T(en, de):
    return {"en": en, "de": de}


def load(p):
    return json.load(open(p, encoding="utf-8-sig"))


def dump(p, d):
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def rect(id, bone, region, x, y, w, h, tone, mat, z=3, regions=None, role="body"):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "kind": "rect", "x": x, "y": y, "w": w, "h": h}
    if regions:
        s["regions"] = regions
    else:
        s["region"] = region
    s["material"] = mat
    return s


def rack():
    p = "server-rack/🔣️.json"
    d = load(p)
    a = d["asset"]
    cab = ["cabinet", "door", "feet"]
    el = [
        rect("cabinet", "frame", None, -0.3, 0, 0.6, 2.0, 0.5, "rack-steel", 3, cab),
        rect("server-block", "equipment", "servers", -0.2, 0.1, 0.4, 0.7112, 0.42, "chassis", 3.2),
        rect("top-group", "equipment", None, -0.2, 1.83355, 0.4, 0.13335, 0.4, "chassis", 3.2, ["switch", "blanks"]),
    ]
    fr = [rect("cabinet", "frame", None, -0.5, 0, 1.0, 2.0, 0.5, "rack-steel", 3, cab)]
    pl = [rect("roof", "frame", None, -0.3, -0.5, 0.6, 1.0, 0.45, "rack-steel", 3, cab)]
    a["representations"]["500"] = {
        "status": "authored",
        "denominator": 500,
        "validRange": {"minDenominator": 350, "maxDenominator": 800},
        "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
        "hatches": [],
        "geometry": {"elevation": el, "front": fr, "plan": pl},
        "decisions": {
            "silhouette": T(
                "A tower 1.2 mm wide and 4 mm tall on paper: one frame block with a dark server block at the bottom and a smaller top group above it, the door field left as open frame between them. Front view: one block 2 mm wide and 4 mm tall. Top view: one 1.2 by 2 mm roof block.",
                "Ein Turm von 1,2 mm Breite und 4 mm Höhe auf Papier: ein Rahmenblock mit einem dunklen Serverblock unten und einer kleineren oberen Gruppe darüber, das Türfeld bleibt dazwischen offener Rahmen. Vorderansicht: ein Block von 2 mm Breite und 4 mm Höhe. Draufsicht: ein Dachblock von 1,2 mal 2 mm.",
            ),
            "retained": T(
                "Height 2.0 m, width 0.6 m, depth 1.0 m, the frame as one block, the 16U server block and the top group of switch and blanking panels.",
                "Höhe 2,0 m, Breite 0,6 m, Tiefe 1,0 m, der Rahmen als ein Block, der 16-HE-Serverblock und die obere Gruppe aus Switch und Blindblenden.",
            ),
            "merged": T(
                "Cabinet, door and casters are one block (regions cabinet, door, feet); the equipment blocks are drawn 0.4 m wide so that the 0.1 m gap to the posts stays at the 0.2 mm gap of the print profile.",
                "Gehäuse, Tür und Rollen sind ein Block (Regionen Gehäuse, Tür, Füße); die Geräteblöcke sind 0,4 m breit gezeichnet, damit der Abstand von 0,1 m zu den Pfosten die 0,2 mm Lücke des Druckprofils hält.",
            ),
            "omitted": T(
                "Corner posts, perforation, unit joints, casters and the door edge (all below 0.1 m, 0.2 mm) in all three views; the equipment is hidden behind the closed door in front and top view.",
                "Eckpfosten, Lochung, Einheitenfugen, Rollen und Türkante (alle unter 0,1 m, 0,2 mm) in allen drei Ansichten; die Geräte sind in Vorder- und Draufsicht hinter der geschlossenen Tür verdeckt.",
            ),
            "thinElements": T(
                "None are drawn; no line is thinner than the 0.13 mm body pen.",
                "Keine werden gezeichnet; keine Linie ist dünner als der Körperstift von 0,13 mm.",
            ),
            "hatching": T("None.", "Keine."),
            "styles": T(
                "Line drawing with one filled frame block and filled equipment blocks; the thermal view paints the merged regions with their area-weighted values.",
                "Strichzeichnung mit einem gefüllten Rahmenblock und gefüllten Geräteblöcken; die Wärmeansicht färbt die zusammengefassten Regionen mit ihren flächengewichteten Werten.",
            ),
            "overlays": T(
                "Dimension lines stand outside the block; the exhaust plume starts at the roof in side and front view and at the roof centre in top view.",
                "Maßlinien stehen außerhalb des Blocks; die Abluftfahne beginnt am Dach in Seiten- und Vorderansicht und in der Dachmitte in der Draufsicht.",
            ),
        },
    }
    dump(p, d)


def vac():
    p = "vacuum-cleaner/🔣️.json"
    d = load(p)
    a = d["asset"]
    r = copy.deepcopy(a["representations"]["200"])
    r["denominator"] = 500
    r["validRange"] = {"minDenominator": 350, "maxDenominator": 800}
    r["pens"] = {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}
    for v, shapes in list(r["geometry"].items()):
        keep = [s for s in shapes if s["role"] == "body"]
        for s in keep:
            if s["id"].startswith(("hose-", "wand-", "grip-")):
                if s["kind"] == "capsule":
                    s["r0"] = s["r1"] = 0.05
                if "solid" in s:
                    s["solid"]["z0"], s["solid"]["z1"] = -0.05, 0.05
        r["geometry"][v] = keep
    r["decisions"] = {
        "silhouette": T(
            "A canister vacuum cleaner 3 mm long: one canister block, a wheel disc, ten hose segments, a wand rod with grip and one wedge-shaped nozzle, each segment bound to its bone so the stroke and the park pose move in all views. Front view: the body block between two wheels, then hose, wand and grip bars and the nozzle block. Top view: the body, two wheel strips, the centre-line hose, wand and grip bars and the nozzle block.",
            "Ein Bodenstaubsauger von 3 mm Länge: ein Kesselblock, eine Radscheibe, zehn Schlauchglieder, ein Rohr mit Griff und eine keilförmige Düse, jedes Glied an seinen Knochen gebunden, sodass Zug und Parkpose in allen Ansichten mitgehen. Vorderansicht: der Körperblock zwischen zwei Rädern, davor Schlauch-, Rohr- und Griffbalken und der Düsenblock. Draufsicht: der Körper, zwei Radstreifen, Schlauch-, Rohr- und Griffbalken auf der Mittellinie und der Düsenblock.",
        ),
        "retained": T(
            "Length 1.5 m, height 0.76 m, width 0.3 m, wand length 0.78 m, the ten hose segments, the nozzle wedge and the two wheels.",
            "Länge 1,5 m, Höhe 0,76 m, Breite 0,3 m, Rohrlänge 0,78 m, die zehn Schlauchglieder, der Düsenkeil und die beiden Räder.",
        ),
        "merged": T(
            "Shell, lid, handle and grille form one block (regions listed); nozzle and brush are one wedge.",
            "Schale, Deckel, Griff und Gitter bilden einen Block (Regionen gelistet); Düse und Bürste sind ein Keil.",
        ),
        "omitted": T(
            "Corrugation, hub, caster, seams, louvres and buttons, and every detail below 0.1 m (0.2 mm).",
            "Wellung, Nabe, Lenkrolle, Fugen, Lamellen und Tasten sowie jedes Detail unter 0,1 m (0,2 mm).",
        ),
        "thinElements": T(
            "Hose, wand and grip are drawn 0.1 m (0.2 mm) wide, about twice their real diameter of 0.048 to 0.06 m, so they reach the 0.2 mm feature width of the office laser profile; their length and joints are true to scale.",
            "Schlauch, Rohr und Griff sind 0,1 m (0,2 mm) breit gezeichnet, etwa doppelt so breit wie ihr wirklicher Durchmesser von 0,048 bis 0,06 m, damit sie die Merkmalsbreite von 0,2 mm des Büro-Laser-Profils erreichen; Länge und Gelenke stimmen maßstäblich.",
        ),
        "hatching": T("None.", "Keine."),
        "styles": T(
            "Line drawing with filled blocks; the thermal view paints the merged regions with their area-weighted values.",
            "Strichzeichnung mit gefüllten Blöcken; die Wärmeansicht färbt die zusammengefassten Regionen mit ihren flächengewichteten Werten.",
        ),
        "overlays": T(
            "Dimension lines stand outside the drawing; the exhaust plume starts behind the canister in side and top view and at the body centre in front view.",
            "Maßlinien stehen außerhalb der Zeichnung; die Abluftfahne beginnt hinter dem Kessel in Seiten- und Draufsicht und in Körpermitte in der Vorderansicht.",
        ),
    }
    a["representations"]["500"] = r
    dump(p, d)


rack()
vac()
