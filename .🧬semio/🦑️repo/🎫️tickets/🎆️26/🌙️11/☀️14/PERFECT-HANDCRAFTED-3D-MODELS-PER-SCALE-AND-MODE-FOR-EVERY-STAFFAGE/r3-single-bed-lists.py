"""🌡 Round 3, furniture/single-bed: the thermal and the structural list of every scale and the bilingual decisions.

Thermal: every body surface is the surface of its region (pine frame, slatted base as the hidden region, mattress); at 1:50 the mattress is the 10 mm quilted cover with the cold-foam core nested inside. Structure: the load path only: posts, rails, ledgers, cross beam and leg, the base rails, the lifting frame and the slats; bolts, dowels and slat caps as joints; glides as supports; the mattress parts as bearing strips with their masses. Panels, piping and cover are absent.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_single_bed_" + name, os.path.join(HERE, f"r3-single-bed-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

REGION = {K.PINE: ("bed-frame", "pine-member"), K.BIRCH: ("slats", "slat-birch"), K.RUBBER: ("slats", "slat-birch"), K.STEEL: ("slats", "slat-birch"), K.QUILT: ("mattress", "quilt")}
PARTS = (("mat-head", K.MAT_X0, K.HINGE_X, -1), ("mat-mid", K.HINGE_X, K.FOOT_X, 0), ("mat-foot", K.FOOT_X, K.MAT_X1, 1))


def thermal(scale, body):
    """🌡 The thermal list of a scale."""
    out = []
    if scale == "50":
        for sid, x0, x1, end in PARTS:
            a = x0 + (0.0 if end == -1 else 0.0)
            core = B.mattress_part("th-" + sid + "-core", sid, a, x1, end, B.END_50, inset=0.01, material=K.FOAM)
            out.append(dict(core, region="mattress", layer="foam-core"))
    for solid in body:
        region, layer = REGION[solid["material"]]
        if region == "slats":
            entry = dict(solid, id="th-" + solid["id"], region=region)
        else:
            entry = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
        if scale == "50":
            entry["layer"] = layer
        out.append(entry)
    if scale == "500":
        hidden = K.box("th-slats", "slat-mid", K.BIRCH, (0.08, 2.04), (K.BASE_Y0, K.MAT_Y0 - 0.005), (-0.44, 0.44))
        out.append(dict(hidden, region="slats"))
    return out


def of(solid, member, part="member", sid=None):
    """🔗 A structural reference to a body solid that is the member itself."""
    return {"id": sid or (("member-" if part == "member" else "joint-") + solid), "of": solid, "member": member, "part": part}


def pin(sid, bone, axis, origin, length, radius, member, **more):
    """📍 A bolt or dowel as a joint solid: a cylinder from `origin` along a bone axis."""
    solid = K.turned(sid, bone, K.STEEL, [(0, 0), (0, radius), (length, radius), (length, 0)], axis, origin, role="detail")
    solid.update({"member": member, "part": "joint"})
    solid.update(more)
    return solid


def glide(sid, anchor, x, z, radius, height, count=1, dw=0.0):
    """🟢 The floor glide under a post or leg as the contact support."""
    solid = K.turned(sid, "frame", K.PINE, [(0, 0), (0, radius), (height, radius), (height, 0)], "v", (x, 0, z), tone=0.15, role="detail")
    if count > 1:
        K.repeat(solid, count, dw=dw)
    solid.update({"member": "frame", "part": "support", "anchor": anchor})
    return solid


def bedding(scale, thick, wide):
    """🧱 The mattress parts as the structure sees them: a bearing strip of foam along each bone that lies on the slat crowns, and the mass of the part at its centre, resting on the strip."""
    out = []
    for sid, x0, x1, _ in PARTS:
        yc = K.MAT_Y0 + K.MAT_T / 2
        strip = K.box("member-" + sid, sid, K.FOAM, (x0 + 0.02, x1 - 0.02), (K.MAT_Y0, K.MAT_Y0 + thick), (-wide / 2, wide / 2), tone=0.4)
        strip.update({"member": sid, "part": "member"})
        out.append(strip)
        lump = K.base("mass-" + sid, "ellipsoid", sid, K.FOAM, 0.3, "detail")
        r = max(yc - K.MAT_Y0 - thick + 0.005, 0.0375)
        wide_r = 0.075 if scale == "500" else r
        lump["radii"] = [K.r6(wide_r), K.r6(r), K.r6(wide_r)]
        ox, oy = K.ORIGIN[sid]
        lump["transform"] = {"translation": [K.r6((x0 + x1) / 2 - ox), K.r6(yc - oy), 0]}
        lump.update({"member": sid, "part": "mass"})
        out.append(lump)
    return out


def supports(radius, height):
    """🦶 One support solid per support anchor."""
    return [
        glide("support-foot-head", "foot-head", 0.03, K.POST_Z, radius, height, 2, -2 * K.POST_Z),
        glide("support-foot-mid", "foot-mid", 1.06, 0, radius, height),
        glide("support-foot-foot-l", "foot-foot-l", K.LENGTH - 0.03, K.POST_Z, radius, height),
        glide("support-foot-foot-r", "foot-foot-r", K.LENGTH - 0.03, -K.POST_Z, radius, height),
    ]


def rail_bolts(radius):
    """🔩 The bed bolts that draw each side rail end into its post: two per rail end."""
    out = []
    for end, x in (("head", 0.005), ("foot", K.LENGTH - 0.115)):
        for hand, side in (("l", 1), ("r", -1)):
            bolt = pin(f"joint-rail-{end}-{hand}", "frame", "u", (x, 0.18, side * (K.CLEAR_Z + K.RAIL_T / 2)), 0.11, radius, "frame")
            out.append(K.repeat(bolt, 2, dv=0.08))
    return out


def dowels(radius, rails):
    """🪵 The dowelled joints of the end rails in the posts."""
    out = []
    for sid, x, y in rails:
        dowel = pin("joint-" + sid, "frame", "w", (x, y, K.BOARD_Z - 0.03), 0.06, radius, "frame", material=K.PINE, tone=0.2)
        out.append(K.repeat(dowel, 2, dw=-(2 * K.BOARD_Z)))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale."""
    ids = {s["id"] for s in body}
    out = []
    if scale == "50":
        for sid in ("post-head", "post-foot", "head-top-rail", "head-low-rail", "foot-top-rail", "foot-low-rail", "side-rail", "ledger", "cross-beam", "leg-mid"):
            out.append(of(sid, "frame"))
        for sid in ("base-rail-mid", "base-cross-head", "slat-mid"):
            out.append(of(sid, "slat-mid"))
        for sid in ("base-rail-foot", "base-cross-foot", "slat-foot"):
            out.append(of(sid, "slat-foot"))
        for sid in ("lift-rail", "lift-cross", "slat-head"):
            out.append(of(sid, "slat-head"))
        out += bedding(scale, 0.03, 0.2)
        out += [of("hinge-bolt-l", "slat-head", "joint"), of("hinge-bolt-r", "slat-head", "joint")]
        for name in ("head", "mid", "foot"):
            for hand in ("l", "r"):
                out.append(of(f"cap-{name}-{hand}", "slat-" + name, "joint"))
        out += rail_bolts(0.004)
        out += dowels(0.005, (("head-top", 0.03, 0.89), ("head-low", 0.03, 0.22), ("foot-top", 2.09, 0.49), ("foot-low", 2.09, 0.22)))
        out += supports(0.022, 0.008)
    elif scale == "100":
        for sid in ("post-head", "post-foot", "head-top-rail", "head-low-rail", "foot-top-rail", "foot-low-rail", "side-rail-l", "side-rail-r", "cross-beam", "leg-mid"):
            out.append(of(sid, "frame"))
        for sid in ("base-rail-mid", "base-cross-head", "slat-mid"):
            out.append(of(sid, "slat-mid"))
        for sid in ("base-rail-foot", "base-cross-foot", "slat-foot"):
            out.append(of(sid, "slat-foot"))
        for sid in ("lift-rail", "lift-cross", "slat-head"):
            out.append(of(sid, "slat-head"))
        out += bedding(scale, 0.03, 0.2)
        for hand, side in (("l", 1), ("r", -1)):
            bolt = pin("joint-hinge-" + hand, "slat-mid", "w", (K.HINGE_X, K.HINGE_Y, side * 0.38), 0.075, 0.0075, "slat-head")
            bolt["axis"] = [0, 0, side]
            out.append(bolt)
        out += rail_bolts(0.0075)
        out += supports(0.025, 0.015)
    elif scale == "200":
        for sid in ("post-head", "post-foot", "side-rail", "cross-beam", "leg-mid"):
            out.append(of(sid, "frame"))
        board = (-K.BOARD_Z, K.BOARD_Z)
        for sid, x, y in (("head-top-rail", 0.015, (0.85, 0.93)), ("head-low-rail", 0.015, (K.RAIL_Y0, K.RAIL_Y1)), ("foot-top-rail", 2.075, (0.45, 0.53)), ("foot-low-rail", 2.075, (K.RAIL_Y0, K.RAIL_Y1))):
            rail = K.box("member-" + sid, "frame", K.PINE, (x, x + K.RAIL_T), y, board)
            rail.update({"member": "frame", "part": "member"})
            out.append(rail)
        for sid in ("slat-head", "slat-mid", "slat-foot"):
            out.append(of(sid, sid))
        out += bedding(scale, 0.03, 0.2)
        for hand, side in (("l", 1), ("r", -1)):
            bolt = pin("joint-hinge-" + hand, "slat-mid", "w", (K.HINGE_X, K.HINGE_Y, side * 0.38), 0.075, 0.015, "slat-head")
            bolt["axis"] = [0, 0, side]
            out.append(bolt)
        out += supports(0.03, 0.03)
    else:
        post = lambda sid, x0, top: K.repeat(dict(K.box(sid, "frame", K.PINE, (x0, x0 + K.POST), (0, top), (K.POST_Z - 0.03, K.POST_Z + 0.03)), member="frame", part="member"), 2, dw=-2 * K.POST_Z)
        out.append(post("member-post-head", 0.0, K.HEAD_H))
        out.append(post("member-post-foot", K.LENGTH - K.POST, K.FOOT_H))
        rail = K.box("member-side-rail", "frame", K.PINE, (K.POST, K.LENGTH - K.POST), (K.RAIL_Y0, K.RAIL_Y1), (K.CLEAR_Z - 0.045, K.CLEAR_Z + K.RAIL_T))
        rail.update({"member": "frame", "part": "member"})
        out.append(K.repeat(rail, 2, dw=-(2 * K.CLEAR_Z + K.RAIL_T - 0.045)))
        for sid, x in (("member-end-rail-head", 0.0), ("member-end-rail-foot", K.LENGTH - K.POST)):
            end = K.box(sid, "frame", K.PINE, (x, x + K.POST), (K.RAIL_Y0, K.RAIL_Y1), (-K.BOARD_Z, K.BOARD_Z))
            end.update({"member": "frame", "part": "member"})
            out.append(end)
        out.append(of("leg-mid", "frame"))
        for sid, x0, x1 in (("slat-head", 0.08, K.HINGE_X), ("slat-mid", K.HINGE_X, K.FOOT_X), ("slat-foot", K.FOOT_X, 2.04)):
            plate = K.box("member-" + sid, sid, K.BIRCH, (x0, x1), (K.BASE_Y0, K.MAT_Y0 - 0.005), (-0.41, 0.41))
            plate.update({"member": sid, "part": "member"})
            out.append(plate)
        out += bedding(scale, 0.075, 0.3)
        out += supports(0.0375, 0.075)
    return out


