"""✏ Round 3, furniture/dining-table: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn by hand from the same measures as the solids: side elevation (seen from the right, bone-local), front elevation (seen from the right end, the near half) and plan (rest-pose world). The shadow gap stays open where the real table lets the light through; blocks, leg tops and the stored leaf fill it where they stand.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_dining_table_kit", os.path.join(HERE, "r3-dining-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

TOP = {"region": "table-top", "material": K.OAK}
APRON = {"region": "apron", "material": K.OAK}
LEGS = {"region": "legs", "material": K.OAK}
LEAF = {"region": "leaf-top", "material": K.OAK}
RAIL = {"region": "slide-rails", "material": K.STEEL}


def n(value):
    """🔢 A coordinate as written in the file."""
    return round(value + 0.0, 4)


def shape(sid, bone, z, tone, geometry, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def rect(x0, y0, x1, y1):
    """▭ A rectangle between two corners."""
    return {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}


def poly(points, closed=True):
    """🔷 A polygon or an open polyline."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def lines(segments):
    """〰 Several open strokes as one path."""
    return {"kind": "path", "d": " ".join(f"M {n(a)} {n(b)} L {n(c)} {n(d)}" for (a, b), (c, d) in segments)}


def local(bone, pts):
    """📐 World (x, y) points into the elevation frame of a bone."""
    ox, oy = K.ORIGIN[bone]
    return [(x - ox, y - oy) for x, y in pts]


def mirror(pts, hand):
    """🪞 Left-half world points carried to the half `hand`."""
    return [(K.wx(x, hand), y) for x, y in pts]


def lrect(bone, hand, x0, y0, x1, y1):
    """▭ A left-half world rectangle on the half `hand`, in the elevation frame of its bone."""
    (a, b), (c, d) = local(bone, mirror([(x0, y0), (x1, y1)], hand))
    return rect(a, b, c, d)


def lpoly(bone, hand, pts, closed=True):
    """🔷 A left-half world polygon on the half `hand`, in the elevation frame of its bone."""
    return poly(local(bone, mirror(pts, hand)), closed)


def tapered(c, foot, shoulder, taper_y, top_y):
    """🦵 The outline of a tapered leg about centre c."""
    return [(c - foot, 0.0), (c + foot, 0.0), (c + shoulder, taper_y), (c + shoulder, top_y), (c - shoulder, top_y), (c - shoulder, taper_y)]


def staves(x0, x1):
    """🪵 The glue joints of the oak staves along a plate, as plan strokes."""
    return lines([((x0, z), (x1, z)) for z in K.STAVES])


def elevation50():
    """🔍 1:50 side elevation."""
    out = [shape("rail", "frame", 1, 0.35, rect(0, K.CENTRE_Y0 - K.FRAME_Y, K.CENTRE_X1 - K.CENTRE_X0, K.CENTRE_Y1 - K.FRAME_Y), **RAIL)]
    for name, bone, hand in K.HALVES:
        p = name[0]
        out.append(shape("top-" + p, bone, 6, 0.7, lpoly(bone, hand, [(0, K.CREASE_Y), (0, K.TOP_Y1), (K.HALF, K.TOP_Y1), (K.HALF, K.TOP_Y0), (K.BEVEL, K.TOP_Y0)]), **TOP))
        out.append(shape("crease-" + p, bone, 6.5, 0.3, lpoly(bone, hand, [(K.BEVEL, K.CREASE_Y), (K.HALF, K.CREASE_Y)], False), role="detail", filled=False, region="table-top"))
        out.append(shape("apron-" + p, bone, 4, 0.55, lrect(bone, hand, K.LEG_X + K.LEG_TOP, K.APRON_Y0, K.HALF, K.APRON_Y1), hatch="grain-h", **APRON))
        for k, x0 in enumerate(K.BLOCK_X):
            out.append(shape(f"block-{p}-{k}", bone, 3, 0.45, lrect(bone, hand, x0, K.GAP_Y0, x0 + K.BLOCK_L, K.TOP_Y0), **APRON))
        out.append(shape("leg-" + p, bone, 5, 0.6, lpoly(bone, hand, tapered(K.LEG_X, K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y, K.TOP_Y0)), hatch="grain-v", **LEGS))
        out.append(shape("shoulder-" + p, bone, 5.5, 0.3, lpoly(bone, hand, [(K.LEG_X - K.LEG_TOP, K.TAPER_Y), (K.LEG_X + K.LEG_TOP, K.TAPER_Y)], False), role="detail", filled=False, region="legs"))
    out.append(shape("leaf", "leaf", 5.5, 0.65, rect(0, 0, 0.6, K.LEAF_T), **LEAF))
    out.append(shape("leaf-crease", "leaf", 5.6, 0.3, poly([(0, 0.015), (0.6, 0.015)], False), role="detail", filled=False, region="leaf-top"))
    return out


