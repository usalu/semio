"""🧊 Round 3, appliances/duct-fan: every solid of every authored scale and list, stated by hand; writes `volumes/appliances/duct-fan.json`.

Usage: python3 r3-duct-fan-volumes.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-duct-fan-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

SOURCE = os.path.join(HERE, "volumes", "appliances", "duct-fan.json")
AXIS = [0, 0, 1]
QUARTER = {"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 2, 7)}
CORNERS = [(-0.092, 0.092), (0.092, -0.092)]


def solid(id, kind, bone, material="abs-plastic", role="body", tone=0.7, **fields):
    """🧩 One solid with the common members first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(id, bone, profile, origin=(0, 0, 0), axis=AXIS, loop=True, **kw):
    """🌀 A revolve of (axial, radial) points; `loop` repeats the first point so the profile is a closed ring section."""
    pts = [k.pt(p) for p in profile]
    if loop:
        pts.append(pts[0])
    return solid(id, "revolve", bone, profile=pts, axis=list(axis), origin=k.pt(origin), **kw)


def plate(id, z0, z1, bore, tone=0.88):
    """⬜ The rounded square wall plate with the nozzle bore."""
    return solid(id, "extrude", "plate", tone=tone, profile=k.rounded_square(k.PLATE / 2, k.PLATE_R), holes=[list(reversed(k.circle(bore)))], plane="xy", **{"from": z0, "to": z1})


def blade(i, bone="rotor", **kw):
    """🪭 Blade i (1-based) as a loft of pitched lens sections."""
    return solid(f"blade-{i}", "loft", bone, tone=0.82, sections=k.blade_sections(i - 1, **kw), capStart=True, capEnd=True)


def planform_blade(i, count, z0, z1, root=0.040, tip=0.092, chord=(0.050, 0.085), sweep=22.0, phi0=k.PHI0, n=8):
    """🪭 Blade i of a coarse scale: its swept planform pushed through the depth of the blade stage."""
    psi0 = 2 * math.pi * (i - 1) / count + phi0
    side = lambda sgn: [(r, psi0 + math.radians(sweep) * (r - root) / (tip - root) + sgn * (chord[0] + (chord[1] - chord[0]) * (r - root) / (tip - root)) / (2 * r)) for r in [root + (tip - root) * j / n for j in range(n + 1)]]
    le, te = side(1), side(-1)
    tip_arc = [(tip, le[-1][1] + (te[-1][1] - le[-1][1]) * j / 4) for j in range(1, 4)]
    root_arc = [(root, te[0][1] + (le[0][1] - te[0][1]) * j / 4) for j in range(1, 4)]
    ring = le + tip_arc + list(reversed(te)) + root_arc
    poly = [k.pt((r * math.cos(a), r * math.sin(a))) for r, a in ring]
    if sum(poly[j][0] * poly[(j + 1) % len(poly)][1] - poly[(j + 1) % len(poly)][0] * poly[j][1] for j in range(len(poly))) < 0:
        poly.reverse()
    return solid(f"blade-{i}", "extrude", "rotor", tone=0.82, profile=poly, plane="xy", **{"from": z0, "to": z1})


