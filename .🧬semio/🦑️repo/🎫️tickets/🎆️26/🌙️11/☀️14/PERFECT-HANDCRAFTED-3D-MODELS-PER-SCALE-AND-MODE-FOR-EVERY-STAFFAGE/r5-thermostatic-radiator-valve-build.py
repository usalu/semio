"""🚿 Round 5 thermostatic radiator valve (round 3 with stepped coarse heads and arms and ring supports at 1:100): the handcrafted volumes of 1:50, 1:100, 1:200 and 1:500 (body, thermal, structure), every section stated by hand in millimetres of the world frame.

World frame = frame of bone `body`: origin where the three axes of the corner angle valve meet, x along the radiator tail, y up (the pipe comes from below), z lateral; the thermostat head points to -z (into the room).
The bones `head` (coupling nut and base ring) and `knob` (the handwheel, turning about z) share this origin: the rig is planar and the head axis is the lateral axis, so every joint lies on the axis at the valve centre. HEAD_Z is the contact face of the M30 x 1.5 connection, KNOB_Z the rear face of the handwheel.

Usage: python3 r5-thermostatic-radiator-valve-build.py   writes volumes/appliances/thermostatic-radiator-valve.json
"""
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "appliances", "thermostatic-radiator-valve.json")
HEAD_Z, KNOB_Z = -23.5, -35.0
BONE_Z = {"body": 0.0, "head": 0.0, "knob": 0.0}
BRASS, ABS = "brass", "abs-plastic"
AX, AY, AZ = [1, 0, 0], [0, 1, 0], [0, 0, 1]


