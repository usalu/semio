"""🌱 Round 3 panel radiator (input: `r3-panel-radiator-2d-before.json`, the 2D shapes of the three scales before this round, so the script can be run again): the root values outside `volumes` (references, dimensions, materials, rig, anchors, the 2D drawings of 1:50, 1:100 and 1:200 as true views of the handcrafted body, thermal titles and areas, supports).

Usage: python3 r3-panel-radiator-roots.py            writes the candidate document (roots + the volumes source) to 🗑️generated/r3-panel-radiator/🔣️.json
       python3 r3-panel-radiator-roots.py --apply    re-reads the catalog file and replaces the changed members and `volumes` in one write
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-panel-radiator-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
r5, pts = K.r5, K.pts

CATALOG = os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🧱️components", "panel-radiator", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "components", "panel-radiator.json")
MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "thermal", "structure", "volumes"]
RADIATOR_KG, HEAD_KG = 35.6, 0.18
AREAS = {"valve": 0.052, "bracket": 0.024, "top-grille": 0.27}
Y0 = 0.11


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def shape(sid, role, z, filled, tone, geometry, bone="radiator", region=None, material=None, pen=None, regions=None):
    """✏️ A 2D shape with its members in the catalog's order."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    s.update(geometry)
    return s


def poly(points, closed=True):
    """📐 Polygon or polyline geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x0, y0, x1, y1, r=None):
    """▭ Rectangle geometry from two corners."""
    g = {"kind": "rect", "x": r5(x0), "y": r5(y0), "w": r5(x1 - x0), "h": r5(y1 - y0)}
    if r is not None:
        g["r"] = r
    return g


def circle(cx, cy, rad):
    """⚪ Circle geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rad), "ry": r5(rad)}


def path(segments):
    """〰️ Open strokes as one path."""
    return {"kind": "path", "d": " ".join("M %s %s L %s %s" % tuple(r5(c) for c in s) for s in segments)}


def zigzag(v_a, v_b, u0, u1, n):
    """⚡ The symbolic fin corrugation of one convector in the section, n segments between u0 and u1."""
    return [(u0 + (u1 - u0) * k / n, v_a if k % 2 == 0 else v_b) for k in range(n + 1)]


HEAD_FRONT = [(-0.497, 0.018), (-0.514, 0.018), (-0.514, 0.0215), (-0.516, 0.026), (-0.583, 0.026), (-0.589, 0.0235), (-0.593, 0.018), (-0.595, 0.009)]


def head_outline(cx, cy, flip_y=False):
    """🌡️ The thermostat head with its collar seen from the side of its axis (front and plan), the axis along x at height cy."""
    top = [(w, cy + r) for w, r in HEAD_FRONT]
    bottom = [(w, cy - r) for w, r in reversed(HEAD_FRONT)]
    ring = top + bottom
    return [(-x, y) for x, y in ring] if flip_y else ring


def plan_head():
    """🌡️ The head in plan: x from the wall, y = w."""
    cx = K.WALL - K.HEAD_V
    return [(cx + r, w) for w, r in HEAD_FRONT] + [(cx - r, w) for w, r in reversed(HEAD_FRONT)]


def slots_plan(x):
    """🕳️ One row of eight grille slots in plan as dashes."""
    gap = (0.91 - K.SLOTS * K.SLOT_LEN) / (K.SLOTS - 1)
    return path([(x, 0.455 - k * (K.SLOT_LEN + gap), x, 0.455 - k * (K.SLOT_LEN + gap) - K.SLOT_LEN) for k in range(K.SLOTS)])


