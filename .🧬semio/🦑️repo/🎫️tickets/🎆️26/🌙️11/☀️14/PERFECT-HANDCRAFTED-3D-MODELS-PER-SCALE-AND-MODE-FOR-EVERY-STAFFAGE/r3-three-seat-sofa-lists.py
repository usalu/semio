"""🌡 Round 3, furniture/three-seat-sofa: the thermal and the structural list of every scale and the bilingual decisions.

Thermal. Every upholstered part is its region's surface. At 1:50 the layers nest inside it, innermost first: the foam cores of the cushions, the batting of the arms, batting and ply of the back frame, and the 18 mm ply shell of the plinth. The feet are beech. At 1:500 the merged blocks are split back into their regions by own solids.

Structure. It shows only the load path:
- the plinth's beech base rails and its ply deck;
- the back frame's bottom rail, uprights and top rail;
- in each arm, the posts, rails and inner ply panel;
- under each loose cushion, its foam bearing layer.
Corner blocks, dowels and bolts are the joints. Glides at the four foot anchors are the supports. Every bone carries its mass at its centre.
"""
import copy
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_three_seat_sofa_kit", os.path.join(HERE, "r3-three-seat-sofa-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

REGION = {"seat": ("seat-cushions", "foam-seat"), "back": ("back-cushions", "foam-back"), "arm": ("arm-pads", "batting-arm"), "foot": ("feet", "beech-foot"), "base": ("base-rail", "ply-skirt"), "back-frame": ("back-frame", None)}
ARMS = (("arm-left", 0.1), ("arm-right", 1.9))


def region_of(sid):
    """🏷 The region and core layer of a body solid."""
    if sid == "back-frame":
        return REGION["back-frame"]
    return REGION[sid.split("-")[0]]


def shrunk(solid, d, sid, region, layer):
    """🫧 A y-loft shrunk by d all round (a nested layer shell inside it)."""
    out = copy.deepcopy(solid)
    secs = [s for s in out["sections"] if min(s["rx"], s["ry"]) > d + 0.003]
    out["sections"] = secs
    for s in secs:
        s["rx"] = K.r6(s["rx"] - d)
        s["ry"] = K.r6(s["ry"] - d)
    axis = 0 if out["bone"].startswith("arm") else 1
    secs[0]["at"][axis] = K.r6(secs[0]["at"][axis] + d)
    secs[-1]["at"][axis] = K.r6(secs[-1]["at"][axis] - d)
    out.update(id=sid, region=region, layer=layer)
    out.pop("repeat", None)
    return out


def thermal(scale, body):
    """🌡 The thermal list of a scale."""
    out = []
    for solid in body:
        sid = solid["id"]
        if solid.get("texture"):
            continue
        if scale == "500" and sid in ("seat-block", "back-block"):
            continue
        name, core = region_of(sid)
        ref = {"id": "th-" + sid, "of": sid, "region": name}
        if scale == "50":
            ref["layer"] = "fabric" if name != "feet" else "beech-foot"
        out.append(ref)
        if scale != "50" or name == "feet":
            continue
        if name == "base-rail":
            out.append(K.along_x("th-base-ply", "frame", K.PLY, 0.5, K.base_profile("50", 0.001), K.BASE_X) | {"region": name, "layer": core})
        elif name == "back-frame":
            out.append(shrunk(solid, 0.001, "th-back-frame-batting", name, "batting-back"))
            out.append(shrunk(solid, 0.021, "th-back-frame-ply", name, "ply-back"))
        else:
            out.append(shrunk(solid, 0.001, f"th-{sid}-core", name, core))
    if scale == "500":
        out.append(K.box("th-base", "frame", K.PLY, 0.45, K.BASE_X, K.BASE_Y, K.BASE_Z) | {"region": "base-rail"})
        out.append(K.box("th-seat", "frame", K.FOAM, 0.72, K.BASE_X, (K.DECK_Y, K.SEAT_TOP), (K.BASE_Z[0], K.SEAT_Z[1])) | {"region": "seat-cushions"})
        cushion = [(K.DECK_Y, K.SEAT_Z[1]), (K.DECK_Y, 0.29), (K.FRAME_TOP, K.frame_face(K.FRAME_TOP)), (K.FRAME_TOP, 0.33), (0.78, 0.22), (K.SEAT_TOP, 0.15), (K.SEAT_TOP, K.SEAT_Z[1])]
        frame = [(K.DECK_Y, 0.29), (K.DECK_Y, 0.45), (K.FRAME_TOP, 0.45), (K.FRAME_TOP, K.frame_face(K.FRAME_TOP))]
        out.append(K.along_x("th-back-cushions", "frame", K.FOAM, 0.62, cushion, K.BASE_X) | {"region": "back-cushions"})
        out.append(K.along_x("th-back-frame", "frame", K.PLY, 0.55, frame, K.BASE_X) | {"region": "back-frame"})
    return out