def front_legs(foot, shoulder, taper_y, top_y, hatch, detail):
    """🦵 Both legs of the near half in the front view (and their shoulder strokes)."""
    out = []
    for k, c in enumerate((K.LEG_Z, -K.LEG_Z)):
        extra = {"hatch": hatch} if hatch else {}
        if foot == shoulder:
            geometry = rect(c - foot, 0, c + foot, top_y)
        else:
            geometry = poly(tapered(c, foot, shoulder, taper_y, top_y))
        out.append(shape(f"leg-{k}", "top-right", 5, 0.6, geometry, **extra, **LEGS))
        if detail:
            out.append(shape(f"shoulder-{k}", "top-right", 5.5, 0.3, poly([(c - shoulder, taper_y), (c + shoulder, taper_y)], False), role="detail", filled=False, region="legs"))
    return out


def front50():
    """🔍 1:50 front elevation (the right end, near)."""
    h, b = K.DEPTH / 2, K.BEVEL
    out = [
        shape("top-r", "top-right", 6, 0.7, poly([(-h, K.CREASE_Y), (-h, K.TOP_Y1), (h, K.TOP_Y1), (h, K.CREASE_Y), (h - b, K.TOP_Y0), (-(h - b), K.TOP_Y0)]), **TOP),
        shape("crease-r", "top-right", 6.5, 0.3, poly([(-h, K.CREASE_Y), (h, K.CREASE_Y)], False), role="detail", filled=False, region="table-top"),
        shape("end-apron-r", "top-right", 4, 0.55, rect(-(K.LEG_Z - K.LEG_TOP), K.APRON_Y0, K.LEG_Z - K.LEG_TOP, K.APRON_Y1), hatch="grain-h", **APRON),
        shape("block-end-r-0", "top-right", 3.5, 0.45, rect(K.END_BLOCK_Z - K.BLOCK_L / 2, K.GAP_Y0, K.END_BLOCK_Z + K.BLOCK_L / 2, K.TOP_Y0), **APRON),
        shape("block-end-r-1", "top-right", 3.5, 0.45, rect(-K.END_BLOCK_Z - K.BLOCK_L / 2, K.GAP_Y0, -K.END_BLOCK_Z + K.BLOCK_L / 2, K.TOP_Y0), **APRON),
        shape("leaf", "leaf", 3, 0.65, poly([(-(h - b), K.GAP_Y0), (h - b, K.GAP_Y0), (h, K.GAP_Y0 + 0.015), (h, K.TOP_Y0), (-h, K.TOP_Y0), (-h, K.GAP_Y0 + 0.015)]), **LEAF),
    ]
    return out + front_legs(K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y, K.TOP_Y0, "grain-v", True)


