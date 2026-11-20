"""🚲 Round 3 handcrafted model of objects/bicycle-stand: rig, anchors, 2D of every scale and view, volumes of every scale.

Every number below is stated by hand from the reference (EN 10219-2 CHS 48.3 x 3.2, centre-line bend radius 180 mm,
outer size 0.75 x 0.80 m, cross rail at 0.40 m). `build` writes the candidate asset document and the volumes source under
the generated folder; `apply` re-reads the catalog file and the volumes source right before writing them.

@see T/round3-brief.md
@see T/r3-objects-bicycle-stand.md
"""
import json
import math
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/bicycle-stand/🔣️.json")
SOURCE = T / "volumes" / "objects" / "bicycle-stand.json"
OUT = T / "🗑️generated" / "r3-bicycle-stand"

D = 0.0483
R = D / 2
DI = 0.0419
RI = DI / 2
XL = 0.35085
CLR = 0.18
YB = 0.59585
YT = 0.77585
LTOP = 0.3417
LCH = 0.254558
LCB = 0.7017
YCB = 0.4
CB = (0.127279, -0.127279, 0.0)
Q45 = [0.0, 0.0, 0.382683, 0.92388]
D5 = 0.0752
R5 = D5 / 2
W5 = 0.063
OFF5 = R5 - R


def tx(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def f(x):
    """🔢 Rounds a coordinate to 0.01 mm."""
    return round(x, 6)


def fmt(*xs):
    """🖋 Formats path numbers."""
    return " ".join(f"{f(x):g}" for x in xs)


def rig():
    """🦴 The structural rig: the bent tube as five straight members on its centre line, the cross rail on the left leg."""
    sec = {"areaM2": 4.5339e-4, "secondMomentM4": 1.15856e-7, "extremeFibreM": 0.02415, "material": "galv-steel", "title": tx("CHS 48.3 x 3.2 (EN 10219-2)", "KHP 48,3 x 3,2 (EN 10219-2)")}

    def mass(kg, length, comy=0.0):
        return {"kg": kg, "comX": f(length / 2), "comY": comy, "distribution": "uniform", "source": "literature", "reference": "en-10219-2"}

    bones = [
        {"id": "stand", "parent": None, "title": tx("Stand reference (ground point between the legs)", "Standbezug (Bodenpunkt zwischen den Beinen)"), "x": 0.0, "y": 0.0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "leg-left", "parent": "stand", "title": tx("Left leg, from the footing to the lower bend tangent", "Linkes Bein, vom Fundament bis zur unteren Biegetangente"), "x": -XL, "y": 0.0, "angleDeg": 90, "lengthM": YB, "joint": {"type": "fixed"}, "mass": mass(2.17, YB), "section": sec},
        {"id": "bend-left", "parent": "leg-left", "title": tx("Left 90 degree bend, radius 180 mm (chord)", "Linker 90-Grad-Bogen, Radius 180 mm (Sehne)"), "x": YB, "y": 0.0, "angleDeg": -45, "lengthM": LCH, "joint": {"type": "fixed"}, "mass": mass(1.03, LCH, 0.0348), "section": sec},
        {"id": "top-rail", "parent": "bend-left", "title": tx("Top rail between the bend tangents", "Oberholm zwischen den Biegetangenten"), "x": LCH, "y": 0.0, "angleDeg": -45, "lengthM": LTOP, "joint": {"type": "fixed"}, "mass": mass(1.24, LTOP), "section": sec},
        {"id": "bend-right", "parent": "top-rail", "title": tx("Right 90 degree bend, radius 180 mm (chord)", "Rechter 90-Grad-Bogen, Radius 180 mm (Sehne)"), "x": LTOP, "y": 0.0, "angleDeg": -45, "lengthM": LCH, "joint": {"type": "fixed"}, "mass": mass(1.03, LCH, 0.0348), "section": sec},
        {"id": "leg-right", "parent": "bend-right", "title": tx("Right leg, from the lower bend tangent to the footing", "Rechtes Bein, von der unteren Biegetangente bis zum Fundament"), "x": LCH, "y": 0.0, "angleDeg": -45, "lengthM": YB, "joint": {"type": "fixed"}, "mass": mass(2.17, YB), "section": sec},
        {"id": "crossbar", "parent": "leg-left", "title": tx("Cross rail, welded between the legs (leg axis to leg axis)", "Querholm, zwischen die Beine geschweißt (Beinachse bis Beinachse)"), "x": YCB, "y": 0.0, "angleDeg": -90, "lengthM": LCB, "joint": {"type": "fixed"}, "mass": mass(2.38, LCB), "section": sec},
    ]
    return {"kind": "structural", "root": "stand", "bones": bones}


def anchors():
    """⚓ Supports at the leg feet, the lean point on the right leg, the lock point at mid rail."""
    return [
        {"id": "leg-left", "bone": "leg-left", "x": 0.0, "y": 0.0, "kind": "support", "title": tx("Left leg embedded in the footing", "Linkes Bein im Fundament eingespannt"), "support": "fixed", "footprintLengthM": D, "footprintWidthM": D},
        {"id": "leg-right", "bone": "leg-right", "x": YB, "y": 0.0, "kind": "support", "title": tx("Right leg embedded in the footing", "Rechtes Bein im Fundament eingespannt"), "support": "fixed", "footprintLengthM": D, "footprintWidthM": D},
        {"id": "lean-point", "bone": "leg-right", "x": f(YB - 0.55), "y": 0.0, "kind": "load", "title": tx("Lean contact on the right leg, 0.55 m above ground", "Anlehnpunkt am rechten Bein, 0,55 m über Boden")},
        {"id": "lock-point", "bone": "crossbar", "x": f(LCB / 2), "y": 0.0, "kind": "load", "title": tx("Lock point at mid cross rail", "Schlosspunkt in Querholmmitte")},
    ]


def arc_pt(c, rad, deg):
    a = math.radians(deg)
    return (c[0] + rad * math.cos(a), c[1] + rad * math.sin(a))


def bend_path(ro, ri, style):
    """🌀 The elevation of a 90 degree bend in its chord frame (local angles 45 to 135 about the bend centre): `arc` true arcs, `cubic` one cubic per arc, `quad` two quadratics per arc."""
    c = CB

    def arc(rad, a0, a1):
        if style == "arc":
            p = arc_pt(c, rad, a1)
            return f"A {fmt(rad, rad)} 0 0 {1 if a1 > a0 else 0} {fmt(*p)}"
        if style == "cubic":
            k = 4 / 3 * math.tan(math.radians(abs(a1 - a0)) / 4) * rad
            s = 1 if a1 > a0 else -1
            p0, p1 = arc_pt(c, rad, a0), arc_pt(c, rad, a1)
            t0 = (-math.sin(math.radians(a0)) * s, math.cos(math.radians(a0)) * s)
            t1 = (-math.sin(math.radians(a1)) * s, math.cos(math.radians(a1)) * s)
            return f"C {fmt(p0[0] + k * t0[0], p0[1] + k * t0[1], p1[0] - k * t1[0], p1[1] - k * t1[1], *p1)}"
        out = []
        for b0, b1 in ((a0, (a0 + a1) / 2), ((a0 + a1) / 2, a1)):
            mid = arc_pt(c, rad / math.cos(math.radians(abs(b1 - b0) / 2)), (b0 + b1) / 2)
            out.append(f"Q {fmt(*mid, *arc_pt(c, rad, b1))}")
        return " ".join(out)

    s0 = arc_pt(c, ro, 45)
    i1 = arc_pt(c, ri, 135)
    return f"M {fmt(*s0)} {arc(ro, 45, 135)} L {fmt(*i1)} {arc(ri, 135, 45)} Z"


def shape(sid, bone, z, tone, kind, region="hoop-galv", role="body", filled=True, **geo):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geo)
    s["region"] = region
    s["material"] = "galv-steel"
    return s