def drawing50(old):
    """🖊️ 1:50: section, front and plan of the valve-compact radiator as built."""
    byid = {v: {s["id"]: s for s in old[v]} for v in old}
    ev = byid["elevation"]
    elevation = [ev["floor"], ev["panel-rear"], ev["panel-front-lower"], ev["panel-front-mid"], ev["panel-front-upper"], ev["end-cap-far"]]
    elevation += [
        dict(ev["convector-rear"], points=pts(zigzag(-0.022, -0.05, 0.025, 0.565, 11))),
        dict(ev["convector-front"], points=pts(zigzag(-0.05, -0.078, 0.025, 0.565, 11))),
        dict({k: v for k, v in ev["top-grille"].items() if k not in ("points", "closed", "kind")}, **poly([(0.575, 0.0015), (0.6, 0.0015), (0.6, -0.1015), (0.575, -0.1015), (0.575, -0.1), (0.585, -0.1), (0.585, 0.0), (0.575, 0.0)])),
        ev["grille-slot-0"], ev["grille-slot-1"], ev["grille-slot-2"], ev["grille-slot-3"],
        dict({k: v for k, v in ev["bracket-upper"].items() if k not in ("points", "closed", "kind")}, **poly([(0.44, 0.035), (0.44, 0.032), (0.47, 0.032), (0.47, 0.0), (0.494, 0.0), (0.494, 0.004), (0.482, 0.004), (0.482, 0.032), (0.49, 0.032), (0.49, 0.035)])),
        dict({k: v for k, v in ev["bracket-lower"].items() if k not in ("points", "closed", "kind")}, **poly([(0.025, 0.035), (0.025, 0.032), (0.05, 0.032), (0.05, 0.0), (0.062, 0.0), (0.062, 0.032), (0.075, 0.032), (0.075, 0.035)])),
        shape("valve-head", "detail", 4, True, 0.3, circle(0, 0, K.HEAD_R), bone="valve-head", region="valve", material="abs-plastic", pen=0.235),
        shape("valve-index", "texture", 5, False, 0.1, poly([(0, 0), (0, 0.022)], False), bone="valve-head", region="valve"),
        shape("connection-nut-upper", "detail", 3, True, 0.3, poly([(-0.03, -0.0673), (-0.012, -0.0673), (-0.012, -0.0605), (0.0, -0.0605), (0.0, -0.0395), (-0.012, -0.0395), (-0.012, -0.0327), (-0.03, -0.0327)]), region="valve", material="brass"),
        shape("connection-block", "detail", 4, True, 0.3, poly([(-0.065, -0.032), (-0.03, -0.032), (-0.03, -0.068), (-0.0375, -0.068), (-0.0375, -0.08), (-0.0575, -0.08), (-0.0575, -0.068), (-0.065, -0.068)]), region="valve", material="brass"),
        shape("connection-nut-lower", "detail", 3, True, 0.3, poly([(-0.08, -0.0656), (-0.064, -0.0656), (-0.064, -0.0344), (-0.08, -0.0344)]), region="valve", material="brass"),
        shape("supply-pipe", "detail", 3, True, 0.3, poly([(-0.11, -0.0575), (-0.08, -0.0575), (-0.08, -0.0425), (-0.11, -0.0425)]), region="valve", material="brass"),
    ]
    fv = byid["front"]
    third = K.PANEL_H / 3
    seams = [(-0.5 + k / K.CHANNELS, Y0 + 0.04, -0.5 + k / K.CHANNELS, Y0 + 0.545) for k in range(1, K.CHANNELS)] + [(-0.5, Y0 + 0.04, 0.5, Y0 + 0.04), (-0.5, Y0 + 0.545, 0.5, Y0 + 0.545)]
    front = [
        dict(fv["front-lower"], **rect(-0.5, Y0, 0.5, Y0 + third)),
        dict(fv["front-mid"], **rect(-0.5, Y0 + third, 0.5, Y0 + 2 * third)),
        dict(fv["front-upper"], **rect(-0.5, Y0 + 2 * third, 0.5, Y0 + K.PANEL_H)),
        dict(fv["grille-strip"], **rect(-0.5015, Y0 + 0.575, 0.5015, Y0 + K.TOP)),
        fv["radiator-outline"],
        shape("panel-channels", "texture", 7, False, 0.5, path(seams)),
        shape("plug-left-lower", "detail", 3, True, 0.3, rect(0.5, Y0 + K.HUB[0] - K.PLUG_R, 0.5 + K.PLUG_OUT, Y0 + K.HUB[0] + K.PLUG_R), region="panel-front-lower", material="brass"),
        shape("plug-left-upper", "detail", 3, True, 0.3, rect(0.5, Y0 + K.HUB[1] - K.PLUG_R, 0.5 + K.PLUG_OUT, Y0 + K.HUB[1] + K.PLUG_R), region="panel-front-upper", material="brass"),
        shape("plug-right-lower", "detail", 3, True, 0.3, rect(-0.5 - K.PLUG_OUT, Y0 + K.HUB[0] - K.PLUG_R, -0.5, Y0 + K.HUB[0] + K.PLUG_R), region="panel-front-lower", material="brass"),
        shape("plug-right-upper", "detail", 3, True, 0.3, rect(-0.5 - K.PLUG_OUT, Y0 + K.HUB[1] - K.PLUG_R, -0.5, Y0 + K.HUB[1] + K.PLUG_R), region="panel-front-upper", material="brass"),
        dict(fv["pipe-left"], **rect(-0.4725, 0.0, -0.4575, Y0 - 0.08)),
        dict(fv["pipe-right"], **rect(-0.4225, 0.0, -0.4075, Y0 - 0.08)),
        shape("connection-nut-lower-outer", "detail", 4, True, 0.3, rect(-0.465 - 0.0135, Y0 - 0.08, -0.465 + 0.0135, Y0 - 0.064), region="valve", material="brass"),
        shape("connection-nut-lower-inner", "detail", 4, True, 0.3, rect(-0.415 - 0.0135, Y0 - 0.08, -0.415 + 0.0135, Y0 - 0.064), region="valve", material="brass"),
        shape("connection-block", "detail", 4, True, 0.3, rect(-0.49, Y0 - 0.065, -0.39, Y0 - 0.03, 0.004), region="valve", material="brass"),
        shape("connection-nut-upper-outer", "detail", 4, True, 0.3, poly([(-0.48, Y0 - 0.03), (-0.45, Y0 - 0.03), (-0.45, Y0 - 0.012), (-0.4545, Y0 - 0.012), (-0.4545, Y0), (-0.4755, Y0), (-0.4755, Y0 - 0.012), (-0.48, Y0 - 0.012)]), region="valve", material="brass"),
        shape("connection-nut-upper-inner", "detail", 4, True, 0.3, poly([(-0.43, Y0 - 0.03), (-0.4, Y0 - 0.03), (-0.4, Y0 - 0.012), (-0.4045, Y0 - 0.012), (-0.4045, Y0), (-0.4255, Y0), (-0.4255, Y0 - 0.012), (-0.43, Y0 - 0.012)]), region="valve", material="brass"),
        shape("spindle-cap-outer", "detail", 5, True, 0.35, circle(-0.465, Y0 - 0.0475, 0.01), region="valve", material="brass"),
        shape("spindle-cap-inner", "detail", 5, True, 0.35, circle(-0.415, Y0 - 0.0475, 0.01), region="valve", material="brass"),
        dict(fv["valve-head"], **{"material": "abs-plastic"}, **poly(head_outline(0, Y0 + K.HEAD_U))),
        dict(fv["valve-index"], points=pts([(-0.56, Y0 + K.HEAD_U + 0.012), (-0.56, Y0 + K.HEAD_U + K.HEAD_R)])),
    ]
    for s in front:
        if s["id"] == "valve-head":
            s.pop("r", None)
            for k in ("x", "y", "w", "h"):
                s.pop(k, None)
        if s["id"] in ("pipe-left", "pipe-right"):
            s.pop("r", None)
    pv = byid["plan"]
    plan = [
        dict(pv["top-grille"], **rect(K.WALL - 0.0015, -0.5015, K.WALL + 0.1015, 0.5015)),
        dict(pv["grille-slot-0"], **slots_plan(0.05)),
        dict(pv["grille-slot-1"], **slots_plan(0.075)),
        dict(pv["grille-slot-2"], **slots_plan(0.095)),
        dict(pv["grille-slot-3"], **slots_plan(0.12)),
        pv["bracket-left"], pv["bracket-right"],
        shape("plug-left-rear", "detail", 3, True, 0.3, rect(K.WALL, 0.5, K.WALL + 2 * K.PLUG_R, 0.5 + K.PLUG_OUT), region="panel-rear", material="brass"),
        shape("plug-left-front", "detail", 3, True, 0.3, rect(K.WALL + 0.1 - 2 * K.PLUG_R, 0.5, K.WALL + 0.1, 0.5 + K.PLUG_OUT), region="panel-front-upper", material="brass"),
        shape("plug-right-rear", "detail", 3, True, 0.3, rect(K.WALL, -0.5 - K.PLUG_OUT, K.WALL + 2 * K.PLUG_R, -0.5), region="panel-rear", material="brass"),
        shape("plug-right-front", "detail", 3, True, 0.3, rect(K.WALL + 0.1 - 2 * K.PLUG_R, -0.5 - K.PLUG_OUT, K.WALL + 0.1, -0.5), region="panel-front-upper", material="brass"),
        shape("valve-head", "detail", 5, True, 0.3, poly(plan_head()), region="valve", material="abs-plastic"),
    ]
    for s in plan:
        if s["id"].startswith("grille-slot"):
            s.pop("points", None)
            s.pop("closed", None)
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing100(old):
    """🖊️ 1:100: one panel body with the split line, grille bar, bracket arms, head, H-block and pipe."""
    byid = {v: {s["id"]: s for s in old[v]} for v in old}
    ev = byid["elevation"]
    elevation = [ev["floor"], ev["radiator-body"], ev["panel-split"], ev["top-grille"], ev["bracket-upper"], ev["bracket-lower"],
                 dict(ev["valve-head"], rx=K.HEAD_R, ry=K.HEAD_R, material="abs-plastic"),
                 dict(ev["supply-pipe"], points=pts([(K.FLOOR_U, K.PORT_V), (-0.06, K.PORT_V)])),
                 shape("connection-block", "detail", 4, True, 0.3, poly([(-0.06, -0.032), (0.0, -0.032), (0.0, -0.068), (-0.06, -0.068)]), region="valve", material="brass")]
    fv = byid["front"]
    third = 0.57 / 3
    front = [
        dict(fv["front-lower"], **rect(-0.5, Y0, 0.5, Y0 + third)),
        dict(fv["front-mid"], **rect(-0.5, Y0 + third, 0.5, Y0 + 2 * third)),
        dict(fv["front-upper"], **rect(-0.5, Y0 + 2 * third, 0.5, Y0 + 0.57)),
        dict(fv["grille-strip"], **rect(-0.5, Y0 + 0.57, 0.5, Y0 + K.TOP)),
        fv["radiator-outline"],
        dict(fv["pipe-left"], **rect(-0.4725, 0.0, -0.4575, Y0 - 0.06)),
        dict(fv["pipe-right"], **rect(-0.4225, 0.0, -0.4075, Y0 - 0.06)),
        shape("connection-block", "detail", 4, True, 0.3, rect(-0.49, Y0 - 0.06, -0.39, Y0), region="valve", material="brass"),
        shape("valve-head", "detail", 5, True, 0.3, poly([(-0.497, Y0 + K.HEAD_U + 0.018), (-0.514, Y0 + K.HEAD_U + 0.018), (-0.514, Y0 + K.HEAD_U + K.HEAD_R), (-0.595, Y0 + K.HEAD_U + K.HEAD_R), (-0.595, Y0 + K.HEAD_U - K.HEAD_R), (-0.514, Y0 + K.HEAD_U - K.HEAD_R), (-0.514, Y0 + K.HEAD_U - 0.018), (-0.497, Y0 + K.HEAD_U - 0.018)]), region="valve", material="abs-plastic"),
    ]
    pv = byid["plan"]
    cx = K.WALL - K.HEAD_V
    plan = [pv["top-grille"], pv["bracket-left"], pv["bracket-right"], pv["convector-edge"],
            shape("valve-head", "detail", 5, True, 0.3, poly([(cx - 0.018, -0.497), (cx - 0.018, -0.514), (cx - K.HEAD_R, -0.514), (cx - K.HEAD_R, -0.595), (cx + K.HEAD_R, -0.595), (cx + K.HEAD_R, -0.514), (cx + 0.018, -0.514), (cx + 0.018, -0.497)]), region="valve", material="abs-plastic")]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing200(old):
    """🖊️ 1:200: one filled panel mass, bracket strokes, head, H-block and one pipe column."""
    byid = {v: {s["id"]: s for s in old[v]} for v in old}
    ev = byid["elevation"]
    elevation = [ev["floor"], ev["radiator-body"], ev["bracket-upper"], ev["bracket-lower"],
                 dict(ev["supply-pipe"], points=pts([(K.FLOOR_U, K.PORT_V), (-0.06, K.PORT_V)])),
                 shape("connection-block", "detail", 4, True, 0.3, poly([(-0.06, -0.032), (0.0, -0.032), (0.0, -0.068), (-0.06, -0.068)]), region="valve", material="brass"),
                 dict(ev["valve-head"], rx=K.HEAD_R, ry=K.HEAD_R, material="abs-plastic")]
    fv = byid["front"]
    front = [
        dict(fv["front-lower"], **rect(-0.5, Y0, 0.5, Y0 + 0.2)),
        dict(fv["front-mid"], **rect(-0.5, Y0 + 0.2, 0.5, Y0 + 0.4)),
        dict(fv["front-upper"], **rect(-0.5, Y0 + 0.4, 0.5, Y0 + K.TOP)),
        fv["radiator-outline"],
        dict({k: v for k, v in fv["valve-left"].items() if k not in ("x", "y", "w", "h")}, id="valve-head", material="abs-plastic", **rect(-0.595, Y0 + K.HEAD_U - K.HEAD_R, -0.5, Y0 + K.HEAD_U + K.HEAD_R)),
        dict({k: v for k, v in fv["valve-right"].items() if k not in ("x", "y", "w", "h")}, id="connection-block", **rect(-0.49, Y0 - 0.06, -0.39, Y0)),
        dict(fv["pipe-left"], id="pipes", **rect(-0.4725, 0.0, -0.4075, Y0 - 0.06)),
    ]
    pv = byid["plan"]
    cx = K.WALL - K.HEAD_V
    plan = [pv["radiator-top"], pv["bracket-left"], pv["bracket-right"],
            shape("valve-head", "detail", 4, True, 0.3, rect(cx - K.HEAD_R, -0.595, cx + K.HEAD_R, -0.5), region="valve", material="abs-plastic")]
    return {"elevation": elevation, "front": front, "plan": plan}


