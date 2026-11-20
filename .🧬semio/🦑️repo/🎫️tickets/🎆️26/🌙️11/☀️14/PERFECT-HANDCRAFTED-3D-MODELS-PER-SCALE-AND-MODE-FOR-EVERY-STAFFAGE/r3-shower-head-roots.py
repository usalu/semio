"""🌱 Round 3, appliances/shower-head: the root fixes (rig with a vertical rail bone, the hand shower standing in its socket, a head bone on the spray axis, anchors, masses, dimensions, areas, texts) and the three drawings of 1:50, 1:100 and 1:200 redrawn as the true views of each scale's body (ruling E1); writes the candidate `🗑️generated/r3-shower-head/🔣️.json` with the volumes source merged in.

Usage: python3 r3-shower-head-roots.py
"""
import copy
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "shower-head.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-shower-head", "🔣️.json")


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, f"r3-shower-head-{name}.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


k = load("kit")
m = load("model")

AREAS = {"rail": 0.083, "valve": 0.0167, "hose": 0.051, "handle": 0.0147, "head": 0.0303}
DIMENSIONS = {"heightM": 1.12, "widthM": 0.245, "depthM": 0.191, "massKg": 2.27}
ROOT_Y = 1.1
MATERIALS = {"rail": "chrome", "valve": "chrome", "hose": "epdm-hose", "handle": "abs-chrome", "head": "abs-chrome"}


def text(en, de):
    return {"en": en, "de": de}


def r5(x):
    return round(x, 5) + 0.0


def pts(points):
    return [[r5(x), r5(y)] for x, y in points]


def sh(i, bone, role, z, tone, region, geometry, filled=True, material=None, solid=None, pen=None):
    """🖊️ One shape: the common members in the order of the file, then the geometry."""
    d = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    d.update(geometry)
    d["region"] = region
    d["material"] = material or MATERIALS[region]
    if solid:
        d["solid"] = solid
    if pen:
        d["penMm"] = pen
    return d


