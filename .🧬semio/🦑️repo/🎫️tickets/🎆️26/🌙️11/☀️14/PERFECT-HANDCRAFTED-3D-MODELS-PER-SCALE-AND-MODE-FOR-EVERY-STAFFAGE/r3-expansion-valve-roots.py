"""🌱 Round 3, machinery/expansion-valve: the root corrections outside `volumes` (texts, references, dimensions, materials, rig, anchors, 2D drawings of 1:50 and 1:100, poses, clips, thermal regions, structure), drawn from the same measures as the solids.

Usage: imported by r3-expansion-valve-build.py (`apply_roots(asset)`).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_expansion_valve_kit", os.path.join(HERE, "r3-expansion-valve-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
m = K.m
AREAS = {"suction-line": 0.0045}


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def shape(sid, bone, role, z, filled, tone, geometry, region, material, pen=None):
    """✏️ One 2D shape in the member order of the catalog."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geometry, "region": region, "material": material}
    if pen:
        out["penMm"] = pen
    return out


def poly(points, closed=True):
    """🔷 A polygon or polyline through points already in metres."""
    return {"kind": "poly", "points": [[round(x, 6), round(y, 6)] for x, y in points], "closed": closed}


def lines(strokes):
    """〰️ Open strokes (lists of points in metres) as one path."""
    return {"kind": "path", "d": " ".join("M " + " L ".join("%g %g" % (round(x, 6), round(y, 6)) for x, y in stroke) for stroke in strokes)}


def circle(cx, cy, r):
    """⭕ A circle (mm in, metres out)."""
    return {"kind": "ellipse", "cx": m(cx), "cy": m(cy), "rx": m(r), "ry": m(r)}


def fcircle(z, y, r):
    """🎯 A circle of the front view at world (z, y) in mm."""
    return circle(z, y - K.CUT, r)


def elev(bone, pts):
    """📐 World points (x, y in mm) as elevation points of a bone."""
    return [K.local2(bone, p) for p in pts]


def loc(pts):
    """📐 Bone-local points (mm) as elevation points (m)."""
    return [(m(u), m(v)) for u, v in pts]


def front(pts):
    """👁️ World points (z, y in mm) as front points."""
    return [(m(z), m(y - K.CUT)) for z, y in pts]


def plan(pts):
    """🗺️ World points (x, z in mm) as plan points."""
    return [(m(x), m(z)) for x, z in pts]