DECISIONS = {
    "50": [
        ("Body, authored for 7.5 mm: four 60 mm pine posts with eased edges and chamfered tops; head and foot ends as frame and panel (30 mm top and bottom rails, 18 mm panels of V-jointed boards); 30 x 160 mm side rails with eased edges and 25 mm ledgers; a 40 x 65 mm cross beam on a centre leg; the sprung slatted base with two base rails and end bars, the lifting head frame on two steel bolts and 28 cambered 38 x 8 mm slats in rubber caps; one 0.90 x 2.00 x 0.20 m mattress in three parts that ride on the slat sections, with rounded ends and piped seams at the top and bottom edges. Glides, screws, the ratchet stay of the head frame, the centre strap of the slats, zip and handles are omitted (below 7.5 mm or hidden).",
         "Körper, für 7,5 mm gebaut: vier 60-mm-Kiefernpfosten mit gebrochenen Kanten und gefasten Köpfen; Kopf- und Fußteil als Rahmen mit Füllung (30 mm starke obere und untere Riegel, 18 mm starke Füllungen aus Profilbrettern mit V-Fuge); Seitenholme 30 x 160 mm mit gebrochenen Kanten und 25-mm-Auflageleisten; ein Mittelsteg 40 x 65 mm auf einem Mittelfuß; der Federholzrahmen mit zwei Holmen und Querstegen, dem aufstellbaren Kopfrahmen auf zwei Stahlbolzen und 28 gewölbten Federleisten 38 x 8 mm in Kautschukkappen; eine Matratze 0,90 x 2,00 x 0,20 m in drei Teilen, die auf den Rostabschnitten liegen, mit gerundeten Enden und Kedernähten an Ober- und Unterkante. Gleiter, Schrauben, der Rastbeschlag des Kopfrahmens, der Mittelgurt der Leisten, Reißverschluss und Griffe entfallen (unter 7,5 mm oder verdeckt)."),
        ("Thermal: the mattress is two nested shells, the 10 mm quilted cover (the body surface) around the cold-foam core; the slatted base with rails, caps and bolts is the hidden region inside the frame; every timber part of the frame is the frame surface. Structure: posts, rails, ledgers, cross beam and leg, the base rails, the lifting frame and every slat are the members; the hinge bolts, the bed bolts of the side rails, the dowels of the end rails and the slat caps are the joints; four glides are the supports; each mattress part is a bearing strip of foam along its bone with its mass at the centre. Panels, cover and piping carry nothing and are absent.",
         "Thermik: die Matratze besteht aus zwei verschachtelten Schalen, dem 10 mm starken Steppbezug (die Körperoberfläche) um den Kaltschaumkern; der Federholzrahmen mit Holmen, Kappen und Bolzen ist die verdeckte Region im Rahmen; jedes Holzteil des Rahmens ist Rahmenoberfläche. Tragwerk: Pfosten, Holme, Auflageleisten, Mittelsteg und Fuß, die Rostholme, der Kopfrahmen und jede Federleiste sind die Stäbe; die Gelenkbolzen, die Bettschrauben der Seitenholme, die Dübel der Querriegel und die Leistenkappen sind die Gelenke; vier Gleiter sind die Auflager; jedes Matratzenteil ist ein tragender Schaumstreifen entlang seines Knochens mit seiner Masse im Schwerpunkt. Füllungen, Bezug und Keder tragen nichts und fehlen."),
    ],
    "100": [
        ("Body, authored anew for 15 mm: plain rounded posts without chamfer; rails and panels as plain boards at their true thickness; side rail and ledger merged into one L section; base rails, end bars and lifting frame kept; the slats and caps of each section merged into one cambered deck; the mattress parts with a simpler rounding and without piping. Bolts and V joints are omitted.",
         "Körper, für 15 mm neu gebaut: glatte gerundete Pfosten ohne Fase; Riegel und Füllungen als glatte Bretter in wahrer Dicke; Seitenholm und Auflageleiste zu einem L-Profil verschmolzen; Rostholme, Querstege und Kopfrahmen bleiben; Leisten und Kappen jedes Abschnitts zu einem gewölbten Deck verschmolzen; die Matratzenteile mit einfacherer Rundung und ohne Keder. Bolzen und V-Fugen entfallen."),
        ("Thermal: one surface per region, the mattress layers merged (cover and core are told apart only at 1:50). Structure: the frame members, base rails, lifting frame and slat decks; hinge and bed bolts as 15 mm pins; glides; bearing strips and masses of the mattress parts.",
         "Thermik: eine Oberfläche je Region, die Matratzenschichten verschmolzen (Bezug und Kern werden nur bei 1:50 unterschieden). Tragwerk: die Rahmenstäbe, Rostholme, Kopfrahmen und Leistendecks; Gelenk- und Bettbolzen als 15-mm-Stifte; Gleiter; Tragstreifen und Massen der Matratzenteile."),
    ],
    "200": [
        ("Body, authored anew for 30 mm: square posts; head and foot end as one 30 mm board each from the rail to the top rail; side rails; cross beam and leg; every slat section merged with its rails into one slab from the ledger to the slat crowns; mattress parts with one rounding step. Ledgers, panels as separate parts and all fittings are merged or omitted.",
         "Körper, für 30 mm neu gebaut: kantige Pfosten; Kopf- und Fußteil je ein 30-mm-Brett vom Holm bis zum oberen Riegel; Seitenholme; Mittelsteg und Fuß; jeder Rostabschnitt mit seinen Holmen zu einer Platte von der Auflageleiste bis zum Leistenscheitel verschmolzen; Matratzenteile mit einer Rundungsstufe. Auflageleisten, Füllungen als eigene Teile und alle Beschläge sind verschmolzen oder entfallen."),
        ("Thermal: one surface per region; the slat slabs are the hidden region. Structure: posts, side rails, the top and bottom rails of both ends as members of their own (the boards are cladding), cross beam, leg and the slat slabs; hinge pins; glides; bearing strips and masses of the mattress parts.",
         "Thermik: eine Oberfläche je Region; die Rostplatten sind die verdeckte Region. Tragwerk: Pfosten, Seitenholme, die oberen und unteren Riegel beider Enden als eigene Stäbe (die Bretter sind Verkleidung), Mittelsteg, Fuß und die Rostplatten; Gelenkstifte; Gleiter; Tragstreifen und Massen der Matratzenteile."),
    ],
    "500": [
        ("Body, authored anew for 75 mm: head end and foot end as one panel each at their true heights of 0.95 and 0.55 m, side rails and slatted base merged into one tray on the centre leg, the mattress as three plain blocks with soft long edges on their bones. The model is minimal: nothing smaller can be left out without losing the bed.",
         "Körper, für 75 mm neu gebaut: Kopf- und Fußteil je eine Platte in wahrer Höhe von 0,95 und 0,55 m, Seitenholme und Lattenrost zu einer Wanne auf dem Mittelfuß verschmolzen, die Matratze als drei glatte Blöcke mit weichen Längskanten auf ihren Knochen. Das Modell ist minimal: kleiner geht es nicht, ohne das Bett zu verlieren."),
        ("Thermal: panels, tray and leg are the frame surface, the mattress blocks the mattress surface, the slatted base is one hidden plate inside the tray. Structure: four posts, two side rails, the two end rails, the leg and three slat plates as members; glides; bearing strips and masses of the mattress parts.",
         "Thermik: Platten, Wanne und Fuß sind die Rahmenoberfläche, die Matratzenblöcke die Matratzenoberfläche, der Lattenrost ist eine verdeckte Platte in der Wanne. Tragwerk: vier Pfosten, zwei Seitenholme, die beiden Querriegel, der Fuß und drei Rostplatten als Stäbe; Gleiter; Tragstreifen und Massen der Matratzenteile."),
    ],
}


def decisions(scale):
    """💬 The bilingual decisions of a scale."""
    return [{"en": en, "de": de} for en, de in DECISIONS[scale]]
