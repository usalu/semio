"""✏ Round 3, furniture/three-seat-sofa: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn from the same station tables as the solids.

- Elevation: the sofa seen from the front (from -z), in each shape's bone frame.
- Front view: Section A-A at x = 1.0 m through the middle seat and back cushion, looking from +x. The cut plinth, deck, cushions and back frame form the section; behind them lie the left arm and the left feet.
- Plan: seen from above (x, z).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_three_seat_sofa_kit", os.path.join(HERE, "r3-three-seat-sofa-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

SEAT = {"region": "seat-cushions", "material": K.FABRIC}
BACK = {"region": "back-cushions", "material": K.FABRIC}
ARM = {"region": "arm-pads", "material": K.FABRIC}
FRAME = {"region": "back-frame", "material": K.FABRIC}
BASE = {"region": "base-rail", "material": K.FABRIC}
FEET = {"region": "feet", "material": K.BEECH}
ARMS = (("arm-l", "arm-left", 0.1), ("arm-r", "arm-right", 1.9))


def n(v):
    """🔢 A coordinate as written in the file."""
    return round(v + 0.0, 4)


def shape(sid, bone, z, tone, geometry, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def poly(points, closed=True):
    """🔷 A polygon or an open polyline, consecutive duplicates dropped."""
    pts = []
    for x, y in points:
        p = [n(x), n(y)]
        if not pts or pts[-1] != p:
            pts.append(p)
    if closed and len(pts) > 1 and pts[0] == pts[-1]:
        pts.pop()
    return {"kind": "poly", "points": pts, "closed": closed}


def rect(x0, y0, x1, y1, r=None):
    """▭ A rectangle between two corners, optionally rounded."""
    out = {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}
    if r:
        out["r"] = r
    return out


def strokes(segments):
    """〰 Several open strokes as one path."""
    return {"kind": "path", "d": " ".join(f"M {n(a)} {n(b)} L {n(c)} {n(d)}" for (a, b), (c, d) in segments)}


def el(bone, pts):
    """📐 World (x, y) points in the elevation frame of a bone."""
    return [K.local(bone, x, y)[:2] for x, y in pts]


def foot_outline(scale, c):
    """🪵 The outline of a turned foot about the lateral or forward centre c."""
    prof = [p for p in K.FOOT[scale] if p[1] > 0]
    return [(c + r, a) for a, r in prof] + [(c - r, a) for a, r in reversed(prof)]


def stations(scale):
    """📚 The station tables of a scale."""
    return {"seat": K.seat_stations(scale), "back": K.back_stations(scale), "arm": K.arm_stations(scale), "frame": K.backframe_stations(scale)}


def elevation(scale):
    """🔍 The elevation (from the front) of scales 1:50 to 1:200."""
    st = stations(scale)
    out = []
    for name, x in (("foot-l", K.FOOT_X[0]), ("foot-r", K.FOOT_X[1])):
        out.append(shape(name, "frame", 1, 0.4, poly(el("frame", foot_outline(scale, x))), **FEET))
        if scale == "50":
            out.append(shape(name.replace("foot", "foot-ring"), "frame", 1.5, 0.3, strokes([tuple(el("frame", [(x - 0.03, y), (x + 0.03, y)])) for y in (0.0695, 0.0785)]), role="detail", filled=False, region="feet"))
    out.append(shape("back-frame", "frame", 2, 0.55, poly(el("frame", K.lofted_outline(st["frame"], 1.0, "x"))), **FRAME))
    out.append(shape("base", "frame", 5.8, 0.45, poly(el("frame", [(K.BASE_X[0], K.BASE_Y[0]), (K.BASE_X[1], K.BASE_Y[0]), (K.BASE_X[1], K.BASE_Y[1]), (K.BASE_X[0], K.BASE_Y[1])])), **BASE))
    for k in range(3):
        cx = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        bs, bb = f"seat-{k + 1}", f"back-{k + 1}"
        out.append(shape(bb, bb, 3, 0.62, poly(el(bb, K.lofted_outline(st["back"], cx, "x"))), **BACK))
        out.append(shape(bs, bs, 5, 0.72, poly(el(bs, K.lofted_outline(st["seat"], cx, "x"))), **SEAT))
        if scale == "50":
            hx = (K.SEAT_X[1] - K.SEAT_X[0]) / 2 - 0.012
            segs = [tuple(el(bs, [(cx - hx, y), (cx + hx, y)])) for y in (0.40, 0.25)]
            out.append(shape(f"piping-{k + 1}", bs, 5.5, 0.3, strokes(segs), role="detail", filled=False, region="seat-cushions"))
    for sid, bone, cx in ARMS:
        out.append(shape(sid, bone, 6, 0.5, poly(el(bone, K.lofted_outline(st["arm"], cx, "x"))), **ARM))
    return out


def front(scale):
    """🔪 Section A-A at x = 1.0 m (scales 1:50 to 1:200)."""
    st = stations(scale)
    out = [shape("arm-l", "arm-left", 2, 0.5, poly(K.lofted_outline(st["arm"], 0.1, "z")), **ARM)]
    for k, z in enumerate((K.FOOT_Z, -K.FOOT_Z)):
        out.append(shape(f"foot-{k}", "frame", 1, 0.4, poly(foot_outline(scale, z)), **FEET))
    outer = [(z, y) for y, z in K.base_profile(scale)]
    out.append(shape("base", "frame", 6.5, 0.45, poly(outer), **BASE))
    out.append(shape("back-frame", "frame", 5, 0.55, poly(K.lofted_outline(st["frame"], 1.0, "z")), **FRAME))
    out.append(shape("seat-2", "seat-2", 6, 0.72, poly(K.lofted_outline(st["seat"], 1.0, "z")), **SEAT))
    out.append(shape("back-2", "back-2", 6, 0.62, poly(K.lofted_outline(st["back"], 1.0, "z")), **BACK))
    return out


def plan(scale):
    """🗺 The plan (scales 1:50 to 1:200)."""
    st = stations(scale)
    out = [shape("base", "frame", 1, 0.45, rect(K.BASE_X[0], K.BASE_Z[0], K.BASE_X[1], K.BASE_Z[1]), **BASE)]
    out.append(shape("back-frame", "frame", 7, 0.55, poly(K.lofted_plan(st["frame"], 1.0, 12)), **FRAME))
    for k in range(3):
        cx = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        out.append(shape(f"seat-{k + 1}", f"seat-{k + 1}", 4, 0.72, poly(K.lofted_plan(st["seat"], cx, 10)), **SEAT))
        out.append(shape(f"back-{k + 1}", f"back-{k + 1}", 5, 0.62, poly(K.lofted_plan(st["back"], cx, 5)), **BACK))
    for sid, bone, cx in ARMS:
        out.append(shape(sid, bone, 6, 0.5, poly(K.lofted_plan(st["arm"], cx, 10)), **ARM))
    return out


BACK500 = [(K.DECK_Y, K.SEAT_Z[1]), (K.DECK_Y, 0.45), (K.FRAME_TOP, 0.45), (K.FRAME_TOP, 0.33), (0.78, 0.22), (K.SEAT_TOP, 0.15), (K.SEAT_TOP, K.SEAT_Z[1])]


def views500():
    """🛰 The three views at 1:500."""
    both_seat = {"regions": ["seat-cushions", "base-rail"], "material": K.FABRIC}
    both_back = {"regions": ["back-cushions", "back-frame"], "material": K.FABRIC}
    elev = [shape(f"foot-{s}", "frame", 1, 0.3, poly(el("frame", [(x - 0.0375, 0), (x + 0.0375, 0), (x + 0.0375, K.FOOT_H), (x - 0.0375, K.FOOT_H)])), **FEET) for s, x in (("l", K.FOOT_X[0]), ("r", K.FOOT_X[1]))]
    elev.append(shape("back-block", "frame", 2, 0.6, poly(el("frame", [(K.BASE_X[0], K.DECK_Y), (K.BASE_X[1], K.DECK_Y), (K.BASE_X[1], K.FRAME_TOP), (K.BASE_X[0], K.FRAME_TOP)])), **both_back))
    elev.append(shape("seat-block", "frame", 4, 0.7, poly(el("frame", [(K.BASE_X[0], K.BASE_Y[0]), (K.BASE_X[1], K.BASE_Y[0]), (K.BASE_X[1], K.SEAT_TOP), (K.BASE_X[0], K.SEAT_TOP)])), **both_seat))
    for sid, bone, cx in ARMS:
        elev.append(shape(sid, bone, 6, 0.5, poly(el(bone, [(cx - 0.1, K.ARM_Y[0]), (cx + 0.1, K.ARM_Y[0]), (cx + 0.1, K.ARM_Y[1]), (cx - 0.1, K.ARM_Y[1])])), **ARM))
    fr = [shape("arm-l", "arm-left", 2, 0.5, rect(K.ARM_Z[0], K.ARM_Y[0], K.ARM_Z[1], K.ARM_Y[1]), **ARM)]
    fr += [shape(f"foot-{k}", "frame", 1, 0.3, rect(z - 0.0375, 0, z + 0.0375, K.FOOT_H), **FEET) for k, z in enumerate((K.FOOT_Z, -K.FOOT_Z))]
    fr.append(shape("seat-block", "frame", 5, 0.7, rect(K.BASE_Z[0], K.BASE_Y[0], K.SEAT_Z[1], K.SEAT_TOP), **both_seat))
    fr.append(shape("back-block", "frame", 5, 0.6, poly([(z, y) for y, z in BACK500]), **both_back))
    pl = [shape(sid, bone, 6, 0.5, rect(cx - 0.1, K.ARM_Z[0], cx + 0.1, K.ARM_Z[1]), **ARM) for sid, bone, cx in ARMS]
    pl.append(shape("seat-block", "frame", 4, 0.7, rect(K.BASE_X[0], K.BASE_Z[0], K.BASE_X[1], K.SEAT_Z[1]), **both_seat))
    pl.append(shape("back-block", "frame", 5, 0.6, rect(K.BASE_X[0], K.SEAT_Z[1], K.BASE_X[1], 0.45), **both_back))
    return {"elevation": elev, "front": fr, "plan": pl}


SECTION = ("The front elevation is Section A-A at x = 1.0 m through the middle seat and back cushion, looking from the right end: the cut plinth, cushions and back frame are the section, the left arm and the left feet lie beyond the cut plane.",
           "Die Vorderansicht ist Schnitt A-A bei x = 1,0 m durch das mittlere Sitz- und Rückenkissen, vom rechten Ende gesehen: Sockel, Kissen und Rückenrahmen sind geschnitten, die linke Armlehne und die linken Füße liegen hinter der Schnittebene.")

DECISIONS = {
    "50": {
        "silhouette": ("Sofa 40 x 17 mm drawn from the 1:50 body: box arms with 60 mm top rolls, the back frame with its rolled top between them, three pillow back cushions, three crowned seat cushions with top and bottom piping, the plinth, four turned feet with their bead. Front view: the plinth, the seat cushion, the leaning back cushion and the back frame in section, the side of the left arm and the two left feet beyond. Plan: arms, rolled back frame, back cushions and seat cushions with their rounded corners. ",
                       "Sofa 40 x 17 mm aus dem 1:50-Körper gezeichnet: Kastenarmlehnen mit 60-mm-Oberrolle, dazwischen der Rückenrahmen mit gerollter Oberkante, drei Rückenkissen, drei bombierte Sitzkissen mit oberem und unterem Keder, der Sockel, vier gedrechselte Füße mit Wulst. Vorderansicht: der Sockel, Sitzkissen, geneigtes Rückenkissen und Rückenrahmen im Schnitt, dahinter die Seite der linken Armlehne und die zwei linken Füße. Grundriss: Armlehnen, gerollter Rückenrahmen, Rücken- und Sitzkissen mit ihren gerundeten Ecken. "),
        "retained": ("Piping of the seat boxing, foot bead, every rounding of 7.5 mm and more.", "Keder der Sitzbox, Fußwulst, jede Rundung ab 7,5 mm."),
        "merged": ("Rear feet coincide with the front feet; the batting and the frame inside arms and back are within their outlines.", "Hintere Füße fallen mit den vorderen zusammen; Vlies und Rahmen in Armlehnen und Rücken liegen in ihren Konturen."),
        "omitted": ("Fabric weave, zips, staples, spring wires, the dust cover.", "Gewebestruktur, Reißverschlüsse, Klammern, Federdrähte, der Staubschutz."),
        "thinElements": ("Piping (8 mm) and the foot bead are 0.25 mm detail lines.", "Keder (8 mm) und Fußwulst sind 0,25-mm-Detaillinien."),
        "hatching": ("No hatching: upholstery is shown by tone.", "Keine Schraffur: Polster werden durch Ton gezeigt."),
        "styles": ("Tone 0.72 seat, 0.62 back cushions, 0.55 back frame, 0.5 arms, 0.45 plinth, 0.4 feet.", "Ton 0,72 Sitz, 0,62 Rückenkissen, 0,55 Rückenrahmen, 0,5 Armlehnen, 0,45 Sockel, 0,4 Füße."),
        "overlays": ("One shape per cushion, arm, back frame, plinth and foot, each with its region, so overlays follow every part separately.", "Eine Form je Kissen, Armlehne, Rückenrahmen, Sockel und Fuß, jede mit ihrer Region, Überlagerungen folgen jedem Teil einzeln."),
    },
    "100": {
        "silhouette": ("Sofa 20 x 8.5 mm from the 1:100 body: arms, back frame, three back and three seat cushions with simpler roundings, plinth, tapered feet. Front view: plinth, seat, back cushion and back frame in section, the left arm and feet beyond. Plan: arms, back frame and the six cushions. ",
                        "Sofa 20 x 8,5 mm aus dem 1:100-Körper: Armlehnen, Rückenrahmen, drei Rücken- und drei Sitzkissen mit einfacheren Rundungen, Sockel, konische Füße. Vorderansicht: Sockel, Sitz, Rückenkissen und Rückenrahmen im Schnitt, dahinter linke Armlehne und Füße. Grundriss: Armlehnen, Rückenrahmen und die sechs Kissen. "),
        "retained": ("Cushion division (each sinks on its own), arm rolls, back roll, foot taper.", "Kissenteilung (jedes sinkt einzeln ein), Armrollen, Rückenrolle, Fußverjüngung."),
        "merged": ("Roundings in two to three steps.", "Rundungen in zwei bis drei Stufen."),
        "omitted": ("Piping, foot bead.", "Keder, Fußwulst."),
        "thinElements": ("None below 15 mm remain.", "Nichts unter 15 mm bleibt."),
        "hatching": ("None.", "Keine."),
        "styles": ("Tones as at 1:50.", "Töne wie bei 1:50."),
        "overlays": ("Each part carries its region.", "Jedes Teil trägt seine Region."),
    },
    "200": {
        "silhouette": ("Sofa 10 x 4.3 mm from the 1:200 body: arms with a chamfered roll, back frame, six cushions in few stations, plinth, two straight feet. Front view: section of plinth, seat, back cushion and back frame with the arm beyond. Plan: arms, back frame and the six cushions. ",
                        "Sofa 10 x 4,3 mm aus dem 1:200-Körper: Armlehnen mit gefaster Rolle, Rückenrahmen, sechs Kissen in wenigen Stationen, Sockel, zwei gerade Füße. Vorderansicht: Schnitt durch Sockel, Sitz, Rückenkissen und Rückenrahmen mit der Armlehne dahinter. Grundriss: Armlehnen, Rückenrahmen und die sechs Kissen. "),
        "retained": ("Cushion division, the leaning back, the rolls as chamfers.", "Kissenteilung, der geneigte Rücken, die Rollen als Fasen."),
        "merged": ("Roundings under 30 mm become one chamfer step.", "Rundungen unter 30 mm werden eine Fasenstufe."),
        "omitted": ("Piping, foot taper.", "Keder, Fußverjüngung."),
        "thinElements": ("Feet 50 to 60 mm stay at true width.", "Füße 50 bis 60 mm bleiben in wahrer Breite."),
        "hatching": ("None.", "Keine."),
        "styles": ("Fabric tones 0.45-0.72, feet 0.4.", "Stofftöne 0,45-0,72, Füße 0,4."),
        "overlays": ("Each part carries its region.", "Jedes Teil trägt seine Region."),
    },
    "500": {
        "silhouette": ("One composed sofa sign 4.0 x 1.7 mm from the 1:500 body: seat block, back block, two arm blocks, two 75 mm feet. Front view: the seat block and the leaning back block in section, the arm beyond, two feet. Plan: arms, seat block and back block. ",
                        "Ein zusammengesetztes Sofazeichen 4,0 x 1,7 mm aus dem 1:500-Körper: Sitzblock, Rückenblock, zwei Armblöcke, zwei 75-mm-Füße. Vorderansicht: Sitzblock und geneigter Rückenblock im Schnitt, die Armlehne dahinter, zwei Füße. Grundriss: Armlehnen, Sitzblock und Rückenblock. "),
        "retained": ("Arm tops at 0.62 m, back top at 0.85 m, seat top at 0.43 m and the feet.", "Armlehnenoberkante 0,62 m, Rückenoberkante 0,85 m, Sitzoberkante 0,43 m und die Füße."),
        "merged": ("Plinth and seat cushions are one block, back cushions and back frame one block.", "Sockel und Sitzkissen sind ein Block, Rückenkissen und Rückenrahmen ein Block."),
        "omitted": ("Cushion seams, roundings, rear feet.", "Kissenfugen, Rundungen, hintere Füße."),
        "thinElements": ("Turned feet (45-60 mm) are drawn at the 75 mm feature.", "Gedrechselte Füße (45-60 mm) sind mit dem 75-mm-Merkmal gezeichnet."),
        "hatching": ("None.", "Keine."),
        "styles": ("Seat 0.7, back 0.6, arms 0.5, feet 0.3.", "Sitz 0,7, Rücken 0,6, Armlehnen 0,5, Füße 0,3."),
        "overlays": ("The seat block lists seat cushions and plinth, the back block back cushions and back frame.", "Der Sitzblock führt Sitzkissen und Sockel, der Rückenblock Rückenkissen und Rückenrahmen."),
    },
}


def decisions(scale):
    """💬 The representation decisions of a scale."""
    out = {}
    for key, (en, de) in DECISIONS[scale].items():
        if key == "silhouette":
            en, de = en + SECTION[0], de + SECTION[1]
        out[key] = {"en": en, "de": de}
    return out


def apply(asset):
    """✏ Writes the 2D geometry and decisions of every scale into the asset in memory."""
    for scale in ("50", "100", "200", "500"):
        rep = asset["representations"][scale]
        rep["geometry"] = views500() if scale == "500" else {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = decisions(scale)