TEXTS = {
    "50": {
        "silhouette": text("The vertical section A-A through the middle of a type 22 valve-compact radiator (panel, convector, convector, panel) hung on its upper and lower wall brackets 0.035 m off the wall of the scene; beyond the cut the side cover of the valve end, the thermostat head end-on with its index mark, and below the left end the H-block with unions, isolating cap and pipe. Front elevation (1.0 m panels, 0.6 m high, 0.11 m above the floor): the front panel in three flow bands with its 30 vertical channels and the two headers drawn as texture, the grille lip at the top, the plugs at both ends, the thermostat head on the left end at the top header, and under the left end the H-block with its two caps, the hex unions and the two pipes to the floor. Plan view: the grille rim (103 x 1003 mm) with four rows of eight slots, the two bracket arms in the wall gap, the plugs at both ends and the thermostat head beyond the left end.", "Der Vertikalschnitt A-A durch die Mitte eines Ventilkompaktheizkörpers Typ 22 (Platte, Konvektor, Konvektor, Platte) an seinen oberen und unteren Wandkonsolen 0,035 m vor der Wand der Szene; hinter dem Schnitt die Seitenblende am Ventilende, der Thermostatkopf von vorn mit seiner Marke und unter dem linken Ende der Hahnblock mit Verschraubungen, Absperrkappe und Rohr. Vorderansicht (Platten 1,0 m, 0,6 m hoch, 0,11 m über dem Boden): die Frontplatte in drei Strömungsbändern mit ihren 30 senkrechten Kanälen und den beiden Sammelkanälen als Textur, die Gitterlippe oben, die Stopfen an beiden Enden, der Thermostatkopf am linken Ende auf Höhe des oberen Sammelkanals und unter dem linken Ende der Hahnblock mit seinen zwei Kappen, den Sechskantverschraubungen und den zwei Rohren zum Boden. Draufsicht: der Gitterrahmen (103 x 1003 mm) mit vier Reihen zu acht Schlitzen, die beiden Konsolarme im Wandabstand, die Stopfen an beiden Enden und der Thermostatkopf jenseits des linken Endes."),
        "retained": text("Both water panels (22 mm), both convector fin packs, side cover, grille with lips and slots, upper and lower brackets as L sections, thermostat head with index mark, H-block with unions, cap and pipe; front channels, plugs.", "Beide Wasserplatten (22 mm), beide Konvektorlamellenpakete, Seitenblende, Gitter mit Lippen und Schlitzen, obere und untere Konsolen als L-Profile, Thermostatkopf mit Marke, Hahnblock mit Verschraubungen, Kappe und Rohr; Frontkanäle, Stopfen."),
        "merged": text("Both convectors share the zigzag vertices at the centre plane. Front: the two pipes and their unions are drawn per pipe, the H-block with its unions as one outline; the head and its collar are one outline. Section: the two pipes lie behind each other and are one shape.", "Beide Konvektoren teilen die Zickzack-Knoten in der Mittelebene. Vorderansicht: die beiden Rohre und ihre Verschraubungen sind je Rohr gezeichnet, der Hahnblock mit seinen Verschraubungen als eine Kontur; Kopf und Überwurf sind eine Kontur. Schnitt: die beiden Rohre liegen hintereinander und sind eine Form."),
        "omitted": text("Section: the right side cover and plugs (behind the retained half). Front: rear panel, fins and brackets (behind the front panel). Plan: the H-block and pipes (below the panels).", "Schnitt: die rechte Seitenblende und Stopfen (hinter der behaltenen Hälfte). Vorderansicht: Rückplatte, Lamellen und Konsolen (hinter der Frontplatte). Draufsicht: Hahnblock und Rohre (unter den Platten)."),
    },
    "100": {
        "silhouette": text("The radiator as one panel body with a split line between the panel pair, the 30 mm grille bar, the bracket arms, the thermostat head end-on, the H-block and the pipe. Front elevation: front panel in three bands over its outline, the grille strip, the thermostat head on the left end at the top, the H-block and the two pipes under the left end. Plan view: the grille plate (1.0 x 0.1 m) with a centre split line, the two bracket arms in the wall gap and the thermostat head beyond the left end.", "Der Heizkörper als ein Paneelkörper mit Teilungslinie zwischen dem Plattenpaar, die 30-mm-Gitterleiste, die Konsolarme, der Thermostatkopf von vorn, der Hahnblock und das Rohr. Vorderansicht: Frontplatte in drei Bändern über ihrer Kontur, der Gitterstreifen, der Thermostatkopf am linken Ende oben, Hahnblock und die zwei Rohre unter dem linken Ende. Draufsicht: die Gitterplatte (1,0 x 0,1 m) mit einer Mittelteilungslinie, die beiden Konsolarme im Wandabstand und der Thermostatkopf jenseits des linken Endes."),
        "retained": text("Panel body, centre split, grille, two bracket lines, thermostat head, H-block, pipes.", "Paneelkörper, Mittelteilung, Gitter, zwei Konsollinien, Thermostatkopf, Hahnblock, Rohre."),
        "merged": text("Rear panel, convectors and front panel are one body; the two 0.022 m water panels are too thin (0.22 mm) to print as separate fills. The H-block takes in its unions and caps; the head and its collar are one outline.", "Rückplatte, Konvektoren und Frontplatte sind ein Körper; die beiden 0,022-m-Wasserplatten sind zu dünn (0,22 mm), um als getrennte Flächen gedruckt zu werden. Der Hahnblock schließt Verschraubungen und Kappen ein; Kopf und Überwurf sind eine Kontur."),
        "omitted": text("Fin corrugation, grille slots, front channels, plugs. Front: rear panel, brackets. Plan: grille slots, H-block and pipes (below the panel).", "Lamellenwellung, Gitterschlitze, Frontkanäle, Stopfen. Vorderansicht: Rückplatte, Konsolen. Draufsicht: Gitterschlitze, Hahnblock und Rohre (unter dem Paneel)."),
        "thinElements": text("Bracket arms (15 mm) are drawn as 0.13 mm lines; the pipe (15 mm) as a 0.13 mm line in the section and at its true width in the front.", "Konsolarme (15 mm) sind als 0,13-mm-Linien gezeichnet; das Rohr (15 mm) im Schnitt als 0,13-mm-Linie und in der Vorderansicht in wahrer Breite."),
    },
    "200": {
        "silhouette": text("The radiator as one filled panel mass, the thermostat head end-on beside its top and the H-block with the pipe under it. Front elevation: three plate bands, the thermostat head on the left end at the top, the H-block and the pipe column under the left end. Plan view: the panel seen from above as one strip (0.5 x 5 mm) with the two brackets as lines in the wall gap and the head beyond the left end.", "Der Heizkörper als eine gefüllte Paneelmasse, der Thermostatkopf von vorn neben seiner Oberkante und darunter der Hahnblock mit dem Rohr. Vorderansicht: drei Plattenbänder, der Thermostatkopf am linken Ende oben, Hahnblock und Rohrsäule unter dem linken Ende. Draufsicht: das Paneel von oben als ein Streifen (0,5 x 5 mm) mit den beiden Konsolen als Linien im Wandabstand und dem Kopf jenseits des linken Endes."),
        "retained": text("Radiator 0.10 m deep and 0.60 m high at 0.11 m above the floor, thermostat head (0.052 m), H-block.", "Heizkörper 0,10 m tief und 0,60 m hoch, 0,11 m über dem Boden, Thermostatkopf (0,052 m), Hahnblock."),
        "merged": text("Rear panel, both convectors, front panel, top grille and the two brackets are one filled mass listing all their regions; the 0.035 m wall clearance closes up because its ink gap is below the minimum. Front: the grille lip is in the upper band; the two pipes are one column, the H-block takes in its unions.", "Rückseitiges Paneel, beide Konvektoren, vorderes Paneel, Abdeckgitter und die beiden Konsolen sind eine gefüllte Masse mit allen Regionen; der Wandabstand von 0,035 m schließt sich, weil seine Tintenlücke unter dem Minimum liegt. Vorderansicht: die Gitterlippe liegt im oberen Band; die beiden Rohre sind eine Säule, der Hahnblock schließt die Verschraubungen ein."),
        "omitted": text("Fin corrugation, grille slots and lip, valve index line, plugs. Front: channels, rear panel, brackets. Plan: grille slots, H-block and pipes (hidden below the panel).", "Lamellenwellung, Gitterschlitze und -lippe, Ventilmarke, Stopfen. Vorderansicht: Kanäle, Rückplatte, Konsolen. Draufsicht: Gitterschlitze, Hahnblock und Rohre (unter dem Paneel verdeckt)."),
        "overlays": text("Thermal cells: radiator mass (area-weighted merge of its five regions), valve.", "Thermische Zellen: Heizkörpermasse (flächengewichtete Zusammenfassung ihrer fünf Regionen), Ventil."),
    },
}


