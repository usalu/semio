"""✏️ Round 3 propane patio heater: the 2D drawings of every scale and view, drawn from the kit's geometry (ruling E1), and the representation decisions.

Elevation shapes are in the bone-local frame of the upright bones (x up the bone, y = -X); front shapes use x = world Z and y = world height; plan shapes x = world X (ahead) and y = world Z.

@see r3-patio-heater-kit.py — the geometry
"""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-patio-heater-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FRONT_SIGN = 1
PLAN_SIGN = 1


def mirror(right):
    """🪞 A symmetric outline from its right half (r, y) listed bottom to top."""
    left = [[-r, y] for r, y in reversed(right) if r != 0]
    return right + left


def shape(sid, bone, role, z, filled, tone, kind, region=None, material=None, **geo):
    """🖍️ One 2D shape with its common fields."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    out.update(geo)
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    return out


def poly(sid, bone, role, z, filled, tone, pts, closed=True, **kw):
    """📈 A polyline or polygon shape."""
    return shape(sid, bone, role, z, filled, tone, "poly", points=[[K.r6(x), K.r6(y)] for x, y in pts], closed=closed, **kw)


def side(view, bone, pts):
    """🔁 World side-profile points (X or Z, Y) into the frame of the view."""
    if view == "elevation":
        return [K.local2(bone, x, y) for x, y in pts]
    return [[FRONT_SIGN * x, y] for x, y in pts]


def base_half(scale):
    """🛢️ Right half (r, Y) of the base silhouette: pan, housing wall, lid and socket."""
    if scale == 50:
        e = 0.009
        pts = [[K.DISH_R - e, 0.0]] + [[c, u] for u, c in K.arc(e, K.DISH_R - e, e, -90, 0, 3)[1:]]
        pts += [[c, u] for u, c in K.arc(K.DISH_TOP - e, K.DISH_R - e, e, 0, 90, 3)]
    elif scale == 100:
        e = 0.015
        pts = [[K.DISH_R - e, 0.0], [K.DISH_R, e], [K.DISH_R, K.DISH_TOP - 0.01], [K.DISH_R - 0.01, K.DISH_TOP]]
    elif scale == 200:
        pts = [[K.DISH_R, 0.0], [K.DISH_R, K.DISH_TOP]]
    else:
        return [[0.225, 0.0], [0.225, 0.70], [0.15, 0.73], [0.0375, 0.755]]
    pts += [[K.HOUSING_R, K.DISH_TOP], [K.HOUSING_R, K.WALL_TOP]]
    pts += [[r, y] for y, r in K.lid_outer(8 if scale <= 100 else 4)[1:-1]]
    if scale == 50:
        pts += [[K.SOCKET_R, 0.7295], [K.SOCKET_R, 0.750], [0.046, 0.755]]
    else:
        pts += [[K.SOCKET_R, 0.7295], [K.SOCKET_R, 0.755]]
    return pts


def head_half(scale):
    """🫖 Right half (r, Y) of the valve housing and burner deck."""
    return [[r, K.HEAD_Y + u] for u, r in K.head_profile(scale)[1:-1]]


def emitter_half(scale):
    """🔆 Right half (r, Y) of the emitter screen with bands and cap."""
    return [[r, K.HEAD_Y + u] for u, r in K.emitter_half(scale)]


def dome_half(scale):
    """☂️ Right half (r, Y) of the dome silhouette from the rim bead up to the crown."""
    y0 = K.REFLECTOR_Y
    if scale == 50:
        pts = [[K.DOME_R, y0 + K.RIM_U - 0.009], [0.3985, y0 + K.RIM_U - 0.007], [0.400, y0 + K.RIM_U - 0.003], [0.3985, y0 + K.RIM_U + 0.001]]
        pts += [[r, y0 + u] for u, r in K.dome_curve(K.DOME_R, 0.0, 14)[:-1]]
    elif scale <= 200:
        pts = [[0.400, y0 + K.RIM_U - 0.004]]
        pts += [[r, y0 + u] for u, r in K.dome_curve(0.400, 0.0, 9 if scale == 100 else 6)[:-1]]
    else:
        pts = [[0.400, y0 + K.RIM_U - 0.004], [0.33, y0 + K.RIM_U + 0.045], [0.20, y0 + K.RIM_U + 0.10], [0.08, y0 + K.RIM_U + 0.14], [0.0, y0 + K.FINIAL_TOP_U]]
    return pts


def finial_half():
    """🔘 Right half (r, Y) of the finial."""
    return [[r, K.REFLECTOR_Y + u] for u, r in K.finial_profile(6)[1:-1]]


def spacer_xz():
    """📍 World (X, Z) of the three spacer studs."""
    out = []
    for k in range(3):
        phi = 2 * math.pi * k / 3
        out.append((-0.095 * math.cos(phi), 0.095 * math.sin(phi)))
    return out


def rod_positions():
    """🧷 Lateral positions of the guard rods seen from the side (both sides coincide)."""
    return sorted({round(K.ROD_RC * math.cos(math.pi / 8 + k * math.pi / 4), 6) for k in range(8)})


def louvres():
    """🪟 (X0, X1, Y) of the louvre slots of the access panel as seen from -Z."""
    out = []
    r = K.HOUSING_R + 0.002
    for y0 in (0.135, 0.152, 0.598, 0.615):
        for k in (-1, 0, 1):
            d0 = k * 0.27 - 0.1
            out.append((sorted([-r * math.sin(d0), -r * math.sin(d0 + 0.2)]), y0 + 0.004))
    return out


def view_shapes(scale, view):
    """🖼️ The shapes of one view (elevation or front) of one scale."""
    out = []
    reg = {"region": "base-housing", "material": "powder-steel"}
    out.append(poly("base", "base", "body", 3, True, 0.6, side(view, "base", mirror(base_half(scale))), **reg))
    if view == "elevation" and scale <= 100:
        cx = K.WHEEL_X
        wheel = K.circle_pts(cx, K.WHEEL_Y, K.WHEEL_R, 20 if scale == 50 else 14)
        out.append(poly("wheel", "base", "body", 3.1, True, 0.9, side(view, "base", wheel), region="base-housing", material="plastic-black"))
        if scale == 50:
            out.append(poly("wheel-hub", "base", "detail", 3.15, False, 0.4, side(view, "base", K.circle_pts(cx, K.WHEEL_Y, 0.0295, 14)), region="base-housing"))
    if scale <= 200:
        out.append(poly("base-lip", "base", "detail", 3.4, False, 0.3, side(view, "base", [[-K.HOUSING_R, K.DISH_TOP], [K.HOUSING_R, K.DISH_TOP]]), closed=False, region="base-housing"))
        out.append(poly("lid-seam", "base", "detail", 3.4, False, 0.3, side(view, "base", [[-K.HOUSING_R, K.WALL_TOP], [K.HOUSING_R, K.WALL_TOP]]), closed=False, region="base-housing"))
    if view == "elevation" and scale <= 200:
        dx = (K.HOUSING_R + K.DOOR_PROUD) * math.sin(K.DOOR_SWEEP / 2)
        out.append(poly("door-seam", "base", "detail", 3.4, False, 0.3, side(view, "base", [[-dx, K.DOOR_Y0], [dx, K.DOOR_Y0], [dx, K.DOOR_Y1], [-dx, K.DOOR_Y1]]), region="base-housing"))
        if scale == 50:
            for i, ((x0, x1), y) in enumerate(louvres()):
                out.append(poly("vent-slot-%d" % i, "base", "texture", 3.45, False, 0.3, side(view, "base", [[x0, y], [x1, y]]), closed=False, region="base-housing"))
            out.append(poly("door-latch", "base", "detail", 3.45, True, 0.3, side(view, "base", K.circle_pts(0.11, 0.38, 0.011, 10)), region="base-housing"))
            out.append(poly("door-latch-edge", "base", "detail", 3.5, False, 0.3, side(view, "base", K.circle_pts(0.11, 0.38, 0.011, 10)), region="base-housing"))
    pr = K.POLE_R if scale < 500 else 0.0375
    out.append(poly("pole", "pole", "body", 2.95, True, 0.45, side(view, "pole", [[-pr, 0.75], [pr, 0.75], [pr, K.HEAD_Y + 0.01], [-pr, K.HEAD_Y + 0.01]]), region="pole-stainless", material="stainless-brushed"))
    if scale == 50:
        for i, y in enumerate((K.POLE_Y + 0.30, K.POLE_Y + 0.61)):
            out.append(poly("pole-joint-%d" % i, "pole", "detail", 3.3, True, 0.35, side(view, "pole", [[-0.0325, y], [0.0325, y], [0.0325, y + 0.03], [-0.0325, y + 0.03]]), region="pole-stainless"))
        out.append(poly("pole-highlight", "pole", "texture", 3.25, False, 0.3, side(view, "pole", [[-0.012, 0.80], [-0.012, 1.62]]), closed=False, region="pole-stainless"))
    elif scale == 100:
        for i, y in enumerate((K.POLE_Y + 0.315, K.POLE_Y + 0.625)):
            out.append(poly("pole-joint-%d" % i, "pole", "detail", 3.3, False, 0.3, side(view, "pole", [[-pr, y], [pr, y]]), closed=False, region="pole-stainless"))
    if scale <= 100:
        vh = mirror(head_half(scale))
        if view == "elevation":
            y = K.KNOB_Y
            right = head_half(scale)
            right = [p for p in right if p[1] < y - 0.026] + [[0.085, y - 0.026], [0.086, y - 0.026], [0.089, y - 0.023], [0.089, y + 0.023], [0.086, y + 0.026], [0.085, y + 0.026]] + [p for p in right if p[1] > y + 0.026]
            vh = mirror(right)
        out.append(poly("valve-housing", "burner-head", "body", 3.0, True, 0.45, side(view, "burner-head", vh), region="pole-stainless", material="stainless-brushed"))
        em = dict(region="emitter-head", material="stainless-hot")
        if scale == 50:
            em["hatch"] = "mesh"
        out.append(poly("head", "burner-head", "body", 2.9, True, 0.8, side(view, "burner-head", mirror(emitter_half(scale))), **em))
        for i, x in enumerate(rod_positions()):
            out.append(poly("guard-rod-%d" % i, "burner-head", "detail", 3.3, False, 0.3, side(view, "burner-head", [[x, K.HEAD_Y + K.DECK_U + 0.004], [x, K.HEAD_Y + K.SCREEN_TOP_U]]), closed=False, region="emitter-head"))
    else:
        if scale == 200:
            prof = [[0.036, K.HEAD_Y], [0.085, K.HEAD_Y + 0.040], [0.085, K.HEAD_Y + 0.118], [K.DECK_R, K.HEAD_Y + 0.136], [K.SCREEN_R, K.HEAD_Y + K.DECK_U], [K.SCREEN_R, K.HEAD_Y + K.CAP_TOP_U - 0.015], [K.CAP_R, K.HEAD_Y + K.CAP_TOP_U - 0.015], [K.CAP_R, K.HEAD_Y + K.CAP_TOP_U]]
        else:
            prof = [[0.0375, K.HEAD_Y], [0.085, K.HEAD_Y + 0.040], [0.085, K.HEAD_Y + 0.12], [K.SCREEN_R, K.HEAD_Y + 0.14], [K.SCREEN_R, K.HEAD_Y + K.CAP_TOP_U], [0.10, K.HEAD_Y + K.CAP_TOP_U], [0.10, K.HEAD_Y + K.CAP_TOP_U + 0.15]]
        out.append(poly("head", "burner-head", "body", 2.9, True, 0.8, side(view, "burner-head", mirror(prof)), region="emitter-head", material="stainless-hot"))
    half = K.KNOB_HALF[scale]
    if view == "elevation":
        kn = [[u, 0.0] for u, r in half[:1]] + [[u, r] for u, r in half]
        out.append(poly("knob", "knob", "body", 3.5, True, 0.2, kn, region="control", material="plastic-black", solid={"kind": "revolved", "axisBone": "knob", "axis": "u", "axisX": 0, "mirror": True}))
        if scale == 50:
            out.append(poly("knob-pointer", "knob", "detail", 3.6, True, 0.3, [[0.020, -0.021], [0.034, -0.021], [0.034, 0.021], [0.020, 0.021]], region="control",
                            solid={"kind": "extruded", "sweep": "w", "z0": -0.00425, "z1": 0.00425}))
    elif scale <= 100:
        out.append(shape("knob-ring", "burner-head", "detail", 3.45, True, 0.35, "ellipse", region="control", cx=0.0, cy=K.KNOB_Y, rx=0.026, ry=0.026))
        if scale == 50:
            out.append(shape("igniter", "burner-head", "detail", 3.45, True, 0.2, "ellipse", region="control", cx=FRONT_SIGN * 0.06, cy=K.KNOB_Y, rx=0.0105, ry=0.012))
    dome = mirror(dome_half(scale))
    out.append(poly("reflector", "reflector", "body", 3.2, True, 0.5, side(view, "reflector", dome), region="reflector", material="alu-reflector"))
    if scale <= 200:
        out.append(poly("top-cap", "reflector", "detail", 3.5, True, 0.25, side(view, "reflector", mirror(finial_half()) + [[0.0, K.TOP_Y]][:0]), region="reflector"))
    if scale == 50:
        y = K.REFLECTOR_Y + K.RIM_U + 0.001
        out.append(poly("dome-bead", "reflector", "detail", 3.4, False, 0.5, side(view, "reflector", [[-K.DOME_R, y], [K.DOME_R, y]]), closed=False, region="reflector"))
    if scale == 50:
        y = K.REFLECTOR_Y + K.dome_outer(K.RIB_R) + 0.001
        out.append(poly("dome-rib", "reflector", "texture", 3.4, False, 0.3, side(view, "reflector", [[-K.RIB_R, y], [K.RIB_R, y]]), closed=False, region="reflector"))
    if scale == 200:
        out.append(poly("stud-ring", "reflector", "body", 3.1, True, 0.4, side(view, "reflector", [[-0.101, K.REFLECTOR_Y], [0.101, K.REFLECTOR_Y], [0.101, K.REFLECTOR_Y + K.RIM_U], [-0.101, K.REFLECTOR_Y + K.RIM_U]]), region="reflector", material="stainless-hot"))
    if scale <= 100:
        seen = []
        for X, Z in spacer_xz():
            lat = X if view == "elevation" else Z
            if any(abs(lat - s) < 1e-6 for s in seen):
                continue
            seen.append(lat)
            i = len(seen) - 1
            out.append(poly("spacer-%d" % i, "reflector", "detail", 3.1, False, 0.4, side(view, "reflector", [[lat, K.REFLECTOR_Y], [lat, K.REFLECTOR_Y + K.RIM_U - 0.009]]), closed=False, region="reflector"))
            if scale == 50:
                top = K.REFLECTOR_Y + K.dome_outer(K.SPACER_R)
                out.append(poly("dome-nut-%d" % i, "reflector", "detail", 3.45, True, 0.3, side(view, "reflector", [[lat - 0.011, top - 0.004], [lat + 0.011, top - 0.004], [lat + 0.011, top + 0.003], [lat + 0.008, top + 0.004], [lat + 0.008, top + 0.012], [lat - 0.008, top + 0.012], [lat - 0.008, top + 0.004], [lat - 0.011, top + 0.003]]), region="reflector"))
    return out


def plan_shapes(scale):
    """🗺️ The plan: the dome hides everything under it."""
    out = [shape("reflector", "reflector", "body", 3.2, True, 0.5, "ellipse", region="reflector", material="alu-reflector", cx=0.0, cy=0.0, rx=0.4, ry=0.4)]
    if scale == 50:
        out.append(shape("dome-bead", "reflector", "detail", 3.55, False, 0.5, "ellipse", region="reflector", cx=0.0, cy=0.0, rx=K.DOME_R - 0.006, ry=K.DOME_R - 0.006))
    if scale == 50:
        out.append(shape("dome-rib", "reflector", "texture", 3.55, False, 0.3, "ellipse", region="reflector", cx=0.0, cy=0.0, rx=K.RIB_R, ry=K.RIB_R))
    if scale <= 200:
        out.append(shape("top-cap", "reflector", "detail", 3.6, True, 0.25, "ellipse", region="reflector", cx=0.0, cy=0.0, rx=0.05, ry=0.05))
        out.append(shape("top-cap-edge", "reflector", "detail", 3.65, False, 0.3, "ellipse", region="reflector", cx=0.0, cy=0.0, rx=0.05, ry=0.05))
    if scale == 50:
        for i, (X, Z) in enumerate(spacer_xz()):
            out.append(shape("dome-nut-%d" % i, "reflector", "detail", 3.6, True, 0.3, "ellipse", region="reflector", cx=K.r6(X), cy=K.r6(PLAN_SIGN * Z), rx=0.011, ry=0.011))
    return out


def geometry(scale):
    """📚 Elevation, front and plan of one scale."""
    return {"elevation": view_shapes(scale, "elevation"), "front": view_shapes(scale, "front"), "plan": plan_shapes(scale)}


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    50: {
        "silhouette": text("A mushroom heater 16 mm wide and 45 mm high in construction detail: base pan with rounded edges and the rear wheel with its hub, the cylindrical bottle housing with the access panel, its louvre rows and latch, the pressed lid with the post socket, the three-part post with its two couplings, the head with the valve housing, knob boss and knob with grip bar, the emitter screen with bands, eight guard rods and cap, the three spacer studs, and the dome with rolled rim, dome nuts and finial. Front: the same turned silhouette seen from the knob side with the knob face, its grip bar and the igniter button. Plan: the dome with its rim, the pressed stiffening rib, the three dome nuts and the finial.",
                           "Ein Heizpilz 16 mm breit und 45 mm hoch im Konstruktionsdetail: Bodenwanne mit gerundeten Kanten und hinterem Rad mit Nabe, das zylindrische Flaschengehäuse mit Zugangsklappe, Lüftungsschlitzreihen und Riegel, der gepresste Deckel mit Mastmuffe, der dreiteilige Mast mit zwei Kupplungen, der Kopf mit Ventilgehäuse, Knopfrosette und Knopf mit Griffsteg, das Strahlergitter mit Bändern, acht Schutzstäben und Deckel, die drei Abstandsbolzen und die Haube mit Rollrand, Haubenmuttern und Knauf. Vorderansicht: dieselbe gedrehte Silhouette von der Knopfseite mit Knopffläche, Griffsteg und Zündknopf. Draufsicht: die Haube mit Rand, der gepressten Versteifungssicke, den drei Haubenmuttern und dem Knauf."),
        "retained": text("Every visible assembly at real size: 47 cm pan, 42 cm housing 73 cm high, 60 mm post, 24 cm emitter screen, 80 cm dome; the knob grip turns with the knob bone.", "Jede sichtbare Baugruppe in realer Größe: Wanne 47 cm, Gehäuse 42 cm und 73 cm hoch, Mast 60 mm, Strahlergitter 24 cm, Haube 80 cm; der Knopfgriff dreht mit dem Knopfknochen."),
        "merged": text("The far wheel lies behind the near one; guard rods and spacers of the far side coincide with the near ones.", "Das ferne Rad liegt hinter dem nahen; Schutzstäbe und Abstandshalter der fernen Seite fallen mit den nahen zusammen."),
        "omitted": text("The bottle, hose and burner inside the housing and the head are hidden (thermal model only); the axle brackets hide behind the wheel; the access panel is not seen from the knob side; everything under the dome is hidden in plan.", "Flasche, Schlauch und Brenner in Gehäuse und Kopf sind verdeckt (nur im Thermalmodell); die Achshalter liegen hinter dem Rad; die Zugangsklappe ist von der Knopfseite nicht sichtbar; alles unter der Haube ist in der Draufsicht verdeckt."),
        "thinElements": text("Guard rods (8 mm) and spacers (12 mm) are 0.18 mm strokes; the couplings and the dome nuts are small filled shapes; louvres and the dome rib are texture strokes.", "Schutzstäbe (8 mm) und Abstandshalter (12 mm) sind 0,18-mm-Striche; Kupplungen und Haubenmuttern sind kleine gefüllte Formen; Lüftungsschlitze und Haubensicke sind Texturstriche."),
        "hatching": text("Fine crosshatch (0.5 mm) on the emitter screen as the perforated mesh.", "Feine Kreuzschraffur (0,5 mm) auf dem Strahlergitter als Lochblech."),
        "styles": text("Housing 0.6, post and valve housing 0.45, hot screen 0.8, dome 0.5, knob 0.2; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.", "Gehäuse 0,6, Mast und Ventilgehäuse 0,45, heißes Gitter 0,8, Haube 0,5, Knopf 0,2; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm."),
        "overlays": text("The screen, its rods and cap carry emitter-head, dome, spacers and nuts reflector, the valve housing pole-stainless, knob and igniter control: the thermal view paints the hot head under the dome.", "Gitter, Stäbe und Deckel tragen emitter-head, Haube, Abstandshalter und Muttern reflector, das Ventilgehäuse pole-stainless, Knopf und Zünder control: die Thermalansicht färbt den heißen Kopf unter der Haube."),
    },
    100: {
        "silhouette": text("A mushroom heater 8 mm wide and 22.5 mm high: pan, housing with lid seam and access panel, rear wheel, post with two coupling lines, valve housing with knob, emitter screen with guard rods and cap, spacers and the dome with finial. Front: the turned silhouette with the knob face on its boss. Plan: dome disc with finial.",
                           "Ein Heizpilz 8 mm breit und 22,5 mm hoch: Wanne, Gehäuse mit Deckelfuge und Zugangsklappe, hinteres Rad, Mast mit zwei Kupplungslinien, Ventilgehäuse mit Knopf, Strahlergitter mit Schutzstäben und Deckel, Abstandshalter und die Haube mit Knauf. Vorderansicht: die gedrehte Silhouette mit Knopffläche auf der Rosette. Draufsicht: Haubenscheibe mit Knauf."),
        "retained": text("Pan, housing, post, head, knob and dome at real size; the wheel stays as it shows the rear of the heater.", "Wanne, Gehäuse, Mast, Kopf, Knopf und Haube in realer Größe; das Rad bleibt, weil es die Rückseite zeigt."),
        "merged": text("Rim bead into the dome outline, screen bands and cap into one emitter outline, knob grip into the knob.", "Rollrand in den Haubenumriss, Gitterbänder und Deckel in einen Strahlerumriss, Knopfgriff in den Knopf."),
        "omitted": text("Louvres, latch, hub, igniter, dome nuts, the dome rib and the post highlight (all under 15 mm).", "Lüftungsschlitze, Riegel, Nabe, Zünder, Haubenmuttern, Haubensicke und Lichtkante des Masts (alle unter 15 mm)."),
        "thinElements": text("Guard rods and spacers are 0.13 mm detail strokes; coupling lines across the post.", "Schutzstäbe und Abstandshalter sind 0,13-mm-Detailstriche; Kupplungslinien quer über den Mast."),
        "hatching": text("No hatching; the hot screen is shown by its tone.", "Keine Schraffur; das heiße Gitter zeigt sich durch seinen Ton."),
        "styles": text("Tones as at 1:50; body pen 0.25 mm, detail 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": text("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": text("A mushroom heater 4 mm wide and 11 mm high: stepped pan and housing with lid seam and access panel, a 0.3 mm post, the head as one outline from valve housing to cap, the knob, and the dome with finial. Front: the same with the round knob. Plan: dome disc with finial.",
                           "Ein Heizpilz 4 mm breit und 11 mm hoch: abgesetzte Wanne und Gehäuse mit Deckelfuge und Zugangsklappe, ein 0,3 mm dicker Mast, der Kopf als ein Umriss vom Ventilgehäuse bis zum Deckel, der Knopf und die Haube mit Knauf. Vorderansicht: dasselbe mit rundem Knopf. Draufsicht: Haubenscheibe mit Knauf."),
        "retained": text("Pan step, housing, true 60 mm post, head, 48 mm knob and dome with finial.", "Wannenstufe, Gehäuse, wahrer 60-mm-Mast, Kopf, 48-mm-Knopf und Haube mit Knauf."),
        "merged": text("Valve housing, emitter and cap into one head; wheels behind the pan (under 30 mm wide).", "Ventilgehäuse, Strahler und Deckel zu einem Kopf; Räder hinter der Wanne (unter 30 mm breit)."),
        "omitted": text("Guard rods, spacers, couplings and louvres.", "Schutzstäbe, Abstandshalter, Kupplungen und Lüftungsschlitze."),
        "thinElements": text("Door seam, lid seam and pan seam as 0.13 mm detail lines.", "Türfuge, Deckelfuge und Wannenfuge als 0,13-mm-Detaillinien."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tones as at 1:50; body pen 0.18 mm.", "Töne wie bei 1:50; Körperstift 0,18 mm."),
        "overlays": text("The head carries emitter-head as one region.", "Der Kopf trägt emitter-head als eine Region."),
    },
    500: {
        "silhouette": text("A mushroom sign 1.6 mm wide and 4.5 mm high: the housing as one block with its sloped lid, a 75 mm post, the head and the dome. Front: the same turned outline. Plan: the 0.80 m dome disc.",
                           "Ein Pilzzeichen 1,6 mm breit und 4,5 mm hoch: das Gehäuse als ein Block mit geneigtem Deckel, ein 75-mm-Mast, der Kopf und die Haube. Vorderansicht: derselbe gedrehte Umriss. Draufsicht: die Haubenscheibe von 0,80 m."),
        "retained": text("Height, dome diameter, housing width and the head under the dome.", "Höhe, Haubendurchmesser, Gehäusebreite und der Kopf unter der Haube."),
        "merged": text("Pan and housing into one block, the post drawn at the 75 mm feature (true 60 mm), head as one outline.", "Wanne und Gehäuse zu einem Block, der Mast im 75-mm-Merkmal gezeichnet (wahr 60 mm), Kopf als ein Umriss."),
        "omitted": text("Finial, seams, wheels and every detail line; the knob stays at the 75 mm size floor (true 47 mm).", "Knauf, Fugen, Räder und jede Detaillinie; der Knopf bleibt an der Größenuntergrenze von 75 mm (wahr 47 mm)."),
        "thinElements": text("None.", "Keine."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tones as at 1:50; body pen 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,13 mm."),
        "overlays": text("Housing base-housing, post pole-stainless, head emitter-head, dome reflector.", "Gehäuse base-housing, Mast pole-stainless, Kopf emitter-head, Haube reflector."),
    },
}


def apply(asset):
    """🖌️ Writes geometry and decisions of every scale."""
    for s in (50, 100, 200, 500):
        rep = asset["representations"][str(s)]
        rep["geometry"] = geometry(s)
        rep["decisions"] = DECISIONS[s]
