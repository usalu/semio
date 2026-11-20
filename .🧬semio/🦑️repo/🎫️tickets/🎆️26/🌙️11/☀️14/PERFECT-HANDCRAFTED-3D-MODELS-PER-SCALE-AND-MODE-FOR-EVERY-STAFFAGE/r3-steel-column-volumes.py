"""🏛️ Round 3 steel column volumes: the HEB 200 column on its welded base plate, stated solid by solid for 1:50, 1:100, 1:200 and 1:500.

Frame of the `column` bone: u up the column from the footing, v across the flanges (the elevation looks onto the web), w along the flange width.
Real subject: HEB 200 (EN 10365: h 200, b 200, tw 9, tf 15, r 18 mm), base plate 300 x 260 x 20, head plate 240 x 220 x 20,
two gussets 16 mm, four M16 anchors (washer 30 x 3, nut 24 across flats x 14.8, ISO 4032 / ISO 7089), fillet welds a = 6 mm all round.

Usage: python3 r3-steel-column-volumes.py   (writes volumes/components/steel-column.json)

See: r3-components-steel-column.md, r3-steel-column-roots.py, round3-brief.md
"""

import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "volumes", "components", "steel-column.json")

HALF = 0.1
FLANGE = 0.015
WEB = 0.0045
ROOT = 0.018
INNER = HALF - FLANGE
SKIN = 0.002
LEG = 0.0085
BASE = (0.15, 0.13)
CAP = (0.12, 0.11)
ANCHORS = [(-0.125, -0.1), (-0.125, 0.1), (0.125, -0.1), (0.125, 0.1)]
CAP_HOLES = [(-0.05, -0.06), (-0.05, 0.06), (0.05, -0.06), (0.05, 0.06)]


def r6(v):
    """🎯 Round to a micrometre."""
    return round(v, 6)


def pts(points):
    """📍 Rounded point list."""
    return [[r6(a), r6(b)] for a, b in points]


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def solid(id, kind, role="body", tone=0.5, bone="column", **fields):
    """🧱 The common members of a solid followed by its own."""
    return {"id": id, "kind": kind, "bone": bone, "material": "steel", "role": role, "tone": tone, **fields}


def extrude(id, profile, plane, start, end, role="body", tone=0.5, holes=None, **more):
    """📐 A polygon pushed along the third axis of its plane."""
    fields = {"profile": pts(profile)}
    if holes:
        fields["holes"] = [pts(h) for h in holes]
    return solid(id, "extrude", role, tone, **fields, plane=plane, **{"from": r6(start), "to": r6(end)}, **more)


def box(id, size, at, role="body", tone=0.4, **more):
    """📦 A box of size (u, v, w) centred at a point."""
    return solid(id, "box", role, tone, size=[r6(s) for s in size], transform={"translation": [r6(c) for c in at]}, **more)


def turned(id, profile, v, w, role="detail", tone=0.3, bone="column", **more):
    """🔩 A profile of [u, radius] turned about the axis parallel to u through (v, w)."""
    return solid(id, "revolve", role, tone, bone, profile=pts(profile), axis=[1, 0, 0], origin=[0.0, r6(v), r6(w)], **more)


def circle(v, w, radius, count):
    """⭕ A circle as a polygon of count corners."""
    return [(v + radius * math.cos(2 * math.pi * k / count), w + radius * math.sin(2 * math.pi * k / count)) for k in range(count)]


def arc(cv, cw, start_deg, end_deg, count):
    """🌙 Points of a root fillet arc about (cv, cw), both ends included."""
    return [
        (cv + ROOT * math.cos(math.radians(start_deg + (end_deg - start_deg) * k / count)), cw + ROOT * math.sin(math.radians(start_deg + (end_deg - start_deg) * k / count)))
        for k in range(count + 1)
    ]