def ref(id, of, **kw):
    """🔗 A reference to a body solid of the same scale."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def th(s, region, layer=None, tags=None):
    """🌡️ A thermal solid of its own."""
    s = dict(s)
    s["region"] = region
    if layer:
        s["layer"] = layer
    if tags:
        s["tags"] = tags
    return s


def st(s, member, part, anchor=None):
    """🏗️ A structural solid of its own."""
    s = dict(s)
    s["member"] = member
    s["part"] = part
    if anchor:
        s["tags"] = [anchor]
        s["anchor"] = anchor
    return s


def pin(z0, z1, d, at):
    """📌 The upper wall screw: a pin along w through the plate (at 1:50 from its cap into the wall plug)."""
    return st(rev("support-wall-screw-top", "plate", [(z0, 0), (z0, d / 2), (z1, d / 2), (z1, 0)], origin=(at[0], at[1], 0), loop=False, material="motor-housing", role="detail", tone=0.25), "plate", "support", "wall-screw-top")


def roller(length, d, at, zc):
    """🛼 The lower wall screw: a roller lying in the plate along u."""
    return st(rev("support-wall-screw-bottom", "plate", [(0, 0), (0, d / 2), (length, d / 2), (length, 0)], origin=(at[0] - length / 2, at[1], zc), axis=(1, 0, 0), loop=False, material="motor-housing", role="detail", tone=0.25), "plate", "support", "wall-screw-bottom")


def mass(id, bone, r, z):
    """⚫ A concentrated mass at its centre of mass."""
    return st(solid(id, "ellipsoid", bone, role="detail", tone=0.15, radii=[r, r, r], transform={"translation": [0, 0, z]}), bone, "mass")


def extract(r0, r1, z0=0.0005, z1=0.003):
    """💨 The extract outlet patch: an annulus across the throat exit between the hub and the throat wall."""
    return th(rev("th-extract", "ring", [(z0, r0), (z0, r1), (z1, r1), (z1, r0)], tone=0.7), "casing", "shell", ["extract"])


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def scale50():
    """🔎 1:50: plate, bell-mouth nozzle, spigot with bead, motor spider and flange, five pitched and twisted blades on the external-rotor hub."""
    mouth = k.arc(k.Z_WALL, k.BORE, k.BORE - k.THROAT, -90, -180, 8)
    body = [
        plate("plate", k.Z_FACE, k.Z_WALL, k.BORE),
        solid("screw", "ellipsoid", "plate", role="detail", tone=0.45, radii=[0.005, 0.005, 0.004], transform={"translation": [0.092, 0.092, k.Z_FACE]}, repeat=QUARTER),
        rev("ring", "ring", [(k.Z_SEAM, k.BORE), (k.Z_SEAM, k.THROAT)] + mouth[:-1] + [(k.Z_FACE, k.BORE)]),
        rev("spigot", "ring", [(k.Z_SEAM, k.BORE), (k.Z_END, k.BORE), (k.Z_END, k.SPIGOT_IN), (0.0135, k.SPIGOT_IN), (0.004, k.THROAT), (k.Z_SEAM, k.THROAT)]),
        rev("lip", "ring", [(k.Z_LIP, k.SPIGOT_IN), (k.Z_LIP, k.LIP_R), (k.Z_END, k.LIP_R), (k.Z_END, k.SPIGOT_IN)], tone=0.6),
        solid("strut", "box", "ring", tone=0.6, size=[0.0745, 0.008, 0.016], cornerRadius=0.002, transform={"translation": [0.06175, 0, 0.022]}, repeat=QUARTER),
        rev("flange", "ring", [(0.010, 0), (0.010, 0.026), (0.030, 0.026), (0.030, 0)], loop=False, tone=0.6),
    ] + [blade(i) for i in range(1, 6)] + [
        rev("hub", "rotor", [(-0.040, 0), (-0.0398, 0.008), (-0.0392, 0.016), (-0.0380, 0.023), (-0.0362, 0.029), (-0.0340, 0.0325), (-0.0315, k.HUB_R), (0.010, k.HUB_R), (0.010, 0)], loop=False, material="motor-housing", tone=0.55),
        solid("hub-cap", "ellipsoid", "rotor", material="motor-housing", role="detail", tone=0.9, radii=[0.012, 0.012, 0.004], transform={"translation": [0, 0, -0.0400]}),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "screw", "ring", "spigot", "lip", "strut", "flange")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 6)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can"), ref("th-hub-cap", "hub-cap", region="hub", layer="can"), extract(k.HUB_R, k.THROAT)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-spigot", "spigot", member="ring", part="member"),
        ref("member-strut", "strut", member="ring", part="member"),
        ref("member-flange", "flange", member="ring", part="member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        st(rev("member-shaft", "rotor", [(-0.030, 0), (-0.030, 0.004), (0.026, 0.004), (0.026, 0)], loop=False, material="motor-housing", role="detail", tone=0.25), "rotor", "member"),
        st(rev("joint-plate-ring", "ring", [(-0.037, 0.096), (-0.037, 0.104), (-0.029, 0.104), (-0.029, 0.096)], role="detail", tone=0.25), "ring", "joint"),
        st(rev("joint-bearing", "rotor", [(0.004, 0), (0.004, 0.009), (0.016, 0.009), (0.016, 0)], loop=False, material="motor-housing", role="detail", tone=0.25), "rotor", "joint"),
        mass("mass-plate", "plate", 0.006, -0.039),
        mass("mass-housing", "ring", 0.012, -0.006),
        mass("mass-rotor", "rotor", 0.011, -0.016),
        pin(k.Z_FACE - 0.004, k.Z_WALL + 0.008, 0.008, CORNERS[0]),
        roller(0.012, 0.008, CORNERS[1], -0.039),
    ]
    decisions = [
        text("Own 1:50 model of a plate-type axial wall fan, axis w, room at -w: a 12 mm ABS plate 0.22 m square with a 0.2 m bore and four domed screw caps, the inlet nozzle with a 12 mm bell mouth into the 176 mm throat, the spigot sleeve (joint at the seam line) that widens behind the rotor to its 195 mm bore and ends in a 208 mm sealing bead, and the motor spider of four 8 x 16 mm arms on a 52 mm stator flange.",
             "Eigenes 1:50-Modell eines Axial-Wandventilators mit Wandplatte, Achse w, Raum bei -w: eine 12 mm starke ABS-Platte von 0,22 m im Quadrat mit 0,2 m Bohrung und vier gewölbten Schraubenkappen, die Einströmdüse mit 12 mm Einlaufradius in den Hals von 176 mm, die Stutzenhülse (Fuge an der Nahtlinie), die sich hinter dem Laufrad auf ihre Bohrung von 195 mm weitet und in einem Dichtwulst von 208 mm endet, und der Motorstern aus vier Armen von 8 x 16 mm an einem Statorflansch von 52 mm."),
        text("The rotor is an external-rotor motor: the 68 mm steel can with its domed front and spinner cap carries five blades lofted through lens sections 3.2 mm thick at the root and 2 mm at the tip, pitched 38 to 24 degrees with the leading edge upstream, chord 40 to 62 mm and swept 22 degrees forward, rooted in a 7.6 mm boss inside the can; the tip corners run 1 mm inside the throat.",
             "Das Laufrad ist ein Außenläufermotor: der Stahlbecher von 68 mm mit gewölbter Front und Nabenkappe trägt fünf Flügel aus Linsenquerschnitten, 3,2 mm stark an der Wurzel und 2 mm an der Spitze, 38 bis 24 Grad angestellt mit der Vorderkante stromauf, Sehne 40 bis 62 mm und 22 Grad vorwärts gepfeilt, mit einer Wurzelverdickung von 7,6 mm im Becher; die Flügelecken laufen 1 mm innerhalb des Halses."),
        text("Thermal: casing = plate, caps, nozzle, spigot, bead, spider and flange, blades and hub by reference with their single layers, and the extract outlet as a 2.5 mm annulus across the throat exit. Structure: plate, nozzle, spigot, spider arms and flange carry the motor; the rotor is its can on an 8 mm shaft; joints are the plate clip ring and the bearing; masses of plate, housing with stator and rotor sit at their centres; the upper left screw is the pin and the lower right one the roller.",
             "Thermik: Gehäuse = Platte, Kappen, Düse, Stutzen, Wulst, Stern und Flansch, Flügel und Nabe als Verweis mit ihrer einen Schicht, und die Abluftöffnung als Ring von 2,5 mm über dem Halsende. Tragwerk: Platte, Düse, Stutzen, Sternarme und Flansch tragen den Motor; das Laufrad ist sein Becher auf einer Welle von 8 mm; Gelenke sind der Klemmring an der Platte und das Lager; die Massen von Platte, Gehäuse mit Stator und Laufrad liegen in ihren Schwerpunkten; die Schraube oben links ist das Festlager, die unten rechts das Rollenlager."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    """🔎 1:100: one housing tube with a chamfered mouth, a 14 mm plate, five generalised 8 mm blades on a plain hub, a four-arm spider."""
    z_wall = -0.031
    body = [
        plate("plate", k.Z_FACE, z_wall, k.BORE),
        rev("ring", "ring", [(k.Z_FACE, k.BORE), (k.Z_END, k.BORE), (k.Z_END, k.SPIGOT_IN), (0.0135, k.SPIGOT_IN), (0.004, k.THROAT), (-0.033, k.THROAT)]),
        solid("strut", "box", "ring", tone=0.6, size=[0.0975, 0.015, 0.016], cornerRadius=0.003, transform={"translation": [0.04875, 0, 0.018]}, repeat=QUARTER),
    ] + [blade(i, chord=(0.042, 0.062), pitch=(44.0, 20.0), sweep=24.0, thick=(0.005, 0.004), boss=0.0152, stations=6, tip=0.086, hub=0.036) for i in range(1, 6)] + [
        rev("hub", "rotor", [(-0.040, 0), (-0.040, 0.028), (-0.032, 0.036), (0.010, 0.036), (0.010, 0)], loop=False, material="motor-housing", tone=0.55),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "ring", "strut")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 6)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can"), extract(0.036, k.THROAT)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-strut", "strut", member="ring", part="member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        st(rev("member-shaft", "rotor", [(-0.030, 0), (-0.030, 0.0075), (0.018, 0.0075), (0.018, 0)], loop=False, material="motor-housing", role="detail", tone=0.25), "rotor", "member"),
        st(rev("joint-bearing", "rotor", [(0.0, 0), (0.0, 0.012), (0.015, 0.012), (0.015, 0)], loop=False, material="motor-housing", role="detail", tone=0.25), "rotor", "joint"),
        mass("mass-plate", "plate", 0.0075, -0.038),
        mass("mass-housing", "ring", 0.0125, -0.006),
        mass("mass-rotor", "rotor", 0.0115, -0.016),
        pin(k.Z_FACE, z_wall, 0.015, CORNERS[0]),
        roller(0.015, 0.014, CORNERS[1], -0.038),
    ]
    decisions = [
        text("Own 1:100 model against the 1:100 shapes: a 14 mm plate (the drawn thickness) with the 0.2 m bore, one housing tube that carries the nozzle as a 12 mm chamfer into the 176 mm throat, the diffuser and the 195 mm spigot bore; the screw caps, the bead, the seam joint, the flange and the spinner are merged away.",
             "Eigenes 1:100-Modell gegen die 1:100-Formen: eine Platte von 14 mm (die gezeichnete Stärke) mit der Bohrung von 0,2 m, ein Gehäuserohr, das die Düse als Fase von 12 mm in den Hals von 176 mm, den Diffusor und die Stutzenbohrung von 195 mm trägt; Schraubenkappen, Wulst, Fuge, Flansch und Nabenkappe gehen darin auf."),
        text("The five blades keep their pitch, twist and sweep but are generalised to lens sections of 5 to 4 mm with a 15 mm root boss buried in the hub (the printable feature); the hub is a plain 72 mm can, the spider four 15 mm arms crossing behind it.",
             "Die fünf Flügel behalten Anstellung, Verwindung und Pfeilung, sind aber auf Linsenquerschnitte von 5 bis 4 mm verallgemeinert, mit einer in der Nabe verborgenen Wurzelverdickung von 15 mm (das druckbare Merkmal); die Nabe ist ein glatter Becher von 72 mm, der Stern vier Arme von 15 mm, die sich dahinter kreuzen."),
        text("Thermal by reference to plate, tube, spider, blades and hub with the outlet annulus; structure: plate, tube and spider, the hub on a 15 mm shaft with its bearing, three masses and the two screws (pin and roller) at 14 to 15 mm.",
             "Thermik als Verweis auf Platte, Rohr, Stern, Flügel und Nabe mit dem Auslassring; Tragwerk: Platte, Rohr und Stern, die Nabe auf einer Welle von 15 mm mit ihrem Lager, drei Massen und die zwei Schrauben (Fest- und Rollenlager) mit 14 bis 15 mm."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    """🔎 1:200: plate, one thin tube with the 192 mm bore, three broad blade planforms through the blade stage, an 88 mm hub, a four-arm spider."""
    bore = 0.096
    body = [
        plate("plate", k.Z_FACE, -0.030, bore),
        rev("ring", "ring", [(-0.030, bore), (-0.030, k.BORE), (k.Z_END, k.BORE), (k.Z_END, bore)]),
        solid("spider", "box", "ring", tone=0.6, size=[0.058, 0.030, 0.030], transform={"translation": [0.069, 0, 0.015]}, repeat=QUARTER),
    ] + [planform_blade(i, 3, -0.033, -0.003) for i in range(1, 4)] + [
        rev("hub", "rotor", [(-0.040, 0), (-0.040, 0.036), (-0.032, 0.044), (0.0, 0.044), (0.0, 0)], loop=False, material="motor-housing", tone=0.55),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "ring", "spider")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 4)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can"), extract(0.044, bore, -0.0028, -0.0003)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-spider", "spider", member="ring", part="member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        mass("mass-fan", "ring", 0.015, -0.016),
        pin(k.Z_FACE, -0.030, 0.030, CORNERS[0]),
        roller(0.030, 0.015, CORNERS[1], -0.0375),
    ]
    decisions = [
        text("Own 1:200 model: the plate at its drawn 15 mm with the 192 mm bore, one thin housing tube without nozzle, diffuser or bead, and the motor spider as four 30 mm arms (the printable floor) that reach under the rim of the 88 mm hub, which takes in the stator flange.",
             "Eigenes 1:200-Modell: die Platte mit ihren gezeichneten 15 mm mit der Bohrung von 192 mm, ein dünnes Gehäuserohr ohne Düse, Diffusor und Wulst, und der Motorstern als vier Arme von 30 mm (die druckbare Untergrenze), die unter den Rand der Nabe von 88 mm greifen, in der der Statorflansch aufgeht."),
        text("Five pitched blades cannot be shown at 30 mm: the drawing gives three broad swept blades, modelled as their planforms pushed through the 30 mm deep blade stage.",
             "Fünf angestellte Flügel lassen sich mit 30 mm nicht zeigen: die Zeichnung gibt drei breite gepfeilte Flügel, modelliert als ihre Grundrisse durch die 30 mm tiefe Flügelstufe."),
        text("Thermal by reference with the outlet annulus; structure: plate, tube, spider and hub as members, one mass of the whole fan at its centre and the two screws at 15 to 30 mm; joints are merged into the members.",
             "Thermik als Verweis mit dem Auslassring; Tragwerk: Platte, Rohr, Stern und Nabe als Stäbe, eine Masse des ganzen Ventilators in seinem Schwerpunkt und die zwei Schrauben mit 15 bis 30 mm; Gelenke gehen in den Stäben auf."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    """💾 Write the source."""
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("written", SOURCE, {s: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for s, v in volumes.items()})


if __name__ == "__main__":
    main()