def chevrons(tone):
    """〽 The visible half of the saddle intersection of the equal-diameter T joints: two straight lines from the leg face to the leg axis."""
    left = [[f(R), f(R)], [0.0, 0.0], [f(R), f(-R)]]
    right = [[f(LCB - R), f(R)], [f(LCB), 0.0], [f(LCB - R), f(-R)]]
    return [shape("weld-l", "crossbar", 4, tone, "poly", "crossbar-galv", "detail", False, points=left, closed=False), shape("weld-r", "crossbar", 4, tone, "poly", "crossbar-galv", "detail", False, points=right, closed=False)]


def geometry(scale):
    """✏ Elevation (bone frames), front and plan (world rest frame) of one scale."""
    tone = 0.5 if scale == 500 else 0.45
    d, off = (D5, OFF5) if scale == 500 else (D, 0.0)
    r = d / 2
    lo, hi = -r - off, r - off
    ro, ri = CLR + R, CLR + R - d
    style = {50: "arc", 100: "cubic", 200: "quad", 500: "arc"}[scale]
    el = [
        shape("leg-left", "leg-left", 3, tone, "rect", x=0.0, y=f(lo), w=YB, h=f(d)),
        shape("bend-left", "bend-left", 3, tone, "path", d=bend_path(ro, ri, style)),
        shape("top-rail", "top-rail", 3, tone, "rect", x=0.0, y=f(lo), w=LTOP, h=f(d)),
        shape("bend-right", "bend-right", 3, tone, "path", d=bend_path(ro, ri, style)),
        shape("leg-right", "leg-right", 3, tone, "rect", x=0.0, y=f(lo), w=YB, h=f(d)),
    ]
    if scale == 200:
        el.append(shape("crossbar", "crossbar", 2.8, tone, "rect", "crossbar-galv", x=0.0, y=f(-R), w=LCB, h=f(d)))
    elif scale == 500:
        el.append(shape("crossbar", "crossbar", 3.2, tone, "rect", "crossbar-galv", x=f(d - R), y=f(-r), w=f(LCB - 2 * (d - R)), h=f(d)))
    else:
        el.append(shape("crossbar", "crossbar", 3.2, tone, "rect", "crossbar-galv", x=f(R), y=f(-R), w=f(LCB - D), h=D))
    if scale in (50, 100):
        el += chevrons(0.3)
    top = 0.8 - r
    h = W5 / 2 if scale == 500 else r
    fr = [
        shape("leg-right", "leg-right", 3, tone, "rect", x=f(-h), y=0.0, w=f(2 * h), h=YB),
        shape("bend-right", "bend-right", 3, tone, "path", d=f"M {fmt(-h, YB)} L {fmt(-h, top)} A {fmt(h, r)} 0 0 0 {fmt(h, top)} L {fmt(h, YB)} Z"),
    ]
    xe = XL - off
    pl = [
        shape("top-rail", "top-rail", 3, tone, "rect", x=f(-(XL - CLR)), y=f(-h), w=LTOP, h=f(2 * h)),
        shape("bend-left", "bend-left", 3, tone, "path", d=f"M {fmt(-(XL - CLR), -h)} L {fmt(-xe, -h)} A {fmt(r, h)} 0 0 0 {fmt(-xe, h)} L {fmt(-(XL - CLR), h)} Z"),
        shape("bend-right", "bend-right", 3, tone, "path", d=f"M {fmt(XL - CLR, -h)} L {fmt(xe, -h)} A {fmt(r, h)} 0 0 1 {fmt(xe, h)} L {fmt(XL - CLR, h)} Z"),
    ]
    return {"elevation": el, "front": fr, "plan": pl}