def rolled(flange, web, root_segments, v0=None):
    """🅷 The outline of the rolled H section in (v, w): flanges of the given thickness, half web thickness, root fillets of r 18 mm as root_segments chords (0: sharp)."""
    inner = HALF - flange
    cv, cw = inner - ROOT, web + ROOT
    if root_segments:
        q1 = arc(cv, cw, 0, -90, root_segments)
        q2 = arc(-cv, cw, -90, -180, root_segments)
        q3 = arc(-cv, -cw, 180, 90, root_segments)
        q4 = arc(cv, -cw, 90, 0, root_segments)
    else:
        q1, q2, q3, q4 = [(inner, web)], [(-inner, web)], [(-inner, -web)], [(inner, -web)]
    return (
        [(HALF, -HALF), (HALF, HALF), (inner, HALF)]
        + q1
        + q2
        + [(-inner, HALF), (-HALF, HALF), (-HALF, -HALF), (-inner, -HALF)]
        + q3
        + q4
        + [(inner, -HALF)]
    )


def bead(id, a, b, normal, plate_u, rise, grow_a=0.0, grow_b=0.0):
    """🔥 One run of the a = 6 mm fillet weld between the shaft face from a to b (points in v, w; normal = air side) and the plate face at plate_u, rising by `rise` along u (negative under the head plate); the run and its twin on the opposite side of the section (half turn about the column axis)."""
    angle = math.atan2(normal[1], normal[0])
    along = (-math.sin(angle), math.cos(angle))
    forward = (b[0] - a[0]) * along[0] + (b[1] - a[1]) * along[1] > 0
    start, grow_start, grow_end = (a, grow_a, grow_b) if forward else (b, grow_b, grow_a)
    length = math.hypot(b[0] - a[0], b[1] - a[1]) + grow_a + grow_b
    origin = (start[0] - along[0] * grow_start, start[1] - along[1] * grow_start)
    return extrude(
        id,
        [(plate_u, 0), (plate_u + rise, 0), (plate_u, LEG)],
        "xy",
        0,
        length,
        "detail",
        0.35,
        transform={"rotation": [r6(math.sin(angle / 2)), 0, 0, r6(math.cos(angle / 2))], "translation": [0, r6(origin[0]), r6(origin[1])]},
        repeat={"count": 2, "rotationAxis": [1, 0, 0], "rotationRad": r6(math.pi)},
    )


def wrap(id, at, turn_deg, plate_u, rise):
    """🔥 The weld wrapped round a flange corner: a quarter cone about the corner edge from the direction turned by turn_deg from +v towards +w, and its half-turn twin."""
    profile = [(plate_u, LEG), (plate_u + rise, 0)] if rise > 0 else [(plate_u + rise, 0), (plate_u, LEG)]
    half = math.radians(turn_deg) / 2
    return solid(
        id,
        "revolve",
        "detail",
        0.35,
        profile=pts(profile),
        axis=[1, 0, 0],
        origin=[0.0, 0.0, 0.0],
        sweepRad=r6(math.pi / 2),
        transform={"rotation": [r6(math.sin(half)), 0, 0, r6(math.cos(half))], "translation": [0, r6(at[0]), r6(at[1])]},
        repeat={"count": 2, "rotationAxis": [1, 0, 0], "rotationRad": r6(math.pi)},
    )


