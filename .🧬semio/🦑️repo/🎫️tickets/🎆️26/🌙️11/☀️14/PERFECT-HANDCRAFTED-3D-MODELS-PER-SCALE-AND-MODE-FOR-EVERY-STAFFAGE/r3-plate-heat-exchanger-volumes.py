"""🧊 Round 3, machinery/plate-heat-exchanger: every solid of the body, thermal and structure lists of 1:50, 1:100 and 1:200, stated by hand.

Usage: imported by `r3-plate-heat-exchanger-build.py` (`volumes()`).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("phx_geo", os.path.join(HERE, "r3-plate-heat-exchanger-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL = "galvanised-steel"
ALU = "aluminium-plate"


def solid(id, kind, bone, material, tone, role="body", **kw):
    """🧩 One body solid."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def box(id, bone, material, tone, cx, cy, cz, sx, sy, sz, **kw):
    """📦 A box given by its world rest centre and size."""
    u, v = G.loc(bone, cx, cy)
    return solid(id, "box", bone, material, tone, size=[sx, sy, sz], transform={"translation": [u, v, cz]}, **kw)


def ext(id, bone, material, tone, plane, profile, frm, to, holes=None, **kw):
    """🧱 An extrude; profile already in bone-local plane coordinates."""
    out = solid(id, "extrude", bone, material, tone, profile=profile, plane=plane, **{"from": frm, "to": to})
    if holes:
        out["holes"] = holes
    out.update(kw)
    return out


def xy(bone, pts):
    """↔ World (x, y) points into the bone-local xy plane."""
    return [G.loc(bone, x, y) for x, y in pts]


def rev(id, bone, material, tone, profile, origin_world, axis, **kw):
    """🌀 A revolve about an axis through a world rest point."""
    u, v = G.loc(bone, origin_world[0], origin_world[1])
    return solid(id, "revolve", bone, material, tone, profile=profile, axis=axis, origin=[u, v, origin_world[2]], **kw)