def circle(rc, rad, n):
    """⭕ A round tube section in the [axial, radial] profile plane of a revolve, centred at radial `rc`, closed by repeating its first point (the closing rule of the kernel)."""
    return [[f(rad * math.cos(2 * math.pi * k / n)), f(rc + rad * math.sin(2 * math.pi * k / n))] for k in range(n + 1)]


def half_ring(rc, ro, ri, n, sign):
    """◐ Half of a hollow tube section (axial >= 0 for sign 1, <= 0 for -1): a revolve profile must be one simple polygon."""
    outer = [[f(sign * ro * math.sin(math.pi * k / n)), f(rc + ro * math.cos(math.pi * k / n))] for k in range(n + 1)]
    inner = [[f(sign * ri * math.sin(math.pi * k / n)), f(rc - ri * math.cos(math.pi * k / n))] for k in range(n + 1)]
    return outer + inner + [outer[0]]


def solid(sid, bone, kind, role="body", tone=0.45, **geo):
    s = {"id": sid, "kind": kind, "bone": bone, "material": "galv-steel", "role": role, "tone": tone}
    s.update(geo)
    return s


def tube(sid, bone, length, rad, v=0.0, u0=0.0, bore=None, **extra):
    """🧪 A straight tube along the bone's u axis, solid or hollow (`bore` = inner radius)."""
    prof = [[f(u0), 0.0], [f(u0), f(rad)], [f(u0 + length), f(rad)], [f(u0 + length), 0.0]] if bore is None else [[f(u0), f(bore)], [f(u0), f(rad)], [f(u0 + length), f(rad)], [f(u0 + length), f(bore)], [f(u0), f(bore)]]
    return solid(sid, bone, "revolve", profile=prof, axis=[1, 0, 0], origin=[0.0, f(v), 0.0], **extra)


def bend(sid, bone, prof, **extra):
    """🌀 A 90 degree bend about its centre, local angles 45 to 135 of the chord frame."""
    return solid(sid, bone, "revolve", profile=prof, axis=[0, 0, 1], origin=[0.0, 0.0, 0.0], sweepRad=1.570796, transform={"translation": list(CB), "rotation": Q45}, **extra)


def body(scale):
    """🧱 The body of one scale: the bent tube in five pieces on its bones and the cross rail from leg axis to leg axis (the union is the true saddle joint)."""
    n = {50: 32, 100: 24, 200: 16, 500: 12}[scale]
    rad, off = (R5, OFF5) if scale == 500 else (R, 0.0)
    tone = 0.5 if scale == 500 else 0.45
    return [
        tube("leg-left", "leg-left", YB, rad, -off, tone=tone, sectionFaces=["-u"]),
        bend("bend-left", "bend-left", circle(CLR - off, rad, n), tone=tone),
        tube("top-rail", "top-rail", LTOP, rad, -off, tone=tone),
        bend("bend-right", "bend-right", circle(CLR - off, rad, n), tone=tone),
        tube("leg-right", "leg-right", YB, rad, -off, tone=tone, sectionFaces=["+u"]),
        tube("crossbar", "crossbar", LCB - 2 * off, rad, 0.0, off, tone=tone),
    ]


HOOP = ["leg-left", "bend-left", "top-rail", "bend-right", "leg-right"]


def refs(prefix, field, value_of):
    """🔗 References onto the six body solids."""
    return [{"id": f"{prefix}{b}", "of": b, **{field: value_of(b)}} for b in HOOP + ["crossbar"]]


def thermal(scale):
    """🌡 Thermal solids: the galvanised surfaces are the body surfaces (one 3.2 mm wall layer, B2); at 1:500 the hoop region is the true 48.3 mm tube as one slender loft inside the pen-floor body."""
    if scale != 500:
        return refs("th-", "region", lambda b: "crossbar-galv" if b == "crossbar" else "hoop-galv")
    return [hoop_loft(), {"id": "th-crossbar", "of": "crossbar", "region": "crossbar-galv"}]


