"""🏔️ Round 3, appliances/kitchen-extractor-hood: the handcrafted volumes of the 0.9 m pyramid chimney hood at 1:50, 1:100, 1:200 and the digital 1:500 (body, thermal, structure, decisions), every solid stated by hand; writes `volumes/appliances/kitchen-extractor-hood.json`.

Usage: python3 r3-kitchen-extractor-hood-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "appliances", "kitchen-extractor-hood.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-kitchen-extractor-hood-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

SS, MESH, PC, GALV, ABS = "stainless-steel", "filter-mesh", "lamp-diffuser", "galvanised-steel", "switch-plastic"
C, CH, CT = "canopy", "chimney", "chimney-top"


def cross(a, b):
    """✖️ The cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def dot(a, b):
    """⚫ The dot product."""
    return sum(x * y for x, y in zip(a, b))


def unit(a):
    """📏 A unit vector."""
    n = math.sqrt(dot(a, a))
    return tuple(x / n for x in a)


def matrix_quat(cu, cv, cw):
    """🧭 The unit quaternion [x, y, z, w] of the rotation whose columns are the images of u, v, w."""
    m = [[cu[i], cv[i], cw[i]] for i in range(3)]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(c, 7) for c in q]


def hip_fold(sid, tone, side, leg, t, head, foot):
    """🪢 The folded hip seam between the front and a side slope sheet: a V strip with a leg in each sheet, running from `head` below the top plate corner down to `foot` above the rim corner, where the square cut ends stay inside the sheets (side +1 = left)."""
    top = (k.TOP_D, k.TOP, side * k.TOP_HW)
    low = (k.DEPTH, k.RIM, side * k.HALF_W)
    d = unit(tuple(low[i] - top[i] for i in range(3)))
    nf = unit((k.TOP - k.RIM, k.DEPTH - k.TOP_D, 0.0))
    ns = unit((0.0, k.HALF_W - k.TOP_HW, side * (k.TOP - k.RIM)))
    ef = unit(cross(nf, d))
    if ef[2] * side > 0:
        ef = tuple(-c for c in ef)
    es = unit(cross(ns, d))
    if es[0] > 0:
        es = tuple(-c for c in es)
    a, b = ef, cross(d, ef)
    to2 = lambda p: (dot(p, a), dot(p, b))
    alpha = -t / dot(nf, es)
    inner = tuple(alpha * (ef[i] + es[i]) for i in range(3))
    ring = [tuple(leg * c for c in ef), (0.0, 0.0, 0.0), tuple(leg * c for c in es), tuple(leg * es[i] - t * ns[i] for i in range(3)), inner, tuple(leg * ef[i] - t * nf[i] for i in range(3))]
    out = k.ext(sid, C, SS, "body", tone, "yz", [to2(p) for p in ring], 0, math.dist(top, low) - head - foot)
    out["transform"] = {"translation": [k.r5(top[i] + head * d[i]) for i in range(3)], "rotation": matrix_quat(d, a, b)}
    return out


def rim(sid, tone, holes, wall=None):
    """🔲 The folded rim band of the canopy: the outer rectangle with either the inner opening of the return flange or the openings of the underside."""
    if wall is not None:
        holes = [k.rect(-k.HALF_W + wall, k.HALF_W - wall, wall, k.DEPTH - wall)]
    return k.ext(sid, C, SS, "body", tone, "zx", k.rect(-k.HALF_W, k.HALF_W, 0, k.DEPTH), 0, k.RIM, holes=holes)


def cassette_rect(c, inset=0.0):
    """🧽 The plan rectangle of a grease filter cassette in (w, u)."""
    return k.rect(c - k.CASSETTE_W / 2 + inset, c + k.CASSETTE_W / 2 - inset, k.CASSETTE_U[0] + inset, k.CASSETTE_U[1] - inset)


def lamp_rect(c):
    """💡 The plan rectangle of a lamp module in (w, u)."""
    return k.rect(c - k.LAMP_HW, c + k.LAMP_HW, k.LAMP_U[0], k.LAMP_U[1])


