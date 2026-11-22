"""🧊 Hand-stated volumes of `machinery/finned-heat-exchanger` for all four scales (author pass 1).

Every number is a dimension of the P40 coil of the asset notes (world frame: x with the air, y up, z along the tubes, headers at +z).
Run `python3 rework-fhx-finned-heat-exchanger-build.py`: writes `volumes/machinery/finned-heat-exchanger.json`; publish with `volumes-insert.py`.
See `rework/assets/machinery--finned-heat-exchanger.md`.
"""
import json
import math

import rework_11_lib as L

SLUG = "finned-heat-exchanger"
R = [0.0, 0.0, -0.7071067811865476, 0.7071067811865476]
ORIGIN = {"frame": 0.0, "fins": 0.02, "tubes": 0.03}
STEEL, COPPER, ALU = ("galvanised-steel", 0.5), ("copper", 0.4), ("aluminium-fin", 0.78)


def tr(bone):
    return {"rotation": R, "translation": [-ORIGIN[bone], 0.0, 0.0]}


def at(bone, x, y, z):
    return [y - ORIGIN[bone], -x, z]


def wbox(id, bone, mat, x0, x1, y0, y1, z0, z1, corner=None, **more):
    """📦 A box between world corners."""
    return L.box(id, bone, mat[0], mat[1], [x1 - x0, y1 - y0, z1 - z0], at(bone, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2), rot=R, corner=corner, **more)


def wext(id, bone, mat, plane, profile, a, b, holes=None, **more):
    """🧇 An extrude stated in world axes."""
    return L.ext(id, bone, mat[0], mat[1], profile, plane, a, b, holes=holes, transform=tr(bone), **more)


def wrev(id, bone, mat, profile, axis, origin, **more):
    """🏺 A revolve stated in world axes."""
    return L.rev(id, bone, mat[0], mat[1], profile, axis, origin, transform=tr(bone), **more)


def ball(id, bone, x, y, z, kg):
    """⚖️ A mass ball of 16 mm per cube root of a kilogram."""
    radius = L.r(0.01601 * kg ** (1 / 3))
    return L.ell(id, bone, "galvanised-steel", 0.5, [radius] * 3, at(bone, x, y, z), rot=R)


ROW = (-0.01732, 0.01732)
FIRST = (0.03, 0.05)
TUBE_R = 0.00794
STEP_UP = {"count": 12, "translation": [0.04, 0, 0]}
C_LEFT = [[0.2, -0.05], [0.2265, -0.05], [0.2265, -0.0485], [0.2015, -0.0485], [0.2015, 0.0485], [0.2265, 0.0485], [0.2265, 0.05], [0.2, 0.05]]
C_RIGHT = [[-0.2, 0.05], [-0.2265, 0.05], [-0.2265, 0.0485], [-0.2015, 0.0485], [-0.2015, -0.0485], [-0.2265, -0.0485], [-0.2265, -0.05], [-0.2, -0.05]]


def bend(steps):
    """🪝 The return bend of circuit 1: 20 mm radius in the plane through both tube ends, beyond the right tube sheet."""
    out = []
    for k in range(steps + 1):
        a = math.pi * k / steps
        out.append(([-0.01732 * math.cos(a), 0.04 - 0.01 * math.cos(a), -0.2115 - 0.02 * math.sin(a)], TUBE_R, TUBE_R))
    return out


def tube_rows():
    return [wrev("tube-row-entering", "tubes", COPPER, [[0, 0], [0, TUBE_R], [0.443, TUBE_R], [0.443, 0]], [0, 0, 1], [ROW[0], FIRST[0], -0.2115], repeat=STEP_UP),
            wrev("tube-row-leaving", "tubes", COPPER, [[0, 0], [0, TUBE_R], [0.443, TUBE_R], [0.443, 0]], [0, 0, 1], [ROW[1], FIRST[1], -0.2115], repeat=STEP_UP)]