def m(mm):
    """📏 Millimetres to metres, rounded to a hundredth of a millimetre."""
    return round(mm / 1000.0, 5)


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def base(sid, kind, bone, material, tone, role="body"):
    """🧱 The common members of a solid in the catalog's order."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def revolve(sid, bone, material, tone, axis, profile, loop=False, **extra):
    """🏺 A revolve about a world axis through the valve centre; profile in millimetres [axial, radial], axial in world coordinates; loop closes the profile into a shell."""
    shift = BONE_Z[bone] if axis == AZ else 0.0
    points = [[m(a - shift), m(r)] for a, r in profile]
    if loop:
        points.append(points[0])
    origin = [0, 0, 0] if axis == AZ else [0, 0, m(-BONE_Z[bone])]
    solid = base(sid, "revolve", bone, material, tone)
    solid.update({"profile": points, "axis": axis, "origin": origin})
    solid.update(extra)
    return solid


def hexagon(across_flats, first_deg):
    """🔩 The six corners of a hexagon of the given width across flats, the first corner at first_deg."""
    corner = across_flats / math.sqrt(3.0)
    return [[m(corner * math.cos(math.radians(first_deg + 60 * k))), m(corner * math.sin(math.radians(first_deg + 60 * k)))] for k in range(6)]


def fluted(radius, depth, flutes, steps=6):
    """🌼 The grip section of the handwheel: a circle of the given radius with round finger flutes of the given depth, stated corner by corner."""
    points = []
    for k in range(flutes * steps):
        phase = (k % steps) / steps
        dip = depth * math.sin(math.pi * phase / 0.667) ** 2 if phase < 0.667 else 0.0
        angle = 2 * math.pi * k / (flutes * steps)
        points.append([m((radius - dip) * math.cos(angle)), m((radius - dip) * math.sin(angle))])
    return points


def extrude(sid, bone, material, tone, plane, profile, a, b, holes=None, **extra):
    """🧇 An extrusion between the world coordinates a < b (millimetres) of the third axis of the plane."""
    shift = BONE_Z[bone] if plane == "xy" else 0.0
    solid = base(sid, "extrude", bone, material, tone)
    solid.update({"profile": profile, "plane": plane, "from": m(a - shift), "to": m(b - shift)})
    if holes:
        solid["holes"] = holes
    solid.update(extra)
    return solid


def moved(solid, x, y, z):
    """🚚 Places a centred solid at a world point (millimetres)."""
    solid["transform"] = {"translation": [m(x), m(y), m(z - BONE_Z[solid["bone"]])]}
    return solid


def box(sid, bone, material, tone, size, at, role="body", **extra):
    """📦 A box of size (u, v, w) millimetres centred at a world point."""
    solid = base(sid, "box", bone, material, tone, role)
    solid["size"] = [m(size[0]), m(size[1]), m(size[2])]
    solid.update(extra)
    return moved(solid, *at)


def ellipsoid(sid, bone, material, tone, radii, at, role="detail"):
    """🥚 An ellipsoid with semi-axes (u, v, w) millimetres centred at a world point."""
    solid = base(sid, "ellipsoid", bone, material, tone, role)
    solid["radii"] = [m(radii[0]), m(radii[1]), m(radii[2])]
    return moved(solid, *at)


def capsule(sid, bone, material, tone, p0, p1, radius, role="body"):
    """💊 A round bar between two world points (millimetres)."""
    solid = base(sid, "capsule", bone, material, tone, role)
    shift = BONE_Z[bone]
    solid.update({"p0": [m(p0[0]), m(p0[1]), m(p0[2] - shift)], "p1": [m(p1[0]), m(p1[1]), m(p1[2] - shift)], "r0": m(radius), "r1": m(radius)})
    return solid


def thermal(solid, region, layer, sid=None, **extra):
    """🌡️ A thermal solid of its own geometry."""
    out = dict(solid)
    if sid:
        out["id"] = sid
    out.update({"region": region, "layer": layer})
    out.update(extra)
    return out


def of(sid, target, region, layer, **extra):
    """🔗 A thermal reference to a body solid of the same scale."""
    out = {"id": sid, "of": target, "region": region, "layer": layer}
    out.update(extra)
    return out


def structural(solid, member, part, sid=None, **extra):
    """🏗️ A structural solid of its own geometry."""
    out = dict(solid)
    if sid:
        out["id"] = sid
    out.update({"member": member, "part": part})
    out.update(extra)
    return out


HEX_INLET = hexagon(27, 0)
HEX_COMPRESSION = hexagon(24, 0)
HEX_UNION = hexagon(30, 30)
GRIP = fluted(26.5, 1.6, 10)

PIPE_OUTER = [(-80, 0), (-80, 7.5), (-33.5, 7.5), (-33.5, 10.5), (-28, 10.5), (-28, 0)]
PIPE_BORED = [(-52, 0), (-52, 6.5), (-80, 6.5), (-80, 7.5), (-33.5, 7.5), (-33.5, 10.5), (-28, 10.5), (-28, 0)]
BARREL_OUTER = [(15.5, 0), (15.5, 9), (14.7, 12.4), (13.2, 14), (-9, 14), (-9.6, 15), (-22.7, 15), (-23.5, 14.2), (-23.5, 0)]
TAIL_OUTER = [(28, 0), (28, 9.25), (44, 9.25), (44, 10.45), (56.8, 10.45), (58, 9.6), (58, 0)]
TAIL_BORED = [(28, 0), (28, 9.25), (44, 9.25), (44, 10.45), (56.8, 10.45), (58, 9.6), (58, 6.5), (34, 6.5), (34, 0)]
NUT_RING = [(-12.5, 15), (-12.5, 16.2), (-13.3, 17), (-24.7, 17), (-25.5, 16.2), (-25.5, 8), (-23.5, 8), (-23.5, 15)]
BASE_OUTER = [(-25, 0), (-25, 20.5), (-26.5, 22), (-41, 22), (-41, 0)]
BAND_OUTER = [(-35, 0), (-35, 25), (-36.5, 26.5), (-46, 26.5), (-46, 25.2), (-48, 25.2), (-48, 0)]
CAP_OUTER = [(-71.5, 0), (-71.5, 24), (-75.5, 23.3), (-75.5, 22.1), (-77.5, 22.1), (-77.5, 22.9), (-80.5, 22.4), (-80.5, 21.2), (-82.5, 21.2), (-82.5, 22), (-86.5, 21.3), (-89.8, 20), (-92, 17.6), (-93, 14.5), (-93, 0)]


def scale50():
    """🔍 1:50, smallest feature 7.5 mm: every assembly of the valve and of the head."""
    body = [
        revolve("pipe", "body", BRASS, 0.55, AY, PIPE_BORED, sectionFaces=["-v"]),
        extrude("compression-nut", "body", BRASS, 0.6, "zx", HEX_COMPRESSION, -46, -32),
        extrude("inlet-socket", "body", BRASS, 0.6, "zx", HEX_INLET, -29, -19),
        revolve("inlet-neck", "body", BRASS, 0.6, AY, [(-20, 0), (-20, 11), (-7, 11), (-7, 0)]),
        revolve("valve-chamber", "body", BRASS, 0.6, AZ, BARREL_OUTER),
        revolve("outlet-boss", "body", BRASS, 0.6, AX, [(4, 0), (4, 13.2), (20, 13.2), (20, 0)]),
        extrude("union-nut", "body", BRASS, 0.6, "yz", HEX_UNION, 18, 32),
        revolve("tailpiece", "body", BRASS, 0.55, AX, TAIL_BORED),
        revolve("coupling-nut", "head", BRASS, 0.6, AZ, NUT_RING, loop=True),
        revolve("base-ring", "head", ABS, 0.8, AZ, BASE_OUTER),
        extrude("index", "head", ABS, 0.12, "zx", [[m(-26.5), m(-3)], [m(-26.5), m(3)], [m(-34.5), 0.0]], 21.5, 23.6, role="texture", texture=True),
        revolve("scale-band", "knob", ABS, 0.95, AZ, BAND_OUTER),
        extrude("grip", "knob", ABS, 0.95, "xy", GRIP, -72, -47.5, taperTo=[0.955, 1]),
        revolve("sensor-cap", "knob", ABS, 0.95, AZ, CAP_OUTER),
        box("pointer", "knob", ABS, 0.1, (11, 4.5, 1.2), (0, 5.75, -93.0), role="texture", texture=True),
    ]
    th = [
        thermal(revolve("th-pipe", "body", BRASS, 0.55, AY, PIPE_OUTER), "body", "wall", sectionFaces=["-v"]),
        of("th-compression-nut", "compression-nut", "body", "wall"),
        of("th-inlet-socket", "inlet-socket", "body", "wall"),
        of("th-inlet-neck", "inlet-neck", "body", "wall"),
        of("th-valve-chamber", "valve-chamber", "body", "wall"),
        of("th-outlet-boss", "outlet-boss", "body", "wall"),
        of("th-union-nut", "union-nut", "body", "wall"),
        thermal(revolve("th-tailpiece", "body", BRASS, 0.55, AX, TAIL_OUTER), "body", "wall", sectionFaces=["+u"]),
        of("th-coupling-nut", "coupling-nut", "head", "shell"),
        of("th-base-ring", "base-ring", "head", "shell"),
        of("th-scale-band", "scale-band", "head", "shell"),
        of("th-grip", "grip", "head", "shell", sectionFaces=["-w"]),
        of("th-sensor-cap", "sensor-cap", "knob", "shell", sectionFaces=["+w"]),
    ]
    st = [
        structural(revolve("member-inlet", "body", BRASS, 0.6, AY, [(-80, 6.5), (-80, 7.5), (-31, 7.5), (-31, 11), (-2, 11), (-2, 8), (-29, 8), (-29, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-outlet", "body", BRASS, 0.6, AX, [(2, 6.5), (2, 10.2), (30, 10.2), (30, 9.25), (58, 9.25), (58, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-chamber", "body", BRASS, 0.6, AZ, [(12, 8), (12, 11), (-23.5, 11), (-23.5, 8)], loop=True), "body", "member"),
        {"id": "member-coupling-nut", "of": "coupling-nut", "member": "head", "part": "member"},
        structural(revolve("member-base-sleeve", "head", ABS, 0.5, AZ, [(-25.5, 8), (-25.5, 11.5), (-52, 11.5), (-52, 8)], loop=True), "head", "member"),
        structural(revolve("member-handwheel-hub", "knob", ABS, 0.5, AZ, [(-35, 12.5), (-35, 15.5), (-56, 15.5), (-60, 12.5), (-76, 12.5), (-76, 4), (-72, 4), (-72, 9.5), (-58, 9.5), (-55, 12.5)], loop=True), "knob", "member"),
        structural(capsule("member-pressure-pin", "knob", BRASS, 0.5, (0, 0, -30), (0, 0, -70), 2.25), "knob", "member"),
        structural(revolve("joint-compression", "body", BRASS, 0.4, AY, [(-44, 7.5), (-44, 10), (-34, 10), (-34, 7.5)], loop=True), "body", "joint"),
        structural(revolve("joint-union", "body", BRASS, 0.4, AX, [(21, 10.2), (21, 13), (31, 13), (31, 9.25)], loop=True), "body", "joint"),
        structural(revolve("joint-setting-thread", "knob", ABS, 0.4, AZ, [(-35, 11.5), (-35, 12.5), (-52, 12.5), (-52, 11.5)], loop=True), "knob", "joint"),
        structural(capsule("support-pipe-joint", "body", BRASS, 0.3, (0, -39, -8), (0, -39, 8), 3.75, role="ground"), "body", "support", anchor="pipe-joint"),
        structural(capsule("support-radiator-union", "body", BRASS, 0.3, (51, 0, -6), (51, 0, 6), 3.75, role="ground"), "body", "support", anchor="radiator-union"),
        structural(ellipsoid("mass-body", "body", BRASS, 0.3, (6, 6, 6), (9, -15, 0)), "body", "mass"),
        structural(ellipsoid("mass-sensor", "knob", ABS, 0.3, (9, 9, 9), (0, 0, -79)), "knob", "mass"),
    ]
    decisions = [
        text("Body: a corner angle valve DN 15 after the cited data sheet: 15 mm pipe with its cut end and bore, compression nut AF 24, hexagon inlet socket AF 27, inlet neck, valve chamber with the M30 x 1.5 collar, G 3/4 outlet boss, union nut AF 30 and the bored R 1/2 tailpiece 58 mm from the centre; the head is the knurled coupling nut, the fixed base ring with its index arrow, and the handwheel of 53 mm with scale band, ten finger flutes and the slotted sensor cap with the embossed front mark.",
             "Körper: ein Winkeleckventil DN 15 nach dem zitierten Datenblatt: 15-mm-Rohr mit Schnittende und Bohrung, Klemmmutter SW 24, Sechskant-Eingangsmuffe SW 27, Eingangshals, Ventilkammer mit dem Bund M30 x 1,5, Ausgangsstutzen G 3/4, Überwurfmutter SW 30 und der gebohrte Schraubnippel R 1/2 bis 58 mm von der Mitte; der Kopf ist die gerändelte Überwurfmutter, der feste Sockelring mit Indexpfeil und das Handrad von 53 mm mit Skalenband, zehn Griffmulden und der geschlitzten Fühlerkappe mit der geprägten Frontmarke."),
        text("Thermal: every region is the outer surface of its body solids: the brass region is the pipe, the fittings, the valve body and the union with the tailpiece (the cut end of the pipe and the end of the tailpiece inside the radiator are section faces, their bores carry water and count for no area), the head region the coupling nut, the base ring, the scale band and the fluted grip, the knob region the slotted sensor cap (the plane where grip and cap meet is a cut through the one handwheel and counts for neither); each region has one layer, so no nested shells are needed.",
             "Thermik: jede Region ist die Außenfläche ihrer Körper: die Messingregion ist Rohr, Verschraubungen, Ventilkörper und Verschraubung mit Schraubnippel (das Schnittende des Rohrs und das Ende des Schraubnippels im Heizkörper sind Schnittflächen, ihre Bohrungen führen Wasser und zählen nicht zur Fläche), die Kopfregion Überwurfmutter, Sockelring, Skalenband und Griffteil, die Knopfregion die geschlitzte Fühlerkappe (die Ebene, in der Griff und Kappe zusammentreffen, ist ein Schnitt durch das eine Handrad und zählt für keine von beiden); jede Region hat eine Schicht, daher braucht es keine verschachtelten Schalen."),
        text("Structure: three brass tubes meeting at the valve centre (inlet with the pipe, outlet with the tailpiece, chamber towards the head), the coupling nut and the base sleeve as the head member, the handwheel hub with the pressure pin as the knob member, joints at the compression fitting, the union and the setting thread, a pin at the pipe joint, a roller at the radiator union and the masses of the body and of the liquid sensor.",
             "Tragwerk: drei Messingrohre, die sich in der Ventilmitte treffen (Eingang mit Rohr, Ausgang mit Schraubnippel, Kammer zum Kopf), Überwurfmutter und Sockelhülse als Glied des Kopfes, die Handradnabe mit dem Druckstift als Glied des Knopfes, Gelenke an Klemmverschraubung, Verschraubung und Einstellgewinde, ein Festlager an der Rohrverschraubung, ein Rollenlager an der Heizkörperverschraubung und die Massen des Körpers und des Flüssigkeitsfühlers."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": th, "structure": st, "decisions": decisions}


def scale100():
    """🔎 1:100, smallest feature 15 mm: hexagons become round collars, the head two masses."""
    body = [
        revolve("inlet", "body", BRASS, 0.6, AY, [(-80, 0), (-80, 7.5), (-46, 7.5), (-46, 12.5), (-31, 12.5), (-29, 14), (-19, 14), (-17, 11), (-6, 11), (-6, 0)]),
        revolve("chamber", "body", BRASS, 0.6, AZ, [(15.5, 0), (15.5, 9.5), (13.2, 14), (-10, 14), (-12, 15), (-23.5, 15), (-23.5, 0)]),
        revolve("outlet", "body", BRASS, 0.6, AX, [(4, 0), (4, 13.2), (18, 13.2), (18, 16), (32, 16), (32, 10), (58, 10), (58, 0)]),
        revolve("head-base", "head", ABS, 0.8, AZ, [(-12.5, 0), (-12.5, 17), (-25, 17), (-26.5, 22), (-38, 22), (-38, 0)]),
        revolve("handwheel", "knob", ABS, 0.95, AZ, [(-35, 0), (-35, 26.5), (-70, 25.5), (-72, 24), (-87, 21.5), (-91.5, 18.5), (-93, 14.5), (-93, 0)]),
    ]
    th = [
        of("th-inlet", "inlet", "body", "wall", sectionFaces=["-v"]),
        of("th-chamber", "chamber", "body", "wall"),
        of("th-outlet", "outlet", "body", "wall", sectionFaces=["+u"]),
        of("th-head-base", "head-base", "head", "shell"),
        thermal(revolve("th-handwheel-barrel", "knob", ABS, 0.95, AZ, [(-35, 0), (-35, 26.5), (-70, 25.5), (-72, 24), (-72, 0)]), "head", "shell", sectionFaces=["-w"]),
        thermal(revolve("th-sensor-cap", "knob", ABS, 0.95, AZ, [(-72, 0), (-72, 24), (-87, 21.5), (-91.5, 18.5), (-93, 14.5), (-93, 0)]), "knob", "shell", sectionFaces=["+w"]),
    ]
    st = [
        structural(revolve("member-inlet", "body", BRASS, 0.6, AY, [(-80, 6.5), (-80, 7.5), (-30, 7.5), (-30, 11), (-2, 11), (-2, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-outlet", "body", BRASS, 0.6, AX, [(2, 6.5), (2, 10), (58, 10), (58, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-chamber", "body", BRASS, 0.6, AZ, [(12, 7.5), (12, 11), (-23.5, 11), (-23.5, 7.5)], loop=True), "body", "member"),
        structural(revolve("member-head-sleeve", "head", ABS, 0.5, AZ, [(-13, 8), (-13, 16.5), (-25, 16.5), (-25, 11.5), (-50, 11.5), (-50, 8)], loop=True), "head", "member"),
        structural(revolve("member-handwheel-hub", "knob", ABS, 0.5, AZ, [(-35, 12), (-35, 15.5), (-74, 15.5), (-74, 12)], loop=True), "knob", "member"),
        structural(revolve("support-pipe-joint", "body", BRASS, 0.3, AY, [(-46.5, 7.5), (-46.5, 13), (-31.5, 13), (-31.5, 7.5)], loop=True, role="ground"), "body", "support", anchor="pipe-joint"),
        structural(revolve("support-radiator-union", "body", BRASS, 0.3, AX, [(43, 10), (43, 13), (58, 13), (58, 10)], loop=True, role="ground"), "body", "support", anchor="radiator-union"),
        structural(ellipsoid("mass-body", "body", BRASS, 0.3, (7.5, 7.5, 7.5), (9, -15, 0)), "body", "mass"),
        structural(ellipsoid("mass-sensor", "knob", ABS, 0.3, (9, 9, 9), (0, 0, -83)), "knob", "mass"),
    ]
    decisions = [
        text("Body: drawn anew for 15 mm: the inlet is one turned column (pipe, a round collar for compression nut and socket, neck), the chamber a plain barrel with its collar, the outlet one stepped arm with a round union collar and the tailpiece; the head is a base (coupling nut and base ring as one stepped ring) and a smooth tapered handwheel without flutes, slots, index or front mark.",
             "Körper: für 15 mm neu gezeichnet: der Eingang ist eine gedrehte Säule (Rohr, ein runder Bund für Klemmmutter und Muffe, Hals), die Kammer eine glatte Trommel mit Bund, der Ausgang ein gestufter Arm mit rundem Verschraubungsbund und Schraubnippel; der Kopf ist ein Sockel (Überwurfmutter und Sockelring als ein gestufter Ring) und ein glattes, verjüngtes Handrad ohne Griffmulden, Schlitze, Index und Frontmarke."),
        text("Thermal: the three brass masses and the head base reuse the body solids with the pipe and tail ends as section faces; the handwheel is cut at the cap shoulder into the barrel (head region) and the sensor cap (knob region), the cut faces count for no area.",
             "Thermik: die drei Messingmassen und der Kopfsockel verwenden die Körper mit Rohr- und Nippelende als Schnittflächen; das Handrad ist an der Kappenschulter in die Trommel (Kopfregion) und die Fühlerkappe (Knopfregion) geschnitten, die Schnittflächen zählen nicht zur Fläche."),
        text("Structure: the three brass tubes, one sleeve for nut and base, the handwheel hub, pin and roller as 15 mm rings round the pipe at the compression fitting and round the tailpiece at the union, and the two masses; the joints and the pressure pin are below the feature and merged into the members.",
             "Tragwerk: die drei Messingrohre, eine Hülse für Mutter und Sockel, die Handradnabe, Fest- und Rollenlager als 15-mm-Ringe um das Rohr an der Klemmverschraubung und um den Schraubnippel an der Verschraubung und die beiden Massen; Gelenke und Druckstift liegen unter der Strukturgröße und gehen in den Gliedern auf."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": th, "structure": st, "decisions": decisions}


def scale200():
    """🗺️ 1:200, smallest feature 30 mm: three turned masses on the three axes."""
    body = [
        revolve("inlet", "body", BRASS, 0.6, AY, [(-80, 0), (-80, 8), (-47, 8), (-44, 13.5), (-18, 13.5), (-8, 14.5), (10, 14.5), (14, 10), (14, 0)]),
        revolve("outlet", "body", BRASS, 0.6, AX, [(6, 0), (6, 12.5), (17, 12.5), (20, 15.5), (32, 15.5), (36, 10), (58, 10), (58, 0)]),
        revolve("head", "head", ABS, 0.85, AZ, [(-9, 0), (-9, 17), (-25, 17), (-26, 22), (-34.5, 22), (-36, 26.5), (-71, 25.6), (-86, 22.5), (-91.5, 18.5), (-93, 14), (-93, 0)]),
    ]
    th = [
        of("th-inlet", "inlet", "body", "wall", sectionFaces=["-v"]),
        of("th-outlet", "outlet", "body", "wall", sectionFaces=["+u"]),
        thermal(revolve("th-head", "head", ABS, 0.85, AZ, [(-9, 0), (-9, 17), (-25, 17), (-26, 22), (-34.5, 22), (-36, 26.5), (-71, 25.6), (-71, 0)]), "head", "shell", sectionFaces=["-w"]),
        thermal(revolve("th-sensor-cap", "knob", ABS, 0.95, AZ, [(-71, 0), (-71, 25.6), (-86, 22.5), (-91.5, 18.5), (-93, 14), (-93, 0)]), "knob", "shell", sectionFaces=["+w"]),
    ]
    st = [
        structural(revolve("member-inlet", "body", BRASS, 0.6, AY, [(-80, 6.5), (-80, 9), (0, 9), (0, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-outlet", "body", BRASS, 0.6, AX, [(0, 6.5), (0, 9.5), (58, 9.5), (58, 6.5)], loop=True), "body", "member"),
        structural(revolve("member-head-sleeve", "head", ABS, 0.5, AZ, [(-9, 8), (-9, 11.5), (-48, 11.5), (-48, 8)], loop=True), "head", "member"),
        structural(revolve("member-handwheel-hub", "knob", ABS, 0.5, AZ, [(-36, 12.5), (-36, 15.5), (-76, 15.5), (-76, 12.5)], loop=True), "knob", "member"),
        structural(box("support-pipe-joint", "body", BRASS, 0.3, (15, 15, 15), (0, -39, 0), role="ground"), "body", "support", anchor="pipe-joint"),
        structural(box("support-radiator-union", "body", BRASS, 0.3, (12, 15, 15), (51, 0, 0), role="ground"), "body", "support", anchor="radiator-union"),
        structural(ellipsoid("mass-valve", "body", BRASS, 0.3, (8, 8, 8), (5, -10, -20)), "body", "mass"),
    ]
    decisions = [
        text("Body: three turned masses for a digital model at 1:200 (no printed drawing exists at this scale): the inlet column with the pipe and one collar up through the chamber, the outlet arm with its union collar, and the head from the valve to the sensor cap as one turned body with the true steps of the reference (34 mm coupling nut, 44 mm base ring, 53 mm handwheel barrel tapering 2 mm, rounded sensor cap); nut, base ring, flutes and marks are merged.",
             "Körper: drei gedrehte Massen für ein digitales Modell 1:200 (eine gedruckte Zeichnung gibt es in diesem Maßstab nicht): die Eingangssäule mit Rohr und einem Bund bis durch die Kammer, der Ausgangsarm mit Verschraubungsbund und der Kopf vom Ventil bis zur Fühlerkappe als ein gedrehter Körper mit den wahren Stufen der Referenz (Überwurfmutter 34 mm, Sockelring 44 mm, Handradtrommel 53 mm mit 2 mm Verjüngung, gerundete Fühlerkappe); Mutter, Sockelring, Griffmulden und Marken sind zusammengefasst."),
        text("Thermal: inlet and outlet carry the brass region, the head body the head region up to the cap shoulder and the front cap the knob region; only the sum of the areas is compared at this scale.",
             "Thermik: Eingang und Ausgang tragen die Messingregion, der Kopfkörper die Kopfregion bis zur Kappenschulter und die Frontkappe die Knopfregion; in diesem Maßstab wird nur die Flächensumme verglichen."),
        text("Structure: two brass tubes along the pipe and the tail, one sleeve along the head axis, the handwheel hub, block supports at the pipe joint and the radiator union and one mass for the whole valve.",
             "Tragwerk: zwei Messingrohre entlang Rohr und Nippel, eine Hülse entlang der Kopfachse, die Handradnabe, Blocklager an Rohrverschraubung und Heizkörperverschraubung und eine Masse für das ganze Ventil."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": th, "structure": st, "decisions": decisions}


def scale500():
    """🛰️ 1:500, smallest feature 75 mm: the three arms as plain tapers, still a corner valve with its head."""
    body = [
        revolve("inlet", "body", BRASS, 0.6, AY, [(-80, 0), (-80, 7.5), (-46, 7.5), (-44, 14.5), (14, 14.5), (14, 0)]),
        revolve("outlet", "body", BRASS, 0.6, AX, [(6, 0), (6, 14), (33, 14), (35, 10), (58, 10), (58, 0)]),
        revolve("head", "head", ABS, 0.85, AZ, [(-9, 0), (-9, 19), (-30, 19), (-34, 26.5), (-84, 26.5), (-93, 18), (-93, 0)]),
    ]
    th = [
        of("th-inlet", "inlet", "body", "wall", sectionFaces=["-v"]),
        of("th-outlet", "outlet", "body", "wall", sectionFaces=["+u"]),
        thermal(revolve("th-head", "head", ABS, 0.85, AZ, [(-9, 0), (-9, 19), (-30, 19), (-34, 26.5), (-74, 26.5), (-74, 0)]), "head", "shell", sectionFaces=["-w"]),
        thermal(revolve("th-sensor-cap", "knob", ABS, 0.95, AZ, [(-74, 0), (-74, 26.5), (-84, 26.5), (-93, 18), (-93, 0)]), "knob", "shell", sectionFaces=["+w"]),
    ]
    st = [
        structural(box("member-inlet", "body", BRASS, 0.6, (15, 80, 15), (0, -40, 0)), "body", "member"),
        structural(box("member-outlet", "body", BRASS, 0.6, (50.5, 13, 13), (32.75, 0, 0)), "body", "member"),
        structural(box("member-head-sleeve", "head", ABS, 0.5, (18, 18, 52.5), (0, 0, -18.75)), "head", "member"),
        structural(box("member-handwheel-hub", "knob", ABS, 0.5, (24, 24, 36), (0, 0, -63)), "knob", "member"),
        structural(box("support-pipe-joint", "body", BRASS, 0.3, (16, 12, 16), (0, -39, 0), role="ground"), "body", "support", anchor="pipe-joint"),
        structural(box("support-radiator-union", "body", BRASS, 0.3, (12, 16, 16), (51, 0, 0), role="ground"), "body", "support", anchor="radiator-union"),
        structural(ellipsoid("mass-valve", "body", BRASS, 0.3, (9, 9, 9), (5, -10, -20)), "body", "mass"),
    ]
    decisions = [
        text("Body: the valve is smaller than two features of 1:500, so the digital model keeps only what makes a corner angle valve: the 15 mm pipe stepping into the valve column, the outlet arm stepping from the union to the tailpiece, and the head as a 38 mm neck with the 53 mm barrel and a chamfered cap on the third axis (no tapers: a cone would read as a horn or a bottle); the model is minimal, the solid count cannot drop further without losing an axis.",
             "Körper: das Ventil ist kleiner als zwei Strukturgrößen von 1:500, daher behält das digitale Modell nur, was ein Winkeleckventil ausmacht: das 15-mm-Rohr, das in die Ventilsäule übergeht, den Ausgangsarm mit der Stufe von der Verschraubung zum Nippel und den Kopf als 38-mm-Hals mit der 53-mm-Trommel und gefaster Kappe auf der dritten Achse (keine Kegel: ein Kegel läse sich als Horn oder Flasche); das Modell ist minimal, die Körperzahl kann nicht weiter sinken, ohne eine Achse zu verlieren."),
        text("Thermal: the same split as at 1:200 on the simplified steps; only the sum of the areas is compared.",
             "Thermik: dieselbe Teilung wie bei 1:200 auf den vereinfachten Stufen; nur die Flächensumme wird verglichen."),
        text("Structure: four square bars along the three axes (inlet, outlet, head sleeve, handwheel hub), two block supports and one mass.",
             "Tragwerk: vier Vierkantstäbe entlang der drei Achsen (Eingang, Ausgang, Kopfhülse, Handradnabe), zwei Blocklager und eine Masse."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": th, "structure": st, "decisions": decisions}


def volumes():
    """📚 All four scales."""
    return {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}


if __name__ == "__main__":
    out = {"volumes": volumes()}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for scale, v in out["volumes"].items():
        print("1:%s body %d thermal %d structure %d" % (scale, len(v["body"]), len(v["thermal"]), len(v["structure"])))