def box(x0, y0, x1, y1):
    """▭ A rectangle through two corners."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def sym(flank):
    """🪞 A closed outline symmetric about the vertical axis from its right flank (half-width, height), bottom to top."""
    return [(r, y) for r, y in flank] + [(-r, y) for r, y in reversed(flank)]


NUT_HALF_ELEV = K.NUT_AF / 2 / math.cos(math.radians(30))
NUT_FLANK = [(6.3, 50), (6.8, 51), (9.0, 55), (9.3, 55.6), (None, 55.6), (None, 67.4), (9.3, 67.4), (8.9, 68)]
BODY_FLANK = [(7.95, 68), (7.95, 70)] + [(r, y) for y, r, _ in K.BARREL]
STEM_HALF = K.STEM_AF / 2 / math.cos(math.radians(30))
STEM_OUTLINE = [(-2, -7), (4, -7), (4, -STEM_HALF), (19.5, -STEM_HALF), (19.5, -8.2), (20.8, -8.2), (22, -7), (22, 7), (20.8, 8.2), (19.5, 8.2), (19.5, STEM_HALF), (4, STEM_HALF), (4, 7), (-2, 7)]
HEAD_OUTLINE = K.mirrored([(0.2, 7.0)] + K.HEAD[1:])
BULB_OUTLINE = K.mirrored(K.BULB + [(K.BULB_TIP, 0.4)])
COIL_Y = (K.COIL_BOTTOM - K.CAP_R, K.COIL_TOP + K.CAP_R)
COIL_HALF = K.COIL_R + K.CAP_R


def nut(half):
    """🔩 The outline of the flare nut seen with the given half-width of its hexagon."""
    return sym([(half if r is None else r, y) for r, y in NUT_FLANK])


def body_elevation():
    """🧱 Side outline of the forged body: threaded inlet, barrel and the outlet boss on the right."""
    right = [(7.95, 68), (7.95, 70)] + [(r, y) for y, r, _ in K.BARREL[:5]] + [(11.5, 84.4), (15.5, 84.4), (17, 85.6), (17, 100.4), (15.5, 101.6), (11.5, 101.6)] + [(r, y) for y, r, _ in K.BARREL[5:]]
    left = [(-r, y) for y, r, _ in reversed(K.BARREL)] + [(-7.95, 70), (-7.95, 68)]
    return right + left


def stem_points():
    """🔩 World points (x, y, z) on the surface of the spindle boss, hexagon cap and end cap."""
    pts = []
    for u, r in [(4, 7), (19.5, 8.2), (20.8, 8.2), (22, 7)]:
        pts += [K.world("stem", u, v, w) for v, w in K.ring(r, 24)]
    for u in (4, 19.5):
        pts += [K.world("stem", u, v, w) for v, w in K.hexagon(K.STEM_AF)]
    return pts


def lead(view, n=10):
    """〰️ The capillary lead projected to a view."""
    pts = K.lead_path(n)
    if view == "elevation":
        return elev("head", [(x, y) for x, y, _ in pts])
    if view == "front":
        return front([(z, y) for _, y, z in pts if math.hypot(z - K.BULB_Z, y - K.BULB_Y) > K.BULB_R + 3.0])
    return plan([(x, z) for x, _, z in pts])


def geometry50():
    """🖊️ 1:50: the three true orthographic views of the 1:50 body."""
    sx0, sx1 = K.SUCTION_X
    sy, by, bz, oy = K.SUCTION_Y, K.BULB_Y, K.BULB_Z, K.OUTLET_Y
    bulb_plan = [(-57 - u, bz + v) for u, v in BULB_OUTLINE]
    coil_ring = [(K.COIL_R * math.cos(math.radians(a)), K.COIL_R * math.sin(math.radians(a))) for a in range(100, 441, 10)]
    elevation = [
        shape("capillary", "head", "detail", 3.8, False, 0.1, lines([elev("head", [(0, 123), (0, COIL_Y[0])]), lead("elevation")]), "capillary", "stainless-steel"),
        shape("capillary-coil", "head", "detail", 3.7, False, 0.1, poly(elev("head", box(-COIL_HALF, COIL_Y[0], COIL_HALF, COIL_Y[1]))), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.2, True, 0.4, poly(elev("bulb", box(sx0, sy - 8, sx1, sy + 8))), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.3, True, 0.5, poly(loc(BULB_OUTLINE)), "bulb", "stainless-steel"),
        shape("bulb-strap", "bulb", "detail", 3.4, False, 0.1, lines([loc([(25, -4.5), (25, 20.5)])]), "suction-line", "stainless-steel"),
        shape("stem", "stem", "body", 3.3, True, 0.3, poly(loc(STEM_OUTLINE)), "body", "brass"),
        shape("head", "head", "body", 3.6, True, 0.55, poly(loc(HEAD_OUTLINE)), "head", "stainless-steel"),
        shape("inlet", "body", "body", 3.2, True, 0.4, poly(elev("body", box(-K.LIQUID_R, K.CUT, K.LIQUID_R, 51))), "inlet", "copper"),
        shape("inlet-nut", "body", "body", 3.4, True, 0.5, poly(elev("body", nut(NUT_HALF_ELEV))), "inlet", "brass"),
        shape("outlet-pipe", "body", "body", 3.15, True, 0.4, poly(elev("body", box(50, oy - K.OUTLET_R, K.OUTLET_END, oy + K.OUTLET_R))), "outlet", "copper"),
        shape("outlet", "body", "body", 3.25, True, 0.4, poly(elev("body", [(15, oy - K.STUB_R), (42, oy - K.STUB_R), (44.5, oy - K.BELL_R), (54, oy - K.BELL_R), (54, oy + K.BELL_R), (44.5, oy + K.BELL_R), (42, oy + K.STUB_R), (15, oy + K.STUB_R)])), "outlet", "copper"),
        shape("body", "body", "body", 3.5, True, 0.5, poly(elev("body", body_elevation())), "body", "brass"),
    ]
    front_view = [
        shape("capillary", "head", "detail", 3.8, False, 0.1, lines([front([(0, 123), (0, COIL_Y[0])]), lead("front")]), "capillary", "stainless-steel"),
        shape("capillary-coil", "head", "detail", 3.7, False, 0.1, poly(front(box(-COIL_HALF, COIL_Y[0], COIL_HALF, COIL_Y[1]))), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.0, True, 0.4, fcircle(bz, sy, K.SUCTION_R), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.1, True, 0.5, fcircle(bz, by, K.BULB_R), "bulb", "stainless-steel"),
        shape("head", "head", "body", 3.6, True, 0.55, poly(front([(v, 108 + u) for u, v in HEAD_OUTLINE])), "head", "stainless-steel"),
        shape("inlet", "body", "body", 3.2, True, 0.4, poly(front(box(-K.LIQUID_R, K.CUT, K.LIQUID_R, 51))), "inlet", "copper"),
        shape("inlet-nut", "body", "body", 3.4, True, 0.5, poly(front(nut(K.NUT_AF / 2))), "inlet", "brass"),
        shape("body", "body", "body", 3.5, True, 0.5, poly(front(sym(BODY_FLANK))), "body", "brass"),
        shape("outlet", "body", "body", 3.6, True, 0.4, fcircle(0, oy, K.BELL_R), "outlet", "copper"),
        shape("outlet-pipe", "body", "body", 3.65, True, 0.4, fcircle(0, oy, K.OUTLET_R), "outlet", "copper"),
    ]
    plan_view = [
        shape("capillary", "head", "detail", 3.9, False, 0.1, lines([lead("plan")]), "capillary", "stainless-steel"),
        shape("capillary-coil", "head", "detail", 3.8, False, 0.1, poly(plan(coil_ring), False), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.1, True, 0.4, poly(plan(box(sx0, bz - K.SUCTION_R, sx1, bz + K.SUCTION_R))), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.3, True, 0.5, poly(plan(bulb_plan)), "bulb", "stainless-steel"),
        shape("bulb-strap", "bulb", "detail", 3.4, False, 0.1, lines([plan([(-82, bz - 5), (-82, bz + 5)])]), "suction-line", "stainless-steel"),
        shape("stem", "stem", "body", 3.2, True, 0.3, poly(plan(K.hull([(x, z) for x, _, z in stem_points()]))), "body", "brass"),
        shape("body", "body", "body", 3.3, True, 0.5, poly(plan([(11.5 * math.copysign(abs(math.cos(a)) ** 0.5, math.cos(a)), 11.5 * math.copysign(abs(math.sin(a)) ** 0.5, math.sin(a))) for a in [math.radians(15 * k) for k in range(24)]])), "body", "brass"),
        shape("outlet-pipe", "body", "body", 3.15, True, 0.4, poly(plan(box(50, -K.OUTLET_R, K.OUTLET_END, K.OUTLET_R))), "outlet", "copper"),
        shape("outlet", "body", "body", 3.25, True, 0.4, poly(plan([(15, -K.STUB_R), (42, -K.STUB_R), (44.5, -K.BELL_R), (54, -K.BELL_R), (54, K.BELL_R), (44.5, K.BELL_R), (42, K.STUB_R), (15, K.STUB_R)])), "outlet", "copper"),
        shape("head", "head", "body", 3.6, True, 0.55, circle(0, 0, 21), "head", "stainless-steel"),
    ]
    return {"elevation": elevation, "front": front_view, "plan": plan_view}


INLET100 = [(7.5, K.CUT), (7.5, 50), (11, 53), (11, 68), (9.5, 70)]
BODY100 = [(11.5, 66), (11.5, 106), (8, 108.5)]
HEAD100 = K.mirrored([(0.2, 8), (2.5, 8), (6, 21), (10.5, 21), (13, 9), (17.5, 5)])
COIL100 = K.mirrored([(17, 29), (18, 30), (34, 30), (35, 29)])
STEM100 = K.mirrored([(-2, 8.5), (20.5, 8.5), (22, 7)])
BULB100 = K.mirrored([(0, 5), (3, 7.5), (47, 7.5), (50, 5)])


def geometry100():
    """🖊️ 1:100: the three true orthographic views of the 1:100 body."""
    sx0, sx1 = K.SUCTION_X
    sy, by, bz, oy = K.SUCTION_Y, K.BULB_Y, K.BULB_Z, K.OUTLET_Y
    stem = []
    for u, r in [(4, 8.5), (20.5, 8.5), (22, 7)]:
        stem += [K.world("stem", u, v, w) for v, w in K.ring(r, 24)]
    elevation = [
        shape("capillary", "head", "body", 3.8, False, 0.1, poly(lead("elevation", 6), False), "capillary", "stainless-steel"),
        shape("capillary-coil", "head", "body", 3.7, True, 0.3, poly(loc(COIL100)), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.2, True, 0.4, poly(elev("bulb", box(sx0, sy - 8, sx1, sy + 8))), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.3, True, 0.5, poly(loc(BULB100)), "bulb", "stainless-steel"),
        shape("stem", "stem", "body", 3.3, True, 0.3, poly(loc(STEM100)), "body", "brass"),
        shape("head", "head", "body", 3.6, True, 0.55, poly(loc(HEAD100)), "head", "stainless-steel"),
        shape("inlet", "body", "body", 3.2, True, 0.4, poly(elev("body", sym(INLET100))), "inlet", "copper"),
        shape("outlet", "body", "body", 3.25, True, 0.4, poly(elev("body", box(8, oy - 7.5, K.OUTLET_END, oy + 7.5))), "outlet", "copper"),
        shape("body", "body", "body", 3.5, True, 0.5, poly(elev("body", sym(BODY100))), "body", "brass"),
    ]
    front_view = [
        shape("capillary", "head", "body", 3.8, False, 0.1, poly(lead("front", 6), False), "capillary", "stainless-steel"),
        shape("capillary-coil", "head", "body", 3.7, True, 0.3, poly(front([(v, 108 + u) for u, v in COIL100])), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.0, True, 0.4, fcircle(bz, sy, K.SUCTION_R), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.1, True, 0.5, fcircle(bz, by, 7.5), "bulb", "stainless-steel"),
        shape("head", "head", "body", 3.6, True, 0.55, poly(front([(v, 108 + u) for u, v in HEAD100])), "head", "stainless-steel"),
        shape("inlet", "body", "body", 3.2, True, 0.4, poly(front(sym(INLET100))), "inlet", "copper"),
        shape("body", "body", "body", 3.5, True, 0.5, poly(front(sym(BODY100))), "body", "brass"),
        shape("outlet", "body", "body", 3.6, True, 0.4, fcircle(0, oy, 7.5), "outlet", "copper"),
    ]
    plan_view = [
        shape("capillary", "head", "body", 3.9, False, 0.1, poly(lead("plan", 6), False), "capillary", "stainless-steel"),
        shape("suction-line", "bulb", "body", 3.1, True, 0.4, poly(plan(box(sx0, bz - K.SUCTION_R, sx1, bz + K.SUCTION_R))), "suction-line", "copper"),
        shape("bulb", "bulb", "body", 3.3, True, 0.5, poly(plan([(-57 - u, bz + v) for u, v in BULB100])), "bulb", "stainless-steel"),
        shape("stem", "stem", "body", 3.2, True, 0.3, poly(plan(K.hull([(x, z) for x, _, z in stem]))), "body", "brass"),
        shape("body", "body", "body", 3.3, True, 0.5, circle(0, 0, 11.5), "body", "brass"),
        shape("outlet", "body", "body", 3.25, True, 0.4, poly(plan(box(8, -7.5, K.OUTLET_END, 7.5))), "outlet", "copper"),
        shape("head", "head", "body", 3.6, True, 0.55, circle(0, 0, 21), "head", "stainless-steel"),
        shape("capillary-coil", "head", "body", 3.7, True, 0.3, circle(0, 0, 30), "capillary", "stainless-steel"),
    ]
    return {"elevation": elevation, "front": front_view, "plan": plan_view}


DECISIONS50 = {
    "silhouette": text("A thermostatic expansion valve as it sits in the pipework, 2.4 mm tall and 3.9 mm wide on paper: the liquid line rises through a flare nut into the forged body, the outlet leaves to the right, the setting spindle cap points down to the left, the flat element lies on top with the coiled capillary above it, and the bulb is strapped on a piece of the suction line at the left.",
                       "Ein thermostatisches Expansionsventil im Rohrnetz, auf Papier 2,4 mm hoch und 3,9 mm breit: Die Flüssigkeitsleitung steigt durch eine Bördelmutter in den Schmiedekörper, der Ausgang geht nach rechts ab, die Kappe der Einstellspindel zeigt nach links unten, das flache Element liegt oben mit der aufgewickelten Kapillare darüber, und der Fühler ist links auf ein Stück Saugleitung gespannt."),
    "retained": text("Side elevation: liquid line, flare nut, body with outlet boss, copper stub with socket and soldered pipe, spindle cap, element, coil envelope, capillary lead, bulb with nib, band and suction line. Front elevation: the same valve seen along the outlet, whose socket and pipe are circles; the suction line and the bulb stand end-on 24 mm beside the valve ; the spindle cap is hidden behind the body. Plan: element, the coil as one ring, the lead, outlet, spindle cap, and the bulb lying on the suction line under its band line.",
                     "Seitenansicht: Flüssigkeitsleitung, Bördelmutter, Körper mit Ausgangsauge, Kupferstutzen mit Muffe und eingelötetem Rohr, Spindelkappe, Element, Hüllform der Windungen, Kapillarzuleitung, Fühler mit Füllnippel, Band und Saugleitung. Vorderansicht: dasselbe Ventil in Richtung des Ausgangs gesehen, dessen Muffe und Rohr Kreise sind; Saugleitung und Fühler stehen 24 mm neben dem Ventil in der Stirnansicht; die Spindelkappe ist vom Körper verdeckt. Grundriss: Element, die Windungen als ein Ring, die Zuleitung, Ausgang, Spindelkappe und der Fühler auf der Saugleitung unter seiner Bandlinie."),
    "merged": text("The seven capillary turns are one envelope rectangle in both elevations and one ring in the plan; the element is one outline.",
                   "Die sieben Kapillarwindungen sind in beiden Ansichten ein Hüllrechteck und im Grundriss ein Ring; das Element ist ein Umriss."),
    "omitted": text("The orifice assembly and filter inside the inlet, the thread turns, the hexagon edges of nut and cap, the pipe bores, the spiral of the capillary above the coil, the band lock and its screw, the engraving on the element. In the plan the flare nut and the liquid line lie hidden under the body.",
                    "Der Düseneinsatz und das Sieb im Eingang, die Gewindegänge, die Sechskantkanten von Mutter und Kappe, die Rohrbohrungen, die Spirale der Kapillare über den Windungen, das Bandschloss mit Schraube, die Gravur auf dem Element. Im Grundriss liegen Bördelmutter und Flüssigkeitsleitung verdeckt unter dem Körper."),
    "thinElements": text("The capillary (1.6 mm real) and the 10 mm band are 0.13 mm lines; the liquid line is 9.5 mm, the outlet pipe 12 mm, the bulb 14.5 mm thick.",
                         "Die Kapillare (real 1,6 mm) und das 10-mm-Band sind 0,13-mm-Linien; die Flüssigkeitsleitung ist 9,5 mm, das Ausgangsrohr 12 mm, der Fühler 14,5 mm dick."),
    "hatching": text("None.", "Keine."),
    "styles": text("Brass parts carry the middle tone, copper pipes a lighter one, the stainless element the darkest, the spindle cap a light accent; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                   "Messingteile tragen den mittleren Ton, Kupferrohre einen helleren, das Edelstahlelement den dunkelsten, die Spindelkappe einen hellen Akzent; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
    "overlays": text("Seven regions: the inlet (liquid line and flare nut) is warm, body, outlet, head, bulb and suction line are cold, the capillary is passive.",
                     "Sieben Regionen: Der Eingang (Flüssigkeitsleitung und Bördelmutter) ist warm, Körper, Ausgang, Kopf, Fühler und Saugleitung sind kalt, die Kapillare ist passiv."),
}

DECISIONS100 = {
    "silhouette": text("A valve 1.2 mm tall and 2 mm wide on paper: a column with a barrel, an outlet bar to the right, an inclined stub to the left, a disc with the coil disc on top and a bulb bar on a pipe bar at the left.",
                       "Ein Ventil, auf Papier 1,2 mm hoch und 2 mm breit: eine Säule mit Zylinder, ein Ausgangsstab nach rechts, ein geneigter Stumpf nach links, eine Scheibe mit der Windungsscheibe darüber und links ein Fühlerstab auf einem Rohrstab."),
    "retained": text("Inlet column, body barrel, outlet bar, spindle stub, element disc, coil disc, capillary lead, bulb and suction line. Front elevation: column, barrel, outlet circle, element and coil discs, bulb and suction line circles beside the valve. Plan: coil disc over the element, outlet bar, spindle stub, bulb on the suction line and the lead.",
                     "Eingangssäule, Körperzylinder, Ausgangsstab, Spindelstumpf, Elementscheibe, Windungsscheibe, Kapillarzuleitung, Fühler und Saugleitung. Vorderansicht: Säule, Zylinder, Ausgangskreis, Element- und Windungsscheibe, Fühler- und Saugleitungskreis neben dem Ventil. Grundriss: Windungsscheibe über dem Element, Ausgangsstab, Spindelstumpf, Fühler auf der Saugleitung und die Zuleitung."),
    "merged": text("Liquid line and flare nut are one column; stub, socket and pipe one outlet bar; spindle boss and cap one stub; the seven capillary turns one disc.",
                   "Flüssigkeitsleitung und Bördelmutter sind eine Säule; Stutzen, Muffe und Rohr ein Ausgangsstab; Spindelauge und Kappe ein Stumpf; die sieben Kapillarwindungen eine Scheibe."),
    "omitted": text("Hexagon edges, flare, chamfers, band, nib, rim and bores. In the plan the element lies hidden under the coil disc and the inlet under the body.",
                    "Sechskantkanten, Bördel, Fasen, Band, Füllnippel, Rand und Bohrungen. Im Grundriss liegt das Element verdeckt unter der Windungsscheibe und der Eingang unter dem Körper."),
    "thinElements": text("Liquid line and outlet pipe are drawn 15 mm thick (real 9.5 mm and 12 mm), the bulb 15 mm; the capillary lead is one 0.18 mm line.",
                         "Flüssigkeitsleitung und Ausgangsrohr sind 15 mm dick gezeichnet (real 9,5 mm und 12 mm), der Fühler 15 mm; die Kapillarzuleitung ist eine 0,18-mm-Linie."),
    "hatching": text("None.", "Keine."),
    "styles": text("Brass carries the middle tone, copper a lighter one, the element the darkest and the coil disc a light one; body pen 0.18 mm, no detail pen, ground 0.25 mm.",
                   "Messing trägt den mittleren Ton, Kupfer einen helleren, das Element den dunkelsten und die Windungsscheibe einen hellen; Körperstift 0,18 mm, kein Detailstift, Boden 0,25 mm."),
    "overlays": text("The seven regions keep their own masses; the capillary region is the coil disc and the lead line.",
                     "Die sieben Regionen behalten ihre eigenen Massen; die Region Kapillare besteht aus der Windungsscheibe und der Zuleitungslinie."),
}


def unavailable(rep, paper):
    """🚫 The texts of an unavailable scale against the real smallest part."""
    rep["reason"] = text("At 1:%d the asset is only %s mm wide on paper and its smallest part, the 9.5 mm liquid line, is far below every print profile." % (rep["denominator"], paper),
                         "Bei 1:%d ist das Gerät nur %s mm breit auf Papier und sein kleinstes Teil, die 9,5 mm dicke Flüssigkeitsleitung, liegt weit unter jedem Druckprofil." % (rep["denominator"], paper.replace(".", ",")))
    rep["limitingFeature"] = text("The 9.5 mm liquid line", "Die 9,5 mm dicke Flüssigkeitsleitung")
    rep["limitingSizeM"] = 0.0095


def bone(asset, bid):
    """🦴 A bone of the rig by id."""
    return next(b for b in asset["rig"]["bones"] if b["id"] == bid)


def region(asset, rid):
    """🌡️ A thermal region by id."""
    return next(r for r in asset["thermal"]["regions"] if r["id"] == rid)


def apply_roots(asset):
    """🌱 Writes every root correction into the asset (in place)."""
    asset["summary"] = text(
        "A brass thermostatic expansion valve of a small heat pump as it sits in the pipework: warm liquid rises through the flare nut, is throttled in the body and leaves to the right as a very cold two-phase mixture; the element on top is steered through the coiled capillary by the bulb strapped on the suction line, so body, outlet, bulb and suction line cool below room temperature.",
        "Ein thermostatisches Expansionsventil aus Messing einer kleinen Wärmepumpe, wie es im Rohrnetz sitzt: Warme Flüssigkeit steigt durch die Bördelmutter auf, wird im Körper gedrosselt und verlässt ihn nach rechts als sehr kaltes Zweiphasengemisch; das Element oben wird über die aufgewickelte Kapillare vom Fühler gesteuert, der auf die Saugleitung gespannt ist, sodass Körper, Ausgang, Fühler und Saugleitung unter Raumtemperatur abkühlen.")
    refs = asset["provenance"]["references"]
    if not any(r["id"] == "danfoss-t2-2021" for r in refs):
        refs.append({"id": "danfoss-t2-2021", "citation": "Danfoss Climate Solutions (2021). Thermostatic expansion valve, type T 2 and TE 2, data sheet AI241486443133en-001401 (functions, dimensions and weights, capillary tube 1.5 m, stainless steel bulb and bulb strap).", "url": "https://assets.danfoss.com/documents/175152/AI241486443133en-001401.pdf"})
    asset["provenance"]["notes"] = text(
        "A thermostatic expansion valve of the 3 kW class after the Danfoss T 2 pattern, flare by solder (danfoss-t2-2021): an angle body of forged brass on a vertical axis, the 3/8 in liquid line entering from below through a flare nut 35 mm under the outlet axis, the 12 mm solder outlet ending 54 mm beside the axis, the superheat setting spindle under a hexagon cap 35 mm to the other side, the thermostatic element of 42 mm ending 30 mm above the outlet axis, a 1.5 m capillary of 1.6 mm whose surplus is coiled above the element, and a stainless bulb of 14.5 mm by 50 mm held by a band on top of the 16 mm suction line. The valve weighs 0.3 kg; with flare nut, the three pipe pieces and the band the asset weighs 0.4 kg. Nothing moves outside when the valve modulates: the diaphragm and the cone travel inside, so the poses differ only in flow and temperature. The inlet carries warm liquid of about 35 degC and is slightly warmer than the room; the throttled outlet and the body are near the evaporating temperature of -5 degC, the suction line and the bulb near 0 degC. The heatW values (authored estimates) are the net heat the surfaces take from the room at that temperature difference. The pipes are cut samples of the pipework that carries the valve; the origin is the cut end of the liquid line on the valve axis.",
        "Ein thermostatisches Expansionsventil der 3-kW-Klasse nach dem Muster Danfoss T 2, Bördel mal Löt (danfoss-t2-2021): ein Eckkörper aus geschmiedetem Messing auf senkrechter Achse, die 3/8-Zoll-Flüssigkeitsleitung von unten durch eine Bördelmutter 35 mm unter der Ausgangsachse, der 12-mm-Lötausgang 54 mm neben der Achse endend, die Überhitzungs-Einstellspindel unter einer Sechskantkappe 35 mm zur anderen Seite, das thermostatische Element von 42 mm 30 mm über der Ausgangsachse endend, eine 1,5 m lange Kapillare von 1,6 mm, deren Überlänge über dem Element aufgewickelt ist, und ein Edelstahlfühler von 14,5 mm mal 50 mm, der mit einem Band oben auf der 16-mm-Saugleitung gehalten wird. Das Ventil wiegt 0,3 kg; mit Bördelmutter, den drei Rohrstücken und dem Band wiegt das Objekt 0,4 kg. Außen bewegt sich nichts, wenn das Ventil regelt: Membran und Kegel bewegen sich im Inneren, daher unterscheiden sich die Posen nur in Durchfluss und Temperatur. Der Eingang führt warme Flüssigkeit von rund 35 degC und ist etwas wärmer als der Raum; der gedrosselte Ausgang und der Körper liegen nahe der Verdampfungstemperatur von -5 degC, Saugleitung und Fühler nahe 0 degC. Die heatW-Werte (Schätzwerte) sind die Nettowärme, die die Oberflächen bei diesem Temperaturunterschied dem Raum entziehen. Die Rohre sind geschnittene Proben des Rohrnetzes, das das Ventil trägt; der Ursprung ist das Schnittende der Flüssigkeitsleitung auf der Ventilachse.")
    asset["dimensions"] = {"heightM": 0.121, "widthM": 0.196, "depthM": 0.062, "massKg": 0.4}
    if not any(x["id"] == "stainless-steel" for x in asset["materials"]):
        asset["materials"].append({"id": "stainless-steel", "title": text("Stainless steel", "Edelstahl"), "emissivity": 0.22, "solarAbsorptivity": 0.4, "densityKgM3": 7900, "conductivityWmK": 14.9, "specificHeatJKgK": 477, "source": "literature", "reference": "incropera-2018"})

    titles = {
        "body": text("Forged brass valve body with liquid line and outlet", "Geschmiedeter Messing-Ventilkörper mit Flüssigkeitsleitung und Ausgang"),
        "head": text("Thermostatic element with capillary", "Thermostatisches Element mit Kapillare"),
        "stem": text("Superheat setting spindle under its cap", "Überhitzungs-Einstellspindel unter ihrer Kappe"),
        "bulb": text("Sensing bulb strapped on the suction line", "Fühler, auf die Saugleitung gespannt"),
    }
    masses = {"body": (0.24, 0.0084, -0.0093), "head": (0.065, 0.011, 0.0), "stem": (0.015, 0.01, 0.0), "bulb": (0.08, 0.025, 0.0067)}
    for bid, (ox, oy, oz, deg) in K.BONES.items():
        b = bone(asset, bid)
        b["parent"] = None if bid == "body" else "body"
        b["x"], b["y"] = (m(ox), m(oy - K.CUT)) if bid == "body" else (m(ox), m(oy - K.OUTLET_Y))
        if oz:
            b["z"] = m(oz)
        b["angleDeg"] = deg
        b["title"] = titles[bid]
        b["lengthM"] = m(K.LENGTHS[bid])
        b["joint"] = {"type": "fixed"}
        kg, cx, cy = masses[bid]
        b["mass"] = {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate"}
    order = ["id", "parent", "x", "y", "z", "angleDeg", "title", "lengthM", "joint", "mass"]
    asset["rig"]["bones"] = [{k: b[k] for k in order if k in b} for b in asset["rig"]["bones"]]

    lx, ly, _ = K.local("body", (0, K.CUT, 0))
    ox, oy, _ = K.local("body", (K.OUTLET_END, K.OUTLET_Y, 0))
    sx, sy, _ = K.local("bulb", (K.SUCTION_X[0], K.SUCTION_Y, K.BULB_Z))
    asset["anchors"] = [
        {"id": "liquid-line", "bone": "body", "x": lx, "y": ly, "kind": "support", "title": text("Cut end of the liquid line (pin)", "Schnittende der Flüssigkeitsleitung (Festlager)"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.0095, "footprintWidthM": 0.0095},
        {"id": "outlet-pipe", "bone": "body", "x": ox, "y": oy, "kind": "support", "title": text("Cut end of the outlet pipe (roller)", "Schnittende des Ausgangsrohrs (Rollenlager)"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.012, "footprintWidthM": 0.012},
        {"id": "suction-line", "bone": "bulb", "x": sx, "y": sy, "kind": "support", "title": text("Cut end of the suction line (roller)", "Schnittende der Saugleitung (Rollenlager)"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.016, "footprintWidthM": 0.016},
    ]
    supports = [a["id"] for a in asset["anchors"]]

    reps = asset["representations"]
    reps["50"]["geometry"] = geometry50()
    reps["50"]["decisions"] = DECISIONS50
    reps["100"]["geometry"] = geometry100()
    reps["100"]["decisions"] = DECISIONS100
    unavailable(reps["200"], "1")
    unavailable(reps["500"], "0.4")

    for pose in asset["poses"]:
        pose["bones"] = {}
        pose["contacts"] = list(supports)
        regions = pose["thermal"]["regions"]
        if "suction-line" not in regions:
            regions["suction-line"] = dict(regions["bulb"])
    described = {
        "closed": text("The valve is shut: no refrigerant flows and everything is at room temperature.", "Das Ventil ist geschlossen: kein Kältemittel fließt und alles hat Raumtemperatur."),
        "half-open": text("Half the rated flow passes: outlet, body and suction line cool.", "Der halbe Nennstrom fließt: Ausgang, Körper und Saugleitung kühlen ab."),
        "open": text("The rated flow expands from liquid to a cold two-phase mixture: the outlet is the coldest part.", "Der Nennstrom entspannt von Flüssigkeit zu einem kalten Zweiphasengemisch: der Ausgang ist der kälteste Teil."),
    }
    for pose in asset["poses"]:
        pose["description"] = described[pose["id"]]
    for clip in asset["clips"]:
        clip["contacts"] = [{"anchor": a, "fromS": 0, "toS": clip["durationS"]} for a in supports]
        if clip["id"] == "modulate":
            clip["description"] = text("The diaphragm follows the superheat at the bulb: the flow rises to its rated value and falls back, and the cold outlet follows; nothing moves on the outside.", "Die Membran folgt der Überhitzung am Fühler: Der Durchfluss steigt auf den Nennwert und fällt zurück, und der kalte Ausgang folgt; außen bewegt sich nichts.")

    thermal = asset["thermal"]
    for rid in ("head", "bulb", "capillary"):
        r = region(asset, rid)
        r["material"] = "stainless-steel"
        for layer in r["layers"]:
            layer["material"] = "stainless-steel"
    region(asset, "head")["title"] = text("Thermostatic element", "Thermostatisches Element")
    region(asset, "inlet")["title"] = text("Liquid line with flare nut", "Flüssigkeitsleitung mit Bördelmutter")
    thermal["regions"] = [r for r in thermal["regions"] if r["id"] != "suction-line"]
    if True:
        thermal["regions"].append({
            "id": "suction-line", "title": text("Suction line under the bulb", "Saugleitung unter dem Fühler"), "kind": "surface", "model": "operating", "material": "copper", "areaM2": AREAS["suction-line"],
            "characteristicLengthM": 0.064, "geometry": "cylinder", "skyViewFactor": 0.5,
            "layers": [{"id": "suction-line-skin", "title": text("Suction line", "Saugleitung"), "kind": "metal", "material": "copper", "thicknessM": 0.001, "resistanceM2KW": 1e-05, "heatCapacityJM2K": 3400, "source": "authored-estimate", "reference": "incropera-2018"}],
            "heatCapacityJM2K": 3400, "operating": {"key": "flow", "heatW": -0.8}, "source": "authored-estimate", "reference": "incropera-2018"})
    thermal["assumptions"] = [text(
        "The valve is shown as a sample cut out of its pipework: the liquid line, the outlet pipe and the suction line end in section faces that exchange no heat, and the suction line stands for the evaporator outlet that the bulb senses.",
        "Das Ventil ist als aus dem Rohrnetz geschnittene Probe gezeigt: Flüssigkeitsleitung, Ausgangsrohr und Saugleitung enden in Schnittflächen ohne Wärmeaustausch, und die Saugleitung steht für den Verdampferausgang, den der Fühler abtastet.")]
    asset["structure"]["supports"] = supports
    asset["structure"]["assumptions"] = [text(
        "The 0.4 kg valve hangs in its pipework: a pin at the cut end of the liquid line and rollers at the cut ends of the outlet pipe and of the suction line carry the weight; the pipes are rigid and the capillary carries nothing.",
        "Das 0,4 kg schwere Ventil hängt in seinen Rohren: ein Festlager am Schnittende der Flüssigkeitsleitung und Rollenlager an den Schnittenden von Ausgangsrohr und Saugleitung tragen das Gewicht; die Rohre sind starr und die Kapillare trägt nichts.")]