def plan_common(texture):
    """🗺 The plan: two top halves, the leaf under them (shows when the halves part), and the stave joints."""
    h = K.DEPTH / 2
    out = [
        shape("top-l", "top-left", 6, 0.7, rect(0, -h, K.HALF, h), **TOP),
        shape("top-r", "top-right", 6, 0.7, rect(K.HALF, -h, K.LENGTH, h), **TOP),
        shape("leaf", "leaf", 5, 0.65, rect(K.LEAF_X0, -h, K.LEAF_X1, h), **LEAF),
    ]
    if texture:
        out += [
            shape("staves-l", "top-left", 6.5, 0.3, staves(0, K.HALF), role="texture", filled=False, **TOP),
            shape("staves-r", "top-right", 6.5, 0.3, staves(K.HALF, K.LENGTH), role="texture", filled=False, **TOP),
            shape("staves-leaf", "leaf", 5.5, 0.3, staves(K.LEAF_X0, K.LEAF_X1), role="texture", filled=False, **LEAF),
        ]
    return out


def elevation100():
    """🔎 1:100 side elevation."""
    out = [shape("rail", "frame", 1, 0.35, rect(0, 0.667 - K.FRAME_Y, 1.0, 0.687 - K.FRAME_Y), **RAIL)]
    for name, bone, hand in K.HALVES:
        p = name[0]
        out.append(shape("top-" + p, bone, 6, 0.7, lrect(bone, hand, 0, K.TOP_Y0, K.HALF, K.TOP_Y1), **TOP))
        out.append(shape("apron-" + p, bone, 4, 0.55, lrect(bone, hand, K.LEG_X + K.LEG_TOP, K.APRON_Y0, K.HALF, K.APRON_Y1), **APRON))
        out.append(shape("leg-" + p, bone, 5, 0.6, lpoly(bone, hand, tapered(K.LEG_X, K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y, K.TOP_Y0)), **LEGS))
    out.append(shape("leaf", "leaf", 5.5, 0.65, rect(0, 0, 0.6, K.LEAF_T), **LEAF))
    return out


def front100():
    """🔎 1:100 front elevation."""
    h = K.DEPTH / 2
    out = [
        shape("top-r", "top-right", 6, 0.7, rect(-h, K.TOP_Y0, h, K.TOP_Y1), **TOP),
        shape("end-apron-r", "top-right", 4, 0.55, rect(-(K.LEG_Z - K.LEG_TOP), K.APRON_Y0, K.LEG_Z - K.LEG_TOP, K.APRON_Y1), **APRON),
        shape("leaf", "leaf", 3, 0.65, rect(-h, K.GAP_Y0, h, K.TOP_Y0), **LEAF),
    ]
    return out + front_legs(K.LEG_FOOT, K.LEG_TOP, K.TAPER_Y, K.TOP_Y0, None, False)


def elevation200():
    """🗺 1:200 side elevation."""
    out = [shape("rail", "frame", 1, 0.35, rect(0, 0.66 - K.FRAME_Y, 1.0, K.GAP_Y0 - K.FRAME_Y), **RAIL)]
    for name, bone, hand in K.HALVES:
        p = name[0]
        out.append(shape("top-" + p, bone, 6, 0.7, lrect(bone, hand, 0, K.TOP_Y0, K.HALF, K.TOP_Y1), **TOP))
        out.append(shape("apron-" + p, bone, 4, 0.55, lrect(bone, hand, K.LEG_X + 0.03, K.APRON_Y0, K.HALF, K.APRON_Y1), **APRON))
        out.append(shape("leg-" + p, bone, 5, 0.6, lrect(bone, hand, K.LEG_X - 0.03, 0, K.LEG_X + 0.03, K.TOP_Y0), **LEGS))
    out.append(shape("leaf", "leaf", 5.5, 0.65, rect(0, 0, 0.6, K.LEAF_T), **LEAF))
    return out


def front200():
    """🗺 1:200 front elevation."""
    h = K.DEPTH / 2
    out = [
        shape("top-r", "top-right", 6, 0.7, rect(-h, K.TOP_Y0, h, K.TOP_Y1), **TOP),
        shape("end-apron-r", "top-right", 4, 0.55, rect(-(K.LEG_Z - 0.03), K.APRON_Y0, K.LEG_Z - 0.03, K.APRON_Y1), **APRON),
        shape("leaf", "leaf", 3, 0.65, rect(-h, K.GAP_Y0, h, K.TOP_Y0), **LEAF),
    ]
    return out + front_legs(0.03, 0.03, K.TAPER_Y, K.TOP_Y0, None, False)


