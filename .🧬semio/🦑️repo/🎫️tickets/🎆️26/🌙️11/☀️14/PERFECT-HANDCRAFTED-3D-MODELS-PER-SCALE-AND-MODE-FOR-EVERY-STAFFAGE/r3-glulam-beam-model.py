"""🪵 Round 3 glulam beam: writes the handcrafted `volumes` (body, thermal, structure, decisions at 1:50, 1:100, 1:200, 1:500)
to `volumes/components/glulam-beam.json`.

Every solid is stated here; the lamella lay-up (board outlines with their finger joints) comes from `r3-glulam-beam-kit.py`,
which the 2D root script shares, so the drawn joints and the modelled board seams are one table.

See: r3-components-glulam-beam.md, round3-brief.md, design.md
"""

import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-glulam-beam-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
r6 = k.r6
L, H, W = k.LENGTH, k.HALF_DEPTH, k.HALF_WIDTH
PL, PT = k.PLATE
XL, XR = k.BEARINGS
SKIN = 0.036
BOARD_TONE = [0.0, 0.03, -0.02]


def solid(id, kind, material, tone, bone="beam", role="body", **rest):
    """🧱 A solid of a bone of the beam."""
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **rest}


def box(id, material, tone, size, centre, **rest):
    """📦 A box by size and centre."""
    return solid(id, "box", material, tone, size=[r6(v) for v in size], transform={"translation": [r6(v) for v in centre]}, **rest)


def xy(id, material, tone, profile, z0=-W, z1=W, **rest):
    """📐 A profile in the bone plane (u, v) pushed across the width."""
    return solid(id, "extrude", material, tone, profile=[[r6(a), r6(b)] for a, b in profile], plane="xy", **{"from": r6(z0), "to": r6(z1)}, **rest)


def yz(id, material, tone, profile, x0=0.0, x1=L, **rest):
    """📏 A cross-section (v, w) pushed along the beam."""
    return solid(id, "extrude", material, tone, profile=[[r6(a), r6(b)] for a, b in profile], plane="yz", **{"from": r6(x0), "to": r6(x1)}, **rest)


def roller(id, diameter, x, **rest):
    """🛞 A steel roller of the given diameter across the width, standing on the ground under x."""
    r = diameter / 2
    return solid(id, "revolve", "steel", 0.4, profile=[[0.0, 0.0], [0.0, r6(r)], [r6(2 * W), r6(r)], [r6(2 * W), 0.0]], axis=[0, 0, 1], origin=[r6(x), r6(-H - PT + r), r6(-W)], **rest)


def ref(id, of, **rest):
    """🔗 A reference to a body solid."""
    return {"id": id, "of": of, **rest}


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def plates(height=PT):
    """🔩 The two steel bearing plates 120 x 160 mm under the square ends, standing on the ground."""
    y = -H - PT + height / 2
    return [box("bearing-plate-left", "steel", 0.3, [PL, height, 2 * W], [XL, y, 0]), box("bearing-plate-right", "steel", 0.3, [PL, height, 2 * W], [XR, y, 0])]


def lamella_tone(lam, board=0):
    """🎨 Alternating lamella tone so the glue lines read, each board a shade apart as sawn boards are."""
    return r6((0.56 if lam % 2 == 0 else 0.62) + BOARD_TONE[board % len(BOARD_TONE)])


def body50():
    """🪚 1:50: every board of the fifteen lamellae with its finger-jointed ends, and the plates."""
    out = []
    for lam in range(k.COUNT):
        for i, outline in enumerate(k.boards(lam)):
            out.append(xy(f"lamella-{lam:02d}-board-{'abc'[i]}", "glulam", lamella_tone(lam, i), outline))
    return out + plates()


def body100():
    """📚 1:100: the fifteen lamellae over the full length (board joints merged), and the plates."""
    out = []
    for lam in range(k.COUNT):
        y0, y1 = k.bottom(lam), k.bottom(lam) + k.LAMELLA
        out.append(yz(f"lamella-{lam:02d}", "glulam", lamella_tone(lam), [[y0, -W], [y1, -W], [y1, W], [y0, W]]))
    return out + plates()