def weld(prefix, plate_u, rise):
    """🔥 The fillet weld all round the H section against a plate: ten runs, each with its half-turn twin; (solid, thermal region) pairs."""
    cv, cw = INNER - ROOT, WEB + ROOT
    k = ROOT * math.sqrt(0.5)
    mid_a, mid_b = (cv + k, cw - k), (-cv - k, cw - k)
    runs = [
        ("flange-face", (HALF, -HALF), (HALF, HALF), (1, 0), 0, 0, "column-flange"),
        ("tip-a", (HALF, HALF), (INNER, HALF), (0, 1), 0, 0, "column-flange"),
        ("inner-a", (INNER, HALF), (INNER, cw), (-1, 0), 0, 0, "column-web"),
        ("root-a-1", (INNER, cw), mid_a, (-math.cos(math.radians(22.5)), math.sin(math.radians(22.5))), 0, 0, "column-web"),
        ("root-a-2", mid_a, (cv, WEB), (-math.sin(math.radians(22.5)), math.cos(math.radians(22.5))), 0, 0, "column-web"),
        ("web", (cv, WEB), (-cv, WEB), (0, 1), 0, 0, "column-web"),
        ("root-b-1", (-cv, WEB), mid_b, (math.sin(math.radians(22.5)), math.cos(math.radians(22.5))), 0, 0, "column-web"),
        ("root-b-2", mid_b, (-INNER, cw), (math.cos(math.radians(22.5)), math.sin(math.radians(22.5))), 0, 0, "column-web"),
        ("inner-b", (-INNER, cw), (-INNER, HALF), (1, 0), 0, 0, "column-web"),
        ("tip-b", (-INNER, HALF), (-HALF, HALF), (0, 1), 0, 0, "column-flange"),
    ]
    corners = [("corner-a-outer", (HALF, HALF), 0), ("corner-a-inner", (INNER, HALF), 90), ("corner-b-inner", (-INNER, HALF), 0), ("corner-b-outer", (-HALF, HALF), 90)]
    out = [(bead(f"{prefix}-{name}", a, b, n, plate_u, rise, ga, gb), region) for name, a, b, n, ga, gb, region in runs]
    return out + [(wrap(f"{prefix}-{name}", at, turn, plate_u, rise), "column-flange") for name, at, turn in corners]


def gusset(id, side, profile, half):
    """📐 A base gusset in the web plane outside a flange: profile in (u, v) for the +v side, mirrored for side -1."""
    return extrude(id, [(u, side * v) for u, v in profile], "xy", -half, half, "body", 0.4)


def ref(id, of, **more):
    """🔗 A reference to a body solid of the same scale."""
    return {"id": id, "of": of, **more}


def chamber(id, side, flange_from, skin_to, root_segments, start, end):
    """🌡️ One chamber half of the web region: half the web with its two root fillets and the skin of the inner flange face (v from flange_from to skin_to), for the +v side or mirrored for side -1."""
    cv, cw = INNER - ROOT, WEB + ROOT
    low = arc(cv, -cw, 90, 0, root_segments) if root_segments else [(INNER, -WEB)]
    high = arc(cv, cw, 0, -90, root_segments) if root_segments else [(INNER, WEB)]
    if flange_from < INNER:
        low, high = low[:-1] + [(flange_from, -cw)], [(flange_from, cw)] + high[1:]
    profile = [(0, -WEB)] + low + [(flange_from, -HALF), (skin_to, -HALF), (skin_to, HALF), (flange_from, HALF)] + high + [(0, WEB)]
    return extrude(id, [(side * v, w) for v, w in profile], "yz", start, end, tone=0.6, sectionFaces=["+v" if side > 0 else "-v", "-u", "+u"], region="column-web")


def shaft_thermal(flange_from, skin_to, plate_from, root_segments, start, end):
    """🌡️ The shaft split by thermal region: the flange plates (outer face and edges; inner face against the skin) and the two chamber halves (web, fillets and the skin of the inner flange faces)."""
    ends = ["-u", "+u"]
    return [
        extrude("th-flange-a", [(plate_from, -HALF), (HALF, -HALF), (HALF, HALF), (plate_from, HALF)], "yz", start, end, sectionFaces=["-v"] + ends, region="column-flange"),
        extrude("th-flange-b", [(-HALF, -HALF), (-plate_from, -HALF), (-plate_from, HALF), (-HALF, HALF)], "yz", start, end, sectionFaces=["+v"] + ends, region="column-flange"),
        chamber("th-chamber-a", 1, flange_from, skin_to, root_segments, start, end),
        chamber("th-chamber-b", -1, flange_from, skin_to, root_segments, start, end),
    ]


def plates_thermal(flange, web, lap, start, end):
    """🌡️ The merged shaft of a coarse scale as its three plates: two flanges (flange region) and the web between them (web region), the web running `lap` into the flanges."""
    inner = HALF - flange
    ends = ["-u", "+u"]
    return [
        extrude("th-flange-a", [(inner, -HALF), (HALF, -HALF), (HALF, HALF), (inner, HALF)], "yz", start, end, sectionFaces=ends, region="column-flange"),
        extrude("th-flange-b", [(-HALF, -HALF), (-inner, -HALF), (-inner, HALF), (-HALF, HALF)], "yz", start, end, sectionFaces=ends, region="column-flange"),
        extrude("th-web", [(-inner - lap, -web), (inner + lap, -web), (inner + lap, web), (-inner - lap, web)], "yz", start, end, tone=0.6, sectionFaces=["-v", "+v"] + ends, region="column-web"),
    ]