def own(solid, member, part, **more):
    """🧩 An own structural solid."""
    solid.update({"member": member, "part": part})
    solid.update(more)
    return solid


def plinth(rail_w, rail_y, deck_t):
    """🟫 The base rails (front and back, through the arms) and the ply deck between the arms."""
    out = []
    z = K.BASE_Z[0] + 0.02
    rail = K.box("member-base-rail", "frame", K.BEECH, 0.3, (0.02, 1.98), rail_y, (z, z + rail_w))
    out.append(own(K.repeat(rail, 2, dw=(K.BASE_Z[1] - 0.02 - rail_w) - z), "frame", "member"))
    if deck_t:
        out.append(own(K.box("member-deck", "frame", K.PLY, 0.35, K.BASE_X, (K.DECK_Y - 0.002 - deck_t, K.DECK_Y - 0.002), (z + rail_w, K.BASE_Z[1] - 0.02 - rail_w)), "frame", "member"))
    return out


def back_frame(upright_xs, rail_t, top_y, upright_w):
    """🟧 The back frame: bottom rail on the plinth, uprights, top rail under the roll."""
    out = [own(K.box("member-back-rail", "frame", K.BEECH, 0.3, K.BASE_X, (K.DECK_Y, K.DECK_Y + 0.07), (0.44 - rail_t, 0.44)), "frame", "member")]
    for k, x in enumerate(upright_xs):
        out.append(own(K.box(f"member-upright-{k + 1}", "frame", K.BEECH, 0.3, (x - upright_w / 2, x + upright_w / 2), (K.DECK_Y + 0.07, top_y[0]), (0.44 - rail_t, 0.44)), "frame", "member"))
    out.append(own(K.box("member-top-rail", "frame", K.BEECH, 0.3, K.BASE_X, top_y, (0.44 - rail_t - 0.01, 0.44)), "frame", "member"))
    return out


def arm_frame(bone, cx, t, panel):
    """🟪 One arm frame: front and back posts, top and bottom rails, and the inner ply panel facing the seat."""
    s = 1 if cx < 1 else -1
    x0, x1 = cx - 0.07, cx + 0.07
    y0, y1 = 0.105, 0.58
    z0, z1 = -0.43, 0.43
    out = [
        own(K.repeat(K.box(f"member-{bone}-post", bone, K.BEECH, 0.3, (x0, x1), (y0, y1), (z0, z0 + t)), 2, dw=z1 - z0 - t), bone, "member"),
        own(K.box(f"member-{bone}-top-rail", bone, K.BEECH, 0.3, (x0, x1), (y1 - t, y1), (z0 + t, z1 - t)), bone, "member"),
        own(K.box(f"member-{bone}-bottom-rail", bone, K.BEECH, 0.3, (x0, x1), (y0, y0 + t), (z0 + t, z1 - t)), bone, "member"),
    ]
    if panel:
        px = (x1 - panel, x1) if s == 1 else (x0, x0 + panel)
        out.append(own(K.box(f"member-{bone}-panel", bone, K.PLY, 0.35, px, (y0 + t, y1 - t), (z0 + t, z1 - t)), bone, "member"))
    return out