def skins(split_soffit):
    """🌡️ The four timber faces as skins of the 36 mm storage depth; the soffit split at the bearing contacts when asked."""
    side = [8.0, 2 * H, SKIN]
    flat = [8.0, SKIN, 2 * W]
    out = [
        box("th-side-l", "glulam", 0.6, side, [L / 2, 0, W - SKIN / 2], region="beam-sides", sectionFaces=["-w", "-v", "+v"]),
        box("th-side-r", "glulam", 0.6, side, [L / 2, 0, -W + SKIN / 2], region="beam-sides", sectionFaces=["+w", "-v", "+v"]),
        box("th-top", "glulam", 0.6, flat, [L / 2, H - SKIN / 2, 0], region="beam-top", sectionFaces=["-v", "-w", "+w"]),
    ]
    y = -H + SKIN / 2
    face = ["+v", "-w", "+w"]
    if split_soffit:
        out += [
            box("th-soffit-bearing-left", "glulam", 0.6, [PL, SKIN, 2 * W], [XL, y, 0], region="beam-soffit", sectionFaces=face),
            box("th-soffit-span", "glulam", 0.6, [L - 2 * PL, SKIN, 2 * W], [L / 2, y, 0], region="beam-soffit", sectionFaces=face),
            box("th-soffit-bearing-right", "glulam", 0.6, [PL, SKIN, 2 * W], [XR, y, 0], region="beam-soffit", sectionFaces=face),
        ]
    else:
        out.append(box("th-soffit", "glulam", 0.6, flat, [L / 2, y, 0], region="beam-soffit", sectionFaces=face))
    return out + [ref("th-bearing-plate-left", "bearing-plate-left", region="bearing-steel"), ref("th-bearing-plate-right", "bearing-plate-right", region="bearing-steel")]


def wedges(bottom_v):
    """🔺 The section from bottom_v to the top shared among its four faces: each face owns the triangle from its edge to the section centre."""
    c = (H + bottom_v) / 2
    b, t = bottom_v, H
    return [
        yz("th-side-l", "glulam", 0.6, [[b, W], [t, W], [c, 0]], region="beam-sides", sectionFaces=["-w"]),
        yz("th-side-r", "glulam", 0.6, [[t, -W], [b, -W], [c, 0]], region="beam-sides", sectionFaces=["+w"]),
        yz("th-top", "glulam", 0.6, [[t, W], [t, -W], [c, 0]], region="beam-top", sectionFaces=["-w", "+w"]),
        yz("th-soffit", "glulam", 0.6, [[b, -W], [b, W], [c, 0]], region="beam-soffit", sectionFaces=["-w", "+w"]),
    ]


def member():
    """🏗️ The beam member: the real 160 x 600 mm GL24h section over the full 8 m."""
    return box("beam-member", "glulam", 0.6, [L, 2 * H, 2 * W], [L / 2, 0, 0], member="beam", part="member")


def pin(height, pad):
    """📌 The pin bearing under the left plate centre: a knife edge of the given height, on a pad of the given thickness when pad > 0."""
    g = -H - PT
    if pad > 0:
        profile = [[XL - PL / 2, g], [XL + PL / 2, g], [XL + PL / 2, g + pad], [XL + 0.015, g + pad], [XL, g + height], [XL - 0.015, g + pad], [XL - PL / 2, g + pad]]
    else:
        profile = [[XL - PL / 2, g], [XL + PL / 2, g], [XL, g + height]]
    return xy("support-bearing-left-pin", "steel", 0.4, profile, member="beam", part="support", anchor="bearing-left")


def slider():
    """🎚️ The prismatic joint of the moving load: a 100 mm load shoe flush with the top face."""
    return box("joint-load-carrier", "steel", 0.3, [0.1, 0.03, 2 * W], [0, H - 0.015, 0], bone="load-carrier", member="load-carrier", part="joint")