def hairpins(steps):
    return L.loft("hairpins", "tubes", "copper", 0.4, bend(steps), capStart=True, capEnd=True, transform=tr("tubes"), repeat=STEP_UP)


def scale_50(old):
    body = {s["id"]: s for s in old["body"]}
    keep = ["casing-bottom", "casing-top"]
    out = [body[i] for i in keep]
    out += [wext("tube-sheet-left", "frame", STEEL, "zx", C_LEFT, 0.0, 0.52), wext("tube-sheet-right", "frame", STEEL, "zx", C_RIGHT, 0.0, 0.52)]
    out += tube_rows() + [hairpins(10)]
    out += [body[i] for i in ("header-return", "header-supply", "outlet-stub", "inlet-stub", "vent-plug", "drain-plug", "fin-pack", "fin-pack-end-sheets")]
    region = {"casing-bottom": "casing", "casing-top": "casing", "tube-sheet-left": "casing", "tube-sheet-right": "casing", "tube-row-entering": "tubes", "tube-row-leaving": "tubes", "hairpins": "tubes", "header-return": "headers", "header-supply": "headers", "outlet-stub": "headers", "inlet-stub": "headers", "vent-plug": "headers", "drain-plug": "headers", "fin-pack": "fins", "fin-pack-end-sheets": "fins"}
    thermal = [L.th(s["id"], region[s["id"]]) for s in out]
    structure = [dict(s) for s in old["structure"] if s["id"] not in ("member-flange-left", "member-flange-right")]
    decisions = [
        L.text("The coil is modelled part by part at its real sizes: 132 fins of 0.2 mm at 3 mm pitch between two pierced end fins, 24 tubes of 15.88 mm in two staggered rows through the tube sheets, twelve return bends of 20 mm radius, two 28 mm headers with domed ends, the stubs with their R 1 sleeves, the vent and the drain plug. Each tube sheet is one folded 1.5 mm sheet, a 100 mm web with two 25 mm duct flanges turned outward; the bottom and the top channel of 100 mm by 20 mm close the frame. The tube holes are filled by the tubes and the 9 mm fixing holes of the flanges, 0.18 mm on paper, are not cut.",
               "Das Register ist Teil für Teil in seinen wirklichen Maßen modelliert: 132 Lamellen von 0,2 mm bei 3 mm Teilung zwischen zwei gelochten Endlamellen, 24 Rohre von 15,88 mm in zwei versetzten Reihen durch die Rohrböden, zwölf Umlenkbögen mit 20 mm Radius, zwei 28-mm-Sammler mit gewölbten Enden, die Stutzen mit ihren Muffen R 1, der Entlüftungs- und der Entleerungsstopfen. Jeder Rohrboden ist ein gekantetes 1,5-mm-Blech, ein 100-mm-Steg mit zwei nach außen gekanteten 25-mm-Kanalflanschen; das untere und das obere Profil von 100 mm mal 20 mm schließen den Rahmen. Die Rohrlöcher sind von den Rohren ausgefüllt, und die 9-mm-Befestigungslöcher der Flansche, 0,18 mm auf dem Papier, sind nicht ausgeschnitten."),
        L.text("The thermal model is the same set of parts in four regions (fins, tubes with bends, headers with stubs and plugs, casing). The structural model is the load path: the tubes span between the tube sheets and are expanded into them (collar joints), the fins hang on the tubes (five sample fins), the headers hang on the tube ends, the frame of two channels and two tube sheets stands on the tube-sheet feet.",
               "Das thermische Modell besteht aus denselben Teilen in vier Regionen (Lamellen, Rohre mit Bögen, Sammler mit Stutzen und Stopfen, Gehäuse). Das Tragwerksmodell ist der Lastpfad: die Rohre spannen zwischen den Rohrböden und sind in sie eingewalzt (Bundgelenke), die Lamellen hängen an den Rohren (fünf Beispiellamellen), die Sammler hängen an den Rohrenden, der Rahmen aus zwei Profilen und zwei Rohrböden steht auf den Füßen der Rohrböden."),
    ]
    return {"status": "authored", "denominator": 50, "body": out, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    body = [
        wbox("casing-bottom", "frame", STEEL, -0.05, 0.05, 0.0, 0.02, -0.2, 0.2),
        wbox("casing-top", "frame", STEEL, -0.05, 0.05, 0.5, 0.52, -0.2, 0.2),
        wext("tube-sheet-left", "frame", STEEL, "zx", C_LEFT, 0.0, 0.52),
        wext("tube-sheet-right", "frame", STEEL, "zx", C_RIGHT, 0.0, 0.52),
    ] + tube_rows() + [
        hairpins(6),
        wrev("header-return", "tubes", COPPER, [[0, 0], [0, 0.014], [0.476, 0.014], [0.476, 0]], [0, 1, 0], [ROW[0], 0.012, 0.2315]),
        wrev("header-supply", "tubes", COPPER, [[0, 0], [0, 0.014], [0.476, 0.014], [0.476, 0]], [0, 1, 0], [ROW[1], 0.032, 0.2315]),
        wrev("outlet-stub", "tubes", COPPER, [[0, 0], [0, 0.014], [0.0441, 0.014], [0.0441, 0.017], [0.0641, 0.017], [0.0641, 0]], [0, 0, 1], [ROW[0], 0.45, 0.2315]),
        wrev("inlet-stub", "tubes", COPPER, [[0, 0], [0, 0.014], [0.0441, 0.014], [0.0441, 0.017], [0.0641, 0.017], [0.0641, 0]], [0, 0, 1], [ROW[1], 0.07, 0.2315]),
        L.box("fin-pack", "fins", "aluminium-fin", 0.78, [0.48, 0.06928, 0.3992], [0.24, 0, 0]),
    ]
    region = {"casing-bottom": "casing", "casing-top": "casing", "tube-sheet-left": "casing", "tube-sheet-right": "casing", "tube-row-entering": "tubes", "tube-row-leaving": "tubes", "hairpins": "tubes", "header-return": "headers", "header-supply": "headers", "outlet-stub": "headers", "inlet-stub": "headers", "fin-pack": "fins"}
    thermal = [L.th(s["id"], region[s["id"]]) for s in body]
    structure = [
        L.st("member-casing-bottom", "frame", "member", of="casing-bottom"),
        L.st("member-casing-top", "frame", "member", of="casing-top"),
        L.st("member-tube-sheet-left", "frame", "member", of="tube-sheet-left"),
        L.st("member-tube-sheet-right", "frame", "member", of="tube-sheet-right"),
        L.st("member-tube-row-entering", "tubes", "member", of="tube-row-entering"),
        L.st("member-tube-row-leaving", "tubes", "member", of="tube-row-leaving"),
        L.st("member-header-return", "tubes", "member", of="header-return"),
        L.st("member-header-supply", "tubes", "member", of="header-supply"),
        L.st("member-fin-plates", "fins", "member", own=L.box("f", "fins", "aluminium-fin", 0.78, [0.48, 0.06928, 0.015], [0.24, 0, -0.16], repeat={"count": 2, "translation": [0, 0, 0.08]})),
        L.st("member-fin-plates-left", "fins", "member", own=L.box("f", "fins", "aluminium-fin", 0.78, [0.48, 0.06928, 0.015], [0.24, 0, 0.08], repeat={"count": 2, "translation": [0, 0, 0.08]})),
        L.st("joint-tube-sheet-left", "tubes", "joint", own=wbox("j", "tubes", COPPER, -0.03, 0.03, 0.02, 0.5, 0.1935, 0.2085)),
        L.st("joint-tube-sheet-right", "tubes", "joint", own=wbox("j", "tubes", COPPER, -0.03, 0.03, 0.02, 0.5, -0.2085, -0.1935)),
        L.st("joint-channel-bottom", "frame", "joint", own=wbox("j", "frame", STEEL, -0.05, 0.05, 0.0, 0.02, -0.2, -0.185, repeat={"count": 2, "translation": [0, 0, 0.385]})),
        L.st("joint-channel-top", "frame", "joint", own=wbox("j", "frame", STEEL, -0.05, 0.05, 0.5, 0.52, -0.2, -0.185, repeat={"count": 2, "translation": [0, 0, 0.385]})),
        L.st("support-base-contact", "frame", "support", anchor="base-contact", own=wbox("s", "frame", STEEL, -0.05, 0.05, 0.0, 0.015, -0.2265, -0.2, repeat={"count": 2, "translation": [0, 0, 0.4265]})),
        L.st("mass-fins", "fins", "mass", own=ball("m", "fins", 0, 0.26, 0, 2.1)),
        L.st("mass-tubes", "tubes", "mass", own=ball("m", "tubes", 0, 0.26, 0.067, 2.7)),
        L.st("mass-tube-sheet-left", "frame", "mass", own=ball("m", "frame", 0, 0.26, 0.162, 1.55)),
        L.st("mass-tube-sheet-right", "frame", "mass", own=ball("m", "frame", 0, 0.26, -0.162, 1.55)),
    ]
    decisions = [
        L.text("At 1:100 a part of 15 mm is the smallest that reads. The fin pack is therefore one block of 69 mm by 480 mm by 400 mm (3 mm fin pitch is 0.03 mm on paper); the 16 mm tubes stay as two rows through it and show as the twelve return bends on the right and at the headers on the left. Headers are plain 28 mm cylinders, the stubs keep their 34 mm sleeve, plugs and fixing holes are left out, the channels are solid 20 mm bars and the tube sheets keep their flanges, between which the bends lie.",
               "Bei 1:100 ist ein Teil von 15 mm das kleinste lesbare. Das Lamellenpaket ist deshalb ein Block von 69 mm mal 480 mm mal 400 mm (3 mm Lamellenteilung sind 0,03 mm auf dem Papier); die 16-mm-Rohre bleiben als zwei Reihen darin und zeigen sich rechts als die zwölf Umlenkbögen und links an den Sammlern. Die Sammler sind glatte 28-mm-Zylinder, die Stutzen behalten ihre 34-mm-Muffe, Stopfen und Befestigungslöcher entfallen, die Profile sind volle 20-mm-Riegel und die Rohrböden behalten ihre Flansche, zwischen denen die Bögen liegen."),
        L.text("Thermal regions are the fin block, the tube rows with the bends inside and beyond it, headers with stubs and the casing. The structure keeps the load path of 1:50 with one joint strip per tube sheet in place of 24 collars: tubes between the tube sheets, four sample fin plates of 15 mm on the tubes, headers on the tube ends, the frame on its two feet.",
               "Thermische Regionen sind der Lamellenblock, die Rohrreihen mit den Bögen in und außerhalb davon, die Sammler mit Stutzen und das Gehäuse. Das Tragwerk behält den Lastpfad von 1:50 mit einem Gelenkstreifen je Rohrboden statt 24 Bunden: Rohre zwischen den Rohrböden, vier Beispiel-Lamellenplatten von 15 mm auf den Rohren, Sammler an den Rohrenden, der Rahmen auf seinen zwei Füßen."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def tube_core(z0=-0.195, z1=0.195):
    return wbox("tube-core", "tubes", COPPER, -0.0253, 0.0253, 0.04, 0.48, z0, z1)


def coarse_structure(frame_members, header, core, plate, first, step):
    return frame_members + [
        L.st("member-fin-plates", "fins", "member", own=L.box("f", "fins", "aluminium-fin", 0.78, [0.46, 0.06928, plate], [0.24, 0, first], repeat={"count": 2, "translation": [0, 0, step]})),
        L.st("member-tube-core", "tubes", "member", own=core),
        L.st("member-header-block", "tubes", "member", of=header),
    ]


def scale_200():
    body = [
        wbox("casing-bottom", "frame", STEEL, -0.05, 0.05, 0.0, 0.03, -0.2, 0.2),
        wbox("casing-top", "frame", STEEL, -0.05, 0.05, 0.49, 0.52, -0.2, 0.2),
        wbox("end-plate-left", "frame", STEEL, -0.05, 0.05, 0.0, 0.52, 0.2, 0.23),
        wbox("end-plate-right", "frame", STEEL, -0.05, 0.05, 0.0, 0.52, -0.23944, -0.2),
        L.box("coil-block", "fins", "aluminium-fin", 0.78, [0.46, 0.06928, 0.4], [0.24, 0, 0]),
        wbox("header-block", "tubes", COPPER, -0.035, 0.035, 0.012, 0.508, 0.23, 0.26, corner=0.014),
        wbox("outlet-stub", "tubes", COPPER, -0.03232, -0.00232, 0.435, 0.465, 0.26, 0.2956),
        wbox("inlet-stub", "tubes", COPPER, 0.00232, 0.03232, 0.055, 0.085, 0.26, 0.2956),
    ]
    thermal = [L.th("casing-bottom", "casing"), L.th("casing-top", "casing"), L.th("end-plate-left", "casing"), L.th("end-plate-right", "casing"), L.th("coil-block", "fins"),
               L.th(None, "tubes", own=dict(tube_core(), id="th-tube-core")), L.th("header-block", "headers"), L.th("outlet-stub", "headers"), L.th("inlet-stub", "headers")]
    structure = coarse_structure([
        L.st("member-casing-bottom", "frame", "member", of="casing-bottom"),
        L.st("member-casing-top", "frame", "member", of="casing-top"),
        L.st("member-end-plate-left", "frame", "member", of="end-plate-left"),
        L.st("member-end-plate-right", "frame", "member", of="end-plate-right"),
    ], "header-block", tube_core(), 0.03, -0.13, 0.26) + [
        L.st("joint-end-plate-left", "tubes", "joint", own=wbox("j", "tubes", COPPER, -0.0253, 0.0253, 0.04, 0.48, 0.185, 0.215)),
        L.st("joint-end-plate-right", "tubes", "joint", own=wbox("j", "tubes", COPPER, -0.0253, 0.0253, 0.04, 0.48, -0.215, -0.185)),
        L.st("support-base-contact", "frame", "support", anchor="base-contact", own=wbox("s", "frame", STEEL, -0.05, 0.05, 0.0, 0.03, -0.23944, -0.2, repeat={"count": 2, "translation": [0, 0, 0.43472]})),
        L.st("mass-coil", "fins", "mass", own=ball("m", "fins", 0, 0.26, 0.01, 7.9)),
    ]
    decisions = [
        L.text("At 1:200 only masses of 30 mm read, so the coil is the eight masses of its drawing: the coil block of fins and tubes, 69 mm deep, between a bottom and a top bar of 30 mm, the left end plate, the right end plate that takes in the return bends (39 mm), the header block of both headers (70 mm by 30 mm) and the two stubs as 30 mm prisms.",
               "Bei 1:200 lesen sich nur Massen von 30 mm, daher besteht das Register aus den acht Massen seiner Zeichnung: dem 69 mm tiefen Registerblock aus Lamellen und Rohren zwischen einem unteren und einem oberen Riegel von 30 mm, der linken Endplatte, der rechten Endplatte, die die Umlenkbögen aufnimmt (39 mm), dem Sammlerblock beider Sammler (70 mm mal 30 mm) und den zwei Stutzen als 30-mm-Prismen."),
        L.text("The tubes are a hidden core of 51 mm by 440 mm by 390 mm inside the coil block for the thermal and the structural model; it carries two sample fin plates of 30 mm and bears on the two end plates through one joint strip each, the frame of bars and end plates stands on the end-plate feet, and one ball carries the 7.9 kg.",
               "Die Rohre sind für das thermische und das Tragwerksmodell ein verdeckter Kern von 51 mm mal 440 mm mal 390 mm im Registerblock; er trägt zwei Beispiel-Lamellenplatten von 30 mm und lagert über je einen Gelenkstreifen auf den zwei Endplatten, der Rahmen aus Riegeln und Endplatten steht auf den Füßen der Endplatten, und eine Kugel trägt die 7,9 kg."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    body = [
        wext("casing-frame", "frame", STEEL, "yz", [[0.0, -0.23944], [0.52, -0.23944], [0.52, 0.23], [0.0, 0.23]], -0.05, 0.05, holes=[[[0.03, -0.2], [0.03, 0.2], [0.49, 0.2], [0.49, -0.2]]]),
        L.box("coil-block", "fins", "aluminium-fin", 0.78, [0.46, 0.06928, 0.4], [0.24, 0, 0]),
        wbox("header-block", "tubes", COPPER, -0.035, 0.035, 0.012, 0.508, 0.23, 0.2956),
    ]
    thermal = [L.th("casing-frame", "casing"), L.th("coil-block", "fins"), L.th(None, "tubes", own=dict(tube_core(-0.23, 0.22), id="th-tube-core")), L.th("header-block", "headers")]
    structure = coarse_structure([L.st("member-casing-frame", "frame", "member", of="casing-frame")], "header-block", tube_core(-0.23, 0.22), 0.075, -0.1, 0.2) + [
        L.st("joint-end-plates", "tubes", "joint", own=wbox("j", "tubes", COPPER, -0.0253, 0.0253, 0.04, 0.48, -0.2375, -0.1625, repeat={"count": 2, "translation": [0, 0, 0.4]})),
        L.st("support-base-contact", "frame", "support", anchor="base-contact", own=wbox("s", "frame", STEEL, -0.05, 0.05, 0.0, 0.03, -0.23944, 0.23)),
        L.st("mass-coil", "fins", "mass", own=ball("m", "fins", 0, 0.26, 0.01, 7.9)),
    ]
    decisions = [
        L.text("At 1:500 the coil is 1 mm high on paper and only its 100 mm depth and its outline carry: one casing frame of 100 mm by 520 mm by 469 mm with the 400 mm by 460 mm air opening, the coil block in the opening and one connection block of 70 mm by 66 mm for both headers and their stubs on the left.",
               "Bei 1:500 ist das Register auf dem Papier 1 mm hoch, und nur seine 100 mm Tiefe und sein Umriss tragen: ein Gehäuserahmen von 100 mm mal 520 mm mal 469 mm mit der Luftöffnung von 400 mm mal 460 mm, der Registerblock in der Öffnung und links ein Anschlussblock von 70 mm mal 66 mm für beide Sammler und ihre Stutzen."),
        L.text("The thermal model nests the tube core, which runs through both end plates, in the coil block inside the casing frame, with the connection block as the header region; the structure is the frame on its base strip with the tube core, which carries two fin plates of 75 mm, bearing on both end plates and one ball for the 7.9 kg.",
               "Das thermische Modell schachtelt den Rohrkern, der durch beide Endplatten läuft, in den Registerblock im Gehäuserahmen, der Anschlussblock ist die Sammlerregion; das Tragwerk ist der Rahmen auf seinem Fußstreifen mit dem Rohrkern, der zwei Lamellenplatten von 75 mm trägt und auf beiden Endplatten lagert, und einer Kugel für die 7,9 kg."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    document, _ = L.load_catalog(SLUG)
    current = json.load(open(L.source_path(SLUG), encoding="utf-8"))["volumes"]["50"]
    volumes = {"50": scale_50(current), "100": scale_100(), "200": scale_200(), "500": scale_500()}
    open(L.source_path(SLUG), "w", encoding="utf-8").write(L.dump({"volumes": volumes}) + "\n")
    print("source written:", L.source_path(SLUG), {k: [len(v[n]) for n in ("body", "thermal", "structure")] for k, v in volumes.items()})


main()