def scale_50():
    """🔬 1:50, smallest feature 7.5 mm."""
    t = 0.008
    openings = [cassette_rect(c) for c in k.CASSETTE_C] + [lamp_rect(c) for c in k.LAMP_C]
    body = [
        rim("rim", 0.78, None, wall=k.FLANGE),
        k.slope_front("slope-front", C, SS, "body", 0.84, t),
        k.slope_side("slope-left", C, SS, "body", 0.8, t, 1),
        k.slope_side("slope-right", C, SS, "body", 0.8, t, -1),
        k.top_plate("top-plate", C, SS, "body", 0.86, t),
        k.back_plate("back-plate", C, SS, "body", 0.7, t),
        k.ext("base-panel", C, SS, "body", 0.74, "zx", k.rect(-k.HALF_W + k.FLANGE, k.HALF_W - k.FLANGE, k.FLANGE, k.DEPTH - k.FLANGE), k.PANEL_V, k.PANEL_V + t, holes=openings),
    ]
    for name, c in zip(("left", "mid", "right"), reversed(k.CASSETTE_C)):
        body.append(k.ext(f"filter-frame-{name}", C, MESH, "body", 0.72, "zx", cassette_rect(c), k.PANEL_V, k.PANEL_V + 0.01, holes=[cassette_rect(c, 0.012)]))
        body.append(k.ext(f"filter-mesh-{name}", C, MESH, "detail", 0.5, "zx", cassette_rect(c, 0.012), k.PANEL_V + 0.0015, k.PANEL_V + 0.0095))
        body.append(k.box(f"filter-latch-{name}", C, MESH, "detail", 0.62, (0.326, 0.0005, c - 0.035), (0.348, 0.008, c + 0.035)))
    for name, c in zip(("left", "right"), reversed(k.LAMP_C)):
        body.append(k.box(f"lamp-{name}", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.004, c - k.LAMP_HW), (k.LAMP_U[1], 0.012, c + k.LAMP_HW)))
    body += [
        k.box("switch-panel", C, ABS, "detail", 0.2, (0.4935, 0.011, 0.155), (0.501, 0.034, 0.245)),
        k.box("switch-buttons", C, ABS, "detail", 0.38, (0.496, 0.0165, 0.162), (0.5035, 0.0285, 0.178), repeat={"count": 4, "translation": [0, 0, 0.02]}),
        k.ext("chimney-lower", CH, SS, "body", 0.86, "zx", k.channel(k.LOWER_HW, k.LOWER_D, k.SHEET_TUBE, 0.015), 0, k.LOWER_H),
        k.ext("chimney-upper", CT, SS, "body", 0.9, "zx", k.channel(k.UPPER_HW, k.UPPER_D, k.SHEET_TUBE, 0.015), -k.UPPER_HIDDEN, k.UPPER_H),
        k.box("louvres-left", CT, ABS, "texture", 0.15, (0.05, 0.128, k.UPPER_HW - 0.0002), (0.14, 0.132, k.UPPER_HW + 0.0004), texture=True, repeat={"count": 5, "translation": [0, 0.02, 0]}),
        k.box("louvres-right", CT, ABS, "texture", 0.15, (0.05, 0.128, -k.UPPER_HW - 0.0004), (0.14, 0.132, -k.UPPER_HW + 0.0002), texture=True, repeat={"count": 5, "translation": [0, 0.02, 0]}),
        k.tube("duct", CT, GALV, "body", 0.6, k.DUCT_R, 0.0005, -k.LOWER_H, k.UPPER_H + k.STUB),
        k.ext("wall-bracket", CT, GALV, "detail", 0.5, "zx", k.channel(k.UPPER_HW - 0.002, 0.03, 0.0015), 0.194, 0.224),
    ]
    ts = k.SHEET_CANOPY
    thermal = [
        k.th(k.box("th-rim-front", C, SS, "body", 0.78, (k.DEPTH - ts, 0, -k.HALF_W), (k.DEPTH, k.RIM, k.HALF_W), sectionFaces=["-u"]), "canopy", "shell"),
        k.th(k.box("th-rim-left", C, SS, "body", 0.78, (0, 0, k.HALF_W - ts), (k.DEPTH - ts, k.RIM, k.HALF_W), sectionFaces=["-w"]), "canopy", "shell"),
        k.th(k.box("th-rim-right", C, SS, "body", 0.78, (0, 0, -k.HALF_W), (k.DEPTH - ts, k.RIM, -k.HALF_W + ts), sectionFaces=["+w"]), "canopy", "shell"),
        k.th(k.box("th-rim-back", C, SS, "body", 0.78, (0, 0, -k.HALF_W + ts), (ts, k.RIM, k.HALF_W - ts), sectionFaces=["+u"]), "canopy", "shell"),
        k.th(k.ext("th-rim-flange", C, SS, "body", 0.78, "zx", k.rect(-k.HALF_W, k.HALF_W, 0, k.DEPTH), 0, ts, holes=[k.rect(-k.HALF_W + k.FLANGE, k.HALF_W - k.FLANGE, k.FLANGE, k.DEPTH - k.FLANGE)], sectionFaces=["+v"]), "canopy", "shell"),
        k.th(k.slope_front("th-slope-front", C, SS, "body", 0.84, ts, sectionFaces=["-v"]), "canopy", "shell"),
        k.th(k.slope_side("th-slope-left", C, SS, "body", 0.8, ts, 1, sectionFaces=["-v"]), "canopy", "shell"),
        k.th(k.slope_side("th-slope-right", C, SS, "body", 0.8, ts, -1, sectionFaces=["-v"]), "canopy", "shell"),
        k.th(k.top_plate("th-top-plate", C, SS, "body", 0.86, ts, sectionFaces=["-v"]), "canopy", "shell"),
        k.th(k.back_plate("th-back-plate", C, SS, "body", 0.7, ts), "canopy", "shell", sectionFaces=["+u"]),
        k.th(k.ext("th-base-panel", C, SS, "body", 0.74, "zx", k.rect(-k.HALF_W + k.FLANGE, k.HALF_W - k.FLANGE, k.FLANGE, k.DEPTH - k.FLANGE), k.PANEL_V, k.PANEL_V + ts, holes=openings, sectionFaces=["+v"]), "canopy", "shell"),
    ]
    for name, c in zip(("left", "mid", "right"), reversed(k.CASSETTE_C)):
        thermal.append(k.th(k.ext(f"th-filter-{name}", C, MESH, "body", 0.6, "zx", cassette_rect(c), k.PANEL_V, k.PANEL_V + 0.01), "filter", "mesh"))
    for name, c in zip(("left", "right"), reversed(k.LAMP_C)):
        thermal.append(k.th(k.box(f"th-lamp-{name}", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.004, c - k.LAMP_HW), (k.LAMP_U[1], 0.006, c + k.LAMP_HW)), "lamps", "diffuser"))
    thermal += [
        k.ref("th-chimney-lower", "chimney-lower", region="chimney", layer="tube"),
        k.ref("th-chimney-upper", "chimney-upper", region="chimney", layer="tube"),
        k.th(k.disc("th-extract-outlet", CT, GALV, "detail", 0.3, k.DUCT_R - 0.0005, k.UPPER_H + k.STUB - 0.0075, k.UPPER_H + k.STUB), "chimney", tags=["emitter", "extract"]),
    ]
    structure = [
        k.st(k.back_plate("member-back-plate", C, SS, "body", 0.6, 0.0075), C, "member"),
        k.st(k.top_plate("member-top-plate", C, SS, "body", 0.6, 0.0075), C, "member"),
        k.st(hip_fold("member-hip-left", 0.6, 1, 0.025, 0.0012, 0.012, 0.032), C, "member"),
        k.st(hip_fold("member-hip-right", 0.6, -1, 0.025, 0.0012, 0.012, 0.032), C, "member"),
        k.ref("member-rim", "rim", member=C, part="member"),
        k.ref("member-chimney-lower", "chimney-lower", member=CH, part="member"),
        k.st(k.ext("member-chimney-upper", CT, SS, "body", 0.6, "zx", k.channel(k.UPPER_HW, k.UPPER_D, k.SHEET_TUBE, 0.015), 0, k.UPPER_H), CT, "member"),
        k.ref("member-wall-bracket", "wall-bracket", member=CT, part="member"),
        k.st(k.ext("joint-chimney-seat", CH, GALV, "detail", 0.5, "zx", k.channel(k.LOWER_HW - 0.002, k.LOWER_D - 0.002, 0.0015), 0, 0.02), CH, "joint"),
        k.st(k.ext("joint-telescope", CT, SS, "detail", 0.5, "zx", k.channel(k.UPPER_HW, k.UPPER_D, k.SHEET_TUBE, 0.015), -k.UPPER_HIDDEN, 0), CT, "joint"),
        k.st(k.rev("support-wall-hook", C, GALV, "detail", 0.4, [(-0.035, 0), (-0.035, 0.003), (0.0075, 0.003), (0.0075, 0.006), (0.0105, 0.006), (0.0105, 0)], [0, k.HOOK_V, -k.SCREW_HW], [1, 0, 0], repeat={"count": 2, "translation": [0, 0, 2 * k.SCREW_HW]}), C, "support", anchor="wall-hook"),
        k.st(k.rev("support-wall-rest", C, GALV, "detail", 0.4, [(-0.0075, 0), (-0.0075, 0.01), (0, 0.01), (0, 0)], [0, k.REST_V, -k.SCREW_HW], [1, 0, 0], repeat={"count": 2, "translation": [0, 0, 2 * k.SCREW_HW]}), C, "support", anchor="wall-rest"),
        k.st(k.disc("mass-fan-volute", C, GALV, "body", 0.45, 0.092, -0.05, 0.05, at=k.FAN_AT, axis=(1, 0, 0)), C, "mass"),
        k.st(k.disc("mass-fan-motor", C, GALV, "body", 0.4, 0.045, 0.05, 0.095, at=k.FAN_AT, axis=(1, 0, 0)), C, "mass"),
        k.st(k.box("mass-fan-outlet", C, GALV, "body", 0.45, (k.DUCT_X - 0.06, 0.19, -0.06), (k.DUCT_X + 0.06, 0.2385, 0.06)), C, "mass"),
        k.st(k.ell("mass-chimney", CH, SS, "body", 0.5, (0.02, 0.02, 0.02), (0.13, 0.2, 0)), CH, "mass"),
        k.st(k.ell("mass-chimney-top", CT, SS, "body", 0.5, (0.022, 0.022, 0.022), (0.12, 0.06, 0)), CT, "mass"),
    ]
    decisions = [
        k.text("Body: a 0.9 m pyramid chimney hood as built. The canopy is the folded rim band of 45 mm with its 12 mm return flange, three sloping sheets (front and two sides) meeting in two hips, the top plate of 0.26 m by 0.2 m and the back sheet on the wall; the sloping sheets are trapezoids in their own planes, so every face and both hips are exact. Underneath, 6 mm above the rim edge: the base panel with three grease filter cassettes of 0.28 m by 0.32 m (frame, mesh pack, latch) and two lamp modules of 0.12 m by 0.05 m in the front strip. The switch panel with four buttons sits on the front rim.",
               "Körper: eine 0,9 m breite Pyramiden-Kaminhaube, wie sie gebaut ist. Der Haubenkörper besteht aus dem gekanteten Randband von 45 mm mit 12 mm Umbug, drei geneigten Blechen (Front und zwei Seiten), die sich in zwei Graten treffen, der Deckplatte von 0,26 m mal 0,2 m und dem Rückblech an der Wand; die geneigten Bleche sind Trapeze in ihrer eigenen Ebene, sodass jede Fläche und beide Grate exakt sind. Darunter, 6 mm über der Randkante: das Bodenblech mit drei Fettfilterkassetten von 0,28 m mal 0,32 m (Rahmen, Gitterpaket, Riegel) und zwei Lampenmodulen von 0,12 m mal 0,05 m im vorderen Streifen. Die Schalterblende mit vier Tasten sitzt am vorderen Rand."),
        k.text("The sheets of the canopy are drawn 8 mm thick (the smallest feature) towards the inside, where the closed underside hides them; the outer faces are true. The chimney is two U channels of 0.6 mm sheet, open to the wall, with 15 mm return lips: the lower one 0.25 m wide, 0.19 m deep and 0.4 m high, the upper one 6 mm narrower, telescoped 146 mm into the lower one and 254 mm visible, with five louvre slots on each side. Inside runs the 150 mm duct from the top plate to 15 mm above the chimney, where it is cut (it continues to the ceiling); the wall bracket of the upper tube is seen from above.",
               "Die Bleche des Haubenkörpers sind nach innen 8 mm dick gezeichnet (das kleinste Merkmal), wo die geschlossene Unterseite sie verbirgt; die Außenflächen sind maßgetreu. Der Kamin besteht aus zwei U-Schächten aus 0,6 mm Blech, zur Wand offen, mit 15 mm Umbug: der untere 0,25 m breit, 0,19 m tief und 0,4 m hoch, der obere 6 mm schmaler, 146 mm in den unteren eingeschoben und 254 mm sichtbar, mit fünf Lüftungsschlitzen je Seite. Innen läuft das 150-mm-Rohr von der Deckplatte bis 15 mm über den Kamin, wo es geschnitten ist (es führt weiter zur Decke); der Wandwinkel des oberen Schachts ist von oben zu sehen."),
        k.text("Thermal: the canopy region is the real 0.6 mm sheet shell (rim band, return flange, three slopes, top plate, back sheet, base panel), the filter region the three 10 mm cassettes, the lamps region the two 2 mm diffusers, the chimney region both U channels at their sheet thickness. The extract emitter is the outlet patch in the mouth of the duct. Filter and lamps lie under the canopy and are seen in no elevation or plan, so they are declared hidden regions with solids of their own.",
               "Thermik: die Region Haube ist die echte Blechschale von 0,6 mm (Randband, Umbug, drei Schrägen, Deckplatte, Rückblech, Bodenblech), die Region Filter die drei Kassetten von 10 mm, die Region Lampen die zwei Diffusoren von 2 mm, die Region Kamin beide U-Schächte in ihrer Blechdicke. Der Abluft-Emitter ist die Austrittsfläche in der Mündung des Rohrs. Filter und Lampen liegen unter der Haube und sind in keiner Ansicht und keiner Draufsicht zu sehen; sie sind daher als verdeckte Regionen mit eigenen Körpern erklärt."),
        k.text("Structure: the load path of the hanging hood. The back sheet hangs on two keyhole screws 0.27 m apart at 0.19 m (pin) and bears on two spacer pads at 0.03 m (roller); the top plate carries the fan unit (volute, motor, outlet box: the concentrated mass) and the lower chimney channel on its seat flange; the two folded hips and the rim band carry the skirt; the upper channel slides in the lower one (the telescope overlap is the joint) and is screwed to the wall bracket. The masses of the two chimney channels are marked at their centres.",
               "Tragwerk: der Lastpfad der hängenden Haube. Das Rückblech hängt an zwei Schlüssellochschrauben im Abstand von 0,27 m auf 0,19 m Höhe (Festlager) und liegt auf zwei Distanzpuffern bei 0,03 m an (Rollenlager); die Deckplatte trägt die Lüftereinheit (Spiralgehäuse, Motor, Ausblaskasten: die Einzelmasse) und den unteren Kaminschacht auf seinem Sitzflansch; die zwei gekanteten Grate und das Randband tragen die Schürze; der obere Schacht gleitet im unteren (die Teleskopüberdeckung ist das Gelenk) und ist am Wandwinkel verschraubt. Die Massen der beiden Kaminschächte sind in ihren Schwerpunkten markiert."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    """🔎 1:100, smallest feature 15 mm."""
    t = 0.015
    field = k.rect(k.CASSETTE_C[0] - k.CASSETTE_W / 2, k.CASSETTE_C[2] + k.CASSETTE_W / 2, k.CASSETTE_U[0], k.CASSETTE_U[1])
    openings = [field] + [lamp_rect(c) for c in k.LAMP_C]
    filters = k.ext("filters", C, MESH, "body", 0.55, "zx", field, k.PANEL_V, k.PANEL_V + t)
    lamps = [k.box(f"lamp-{name}", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.003, c - k.LAMP_HW), (k.LAMP_U[1], 0.003 + t, c + k.LAMP_HW)) for name, c in zip(("left", "right"), reversed(k.LAMP_C))]
    body = [
        rim("rim", 0.78, openings),
        k.slope_front("slope-front", C, SS, "body", 0.84, t),
        k.slope_side("slope-left", C, SS, "body", 0.8, t, 1),
        k.slope_side("slope-right", C, SS, "body", 0.8, t, -1),
        k.top_plate("top-plate", C, SS, "body", 0.86, t),
        k.back_plate("back-plate", C, SS, "body", 0.7, t),
        filters,
        *lamps,
        k.ext("chimney-lower", CH, SS, "body", 0.86, "zx", k.channel(k.LOWER_HW, k.LOWER_D, k.SHEET_TUBE), 0, k.LOWER_H),
        k.ext("chimney-upper", CT, SS, "body", 0.9, "zx", k.channel(k.LOWER_HW, k.LOWER_D, k.SHEET_TUBE), 0, k.UPPER_H),
        k.tube("duct", CT, GALV, "body", 0.6, k.DUCT_R, 0.0005, 0, k.UPPER_H + k.STUB),
    ]
    thermal = [
        k.ref("th-rim", "rim", region="canopy", sectionFaces=["+v"]),
        k.ref("th-slope-front", "slope-front", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-left", "slope-left", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-right", "slope-right", region="canopy", sectionFaces=["-v"]),
        k.ref("th-top-plate", "top-plate", region="canopy", sectionFaces=["-v"]),
        k.ref("th-back-plate", "back-plate", region="canopy", sectionFaces=["+u"]),
        k.th(k.ext("th-filters", C, MESH, "body", 0.55, "zx", field, k.PANEL_V, k.PANEL_V + 0.01), "filter", "mesh"),
        *[k.th(k.box(f"th-lamp-{name}", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.004, c - k.LAMP_HW), (k.LAMP_U[1], 0.006, c + k.LAMP_HW)), "lamps", "diffuser") for name, c in zip(("left", "right"), reversed(k.LAMP_C))],
        k.ref("th-chimney-lower", "chimney-lower", region="chimney"),
        k.ref("th-chimney-upper", "chimney-upper", region="chimney"),
        k.th(k.disc("th-extract-outlet", CT, GALV, "detail", 0.3, k.DUCT_R - 0.0005, k.UPPER_H, k.UPPER_H + k.STUB), "chimney", tags=["emitter", "extract"]),
    ]
    structure = [
        k.st(k.back_plate("member-back-plate", C, SS, "body", 0.6, t), C, "member"),
        k.st(hip_fold("member-hip-left", 0.6, 1, 0.03, 0.003, 0.015, 0.04), C, "member"),
        k.st(hip_fold("member-hip-right", 0.6, -1, 0.03, 0.003, 0.015, 0.04), C, "member"),
        k.st(rim("member-rim", 0.6, None, wall=t), C, "member"),
        k.ref("member-top-plate", "top-plate", member=C, part="member"),
        k.ref("member-chimney-lower", "chimney-lower", member=CH, part="member"),
        k.ref("member-chimney-upper", "chimney-upper", member=CT, part="member"),
        k.st(k.box("joint-chimney-seat", CH, GALV, "detail", 0.5, (0.004, 0, -k.LOWER_HW + 0.004), (k.LOWER_D - 0.004, t, k.LOWER_HW - 0.004)), CH, "joint"),
        k.st(k.box("joint-wall-bracket", CT, GALV, "detail", 0.5, (0, 0.194, -k.UPPER_HW + 0.002), (t, 0.224, k.UPPER_HW - 0.002)), CT, "joint"),
        k.st(k.disc("support-wall-hook", C, GALV, "detail", 0.4, 0.0075, -0.03, 0.015, at=(0, k.HOOK_V, -k.SCREW_HW), axis=(1, 0, 0), repeat={"count": 2, "translation": [0, 0, 2 * k.SCREW_HW]}), C, "support", anchor="wall-hook"),
        k.st(k.disc("support-wall-rest", C, GALV, "detail", 0.4, 0.012, -0.015, 0, at=(0, k.REST_V, -k.SCREW_HW), axis=(1, 0, 0), repeat={"count": 2, "translation": [0, 0, 2 * k.SCREW_HW]}), C, "support", anchor="wall-rest"),
        k.st(k.disc("mass-fan-unit", C, GALV, "body", 0.45, 0.092, -0.05, 0.08, at=k.FAN_AT, axis=(1, 0, 0)), C, "mass"),
        k.st(k.ell("mass-chimney", CH, SS, "body", 0.5, (0.02, 0.02, 0.02), (0.13, 0.2, 0)), CH, "mass"),
        k.st(k.ell("mass-chimney-top", CT, SS, "body", 0.5, (0.022, 0.022, 0.022), (0.12, 0.13, 0)), CT, "mass"),
    ]
    decisions = [
        k.text("Body: the pyramid hood in eleven masses. The rim band is one block with the openings of the underside cut into it; the three filter cassettes are one filter field of 0.86 m by 0.32 m and the two lamp modules stay as blocks in the front strip. The three sloping sheets, the top plate and the back sheet are 15 mm thick towards the hidden inside; their outer faces and the two hips are exact. Switch panel, buttons, latches, cassette frames, louvres, return lips and the wall bracket are below 15 mm and are left out.",
               "Körper: die Pyramidenhaube in elf Massen. Das Randband ist ein Block, in den die Öffnungen der Unterseite geschnitten sind; die drei Filterkassetten sind ein Filterfeld von 0,86 m mal 0,32 m, die zwei Lampenmodule bleiben als Blöcke im vorderen Streifen. Die drei geneigten Bleche, die Deckplatte und das Rückblech sind zur verdeckten Innenseite 15 mm dick; ihre Außenflächen und die beiden Grate sind exakt. Schalterblende, Tasten, Riegel, Kassettenrahmen, Lüftungsschlitze, Umbüge und der Wandwinkel liegen unter 15 mm und entfallen."),
        k.text("The chimney is two plain U channels of sheet, open to the wall, the upper one 6 mm narrower and only its visible 254 mm; the 150 mm duct shows in the open top and ends 15 mm above it.",
               "Der Kamin besteht aus zwei glatten U-Schächten aus Blech, zur Wand offen, der obere 6 mm schmaler und nur mit seinen sichtbaren 254 mm; das 150-mm-Rohr ist in der offenen Oberseite zu sehen und endet 15 mm darüber."),
        k.text("Thermal: the canopy region is the outer surface of rim, slopes, top plate and back sheet; the filter region is the filter field, the lamps region the two lamp blocks (both hidden regions with their own solids); the chimney region is both channels; the extract outlet is the patch in the duct mouth. Layers are merged into the surfaces.",
               "Thermik: die Region Haube ist die Außenfläche von Rand, Schrägen, Deckplatte und Rückblech; die Region Filter ist das Filterfeld, die Region Lampen sind die zwei Lampenblöcke (beides verdeckte Regionen mit eigenen Körpern); die Region Kamin sind beide Schächte; der Abluftaustritt ist die Fläche in der Rohrmündung. Die Schichten gehen in den Oberflächen auf."),
        k.text("Structure: back sheet on the two hook screws and the two spacer pads, the top plate with the fan unit as one mass, the two hips as folded strips running down to the rim band, which is a closed frame of 15 mm, both chimney channels with the seat on the top plate and the wall bracket of the upper channel as joints; the telescope overlap is merged into the channels.",
               "Tragwerk: Rückblech an den zwei Hakenschrauben und den zwei Distanzpuffern, die Deckplatte mit der Lüftereinheit als einer Masse, die zwei Grate als gekantete Streifen bis zum Randband, das ein geschlossener Rahmen von 15 mm ist, beide Kaminschächte mit dem Sitz auf der Deckplatte und dem Wandwinkel des oberen Schachts als Gelenken; die Teleskopüberdeckung geht in den Schächten auf."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200():
    """🔭 1:200, smallest feature 30 mm."""
    t = 0.03
    filters = k.box("th-filters", C, MESH, "body", 0.55, (k.CASSETTE_U[0], 0.006, k.CASSETTE_C[0] - k.CASSETTE_W / 2), (k.CASSETTE_U[1], 0.016, k.CASSETTE_C[2] + k.CASSETTE_W / 2))
    lamps = k.box("th-lamps", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.004, -0.26), (k.LAMP_U[1], 0.006, -0.14), repeat={"count": 2, "translation": [0, 0, 0.4]})
    body = [
        k.box("rim", C, SS, "body", 0.78, (0, 0, -k.HALF_W), (k.DEPTH, k.RIM, k.HALF_W)),
        k.slope_front("slope-front", C, SS, "body", 0.84, t),
        k.slope_side("slope-left", C, SS, "body", 0.8, t, 1),
        k.slope_side("slope-right", C, SS, "body", 0.8, t, -1),
        k.top_plate("top-plate", C, SS, "body", 0.86, t),
        k.back_plate("back-plate", C, SS, "body", 0.7, t),
        k.ext("chimney-lower", CH, SS, "body", 0.86, "zx", k.channel(k.LOWER_HW, k.LOWER_D, 0.003), 0, k.LOWER_H),
        k.ext("chimney-upper", CT, SS, "body", 0.9, "zx", k.channel(k.LOWER_HW, k.LOWER_D, 0.003), 0, k.UPPER_H + k.STUB),
    ]
    thermal = [
        k.ref("th-rim", "rim", region="canopy", sectionFaces=["+v"]),
        k.ref("th-slope-front", "slope-front", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-left", "slope-left", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-right", "slope-right", region="canopy", sectionFaces=["-v"]),
        k.ref("th-top-plate", "top-plate", region="canopy", sectionFaces=["-v"]),
        k.ref("th-back-plate", "back-plate", region="canopy", sectionFaces=["+u"]),
        k.th(filters, "filter", "mesh"),
        k.th(lamps, "lamps", "diffuser"),
        k.ref("th-chimney-lower", "chimney-lower", region="chimney"),
        k.ref("th-chimney-upper", "chimney-upper", region="chimney"),
        k.th(k.disc("th-extract-outlet", CT, GALV, "detail", 0.3, k.DUCT_R, k.UPPER_H + k.STUB - t, k.UPPER_H + k.STUB), "chimney", tags=["emitter", "extract"]),
    ]
    structure = [
        k.ref("member-back-plate", "back-plate", member=C, part="member"),
        k.ref("member-top-plate", "top-plate", member=C, part="member"),
        k.ref("member-chimney-lower", "chimney-lower", member=CH, part="member"),
        k.ref("member-chimney-upper", "chimney-upper", member=CT, part="member"),
        k.st(k.box("support-wall-hook", C, GALV, "detail", 0.4, (-0.03, k.HOOK_V - 0.015, -k.SCREW_HW - 0.015), (0, k.HOOK_V + 0.015, k.SCREW_HW + 0.015)), C, "support", anchor="wall-hook"),
        k.st(k.box("support-wall-rest", C, GALV, "detail", 0.4, (-0.03, k.REST_V - 0.015, -k.SCREW_HW - 0.015), (0, k.REST_V + 0.015, k.SCREW_HW + 0.015)), C, "support", anchor="wall-rest"),
        k.st(k.ell("mass-fan-unit", C, GALV, "body", 0.45, (0.07, 0.07, 0.09), k.FAN_AT), C, "mass"),
    ]
    decisions = [
        k.text("Body: eight masses. The rim is a closed slab of 45 mm, the pyramid is the back sheet, the top plate and the three sloping sheets, all 30 mm thick towards the hidden inside (faces and hips exact), the chimney is two U channels of one width with the duct stub merged into the upper one up to 0.915 m. Filters, lamps, the telescopic step of 3 mm and the duct are below 30 mm or hidden and are left out of the body.",
               "Körper: acht Massen. Der Rand ist eine geschlossene Platte von 45 mm, die Pyramide besteht aus Rückblech, Deckplatte und den drei geneigten Blechen, alle zur verdeckten Innenseite 30 mm dick (Flächen und Grate exakt), der Kamin zwei U-Schächte gleicher Breite, wobei der Rohrstutzen im oberen bis 0,915 m aufgeht. Filter, Lampen, der Teleskopabsatz von 3 mm und das Rohr liegen unter 30 mm oder sind verdeckt und entfallen im Körper."),
        k.text("Thermal: the canopy region is the outer surface of rim, slopes, top plate and back sheet; the hidden filter field and the two lamp blocks are plates inside the rim slab; the chimney region is both channels; the extract outlet is a patch in the open top of the upper channel.",
               "Thermik: die Region Haube ist die Außenfläche von Rand, Schrägen, Deckplatte und Rückblech; das verdeckte Filterfeld und die zwei Lampenblöcke sind Platten in der Randplatte; die Region Kamin sind beide Schächte; der Abluftaustritt ist eine Fläche in der offenen Oberseite des oberen Schachts."),
        k.text("Structure: the back sheet on one hook strip and one rest strip (each screw pair merged), the top plate with the fan unit as one mass, and both chimney channels; hips, joints and the masses of the channels are below the scale.",
               "Tragwerk: das Rückblech an einem Hakenstreifen und einem Auflagestreifen (jedes Schraubenpaar zusammengefasst), die Deckplatte mit der Lüftereinheit als einer Masse und beide Kaminschächte; Grate, Gelenke und die Massen der Schächte liegen unter dem Maßstab."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    """🛰️ 1:500, a digital scale without a drawing: the hood is 1 mm by 1.8 mm on paper."""
    t = 0.05
    body = [
        k.box("rim", C, SS, "body", 0.78, (0, 0, -k.HALF_W), (k.DEPTH, k.RIM, k.HALF_W)),
        k.slope_front("slope-front", C, SS, "body", 0.84, t),
        k.slope_side("slope-left", C, SS, "body", 0.8, t, 1),
        k.slope_side("slope-right", C, SS, "body", 0.8, t, -1),
        k.top_plate("top-plate", C, SS, "body", 0.86, t),
        k.back_plate("back-plate", C, SS, "body", 0.7, t),
        k.box("chimney-lower", CH, SS, "body", 0.86, (0, 0, -k.LOWER_HW), (k.LOWER_D, k.LOWER_H, k.LOWER_HW)),
        k.box("chimney-upper", CT, SS, "body", 0.9, (0, 0, -k.LOWER_HW), (k.LOWER_D, k.UPPER_H + k.STUB, k.LOWER_HW)),
    ]
    thermal = [
        k.ref("th-rim", "rim", region="canopy", sectionFaces=["+v"]),
        k.ref("th-slope-front", "slope-front", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-left", "slope-left", region="canopy", sectionFaces=["-v"]),
        k.ref("th-slope-right", "slope-right", region="canopy", sectionFaces=["-v"]),
        k.ref("th-top-plate", "top-plate", region="canopy", sectionFaces=["-v"]),
        k.ref("th-back-plate", "back-plate", region="canopy", sectionFaces=["+u"]),
        k.th(k.box("th-filters", C, MESH, "body", 0.55, (k.CASSETTE_U[0], 0.006, -0.428), (k.CASSETTE_U[1], 0.016, 0.428)), "filter", "mesh"),
        k.th(k.box("th-lamps", C, PC, "detail", 0.97, (k.LAMP_U[0], 0.004, -0.2 - 2 * k.LAMP_HW), (k.LAMP_U[1], 0.006, -0.2 + 2 * k.LAMP_HW)), "lamps", "diffuser"),
        k.ref("th-chimney-lower", "chimney-lower", region="chimney"),
        k.ref("th-chimney-upper", "chimney-upper", region="chimney"),
        k.th(k.disc("th-extract-outlet", CT, GALV, "detail", 0.3, k.DUCT_R, k.UPPER_H + k.STUB - 0.02, k.UPPER_H + k.STUB + 0.003), "chimney", tags=["emitter", "extract"]),
    ]
    hanger = [(0, k.REST_V - 0.02), (0.02, k.REST_V - 0.02), (0.02, k.TOP - 0.02), (k.TOP_D, k.TOP - 0.02), (k.TOP_D, k.TOP), (0, k.TOP)]
    structure = [
        k.st(k.ext("member-hanger", C, SS, "body", 0.6, "xy", hanger, -k.TOP_HW, k.TOP_HW), C, "member"),
        k.st(k.ext("member-chimney", CH, SS, "body", 0.6, "zx", k.channel(k.LOWER_HW, k.LOWER_D, 0.02), 0, k.LOWER_H), CH, "member"),
        k.st(k.ext("member-chimney-top", CT, SS, "body", 0.6, "zx", k.channel(k.LOWER_HW, k.LOWER_D, 0.02), 0, k.UPPER_H), CT, "member"),
        k.st(k.box("support-wall-hook", C, GALV, "detail", 0.4, (-0.04, k.HOOK_V - 0.02, -k.SCREW_HW - 0.02), (0, k.HOOK_V + 0.02, k.SCREW_HW + 0.02)), C, "support", anchor="wall-hook"),
        k.st(k.box("support-wall-rest", C, GALV, "detail", 0.4, (-0.04, k.REST_V - 0.02, -k.SCREW_HW - 0.02), (0, k.REST_V + 0.02, k.SCREW_HW + 0.02)), C, "support", anchor="wall-rest"),
        k.st(k.box("mass-fan-unit", C, GALV, "body", 0.45, (0.03, 0.06, -0.09), (0.18, 0.21, 0.09)), C, "mass"),
    ]
    decisions = [
        k.text("Digital scale without a drawing: at 1:500 the hood is 1 mm by 1.8 mm. Body: eight masses. The canopy is the closed rim slab of 45 mm and the pyramid above it, built from the three sloping plates, the top plate of 0.26 m by 0.2 m and the back plate, each 50 mm thick towards the inside so that they close into one mass with the true slopes (front 34 degrees, sides 32 degrees) and the two exact hips; the chimney is two closed prisms of 0.25 m by 0.19 m, the lower 0.4 m and the upper 0.269 m high with the duct stub merged in, up to 0.915 m. The underside, the telescopic step and the duct are not modelled.",
               "Digitaler Maßstab ohne Zeichnung: bei 1:500 misst die Haube 1 mm mal 1,8 mm. Körper: acht Massen. Der Haubenkörper ist die geschlossene Randplatte von 45 mm und die Pyramide darüber, gebaut aus den drei geneigten Platten, der Deckplatte von 0,26 m mal 0,2 m und der Rückplatte, jede nach innen 50 mm dick, sodass sie sich zu einer Masse mit den wahren Schrägen (Front 34 Grad, Seiten 32 Grad) und den zwei exakten Graten schließen; der Kamin besteht aus zwei geschlossenen Prismen von 0,25 m mal 0,19 m, das untere 0,4 m und das obere 0,269 m hoch mit aufgegangenem Rohrstutzen, bis 0,915 m. Unterseite, Teleskopabsatz und Rohr sind nicht modelliert."),
        k.text("Thermal: the canopy region is the outer surface of the rim slab and the five plates, the chimney region the surface of both prisms; the hidden filter field of 0.86 m by 0.32 m and one lamp strip for both modules lie as real plates inside the rim slab; the extract outlet is a patch 3 mm proud of the top of the upper prism.",
               "Thermik: die Region Haube ist die Außenfläche der Randplatte und der fünf Platten, die Region Kamin die Oberfläche beider Prismen; das verdeckte Filterfeld von 0,86 m mal 0,32 m und ein Lampenstreifen für beide Module liegen als echte Platten in der Randplatte; der Abluftaustritt ist eine Fläche, die 3 mm über der Oberseite des oberen Prismas steht."),
        k.text("Structure: one hanger angle, the strip of the back sheet between the screws with the top plate, on a hook strip and a rest strip; the two chimney channels standing on it, open to the wall; the fan unit as one block mass under the top plate.",
               "Tragwerk: ein Hängewinkel, der Streifen des Rückblechs zwischen den Schrauben mit der Deckplatte, an einem Hakenstreifen und einem Auflagestreifen; die zwei darauf stehenden, zur Wand offenen Kaminschächte; die Lüftereinheit als eine Blockmasse unter der Deckplatte."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    volumes = {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}
    with open(SOURCE, "w", encoding="utf-8") as fh:
        json.dump({"volumes": volumes}, fh, indent=1, ensure_ascii=False)
        fh.write("\n")
    for key in ("50", "100", "200", "500"):
        v = volumes[key]
        print(key, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]))


if __name__ == "__main__":
    main()