def scale(n, body, thermal, structure, decisions):
    """🗂️ One authored scale."""
    return {"status": "authored", "denominator": n, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def build():
    """🧮 All four scales."""
    v50 = scale(
        50,
        body50(),
        skins(True),
        [member(), pin(PT, 0.01), roller("support-bearing-right-roller", PT, XR, member="beam", part="support", anchor="bearing-right"), slider()],
        [
            text(
                "Body: the beam as it is made: each of the fifteen 40 mm lamellae is a run of sawn boards (40 solids) joined by the 25 finger joints of the lay-up, one or two per lamella, staggered and kept out of the middle third of the bottom tension lamella; every joint is a two-finger profile of 20 mm finger length at 20 mm pitch across the full 160 mm width, exactly the symbol of the side elevation, so the board seams of the model are the drawn joints. Lamellae alternate in tone and each board is a shade apart, as sawn boards are. Under the square-sawn ends sit the two 120 x 20 x 160 mm steel bearing plates. The arris chamfers of a few millimetres are below the 7.5 mm smallest feature and are not modelled.",
                "Körper: der Balken, wie er hergestellt wird: jede der fünfzehn Lamellen zu 40 mm ist eine Folge gesägter Bretter (40 Körper), verbunden durch die 25 Keilzinkenstöße des Aufbaus, ein oder zwei je Lamelle, versetzt und aus dem mittleren Drittel der unteren Zuglamelle gehalten; jeder Stoß ist ein Profil mit zwei Zinken von 20 mm Länge und 20 mm Teilung über die volle Breite von 160 mm, genau das Zeichen der Seitenansicht, sodass die Brettfugen des Modells die gezeichneten Stöße sind. Die Lamellen wechseln im Ton, jedes Brett ist um eine Nuance verschieden, wie gesägte Bretter. Unter den rechtwinklig gesägten Enden liegen die beiden Stahllagerplatten 120 x 20 x 160 mm. Die Kantenfasen von wenigen Millimetern liegen unter der kleinsten Einzelheit von 7,5 mm und sind nicht modelliert.",
            ),
            text(
                "Thermal: the timber faces are skins of the effective storage depth of 36 mm (24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0.036 m): both sides over the full height, the top over the full width, and the soffit split into the free span and the two 120 mm bearing contacts where it sits on the steel; the bearing plates are referenced from the body. Side and flat skins share the 36 mm arris prisms; the faces against the core and the overlap are section faces, so each region counts its own outer face.",
                "Thermisch: die Holzflächen sind Schalen der wirksamen Speichertiefe von 36 mm (24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0,036 m): beide Seiten über die volle Höhe, die Oberseite über die volle Breite und die Unterseite geteilt in das freie Feld und die beiden 120-mm-Auflagerkontakte, wo sie auf dem Stahl liegt; die Lagerplatten sind aus dem Körper referenziert. Seiten- und Flachschalen teilen sich die 36-mm-Kantenprismen; die Flächen zum Kern und die Überlappung sind Schnittflächen, sodass jede Region ihre eigene Außenfläche zählt.",
            ),
            text(
                "Structure: the member is the real 160 x 600 mm GL24h section over the full 8 m (the glued lamellae act as one section); the pin bearing is a 10 mm steel pad with a 10 mm knife edge under the left plate centre, the roller bearing a 20 mm steel roller across the width under the right plate centre, both standing on the ground at their anchors 7.88 m apart; the slider of the moving load is a 100 mm load shoe flush with the top face. The self-weight is distributed uniformly along the member, so no concentrated mass is modelled.",
                "Tragwerk: das Glied ist der echte Querschnitt GL24h 160 x 600 mm über die vollen 8 m (die verleimten Lamellen wirken als ein Querschnitt); das Festlager ist eine 10-mm-Stahlplatte mit 10-mm-Schneide unter der Mitte der linken Platte, das Rollenlager eine 20-mm-Stahlrolle über die Breite unter der Mitte der rechten Platte, beide stehen am Boden an ihren Ankern im Abstand von 7,88 m; der Schlitten der wandernden Last ist ein 100-mm-Lastschuh bündig mit der Oberseite. Das Eigengewicht ist gleichmäßig über das Glied verteilt, daher ist keine Einzelmasse modelliert.",
            ),
        ],
    )
    v100 = scale(
        100,
        body100(),
        skins(False),
        [member(), pin(PT, 0.0), roller("support-bearing-right-roller", PT, XR, member="beam", part="support", anchor="bearing-right"), slider()],
        [
            text(
                "Body: the fifteen lamellae over the full length, as the lamella hatch of the drawing shows them; the board joints are merged because the 1:100 drawing does not show finger joints. The plates are modelled at their true 20 mm; the drawing thickens them to 25 mm for printing, a difference below the 15 mm smallest feature.",
                "Körper: die fünfzehn Lamellen über die volle Länge, wie die Lamellenschraffur der Zeichnung sie zeigt; die Brettstöße sind zusammengefasst, weil die Zeichnung 1:100 keine Keilzinken zeigt. Die Platten sind in ihren wahren 20 mm modelliert; die Zeichnung verstärkt sie für den Druck auf 25 mm, ein Unterschied unter der kleinsten Einzelheit von 15 mm.",
            ),
            text(
                "Thermal: the four faces as 36 mm storage skins, the soffit now one skin (the bearing contacts of 1:50 merged); the plates are referenced from the body.",
                "Thermisch: die vier Flächen als 36-mm-Speicherschalen, die Unterseite jetzt eine Schale (die Auflagerkontakte von 1:50 zusammengefasst); die Platten sind aus dem Körper referenziert.",
            ),
            text(
                "Structure: the true member section; the pin is a plain 120 mm steel wedge of the plate height with its edge under the beam, the roller a 20 mm roller across the width; the load shoe stays as the slider of the moving load.",
                "Tragwerk: der wahre Gliedquerschnitt; das Festlager ist ein einfacher 120-mm-Stahlkeil in Plattenhöhe mit der Schneide unter dem Balken, das Rollenlager eine 20-mm-Rolle über die Breite; der Lastschuh bleibt als Schlitten der wandernden Last.",
            ),
        ],
    )
    v200 = scale(
        200,
        [box("beam", "glulam", 0.6, [L, 2 * H, 2 * W], [L / 2, 0, 0])] + plates(),
        wedges(-H) + [ref("th-bearing-plate-left", "bearing-plate-left", region="bearing-steel"), ref("th-bearing-plate-right", "bearing-plate-right", region="bearing-steel")],
        [member(), pin(0.03, 0.0), roller("support-bearing-right-roller", 0.03, XR, member="beam", part="support", anchor="bearing-right")],
        [
            text(
                "Body: one 160 x 600 x 8000 mm beam (lamellae merged, the drawing has no lamination at 1:200) on its two 120 x 20 mm plates, which the drawing fills at their true size as the dark bearing bands between ground and soffit.",
                "Körper: ein Balken 160 x 600 x 8000 mm (Lamellen zusammengefasst, die Zeichnung 1:200 zeigt keine Lamellen) auf seinen beiden Platten 120 x 20 mm, die die Zeichnung in wahrer Größe als dunkle Auflagerbänder zwischen Boden und Balkenunterkante füllt.",
            ),
            text(
                "Thermal: the 36 mm storage skins are close to the 30 mm smallest feature, so the section is shared among its faces instead: each face region owns the triangular prism from its face to the section axis (the inner faces are section faces); the plates are referenced from the body.",
                "Thermisch: die 36-mm-Speicherschalen liegen nahe an der kleinsten Einzelheit von 30 mm, daher wird stattdessen der Querschnitt auf seine Flächen aufgeteilt: jede Flächenregion besitzt das Dreiecksprisma von ihrer Fläche bis zur Querschnittsachse (die inneren Flächen sind Schnittflächen); die Platten sind aus dem Körper referenziert.",
            ),
            text(
                "Structure: member, a 30 mm pin wedge and a 30 mm roller (the smallest feature; their top 10 mm reach into the member as the bearing zone); the load shoe is merged into the member, the moving load is shown by its force glyph.",
                "Tragwerk: Glied, ein 30-mm-Festlagerkeil und eine 30-mm-Rolle (kleinste Einzelheit; ihre oberen 10 mm reichen als Auflagerzone ins Glied); der Lastschuh geht im Glied auf, die wandernde Last zeigt ihr Kraftsymbol.",
            ),
        ],
    )
    g = -H - PT
    v500 = scale(
        500,
        [box("beam", "glulam", 0.4, [L, 2 * H + PT, 2 * W], [L / 2, PT / -2, 0])],
        wedges(g) + [box("th-bearing-plate-left", "steel", 0.3, [PL, 0.075, 2 * W], [XL, g + 0.0375, 0], region="bearing-steel"), box("th-bearing-plate-right", "steel", 0.3, [PL, 0.075, 2 * W], [XR, g + 0.0375, 0], region="bearing-steel")],
        [member(), pin(0.075, 0.0), roller("support-bearing-right-roller", 0.075, XR, member="beam", part="support", anchor="bearing-right")],
        [
            text(
                "Body: one 8000 x 620 x 160 mm block from the ground to the top of the beam, as the 1:500 drawing (a single rectangle); lamellae and plates are merged into it.",
                "Körper: ein Block 8000 x 620 x 160 mm vom Boden bis zur Balkenoberkante, wie die Zeichnung 1:500 (ein einziges Rechteck); Lamellen und Platten sind darin aufgegangen.",
            ),
            text(
                "Thermal: the block's section shared among its four faces as at 1:200 but over the full 620 mm height of the block; the bearing steel is a 75 mm block (the smallest feature) at each end inside the soffit prism.",
                "Thermisch: der Querschnitt des Blocks ist wie bei 1:200 auf seine vier Flächen aufgeteilt, aber über die volle Blockhöhe von 620 mm; der Lagerstahl ist an jedem Ende ein 75-mm-Block (kleinste Einzelheit) im Prisma der Unterseite.",
            ),
            text(
                "Structure: the true member section, a 75 mm pin wedge and a 75 mm roller under the bearing centres.",
                "Tragwerk: der wahre Gliedquerschnitt, ein 75-mm-Festlagerkeil und eine 75-mm-Rolle unter den Auflagermitten.",
            ),
            text(
                "Minimal: thermal and structure keep the counts of 1:200 because they are already the least that carries the asset: four face prisms and two plate blocks keep every thermal region, and member, pin and roller are the whole load path of a simply supported beam.",
                "Minimal: Thermik und Tragwerk behalten die Anzahlen von 1:200, weil sie bereits das Wenigste sind, das das Asset trägt: vier Flächenprismen und zwei Plattenblöcke erhalten jede thermische Region, und Glied, Festlager und Rollenlager sind der ganze Lastpfad eines Einfeldträgers.",
            ),
        ],
    )
    return {"volumes": {"50": v50, "100": v100, "200": v200, "500": v500}}


def main():
    """🚀 Write the volumes source."""
    path = os.path.join(HERE, "volumes", "components", "glulam-beam.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(build(), f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(path)


if __name__ == "__main__":
    main()