def elevation500():
    """🌍 1:500 side elevation."""
    out = []
    for name, bone, hand in K.HALVES:
        out.append(shape("slab-" + name, bone, 6, 0.7, lpoly(bone, hand, [(0, K.SLAB_Y0), (0, K.TOP_Y1), (K.HALF, K.TOP_Y1), (K.HALF, K.FRAME500_Y0), (0.1, K.FRAME500_Y0), (0.1, K.SLAB_Y0)]), regions=["table-top", "apron"], material=K.OAK))
        out.append(shape("leg-" + name[0], bone, 3, 0.3, lrect(bone, hand, K.LEG_X - 0.0375, 0, K.LEG_X + 0.0375, K.FRAME500_Y0), **LEGS))
    out.append(shape("leaf-bar", "leaf", 5, 0.6, rect(0, -0.045, 0.6, K.LEAF_T), regions=["leaf-top", "slide-rails"], material=K.OAK))
    return out


def front500():
    """🌍 1:500 front elevation."""
    h = K.DEPTH / 2
    out = [
        shape("slab-right", "top-right", 6, 0.7, poly([(-h, K.SLAB_Y0), (-h, K.TOP_Y1), (h, K.TOP_Y1), (h, K.SLAB_Y0), (0.4, K.SLAB_Y0), (0.4, K.FRAME500_Y0), (-0.4, K.FRAME500_Y0), (-0.4, K.SLAB_Y0)]), regions=["table-top", "apron"], material=K.OAK),
        shape("leaf-bar", "leaf", 5, 0.6, rect(-K.LEAF500_Z, 0.645, K.LEAF500_Z, K.TOP_Y0), regions=["leaf-top", "slide-rails"], material=K.OAK),
    ]
    for k, c in enumerate((K.LEG_Z, -K.LEG_Z)):
        out.append(shape(f"leg-{k}", "top-right", 3, 0.3, rect(c - 0.0375, 0, c + 0.0375, K.FRAME500_Y0), **LEGS))
    return out


