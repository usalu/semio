"""✏️ Round 3 cast drinking fountain: the 2D drawings of every scale and view, drawn from the kit's geometry (ruling E1), and the representation decisions.

Elevation shapes are in the bone-local frame (upright bones: x up the bone, y = -X); front shapes use x = world Z and y = world height; plan shapes x = world X (ahead) and y = world Z.

@see r3-drinking-fountain-kit.py — the geometry
"""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-drinking-fountain-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

CAST = {"region": "pedestal-cast", "material": "cast-iron"}
BOWL = {"region": "basin-steel", "material": "stainless-basin"}
BRASS = {"region": "bubbler-brass", "material": "chrome-brass"}
WET = {"region": "water-jet"}
LIP_Y = K.cradle_y(K.CUP_R)
TOP = K.RIM_Y + K.BEAD_R


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def shape(sid, bone, role, z, filled, tone, kind, **geo):
    """🖍️ One 2D shape with its common fields."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    out.update(geo)
    return out


def poly(sid, bone, role, z, filled, tone, pts, closed=True, **kw):
    """📈 A polyline or polygon shape."""
    return shape(sid, bone, role, z, filled, tone, "poly", points=[[K.r6(x), K.r6(y)] for x, y in pts], closed=closed, **kw)


def side(view, bone, pts):
    """🔁 World side-profile points (X or Z, Y) into the frame of the view."""
    if view == "elevation":
        return [K.local2(bone, x, y) for x, y in pts]
    return [[x, y] for x, y in pts]


def mirror(half):
    """🪞 A symmetric outline from its right half (r, Y) listed bottom to top."""
    return half + [[-r, y] for r, y in reversed(half) if r != 0]


def flip(profile):
    """↔️ Kit profile points (Y, r) as (r, Y)."""
    return [[r, y] for y, r in profile]


def turned(sid, view, z, half, tone=0.6, bone="pedestal", role="body", **kw):
    """🏺 A turned part as the filled mirror of its right half."""
    return poly(sid, bone, role, z, True, tone, side(view, bone, mirror(half)), **kw)


def ribbon(path, radii):
    """🎗️ Outline of a round tube seen from the side: the centre line offset by the radius to both sides."""
    left, right = [], []
    for i, (x, y) in enumerate(path):
        x0, y0 = path[max(0, i - 1)]
        x1, y1 = path[min(len(path) - 1, i + 1)]
        L = math.hypot(x1 - x0, y1 - y0)
        nx, ny = -(y1 - y0) / L, (x1 - x0) / L
        left.append([x + radii[i] * nx, y + radii[i] * ny])
        right.append([x - radii[i] * nx, y - radii[i] * ny])
    return left + right[::-1]


def neck(scale):
    """🦢 Centre line and radii of the goose-neck as the scale models it, from the bowl floor to the mouth."""
    dx, dy = K.out_dir()
    if scale == 50:
        path = [(K.BX, 0.8)] + K.neck_path(1, 10, K.BEND_Y)
        return path, [K.NECK_R] * len(path)
    if scale == 100:
        path = [(K.BX, 0.8)] + K.neck_path(1, 6, K.BEND_Y)
        path[-1] = (path[-1][0] + K.TIP_L * dx, path[-1][1] + K.TIP_L * dy)
        return path, [K.NECK_R] * (len(path) - 1) + [0.015]
    if scale == 200:
        path = [(K.BX, 0.8), (K.BX, K.BEND_Y)] + [K.bend_point(a) for a in (150, 115, 80, 50)]
        path.append((path[-1][0] + 0.032 * dx, path[-1][1] + 0.032 * dy))
        return path, [K.NECK_R] * len(path)
    return [(K.BX, 0.816), (K.BX, 0.925), (-0.083, 0.966), (-0.048, 0.978), (K.outlet()[0] - 0.004, K.outlet()[1] + 0.004)], [0.027, 0.024, 0.02, 0.018, 0.016]


def tip_quad():
    """🔧 The nozzle tip ring seen from the side."""
    x, y = K.neck_path()[-1]
    dx, dy = K.out_dir()
    nx, ny = -dy, dx
    a0, a1, r = -0.004, K.TIP_L, 0.0155
    return [[x + a0 * dx + r * nx, y + a0 * dy + r * ny], [x + a1 * dx + r * nx, y + a1 * dy + r * ny], [x + a1 * dx - r * nx, y + a1 * dy - r * ny], [x + a0 * dx - r * nx, y + a0 * dy - r * ny]]


def pedestal_halves(scale):
    """🏛️ Right halves (r, Y) of the drawn pedestal parts of a scale, bottom to top, as (id, half)."""
    D = K.DIE_R
    if scale == 50:
        return [
            ("pedestal", flip(K.base_outer(4))),
            ("shaft-base", [[0.112, 0.36]] + flip(K.shaft_base_loop(3)[4:-4])),
            ("shaft", [[K.SHAFT_R0, K.SHAFT_Y0], [K.SHAFT_R1, K.SHAFT_Y1]]),
            ("astragal", flip(K.astragal_loop(3)[1:-2])),
            ("capital", flip(K.capital_outer(6))),
        ]
    if scale == 100:
        return [
            ("pedestal", [[0.2, 0.0], [0.2, 0.022], [0.166, 0.022], [0.166, 0.042], [0.148, 0.046], [0.13, 0.062], [D, 0.078], [D, 0.322], [0.126, 0.33], [0.138, 0.345], [0.138, 0.36]]),
            ("shaft", [[0.114, 0.36], [0.114, 0.384], [0.089, 0.396], [0.077, 0.69], [0.088, 0.69], [0.088, 0.705]]),
            ("capital", [[0.082, 0.705], [0.082, 0.767], [0.086, 0.776], [0.097, 0.788], [0.111, 0.799], [0.122, 0.806], [0.122, LIP_Y]]),
        ]
    if scale == 200:
        return [("pedestal", [[0.2, 0.0], [0.2, 0.03], [0.16, 0.03], [D, 0.075], [D, 0.33], [0.138, 0.33], [0.138, 0.36], [0.105, 0.36], [0.09, 0.395], [0.08, 0.705], [0.082, 0.767], [0.122, 0.806], [0.122, LIP_Y]])]
    return [("pedestal", [[0.2, 0.0], [D, 0.075], [D, 0.36], [0.09, 0.36], [0.08, 0.72], [0.122, 0.8], [0.122, LIP_Y]])]


def bowl_half(scale):
    """🥣 Right half (r, Y) of the bowl silhouette with its rim."""
    n = {50: 12, 100: 9, 200: 7, 500: 4}[scale]
    curve = flip(K.bowl_curve(0.0, K.BOWL_A, n))
    if scale == 50:
        return curve[:-1] + flip(K.arc(K.RIM_Y, K.BEAD_C, K.BEAD_R, -135, 180, 9))
    if scale == 100:
        return curve[:-1] + [[0.1895, 0.887], [0.2, 0.887], [0.2, 0.897], [0.198, TOP], [0.1885, TOP]]
    return curve + [[0.2, TOP], [0.1885, TOP]]


def jet_points(scale):
    """💧 World (X, Y) of the drawn jet centre line."""
    return [(x, y) for x, y, _ in K.jet_path({50: 8, 100: 4, 200: 3, 500: 2}[scale])]


def upright(scale, view):
    """🖼️ The shapes of the side elevation or the front elevation of one scale."""
    out = []
    jet_pen = {50: 0.2, 100: 0.13, 200: 0.13, 500: 0.13}[scale]
    jp = jet_points(scale)
    if view == "elevation":
        out.append(poly("jet", "jet", "detail", 2.6, False, 0.5, [K.local2("jet", x, y) for x, y in jp], closed=False, penMm=jet_pen, **WET))
        if scale == 50:
            x1, y1 = jp[-1]
            for i, (ox, oy, rx) in enumerate(((0.016, 0.017, 0.005), (-0.013, 0.022, 0.004))):
                c = K.local2("jet", x1 + ox, y1 + oy)
                out.append(shape("drop-%d" % i, "jet", "detail", 2.7, True, 0.3, "ellipse", cx=c[0], cy=c[1], rx=rx, ry=0.004, **WET))
    else:
        out.append(poly("jet", "jet", "detail", 2.95, False, 0.5, [[0.0, jp[0][1]], [0.0, jp[-1][1]]], closed=False, penMm=jet_pen, **WET))
    path, radii = neck(scale)
    if view == "elevation":
        out.append(poly("bubbler", "bubbler", "body", 2.8, True, 0.7, side(view, "bubbler", ribbon(path, radii)), **BRASS))
        if scale == 50:
            out.append(poly("nozzle", "bubbler", "detail", 2.85, True, 0.3, side(view, "bubbler", tip_quad()), **BRASS))
    else:
        w = max(radii) if scale == 500 else K.NECK_R
        top = max(y + r for (_, y), r in zip(path, radii))
        out.append(shape("bubbler", "bubbler", "body", 2.8, True, 0.7, "rect", x=K.r6(-w), y=K.r6(path[0][1]), w=K.r6(2 * w), h=K.r6(top - path[0][1]), **BRASS))
        if scale == 50:
            x, y = K.neck_path()[-1]
            dy = K.out_dir()[1]
            out.append(shape("nozzle", "bubbler", "detail", 2.85, True, 0.3, "ellipse", cx=0.0, cy=K.r6(y + 0.002 * dy), rx=0.0155, ry=0.0155, **BRASS))
        if scale == 50:
            out.append(shape("nozzle-bore", "bubbler", "detail", 2.9, True, 0.15, "ellipse", cx=0.0, cy=K.r6(K.outlet()[1]), rx=K.JET_R0, ry=0.0046, **BRASS))
    for i, (sid, half) in enumerate(pedestal_halves(scale)):
        out.append(turned(sid, view, 3.0 + 0.01 * i if sid != "capital" else 3.2, half, **CAST))
    if scale == 50:
        for k, c in enumerate((0.92388, 0.707107, 0.382683, 0.0, -0.382683, -0.707107, -0.92388)):
            line = [[K.SHAFT_R0 * c, K.SHAFT_Y0], [K.SHAFT_R1 * c, K.SHAFT_Y1]]
            out.append(poly("arris-%d" % k, "pedestal", "texture", 3.5, False, 0.3, side(view, "pedestal", line), closed=False, region="pedestal-cast"))
        for k, sx in enumerate((-1, 1)):
            x = sx * K.BOLT_RC * math.cos(math.pi / 4)
            nut = [[x - 0.011, 0.022], [x + 0.011, 0.022], [x + 0.011, 0.033], [x + 0.006, 0.033], [x + 0.006, 0.04], [x - 0.006, 0.04], [x - 0.006, 0.033], [x - 0.011, 0.033]]
            out.append(poly("bolt-%d" % k, "pedestal", "detail", 3.6, True, 0.3, side(view, "pedestal", nut), **CAST))
    if view == "elevation" and scale <= 200:
        hx = (K.DIE_R + K.DOOR_PROUD) * math.sin(math.radians(K.DOOR_HALF))
        door = [[-hx, K.DOOR_Y0], [hx, K.DOOR_Y0], [hx, K.DOOR_Y1], [-hx, K.DOOR_Y1]]
        out.append(poly("door", "pedestal", "detail", 3.5, False, 0.3, side(view, "pedestal", door), region="pedestal-cast"))
        if scale == 50:
            sx = (K.DIE_R + K.DOOR_PROUD) * math.sin(math.radians(22))
            for k, (x, y) in enumerate(((-sx, 0.13), (sx, 0.13), (-sx, 0.27), (sx, 0.27))):
                c = K.local2("pedestal", x, y)
                out.append(shape("door-screw-%d" % k, "pedestal", "texture", 3.6, True, 0.3, "ellipse", cx=c[0], cy=c[1], rx=0.005, ry=0.005, region="pedestal-cast"))
    out.append(turned("basin", view, 3.1, bowl_half(scale), tone=0.5, bone="basin", **BOWL))
    if scale <= 100:
        seam = [[-0.1885, K.RIM_Y - 0.006], [0.1885, K.RIM_Y - 0.006]]
        out.append(poly("rim", "basin", "detail", 3.5, False, 0.3, side(view, "basin", seam), closed=False, region="basin-steel"))
    out += button(scale, view)
    return out


def button(scale, view):
    """🔘 Boss, bezel and push button of one view."""
    out = []
    y = K.BTN_Y
    if view == "elevation":
        if scale == 50:
            boss = [[0.079, y - 0.03], [0.094, y - 0.03], [0.096, y - 0.028], [0.096, y + 0.028], [0.094, y + 0.03], [0.079, y + 0.03]]
            out.append(poly("button-boss", "pedestal", "body", 3.3, True, 0.6, side(view, "pedestal", boss), **CAST))
            bezel = [[0.095, y - 0.027], [0.101, y - 0.027], [0.104, y - 0.023], [0.104, y + 0.023], [0.101, y + 0.027], [0.095, y + 0.027]]
            out.append(poly("bezel", "pedestal", "detail", 3.35, True, 0.75, side(view, "pedestal", bezel), **BRASS))
            cap = [[-0.002, -K.BTN_R], [0.018, -K.BTN_R], [0.0205, -0.017], [0.022, -0.011], [0.0225, 0.0], [0.022, 0.011], [0.0205, 0.017], [0.018, K.BTN_R], [-0.002, K.BTN_R]]
            out.append(poly("button", "button", "body", 3.25, True, 0.8, cap, **BRASS))
        elif scale == 100:
            bezel = [[0.079, y - 0.03], [0.096, y - 0.03], [0.104, y - 0.024], [0.104, y + 0.024], [0.096, y + 0.03], [0.079, y + 0.03]]
            out.append(poly("bezel", "pedestal", "body", 3.35, True, 0.7, side(view, "pedestal", bezel), **BRASS))
            cap = [[0.0, -K.BTN_R], [0.019, -K.BTN_R], [0.0225, -0.012], [0.0225, 0.012], [0.019, K.BTN_R], [0.0, K.BTN_R]]
            out.append(poly("button", "button", "body", 3.25, True, 0.8, cap, **BRASS))
        elif scale == 200:
            cap = [[-0.017, -0.027], [0.0, -0.027], [0.0, -K.BTN_R], [0.0225, -K.BTN_R], [0.0225, K.BTN_R], [0.0, K.BTN_R], [0.0, 0.027], [-0.017, 0.027]]
            out.append(poly("button", "button", "body", 3.4, True, 0.8, cap, **BRASS))
        else:
            out.append(shape("button", "button", "body", 3.4, True, 0.8, "rect", x=-0.017, y=-0.0225, w=0.0395, h=0.045, **BRASS))
        return out
    if scale == 50:
        out.append(shape("button-boss", "pedestal", "body", 3.3, True, 0.6, "ellipse", cx=0.0, cy=y, rx=0.03, ry=0.03, **CAST))
        out.append(shape("bezel", "pedestal", "detail", 3.35, True, 0.75, "ellipse", cx=0.0, cy=y, rx=0.027, ry=0.027, **BRASS))
        out.append(shape("button", "button", "body", 3.4, True, 0.8, "ellipse", cx=0.0, cy=y, rx=K.BTN_R, ry=K.BTN_R, **BRASS))
    elif scale == 100:
        out.append(shape("bezel", "pedestal", "body", 3.35, True, 0.7, "ellipse", cx=0.0, cy=y, rx=0.03, ry=0.03, **BRASS))
        out.append(shape("button", "button", "body", 3.4, True, 0.8, "ellipse", cx=0.0, cy=y, rx=K.BTN_R, ry=K.BTN_R, **BRASS))
    else:
        r = 0.027 if scale == 200 else 0.0225
        out.append(shape("button", "button", "body", 3.4, True, 0.8, "ellipse", cx=0.0, cy=y, rx=r, ry=r, **BRASS))
    return out


def plan(scale):
    """🧭 The plan: the bowl from above with the goose-neck over it."""
    out = [shape("basin", "basin", "body", 3.2, True, 0.5, "ellipse", cx=0, cy=0, rx=0.2, ry=0.2, **BOWL)]
    if scale <= 100:
        out.append(shape("rim", "basin", "detail", 3.5, False, 0.3, "ellipse", cx=0, cy=0, rx=0.188, ry=0.188, region="basin-steel"))
    if scale == 50:
        out.append(shape("drain", "basin", "detail", 3.4, True, 0.35, "ellipse", cx=0, cy=0, rx=0.027, ry=0.027, **BOWL))
        out.append(shape("strainer", "basin", "texture", 3.45, False, 0.2, "ellipse", cx=0, cy=0, rx=0.021, ry=0.021, region="basin-steel"))
    if scale <= 100:
        out.append(shape("bubbler-base", "bubbler", "detail", 3.25, True, 0.7, "ellipse", cx=K.BX, cy=0, rx=0.017, ry=0.017, **BRASS))
    if scale == 50:
        hexagon = [[K.BX + 0.0162 * math.cos(math.radians(60 * k)), 0.0162 * math.sin(math.radians(60 * k))] for k in range(6)]
        out.append(poly("bubbler-nut", "bubbler", "detail", 3.27, True, 0.65, hexagon, **BRASS))
    path, radii = neck(scale)
    x_end = max(x for x, _ in path)
    r0, r1 = (radii[0], radii[-1]) if scale == 500 else (K.NECK_R, K.NECK_R)
    out.append(shape("bubbler", "bubbler", "body", 3.3, True, 0.7, "capsule", x0=K.BX, y0=0, x1=K.r6(x_end - r1 * 0.5), y1=0, r0=r0, r1=r1, **BRASS))
    if scale == 50:
        x0 = K.neck_path()[-1][0]
        tip = [[x0 - 0.003, -0.0155], [K.outlet()[0], -0.0155], [K.outlet()[0], 0.0155], [x0 - 0.003, 0.0155]]
        out.append(poly("nozzle", "bubbler", "detail", 3.35, True, 0.3, tip, **BRASS))
    return out


DECISIONS = {
    50: {
        "silhouette": text("A fountain 8 mm wide and 20 mm high in construction detail: flange with two anchor nuts, torus and scotia, the die with its access door and four screws, cornice, shaft base, the tapering shaft with the arrises of its sixteen flutes, astragal, neck band with the push button in its boss and bezel, the bell of the capital, the bowl with rolled rim, the goose-neck with its nozzle tip and the falling jet with two drops. Front: the same turned silhouette seen from the drinker's side with the button face in its bezel and boss, the neck as a column over the rim with the nozzle ring and bore, and the jet in front of it. Plan: the bowl with rim, waste and strainer, the base, union nut, neck and nozzle of the goose-neck.",
                           "Ein Brunnen 8 mm breit und 20 mm hoch im Konstruktionsdetail: Flansch mit zwei Ankermuttern, Wulst und Kehle, der Sockelkörper mit Revisionstür und vier Schrauben, Gesims, Schaftbasis, der verjüngte Schaft mit den Stegen seiner sechzehn Kannelüren, Halsring, Halsband mit Drucktaster in Nabe und Blende, die Glocke des Kapitells, das Becken mit Rollrand, der Schwanenhals mit Düsenspitze und der fallende Strahl mit zwei Tropfen. Vorderansicht: dieselbe gedrehte Silhouette von der Trinkseite mit der Tasterfläche in Blende und Nabe, dem Hals als Säule über dem Rand mit Düsenring und Bohrung und dem Strahl davor. Draufsicht: das Becken mit Rand, Ablauf und Sieb, Fuß, Überwurfmutter, Hals und Düse des Schwanenhalses."),
        "retained": text("All visible parts at real size; the button slides 10 mm into its boss with the button bone, the jet sinks down the axis with the jet bone.", "Alle sichtbaren Teile in realer Größe; der Taster fährt 10 mm mit dem Tasterknochen in die Nabe, der Strahl versinkt mit dem Strahlknochen entlang der Achse."),
        "merged": text("The two far anchor nuts lie behind the near ones; the flutes are drawn by their arrises.", "Die zwei fernen Ankermuttern liegen hinter den nahen; die Kannelüren sind durch ihre Stege gezeichnet."),
        "omitted": text("Maker plate and the strainer holes. The base and union nut of the neck stand inside the bowl and show only in plan; the pedestal lies under the bowl and the jet is not drawn in plan, because it would show in every pose.", "Typenschild und Sieblöcher. Fuß und Überwurfmutter des Halses stehen im Becken und zeigen sich nur in der Draufsicht; der Sockel liegt unter dem Becken und der Strahl wird in der Draufsicht nicht gezeichnet, weil er in jeder Pose erschiene."),
        "thinElements": text("The jet is a 0.2 mm line for the stream of 12 to 6 mm; drops are 8 to 10 mm ellipses; the arrises are 0.13 mm texture lines.", "Der Strahl ist eine 0,2-mm-Linie für den 12 bis 6 mm dicken Strahl; Tropfen sind Ellipsen von 8 bis 10 mm; die Stege sind 0,13-mm-Texturlinien."),
        "hatching": text("None; the flutes are drawn by seven arris lines in true projection.", "Keine; die Kannelüren sind durch sieben Steglinien in wahrer Projektion gezeichnet."),
        "styles": text("Castings 0.6, bowl 0.5, goose-neck 0.7, bezel 0.75, button 0.8; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.", "Gussteile 0,6, Becken 0,5, Schwanenhals 0,7, Blende 0,75, Taster 0,8; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm."),
        "overlays": text("Jet and drops carry water-jet: the thermal view paints the 12 degC water against the warm bowl.", "Strahl und Tropfen tragen water-jet: die Thermalansicht färbt das 12-grädige Wasser gegen das warme Becken."),
    },
    100: {
        "silhouette": text("A fountain 4 mm wide and 10 mm high: flange, stepped foot, die with the access door, cornice, shaft with base and astragal steps, neck with the button on its chrome boss, capital bell, bowl with lip, goose-neck and jet. Front: the turned silhouette with the round button, the neck column and the jet; plan: bowl with rim line and the goose-neck on its base.",
                           "Ein Brunnen 4 mm breit und 10 mm hoch: Flansch, gestufter Fuß, Sockelkörper mit Revisionstür, Gesims, Schaft mit Basis- und Halsringstufe, Hals mit Taster auf Chromnabe, Kapitellglocke, Becken mit Rand, Schwanenhals und Strahl. Vorderansicht: die gedrehte Silhouette mit rundem Taster, die Halssäule und der Strahl; Draufsicht: Becken mit Randlinie und der Schwanenhals auf seinem Fuß."),
        "retained": text("Flange, die, door, cornice, shaft, capital, bowl, neck, button and jet at real size.", "Flansch, Sockelkörper, Tür, Gesims, Schaft, Kapitell, Becken, Hals, Taster und Strahl in realer Größe."),
        "merged": text("Torus and scotia into two steps, boss and bezel into one chrome boss, bead into a folded lip, nozzle tip into the neck.", "Wulst und Kehle zu zwei Stufen, Nabe und Blende zu einer Chromnabe, Wulst zu einem Falzrand, Düsenspitze in den Hals."),
        "omitted": text("Flutes, anchor nuts, door screws, union nut, waste and drops (under 15 mm). The jet is not drawn in plan.", "Kannelüren, Ankermuttern, Türschrauben, Überwurfmutter, Ablauf und Tropfen (unter 15 mm). Der Strahl wird in der Draufsicht nicht gezeichnet."),
        "thinElements": text("The neck is its real 28 mm (0.28 mm); the jet is a 0.13 mm line for the 12 mm stream.", "Der Hals hat seine realen 28 mm (0,28 mm); der Strahl ist eine 0,13-mm-Linie für den 12 mm dicken Strahl."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tones as at 1:50; body pen 0.25 mm, detail 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": text("The jet carries water-jet.", "Der Strahl trägt water-jet."),
    },
    200: {
        "silhouette": text("A fountain 2 mm wide and 5 mm high: one turned pedestal with flange, splayed foot, die with the door outline, cornice step, tapering shaft and bell, the bowl, the hooked neck, the button and the jet. Front: the same with the round button and the neck column; plan: bowl disc with the neck.",
                           "Ein Brunnen 2 mm breit und 5 mm hoch: ein gedrehter Sockel mit Flansch, ausgestelltem Fuß, Sockelkörper mit Türumriss, Gesimsstufe, verjüngtem Schaft und Glocke, das Becken, der Hakenhals, der Taster und der Strahl. Vorderansicht: dasselbe mit rundem Taster und Halssäule; Draufsicht: Beckenscheibe mit dem Hals."),
        "retained": text("Flange, die, cornice step, shaft, bell, bowl, hooked neck, button, jet.", "Flansch, Sockelkörper, Gesimsstufe, Schaft, Glocke, Becken, Hakenhals, Taster, Strahl."),
        "merged": text("The three castings into one outline; boss, bezel and button into one stepped stud.", "Die drei Gussteile zu einem Umriss; Nabe, Blende und Taster zu einem gestuften Zapfen."),
        "omitted": text("Mouldings under 30 mm, rim detail, nozzle tip, neck base. Button and jet are not drawn in plan.", "Profile unter 30 mm, Randdetail, Düsenspitze, Halsfuß. Taster und Strahl werden in der Draufsicht nicht gezeichnet."),
        "thinElements": text("The neck is its real 28 mm (0.14 mm); the jet is a 0.13 mm hairline for the 12 mm stream.", "Der Hals hat seine realen 28 mm (0,14 mm); der Strahl ist eine 0,13-mm-Haarlinie für den 12 mm dicken Strahl."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tones as at 1:50; body pen 0.18 mm, detail 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,18 mm, Detail 0,13 mm."),
        "overlays": text("The jet carries water-jet.", "Der Strahl trägt water-jet."),
    },
    500: {
        "silhouette": text("A fountain sign 0.8 mm wide and 2 mm high: a splayed foot with the die, the shaft flaring into the capital, the bowl and the neck as a hooked horn. Front: the same turned outline with the neck column; plan: the bowl disc with the neck.",
                           "Ein Brunnenzeichen, 0,8 mm breit und 2 mm hoch: ein ausgestellter Fuß mit Sockelkörper, der Schaft, der ins Kapitell ausläuft, das Becken und der Hals als gekrümmtes Horn. Vorderansicht: derselbe gedrehte Umriss mit der Halssäule; Draufsicht: die Beckenscheibe mit dem Hals."),
        "retained": text("Height 1.0 m and bowl width 0.40 m exactly; foot, die, shaft, capital, bowl, neck hook, button and the jet hairline.", "Höhe 1,0 m und Beckenbreite 0,40 m exakt; Fuß, Sockelkörper, Schaft, Kapitell, Becken, Halshaken, Taster und die Strahl-Haarlinie."),
        "merged": text("Flange and foot into one splay; the neck is a horn of 54 to 32 mm (real 28 mm); the button is a 45 mm stud.", "Flansch und Fuß zu einer Ausstellung; der Hals ist ein Horn von 54 bis 32 mm (real 28 mm); der Taster ist ein 45-mm-Zapfen."),
        "omitted": text("Every moulding, the door, rim and nozzle. Button and jet are not drawn in plan.", "Jedes Profil, die Tür, Rand und Düse. Taster und Strahl werden in der Draufsicht nicht gezeichnet."),
        "thinElements": text("The jet is a 0.13 mm hairline for the 12 mm stream.", "Der Strahl ist eine 0,13-mm-Haarlinie für den 12 mm dicken Strahl."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tones as at 1:50; body pen 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,13 mm."),
        "overlays": text("The jet carries water-jet.", "Der Strahl trägt water-jet."),
    },
}


def apply(asset):
    """🖊️ Rewrites the geometry and the decisions of every representation; pens and valid ranges stay."""
    for scale in (50, 100, 200, 500):
        rep = asset["representations"][str(scale)]
        rep["hatches"] = []
        rep["geometry"] = {"elevation": upright(scale, "elevation"), "front": upright(scale, "front"), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[scale]
