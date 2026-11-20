"""🌡 Round 3, furniture/cafe-chair: the thermal and structural lists and the volume decisions of every scale. Thermal: every beech and steel body solid is its region's surface (glides and caps are nylon, no region). Structure: the real load path (tubes, seat board and rails, back board and rail), welds, bends and screwed fixings as joints, one glide pad per contact anchor, masses at the centres of the parts.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cafe_chair_kit", os.path.join(HERE, "r3-cafe-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

REGION = {"seat-board": "seat-beech", "seat-slab": "seat-beech", "rail-side-l": "seat-beech", "rail-side-r": "seat-beech", "rail-front": "seat-beech", "rail-back": "seat-beech", "apron": "seat-beech", "back-board": "back-beech", "back-rail": "back-beech", "back-slab": "back-beech"}
LAYER = {"seat-beech": "beech", "back-beech": "beech", "steel-tube": "tube-wall"}
MEMBER = {"rear-tube": "leg-rear", "post": "back", "front-tube": "leg-front", "stretcher": "stretcher", "seat": "seat", "rail": "seat", "apron": "seat", "back": "back"}
ANCHORS = (("contact-rear-left", "leg-rear", K.REAR_FOOT, K.REAR_FOOT_Z), ("contact-rear-right", "leg-rear", K.REAR_FOOT, -K.REAR_FOOT_Z), ("contact-front-left", "leg-front", K.FRONT_FOOT, K.FRONT_FOOT_Z), ("contact-front-right", "leg-front", K.FRONT_FOOT, -K.FRONT_FOOT_Z))
JOINT_TONE, MASS_TONE, SUPPORT_TONE = 0.25, 0.4, 0.2


def region_of(solid):
    """🏷 The thermal region of a body solid, or None for the nylon parts."""
    if solid["material"] == K.NYLON:
        return None
    return REGION.get(solid["id"], "steel-tube")


def thermal(scale, body):
    """🌡 Every beech and steel body solid as a reference into its region (with its one layer at 1:50)."""
    out = []
    for solid in body:
        region = region_of(solid)
        if region is None:
            continue
        ref = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
        if scale == "50":
            ref["layer"] = LAYER[region]
        out.append(ref)
    return out


def member_of(sid):
    """🦴 The member bone of a body solid id."""
    for prefix, bone in MEMBER.items():
        if sid.startswith(prefix):
            return bone
    raise KeyError(sid)


def ref(solid, member=None):
    """🔗 A member that is the body solid itself."""
    return {"id": "m-" + solid["id"], "of": solid["id"], "member": member or member_of(solid["id"]), "part": "member"}


def own(solid, member, part, **more):
    """🔩 An own structural solid."""
    solid.update(member=member, part=part, **more)
    return solid


def supports(radius, height):
    """🟢 One contact pad per anchor: the glide footprint, standing on the floor about the vertical through the foot."""
    profile = [(0.0, 0.0), (0.0, radius), (height, radius), (height, 0.0)]
    return [own(K.revolve("support-" + anchor, bone, K.NYLON, profile, (0, 1, 0), (foot[0], 0.0, z), tone=SUPPORT_TONE, role="detail"), bone, "support", anchor=anchor) for anchor, bone, foot, z in ANCHORS]


def tube_masses(radius, posts=True, stretch=True):
    """🟣 One mass at the middle of every tube (the pairs of a bone have their common centre in the air between the two tubes, so each tube carries its own)."""
    out = []
    for p, side in K.SIDES:
        a = K.rear_axis(K.ARC_A[1] / 2)
        out.append(own(K.ellipsoid(f"mass-rear-{p}", "leg-rear", K.STEEL, (a[0], a[1], side * a[2]), (radius, radius, radius), tone=MASS_TONE, role="detail"), "leg-rear", "mass"))
        f = K.front_axis(K.FRONT_TOP[1] / 2)
        out.append(own(K.ellipsoid(f"mass-front-{p}", "leg-front", K.STEEL, (f[0], f[1], side * f[2]), (radius, radius, radius), tone=MASS_TONE, role="detail"), "leg-front", "mass"))
        if stretch:
            s0, s1 = K.STRETCH_REAR, K.STRETCH_FRONT
            out.append(own(K.ellipsoid(f"mass-stretcher-{p}", "stretcher", K.STEEL, ((s0[0] + s1[0]) / 2, K.STRETCH_Y, side * (s0[2] + s1[2]) / 2), (radius, radius, radius), tone=MASS_TONE, role="detail"), "stretcher", "mass"))
        if posts:
            m = K.add(K.CORNER, K.POST_DIR, K.POST_L / 2)
            out.append(own(K.ellipsoid(f"mass-post-{p}", "back", K.STEEL, (m[0], m[1], side * K.POST_Z), (radius, radius, radius), tone=MASS_TONE, role="detail"), "back", "mass"))
    return out


def seat_mass(ru, rv, v=None):
    """🟣 The seat board and apron frame: their common centre lies in the board, on the centre line (at 1:500 in the middle of the merged slab)."""
    m = K.masses()["seat"]
    return own(K.box("mass-seat", "seat", K.STEEL, (m[1], max(m[2], -K.BOARD_T + rv / 2) if v is None else v, 0.0), (2 * ru, rv, 2 * ru), tone=MASS_TONE, role="detail"), "seat", "mass")


def back_masses(ru, rv, rail=True):
    """🟣 The back board and the back rail at their own centres (between the posts, on the curve)."""
    out = [own(K.box("mass-back-board", "back", K.STEEL, ((K.BOARD_U0 + K.BOARD_U1) / 2, K.back_vc(0.0), 0.0), (2 * ru, rv, 2 * ru), tone=MASS_TONE, role="detail"), "back", "mass")]
    if rail:
        out.append(own(K.box("mass-back-rail", "back", K.STEEL, ((K.RAIL_U0 + K.RAIL_U1) / 2, K.back_vc(0.0), 0.0), (2 * ru, rv, 2 * ru), tone=MASS_TONE, role="detail"), "back", "mass"))
    return out


def bends(radii):
    """🟠 The bend of each rear tube from leg into post: the rigid frame corner between leg-rear and back."""
    return [own(K.ellipsoid(f"joint-bend-{p}", "back", K.STEEL, (K.CORNER[0], K.CORNER[1], side * K.POST_Z), radii, tone=JOINT_TONE, role="detail"), "back", "joint") for p, side in K.SIDES]


def welds(radius):
    """🟠 The welds of the stretchers on the legs."""
    out = []
    for p, side in K.SIDES:
        for end, q in (("rear", K.STRETCH_REAR), ("front", K.STRETCH_FRONT)):
            out.append(own(K.ellipsoid(f"joint-weld-{end}-{p}", "stretcher", K.STEEL, (q[0], q[1], side * q[2]), (radius, radius, radius), tone=JOINT_TONE, role="detail"), "stretcher", "joint"))
    return out


def seat_fixings(ru, rv, rw):
    """🟠 The screwed tab fixings of the seat frame on the four tubes, at the rail ends."""
    out = []
    vm = (K.APRON_V0 + K.APRON_V1) / 2
    for p, side in K.SIDES:
        for end, u, w in (("rear", K.U_POST, K.rear_axis(0.41)[2]), ("front", K.U_FRONT, K.front_axis(0.41)[2])):
            out.append(own(K.box(f"joint-seat-{end}-{p}", "seat", K.STEEL, (u, vm, side * w), (2 * ru, 2 * rv, 2 * rw), tone=JOINT_TONE, role="detail"), "seat", "joint"))
    return out


def back_fixings(r):
    """🟠 The screws of back board and back rail on the post fronts."""
    out = []
    for p, side in K.SIDES:
        for name, u0, u1 in (("board", K.BOARD_U0, K.BOARD_U1), ("rail", K.RAIL_U0, K.RAIL_U1)):
            out.append(own(K.box(f"joint-back-{name}-{p}", "back", K.STEEL, ((u0 + u1) / 2, K.back_vc(K.POST_Z) + K.BACK_T / 2 - r / 2, side * K.POST_Z), (min(u1 - u0, 0.06), r, 2 * K.R), tone=JOINT_TONE, role="detail"), "back", "joint"))
    return out


def slender_member(sid, bone, p0, p1, r, member):
    """➖ A slender idealised bar of a member between two world points (true tube size)."""
    return own(K.capsule(sid, bone, p0, p1, r, slender=True), member, "member")


def frame_members():
    """➖ The idealised tube members of the coarse scales: rear leg (foot to corner), post (corner to top), front leg, stretcher, per side, at true tube size."""
    out = []
    for p, side in K.SIDES:
        out.append(slender_member(f"m-rear-{p}", "leg-rear", (0.0, 0.0, side * K.REAR_FOOT_Z), (K.CORNER[0], K.CORNER[1], side * K.POST_Z), K.R, "leg-rear"))
        out.append(slender_member(f"m-post-{p}", "back", (K.CORNER[0], K.CORNER[1], side * K.POST_Z), (K.POST_TOP[0], K.POST_TOP[1], side * K.POST_Z), K.R, "back"))
        out.append(slender_member(f"m-front-{p}", "leg-front", (K.FRONT_FOOT[0], 0.0, side * K.FRONT_FOOT_Z), (K.FRONT_TOP[0], K.FRONT_TOP[1], side * K.FRONT_TOP_Z), K.R, "leg-front"))
        a, b = K.STRETCH_REAR, K.STRETCH_FRONT
        out.append(slender_member(f"m-stretcher-{p}", "stretcher", (a[0], a[1], side * a[2]), (b[0], b[1], side * b[2]), K.RS, "stretcher"))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale."""
    if scale == "50":
        members = [ref(s) for s in body if s["material"] != K.NYLON]
        return members + bends((0.02, 0.016, 0.016)) + welds(0.0125) + seat_fixings(0.012, 0.0225, 0.012) + back_fixings(0.008) + supports(K.GLIDE_R, 0.008) + tube_masses(0.0155) + [seat_mass(0.03, 0.012)] + back_masses(0.022, 0.012)
    if scale == "100":
        members = [ref(s) for s in body]
        return members + bends((0.024, 0.018, 0.018)) + welds(0.0155) + seat_fixings(0.015, 0.0225, 0.015) + supports(0.015, 0.015) + tube_masses(0.0175) + [seat_mass(0.035, 0.016)] + back_masses(0.025, 0.016)
    if scale == "200":
        members = frame_members() + [ref(s) for s in body if s["id"] in ("seat-board", "apron", "back-board", "back-rail")]
        return members + bends((0.03, 0.03, 0.03)) + seat_fixings(0.015, 0.0225, 0.015) + supports(0.02, 0.03) + tube_masses(0.02) + [seat_mass(0.04, 0.03)] + back_masses(0.03, 0.03)
    members = frame_members() + [ref(s) for s in body if s["id"] in ("seat-slab", "back-slab")]
    return members + seat_fixings(0.0375, 0.0375, 0.0375) + supports(0.0375, 0.075) + tube_masses(0.0375, posts=False) + [seat_mass(0.05, 0.075, -0.0375)] + [own(K.box("mass-back", "back", K.STEEL, (K.masses()["back"][1], (K.SLAB500_V0 + K.SLAB500_V1) / 2, 0.0), (0.075, 0.075, 0.075), tone=MASS_TONE, role="detail"), "back", "mass")]