def hoop_loft():
    """➰ The true centre line of the bent tube in the left leg's frame, round 48.3 mm sections every 22.5 degrees in the bends (chord sagitta 3.5 mm, a twentieth of the 1:500 feature)."""
    pts = [((-XL, 0.0), (0.0, 1.0)), ((-XL, YB), (0.0, 1.0))]
    for a in (157.5, 135, 112.5):
        r_ = math.radians(a)
        pts.append(((-(XL - CLR) + CLR * math.cos(r_), YB + CLR * math.sin(r_)), (math.sin(r_), -math.cos(r_))))
    pts += [((-(XL - CLR), YT), (1.0, 0.0)), ((XL - CLR, YT), (1.0, 0.0))]
    for a in (67.5, 45, 22.5):
        r_ = math.radians(a)
        pts.append(((XL - CLR + CLR * math.cos(r_), YB + CLR * math.sin(r_)), (math.sin(r_), -math.cos(r_))))
    pts += [((XL, YB), (0.0, -1.0)), ((XL, 0.0), (0.0, -1.0))]
    s = 0.707107
    secs = []
    for (wx, wy), (tx_, ty) in pts:
        a, b = ty, -tx_
        secs.append({"at": [f(wy), f(-(wx + XL)), 0.0], "rx": R, "ry": R, "rotation": [f(-b * s), f(a * s), 0.0, s]})
    return {"id": "th-hoop", "kind": "loft", "bone": "leg-left", "material": "galv-steel", "role": "body", "tone": 0.45, "sections": secs, "capStart": True, "capEnd": True, "slender": True, "region": "hoop-galv"}


def joint(sid, member, bone, u, rad):
    return {**solid(sid, bone, "ellipsoid", "detail", 0.3, radii=[rad, rad, rad], transform={"translation": [f(u), 0.0, 0.0]}), "member": member, "part": "joint"}


def supports(t, w, v):
    return [
        {**solid("support-leg-left", "leg-left", "box", tone=0.15, size=[f(t), f(w), f(w)], transform={"translation": [f(-t / 2), f(v), 0.0]}), "member": "leg-left", "part": "support", "anchor": "leg-left"},
        {**solid("support-leg-right", "leg-right", "box", tone=0.15, size=[f(t), f(w), f(w)], transform={"translation": [f(YB + t / 2), f(v), 0.0]}), "member": "leg-right", "part": "support", "anchor": "leg-right"},
    ]


def welds(rad, face):
    return [joint("joint-weld-left", "crossbar", "crossbar", face, rad), joint("joint-weld-right", "crossbar", "crossbar", LCB - face, rad)]


def tangents(rad):
    return [joint("joint-bend-left-low", "bend-left", "bend-left", 0.0, rad), joint("joint-bend-left-high", "top-rail", "top-rail", 0.0, rad), joint("joint-bend-right-high", "bend-right", "bend-right", 0.0, rad), joint("joint-bend-right-low", "leg-right", "leg-right", 0.0, rad)]


def member(s, bone):
    return {**s, "member": bone, "part": "member"}


def structure(scale):
    """🏗 The load path: CHS 48.3 x 3.2 members on the bones, joints at the bend tangents and the cross rail welds, fixed blocks at the footing."""
    if scale in (50, 100):
        n = 16 if scale == 50 else 12
        hollow = [
            member(tube("m-leg-left", "leg-left", YB, R, bore=RI), "leg-left"),
            member(bend("m-bend-left-out", "bend-left", half_ring(CLR, R, RI, n, 1)), "bend-left"),
            member(bend("m-bend-left-in", "bend-left", half_ring(CLR, R, RI, n, -1)), "bend-left"),
            member(tube("m-top-rail", "top-rail", LTOP, R, bore=RI), "top-rail"),
            member(bend("m-bend-right-out", "bend-right", half_ring(CLR, R, RI, n, 1)), "bend-right"),
            member(bend("m-bend-right-in", "bend-right", half_ring(CLR, R, RI, n, -1)), "bend-right"),
            member(tube("m-leg-right", "leg-right", YB, R, bore=RI), "leg-right"),
            member(tube("m-crossbar", "crossbar", LCB, R, bore=RI), "crossbar"),
        ]
        rj, t = (0.029, 0.0075) if scale == 50 else (0.032, 0.015)
        return hollow + tangents(rj) + welds(rj, R) + supports(t, D + 2 * t, 0.0)
    if scale == 200:
        return [{"id": f"m-{b}", "of": b, "member": b, "part": "member"} for b in HOOP + ["crossbar"]] + welds(0.04, R) + supports(0.03, D + 0.03, 0.0)
    return [{"id": f"m-{b}", "of": b, "member": b, "part": "member"} for b in HOOP + ["crossbar"]] + welds(0.06, D5 - R) + supports(D5, D5 + 0.03, -OFF5)