def poly(points, closed=True):
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x, y, w, h, r=None):
    d = {"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)}
    if r:
        d["r"] = r
    return d


def circle(cx, cy, r):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def revolved(bone):
    return {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "mirror": True}


def rail_local(points):
    """🧭 World (x, y) points in the frame of the vertical rail bone."""
    return [k.to_local("rail", p) for p in points]


def about_rail(profile):
    """🪈 The side outline of a part turned about the rail axis, in the rail frame."""
    return [(a, -k.RAIL_X + r) for a, r in k.mirror_profile(profile)]


def about_wall(profile, y):
    """🧱 The side outline of a part turned about a horizontal axis out of the wall at height y, in world."""
    return [(a, y + r) for a, r in k.mirror_profile(profile)]


def downward(profile, x=k.VALVE_X, y0=0.0):
    """⬇️ The side outline of a part turned about the vertical outlet axis (axial = distance down from the elbow axis), elbow frame."""
    return [(x + r, y0 - a) for a, r in k.mirror_profile(profile)]


def hose_shape(view, pen, z):
    return sh("hose", "elbow", "body", z, 0.55, "hose", {"kind": "path", "d": k.hose_path_d(view)}, filled=False, pen=pen)


def lateral(geometry, view):
    """↔️ A front or plan geometry of the elbow moved to the lateral position of the elbow."""
    g = copy.deepcopy(geometry)
    axis = "x" if view == "front" else "y"
    if g["kind"] == "poly":
        g["points"] = [[r5(x + k.ELBOW_Z), y] if view == "front" else [x, r5(y + k.ELBOW_Z)] for x, y in g["points"]]
    elif g["kind"] == "ellipse":
        g["c" + axis] = r5(g["c" + axis] + k.ELBOW_Z)
    else:
        g[axis] = r5(g[axis] + k.ELBOW_Z)
    return g


def shower_shapes(handle, head, role_small, with_parts, neck=None):
    """🚿 The hand shower: every part is a turned solid, so the three views are one definition."""
    out = []
    if with_parts:
        out.append(sh("nut", "shower", role_small, 3.42, 0.75, "hose", poly(k.mirror_profile(m.NUT_50)), material="chrome", solid=revolved("shower")))
    out.append(sh("handle", "shower", "body", 3.5, 0.82, "handle", poly(k.mirror_profile(handle)), solid=revolved("shower")))
    if with_parts:
        out.append(sh("socket", "shower", "body", 3.55, 0.72, "rail", poly(k.mirror_profile(m.SOCKET_50)), solid=revolved("shower")))
    if neck:
        out.append(sh("neck", "shower", "body", 3.57, 0.86, "head", poly(neck)))
    out.append(sh("head", "head", "body", 3.6, 0.9, "head", poly(k.mirror_profile(head)), solid=revolved("head")))
    return out


def geometry_50():
    lever = {"kind": "capsule", "x0": 0, "y0": 0, "x1": 0.0435, "y1": 0, "r0": 0.008, "r1": 0.0065}
    lever_solid = {"kind": "extruded", "sweep": "w", "z0": r5(k.ELBOW_Z - 0.036), "z1": r5(k.ELBOW_Z - 0.027)}
    bracket = lambda y: [(0, y - 0.022), (0.006, y - 0.022), (0.010, y - 0.019), (0.010, y - 0.011), (0.047, y - 0.0101), (0.047, y + 0.0101), (0.010, y + 0.011), (0.010, y + 0.019), (0.006, y + 0.022), (0, y + 0.022)]
    face = poly([(0.0115, -0.045), (0.0115, 0.045)], False)
    neck = [(u, v + ry) for u, v, rx, ry in m.NECK_50] + [(u, v - ry) for u, v, rx, ry in reversed(m.NECK_50)]
    half_front = [(0.0115, 0.725), (0.0129, 0.7394), (0.0269, 0.7857), (0.026, 0.7943), (0.0199, 0.8026), (0.0105, 0.8053)]
    neck_front = half_front + [(-x, y) for x, y in reversed(half_front)]
    half_plan = [(0.139, 0.0115), (0.1425, 0.0129), (0.1594, 0.027), (0.1652, 0.0261), (0.173, 0.0143)]
    neck_plan = [(x, -y) for x, y in half_plan] + list(reversed(half_plan))
    cheek = [(0.0275, -0.010), (0.0275, -0.0085), (0.0315, -0.0075), (0.0315, 0.0075), (0.0275, 0.0085), (0.0275, 0.010)]
    yoke_front = [(z, k.HOLDER_Y + y) for z, y in cheek + [(-z, -y) for z, y in cheek]]
    yoke_plan = [(k.RAIL_X + u, w) for w, u in [(-0.020, 0.014), (0.020, 0.014), (0.0275, 0.022), (0.0275, 0.037), (0.0315, 0.038), (0.0315, 0.052), (0.0255, 0.053), (0.020, 0.053), (0.020, 0.021), (-0.020, 0.021), (-0.020, 0.053), (-0.0255, 0.053), (-0.0315, 0.052), (-0.0315, 0.038), (-0.0275, 0.037), (-0.0275, 0.022)]]
    elevation = [
        hose_shape("elevation", 0.26, 3),
        sh("tube", "rail", "body", 3.05, 0.8, "rail", rect(0.110, -k.RAIL_X - k.RAIL_R, 0.670, 2 * k.RAIL_R)),
        sh("bracket-low", "rail", "body", 3.1, 0.6, "rail", poly(rail_local(bracket(k.Y_LOW)))),
        sh("bracket-top", "rail", "body", 3.1, 0.6, "rail", poly(rail_local(bracket(k.Y_TOP)))),
        sh("socket-low", "rail", "body", 3.15, 0.6, "rail", poly(about_rail(m.SOCKET_LOW_50))),
        sh("socket-top", "rail", "body", 3.15, 0.6, "rail", poly(about_rail(m.SOCKET_TOP_50))),
        sh("elbow", "elbow", "body", 3.2, 0.7, "valve", poly(k.mirror_profile(m.ESCUTCHEON_50))),
        sh("nut-elbow", "elbow", "detail", 3.25, 0.75, "hose", poly(downward(m.NUT_ELBOW_50)), material="chrome"),
        sh("valve", "elbow", "body", 3.3, 0.7, "valve", poly(k.mirror_profile(m.VALVE_50))),
        sh("lever", "lever", "detail", 3.4, 0.85, "valve", lever),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", rect(-0.017, -0.025, 0.034, 0.050, 0.002)),
        *shower_shapes(m.HANDLE_50, m.HEAD_50, "detail", True, neck),
        sh("holder-arm", "holder", "body", 3.62, 0.7, "rail", rect(0.014, -m.YOKE_HALF, 0.039, 2 * m.YOKE_HALF)),
        sh("pivot-right", "holder", "detail", 3.64, 0.75, "rail", circle(k.PIVOT_X - k.RAIL_X, 0, 0.0085)),
        sh("face", "head", "detail", 3.7, 0.4, "head", face, filled=False, material="rubber-nozzle"),
    ]
    front = [
        sh("bracket-low", "rail", "body", 3.1, 0.6, "rail", circle(0, k.Y_LOW, k.ROSETTE_R)),
        sh("bracket-top", "rail", "body", 3.1, 0.6, "rail", circle(0, k.Y_TOP, k.ROSETTE_R)),
        sh("elbow", "elbow", "body", 3.1, 0.7, "valve", lateral(circle(0, k.ELBOW_Y, 0.030), "front")),
        sh("tube", "rail", "body", 3.15, 0.8, "rail", rect(-k.RAIL_R, 0.110, 2 * k.RAIL_R, 0.670)),
        sh("socket-low", "rail", "body", 3.2, 0.6, "rail", poly([(-0.015, 0.122), (-0.015, 0.088), (-0.0135, 0.085), (-0.009, 0.0815), (0, 0.080), (0.009, 0.0815), (0.0135, 0.085), (0.015, 0.088), (0.015, 0.122)])),
        sh("socket-top", "rail", "body", 3.2, 0.6, "rail", poly([(-0.015, 0.768), (-0.015, 0.802), (-0.0135, 0.805), (-0.009, 0.8085), (0, 0.810), (0.009, 0.8085), (0.0135, 0.805), (0.015, 0.802), (0.015, 0.768)])),
        sh("lever-boss", "elbow", "detail", 3.25, 0.7, "valve", lateral(rect(-0.028, k.ELBOW_Y - 0.0095, 0.013, 0.019), "front")),
        sh("knob", "holder", "detail", 3.28, 0.75, "rail", poly([(0.015, 0.609), (0.030, 0.607), (0.034, 0.609), (0.034, 0.631), (0.030, 0.633), (0.015, 0.631)])),
        sh("valve", "elbow", "body", 3.3, 0.7, "valve", lateral(circle(0, k.ELBOW_Y, 0.017), "front")),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", rect(-0.017, k.HOLDER_Y - 0.025, 0.034, 0.050, 0.002)),
        sh("nut-elbow", "elbow", "detail", 3.35, 0.75, "hose", lateral(poly([(-0.0125, 0.026), (0.0125, 0.026), (0.0125, 0.010), (0.009, 0.0), (-0.009, 0.0), (-0.0125, 0.010)]), "front"), material="chrome"),
        hose_shape("front", 0.26, 3.36),
        sh("lever", "lever", "detail", 3.38, 0.85, "valve", lever, solid=lever_solid),
        sh("holder-arm", "holder", "body", 3.4, 0.7, "rail", poly(yoke_front)),
        sh("neck", "shower", "body", 3.41, 0.86, "head", poly(neck_front)),
        *shower_shapes(m.HANDLE_50, m.HEAD_50, "detail", True),
        sh("face", "head", "detail", 3.7, 0.4, "head", face, filled=False, material="rubber-nozzle", solid=revolved("head")),
    ]
    plan = [
        hose_shape("plan", 0.26, 3),
        sh("elbow", "elbow", "body", 3.05, 0.7, "valve", lateral(poly([(0, -0.030), (0.005, -0.030), (0.009, -0.027), (0.009, 0.027), (0.005, 0.030), (0, 0.030)]), "plan")),
        sh("lever-boss", "elbow", "detail", 3.08, 0.7, "valve", lateral(rect(k.VALVE_X - 0.0095, -0.028, 0.019, 0.013), "plan")),
        sh("valve", "elbow", "body", 3.1, 0.7, "valve", lateral(poly(k.mirror_profile(m.VALVE_50)), "plan")),
        sh("lever", "lever", "detail", 3.12, 0.85, "valve", lever, solid=lever_solid),
        sh("knob", "holder", "detail", 3.28, 0.75, "rail", poly([(0.049, 0.015), (0.071, 0.015), (0.073, 0.030), (0.071, 0.034), (0.049, 0.034), (0.047, 0.030)])),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", circle(k.RAIL_X, 0, 0.017)),
        sh("holder-arm", "holder", "body", 3.32, 0.7, "rail", poly(yoke_plan)),
        sh("bracket-top", "rail", "body", 3.36, 0.6, "rail", poly(bracket(0.0))),
        sh("socket-top", "rail", "body", 3.38, 0.6, "rail", circle(k.RAIL_X, 0, k.SOCKET_R)),
        *shower_shapes(m.HANDLE_50, m.HEAD_50, "detail", True),
        sh("neck", "shower", "body", 3.65, 0.86, "head", poly(neck_plan)),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry_100():
    bracket = lambda y: about_wall(m.BRACKET_100, y)
    knuckle = [(0.010, -0.012), (0.057, -0.012), (0.057, 0.012), (0.010, 0.012)]
    elevation = [
        hose_shape("elevation", 0.15, 3),
        sh("tube", "rail", "body", 3.05, 0.8, "rail", rect(0.115, -k.RAIL_X - k.RAIL_R, 0.660, 2 * k.RAIL_R)),
        sh("bracket-low", "rail", "body", 3.1, 0.6, "rail", poly(rail_local(bracket(k.Y_LOW)))),
        sh("bracket-top", "rail", "body", 3.1, 0.6, "rail", poly(rail_local(bracket(k.Y_TOP)))),
        sh("socket-low", "rail", "body", 3.15, 0.6, "rail", rect(0.080, -k.RAIL_X - k.SOCKET_R, 0.042, 2 * k.SOCKET_R)),
        sh("socket-top", "rail", "body", 3.15, 0.6, "rail", rect(0.768, -k.RAIL_X - k.SOCKET_R, 0.042, 2 * k.SOCKET_R)),
        sh("outlet", "elbow", "body", 3.2, 0.7, "valve", poly(downward(m.OUTLET_100))),
        sh("elbow", "elbow", "body", 3.3, 0.7, "valve", poly(k.mirror_profile(m.ELBOW_100))),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", rect(-0.017, -0.025, 0.034, 0.050)),
        sh("knuckle", "holder", "body", 3.34, 0.7, "rail", poly(knuckle)),
        *shower_shapes(m.HANDLE_100, m.HEAD_100, "body", False),
    ]
    front = [
        sh("bracket-low", "rail", "body", 3.1, 0.6, "rail", circle(0, k.Y_LOW, k.ROSETTE_R)),
        sh("bracket-top", "rail", "body", 3.1, 0.6, "rail", circle(0, k.Y_TOP, k.ROSETTE_R)),
        sh("elbow", "elbow", "body", 3.1, 0.7, "valve", lateral(circle(0, k.ELBOW_Y, 0.030), "front")),
        sh("tube", "rail", "body", 3.15, 0.8, "rail", rect(-k.RAIL_R, 0.115, 2 * k.RAIL_R, 0.660)),
        sh("socket-low", "rail", "body", 3.2, 0.6, "rail", rect(-k.SOCKET_R, 0.080, 2 * k.SOCKET_R, 0.042)),
        sh("socket-top", "rail", "body", 3.2, 0.6, "rail", rect(-k.SOCKET_R, 0.768, 2 * k.SOCKET_R, 0.042)),
        sh("knob", "holder", "body", 3.28, 0.75, "rail", rect(0.015, k.HOLDER_Y - 0.012, 0.019, 0.024)),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", rect(-0.017, k.HOLDER_Y - 0.025, 0.034, 0.050)),
        sh("outlet", "elbow", "body", 3.35, 0.7, "valve", lateral(poly([(-0.0125, 0.040), (0.0125, 0.040), (0.0125, 0.008), (0.009, 0.0), (-0.009, 0.0), (-0.0125, 0.008)]), "front")),
        hose_shape("front", 0.15, 3.36),
        sh("knuckle", "holder", "body", 3.4, 0.7, "rail", rect(-0.019, k.HOLDER_Y - 0.012, 0.038, 0.024)),
        *shower_shapes(m.HANDLE_100, m.HEAD_100, "body", False),
    ]
    plan = [
        hose_shape("plan", 0.15, 3),
        sh("elbow", "elbow", "body", 3.1, 0.7, "valve", lateral(poly(k.mirror_profile(m.ELBOW_100)), "plan")),
        sh("knob", "holder", "body", 3.28, 0.75, "rail", rect(k.RAIL_X - 0.012, 0.015, 0.024, 0.019)),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", circle(k.RAIL_X, 0, 0.017)),
        sh("knuckle", "holder", "body", 3.34, 0.7, "rail", poly([(0.070, -0.013), (0.094, -0.019), (0.117, -0.019), (0.117, 0.019), (0.094, 0.019), (0.070, 0.013)])),
        sh("bracket-top", "rail", "body", 3.36, 0.6, "rail", poly(bracket(0.0))),
        sh("socket-top", "rail", "body", 3.38, 0.6, "rail", circle(k.RAIL_X, 0, k.SOCKET_R)),
        *shower_shapes(m.HANDLE_100, m.HEAD_100, "body", False),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry_200():
    low, top = k.Y_LOW - 0.007, k.Y_TOP + 0.007
    holder = [(-0.019, -0.019), (0.020, -0.019), (0.057, -0.016), (0.057, 0.016), (0.020, 0.019), (-0.019, 0.019)]
    elevation = [
        hose_shape("elevation", 0.15, 3),
        sh("bracket-low", "rail", "body", 3.1, 0.8, "rail", rect(low - 0.015, -0.050, 0.030, 0.050)),
        sh("bracket-top", "rail", "body", 3.1, 0.8, "rail", rect(top - 0.015, -0.050, 0.030, 0.050)),
        sh("tube", "rail", "body", 3.2, 0.8, "rail", rect(0.080, -k.RAIL_X - 0.015, 0.730, 0.030)),
        sh("elbow", "elbow", "body", 3.3, 0.7, "valve", poly(k.mirror_profile(m.ELBOW_200))),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", poly(holder)),
        *shower_shapes(m.HANDLE_200, m.HEAD_200, "body", False),
    ]
    front = [
        sh("elbow", "elbow", "body", 3.1, 0.7, "valve", lateral(circle(0, k.ELBOW_Y, 0.030), "front")),
        sh("tube", "rail", "body", 3.2, 0.8, "rail", rect(-0.015, 0.080, 0.030, 0.730)),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", rect(-0.019, k.HOLDER_Y - 0.019, 0.038, 0.038)),
        hose_shape("front", 0.15, 3.36),
        *shower_shapes(m.HANDLE_200, m.HEAD_200, "body", False),
    ]
    plan = [
        hose_shape("plan", 0.15, 3),
        sh("elbow", "elbow", "body", 3.1, 0.7, "valve", lateral(poly(k.mirror_profile(m.ELBOW_200)), "plan")),
        sh("holder", "holder", "body", 3.3, 0.7, "rail", poly([(x + k.RAIL_X, y) for x, y in holder])),
        sh("bracket-top", "rail", "body", 3.36, 0.8, "rail", rect(0, -0.015, 0.050, 0.030)),
        sh("tube", "rail", "body", 3.38, 0.8, "rail", circle(k.RAIL_X, 0, 0.015)),
        *shower_shapes(m.HANDLE_200, m.HEAD_200, "body", False),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


DECISIONS = {
    "50": {
        "silhouette": text("A wall set of 1.12 m by 0.25 m, 22 mm by 5 mm on paper: a thin riser rail between two end brackets, a sliding holder whose conical socket carries a hand shower standing on its hose nut (a slim handle whose neck runs as a flat tongue onto the back of a round spray disc seen edge-on), a wall elbow with a small lever under the rail and the hose hanging as one bight from the elbow up to the handle. Front view: rosettes, end sockets and rail, the elbow as two circles with its lever at the side, the holder with its clamp button and the fork with its two pivot pins, and the spray disc as the ellipse of a 110 mm disc tilted 41 degrees; the hose hangs in the plane of the rail and shows as a line. Top view: the same parts from above.", "Eine Wandgarnitur von 1,12 m mal 0,25 m, auf Papier 22 mm mal 5 mm: eine dünne Brausestange zwischen zwei Endhaltern, ein Gleithalter, dessen konische Aufnahme eine auf ihrer Schlauchmutter stehende Handbrause trägt (ein schlanker Griff, dessen Hals als flache Zunge auf den Rücken einer runden, in der Kante gesehenen Brausescheibe läuft), ein Wandanschluss mit kleinem Hebel unter der Stange und der Schlauch, der als ein Bogen vom Anschluss hinauf zum Griff hängt. Vorderansicht: Rosetten, Endhülsen und Stange, der Anschluss als zwei Kreise mit seitlichem Hebel, der Halter mit Klemmknopf und Gabel mit ihren zwei Gelenkzapfen und die Brausescheibe als Ellipse einer um 41 Grad geneigten Scheibe von 110 mm; der Schlauch hängt in der Ebene der Stange und erscheint als Linie. Draufsicht: dieselben Teile von oben."),
        "retained": text("Rail of 22 mm, both brackets with rosette, arm and end socket, holder sleeve with forked arm and pivot pin, socket clip, hose nuts, handle of 25 to 29 mm, neck tongue, spray disc of 110 by 26 mm with its nozzle face line, elbow escutcheon, valve body, lever, hose of 1.25 m.", "Stange von 22 mm, beide Halter mit Rosette, Arm und Endhülse, Halterhülse mit Gabelarm und Gelenkzapfen, Aufnahmeklammer, Schlauchmuttern, Griff von 25 bis 29 mm, Halszunge, Brausescheibe von 110 mal 26 mm mit der Linie ihrer Düsenfläche, Anschlussrosette, Ventilkörper, Hebel, Schlauch von 1,25 m."),
        "merged": text("The outlet thread joins the elbow nut; the lever hub joins the lever; the rubber nozzles are the face line.", "Das Abgangsgewinde geht in der Anschlussmutter auf, die Hebelnabe im Hebel; die Gummidüsen sind die Linie der Düsenfläche."),
        "omitted": text("The spray mode selector, the bracket screws and the water itself, which only the emitters show. The side view omits the clamp button, the lever boss and the slot of the socket clip (hidden); the front view omits the bracket arms (hidden); the top view omits the lower bracket, the rail, the elbow nut and the nozzle face (hidden).", "Der Strahlartwähler, die Halterschrauben und das Wasser selbst, das nur die Emitter zeigen. Die Seitenansicht lässt Klemmknopf, Hebelnabe und den Schlitz der Aufnahmeklammer weg (verdeckt); die Vorderansicht die Halterarme der Stange (verdeckt); die Draufsicht den unteren Halter, die Stange, die Anschlussmutter und die Düsenfläche (verdeckt)."),
        "thinElements": text("The hose is drawn as a 0.26 mm line, its true width of 13 mm; the lever is 13 to 16 mm wide (0.3 mm); the face line is a 0.13 mm detail line.", "Der Schlauch wird als 0,26-mm-Linie gezeichnet, seine wahre Breite von 13 mm; der Hebel ist 13 bis 16 mm (0,3 mm) breit; die Düsenlinie ist eine 0,13-mm-Detaillinie."),
        "hatching": text("None.", "Keine."),
        "styles": text("Brackets and sockets 0.6, tube 0.8, elbow and holder 0.7, socket 0.72, nuts and pivot pin 0.75, lever 0.85, handle 0.82, neck 0.86, head 0.9, hose 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Halter und Hülsen 0,6, Rohr 0,8, Anschluss und Gleithalter 0,7, Aufnahme 0,72, Muttern und Gelenkzapfen 0,75, Hebel 0,85, Griff 0,82, Hals 0,86, Kopf 0,9, Schlauch 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Rail, valve, hose, handle and head are five regions (the hose nuts count to the handle and the valve, the neck to the head): in the thermal view the running water warms the head, the handle and the hose; the spray region is air-borne and has no shape (see its hidden note).", "Stange, Ventil, Schlauch, Griff und Kopf sind fünf Regionen (die Schlauchmuttern zählen zu Griff und Ventil, der Hals zum Kopf): in der Thermalansicht erwärmt das laufende Wasser Kopf, Griff und Schlauch; die Sprühregion ist luftgetragen und hat keine Form (siehe ihren Hinweis)."),
    },
    "100": {
        "silhouette": text("A set of 11 mm by 2.5 mm: a thin rail between two brackets, a holder carrying a slim handle with a round disc at its top, a wall elbow and the hose bight. Front view: rosettes, sockets and rail, the elbow circle with its outlet, the holder with its button, the handle and the spray disc as an ellipse, the hose as a line in the plane of the rail. Top view: the same parts from above.", "Eine Garnitur von 11 mm mal 2,5 mm: eine dünne Stange zwischen zwei Haltern, ein Gleithalter mit schlankem Griff und runder Scheibe an dessen Ende, ein Wandanschluss und der Schlauchbogen. Vorderansicht: Rosetten, Hülsen und Stange, der Anschlusskreis mit Abgang, der Halter mit Knopf, der Griff und die Brausescheibe als Ellipse, der Schlauch als Linie in der Ebene der Stange. Draufsicht: dieselben Teile von oben."),
        "retained": text("Rail at its true 22 mm (0.22 mm), both brackets with end sockets, holder sleeve with hinge block and button, handle, spray disc, elbow with outlet, hose bight.", "Stange in ihren wahren 22 mm (0,22 mm), beide Halter mit Endhülsen, Halterhülse mit Gelenkblock und Knopf, Griff, Brausescheibe, Anschluss mit Abgang, Schlauchbogen."),
        "merged": text("Rosette and arm of each bracket are one turned piece; socket and hose nut join the handle; escutcheon and valve are one body; outlet and nut are one cylinder; arm and hinge are one block.", "Rosette und Arm jedes Halters sind ein Drehteil; Aufnahme und Schlauchmutter gehen im Griff auf; Rosette und Ventil sind ein Körper; Abgang und Mutter ein Zylinder; Arm und Gelenk ein Block."),
        "omitted": text("Lever, neck tongue, nozzle line and mode selector. The front view omits the bracket arms and the top view the lower bracket, the rail and the outlet (hidden).", "Hebel, Halszunge, Düsenlinie und Strahlartwähler. Die Vorderansicht lässt die Halterarme weg, die Draufsicht den unteren Halter, die Stange und den Abgang (verdeckt)."),
        "thinElements": text("Nothing is thickened: the rail of 22 mm prints at 0.22 mm; the hose is a 0.15 mm line, the smallest printable feature, for its true 0.13 mm.", "Nichts ist verdickt: die Stange von 22 mm druckt mit 0,22 mm; der Schlauch ist eine 0,15-mm-Linie, das kleinste druckbare Merkmal, für wahre 0,13 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Brackets and sockets 0.6, tube 0.8, elbow and holder 0.7, handle 0.82, head 0.9, hose 0.55; one body pen of 0.18 mm.", "Halter und Hülsen 0,6, Rohr 0,8, Anschluss und Gleithalter 0,7, Griff 0,82, Kopf 0,9, Schlauch 0,55; ein Körperstift von 0,18 mm."),
        "overlays": text("All five regions keep their own shapes; the hose nuts count to the handle and the valve.", "Alle fünf Regionen behalten ihre eigenen Formen; die Schlauchmuttern zählen zu Griff und Ventil."),
    },
    "200": {
        "silhouette": text("A set of 5.6 mm by 1.2 mm: a pole with two wall stubs, a holder block carrying a short handle with a disc at its top, the elbow block under the pole and the hose bight as a line. Front view: the pole, the elbow circle, the holder, the handle and the spray disc as an ellipse. Top view: wall stub and pole, elbow, holder, handle and disc.", "Eine Garnitur von 5,6 mm mal 1,2 mm: ein Stab mit zwei Wandstutzen, ein Halterblock mit kurzem Griff und Scheibe an dessen Ende, der Anschlussblock unter dem Stab und der Schlauchbogen als Linie. Vorderansicht: der Stab, der Anschlusskreis, der Halter, der Griff und die Brausescheibe als Ellipse. Draufsicht: Wandstutzen und Stab, Anschluss, Halter, Griff und Scheibe."),
        "retained": text("Pole with two wall stubs, holder, handle, spray disc, elbow and hose bight.", "Stab mit zwei Wandstutzen, Gleithalter, Griff, Brausescheibe, Anschluss und Schlauchbogen."),
        "merged": text("Rail, end sockets and caps are the pole; rosettes and arms are the stubs; escutcheon, valve, outlet and lever are the elbow block; sleeve, arm, hinge and socket are the holder block; hose nut and handle are one rod.", "Stange, Endhülsen und Kappen sind der Stab; Rosetten und Arme die Stutzen; Rosette, Ventil, Abgang und Hebel der Anschlussblock; Hülse, Arm, Gelenk und Aufnahme der Halterblock; Schlauchmutter und Griff ein Stab."),
        "omitted": text("Clamp button, lever and nozzle line. The front view omits the wall stubs (behind the pole) and the top view the lower stub.", "Klemmknopf, Hebel und Düsenlinie. Die Vorderansicht lässt die Wandstutzen weg (hinter dem Stab), die Draufsicht den unteren Stutzen."),
        "thinElements": text("The pole is drawn 30 mm wide (0.15 mm) instead of 22 mm, the handle 30 mm instead of 23 to 29 mm and the disc 30 mm thick instead of 26 mm; the hose bight is a 0.15 mm line for a true 0.065 mm and is not thickened further.", "Der Stab ist 30 mm (0,15 mm) statt 22 mm breit gezeichnet, der Griff 30 mm statt 23 bis 29 mm und die Scheibe 30 mm dick statt 26 mm; der Schlauchbogen ist eine 0,15-mm-Linie für wahre 0,065 mm und wird nicht weiter verdickt."),
        "hatching": text("None.", "Keine."),
        "styles": text("Pole and stubs 0.8, elbow and holder 0.7, handle 0.82, head 0.9, hose 0.55; body pen 0.13 mm, hose line 0.15 mm.", "Stab und Stutzen 0,8, Anschluss und Halter 0,7, Griff 0,82, Kopf 0,9, Schlauch 0,55; Körperstift 0,13 mm, Schlauchlinie 0,15 mm."),
        "overlays": text("All five regions keep a shape of their own.", "Alle fünf Regionen behalten eine eigene Form."),
    },
}


def title(asset, bone):
    return next(b["title"] for b in asset["rig"]["bones"] if b["id"] == bone)


def rig(asset):
    old = {b["id"]: b for b in asset["rig"]["bones"]}
    head_o = k.HEAD_AT
    mass = lambda bone, x, y: {**old[bone]["mass"], "comX": x, "comY": y}
    return {"kind": "mechanical", "root": "rail", "bones": [
        {"id": "rail", "parent": None, "x": 0, "y": 0, "angleDeg": 90, "title": text("Wall brackets and riser rail", "Wandhalter und Brausestange"), "lengthM": k.RAIL_TOP, "joint": {"type": "fixed"}, "mass": mass("rail", 0.445, -0.055)},
        {"id": "elbow", "parent": "rail", "x": k.ELBOW_Y, "y": 0, "z": k.ELBOW_Z, "angleDeg": -90, "title": old["elbow"]["title"], "lengthM": 0.06, "joint": {"type": "fixed"}, "mass": mass("elbow", 0.0545, 0.0168)},
        {"id": "lever", "parent": "elbow", "x": k.VALVE_X, "y": 0, "angleDeg": 0, "title": old["lever"]["title"], "lengthM": 0.05, "joint": old["lever"]["joint"], "mass": old["lever"]["mass"]},
        {"id": "holder", "parent": "rail", "x": k.HOLDER_Y, "y": -k.RAIL_X, "angleDeg": -90, "title": old["holder"]["title"], "lengthM": r5(k.PIVOT_X - k.RAIL_X), "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.47, "maxM": 0.12}, "mass": mass("holder", 0.015, 0)},
        {"id": "shower", "parent": "holder", "x": r5(k.PIVOT_X - k.RAIL_X), "y": 0, "angleDeg": k.HANDLE_DEG, "title": old["shower"]["title"], "lengthM": 0.24, "joint": {"type": "revolute", "minDeg": -40, "maxDeg": 10}, "mass": mass("shower", 0.134, -0.008)},
        {"id": "head", "parent": "shower", "x": head_o[0], "y": head_o[1], "angleDeg": k.CRANK_DEG - 90, "title": text("Spray disc on its spray axis", "Brausescheibe auf ihrer Strahlachse"), "lengthM": 0.01, "joint": {"type": "fixed"}},
    ]}


def anchors(asset):
    out = copy.deepcopy(asset["anchors"])
    by = {a["id"]: a for a in out}
    by["wall-low"].update({"x": k.Y_LOW, "y": 0, "footprintLengthM": 0.044, "footprintWidthM": 0.044})
    by["wall-top"].update({"x": k.Y_TOP, "y": 0, "footprintLengthM": 0.044, "footprintWidthM": 0.044})
    by["nozzle"].update({"bone": "head", "x": 0.01, "y": 0})
    by["steam-top"].update({"bone": "head", "x": 0.002, "y": 0.055, "title": text("Upper rim of the head, above the spray", "Oberer Rand des Kopfes über dem Strahl")})
    return out


def build():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    asset = doc["asset"]
    asset["summary"] = text("A wall-mounted hand shower of 0.11 m head diameter on a 0.7 m riser rail of 22 mm with a 1.25 m hose hanging as a bight to the wall elbow; running it delivers 12 L/min (eco 9 L/min) of 38 degC water as a warm spray and a vapour plume.", "Eine wandmontierte Handbrause mit 0,11 m Kopfdurchmesser an einer 0,7 m langen Brausestange von 22 mm mit einem 1,25 m langen Schlauch, der als Bogen zum Wandanschluss hängt; im Betrieb liefert sie 12 L/min (Sparmodus 9 L/min) Wasser von 38 degC als warmen Strahl und Dampffahne.")
    refs = [r for r in asset["provenance"]["references"] if r["id"] != "hansgrohe-croma-select-s"]
    refs.append({"id": "hansgrohe-croma-select-s", "citation": "hansgrohe SE, product data of the Croma Select S 110 shower set with the Unica'Croma wall bar 0.65 m (art. 26560400 and 26503000: spray disc 110 mm, bar 22 mm, pipe length 669 mm) and of the Isiflex shower hose 1.25 m (art. 28272000: G 1/2 with conical nuts at both ends, about 13 mm outside).", "url": "https://pro.hansgrohe-asia.com/articledetail-croma-select-s-shower-set-110-multi-with-shower-bar-65-cm-26560400"})
    asset["provenance"]["references"] = refs
    asset["provenance"]["notes"] = text(
        "A hand shower of the common 110 mm head diameter on a riser rail of 22 mm (hansgrohe product class): the rail is held at both ends by brackets (wall rosette, arm, end socket; bracket axes 0.686 m apart, rail centre 60 mm from the wall); a sleeve slides on it and carries, 45 mm in front of the rail, a tilting conical socket in which the hand shower stands on the conical nut of its hose. The wall elbow with a quarter-turn shut-off sits under the rail, its G 1/2 outlet points down, and the 1.25 m hose of 13 mm hangs from it as one bight in the plane of the rail up to the handle. The wall lies at x = 0; the origin is the lower end of the hose nut under the elbow on the wall. At rest the handle stands 72 degrees above the horizontal; the spray disc (110 mm across, 26 mm thick) is cranked 15 degrees forward on it, so that the spray goes 33 degrees below the horizontal; the poses tilt it by 8 and 4 degrees. The bone of the rail runs up the wall, the bone of the hand shower along the handle from the tilt axis, the bone of the head along the spray axis. Dimensions are the envelope in the running pose; the mass of 2.3 kg is rounded from this product class (authored estimate). The valve lever stands for the flow: down = off, halfway = eco 9 L/min (an aerated, water-saving spray), up = 12 L/min. The set is placed with the elbow 1.14 m and the spray disc about 1.9 m above the floor (common installation practice, authored estimate). The water temperature is 38 degC (DIN 1988-300). The spray is air-borne water without outline and carries 300 W of sensible heat to the room air at level 1 (12 L/min x 4.18 kJ/kgK x 1.5 K of cooling in flight); the latent part is the vapour of the spray-mist emitter: 0.036 kg/s of air with 0.007 kg/kg of excess vapour = 0.25 g/s = 600 W latent, about 0.15 kg of vapour in a 10 minute shower (ASHRAE).",
        "Eine Handbrause mit üblichem Kopfdurchmesser von 110 mm an einer Brausestange von 22 mm (Produktklasse hansgrohe): die Stange wird an beiden Enden von Haltern gefasst (Wandrosette, Arm, Endhülse; Halterachsen 0,686 m auseinander, Stangenmitte 60 mm von der Wand); darauf gleitet eine Hülse, die 45 mm vor der Stange eine schwenkbare konische Aufnahme trägt, in der die Handbrause auf der konischen Mutter ihres Schlauchs steht. Der Wandanschluss mit Vierteldreh-Absperrung sitzt unter der Stange, sein Abgang G 1/2 zeigt nach unten, und der 1,25 m lange Schlauch von 13 mm hängt von dort als ein Bogen in der Ebene der Stange hinauf zum Griff. Die Wand liegt bei x = 0; der Ursprung ist das untere Ende der Schlauchmutter unter dem Anschluss an der Wand. In Ruhe steht der Griff 72 Grad über der Waagerechten; die Brausescheibe (110 mm breit, 26 mm dick) ist darauf um 15 Grad nach vorn gekröpft, sodass der Strahl 33 Grad unter die Waagerechte geht; die Posen neigen ihn um 8 und 4 Grad. Der Knochen der Stange läuft die Wand hinauf, der Knochen der Handbrause von der Schwenkachse den Griff entlang, der Knochen des Kopfes entlang der Strahlachse. Die Maße sind die Hülle in der Pose Läuft; die Masse von 2,3 kg ist aus dieser Produktklasse gerundet (Schätzwert). Der Ventilhebel steht für die Menge: unten = aus, halbhoch = Sparmodus 9 L/min (ein belüfteter, wassersparender Strahl), oben = 12 L/min. Die Garnitur sitzt mit dem Anschluss 1,14 m und der Brausescheibe etwa 1,9 m über dem Boden (übliche Montagepraxis, Schätzwert). Die Wassertemperatur beträgt 38 degC (DIN 1988-300). Der Strahl ist luftgetragenes Wasser ohne Kontur und gibt auf Stufe 1 300 W fühlbare Wärme an die Raumluft ab (12 L/min x 4,18 kJ/kgK x 1,5 K Abkühlung im Flug); der latente Anteil ist der Dampf des Emitters spray-mist: 0,036 kg/s Luft mit 0,007 kg/kg Dampfüberschuss = 0,25 g/s = 600 W latent, etwa 0,15 kg Dampf in einer Dusche von 10 Minuten (ASHRAE).")
    asset["dimensions"] = dict(DIMENSIONS)
    asset["rig"] = rig(asset)
    asset["anchors"] = anchors(asset)
    for scale, geometry in (("50", geometry_50()), ("100", geometry_100()), ("200", geometry_200())):
        asset["representations"][scale]["geometry"] = geometry
        asset["representations"][scale]["decisions"] = DECISIONS[scale]
    for pose in asset["poses"]:
        pose["root"]["y"] = ROOT_Y
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "hose":
            region["characteristicLengthM"] = 0.013
    for emitter in asset["thermal"]["emitters"]:
        emitter["directionDeg"] = 0 if emitter["id"] == "spray-mist" else 123
    asset["structure"]["assumptions"] = [text("The 2.3 kg set hangs on the lower bracket (pin) and is held at the upper bracket (roller): the pair, 0.686 m apart, resists the overturning moment of the hand shower, whose disc stands 0.2 m from the wall.", "Die 2,3 kg schwere Garnitur hängt am unteren Halter (Festlager) und wird am oberen Halter gehalten (Rollenlager): das Paar, 0,686 m auseinander, nimmt das Kippmoment der Handbrause auf, deren Scheibe 0,2 m vor der Wand steht.")]
    asset["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    return doc


def main():
    doc = build()
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(doc, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    print("written", OUT)


if __name__ == "__main__":
    main()