def plan500():
    """🌍 1:500 plan."""
    h = K.DEPTH / 2
    return [
        shape("slab-left", "top-left", 6, 0.7, rect(0, -h, K.HALF, h), regions=["table-top", "apron"], material=K.OAK),
        shape("slab-right", "top-right", 6, 0.7, rect(K.HALF, -h, K.LENGTH, h), regions=["table-top", "apron"], material=K.OAK),
        shape("leaf-bar", "leaf", 5, 0.6, rect(K.LEAF_X0, -K.LEAF500_Z, K.LEAF_X1, K.LEAF500_Z), regions=["leaf-top", "slide-rails"], material=K.OAK),
    ]


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": T("Table 32 x 15 mm with its whole construction: the top halves with the underside bevel at the free ends, the 30 mm shadow gap with the oak blocks and the stored leaf, the aprons with grain and the legs tapered from the apron down. Front view: the right end with the bevelled top, the gap with the end blocks and the leaf end behind them, the end apron and the two tapered legs. Plan: the two halves with their stave joints; the leaf with its own joints lies under them and shows when they part.",
                        "Tisch 32 x 15 mm mit seiner ganzen Konstruktion: die Plattenhälften mit der Unterfase an den freien Enden, die 30-mm-Schattenfuge mit den Eichenklötzen und der verstauten Einlage, die Zargen mit Maserung und die von der Zarge abwärts verjüngten Beine. Vorderansicht: das rechte Ende mit der gefasten Platte, die Fuge mit den Stirnklötzen und dahinter der Stirnseite der Einlage, die Stirnzarge und die zwei verjüngten Beine. Grundriss: die beiden Hälften mit ihren Lamellenfugen; die Einlage mit eigenen Fugen liegt darunter und zeigt sich, wenn die Hälften auseinandergehen."),
        "retained": T("The 15 mm bevel and its crease, the shadow gap, the gap blocks, the taper start of the legs, the stave joints and the slide centre bar that shows under the leaf when the table is extended.",
                      "Die 15-mm-Fase und ihre Kante, die Schattenfuge, die Fugenklötze, der Verjüngungsansatz der Beine, die Lamellenfugen und der Mittelstab des Auszugs, der bei ausgezogenem Tisch unter der Einlage sichtbar wird."),
        "merged": T("Front and rear legs, aprons and blocks coincide in the side and front views.", "Vordere und hintere Beine, Zargen und Klötze fallen in Seiten- und Vorderansicht zusammen."),
        "omitted": T("Runners behind the aprons, screws, felt glides, dowels and table locks.", "Laufschienen hinter den Zargen, Schrauben, Filzgleiter, Dübel und Tischverbinder."),
        "thinElements": T("Top, leaf and gap at their real 30 mm (0.6 mm); the 8 mm centre bar as a 17 mm filled strip.", "Platte, Einlage und Fuge in wahren 30 mm (0,6 mm); der 8-mm-Mittelstab als 17 mm hoher gefüllter Streifen."),
        "hatching": T("Grain lines 0.6 / 0.5 mm spacing, 0.13 mm pen on aprons and legs.", "Maserungslinien 0,6 / 0,5 mm Abstand, Stift 0,13 mm an Zargen und Beinen."),
        "styles": T("Top 0.7, leaf 0.65, legs 0.6, aprons 0.55, blocks 0.45 (in the shadow), steel 0.35.", "Platte 0,7, Einlage 0,65, Beine 0,6, Zargen 0,55, Klötze 0,45 (im Schatten), Stahl 0,35."),
        "overlays": T("One shape per region; hatched shapes keep their region; the blocks belong to the aprons.", "Eine Form je Region; schraffierte Formen behalten ihre Region; die Klötze gehören zu den Zargen."),
    },
    "100": {
        "silhouette": T("Table 16 x 7.5 mm: square-edged top halves, the open shadow gap with the stored leaf, aprons, tapered legs and the slide that shows under the leaf when extended. Front view: the right end with top, gap and leaf end, end apron and the two tapered legs. Plan: the two halves with stave joints and the leaf below them.",
                        "Tisch 16 x 7,5 mm: Plattenhälften mit gerader Kante, die offene Schattenfuge mit der verstauten Einlage, Zargen, verjüngte Beine und der Auszug, der ausgezogen unter der Einlage sichtbar wird. Vorderansicht: das rechte Ende mit Platte, Fuge und Stirnseite der Einlage, Stirnzarge und die zwei verjüngten Beine. Grundriss: die beiden Hälften mit Lamellenfugen und die Einlage darunter."),
        "retained": T("Shadow gap, leaf, aprons, leg taper, stave joints.", "Schattenfuge, Einlage, Zargen, Beinverjüngung, Lamellenfugen."),
        "merged": T("The bevel into the square edge; runner and centre bar into one slide.", "Die Fase in die gerade Kante; Laufschiene und Mittelstab zu einem Auszug."),
        "omitted": T("Gap blocks, grain, taper crease, fittings.", "Fugenklötze, Maserung, Verjüngungskante, Beschläge."),
        "thinElements": T("Top, leaf and gap 0.3 mm each, drawn as filled shapes and an open band.", "Platte, Einlage und Fuge je 0,3 mm, als gefüllte Formen und offener Streifen gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Top 0.7, leaf 0.65, legs 0.6, aprons 0.55, slide 0.35.", "Platte 0,7, Einlage 0,65, Beine 0,6, Zargen 0,55, Auszug 0,35."),
        "overlays": T("One shape per region.", "Eine Form je Region."),
    },
    "200": {
        "silhouette": T("Table 8 x 3.8 mm: top halves, the shadow gap (in print closed by the pens), aprons and straight legs. Front view: top, leaf end in the gap, end apron, two legs. Plan: the two halves and the leaf below them.",
                        "Tisch 8 x 3,8 mm: Plattenhälften, die Schattenfuge (im Druck durch die Stifte geschlossen), Zargen und gerade Beine. Vorderansicht: Platte, Stirnseite der Einlage in der Fuge, Stirnzarge, zwei Beine. Grundriss: die beiden Hälften und die Einlage darunter."),
        "retained": T("Top, gap, aprons, legs, leaf and slide as own shapes so the extension reads when it opens.", "Platte, Fuge, Zargen, Beine, Einlage und Auszug als eigene Formen, damit sich das Ausziehen lesen lässt."),
        "merged": T("Front and rear legs coincide.", "Vordere und hintere Beine fallen zusammen."),
        "omitted": T("Leg taper (25 mm, below the 30 mm feature), bevel, blocks, stave joints.", "Beinverjüngung (25 mm, unter dem 30-mm-Merkmal), Fase, Klötze, Lamellenfugen."),
        "thinElements": T("Top, leaf and gap at the 0.15 mm print minimum.", "Platte, Einlage und Fuge im Druckminimum von 0,15 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Top 0.7, leaf 0.65, legs 0.6, aprons 0.55, slide 0.35.", "Platte 0,7, Einlage 0,65, Beine 0,6, Zargen 0,55, Auszug 0,35."),
        "overlays": T("One shape per region.", "Eine Form je Region."),
    },
    "500": {
        "silhouette": T("Table sign 3.2 x 1.5 mm: per half one slab for top, gap and apron with the apron set back at the end, two legs; the leaf bar lies hidden in the slabs until the halves part. Front view: the slab with the set-back apron and the two legs. Plan: two halves and the leaf between them.",
                        "Tischzeichen 3,2 x 1,5 mm: je Hälfte eine Platte für Platte, Fuge und Zarge mit der am Ende eingerückten Zarge, zwei Beine; der Einlegebalken liegt verdeckt in den Platten, bis die Hälften auseinandergehen. Vorderansicht: die Platte mit eingerückter Zarge und die zwei Beine. Grundriss: zwei Hälften und die Einlage dazwischen."),
        "retained": T("Top level 0.75 m, apron zone, leaf bar, legs.", "Plattenhöhe 0,75 m, Zargenzone, Einlegebalken, Beine."),
        "merged": T("Top, gap and aprons into one outline per half; leaf and slides into one bar.", "Platte, Fuge und Zargen zu einem Umriss je Hälfte; Einlage und Auszüge zu einem Balken."),
        "omitted": T("Taper, bevel, gap, rear legs.", "Verjüngung, Fase, Fuge, hintere Beine."),
        "thinElements": T("Legs 75 mm = 0.15 mm, filled at the print minimum.", "Beine 75 mm = 0,15 mm, gefüllt im Druckminimum."),
        "hatching": T("None.", "Keine."),
        "styles": T("Slabs 0.7, leaf bar 0.6, legs 0.3.", "Platten 0,7, Einlegebalken 0,6, Beine 0,3."),
        "overlays": T("Slabs list table-top and apron; the leaf bar lists leaf-top and slide-rails; legs carry their region.", "Platten führen table-top und apron; der Einlegebalken führt leaf-top und slide-rails; Beine tragen ihre Region."),
    },
}


def apply(asset):
    """🖊 Replaces the geometry and decisions of every scale's representation."""
    views = {
        "50": (elevation50(), front50(), plan_common(True)),
        "100": (elevation100(), front100(), plan_common(True)),
        "200": (elevation200(), front200(), plan_common(False)),
        "500": (elevation500(), front500(), plan500()),
    }
    for scale, (elevation, front, plan) in views.items():
        rep = asset["representations"][scale]
        rep["geometry"] = {"elevation": elevation, "front": front, "plan": plan}
        rep["decisions"] = DECISIONS[scale]