def cushion_bearings(t, pad, band):
    """🟫 The loose cushions as members along their bones: under each seat cushion a foam bearing band of width `band` on the deck under the sitting line (z -0.2 m), behind each back cushion a band at the lumbar height (0.46 m) on the frame face; `pad` > 0 adds the contact pads as the joints."""
    out = []
    for k in range(3):
        bone_s, bone_b = f"seat-{k + 1}", f"back-{k + 1}"
        x0, x1 = K.SEAT_X[k] + 0.01, K.SEAT_X[k + 1] - 0.01
        xc = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        out.append(own(K.box(f"member-{bone_s}", bone_s, K.FOAM, 0.4, (x0, x1), (K.DECK_Y, K.DECK_Y + t), (-0.2 - band / 2, -0.2 + band / 2)), bone_s, "member"))
        stations = [(y, (x1 - x0) / 2, K.frame_face(y) - t / 2, t / 2) for y in (0.46 - band / 2, 0.46 + band / 2)]
        out.append(own(K.yloft(f"member-{bone_b}", bone_b, K.FOAM, 0.4, xc, stations, 6), bone_b, "member"))
        if pad:
            zc = -0.2
            out.append(own(K.box(f"joint-{bone_s}", bone_s, K.FABRIC, 0.2, (xc - 0.08, xc + 0.08), (K.DECK_Y - pad, K.DECK_Y), (zc - 0.08, zc + 0.08), role="detail"), bone_s, "joint"))
            jst = [(y, 0.08, K.frame_face(y) + pad / 2, pad / 2) for y in (0.42, 0.56)]
            out.append(own(K.yloft(f"joint-{bone_b}", bone_b, K.FABRIC, 0.2, xc, jst, 4, role="detail"), bone_b, "joint"))
    return out


def corner_blocks(size, with_back):
    """🔩 Corner blocks where the base rails meet the arm frames, and the bolts of the back frame into the arm back posts."""
    out = []
    zf = K.BASE_Z[0] + 0.02 + 0.025
    for name, x in (("l", K.BASE_X[0] + size / 2), ("r", K.BASE_X[1] - size / 2)):
        b = K.box(f"joint-corner-{name}", "frame", K.BEECH, 0.25, (x - size / 2, x + size / 2), (0.13, 0.13 + size), (zf, zf + size), role="detail")
        out.append(own(K.repeat(b, 2, dw=(K.BASE_Z[1] - 0.045 - size) - zf), "frame", "joint"))
        if with_back:
            out.append(own(K.box(f"joint-back-bolt-{name}", "frame", K.BEECH, 0.2, (x - size / 2, x + size / 2), (0.44, 0.44 + max(size / 2, 0.01)), (0.405, 0.405 + max(size / 2, 0.01)), role="detail"), "frame", "joint"))
    return out


def supports(radius, height):
    """🟢 One felt glide under each foot as the support of its anchor."""
    out = []
    for anchor, x, z in (("foot-left-l", K.FOOT_X[0], K.FOOT_Z), ("foot-left-r", K.FOOT_X[0], -K.FOOT_Z), ("foot-right-l", K.FOOT_X[1], K.FOOT_Z), ("foot-right-r", K.FOOT_X[1], -K.FOOT_Z)):
        if height >= 0.075:
            g = K.box("support-" + anchor, "frame", K.BEECH, 0.2, (x - radius, x + radius), (0.0, height), (z - radius, z + radius), role="ground")
        else:
            g = K.turned("support-" + anchor, "frame", K.BEECH, 0.2, [(0, 0), (0, radius), (height, radius), (height, 0)], (x, 0.0, z), role="ground")
        out.append(own(g, "frame", "support", anchor=anchor))
    return out