def load_pad(radius, depth, proud):
    """🎯 The bearing pad of the eccentric load in the head plate; it rides on the carrier bone, so it follows the pose and the sweep clip."""
    return turned("joint-load-pad", [(-depth, 0), (-depth, radius), (proud, radius), (proud, 0)], 0, 0, "detail", 0.3, "eccentric-carrier", member="eccentric-carrier", part="joint")


def mass(radius):
    """⚖️ The mass point at the centre of mass (1.5 m, on the axis)."""
    return solid("mass-column", "ellipsoid", "detail", 0.1, radii=[radius, radius, radius], transform={"translation": [1.5, 0.0, 0.0]}, member="column", part="mass")


def scale_50():
    """🔍 1:50: the column as built."""
    plate, top = 0.02, 2.98
    body = [extrude("shaft", rolled(FLANGE, WEB, 6), "yz", plate, top, "body", 0.5)]
    body.append(extrude("base-plate", [(-BASE[0], -BASE[1]), (BASE[0], -BASE[1]), (BASE[0], BASE[1]), (-BASE[0], BASE[1])], "yz", 0, plate, "body", 0.4, holes=[circle(v, w, 0.009, 16) for v, w in ANCHORS]))
    gusset_profile = [(0.02, 0.115), (0.02, 0.15), (0.045, 0.15), (0.11, 0.11), (0.11, 0.1), (0.035, 0.1)]
    body.append(gusset("stiffener-a", 1, gusset_profile, 0.008))
    body.append(gusset("stiffener-b", -1, gusset_profile, 0.008))
    base_weld = weld("base-weld", plate, LEG)
    body += [s for s, _ in base_weld]
    for n, (v, w) in enumerate(ANCHORS, 1):
        body.append(turned(f"anchor-rod-{n}", [(0, 0), (0, 0.008), (0.02, 0.008), (0.02, 0.015), (0.023, 0.015), (0.023, 0.008), (0.073, 0.008), (0.075, 0.006), (0.075, 0)], v, w))
        hexagon = [(v + 0.013856 * math.cos(math.radians(30 + 60 * k)), w + 0.013856 * math.sin(math.radians(30 + 60 * k))) for k in range(6)]
        body.append(extrude(f"anchor-nut-{n}", hexagon, "yz", 0.023, 0.038, "detail", 0.3, holes=[circle(v, w, 0.008, 16)]))
    body.append(extrude("cap-plate", [(-CAP[0], -CAP[1]), (CAP[0], -CAP[1]), (CAP[0], CAP[1]), (-CAP[0], CAP[1])], "yz", top, 3.0, "body", 0.4, holes=[circle(v, w, 0.009, 16) for v, w in CAP_HOLES]))
    cap_weld = weld("cap-weld", top, -LEG)
    body += [s for s, _ in cap_weld]

    thermal = shaft_thermal(INNER, INNER + SKIN, INNER + SKIN, 6, plate, top)
    thermal += [ref("th-" + s["id"], s["id"], region=region) for s, region in base_weld + cap_weld]
    thermal.append(ref("th-base-plate", "base-plate", region="base-plate", sectionFaces=["-u"]))
    thermal += [ref("th-" + s["id"], s["id"], region="base-plate") for s in body if s["id"].startswith(("stiffener", "anchor"))]
    thermal.append(ref("th-cap-plate", "cap-plate", region="cap-plate"))

    structure = [ref("st-shaft", "shaft", member="column", part="member")]
    structure.append(ref("support-footing", "base-plate", member="column", part="support", anchor="footing"))
    structure += [ref("st-" + s["id"], s["id"], member="column", part="joint") for s in body if s["id"].startswith(("stiffener", "anchor", "base-weld", "cap-weld", "cap-plate"))]
    structure.append(load_pad(0.018, 0.02, 0.003))
    structure.append(mass(0.045))

    decisions = [
        text(
            "The 1:50 body is the column as built. The shaft is one rolled HEB 200 extrusion (200 x 200 mm, 15 mm flanges, 9 mm web, the 18 mm root fillets as six chords each, 78.1 cm2) of 2.96 m between the plates. The base plate is 300 x 260 x 20 mm, flame-cut square, with four 18 mm holes; two 16 mm gussets stand in the web plane outside the flanges, 50 mm wide and 90 mm high with a 25 mm nose and a 15 mm snipe that clears the flange weld. Each anchor is an M16 rod with chamfered end that carries its 30 x 3 mm washer as a collar (one turned solid, the washer at its real 3 mm) under a hexagon nut of 24 mm across flats. The head plate is 240 x 220 x 20 mm with four 18 mm holes in the chambers for the bolts of the beam above. The shaft is welded to both plates all round with a 6 mm fillet weld (8.5 mm legs), ten runs and four quarter cones round the flange corners per end, each with its twin on the opposite side; the 4 mm gusset welds are below the 7.5 mm feature and left out.",
            "Der 1:50-Körper ist die Stütze wie gebaut. Der Schaft ist ein gewalzter HEB-200-Strang (200 x 200 mm, 15-mm-Flansche, 9-mm-Steg, die 18-mm-Ausrundungen je als sechs Sehnen, 78,1 cm²) von 2,96 m zwischen den Platten. Die Fußplatte misst 300 x 260 x 20 mm, rechtwinklig gebrannt, mit vier 18-mm-Löchern; zwei 16-mm-Rippen stehen in der Stegebene außen an den Flanschen, 50 mm breit und 90 mm hoch mit 25-mm-Nase und 15-mm-Eckausschnitt, der die Flanschnaht freilässt. Jeder Anker ist eine M16-Stange mit gefastem Ende, die ihre Scheibe 30 x 3 mm als Bund trägt (ein Drehkörper, die Scheibe mit ihren wirklichen 3 mm), unter einer Sechskantmutter mit Schlüsselweite 24 mm. Die Kopfplatte misst 240 x 220 x 20 mm mit vier 18-mm-Löchern in den Kammern für die Schrauben des Trägers darüber. Der Schaft ist an beiden Platten umlaufend mit einer 6-mm-Kehlnaht (8,5 mm Schenkel) angeschweißt, zehn Abschnitte und vier Viertelkegel um die Flanschecken je Ende, jeder mit seinem Gegenstück auf der anderen Seite; die 4-mm-Nähte der Rippen liegen unter dem 7,5-mm-Merkmal und entfallen.",
        ),
        text(
            "Thermal follows the declared split of the surface. column-flange is the two flange plates with their outer face and edges (13 mm solids whose inner face lies against the inner skin) plus the weld runs on the outer faces and tips. column-web is the surface of the two chambers as two halves that meet on the column axis: half the web with its fillets and a 2 mm skin on the inner flange face each, plus the weld runs in the chambers. base-plate is the plate (its underside lies on the footing and is a mounted face), the gussets, washers, nuts and rods by reference; cap-plate is the head plate. The faces where the shaft meets the plates are welded contact faces.",
            "Die Thermik folgt der angegebenen Aufteilung der Oberfläche. column-flange sind die zwei Flanschplatten mit Außenseite und Kanten (13-mm-Körper, deren Innenseite an der Innenhaut liegt) samt den Nahtabschnitten an Außenseiten und Flanschenden. column-web ist die Oberfläche der zwei Kammern als zwei Hälften, die sich auf der Stützenachse treffen: je der halbe Steg mit seinen Ausrundungen und eine 2-mm-Haut auf der Flanschinnenseite, dazu die Nahtabschnitte in den Kammern. base-plate ist die Platte (ihre Unterseite liegt auf dem Fundament und ist eine Auflagefläche), dazu Rippen, Scheiben, Muttern und Stangen per Verweis; cap-plate ist die Kopfplatte. Die Flächen, an denen der Schaft auf die Platten trifft, sind verschweißte Kontaktflächen.",
        ),
        text(
            "Structure is the load path of a bare steel column, where every part bears and nothing is cladding: the bearing pad of the load in the head plate (it rides on the carrier bone, 0.1 m off the axis in the default pose), the head plate and its weld as joints, the rolled shaft as the member, the base weld, gussets and the four anchors as joints of the fixed base, the base plate as the fixed support on the footing, and the mass point of 184 kg at 1.5 m.",
            "Das Tragwerk ist der Lastpfad einer blanken Stahlstütze, an der jedes Teil trägt und nichts Verkleidung ist: das Lastlager in der Kopfplatte (es sitzt auf dem Trägerknochen, in der Standardpose 0,1 m außerhalb der Achse), Kopfplatte und ihre Naht als Knoten, der gewalzte Schaft als Stab, Fußnaht, Rippen und die vier Anker als Knoten des eingespannten Fußes, die Fußplatte als festes Auflager auf dem Fundament und der Massenpunkt von 184 kg bei 1,5 m.",
        ),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    """🔎 1:100: the section with chamfered roots, drawn plates, gussets and anchor studs."""
    plate, top = 0.035, 2.965
    body = [extrude("shaft", rolled(FLANGE, WEB, 1), "yz", plate, top, "body", 0.5)]
    body.append(box("base-plate", (plate, 2 * BASE[0], 2 * BASE[1]), (plate / 2, 0, 0)))
    gusset_profile = [(0.035, 0.1), (0.035, 0.15), (0.055, 0.15), (0.11, 0.115), (0.11, 0.1)]
    body.append(gusset("stiffener-a", 1, gusset_profile, 0.01))
    body.append(gusset("stiffener-b", -1, gusset_profile, 0.01))
    for n, (v, w) in enumerate(ANCHORS, 1):
        body.append(turned(f"anchor-{n}", [(plate, 0), (plate, 0.012), (0.075, 0.012), (0.075, 0)], v, w))
    body.append(box("cap-plate", (3.0 - top, 2 * CAP[0], 2 * CAP[1]), ((3.0 + top) / 2, 0, 0)))

    thermal = shaft_thermal(INNER - 0.0002, INNER + SKIN, INNER, 1, plate, top)
    thermal.append(ref("th-base-plate", "base-plate", region="base-plate", sectionFaces=["-u"]))
    thermal += [ref("th-" + s["id"], s["id"], region="base-plate") for s in body if s["id"].startswith(("stiffener", "anchor"))]
    thermal.append(ref("th-cap-plate", "cap-plate", region="cap-plate"))

    structure = [ref("st-shaft", "shaft", member="column", part="member")]
    structure.append(ref("support-footing", "base-plate", member="column", part="support", anchor="footing"))
    structure += [ref("st-" + s["id"], s["id"], member="column", part="joint") for s in body if s["id"].startswith(("stiffener", "anchor", "cap-plate"))]
    structure.append(load_pad(0.018, 0.035, 0.005))
    structure.append(mass(0.06))

    decisions = [
        text(
            "At 1:100 the shaft is one H extrusion with the real 15 mm flanges and 9 mm web; each 18 mm root fillet is one 45 degree chamfer. The plates are the drawn 35 mm blocks (20 mm real, thickened by the drawing so that they print), without holes: an 18 mm hole is 0.18 mm on paper. The gussets are 20 mm trapezoids of the true 50 mm width up to the true height of 0.11 m, without snipe. Each anchor is one 24 mm stud of 40 mm (washer, nut and rod end merged). The welds (8.5 mm) are below the 15 mm feature and left out.",
            "Bei 1:100 ist der Schaft ein H-Strang mit den wirklichen 15-mm-Flanschen und dem 9-mm-Steg; jede 18-mm-Ausrundung ist eine 45-Grad-Fase. Die Platten sind die gezeichneten 35-mm-Blöcke (wirklich 20 mm, von der Zeichnung verdickt, damit sie drucken), ohne Löcher: ein 18-mm-Loch misst auf dem Papier 0,18 mm. Die Rippen sind 20-mm-Trapeze der wahren Breite von 50 mm bis zur wahren Höhe von 0,11 m, ohne Eckausschnitt. Jeder Anker ist ein 24-mm-Bolzen von 40 mm (Scheibe, Mutter und Stangenende zusammengefasst). Die Nähte (8,5 mm) liegen unter dem 15-mm-Merkmal und entfallen.",
        ),
        text(
            "Thermal: the flange plates with outer face and edges, the chambers as two halves of the web with chamfered roots and a 2 mm skin on the inner flange face each (0.2 mm proud of the flange plate so that the two regions never share a face), the base plate (underside on the footing), gussets and studs by reference, the head plate by reference.",
            "Thermik: die Flanschplatten mit Außenseite und Kanten, die Kammern als zwei Hälften des Stegs mit gefasten Wurzeln und je einer 2-mm-Haut auf der Flanschinnenseite (0,2 mm vor der Flanschplatte, damit die zwei Regionen nie eine Fläche teilen), Fußplatte (Unterseite auf dem Fundament), Rippen und Bolzen per Verweis, die Kopfplatte per Verweis.",
        ),
        text(
            "Structure: the shaft as the member, the base plate as the fixed support, gussets, studs and head plate as joints, the bearing pad of the load on the carrier bone and the mass point at 1.5 m.",
            "Tragwerk: der Schaft als Stab, die Fußplatte als festes Auflager, Rippen, Bolzen und Kopfplatte als Knoten, das Lastlager auf dem Trägerknochen und der Massenpunkt bei 1,5 m.",
        ),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200():
    """🔭 1:200: a plain H of 30 mm plates between the drawn 50 mm plates."""
    plate, top = 0.05, 2.95
    body = [
        extrude("shaft", rolled(0.03, 0.015, 0), "yz", plate, top, "body", 0.5),
        box("base-plate", (plate, 2 * BASE[0], 2 * BASE[1]), (plate / 2, 0, 0)),
        box("cap-plate", (3.0 - top, 2 * CAP[0], 2 * CAP[1]), ((3.0 + top) / 2, 0, 0)),
    ]
    thermal = plates_thermal(0.03, 0.015, 0, plate, top)
    thermal.append(ref("th-base-plate", "base-plate", region="base-plate", sectionFaces=["-u"]))
    thermal.append(ref("th-cap-plate", "cap-plate", region="cap-plate"))
    structure = [
        ref("st-shaft", "shaft", member="column", part="member"),
        ref("support-footing", "base-plate", member="column", part="support", anchor="footing"),
        ref("st-cap-plate", "cap-plate", member="column", part="joint"),
        load_pad(0.018, 0.05, 0.008),
        mass(0.07),
    ]
    decisions = [
        text(
            "At 1:200 the shaft is a plain H with 30 mm flanges and a 30 mm web (the feature; the real 15 and 9 mm are below it) and sharp roots, between the drawn 50 mm base and head plates of 300 x 260 and 240 x 220 mm. Gussets (16 mm), anchors and welds are below the feature and left out.",
            "Bei 1:200 ist der Schaft ein schlichtes H mit 30-mm-Flanschen und 30-mm-Steg (das Merkmal; die wirklichen 15 und 9 mm liegen darunter) und scharfen Wurzeln zwischen den gezeichneten 50-mm-Fuß- und Kopfplatten von 300 x 260 und 240 x 220 mm. Rippen (16 mm), Anker und Nähte liegen unter dem Merkmal und entfallen.",
        ),
        text(
            "Thermal: the merged shaft as its three plates, the two flanges for the flange region and the web between them for the web region, and the two plates by reference (base underside on the footing); only the total is compared at this scale.",
            "Thermik: der zusammengefasste Schaft als seine drei Platten, die zwei Flansche für die Flanschregion und der Steg dazwischen für die Stegregion, dazu die beiden Platten per Verweis (Unterseite der Fußplatte auf dem Fundament); in diesem Maßstab zählt nur die Summe.",
        ),
        text(
            "Structure: the shaft as the one member (one rig bone with one section), the base plate as the fixed support, the head plate as the joint of the load with the bearing pad on the carrier bone, and the mass point.",
            "Tragwerk: der Schaft als der eine Stab (ein Rig-Knochen mit einem Querschnitt), die Fußplatte als festes Auflager, die Kopfplatte als Knoten der Last mit dem Lastlager auf dem Trägerknochen und der Massenpunkt.",
        ),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    """🛰️ 1:500: a stout H between two 75 mm plates."""
    plate, top = 0.075, 2.925
    body = [
        extrude("shaft", rolled(0.075, 0.0375, 0), "yz", plate, top, "body", 0.5),
        box("base-plate", (plate, 2 * BASE[0], 2 * BASE[1]), (plate / 2, 0, 0)),
        box("cap-plate", (3.0 - top, 2 * CAP[0], 2 * CAP[1]), ((3.0 + top) / 2, 0, 0)),
    ]
    thermal = plates_thermal(0.075, 0.0375, 0.0125, plate, top)
    thermal.append(ref("th-base-plate", "base-plate", region="base-plate", sectionFaces=["-u"]))
    thermal.append(ref("th-cap-plate", "cap-plate", region="cap-plate"))
    structure = [
        ref("st-shaft", "shaft", member="column", part="member"),
        ref("support-footing", "base-plate", member="column", part="support", anchor="footing"),
        ref("st-cap-plate", "cap-plate", member="column", part="joint"),
        mass(0.08),
    ]
    decisions = [
        text(
            "At 1:500 the column is 0.4 mm wide on paper. The shaft stays an H so that it reads as a rolled section: flanges and web at the 75 mm feature (15 and 9 mm real) leave two chambers of 50 x 62.5 mm. The plates are 75 mm blocks (20 mm real) of the true 300 x 260 and 240 x 220 mm.",
            "Bei 1:500 ist die Stütze auf dem Papier 0,4 mm breit. Der Schaft bleibt ein H, damit er sich als Walzprofil liest: Flansche und Steg mit dem 75-mm-Merkmal (wirklich 15 und 9 mm) lassen zwei Kammern von 50 x 62,5 mm. Die Platten sind 75-mm-Blöcke (wirklich 20 mm) der wahren 300 x 260 und 240 x 220 mm.",
        ),
        text(
            "Thermal: the three plates of the stout H (flanges for the flange region, the web for the web region, running 12.5 mm into each flange so that it is 75 mm deep) and the two plates by reference; the thermal list is minimal with one solid per region part, only the total is compared at this scale.",
            "Thermik: die drei Platten des gedrungenen H (Flansche für die Flanschregion, der Steg für die Stegregion, der je 12,5 mm in die Flansche läuft, damit er 75 mm tief ist) und die beiden Platten per Verweis; die Thermikliste ist minimal mit einem Körper je Regionsteil, in diesem Maßstab zählt nur die Summe.",
        ),
        text(
            "Structure: the shaft as the one member (one rig bone with one section), the base plate as the fixed support, the head plate as the joint of the load and the mass point; the bearing pad is below the feature.",
            "Tragwerk: der Schaft als der eine Stab (ein Rig-Knochen mit einem Querschnitt), die Fußplatte als festes Auflager, die Kopfplatte als Knoten der Last und der Massenpunkt; das Lastlager liegt unter dem Merkmal.",
        ),
    ]
    decisions.append(
        text(
            "Minimal: body and thermal keep the solids of 1:200 in number, one shaft and two plates and the three plates of the shaft; a steel column has no fewer parts, and each is authored anew at the 75 mm feature.",
            "Minimal: Körper und Thermik behalten die Körperzahl von 1:200, ein Schaft und zwei Platten sowie die drei Platten des Schafts; eine Stahlstütze hat nicht weniger Teile, und jedes ist für das 75-mm-Merkmal neu verfasst.",
        )
    )
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    """🚀 Write the volumes source."""
    volumes = {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(OUT, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in volumes.items()})


if __name__ == "__main__":
    main()