DECISIONS = {
    "50": [
        {"en": "1:50 is the full construction of the real chair: per side one Ø22 x 2 mm steel tube that rises from its nylon glide as the rear leg (raked back 7.7 degrees), turns in an 80 mm bend at the seat and continues as the back post (reclined 8 degrees) up to a domed end cap; two straight front legs on glides; two Ø16 side stretchers welded between the legs at 0.15 m; a 20 mm beech seat board, trapezoidal with rounded front corners and notched round the posts, raked 2 degrees, on an apron frame of four 18 x 45 mm beech rails; a curved 18 mm beech backrest board and a lower rail screwed to the post fronts.", "de": "1:50 ist die volle Konstruktion des echten Stuhls: je Seite ein Stahlrohr Ø22 x 2 mm, das von seinem Nylongleiter als Hinterbein (7,7 Grad nach hinten ausgestellt) aufsteigt, am Sitz in einem 80-mm-Bogen umbiegt und als Lehnenpfosten (8 Grad geneigt) bis zur gewölbten Endkappe weiterläuft; zwei gerade Vorderbeine auf Gleitern; zwei seitliche Fußstreben Ø16, auf 0,15 m zwischen die Beine geschweißt; ein 20 mm Buchensitzbrett, trapezförmig mit gerundeten Vorderecken und um die Pfosten ausgeklinkt, 2 Grad geneigt, auf einem Zargenrahmen aus vier Buchenleisten 18 x 45 mm; ein gebogenes 18-mm-Buchenrückenbrett und eine untere Leiste, auf die Pfostenvorderseiten geschraubt."},
        {"en": "Thermal: every beech and steel solid is its region's surface with the region's one layer (the closed tubes are solid, their sealed bore is not outer surface); glides and caps are nylon and belong to no region. Structure: the tubes, seat board, rails, back board and rail are the members of their bones; the two bends, four stretcher welds, four seat tab fixings and four back screw fixings are the joints; one glide pad per contact anchor; a mass at the middle of every tube and post and at the centres of seat and back boards.", "de": "Thermik: jeder Buchen- und Stahlkörper ist die Oberfläche seiner Region mit deren einziger Schicht (die geschlossenen Rohre sind voll, ihre verschlossene Bohrung ist keine Außenfläche); Gleiter und Kappen sind aus Nylon und gehören zu keiner Region. Tragwerk: Rohre, Sitzbrett, Zargen, Rückenbrett und Leiste sind die Glieder ihrer Knochen; die zwei Bögen, vier Strebenschweißungen, vier Sitzlaschen und vier Rückenverschraubungen sind die Gelenke; ein Gleiterlager je Kontaktanker; eine Masse in der Mitte jedes Rohrs und Pfostens und in den Schwerpunkten von Sitz und Rückenbrett."},
    ],
    "100": [
        {"en": "1:100 merges glides and caps into the tubes (8-10 mm, below the 15 mm feature), which now stand on the floor; tubes and bend keep their true Ø22, the stretchers their Ø16; the four apron rails become one 18 mm ring under the 20 mm board, whose front corners are rounded in two steps; back board and rail keep their curve with five sections each.", "de": "1:100 lässt Gleiter und Kappen in den Rohren aufgehen (8-10 mm, unter dem 15-mm-Mindestmaß), die nun auf dem Boden stehen; Rohre und Bogen behalten ihr echtes Ø22, die Streben ihr Ø16; die vier Zargenleisten werden ein 18-mm-Ring unter dem 20-mm-Brett, dessen Vorderecken in zwei Schritten gerundet sind; Rückenbrett und Leiste behalten ihre Krümmung mit je fünf Schnitten."},
        {"en": "Thermal references the body without layers; structure: tubes, board, ring, back board and rail as members, bends, welds and seat fixings as joints (the back screws are below the feature), 30 mm glide pads, masses as at 1:50.", "de": "Thermik verweist ohne Schichten auf den Körper; Tragwerk: Rohre, Brett, Ring, Rückenbrett und Leiste als Glieder, Bögen, Schweißungen und Sitzlaschen als Gelenke (die Rückenschrauben liegen unter dem Mindestmaß), 30-mm-Gleiterlager, Massen wie bei 1:50."},
    ],
    "200": [
        {"en": "1:200 draws the tubes as strokes, so they are slender solids at their true Ø22 and Ø16 (C24): one continuous tube per side from the floor over the bend to the post top, two front legs, two stretchers; seat board and apron ring are generalised to 30 mm (the board notch and the 30 mm corner radius are below the feature); back board and rail are flat 30 mm boards in front of the posts (their 26 mm curve is below the feature).", "de": "1:200 zeichnet die Rohre als Striche, daher sind sie schlanke Körper in echter Größe Ø22 und Ø16 (C24): je Seite ein durchgehendes Rohr vom Boden über den Bogen bis zur Pfostenspitze, zwei Vorderbeine, zwei Streben; Sitzbrett und Zargenring sind auf 30 mm verallgemeinert (Ausklinkung und 30-mm-Eckradius liegen unter dem Mindestmaß); Rückenbrett und Leiste sind ebene 30-mm-Bretter vor den Pfosten (ihre 26-mm-Krümmung liegt unter dem Mindestmaß)."},
        {"en": "Thermal references the body; structure: idealised true-size bars per tube member (rear leg, post, front leg, stretcher) beside the referenced boards and ring, the two bends and four seat fixings as joints, 40 mm glide pads, masses per tube and at the board centres.", "de": "Thermik verweist auf den Körper; Tragwerk: idealisierte Stäbe in echter Größe je Rohrglied (Hinterbein, Pfosten, Vorderbein, Strebe) neben den verwiesenen Brettern und dem Ring, die zwei Bögen und vier Sitzlaschen als Gelenke, 40-mm-Gleiterlager, Massen je Rohr und in den Brettschwerpunkten."},
    ],
    "500": [
        {"en": "1:500 is the chair sign: a 75 mm seat slab (board and apron merged), a 75 mm back slab (backrest board and the posts beside it merged; the lower rail is dropped), four slender true-size tubes (the rear ones straight from foot to bend and from bend to top); the stretchers are omitted from the body (16 mm, 0.03 mm on paper).", "de": "1:500 ist das Stuhlzeichen: eine 75-mm-Sitzplatte (Brett und Zarge verschmolzen), eine 75-mm-Rückenplatte (Rückenbrett und die Pfosten daneben verschmolzen; die untere Leiste entfällt), vier schlanke Rohre in echter Größe (die hinteren gerade vom Fuß zum Bogen und vom Bogen zur Spitze); die Streben entfallen im Körper (16 mm, 0,03 mm auf Papier)."},
        {"en": "Thermal references the body and keeps the two stretchers as slender steel rods; structure: true-size bars per tube member including the stretchers, the two slabs, four 75 mm seat corner joints (the bends lie in the seat slab), 75 mm pads, masses per leg tube, at the seat centre and at the back centre.", "de": "Thermik verweist auf den Körper und behält die zwei Streben als schlanke Stahlstäbe; Tragwerk: Stäbe in echter Größe je Rohrglied einschließlich der Streben, die zwei Platten, vier 75-mm-Sitzeckgelenke (die Bögen liegen in der Sitzplatte), 75-mm-Lager, Massen je Beinrohr, im Sitz- und im Rückenschwerpunkt."},
    ],
}


def decisions(scale):
    """📝 The bilingual volume decisions of a scale."""
    return DECISIONS[scale]