def lumps(r):
    """⚫ The mass of every bone at its centre: frame (plinth, back frame, feet), arms, seat and back cushions."""
    m = K.masses()
    out = [own(K.ellipsoid("mass-frame", "frame", K.PLY, 0.3, (1.0, m["frame"][2], 0.1), (r, r, r)), "frame", "mass")]
    for bone, cx in ARMS:
        out.append(own(K.ellipsoid(f"mass-{bone}", bone, K.FABRIC, 0.3, (cx, m["arm"][2], 0.0), (r, r, r)), bone, "mass"))
    for k in range(3):
        xc = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        out.append(own(K.ellipsoid(f"mass-seat-{k + 1}", f"seat-{k + 1}", K.FOAM, 0.3, (xc, m["seat"][2], -0.145), (r, r, r)), f"seat-{k + 1}", "mass"))
        out.append(own(K.ellipsoid(f"mass-back-{k + 1}", f"back-{k + 1}", K.FOAM, 0.3, (xc, m["back"][2], 0.25), (r, r, r)), f"back-{k + 1}", "mass"))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale: the feet are body solids that are themselves the posts to the floor."""
    out = [{"id": "m-" + sid, "of": sid, "member": "frame", "part": "member"} for sid in ("foot-l", "foot-r")]
    if scale == "50":
        out += plinth(0.025, (0.115, 0.205), 0.018)
        out += back_frame((0.2125, 0.7333, 1.2667, 1.7875), 0.04, (0.76, 0.81), 0.025)
        for bone, cx in ARMS:
            out += arm_frame(bone, cx, 0.025, 0.018)
        out += cushion_bearings(0.03, 0.008, 0.15)
        out += corner_blocks(0.04, True)
        out += supports(0.022, 0.0075)
        out += lumps(0.02)
    elif scale == "100":
        out += plinth(0.025, (0.115, 0.205), 0.018)
        out += back_frame((0.2125, 1.7875), 0.04, (0.76, 0.81), 0.025)
        for bone, cx in ARMS:
            out += arm_frame(bone, cx, 0.03, 0.0)
        out += cushion_bearings(0.03, 0.015, 0.15)
        out += corner_blocks(0.04, True)
        out += supports(0.0225, 0.015)
        out += lumps(0.025)
    elif scale == "200":
        out += plinth(0.03, (0.11, 0.20), 0.03)
        out.append(own(K.box("member-back-panel", "frame", K.PLY, 0.3, K.BASE_X, (K.DECK_Y, 0.82), (0.40, 0.43)), "frame", "member"))
        for bone, cx in ARMS:
            out.append(own(K.box(f"member-{bone}", bone, K.PLY, 0.3, (cx - 0.015, cx + 0.015), (0.10, 0.58), (-0.42, 0.42)), bone, "member"))
        out += cushion_bearings(0.03, 0.0, 0.15)
        out += corner_blocks(0.04, False)
        out += supports(0.03, 0.03)
        out += lumps(0.03)
    else:
        rail = K.box("member-base-rail", "frame", K.BEECH, 0.3, (0.0375, 1.9625), (0.10, 0.20), (K.BASE_Z[0], K.BASE_Z[0] + 0.075))
        out.append(own(K.repeat(rail, 2, dw=(K.BASE_Z[1] - K.BASE_Z[0] - 0.075)), "frame", "member"))
        out.append(own(K.box("member-back-panel", "frame", K.PLY, 0.3, K.BASE_X, (K.DECK_Y, K.FRAME_TOP), (0.375, 0.45)), "frame", "member"))
        for bone, cx in ARMS:
            out.append(own(K.box(f"member-{bone}", bone, K.PLY, 0.3, (cx - 0.0375, cx + 0.0375), (0.10, 0.58), (-0.42, 0.42)), bone, "member"))
        for k in range(3):
            bs, bb = f"seat-{k + 1}", f"back-{k + 1}"
            out.append(own(K.box(f"member-{bs}", bs, K.FOAM, 0.4, (K.SEAT_X[k] + 0.01, K.SEAT_X[k + 1] - 0.01), (K.DECK_Y, K.DECK_Y + 0.075), (-0.275, -0.125)), bs, "member"))
            out.append(own(K.box(f"member-{bb}", bb, K.FOAM, 0.4, (K.SEAT_X[k] + 0.01, K.SEAT_X[k + 1] - 0.01), (0.38, 0.53), (0.27, 0.345)), bb, "member"))
        out += supports(0.0375, 0.075)
        out += lumps(0.0375)
    return out


DECISIONS = {
    "50": [
        ("Body, authored for 7.5 mm: four turned beech feet (45 mm at the floor, 60 mm under the frame, 100 mm high, with a bead at 70 mm); the plinth as a closed upholstered box (0.10 to 0.23 m: 18 mm ply over the base rails, padding and fabric, dust cover below) with a 20 mm round front edge, set 10 mm proud of the seat cushions; two box arms 0.20 x 0.90 m from 0.10 to 0.62 m with 60 mm top rolls and 15 mm bottom edges; the back frame between the arms with its rear face vertical, its inner face leaning 8 degrees and its top rolled to 0.85 m; three seat cushions 0.533 x 0.57 x 0.20 m on the deck with a 10 mm crown and 8 mm piping round the top and bottom boxing seams; three pillow back cushions 0.15 to 0.17 m thick standing on the deck against the frame face, with a lumbar belly and a rounded top at 0.80 m. Zips, staples, spring wires and the cavity inside the plinth are omitted (hidden or below 7.5 mm).",
         "Körper, für 7,5 mm gebaut: vier gedrechselte Buchenfüße (am Boden 45 mm, unter dem Rahmen 60 mm, 100 mm hoch, mit einem Wulst bei 70 mm); der Sockel als geschlossener Polsterkasten (0,10 bis 0,23 m: 18-mm-Sperrholz über den Sockelschienen, Polster und Stoff, unten Staubschutz) mit 20 mm gerundeter Vorderkante, 10 mm vor den Sitzkissen; zwei Kastenarmlehnen 0,20 x 0,90 m von 0,10 bis 0,62 m mit 60-mm-Oberrolle und 15-mm-Unterkanten; der Rückenrahmen zwischen den Armlehnen mit senkrechter Rückseite, um 8 Grad geneigter Innenseite und bis 0,85 m gerollter Oberkante; drei Sitzkissen 0,533 x 0,57 x 0,20 m auf dem Deck mit 10 mm Bombierung und 8-mm-Keder an der oberen und unteren Boxnaht; drei Rückenkissen 0,15 bis 0,17 m dick, auf dem Deck an der Rahmenfläche stehend, mit Lendenbauch und gerundeter Oberkante auf 0,80 m. Reißverschlüsse, Klammern, Federdrähte und der Hohlraum im Sockel fehlen (verdeckt oder unter 7,5 mm)."),
        ("Thermal: every region is its body surface with its layers nested inside at their real build-up (cushion foam cores, arm batting, back-frame batting and 18 mm ply, the ply core of the plinth under the fabric), feet as beech. Structure: the four feet as posts; the plinth's base rails (25 x 90 mm beech, through the arms) and its 18 mm ply deck; the back frame's bottom rail, four uprights and top rail; per arm two posts, top and bottom rail and the inner ply panel; per loose cushion its foam bearing band (30 x 150 mm along the cushion) on the deck under the sitting line or on the frame face at lumbar height. Joints: corner blocks of the base rails in the arm frames, bolts of the back frame into the arm back posts, the contact pads of the cushions. Supports: four felt glides at the foot anchors. Every bone carries its mass at its centre.",
         "Thermik: jede Region ist ihre Körperoberfläche, die Schichten liegen in ihrem realen Aufbau darin (Schaumkerne der Kissen, Vlies der Armlehnen, Vlies und 18-mm-Sperrholz des Rückenrahmens, der Sperrholzkern des Sockels unter dem Stoff), Füße aus Buche. Tragwerk: die vier Füße als Stützen; die Sockelschienen (Buche 25 x 90 mm, durch die Armlehnen) und das 18-mm-Sperrholzdeck; Unterholm, vier Stiele und Oberholm des Rückenrahmens; je Armlehne zwei Pfosten, Ober- und Unterholm und die innere Sperrholzplatte; je loses Kissen sein Schaumauflageband (30 x 150 mm längs des Kissens) auf dem Deck unter der Sitzlinie oder an der Rahmenfläche in Lendenhöhe. Gelenke: Eckklötze der Sockelschienen in den Armrahmen, Schrauben des Rückenrahmens in die hinteren Armpfosten, die Kontaktflächen der Kissen. Auflager: vier Filzgleiter an den Fußankern. Jeder Knochen trägt seine Masse im Schwerpunkt."),
    ],
    "100": [
        ("Body, authored anew for 15 mm: the same masses with fewer stations; the plinth with its round front edge; tapered feet without the bead (3 mm); arm rolls and cushion edges in two to three steps; seat cushions crowned, piping omitted (8 mm bead below the feature); back cushions with a simpler belly.",
         "Körper, für 15 mm neu gebaut: dieselben Massen mit weniger Stationen; der Sockel mit gerundeter Vorderkante; konische Füße ohne Wulst (3 mm); Armrollen und Kissenkanten in zwei bis drei Stufen; Sitzkissen bombiert, Keder entfallen (8-mm-Wulst unter dem Merkmal); Rückenkissen mit einfacherem Bauch."),
        ("Thermal: one surface per region, the layers merged into it. Structure: feet, base rails and deck, back rail, two end uprights and top rail, per arm posts and rails, foam bearing layers, corner blocks, back bolts and cushion pads as joints, glides, masses.",
         "Thermik: eine Oberfläche je Region, die Schichten darin zusammengefasst. Tragwerk: Füße, Sockelschienen und Deck, Unterholm, zwei Endstiele und Oberholm, je Armlehne Pfosten und Holme, Schaumauflagen, Eckklötze, Rückenschrauben und Kissenauflagen als Gelenke, Gleiter, Massen."),
    ],
    "200": [
        ("Body, authored anew for 30 mm: square plinth with a 30 mm round front edge, straight round feet, arm rolls and cushion tops as one chamfer step, the back frame with a chamfered top, back cushions in four stations.",
         "Körper, für 30 mm neu gebaut: rechteckiger Sockel mit 30 mm gerundeter Vorderkante, gerade runde Füße, Armrollen und Kissenoberseiten als eine Fasenstufe, der Rückenrahmen mit gefaster Oberkante, Rückenkissen in vier Stationen."),
        ("Thermal: one surface per region. Structure: feet, base rails and a 30 mm deck, one back panel, one 30 mm panel per arm as its frame, the cushion bearing layers, the arm corners as joints, glides, masses.",
         "Thermik: eine Oberfläche je Region. Tragwerk: Füße, Sockelschienen und ein 30-mm-Deck, eine Rückenplatte, je Armlehne eine 30-mm-Platte als ihr Rahmen, die Kissenauflagen, die Armecken als Gelenke, Gleiter, Massen."),
    ],
    "500": [
        ("Body, authored anew for 75 mm: one seat block (plinth and seat cushions merged, 0.10 to 0.43 m), one back block (back cushions and back frame merged, leaning face, 0.85 m), two arm blocks to 0.62 m and four 75 mm feet. The model is minimal: nothing smaller can be left out without losing the sofa.",
         "Körper, für 75 mm neu gebaut: ein Sitzblock (Sockel und Sitzkissen zusammengefasst, 0,10 bis 0,43 m), ein Rückenblock (Rückenkissen und Rückenrahmen zusammengefasst, geneigte Fläche, 0,85 m), zwei Armblöcke bis 0,62 m und vier 75-mm-Füße. Das Modell ist minimal: kleiner geht es nicht, ohne das Sofa zu verlieren."),
        ("Thermal: the blocks are split back into their regions by own solids (plinth, seat cushions, back cushions, back frame), arms and feet are their surfaces. Structure: the feet, two 75 mm base rails, a 75 mm back panel, a 75 mm panel per arm, 75 mm bearing bands of the cushions, glides of the foot size, masses.",
         "Thermik: die Blöcke sind durch eigene Körper wieder in ihre Regionen geteilt (Sockel, Sitzkissen, Rückenkissen, Rückenrahmen), Armlehnen und Füße sind ihre Oberflächen. Tragwerk: die Füße, zwei 75-mm-Sockelschienen, eine 75-mm-Rückenplatte, je Armlehne eine 75-mm-Platte, 75-mm-Auflagebänder der Kissen, Gleiter in Fußgröße, Massen."),
    ],
}


def decisions(scale):
    """💬 The bilingual decisions of a scale."""
    return [{"en": en, "de": de} for en, de in DECISIONS[scale]]
