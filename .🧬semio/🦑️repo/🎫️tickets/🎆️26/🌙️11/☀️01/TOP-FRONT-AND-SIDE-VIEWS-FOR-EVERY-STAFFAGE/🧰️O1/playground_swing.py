import sys, os, copy
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "playground-swing"
GALV = dict(region="frame-galv", material="steel-galvanised")
CHAIN = dict(region="chain-steel", material="steel-galvanised")
SEAT = dict(region="seat-rubber", material="rubber")
FRAME_Y = 0.56
SEAT_Z = 0.26
CHAIN_Z = 0.23
HANGER_Y = 2.3


def ext(id, bone, z0, z1, profile, z=3, tone=0.3, role="body", filled=True, **reg):
    s = dict(profile)
    s.update({"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone})
    s.update(reg)
    s["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
    return s


def prof_rect(x, y, w, h):
    return {"kind": "rect", "x": x, "y": y, "w": w, "h": h}


def prof_cap(x0, x1, r):
    return {"kind": "capsule", "x0": x0, "y0": 0, "x1": x1, "y1": 0, "r0": r, "r1": r}


def prof_path(d):
    return {"kind": "path", "d": d}


def seat_prof(scale):
    return prof_path(orig_shape(A, scale, "elevation", "seat-plank")["d"])


def chains(scale, r, w, tone=0.3, z=2.5, **kw):
    return [ext("chain-left", "chain-left", CHAIN_Z - w / 2, CHAIN_Z + w / 2, prof_cap(0.05, 1.9, r), z=z, tone=tone, **CHAIN, **kw),
            ext("chain-right", "chain-right", -CHAIN_Z - w / 2, -CHAIN_Z + w / 2, prof_cap(0.05, 1.9, r), z=z, tone=tone, **CHAIN, **kw)]


def seat(scale, **kw):
    return ext("seat", "seat", -SEAT_Z, SEAT_Z, seat_prof(scale), z=4, tone=0.25, **SEAT, **kw)


def stripes(w, y0, y1, z=2, tone=0.4, **kw):
    return [rect("frame-left", "leg-left", FRAME_Y - w, y0, 2 * w, y1 - y0, z=z, tone=tone, **GALV, **kw),
            rect("frame-right", "leg-right", -FRAME_Y - w, y0, 2 * w, y1 - y0, z=z, tone=tone, **GALV, **kw)]


def leg_strips(w, z=2, tone=0.4, **kw):
    out = []
    for s, nm in ((1, "left"), (-1, "right")):
        out.append(rect(f"frame-{nm}-leg-left", "leg-left", 0, s * FRAME_Y - w, 1.3, 2 * w, z=z, tone=tone, **GALV, **kw))
        out.append(rect(f"frame-{nm}-leg-right", "leg-right", 1.3, s * FRAME_Y - w, 1.3, 2 * w, z=z, tone=tone, **GALV, **kw))
    return out


def plates(h=0.012, z=2.3, **kw):
    out = []
    for s, nm in ((1, "left"), (-1, "right")):
        out.append(rect(f"foot-{nm}-front-plate", "ground", -0.09, s * FRAME_Y - 0.06, 0.18, 0.12, z=z, tone=0.25, **GALV, **kw))
        out.append(rect(f"foot-{nm}-back-plate", "ground", 2.51, s * FRAME_Y - 0.06, 0.18, 0.12, z=z, tone=0.25, **GALV, **kw))
    return out


def plates_front(z=2.3, **kw):
    return [rect("foot-left-plate", "ground", FRAME_Y - 0.06, 0, 0.12, 0.012, z=z, tone=0.25, **GALV, **kw),
            rect("foot-right-plate", "ground", -FRAME_Y - 0.06, 0, 0.12, 0.012, z=z, tone=0.25, **GALV, **kw)]


def beam_front(h=0.05, z=3, half=0.6, **kw):
    return rect("beam", "beam", -half, HANGER_Y - h, 2 * half, 2 * h, z=z, tone=0.38, **GALV, **kw)


def beam_plan(w=0.05, z=3, half=0.6, **kw):
    return rect("beam", "beam", 1.3 - w, -half, 2 * w, 2 * half, z=z, tone=0.38, **GALV, **kw)


views = {}

views[500] = (
    stripes(0.03, 0, 2.3) + [beam_front(0.05, half=0.62)] + chains(500, 0.0055, 0.03) + [seat(500)],
    leg_strips(0.03) + plates() + [beam_plan(0.05, half=0.62)] + chains(500, 0.0055, 0.03) + [seat(500)],
    {"silhouette": ("Front: the two A-frames seen edge-on as stripes, the beam between them, both chains and the seat as solids of the side view: they stretch and tilt with every swing; plan: the A-frame legs as strips, the beam, the two chains as strips that swing forward and back and the seat.",
                    "Vorderansicht: die beiden A-Böcke von der Seite als Streifen, der Holm dazwischen, beide Ketten und die Sitzfläche als Körper der Seitenansicht: sie strecken und neigen sich bei jedem Schwung; Draufsicht: die Bockbeine als Streifen, der Holm, die beiden Ketten als Streifen, die vor- und zurückschwingen, und die Sitzfläche."),
     "omitted": ("Foot plates, brace and chain links are merged into the strips in front; the brace is hidden under the legs in plan.",
                 "Fußplatten, Strebe und Kettenglieder gehen in der Vorderansicht in den Streifen auf; die Strebe liegt in der Draufsicht unter den Beinen.")})

views[200] = (
    stripes(0.03, 0, 2.3) + plates_front() + [beam_front(0.05)] + chains(200, 0.0055, 0.04, penMm=0.146) + [seat(200, penMm=0.266)],
    leg_strips(0.03) + plates() + [beam_plan(0.05)] + chains(200, 0.0055, 0.04, penMm=0.146) + [seat(200, penMm=0.266)],
    {"silhouette": ("Front: the A-frames edge-on with foot plates, the beam, two chains and the seat; plan: the leg strips with foot plates, the beam, two swinging chain strips and the seat.",
                    "Vorderansicht: die A-Böcke von der Seite mit Fußplatten, der Holm, zwei Ketten und die Sitzfläche; Draufsicht: die Beinstreifen mit Fußplatten, der Holm, zwei schwingende Kettenstreifen und die Sitzfläche."),
     "retained": ("Chains and seat are solids bound to their swinging bones, so front and plan follow every pendulum pose and clip.",
                  "Ketten und Sitzfläche sind an ihre schwingenden Knochen gebundene Körper, Vorder- und Draufsicht folgen daher jeder Pendelpose und jedem Clip.")})

views[100] = (
    stripes(0.03, 0, 2.3) + plates_front(r=0.002, penMm=0.25)
    + [ell("brace-left", "brace", FRAME_Y, 0.6, 0.02, 0.02, z=2.5, tone=0.45, **GALV), ell("brace-right", "brace", -FRAME_Y, 0.6, 0.02, 0.02, z=2.5, tone=0.45, **GALV),
       beam_front(0.05)]
    + chains(100, 0.0055, 0.02, penMm=0.267) + [seat(100, penMm=0.437)],
    leg_strips(0.03) + plates(r=0.002, penMm=0.25)
    + [beam_plan(0.05)] + chains(100, 0.0055, 0.02, penMm=0.267) + [seat(100, penMm=0.437)],
    {"silhouette": ("Front: the A-frames edge-on with foot plates and the brace ends, the beam, two chains and the rounded seat; plan: the leg strips with plates, the beam, the swinging chain strips and the rounded seat.",
                    "Vorderansicht: die A-Böcke von der Seite mit Fußplatten und Strebenenden, der Holm, zwei Ketten und die gerundete Sitzfläche; Draufsicht: die Beinstreifen mit Platten, der Holm, die schwingenden Kettenstreifen und die gerundete Sitzfläche.")})

views[50] = (
    stripes(0.03, 0, 2.3, penMm=0.35) + plates_front(r=0.002, penMm=0.3)
    + [poly("weld-left", "leg-left", [(FRAME_Y - 0.03, 0.244), (FRAME_Y + 0.03, 0.244)], role="detail", z=2.1, tone=0.3, closed=False, **GALV),
       poly("weld-right", "leg-right", [(-FRAME_Y - 0.03, 0.244), (-FRAME_Y + 0.03, 0.244)], role="detail", z=2.1, tone=0.3, closed=False, **GALV),
       rect("leg-left-clamp", "leg-left", FRAME_Y - 0.04, 2.0, 0.08, 0.09, z=2.2, tone=0.3, r=0.006, **GALV),
       rect("leg-right-clamp", "leg-right", -FRAME_Y - 0.04, 2.0, 0.08, 0.09, z=2.2, tone=0.3, r=0.006, **GALV),
       ell("brace-left", "brace", FRAME_Y, 0.6, 0.02, 0.02, z=2.5, tone=0.45, penMm=0.3, **GALV), ell("brace-right", "brace", -FRAME_Y, 0.6, 0.02, 0.02, z=2.5, tone=0.45, penMm=0.3, **GALV),
       rect("brace-clamp-left", "brace", FRAME_Y - 0.032, 0.57, 0.064, 0.06, z=2.6, tone=0.3, r=0.005, **GALV),
       rect("brace-clamp-right", "brace", -FRAME_Y - 0.032, 0.57, 0.064, 0.06, z=2.6, tone=0.3, r=0.005, **GALV),
       beam_front(0.05, penMm=0.35),
       rect("beam-cap-left", "beam", FRAME_Y + 0.04, HANGER_Y - 0.034, 0.02, 0.068, z=3.1, tone=0.3, **GALV),
       rect("beam-cap-right", "beam", -FRAME_Y - 0.06, HANGER_Y - 0.034, 0.02, 0.068, z=3.1, tone=0.3, **GALV),
       pth("beam-hanger-left", "beam", f"M {CHAIN_Z - 0.025} {HANGER_Y - 0.045} C {CHAIN_Z - 0.03} {HANGER_Y - 0.09} {CHAIN_Z + 0.03} {HANGER_Y - 0.09} {CHAIN_Z + 0.025} {HANGER_Y - 0.045}", z=3.2, tone=0.2, filled=False, **CHAIN),
       pth("beam-hanger-right", "beam", f"M {-CHAIN_Z - 0.025} {HANGER_Y - 0.045} C {-CHAIN_Z - 0.03} {HANGER_Y - 0.09} {-CHAIN_Z + 0.03} {HANGER_Y - 0.09} {-CHAIN_Z + 0.025} {HANGER_Y - 0.045}", z=3.2, tone=0.2, filled=False, **CHAIN)]
    + chains(50, 0.0055, 0.02, penMm=0.13)
    + [ext("shackle-left", "seat", CHAIN_Z - 0.011, CHAIN_Z + 0.011, {"kind": "ellipse", "cx": 0, "cy": -0.025, "rx": 0.011, "ry": 0.027}, z=4.5, tone=0.2, **CHAIN),
       ext("shackle-right", "seat", -CHAIN_Z - 0.011, -CHAIN_Z + 0.011, {"kind": "ellipse", "cx": 0, "cy": -0.025, "rx": 0.011, "ry": 0.027}, z=4.5, tone=0.2, **CHAIN),
       seat(50, penMm=0.35)],
    leg_strips(0.03, penMm=0.35) + plates(r=0.002, penMm=0.3)
    + [rect("clamp-left-leg", "leg-left", 1.1, FRAME_Y - 0.04, 0.1, 0.08, z=2.2, tone=0.3, r=0.006, **GALV),
       rect("clamp-left-leg-right", "leg-right", 1.4, FRAME_Y - 0.04, 0.1, 0.08, z=2.2, tone=0.3, r=0.006, **GALV),
       rect("clamp-right-leg", "leg-left", 1.1, -FRAME_Y - 0.04, 0.1, 0.08, z=2.2, tone=0.3, r=0.006, **GALV),
       rect("clamp-right-leg-right", "leg-right", 1.4, -FRAME_Y - 0.04, 0.1, 0.08, z=2.2, tone=0.3, r=0.006, **GALV),
       beam_plan(0.05, penMm=0.35),
       rect("beam-cap-left", "beam", 1.3 - 0.034, FRAME_Y + 0.04, 0.068, 0.02, z=3.1, tone=0.3, **GALV),
       rect("beam-cap-right", "beam", 1.3 - 0.034, -FRAME_Y - 0.06, 0.068, 0.02, z=3.1, tone=0.3, **GALV)]
    + chains(50, 0.0055, 0.02, penMm=0.13)
    + [seat(50, penMm=0.35)],
    {"silhouette": ("Front: the A-frames edge-on with welds, clamps, foot plates and brace ends, the beam with end caps and chain hangers, both chains with the seat shackles and the rounded seat; plan: leg strips with clamps and plates, the beam with end caps, both swinging chains and the seat.",
                    "Vorderansicht: die A-Böcke von der Seite mit Schweißnähten, Schellen, Fußplatten und Strebenenden, der Holm mit Endkappen und Kettenaufhängungen, beide Ketten mit den Sitzschäkeln und die gerundete Sitzfläche; Draufsicht: Beinstreifen mit Schellen und Platten, der Holm mit Endkappen, beide schwingenden Ketten und die Sitzfläche."),
     "omitted": ("Chain links and seat tread are not drawn in front and plan; the chains are drawn as continuous strips.",
                 "Kettenglieder und Sitzprofil werden in Vorder- und Draufsicht nicht gezeichnet; die Ketten erscheinen als durchgehende Streifen.")})

apply(A, views)

doc = load(A)
PEN = {50: 0.5, 100: 0.45, 200: 0.35, 500: 0.3}
for sc, pen in PEN.items():
    for s in doc["asset"]["representations"][str(sc)]["geometry"]["elevation"]:
        if s["bone"] == "chain-left":
            if s["id"] == "chain-left-link": s["penMm"] = pen
            else: s.pop("penMm", None)
            s["z"] = 3.9
save(A, doc)
