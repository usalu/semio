"""✏ Round 3, furniture/park-bench: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn from the same measures as the solids: side elevation (seen from the front of the bench, -z; seat and backrest bone-local, end frames in their vertical bone frames), front elevation (seen from the right end, +x: the slat ends, then the near end frame with its I-section) and plan (from above, x by z).
"""
import importlib.util
import os

from shapely.geometry import Polygon
from shapely.geometry.polygon import orient

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_park_bench_" + name, os.path.join(HERE, f"r3-park-bench-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

SEAT = {"region": "seat-timber", "material": K.OAK}
BACK = {"region": "back-timber", "material": K.OAK}
STEEL = {"region": "steel-frame", "material": K.STEEL}
ENDS = (("l", "end-left", K.FRAME_X[0]), ("r", "end-right", K.FRAME_X[1]))
HALF = K.FLANGE / 2
ARM_HALF = 0.025
PAD_HALF = K.PAD_X / 2
TOP = K.post_front(K.POST_TOP)[1]


def n(v):
    """🔢 A coordinate as written in the file."""
    return round(v + 0.0, 4)


def shape(sid, bone, z, tone, geometry, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def rect(x0, y0, x1, y1, r=0.0):
    """▭ A rectangle between two corners."""
    out = {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}
    if r:
        out["r"] = r
    return out


def poly(points, closed=True):
    """🔷 A polygon."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def circle(cx, cy, r):
    """⚪ A circle."""
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": n(r), "ry": n(r)}


def path_of(polygon):
    """🧭 A polygon with holes as one path (outer ring counter-clockwise, holes clockwise: non-zero winding leaves them open)."""
    p = orient(polygon, 1.0)
    rings = [list(p.exterior.coords)[:-1]] + [list(r.coords)[:-1] for r in p.interiors]
    d = " ".join("M " + " L ".join(f"{n(x)} {n(y)}" for x, y in ring) + " Z" for ring in rings)
    return {"kind": "path", "d": d}


def end_rect(xc, x0, x1, y0, y1, r=0.0):
    """▭ A world rectangle (x range, y range) in the elevation frame of a vertical end-frame bone at xc: u = y, v = -(x - xc)."""
    return rect(y0, -(x1 - xc), y1, -(x0 - xc), r)


def yrange(points):
    """↕ The height range of a section."""
    ys = [p[1] for p in points]
    return min(ys), max(ys)


def zrange(points):
    """↔ The lateral range of a section."""
    zs = [p[0] for p in points]
    return min(zs), max(zs)


def armrest_side(knuckle):
    """🦾 The armrest seen from the side (front view, z by y): the bar between the knuckle and the post."""
    yc = (K.ARM_Y0 + K.ARM_Y1) / 2
    h = (K.ARM_Y1 - K.ARM_Y0) / 2
    f, b = K.ARM_FRONT, K.arm_reach()
    if knuckle:
        pts = [(f, yc - 0.35 * h), (f + 0.003, yc - 0.75 * h), (f + 0.009, yc - 0.94 * h), (f + 0.018, yc - h), (b, yc - h), (b, yc + h), (f + 0.018, yc + h), (f + 0.009, yc + 0.94 * h), (f + 0.003, yc + 0.75 * h), (f, yc + 0.35 * h)]
    else:
        pts = [(f, yc - h), (b, yc - h), (b, yc + h), (f, yc + h)]
    return poly(pts)


def seat_bands(sections, sy, tones):
    """💺 The seat slats in the side elevation: each slat's height band, the front one on top."""
    out = []
    for k, sec in enumerate(sections):
        y0, y1 = yrange(sec)
        out.append((k, rect(0, y0 - sy, K.LENGTH, y1 - sy), tones[k]))
    return out


def elevation(scale):
    """🔍 The side elevation of a scale."""
    sy, by = K.seat_y(), K.back_y()
    out = []
    if scale in ("50", "100"):
        hatch = {"hatch": "oak-grain", "penMm": 0.35} if scale == "50" else {}
        for k, r, tone in seat_bands(K.seat_slats(K.SLAT_R if scale == "50" else 0), sy, (0.62, 0.52, 0.47, 0.42)):
            out.append(shape(f"seat-slat-{k + 1}", "seat", 4.4 - 0.1 * k, tone, r, **hatch, **SEAT))
        for k, sec in enumerate(K.back_slats(K.SLAT_R if scale == "50" else 0)):
            y0, y1 = yrange(sec)
            out.append(shape(f"back-slat-{k + 1}", "backrest", 3, 0.58, rect(0, y0 - by, K.LENGTH, y1 - by), **hatch, **BACK))
            if scale == "50":
                t = sum(K.BACK_S[k]) / 2
                z, y = K.add(K.post_front(t), K.M, K.BACK_T)
                for p, x in zip(("l", "r"), K.FRAME_X):
                    out.append(shape(f"back-slat-{k + 1}-bolt-{p}", "backrest", 3.3, 0.15, circle(x, y - by, 0.0095), role="detail", filled=False, **STEEL))
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-post", bone, 2, 0.4, end_rect(xc, xc - HALF, xc + HALF, 0.35, TOP), **STEEL))
            out.append(shape(f"{p}-leg", bone, 5, 0.42, poly([(K.PAD_H, 0.025), (0.1, HALF), (K.ARM_Y1 - 0.004, HALF), (K.ARM_Y1 - 0.004, -HALF), (0.1, -HALF), (K.PAD_H, -0.025)]), **STEEL))
            out.append(shape(f"{p}-pads", bone, 5.1, 0.3, end_rect(xc, xc - PAD_HALF, xc + PAD_HALF, 0, K.PAD_H, 0.004 if scale == "50" else 0), **STEEL))
            out.append(shape(f"{p}-armrest", bone, 6, 0.45, end_rect(xc, xc - ARM_HALF, xc + ARM_HALF, K.ARM_Y0, K.ARM_Y1, 0.012 if scale == "50" else 0.008), **STEEL))
    elif scale == "200":
        s0, s1 = K.SEAT_S[0][0], K.SEAT_S[-1][1]
        y0, y1 = yrange(K.slat_section(K.seat_point(s0), K.D, K.N, s1 - s0, K.SEAT_T, 0))
        out.append(shape("seat-board", "seat", 4, 0.6, rect(0, y0 - sy, K.LENGTH, y1 - sy), **SEAT))
        for k, (t0, t1) in enumerate([K.BACK_S[0], (K.BACK_S[1][0], K.BACK_S[2][1])]):
            sec = K.slat_section(K.add(K.post_front(t0), K.M, K.BACK_T), K.E, (-K.M[0], -K.M[1]), t1 - t0, K.BACK_T, 0)
            a, b = yrange(sec)
            out.append(shape(f"back-board-{k + 1}", "backrest", 3, 0.56, rect(0, a - by, K.LENGTH, b - by), **BACK))
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-post", bone, 2, 0.4, end_rect(xc, xc - HALF, xc + HALF, 0.35, TOP), **STEEL))
            out.append(shape(f"{p}-leg", bone, 5, 0.42, poly([(0, 0.035), (0.03, HALF + 0.005), (0.1, HALF), (K.ARM_Y1, HALF), (K.ARM_Y1, -HALF), (0.1, -HALF), (0.03, -HALF - 0.005), (0, -0.035)]), **STEEL))
    else:
        h = B.FRAME500 / 2
        out.append(shape("seat-slab", "seat", 4, 0.6, rect(0, B.SEAT500[0] - sy, K.LENGTH, B.SEAT500[1] - sy), **SEAT))
        back = [p[1] for p in B.back500()]
        out.append(shape("back-slab", "backrest", 3, 0.56, rect(0, min(back) - by, K.LENGTH, max(back) - by), **BACK))
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-frame", bone, 5, 0.4, end_rect(xc, xc - h, xc + h, 0, B.profile500().bounds[3]), **STEEL))
    return out


def front(scale):
    """🔍 The front elevation of a scale: the near (right) end frame and the slat ends."""
    out = []
    if scale in ("50", "100"):
        r = K.SLAT_R if scale == "50" else 0
        prof = K.frame_profile(True)
        out.append(shape("r-frame", "end-right", 2, 0.42, path_of(prof), **STEEL))
        if scale == "50":
            core, _ = K.i_section(prof)
            out.append(shape("r-web", "end-right", 2.5, 0.32, path_of(core), role="detail", **STEEL))
        out.append(shape("r-armrest", "end-right", 3, 0.45, armrest_side(scale == "50"), **STEEL))
        out.append(shape("r-pad-front", "end-right", 3, 0.3, rect(K.FRONT_Z - K.PAD_L / 2, 0, K.FRONT_Z + K.PAD_L / 2, K.PAD_H), **STEEL))
        out.append(shape("r-pad-rear", "end-right", 3, 0.3, rect(K.REAR_FOOT_Z - K.PAD_L / 2, 0, K.REAR_FOOT_Z + K.PAD_L / 2, K.PAD_H), **STEEL))
        pen = {"penMm": 0.35} if scale == "50" else {}
        for k, sec in enumerate(K.seat_slats(r)):
            out.append(shape(f"seat-slat-{k + 1}", "seat", 5, 0.62, poly(sec), **pen, **SEAT))
        for k, sec in enumerate(K.back_slats(r)):
            out.append(shape(f"back-slat-{k + 1}", "backrest", 5, 0.58, poly(sec), **pen, **BACK))
    elif scale == "200":
        out.append(shape("r-frame", "end-right", 2, 0.42, path_of(B.coarse_profile()), **STEEL))
        s0, s1 = K.SEAT_S[0][0], K.SEAT_S[-1][1]
        out.append(shape("seat-board", "seat", 5, 0.6, poly(K.slat_section(K.seat_point(s0), K.D, K.N, s1 - s0, K.SEAT_T, 0)), **SEAT))
        for k, (t0, t1) in enumerate([K.BACK_S[0], (K.BACK_S[1][0], K.BACK_S[2][1])]):
            sec = K.slat_section(K.add(K.post_front(t0), K.M, K.BACK_T), K.E, (-K.M[0], -K.M[1]), t1 - t0, K.BACK_T, 0)
            out.append(shape(f"back-board-{k + 1}", "backrest", 5, 0.56, poly(sec), **BACK))
    else:
        out.append(shape("r-frame", "end-right", 2, 0.4, path_of(B.profile500()), **STEEL))
        s1 = K.seat_point(K.SEAT_S[-1][1])[0]
        out.append(shape("seat-slab", "seat", 5, 0.6, rect(K.NOSE[0], B.SEAT500[0], s1, B.SEAT500[1]), **SEAT))
        out.append(shape("back-slab", "backrest", 5, 0.56, poly(B.back500()), **BACK))
    return out


def plan(scale):
    """🗺 The plan of a scale."""
    out = []
    if scale in ("50", "100"):
        r = K.SLAT_R if scale == "50" else 0
        hatch = {"hatch": "oak-grain", "penMm": 0.35} if scale == "50" else {}
        for k, sec in enumerate(K.seat_slats(r)):
            z0, z1 = zrange(sec)
            out.append(shape(f"seat-slat-{k + 1}", "seat", 4, 0.62, rect(0, z0, K.LENGTH, z1), **hatch, **SEAT))
        for k, sec in enumerate(K.back_slats(r)):
            z0, z1 = zrange(sec)
            out.append(shape(f"back-slat-{k + 1}", "backrest", 5 + 0.1 * k, 0.58, rect(0, z0, K.LENGTH, z1), **hatch, **BACK))
        prof = K.frame_profile(True)
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-pads", bone, 2, 0.3, rect(xc - PAD_HALF, K.FRONT_Z - K.PAD_L / 2, xc + PAD_HALF, K.FRONT_Z + K.PAD_L / 2), **STEEL))
            out.append(shape(f"{p}-pads-rear", bone, 2, 0.3, rect(xc - PAD_HALF, K.REAR_FOOT_Z - K.PAD_L / 2, xc + PAD_HALF, K.REAR_FOOT_Z + K.PAD_L / 2), **STEEL))
            out.append(shape(f"{p}-frame", bone, 3, 0.42, rect(xc - HALF, prof.bounds[0], xc + HALF, prof.bounds[2]), **STEEL))
            out.append(shape(f"{p}-armrest", bone, 6, 0.45, rect(xc - ARM_HALF, K.ARM_FRONT, xc + ARM_HALF, K.arm_reach(), 0.012 if scale == "50" else 0), **STEEL))
    elif scale == "200":
        s0, s1 = K.SEAT_S[0][0], K.SEAT_S[-1][1]
        z0, z1 = zrange(K.slat_section(K.seat_point(s0), K.D, K.N, s1 - s0, K.SEAT_T, 0))
        out.append(shape("seat-board", "seat", 4, 0.6, rect(0, z0, K.LENGTH, z1), **SEAT))
        for k, (t0, t1) in enumerate([K.BACK_S[0], (K.BACK_S[1][0], K.BACK_S[2][1])]):
            sec = K.slat_section(K.add(K.post_front(t0), K.M, K.BACK_T), K.E, (-K.M[0], -K.M[1]), t1 - t0, K.BACK_T, 0)
            a, b = zrange(sec)
            out.append(shape(f"back-board-{k + 1}", "backrest", 5 + 0.1 * k, 0.56, rect(0, a, K.LENGTH, b), **BACK))
        prof = B.coarse_profile()
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-feet", bone, 2, 0.3, rect(xc - 0.035, prof.bounds[0], xc + 0.035, prof.bounds[2]), **STEEL))
            out.append(shape(f"{p}-frame", bone, 6, 0.42, rect(xc - HALF, prof.bounds[0], xc + HALF, prof.bounds[2]), **STEEL))
    else:
        h = B.FRAME500 / 2
        s1 = K.seat_point(K.SEAT_S[-1][1])[0]
        out.append(shape("seat-slab", "seat", 4, 0.6, rect(0, K.NOSE[0], K.LENGTH, s1), **SEAT))
        a, b = zrange(B.back500())
        out.append(shape("back-slab", "backrest", 5, 0.56, rect(0, a, K.LENGTH, b), **BACK))
        prof = B.profile500()
        for p, bone, xc in ENDS:
            out.append(shape(f"{p}-frame", bone, 6, 0.4, rect(xc - h, prof.bounds[0], xc + h, prof.bounds[2]), **STEEL))
    return out


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": T("Bench 36 x 17 mm with its whole construction: seen from the front, the four seat slats step down to the rear with the seat rake, the lumbar slat and the two upper back slats with the 90 mm gap where the armrests meet the posts, the bolt heads over each frame, the front legs with their arm posts and rounded armrests standing in front of the slats, the back posts behind them and the foot pads. Front view (right end): the slat ends with eased edges, the one-piece cast end frame with its flange rim and recessed web, both openings, the raked rail and back post, the rear leg kicked back, the brace, the armrest with its knuckle and the two pads. Plan: four seat slats with the drainage gaps, three back slats stepping back with the post rake, armrests over the frames, the frame flanges and pads where they show.",
                        "Bank 36 x 17 mm mit ihrer ganzen Konstruktion: von vorn treten die vier Sitzlatten mit der Sitzneigung nach hinten abwärts, die Lendenlatte und die zwei oberen Rückenlatten mit der 90-mm-Fuge, in der die Armlehnen die Pfosten treffen, die Schraubenköpfe über jedem Gestell, die Vorderbeine mit Armstützen und gerundeten Armlehnen vor den Latten, die Rückenpfosten dahinter und die Fußplatten. Vorderansicht (rechtes Ende): die Lattenenden mit gebrochenen Kanten, das einteilige Gussgestell mit Randflansch und vertieftem Steg, beide Öffnungen, geneigte Zarge und Rückenpfosten, das nach hinten ausgestellte Hinterbein, die Strebe, die Armlehne mit Knauf und die zwei Fußplatten. Grundriss: vier Sitzlatten mit den Wasserfugen, drei mit der Pfostenneigung zurücktretende Rückenlatten, Armlehnen über den Gestellen, Gestellflansche und Fußplatten, wo sie sichtbar sind."),
        "retained": T("Every slat with its gap and eased edges, the I-section of the casting (flange rim and web as two tones), the armrest knuckle, the foot pads, the carriage-bolt heads of the back slats.",
                      "Jede Latte mit Fuge und gebrochenen Kanten, der I-Querschnitt des Gusses (Randflansch und Steg in zwei Tönen), der Armlehnenknauf, die Fußplatten, die Schlossschraubenköpfe der Rückenlatten."),
        "merged": T("Left and right end frames coincide in the front view; front leg, rail, brace and rear leg coincide edge-on in the side elevation.",
                    "Linkes und rechtes Endgestell fallen in der Vorderansicht zusammen; Vorderbein, Zarge, Strebe und Hinterbein fallen in der Seitenansicht in einer Kante zusammen."),
        "omitted": T("Bolt heads of the seat (under the armrests in plan, 5 mm high in elevation), nuts and washers under the rail flange.",
                     "Schraubenköpfe des Sitzes (im Grundriss unter den Armlehnen, in der Ansicht 5 mm hoch), Muttern und Scheiben unter dem Zargenflansch."),
        "thinElements": T("The 10 mm drainage gaps are 0.2 mm on paper and stay open between the slat outlines; the 9 mm flange rim is the band between the frame outline and the web.",
                          "Die 10-mm-Wasserfugen sind auf Papier 0,2 mm und bleiben zwischen den Lattenumrissen offen; der 9-mm-Randflansch ist das Band zwischen Gestellumriss und Steg."),
        "hatching": T("Oak grain as lines every 0.6 mm on the slats in elevation and plan; steel plain tone.",
                      "Eichenmaserung als Linien alle 0,6 mm auf den Latten in Ansicht und Grundriss; Stahl nur Tonfläche."),
        "styles": T("Oak light (seat 0.62 stepping darker to the rear slats, back 0.58), steel dark (rim 0.42, recessed web 0.32, pads 0.3), bolt heads near black.",
                    "Eiche hell (Sitz 0,62, zu den hinteren Latten dunkler, Lehne 0,58), Stahl dunkel (Flansch 0,42, vertiefter Steg 0,32, Fußplatten 0,3), Schraubenköpfe fast schwarz."),
        "overlays": T("Every shape carries one region (seat-timber, back-timber, steel-frame).", "Jede Form trägt genau eine Region (seat-timber, back-timber, steel-frame)."),
    },
    "100": {
        "silhouette": T("Bench 18 x 8.5 mm: the stepped seat slats, three back slats with the armrest gap, front legs with armrests in front, posts behind, pads. Front view: square slat ends, the end frame as one outline with both openings, the straight armrest and the pads. Plan: slats with their gaps, armrests, frame flanges and pads.",
                        "Bank 18 x 8,5 mm: die gestuften Sitzlatten, drei Rückenlatten mit der Armlehnenfuge, Vorderbeine mit Armlehnen davor, Pfosten dahinter, Fußplatten. Vorderansicht: scharfkantige Lattenenden, das Endgestell als ein Umriss mit beiden Öffnungen, die gerade Armlehne und die Fußplatten. Grundriss: Latten mit ihren Fugen, Armlehnen, Gestellflansche und Fußplatten."),
        "retained": T("Every slat and gap, both frame openings, pads.", "Jede Latte und Fuge, beide Gestellöffnungen, Fußplatten."),
        "merged": T("Web and flange of the casting into one plate; the armrest knuckle into the bar.", "Steg und Flansch des Gusses zu einer Platte; der Armlehnenknauf in den Stab."),
        "omitted": T("Bolt heads, eased edges, grain.", "Schraubenköpfe, gebrochene Kanten, Maserung."),
        "thinElements": T("The 10 mm gaps are drawn at the 0.15 mm print minimum (0.1 mm true).", "Die 10-mm-Fugen sind im Druckminimum 0,15 mm gezeichnet (wahr 0,1 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Oak 0.62 / 0.58, steel 0.42, pads 0.3.", "Eiche 0,62 / 0,58, Stahl 0,42, Fußplatten 0,3."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
    "200": {
        "silhouette": T("Bench 9 x 4.3 mm: the seat as one board, the back as the lumbar slat and one board above the armrest gap, the end frames with legs to the floor. Front view: the casting outline with both openings, the raked seat board and the two back boards. Plan: seat board, back boards and the frames.",
                        "Bank 9 x 4,3 mm: der Sitz als ein Brett, die Lehne als Lendenlatte und ein Brett über der Armlehnenfuge, die Endgestelle mit Beinen bis zum Boden. Vorderansicht: der Gussumriss mit beiden Öffnungen, das geneigte Sitzbrett und die zwei Lehnenbretter. Grundriss: Sitzbrett, Lehnenbretter und die Gestelle."),
        "retained": T("Seat rake, back rake, the armrest gap in the back, both openings of the frame.", "Sitzneigung, Lehnenneigung, die Armlehnenfuge der Lehne, beide Öffnungen des Gestells."),
        "merged": T("Seat slats into one board, the two upper back slats into one board, the armrest into the arm band, the pads into the feet.", "Sitzlatten zu einem Brett, die zwei oberen Rückenlatten zu einem Brett, die Armlehne ins Armband, die Fußplatten in die Füße."),
        "omitted": T("Slat gaps, bolts, eased edges.", "Lattenfugen, Schrauben, gebrochene Kanten."),
        "thinElements": T("Boards 35-45 mm thick (0.18-0.23 mm), drawn true.", "Bretter 35-45 mm stark (0,18-0,23 mm), wahr gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Oak 0.6 / 0.56, steel 0.42.", "Eiche 0,6 / 0,56, Stahl 0,42."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
    "500": {
        "silhouette": T("Bench sign 3.6 x 1.7 mm: the seat slab, the back slab above it and the two end frames. Front view: the frame of 75 mm members with the space under the seat and the gap under the armrest open, the flat seat slab and the raked back slab. Plan: seat, back and the two frames.",
                        "Bankzeichen 3,6 x 1,7 mm: die Sitzplatte, darüber die Lehnenplatte und die zwei Endgestelle. Vorderansicht: das Gestell aus 75-mm-Gliedern mit offenem Raum unter dem Sitz und offener Lücke unter der Armlehne, die flache Sitzplatte und die geneigte Lehnenplatte. Grundriss: Sitz, Lehne und die zwei Gestelle."),
        "retained": T("Overall size, seat height, back rake, armrest height, the open leg space.", "Gesamtgröße, Sitzhöhe, Lehnenneigung, Armlehnenhöhe, der offene Beinraum."),
        "merged": T("Slats into one seat slab and one back slab; legs, rail and post into 75 mm bands; the brace into the frame.", "Latten zu einer Sitz- und einer Lehnenplatte; Beine, Zarge und Pfosten zu 75-mm-Bändern; die Strebe ins Gestell."),
        "omitted": T("Seat rake (38 mm), pads, bolts, gaps.", "Sitzneigung (38 mm), Fußplatten, Schrauben, Fugen."),
        "thinElements": T("Every member at least 75 mm (0.15 mm print minimum).", "Jedes Glied mindestens 75 mm (Druckminimum 0,15 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Seat 0.6, back 0.56, frames 0.4.", "Sitz 0,6, Lehne 0,56, Gestelle 0,4."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
}


def apply(asset):
    """🖊 Replaces the geometry and decisions of every scale's representation."""
    for scale in ("50", "100", "200", "500"):
        rep = asset["representations"][scale]
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[scale]