VOLUME_DECISIONS = {
    50: [
        tx("Body: the real assembly, one cold-bent CHS 48.3 x 3.2 hoop and one welded cross rail, on the six bones of the rig: the legs (0 to 596 mm) and the 342 mm top rail as true cylinders, the two bends as true 90 degree tori of 180 mm centre-line radius (round 32-point section), the cross rail at 400 mm from leg axis to leg axis so that its union with the legs is the true saddle of an equal-diameter T joint. The leg ends at the ground are section faces: the tube continues 0.3 m into the footing. The fillet welds (about 5 mm) and the 10 mm galvanising vent holes are below the 7.5 mm feature and not modelled; the sealed tube is its outer envelope (C31).", "Körper: die reale Baugruppe, ein kaltgebogener Bügel aus KHP 48,3 x 3,2 und ein angeschweißter Querholm, auf den sechs Knochen des Rigs: die Beine (0 bis 596 mm) und der 342 mm lange Oberholm als wahre Zylinder, die zwei Bögen als wahre 90-Grad-Tori mit 180 mm Mittellinienradius (runder Schnitt mit 32 Punkten), der Querholm auf 400 mm von Beinachse zu Beinachse, so dass seine Vereinigung mit den Beinen der wahre Sattel eines T-Stoßes gleicher Durchmesser ist. Die Beinenden am Boden sind Schnittflächen: das Rohr geht 0,3 m ins Fundament weiter. Die Kehlnähte (etwa 5 mm) und die Entlüftungsbohrungen von 10 mm für das Verzinken liegen unter der Einzelheit 7,5 mm und sind nicht modelliert; das geschlossene Rohr ist seine Außenhülle (C31)."),
        tx("Thermal: both regions are the galvanised outer surfaces (one 3.2 mm steel wall layer, so one surface each, B2), referenced from the six body solids; no hidden regions, no emitters.", "Thermisch: beide Regionen sind die verzinkten Außenflächen (eine Stahlwandschicht von 3,2 mm, also je eine Oberfläche, B2), aus den sechs Körpern referenziert; keine verdeckten Regionen, keine Emitter."),
        tx("Structure: the real hollow CHS 48.3 x 3.2 (A 4.53 cm2, I 11.6 cm4) on every member bone, the bends as two half-ring sweeps each (a revolve profile must be one simple polygon), joints at the four bend tangents and at the two cross rail welds (29 mm nodes), fixed blocks 63 x 7.5 x 63 mm under the leg axes for the embedment in the footing. Masses are spread uniformly along the members (3.64 kg/m with zinc), so there is no concentrated mass solid.", "Tragwerk: das reale Hohlprofil KHP 48,3 x 3,2 (A 4,53 cm2, I 11,6 cm4) an jedem Stabknochen, die Bögen als je zwei Halbring-Rotationen (ein Rotationsprofil muss ein einfaches Polygon sein), Gelenke an den vier Biegetangenten und den zwei Querholmnähten (Knoten 29 mm), feste Blöcke 63 x 7,5 x 63 mm unter den Beinachsen für die Einspannung im Fundament. Die Massen sind gleichmäßig über die Stäbe verteilt (3,64 kg/m mit Zink), daher gibt es keinen Körper für eine Einzelmasse."),
    ],
    100: [
        tx("Body: the same true tube (48.3 mm is three features at 1:100) on the six bones, the bend sections simplified to 24 points (chord deviation 0.2 mm), the cross rail again from axis to axis for the saddle; welds and vent holes are far below the 15 mm feature.", "Körper: dasselbe wahre Rohr (48,3 mm sind bei 1:100 drei Einzelheiten) auf den sechs Knochen, die Bogenschnitte auf 24 Punkte vereinfacht (Sehnenabweichung 0,2 mm), der Querholm wieder von Achse zu Achse für den Sattel; Nähte und Entlüftungsbohrungen liegen weit unter der Einzelheit 15 mm."),
        tx("Thermal: hoop-galv and crossbar-galv referenced from the six body solids (one wall layer).", "Thermisch: hoop-galv und crossbar-galv aus den sechs Körpern referenziert (eine Wandschicht)."),
        tx("Structure: the hollow CHS members as at 1:50 with 12-segment half rings in the bends, 32 mm joint nodes at the bend tangents and welds, fixed blocks of 15 mm under the legs.", "Tragwerk: die Hohlprofilstäbe wie bei 1:50 mit Halbringen aus 12 Segmenten in den Bögen, Gelenkknoten von 32 mm an den Biegetangenten und Nähten, feste Blöcke von 15 mm unter den Beinen."),
    ],
    200: [
        tx("Body: the tube stays at its true 48.3 mm (above the 30 mm feature), bends as tori with 16-point sections, the cross rail axis to axis; the drawing has no weld lines at this scale.", "Körper: das Rohr bleibt in wahrer Stärke 48,3 mm (über der Einzelheit 30 mm), Bögen als Tori mit Schnitten aus 16 Punkten, der Querholm von Achse zu Achse; die Zeichnung hat bei diesem Maßstab keine Nahtlinien."),
        tx("Thermal: both regions referenced from the body tubes.", "Thermisch: beide Regionen aus den Körperrohren referenziert."),
        tx("Structure: the body tubes are the members themselves (a bent-tube frame, D3), referenced per bone; the 3.2 mm wall is below the 30 mm feature and merges into the full section. Joints only at the two cross rail welds (80 mm nodes), the bend tangents are continuous tube; fixed blocks of 30 mm.", "Tragwerk: die Körperrohre sind selbst die Stäbe (ein Rahmen aus gebogenem Rohr, D3), je Knochen referenziert; die Wand von 3,2 mm liegt unter der Einzelheit 30 mm und geht im vollen Querschnitt auf. Gelenke nur an den zwei Querholmnähten (Knoten 80 mm), die Biegetangenten sind durchlaufendes Rohr; feste Blöcke von 30 mm."),
    ],
    500: [
        tx("Body: the 48.3 mm tube is 0.1 mm on paper, below the 75 mm feature, so the tube is a round 75.2 mm tube at the print floor, thickened inwards from the true outer edge (outer size 0.75 x 0.80 m exact, bend outer radius 204 mm kept, inner 129 mm), bends as tori with 12-point sections, the cross rail between the thickened legs; front and plan draw it 63 mm across (lateral extent rule), 0.024 mm on paper less than the model.", "Körper: das Rohr 48,3 mm ist auf dem Papier 0,1 mm breit, unter der Einzelheit 75 mm, daher ist es ein rundes Rohr von 75,2 mm an der Druckgrenze, vom wahren Außenrand nach innen verdickt (Außenmaß 0,75 x 0,80 m exakt, äußerer Biegeradius 204 mm behalten, innen 129 mm), Bögen als Tori mit Schnitten aus 12 Punkten, der Querholm zwischen den verdickten Beinen; Ansicht und Grundriss zeichnen es quer 63 mm (Regel der seitlichen Ausdehnung), 0,024 mm auf dem Papier weniger als das Modell."),
        tx("Thermal: the hoop region is the true 48.3 mm tube along its real centre line, one slender loft inside the thickened body, because the region's area is that of the real tube; the cross rail references the body rail (the C15 total then reads the real hoop and the floor-size rail).", "Thermisch: die Bügelregion ist das wahre Rohr 48,3 mm entlang seiner wirklichen Mittellinie, eine schlanke Loft im verdickten Körper, weil die Fläche der Region die des realen Rohrs ist; der Querholm referenziert den Körperholm (die Summe nach C15 liest dann den realen Bügel und den Holm in Mindestgröße)."),
        tx("Structure: the 75.2 mm body tubes are the members (referenced per bone), joints at the two welds as 120 mm nodes, fixed blocks 105 x 75 x 105 mm under the legs.", "Tragwerk: die Körperrohre von 75,2 mm sind die Stäbe (je Knochen referenziert), Gelenke an den zwei Nähten als Knoten von 120 mm, feste Blöcke 105 x 75 x 105 mm unter den Beinen."),
    ],
}