def ref(id, of, **kw):
    """🔗 A reference to a body solid."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def casing(s):
    """🗄️ The six panels of the casing at scale s: bottom, top with the bypass opening, two end walls with two spigot holes each, back and front (inspection cover)."""
    t = G.walls(s)
    h, b, top = G.HALF, G.BASE, G.TOP
    o = G.OPENING
    end_holes = [G.circle(round(y - G.RIG["frame"][1], 6), 0.0, G.SPIGOT_R) for y in G.SPIGOT_Y]
    yz = [[b, -(h - t)], [top, -(h - t)], [top, h - t], [b, h - t]]
    fx = lambda x: round(x - G.RIG["frame"][0], 6)
    out = [
        box("casing-bottom", "frame", STEEL, 0.35, 0.0, b + t / 2, 0.0, 2 * (h - t), t, 2 * (h - t)),
        ext("casing-top", "frame", STEEL, 0.35, "zx", [[-(h - t), fx(-(h - t))], [h - t, fx(-(h - t))], [h - t, fx(h - t)], [-(h - t), fx(h - t)]], top - t, top, holes=[[[-o, fx(-o)], [-o, fx(o)], [o, fx(o)], [o, fx(-o)]]]),
        ext("casing-end-rear", "frame", STEEL, 0.35, "yz", yz, fx(-h), fx(-h + t), holes=end_holes),
        ext("casing-end-front", "frame", STEEL, 0.35, "yz", yz, fx(h - t), fx(h), holes=end_holes),
        box("casing-back", "frame", STEEL, 0.35, 0.0, (b + top) / 2, h - t / 2, 2 * h, top - b, t),
        box("casing-cover", "frame", STEEL, 0.38, 0.0, (b + top) / 2, -h + t / 2, 2 * h, top - b, t),
    ]
    return out


def base_rails(s):
    """🛤️ Two base rails along z: inverted U channels of 2 mm galvanised steel at 1:50, solid bars of the rail section at 1:100 and 1:200."""
    w, hh = G.RAIL_W / 2, G.RAIL_H
    out = []
    for name, x in (("rear", -G.RAIL_X), ("front", G.RAIL_X)):
        if s == 50:
            prof = [[x - w, 0.0], [x - w + 0.002, 0.0], [x - w + 0.002, hh - 0.002], [x + w - 0.002, hh - 0.002], [x + w - 0.002, 0.0], [x + w, 0.0], [x + w, hh], [x - w, hh]]
        else:
            prof = G.rect(x - w, 0.0, x + w, hh)
        out.append(ext(f"base-rail-{name}", "frame", STEEL, 0.25, "xy", xy("frame", prof), -G.HALF, G.HALF))
    return out


def seals(s):
    """🔧 The cradle, the top rail and (1:50) the two side seal rails that hold the core corners and separate the four chambers."""
    if s == 200:
        return []
    t = G.walls(s)
    span = (-(G.HALF - t), G.HALF - t)
    out = [
        ext("core-cradle", "frame", STEEL, 0.3, "xy", xy("frame", G.cradle(s)), *span),
        ext("core-top-rail", "frame", STEEL, 0.3, "xy", xy("frame", G.top_rail(s)), *span),
    ]
    if s == 50:
        out += [
            ext("seal-rail-rear", "frame", STEEL, 0.3, "xy", xy("frame", G.side_rail(-1)), *span),
            ext("seal-rail-front", "frame", STEEL, 0.3, "xy", xy("frame", G.side_rail(1)), *span),
        ]
    return out


def core_solids(s):
    """💠 The plate core: the pack between two end plates and four corner profiles at 1:50; one prism at 1:100 and 1:200."""
    t = G.walls(s)
    a, b = G.core(s)
    inner = G.HALF - t
    pts = xy("plates", G.diamond(a, b))
    if s != 50:
        return [ext("plate-pack", "plates", ALU, 0.7, "xy", pts, -inner, inner)]
    end = 0.0075
    c = 0.015
    corner = xy("plates", [[0.0, G.CORE_Y - b - c], [c, G.CORE_Y - b], [0.0, G.CORE_Y - b + c], [-c, G.CORE_Y - b]])
    centre = G.loc("plates", 0.0, G.CORE_Y)
    return [
        ext("plate-pack", "plates", ALU, 0.72, "xy", pts, -inner + end, inner - end),
        ext("core-end-plate", "plates", ALU, 0.62, "xy", pts, -inner, -inner + end, repeat={"count": 2, "translation": [0, 0, 2 * inner - end]}),
        ext("core-corner-profile", "plates", ALU, 0.55, "xy", corner, -inner, inner, repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": math.pi / 2, "origin": [centre[0], centre[1], 0]}),
    ]


def spigot_profile(s):
    """🌀 Meridian loop of a spigot (axial from the end wall's outer face, radial): 1 mm galvanised tube, with a 15 mm flange of one feature at 1:50."""
    t = G.walls(s)
    r, ri, out = G.SPIGOT_R, G.SPIGOT_R - G.SPIGOT_WALL, G.SPIGOT_OUT
    if s == 50:
        f = 0.0075
        loop = [[-t, ri], [out, ri], [out, r], [f, r], [f, G.FLANGE_R], [0.0, G.FLANGE_R], [0.0, r], [-t, r]]
    else:
        loop = [[-t, ri], [out, ri], [out, r], [-t, r]]
    return loop + [loop[0]]


SPIGOTS = [("collar-upper-left", -1, 1), ("collar-lower-left", -1, 0), ("collar-upper-right", 1, 1), ("collar-lower-right", 1, 0)]


def spigots(s):
    """🕳️ The four DN 200 spigots, each on its chamber of the end walls."""
    prof = spigot_profile(s)
    return [rev(id, "ports", STEEL, 0.42, prof, [side * G.HALF, G.SPIGOT_Y[up], 0.0], [side, 0, 0]) for id, side, up in SPIGOTS]


def damper(s):
    """🚪 The bypass flap: a blade lying on the lid and its 20 mm hinge barrel (1:50), one profile of blade and barrel (1:100, 1:200)."""
    r = G.BARREL_R
    prof = G.damper_profile(s)
    out = [ext("damper", "damper", STEEL, 0.3, "xy", prof, -G.FLAP_HALF_Z, G.FLAP_HALF_Z)]
    if s == 50:
        out.append(solid("damper-hinge", "capsule", "damper", STEEL, 0.15, role="detail", slender=True, p0=[0, 0, -G.FLAP_HALF_Z], p1=[0, 0, G.FLAP_HALF_Z], r0=r, r1=r))
    return out


def details50():
    """🔩 1:50 only: hinge leaf on the top panel, the condensate drain spigot under the exhaust chamber, four quarter-turn fasteners of the inspection cover."""
    t = G.walls(50)
    return [
        box("hinge-leaf", "frame", STEEL, 0.2, -0.2175, G.TOP + 0.00375, 0.0, 0.025, 0.0075, 2 * G.FLAP_HALF_Z),
        rev("condensate-drain", "frame", STEEL, 0.3, [[-t, 0.0095], [G.BASE - 0.0075, 0.0095], [G.BASE - 0.0075, 0.011], [-t, 0.011], [-t, 0.0095]], [-0.12, G.BASE, 0.0], [0, -1, 0]),
        rev("cover-fastener-low", "frame", STEEL, 0.15, [[0.0, 0.0], [0.011, 0.0], [0.011, 0.01], [0.0, 0.01]], [-0.19, 0.1, -(G.HALF - t)], [0, 0, -1], repeat={"count": 2, "translation": [0.38, 0, 0]}),
        rev("cover-fastener-high", "frame", STEEL, 0.15, [[0.0, 0.0], [0.011, 0.0], [0.011, 0.01], [0.0, 0.01]], [-0.19, 0.53, -(G.HALF - t)], [0, 0, -1], repeat={"count": 2, "translation": [0.38, 0, 0]}),
    ]


def body(s):
    """🧍 The body list of scale s."""
    out = casing(s) + base_rails(s) + seals(s) + core_solids(s) + spigots(s) + damper(s)
    if s == 50:
        out += details50()
    return out


def outlets(s):
    """💨 The two vent emitters as outlet discs flush in the mouths of the supply (lower right) and exhaust (lower left) spigots (D2)."""
    t = G.FEATURE[s]
    ri = G.SPIGOT_R - G.SPIGOT_WALL
    out = []
    for id, tag, side in (("th-supply-air-outlet", "supply-air", 1), ("th-exhaust-air-outlet", "exhaust-air", -1)):
        mouth = side * (G.HALF + G.SPIGOT_OUT)
        d = rev(id, "ports", STEEL, 0.9, [[-t, 0.0], [0.0, 0.0], [0.0, ri], [-t, ri]], [mouth, G.SPIGOT_Y[0], 0.0], [side, 0, 0], tags=[tag])
        d["region"] = "ports"
        out.append(d)
    return out


def ports_thermal(s):
    """🌡️ The spigot surfaces in the room: the tube as a filled body (its bore carries the duct air, not room air) whose mouth and inner end are section faces where the duct run continues (C25 a), and at 1:50 the flange ring on the end wall."""
    t = G.walls(s)
    r = G.SPIGOT_R
    tube = [[-t, 0.0], [G.SPIGOT_OUT, 0.0], [G.SPIGOT_OUT, r], [-t, r]]
    flange = [[0.0, r], [0.0075, r], [0.0075, G.FLANGE_R], [0.0, G.FLANGE_R], [0.0, r]]
    out = []
    for id, side, up in SPIGOTS:
        origin = [side * G.HALF, G.SPIGOT_Y[up], 0.0]
        d = rev("th-" + id, "ports", STEEL, 0.42, tube, origin, [side, 0, 0], sectionFaces=["-u", "+u"])
        d["region"] = "ports"
        out.append(d)
        if s == 50:
            f = rev("th-" + id + "-flange", "ports", STEEL, 0.42, flange, origin, [side, 0, 0])
            f["region"] = "ports"
            out.append(f)
    return out


INNER = {"casing-bottom": ["+v"], "casing-top": ["-v"], "casing-end-rear": ["+u"], "casing-end-front": ["-u"], "casing-back": ["-w"], "casing-cover": ["+w"]}
INSIDE = {"core-cradle", "core-top-rail", "seal-rail-rear", "seal-rail-front"}


REGION = {"frame": "frame", "plates": "plates", "ports": "ports", "damper": "damper"}


def thermal(s, bodies):
    """🌡️ The thermal list: references for the casing (its inner faces face the duct air in the closed casing and are marked, C31), base rails, details, core and flap; the steel seals inside the airway are not room surfaces and stay out; own filled spigots; the two outlets."""
    out = []
    for b in bodies:
        if b["bone"] == "ports" or b["id"] in INSIDE:
            continue
        d = ref("th-" + b["id"], b["id"], region=REGION[b["bone"]])
        if b["id"] in INNER:
            d["sectionFaces"] = INNER[b["id"]]
        out.append(d)
    return out + ports_thermal(s) + outlets(s)


def st(d, member, part, **kw):
    """🏗️ Turns a body-like solid into a structural solid."""
    d = dict(d)
    d["member"] = member
    d["part"] = part
    d.update(kw)
    return d


def structure(s):
    """🏗️ The load path: base rails on the floor, bottom panel, folded corners and top edge of the casing, cradle and top rail, the core frame of end plates and corner profiles, spigots cantilevered from their flanges, the flap on its hinge; supports under the rails, masses at the centres of mass."""
    t = G.walls(s)
    h = G.HALF
    col = {50: 0.02, 100: 0.03, 200: 0.045}[s]
    out = [
        ref("st-base-rail-rear", "base-rail-rear", member="frame", part="member"),
        ref("st-base-rail-front", "base-rail-front", member="frame", part="member"),
        ref("st-casing-bottom", "casing-bottom", member="frame", part="member"),
        st(box("st-corner-post-rear", "frame", STEEL, 0.35, -h + col / 2, (G.BASE + G.TOP) / 2, -h + col / 2, col, G.TOP - G.BASE, col, repeat={"count": 2, "translation": [0, 0, 2 * h - col]}), "frame", "member"),
        st(box("st-corner-post-front", "frame", STEEL, 0.35, h - col / 2, (G.BASE + G.TOP) / 2, -h + col / 2, col, G.TOP - G.BASE, col, repeat={"count": 2, "translation": [0, 0, 2 * h - col]}), "frame", "member"),
        st(ext("st-top-edge", "frame", STEEL, 0.35, "zx", [[-h, 0.0], [h, 0.0], [h, 2 * h], [-h, 2 * h]], G.TOP - col, G.TOP, holes=[[[-h + col, col], [-h + col, 2 * h - col], [h - col, 2 * h - col], [h - col, col]]]), "frame", "member"),
        ref("st-core-cradle", "core-cradle", member="frame", part="member"),
        ref("st-core-top-rail", "core-top-rail", member="frame", part="member"),
    ]
    a, b = G.core(s)
    centre = G.loc("plates", 0.0, G.CORE_Y)
    if s == 50:
        out += [
            ref("st-core-end-plate", "core-end-plate", member="plates", part="member"),
            ref("st-core-corner-profile", "core-corner-profile", member="plates", part="member"),
        ]
    else:
        c = {100: 0.02, 200: 0.035}[s]
        bar = xy("plates", [[0.0, G.CORE_Y - b - c / 2], [c / 2, G.CORE_Y - b + c / 2], [0.0, G.CORE_Y - b + 1.5 * c], [-c / 2, G.CORE_Y - b + c / 2]])
        out.append(st(ext("st-core-corner-bar", "plates", ALU, 0.6, "xy", bar, -(h - t), h - t, repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": math.pi / 2, "origin": [centre[0], centre[1], 0]}), "plates", "member"))
    seat = {50: 0.03, 100: 0.04, 200: 0.06}[s]
    out.append(st(box("st-core-seat", "plates", ALU, 0.5, 0.0, G.CORE_Y - b + seat / 3, 0.0, seat, seat, 2 * (h - t)), "plates", "joint"))
    out += [ref("st-" + id, id, member="ports", part="member") for id, _, _ in SPIGOTS]
    ring = [[0.0, G.SPIGOT_R - 0.004], [0.0075 if s == 50 else G.FEATURE[s], G.SPIGOT_R - 0.004], [0.0075 if s == 50 else G.FEATURE[s], G.FLANGE_R - (0 if s == 50 else 0.005)], [0.0, G.FLANGE_R - (0 if s == 50 else 0.005)]]
    ring = ring + [ring[0]]
    for id, up in (("st-spigot-flange-upper", 1), ("st-spigot-flange-lower", 0)):
        out.append(st(rev(id, "ports", STEEL, 0.4, ring, [G.HALF, G.SPIGOT_Y[up], 0.0], [1, 0, 0], repeat={"count": 2, "rotationAxis": [0, 1, 0], "rotationRad": math.pi, "origin": [-G.RIG["ports"][0], 0, 0]}), "ports", "joint"))
    out.append(ref("st-damper", "damper", member="damper", part="member"))
    r = G.BARREL_R
    out.append(st(solid("st-damper-hinge", "capsule", "damper", STEEL, 0.15, slender=True, p0=[0, 0, -G.FLAP_HALF_Z + r], p1=[0, 0, G.FLAP_HALF_Z - r], r0=r, r1=r), "damper", "joint"))
    m = {50: (0.06, 0.04), 100: (0.07, 0.05), 200: (0.09, 0.06)}[s]
    out.append(st(solid("mass-plates", "ellipsoid", "plates", ALU, 0.7, radii=[m[0]] * 3, transform={"translation": [*centre, 0.0]}), "plates", "mass"))
    fc = G.loc("frame", 0.0, 0.3)
    out.append(st(solid("mass-frame", "ellipsoid", "frame", STEEL, 0.35, radii=[m[1]] * 3, transform={"translation": [*fc, 0.0]}), "frame", "mass"))
    pad = G.FEATURE[s]
    out.append(st(box("base-contact-pad", "frame", STEEL, 0.4, -G.RAIL_X, pad / 2, 0.0, G.RAIL_W, pad, 2 * h, repeat={"count": 2, "translation": [2 * G.RAIL_X, 0, 0]}), "frame", "support", anchor="base-contact"))
    return out


DECISIONS = {
    50: [
        {"en": "Detailed model of the real exchanger section: a galvanised casing of six panels (the 1 mm sheet drawn at the 7.5 mm smallest feature) with the bypass opening of 0.36 m in the top panel and two DN 200 spigot holes in each end wall, the front panel as the inspection cover with four quarter-turn fasteners, on two inverted U-channel base rails of 2 mm steel; inside, the cross-flow core of 0.318 m square aluminium plates stands on one corner: the plate pack between two end plates, four corner profiles, seated in a V cradle, held by a top rail and two side seal rails that divide the casing into the four chambers.", "de": "Detailliertes Modell des realen Übertragerteils: ein verzinktes Gehäuse aus sechs Blechen (das 1-mm-Blech im kleinsten Merkmal von 7,5 mm) mit der 0,36 m großen Bypassöffnung im Deckel und zwei DN-200-Stutzenlöchern in jeder Stirnwand, die vordere Wand als Revisionsdeckel mit vier Schnellverschlüssen, auf zwei umgekehrten U-Profil-Fußschienen aus 2-mm-Stahl; innen steht der Kreuzstromkern aus 0,318 m großen quadratischen Aluminiumplatten auf einer Ecke: das Plattenpaket zwischen zwei Endplatten, vier Eckprofile, in einer V-Wiege, gehalten von einer Deckschiene und zwei seitlichen Dichtschienen, die das Gehäuse in die vier Kammern teilen."},
        {"en": "Spigots are 1 mm galvanised tubes of 200 mm with a flange on the end wall, 60 mm proud: outdoor air enters upper left, exhaust leaves lower left, extract enters upper right and supply leaves lower right, so both streams cross the diamond core diagonally. The flap is a 0.4 m blade on a 20 mm hinge barrel over a hinge leaf; the condensate drain leaves the exhaust chamber between the rails.", "de": "Die Stutzen sind 200-mm-Rohre aus 1-mm-verzinktem Blech mit Flansch an der Stirnwand und 60 mm Überstand: Außenluft tritt oben links ein, Fortluft unten links aus, Abluft oben rechts ein und Zuluft unten rechts aus, so dass beide Ströme den auf der Ecke stehenden Kern diagonal kreuzen. Die Klappe ist ein 0,4-m-Blatt auf einem 20-mm-Scharnierrohr über einem Scharnierlappen; der Kondensatablauf verlässt die Fortluftkammer zwischen den Schienen."},
        {"en": "Thermal: the casing, rails, seals and flap are references to their body solids; the spigots are their outside surfaces as filled bodies (the bore belongs to the duct air, mouth and inner end are section faces where the duct run continues), which also closes the casing so that its inside does not count as outer area (C31); the plate pack is an exchange area (coverage only); the two vent emitters are outlet discs flush in the supply and exhaust mouths.", "de": "Thermik: Gehäuse, Schienen, Dichtungen und Klappe sind Verweise auf ihre Körper; die Stutzen sind ihre Außenflächen als gefüllte Körper (die Bohrung gehört der Kanalluft, Mündung und inneres Ende sind Schnittflächen, an denen der Kanal weiterläuft), wodurch das Gehäuse geschlossen ist und sein Inneres nicht als Außenfläche zählt (C31); das Plattenpaket ist eine Austauschfläche (nur Abdeckung); die beiden Lüftungsemitter sind bündige Auslassscheiben in den Mündungen von Zuluft und Fortluft."},
        {"en": "Structure: base rails on two contact pads, the bottom panel, the four folded casing corners as 20 mm posts and the top edge as a ring, the cradle and top rail that carry and locate the core, the core frame of end plates and corner profiles, the spigots cantilevered from their flanges (joints), the flap on its hinge joint, and the masses of core and casing at their centres.", "de": "Tragwerk: Fußschienen auf zwei Kontaktflächen, das Bodenblech, die vier gekanteten Gehäuseecken als 20-mm-Pfosten und die Oberkante als Ring, Wiege und Deckschiene, die den Kern tragen und führen, der Kernrahmen aus Endplatten und Eckprofilen, die an ihren Flanschen (Gelenke) auskragenden Stutzen, die Klappe an ihrem Scharniergelenk sowie die Massen von Kern und Gehäuse in ihren Schwerpunkten."},
    ],
    100: [
        {"en": "Redrawn for 1:100 (15 mm feature): the six casing panels at 15 mm, two solid base rails of the 40 by 30 mm rail section, the core as one diamond prism whose side corners reach the end walls (the side seal rails of 1:50 are below the feature and merged into the core), cradle and top rail kept, spigots as plain 1 mm tubes without flange, the flap as one profile of blade and hinge barrel; hinge leaf, drain and fasteners are omitted.", "de": "Für 1:100 neu gezeichnet (Merkmal 15 mm): die sechs Gehäusebleche mit 15 mm, zwei volle Fußschienen im Schienenquerschnitt 40 mal 30 mm, der Kern als ein Rautenprisma, dessen Seitenecken die Stirnwände erreichen (die seitlichen Dichtschienen von 1:50 liegen unter dem Merkmal und gehen im Kern auf), Wiege und Deckschiene bleiben, Stutzen als glatte 1-mm-Rohre ohne Flansch, die Klappe als ein Profil aus Blatt und Scharnierrohr; Scharnierlappen, Ablauf und Verschlüsse entfallen."},
        {"en": "Thermal: references for casing, rails, core and flap, filled spigot bodies with section faces at mouth and inner end, outlet discs of 15 mm in the supply and exhaust mouths. Structure: rails, bottom panel, 30 mm corner posts and top edge ring, cradle and top rail, the core as four 20 mm corner bars with its seat joint, the spigots with flange joints, the flap with its hinge, two masses and the contact pads.", "de": "Thermik: Verweise für Gehäuse, Schienen, Kern und Klappe, gefüllte Stutzenkörper mit Schnittflächen an Mündung und innerem Ende, 15-mm-Auslassscheiben in den Mündungen von Zuluft und Fortluft. Tragwerk: Schienen, Bodenblech, 30-mm-Eckpfosten und Kantenring oben, Wiege und Deckschiene, der Kern als vier 20-mm-Eckstäbe mit Auflagergelenk, die Stutzen mit Flanschgelenken, die Klappe mit Scharnier, zwei Massen und die Kontaktflächen."},
    ],
    200: [
        {"en": "Generalised for 1:200 (30 mm feature): the casing panels at 30 mm, the two base rails as 40 by 30 mm bars, the core as one rhombic prism that spans the inside of the casing from floor to lid and wall to wall (cradle, top rail and side seals merged into it), four plain spigot tubes, the flap as one 20 mm slab with the hinge, drawn as thick as the 2D draws it.", "de": "Für 1:200 verallgemeinert (Merkmal 30 mm): die Gehäusebleche mit 30 mm, die beiden Fußschienen als Stäbe von 40 mal 30 mm, der Kern als ein rautenförmiges Prisma, das das Gehäuseinnere von Boden bis Deckel und Wand bis Wand ausfüllt (Wiege, Deckschiene und Seitendichtungen gehen darin auf), vier glatte Stutzenrohre, die Klappe als eine 20-mm-Platte mit Scharnier, so dick wie die 2D-Zeichnung."},
        {"en": "Thermal: references for casing, rails, core and flap, filled spigot bodies, outlet discs of 30 mm. Structure: rails, bottom panel, 45 mm corner posts and top ring, four 35 mm core corner bars on their seat, spigots with flange joints, the flap on its hinge, two masses and the contact pads.", "de": "Thermik: Verweise für Gehäuse, Schienen, Kern und Klappe, gefüllte Stutzenkörper, 30-mm-Auslassscheiben. Tragwerk: Schienen, Bodenblech, 45-mm-Eckpfosten und Ring oben, vier 35-mm-Eckstäbe des Kerns auf ihrem Auflager, Stutzen mit Flanschgelenken, die Klappe an ihrem Scharnier, zwei Massen und die Kontaktflächen."},
    ],
}


def scale_volume(s):
    """📐 One authored scale."""
    b = body(s)
    st_list = structure(s)
    ids = {x["id"] for x in b}
    st_list = [x for x in st_list if "of" not in x or x["of"] in ids]
    return {"status": "authored", "denominator": s, "body": b, "thermal": thermal(s, b), "structure": st_list, "decisions": DECISIONS[s]}


def volumes():
    """📦 The `volumes` member."""
    return {"50": scale_volume(50), "100": scale_volume(100), "200": scale_volume(200), "500": {"status": "unavailable", "denominator": 500}}
