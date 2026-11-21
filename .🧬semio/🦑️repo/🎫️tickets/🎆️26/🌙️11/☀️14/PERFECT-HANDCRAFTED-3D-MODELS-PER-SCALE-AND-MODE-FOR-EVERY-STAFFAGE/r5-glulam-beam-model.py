"""🪵 Round 5 glulam beam: the round 3 beam (`r3-glulam-beam-model.py`: board-by-board lamellae, plates, wedges, member, pin, roller) with two defects removed at 1:50 and 1:100:
the thermal storage skins of sides, top and soffit met by overlapping in the 36 mm arris prisms (two regions in one volume, visible as interpenetration), now they meet on a mitre;
the load shoe of the moving load sat inside the member (interpenetrating, flush with the top face), now it is a steel load plate on the top face where the point load enters.
Writes `volumes/components/glulam-beam.json`.

See: r5-brief.md, r3-components-glulam-beam.md
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3model", os.path.join(HERE, "r3-glulam-beam-model.py"))
R3 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(R3)
k = R3.k
L, H, W, SKIN = R3.L, R3.H, R3.W, R3.SKIN
PL = R3.PL
XL, XR = R3.XL, R3.XR
text = R3.text
MITRE = 1.2


def side(id_, sign):
    """↔️ A side skin (sign +1 = left at +w, -1 = right at -w): full height on the outer face, mitred to the top and soffit skins along the arris."""
    s = sign
    profile = [[H, s * W], [H - SKIN / MITRE, s * (W - SKIN)], [-H + SKIN / MITRE, s * (W - SKIN)], [-H, s * W]]
    return R3.yz(id_, "glulam", 0.6, profile, region="beam-sides", sectionFaces=["-w" if s > 0 else "+w", "+v", "-v"])


def flat(id_, region, top, x0=0.0, x1=L):
    """⬆️ The top (top=True) or soffit skin over x0 .. x1: full width on the outer face, mitred to the side skins."""
    v0, v1 = (H, H - SKIN) if top else (-H, -H + SKIN)
    inner = W - MITRE * SKIN
    profile = [[v0, W], [v0, -W], [v1, -inner], [v1, inner]]
    return R3.yz(id_, "glulam", 0.6, profile, x0, x1, region=region, sectionFaces=["-v" if top else "+v"])


def skins(split_soffit):
    """🌡️ The four timber faces as skins of the 36 mm storage depth meeting on mitres; the soffit split at the bearing contacts when asked; the plates referenced from the body."""
    out = [side("th-side-l", 1), side("th-side-r", -1), flat("th-top", "beam-top", True)]
    if split_soffit:
        out += [flat("th-soffit-bearing-left", "beam-soffit", False, XL - PL / 2, XL + PL / 2),
                flat("th-soffit-span", "beam-soffit", False, XL + PL / 2, XR - PL / 2),
                flat("th-soffit-bearing-right", "beam-soffit", False, XR - PL / 2, XR + PL / 2)]
    else:
        out.append(flat("th-soffit", "beam-soffit", False))
    return out + [R3.ref("th-bearing-plate-left", "bearing-plate-left", region="bearing-steel"), R3.ref("th-bearing-plate-right", "bearing-plate-right", region="bearing-steel")]


def load_plate(thick, seat):
    """🎚️ The prismatic joint of the moving load: a 100 x 160 mm steel load plate of the given thickness on the top face, where the point load enters, bedded `seat` into the timber (2 mm at 1:100, so that the 15 mm plate stays within one smallest feature of the body outline)."""
    return R3.box("joint-load-carrier", "steel", 0.3, [0.1, thick, 2 * W], [0, H + thick / 2 - seat, 0], bone="load-carrier", member="load-carrier", part="joint")


THERMAL = {
    "50": text(
        "Thermal: the timber faces are skins of the effective storage depth of 36 mm (24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0.036 m): both sides over the full height, the top over the full width, and the soffit split into the free span and the two 120 mm bearing contacts where it sits on the steel; neighbouring skins meet on a mitre along each arris, so no volume belongs to two regions; the faces against the core and the mitres are section faces, so each region counts its own outer face; the bearing plates are referenced from the body.",
        "Thermisch: die Holzflächen sind Schalen der wirksamen Speichertiefe von 36 mm (24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0,036 m): beide Seiten über die volle Höhe, die Oberseite über die volle Breite und die Unterseite geteilt in das freie Feld und die beiden 120-mm-Auflagerkontakte, wo sie auf dem Stahl liegt; benachbarte Schalen treffen sich an jeder Kante auf Gehrung, sodass kein Volumen zwei Regionen gehört; die Flächen zum Kern und die Gehrungen sind Schnittflächen, sodass jede Region ihre eigene Außenfläche zählt; die Lagerplatten sind aus dem Körper referenziert.",
    ),
    "100": text(
        "Thermal: the four faces as 36 mm storage skins meeting on mitres along the arrises, the soffit now one skin (the bearing contacts of 1:50 merged); the plates are referenced from the body.",
        "Thermisch: die vier Flächen als 36-mm-Speicherschalen, die sich an den Kanten auf Gehrung treffen, die Unterseite jetzt eine Schale (die Auflagerkontakte von 1:50 zusammengefasst); die Platten sind aus dem Körper referenziert.",
    ),
}

STRUCTURE = {
    "50": text(
        "Structure: the member is the real 160 x 600 mm GL24h section over the full 8 m (the glued lamellae act as one section); the pin bearing is a 10 mm steel pad with a 10 mm knife edge under the left plate centre, the roller bearing a 20 mm steel roller across the width under the right plate centre, both standing on the ground at their anchors 7.88 m apart; the slider of the moving load is a 100 x 160 x 7.5 mm steel load plate on the top face, where the point load enters the beam. The self-weight is distributed uniformly along the member, so no concentrated mass is modelled.",
        "Tragwerk: das Glied ist der echte Querschnitt GL24h 160 x 600 mm über die vollen 8 m (die verleimten Lamellen wirken als ein Querschnitt); das Festlager ist eine 10-mm-Stahlplatte mit 10-mm-Schneide unter der Mitte der linken Platte, das Rollenlager eine 20-mm-Stahlrolle über die Breite unter der Mitte der rechten Platte, beide stehen am Boden an ihren Ankern im Abstand von 7,88 m; der Schlitten der wandernden Last ist eine Stahllastplatte 100 x 160 x 7,5 mm auf der Oberseite, wo die Einzellast in den Balken eintritt. Das Eigengewicht ist gleichmäßig über das Glied verteilt, daher ist keine Einzelmasse modelliert.",
    ),
    "100": text(
        "Structure: the true member section; the pin is a plain 120 mm steel wedge of the plate height with its edge under the beam, the roller a 20 mm roller across the width; the slider of the moving load is the load plate on the top face, 15 mm thick (one smallest feature).",
        "Tragwerk: der wahre Gliedquerschnitt; das Festlager ist ein einfacher 120-mm-Stahlkeil in Plattenhöhe mit der Schneide unter dem Balken, das Rollenlager eine 20-mm-Rolle über die Breite; der Schlitten der wandernden Last ist die Lastplatte auf der Oberseite, 15 mm dick (eine kleinste Einzelheit).",
    ),
}


def build():
    """🧮 All four scales: round 3 with the mitred skins and the load plate."""
    out = R3.build()
    v = out["volumes"]
    for s, split, thick, seat in (("50", True, 0.0075, 0.0), ("100", False, 0.015, 0.002)):
        v[s]["thermal"] = skins(split)
        v[s]["structure"] = [load_plate(thick, seat) if x["id"] == "joint-load-carrier" else x for x in v[s]["structure"]]
        v[s]["decisions"][1] = THERMAL[s]
        v[s]["decisions"][2] = STRUCTURE[s]
    return out


def main():
    """🚀 Write the volumes source."""
    path = os.path.join(HERE, "volumes", "components", "glulam-beam.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(build(), f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(path)


if __name__ == "__main__":
    main()