def rep_decisions(scale):
    """📝 The drawing decisions of one scale, rewritten for the true orthographic views."""
    common_overlays = tx("Every piece of the bent tube carries hoop-galv, the cross rail crossbar-galv; the force and moment views colour each piece by its own member.", "Jedes Stück des gebogenen Rohrs trägt hoop-galv, der Querholm crossbar-galv; die Kraft- und Momentenansichten färben jedes Stück nach seinem eigenen Stab.")
    if scale == 50:
        return {
            "silhouette": tx("The hoop at true size (0.97 mm band) drawn per member: legs, two 90 degree bends of 180 mm centre-line radius as true arcs, top rail; the cross rail between the leg faces with the visible half of each saddle joint as a chevron. Front: the near leg and bend end-on, the far parts and the cross rail hidden behind them; plan: the top rail and the two bends seen from above, ending in the leg tubes' semicircles.", "Der Bügel in wahrer Größe (Band 0,97 mm) je Stab gezeichnet: Beine, zwei 90-Grad-Bögen mit 180 mm Mittellinienradius als wahre Kreisbögen, Oberholm; der Querholm zwischen den Beinflächen mit der sichtbaren Hälfte jedes Sattelstoßes als Winkel. Ansicht: das nahe Bein und der nahe Bogen von vorn, die fernen Teile und der Querholm dahinter verdeckt; Grundriss: Oberholm und die zwei Bögen von oben, mit den Halbkreisen der Beinrohre als Enden."),
            "retained": tx("Tube 48.3 mm, bend radius 180 mm, cross rail at 400 mm, saddle joints, bend tangents (as the joints of the pieces).", "Rohr 48,3 mm, Biegeradius 180 mm, Querholm auf 400 mm, Sattelstöße, Biegetangenten (als Stöße der Stücke)."),
            "merged": tx("Tube wall and zinc coat are one filled band.", "Rohrwand und Zinkschicht sind ein gefülltes Band."),
            "omitted": tx("Fillet welds and vent holes (below 0.15 mm on paper), the 0.3 m embedded leg ends, hidden parts in front and plan.", "Kehlnähte und Entlüftungsbohrungen (unter 0,15 mm auf dem Papier), die 0,3 m einbetonierten Beinenden, verdeckte Teile in Ansicht und Grundriss."),
            "thinElements": tx("The weld chevrons are 0.18 mm detail strokes on the 0.97 mm tube.", "Die Nahtwinkel sind Detailstriche von 0,18 mm auf dem 0,97-mm-Rohr."),
            "hatching": tx("None; the tube is narrower than any hatch spacing.", "Keine; das Rohr ist schmaler als jeder Schraffurabstand."),
            "styles": tx("Tone 0.45; body pen 0.35 mm, detail 0.18 mm, ground 0.5 mm.", "Ton 0,45; Körperstift 0,35 mm, Detail 0,18 mm, Boden 0,5 mm."),
            "overlays": common_overlays,
        }
    if scale == 100:
        return {
            "silhouette": tx("The hoop at true size (0.48 mm band) per member, each bend as one cubic arc, the cross rail between the leg faces with its two saddle chevrons. Front: near leg and bend end-on; plan: top rail and the two bends from above.", "Der Bügel in wahrer Größe (Band 0,48 mm) je Stab, jeder Bogen als ein kubischer Bogen, der Querholm zwischen den Beinflächen mit seinen zwei Sattelwinkeln. Ansicht: nahes Bein und naher Bogen von vorn; Grundriss: Oberholm und die zwei Bögen von oben."),
            "retained": tx("True tube size and bend radius, cross rail at 400 mm, both saddle joints.", "Wahre Rohrstärke und Biegeradius, Querholm auf 400 mm, beide Sattelstöße."),
            "merged": tx("Each bend arc drawn as one cubic curve (0.05 mm from the circle).", "Jeder Biegebogen als eine kubische Kurve gezeichnet (0,05 mm vom Kreis)."),
            "omitted": tx("Welds, vent holes, embedded leg ends, hidden parts.", "Nähte, Entlüftungsbohrungen, einbetonierte Beinenden, verdeckte Teile."),
            "thinElements": tx("Saddle chevrons are 0.13 mm detail strokes across the 0.48 mm tube.", "Sattelwinkel sind Detailstriche von 0,13 mm quer zum 0,48-mm-Rohr."),
            "hatching": tx("None; the tube is too narrow for hatching.", "Keine; das Rohr ist für Schraffur zu schmal."),
            "styles": tx("Tone 0.45; body pen 0.25 mm, detail 0.13 mm, ground 0.35 mm.", "Ton 0,45; Körperstift 0,25 mm, Detail 0,13 mm, Boden 0,35 mm."),
            "overlays": common_overlays,
        }
    if scale == 200:
        return {
            "silhouette": tx("Inverted U at true tube size (0.24 mm wide) per member, each bend arc as two quadratic curves, the cross rail as a bar from leg axis to leg axis under the legs. Front: near leg and bend end-on; plan: top rail and bends from above.", "Umgekehrtes U in wahrer Rohrstärke (0,24 mm breit) je Stab, jeder Biegebogen als zwei quadratische Kurven, der Querholm als Balken von Beinachse zu Beinachse unter den Beinen. Ansicht: nahes Bein und naher Bogen von vorn; Grundriss: Oberholm und Bögen von oben."),
            "retained": tx("Real tube diameter 0.0483 m, bend radius 0.18 m, cross rail at 0.40 m.", "Reale Rohrstärke 0,0483 m, Biegeradius 0,18 m, Querholm auf 0,40 m."),
            "merged": tx("The saddle joints merge into the legs (the rail runs under them).", "Die Sattelstöße gehen in den Beinen auf (der Holm läuft unter ihnen durch)."),
            "omitted": tx("Weld chevrons, welds, vent holes, embedded leg ends, hidden parts.", "Nahtwinkel, Nähte, Entlüftungsbohrungen, einbetonierte Beinenden, verdeckte Teile."),
            "thinElements": tx("Tube is 0.24 mm wide, just above the 0.2 mm feature width; outline 0.18 mm.", "Das Rohr ist 0,24 mm breit, knapp über der Mindestbreite von 0,2 mm; Kontur 0,18 mm."),
            "hatching": tx("None.", "Keine."),
            "styles": tx("Tone 0.45; body pen 0.18 mm, ground 0.25 mm.", "Ton 0,45; Körperstift 0,18 mm, Boden 0,25 mm."),
            "overlays": common_overlays,
        }
    return {
        "silhouette": tx("One U band per member at the print floor (75.2 mm = 0.15 mm on paper) with the cross rail between the legs; the 0.75 x 0.80 m outer size and the 204 mm outer bend radius are kept exactly. Front: the near leg and bend end-on as a 63 mm bar; plan: top rail and bends as one 63 mm bar with round ends (the lateral extent may exceed the real 48.3 mm by 15 mm at most, so the 3D tube of 75.2 mm is drawn 12 mm narrower across, 0.024 mm on paper).", "Ein U-Band je Stab an der Druckgrenze (75,2 mm = 0,15 mm auf dem Papier) mit dem Querholm zwischen den Beinen; das Außenmaß 0,75 x 0,80 m und der äußere Biegeradius 204 mm bleiben exakt. Ansicht: das nahe Bein und der nahe Bogen von vorn als Balken von 63 mm; Grundriss: Oberholm und Bögen als ein Balken von 63 mm mit runden Enden (die seitliche Ausdehnung darf die realen 48,3 mm um höchstens 15 mm übersteigen, daher ist das 3D-Rohr von 75,2 mm quer 12 mm schmaler gezeichnet, 0,024 mm auf dem Papier)."),
        "retained": tx("Outer width 0.75 m, height 0.80 m, outer bend radius, the cross rail at 0.40 m.", "Außenbreite 0,75 m, Höhe 0,80 m, äußerer Biegeradius, Querholm auf 0,40 m."),
        "merged": tx("Tube wall, welds and saddle joints merge into the bands.", "Rohrwand, Nähte und Sattelstöße gehen in den Bändern auf."),
        "omitted": tx("Weld lines, embedded leg ends, hidden parts.", "Nahtlinien, einbetonierte Beinenden, verdeckte Teile."),
        "thinElements": tx("The 0.0483 m tube would be 0.097 mm on paper: every band is thickened inwards to 0.0752 m (0.15 mm), the print floor, so the outer size stays exact and the inner opening shrinks by 27 mm per side.", "Das Rohr von 0,0483 m wäre auf dem Papier 0,097 mm breit: jedes Band ist nach innen auf 0,0752 m (0,15 mm) verdickt, die Druckgrenze, so dass das Außenmaß exakt bleibt und die Innenöffnung je Seite um 27 mm schrumpft."),
        "hatching": tx("None.", "Keine."),
        "styles": tx("Solid band tone 0.5, outline 0.13 mm; ground line 0.18 mm.", "Volles Band Ton 0,5, Kontur 0,13 mm; Bodenlinie 0,18 mm."),
        "overlays": common_overlays,
    }