def anchor(aid, x, y, z, title, support, **extra):
    """⚓ A wall fixing on the radiator bone."""
    a = {"id": aid, "bone": "radiator", "x": x, "y": y, "z": z, "kind": "support", "title": title, "support": support}
    a.update(extra)
    return a


def apply(asset):
    """🌱 Every root change of this round in one place."""
    asset["summary"] = text(
        "A compact type 22 steel valve radiator (Ventilkompakt: two pressed water panels with vertical channels, two convector fin packs, side covers and a slotted top grille; panels 600 mm high, 1000 mm long, 100 mm deep, rated 1.2 kW at 75/65/20 degC) drawn in vertical section (viewpoint elevation) and in front elevation (viewpoint front), hung on four wall brackets 35 mm off the wall of the scene, with the thermostat head on its built-in valve at the left end and the bottom connection block with two pipes to the floor; output scales with the EN 442 exponent n = 1.3 and rises as a convection plume.",
        "Ein kompakter Stahl-Ventilheizkörper Typ 22 (Ventilkompakt: zwei gepresste Wasserplatten mit senkrechten Kanälen, zwei Konvektorpakete, Seitenblenden und ein geschlitztes Abdeckgitter; Platten 600 mm hoch, 1000 mm lang, 100 mm tief, 1,2 kW bei 75/65/20 °C) im Vertikalschnitt (Ansicht elevation) und in der Vorderansicht (Ansicht front), an vier Wandkonsolen 35 mm vor der Wand der Szene, mit dem Thermostatkopf auf dem eingebauten Ventil am linken Ende und dem Hahnblock mit zwei Rohren zum Boden; die Leistung folgt dem EN-442-Exponenten n = 1,3 und steigt als Konvektionsfahne auf.")
    refs = asset["provenance"]["references"]
    for r in refs:
        if r["id"] == "radiator-catalogues":
            r["citation"] = "Manufacturer catalogues for compact panel radiators type 22, height 600 mm, length 1000 mm, depth 100 to 102 mm: pressed panels of two 1.25 mm sheets with vertical water channels at about 33 mm pitch, welded trapezoidal convector fins, clip-on side covers and top grille, connection hub distance = height - 50 mm. Rated output about 1.6 to 1.7 kW at 75/65/20 degC for the 600 x 1000 type 22 (the asset keeps its calibrated 1.2 kW, see the thermal assumptions); water content about 6.6 l, empty mass about 29 kg."
    known = {r["id"] for r in refs}
    for r in [
        {"id": "purmo-ventil-compact", "citation": "Purmo Ventil Compact datasheet (Rettig Heating), dimension drawings: type 22 depth 102 mm, 25 mm wall distance on the brackets, hub distance B - 50 mm, two G 1/2 bottom connections at 50 mm centres in the middle of the depth (76 mm from the wall) plus four G 1/2 side tappings, built-in thermostatic valve insert with M30 x 1.5 head connection.", "url": "https://www.heizungsdiscount24.de/pdf/Purmo-Ventil-Compact-Datenblatt.pdf"},
        {"id": "purmo-c22-600-1000", "citation": "Distributor listing of the Purmo Compact C22 600 x 1000 radiator: heat emission 1709 W at 75/65/20 degC after EN 442, water content 6.6 l.", "url": "https://onninen.fi/en/purmo-radiator-purmo-compact-c22-600-1000/p/AAO853"},
        {"id": "kermi-profil-k-22", "citation": "Retailer listing of a Kermi therm-x2 Profil-K type 22, 400 x 600 mm: 724 W at 75/65/20 degC, water content 2.7 l, mass 12.52 kg; scaled by the panel area to 600 x 1000 mm this gives about 31 kg empty and 6.8 l of water."},
        {"id": "thermostat-heads", "citation": "Thermostat heads with M30 x 1.5 connection for valve radiators (e.g. Heimeier K, Danfoss RAW): white plastic body about 50 to 55 mm across and 80 to 95 mm long with a turning dial and set-point marks; isolating and connection H-blocks for the bottom connections of valve radiators (e.g. Heimeier Vekolux, Oventrop Multiflex) are nickel-plated brass with two isolating spindles under caps and union nuts for 15 mm pipes."},
    ]:
        if r["id"] not in known:
            refs.append(r)
    asset["provenance"]["notes"] = text(
        "Output per level uses Phi = Phi_50 (dT/50)^n with n = 1.3; the 1.2 kW rating and the convective share are authored estimates (catalogue values for this size are about 1.7 kW). The construction (panels, channels, fins, covers, grille, brackets, valve-compact connection) and the mass follow the cited catalogues.",
        "Die Leistung je Stufe folgt Phi = Phi_50 (dT/50)^n mit n = 1,3; die Nennleistung 1,2 kW und der Konvektionsanteil sind Schätzwerte (Katalogwerte für diese Größe liegen bei etwa 1,7 kW). Die Konstruktion (Platten, Kanäle, Lamellen, Blenden, Gitter, Konsolen, Ventilkompakt-Anschluss) und die Masse folgen den zitierten Katalogen.")
    asset["dimensions"]["depthM"] = r5(0.5 + K.PLUG_OUT - K.HEAD_W[-1])
    asset["dimensions"]["massKg"] = r5(RADIATOR_KG + HEAD_KG)
    if not any(m["id"] == "abs-plastic" for m in asset["materials"]):
        asset["materials"].append({"id": "abs-plastic", "title": text("White ABS plastic (thermostat head)", "Weißer ABS-Kunststoff (Thermostatkopf)"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "en-iso-10456"})
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    bones["radiator"]["title"] = text("Type 22 valve-compact radiator on its four wall brackets (fixed root)", "Ventilkompaktheizkörper Typ 22 an seinen vier Wandkonsolen (feste Wurzel)")
    bones["radiator"]["mass"]["kg"] = RADIATOR_KG
    bones["valve-head"]["x"], bones["valve-head"]["y"] = K.HEAD_U, K.HEAD_V
    bones["valve-head"]["title"] = text("Thermostatic head (dial) on the built-in valve at the left end", "Thermostatkopf (Einstellrad) auf dem eingebauten Ventil am linken Ende")
    anchors = []
    for a in asset["anchors"]:
        if a["id"] == "bracket-top":
            anchors += [
                anchor("hook-left", 0.465, K.WALL, 0.4, text("Left upper bracket: wall screw, the radiator hangs on its hook", "Linke obere Konsole: Wandschraube, der Heizkörper hängt an ihrem Haken"), "pin"),
                anchor("hook-right", 0.465, K.WALL, -0.4, text("Right upper bracket: wall screw, the radiator hangs on its hook", "Rechte obere Konsole: Wandschraube, der Heizkörper hängt an ihrem Haken"), "pin"),
                anchor("rest-left", 0.05, K.WALL, 0.4, text("Left lower holder: flange against the wall, horizontal reaction", "Linker unterer Halter: Flansch an der Wand, waagrechte Reaktion"), "roller", normalDeg=0),
                anchor("rest-right", 0.05, K.WALL, -0.4, text("Right lower holder: flange against the wall, horizontal reaction", "Rechter unterer Halter: Flansch an der Wand, waagrechte Reaktion"), "roller", normalDeg=0),
            ]
            continue
        if a["id"] == "front-top-edge":
            a["y"] = -0.1
        if a["id"] == "grille-centre":
            a["y"] = -0.05
        anchors.append(a)
    asset["anchors"] = anchors
    reps = asset["representations"]
    before = json.load(open(os.path.join(T, "r3-panel-radiator-2d-before.json"), encoding="utf-8"))
    for scale, fn in (("50", drawing50), ("100", drawing100), ("200", drawing200)):
        reps[scale]["geometry"] = fn(before[scale])
        reps[scale]["decisions"].update(TEXTS[scale])
    th = asset["thermal"]
    for r in th["regions"]:
        if r["id"] == "valve":
            r["title"] = text("Thermostat head, connection block and pipes", "Thermostatkopf, Hahnblock und Anschlussrohre")
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]
    th["assumptions"][3] = text(
        "The front panel is split into three thermal bands along the flow: in the valve-compact radiator the supply enters through the built-in valve at the top of the left end, so the water of the upper, middle and lower band is 1/6, 1/2 and 5/6 of the way from the supply to the return temperature (about 71, 68 and 65 degC at the rated level, the 10 K spread of the circuit); the valve region (thermostat head, H-block and pipes) sits at the supply side behind a brass wall and film of 0.01 m2K/W and stays below the supply temperature; the front elevation (viewpoint front) draws these bands, the head and the connection block, while the section (viewpoint elevation) draws the same regions in profile.",
        "Die Frontplatte ist in drei thermische Bänder entlang der Strömung geteilt: im Ventilkompaktheizkörper tritt der Vorlauf durch das eingebaute Ventil oben am linken Ende ein, daher liegt das Wasser des oberen, mittleren und unteren Bands bei 1/6, 1/2 und 5/6 des Wegs von der Vorlauf- zur Rücklauftemperatur (etwa 71, 68 und 65 °C bei Nennleistung, die 10-K-Spreizung des Kreises); die Ventilregion (Thermostatkopf, Hahnblock und Rohre) sitzt auf der Vorlaufseite hinter einer Messingwand und einem Film von 0,01 m2K/W und bleibt unter der Vorlauftemperatur; die Vorderansicht (Ansicht front) zeichnet diese Bänder, den Kopf und den Hahnblock, der Schnitt (Ansicht elevation) dieselben Regionen im Profil.")
    st = asset["structure"]
    st["supports"] = ["hook-left", "hook-right", "rest-left", "rest-right"]
    st["assumptions"][0] = text(
        "The wall is not part of the asset: the radiator hangs with the welded lugs of its rear panel on the hook noses of the two upper brackets, whose wall screws are pins, and is held off the wall by the two lower holders, whose flanges bear on the wall as rollers with a horizontal reaction; the radiator is one rigid part. The asset mass of %s kg is the radiator with its water %s kg (about 29 kg of steel and 6.6 l of water) and the thermostat head %s kg." % (str(r5(RADIATOR_KG + HEAD_KG)), str(RADIATOR_KG), str(HEAD_KG)),
        "Die Wand gehört nicht zum Asset: der Heizkörper hängt mit den angeschweißten Laschen seiner Rückplatte an den Hakennasen der beiden oberen Konsolen, deren Wandschrauben Festlager sind, und wird von den beiden unteren Haltern auf Abstand gehalten, deren Flansche als Rollenlager mit waagrechter Reaktion an der Wand anliegen; der Heizkörper ist ein starrer Teil. Die Assetmasse von %s kg besteht aus dem Heizkörper mit Wasser %s kg (etwa 29 kg Stahl und 6,6 l Wasser) und dem Thermostatkopf %s kg." % (str(r5(RADIATOR_KG + HEAD_KG)).replace(".", ","), str(RADIATOR_KG).replace(".", ","), str(HEAD_KG).replace(".", ",")))
    asset["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    return asset


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    if source[index] == '"':
        end = index + 1
        while source[end] != '"':
            end += 2 if source[end] == "\\" else 1
        return index, end + 1
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = re.sub(r"(\d)e-0(\d)(?=,?$)", r"\1e-\2", json.dumps(value, indent=2, ensure_ascii=False), flags=re.M).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            if key != "volumes" and dumped(before["asset"][key]) != source[start:end]:
                raise SystemExit("member %s is not in the plain layout, not written" % key)
            source = source[:start] + dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        folder = os.path.join(T, "🗑️generated", "r3-panel-radiator")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written: depthM", document["asset"]["dimensions"]["depthM"], "massKg", document["asset"]["dimensions"]["massKg"])