def symbol_true(sym):
    """🔣 The true-extent symbol redrawn on the true bend radius and rail height."""
    g = sym["geometry"]
    for s in g["elevation"]:
        if s["id"] == "hoop-outer":
            s["d"] = f"M -0.375 0 L -0.375 {fmt(YB)} A {fmt(CLR + R, CLR + R)} 0 0 1 {fmt(-(XL - CLR), 0.8)} L {fmt(XL - CLR, 0.8)} A {fmt(CLR + R, CLR + R)} 0 0 1 0.375 {fmt(YB)} L 0.375 0"
        if s["id"] == "hoop-inner":
            s["d"] = f"M {fmt(-(XL - R))} 0 L {fmt(-(XL - R), YB)} A {fmt(CLR - R, CLR - R)} 0 0 1 {fmt(-(XL - CLR), YT - R)} L {fmt(XL - CLR, YT - R)} A {fmt(CLR - R, CLR - R)} 0 0 1 {fmt(XL - R, YB)} L {fmt(XL - R)} 0"
        if s["id"] == "cross-bar":
            s.update({"x": f(-(XL - R)), "y": f(YCB - R), "w": f(LCB - D), "h": D})
    for s in g["front"]:
        if s["id"] == "cross-bar":
            s["points"] = [[f(-R), YCB], [f(R), YCB]]
        if s["id"] == "hoop-tube":
            s.update({"x": f(-R), "w": D})
    for s in g["plan"]:
        if s["id"] == "hoop-plan":
            s.update({"y": f(-R), "h": D})
        if s["id"] in ("leg-plan-l", "leg-plan-r"):
            s["cx"] = f(-XL if s["id"].endswith("l") else XL)


SUMMARY = tx(
    "A galvanised CHS 48.3 x 3.2 tube bent into an inverted U (bend radius 180 mm) with a welded cross rail at 0.40 m, set 0.3 m deep in a concrete footing: 0.75 m wide, 0.80 m high, drawn face-on from the side where the bikes park; a leaning bicycle is a load, not a second asset. Seen from above the hoop is one bar with round ends.",
    "Ein feuerverzinktes Rohr KHP 48,3 x 3,2, zu einem umgekehrten U gebogen (Biegeradius 180 mm), mit angeschweißtem Querholm auf 0,40 m, 0,3 m tief im Betonfundament: 0,75 m breit, 0,80 m hoch, von der Parkseite her frontal gezeichnet; ein angelehntes Fahrrad ist eine Last, kein zweites Asset. Von oben ist der Bügel ein Balken mit runden Enden.",
)


def roots(asset):
    """🌱 Applies every root change to a parsed asset (dict), in place, keeping member order."""
    asset["summary"] = SUMMARY
    asset["dimensions"]["massKg"] = 10.02
    asset["rig"] = rig()
    asset["anchors"] = anchors()
    for scale in (50, 100, 200, 500):
        rep = asset["representations"][str(scale)]
        rep["geometry"] = geometry(scale)
        rep["decisions"] = rep_decisions(scale)
    for sym in asset["symbols"]:
        if sym["id"] == "bicycle-stand-symbol-true-extent":
            symbol_true(sym)
    return asset


def volumes():
    """🧊 The volumes source of all four scales."""
    return {str(s): {"status": "authored", "denominator": s, "body": body(s), "thermal": thermal(s), "structure": structure(s), "decisions": VOLUME_DECISIONS[s]} for s in (50, 100, 200, 500)}


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "build"
    OUT.mkdir(parents=True, exist_ok=True)
    doc = json.loads(CATALOG.read_text())
    roots(doc["asset"])
    doc["asset"]["volumes"] = volumes()
    (OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    if mode == "apply":
        SOURCE.write_text(json.dumps({"volumes": volumes()}, ensure_ascii=False, indent=2) + "\n")
        fresh = json.loads(CATALOG.read_text())
        roots(fresh["asset"])
        fresh["asset"]["volumes"] = volumes()
        CATALOG.write_text(json.dumps(fresh, ensure_ascii=False, indent=2) + "\n")
    print(f"[DEBUG] {mode} done")


if __name__ == "__main__":
    main()
