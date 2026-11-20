"""🔥 Round 3 charcoal kettle grill, 1:100 (smallest feature 15 mm): the kettle authored anew for 15 mm, beads, holes, fins and the bottom damper merged or left to the thermal list."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-charcoal-grill-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

Y = [0, 1, 0]
RIM = 0.742


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def hidden_copy(solid, region, layer):
    """🙈 A hidden region is its own solid inside the body (B4): the part stated again in the thermal list, never a reference to the body."""
    c = {k: v for k, v in solid.items() if k != "role"}
    c.update({"id": "th-" + solid["id"], "role": "detail", "region": region, "layer": layer})
    return c


def text(en, de):
    return {"en": en, "de": de}


def side_handle(sid, z):
    secs = [{"at": (x, K.HANDLE_Y, z), "rx": rx, "ry": ry, "n": 3} for x, rx, ry in ((-0.065, 0.0085, 0.0095), (-0.056, 0.013, 0.0155), (0.0, 0.0132, 0.0158), (0.056, 0.013, 0.0155), (0.065, 0.0085, 0.0095))]
    return K.loft(sid, "bowl", "wood-handle", "body", 0.55, secs)


def side_bracket(sid, x, s):
    zs = math.sqrt(K.bowl_r(K.HANDLE_Y) ** 2 - x * x)
    p1 = (x, K.HANDLE_Y, s * zs)
    return K.flush_loft(sid, "bowl", "steel-black", "detail", 0.25, (x, K.HANDLE_Y, s * (K.HANDLE_Z - 0.004)), p1, 0.0075, n1=K.bowl_normal(p1))


def lid_handle():
    secs = [{"at": (K.LID_GRIP_U, K.LID_GRIP_V, w), "rx": rx, "ry": ry, "n": 3} for w, rx, ry in ((-0.08, 0.0095, 0.0085), (-0.07, 0.0148, 0.0128), (0.0, 0.0152, 0.0132), (0.07, 0.0148, 0.0128), (0.08, 0.0095, 0.0085))]
    return K.loft("lid-handle", "lid", "wood-handle", "body", 0.55, secs)


def lid_bracket(sid, w):
    cv = K.C_L - K.LID_Y0
    us = 0.2925 + math.sqrt(K.RHO_L ** 2 - (K.LID_GRIP_V - cv) ** 2 - w * w)
    p1 = (us, K.LID_GRIP_V, w)
    return K.flush_loft(sid, "lid", "steel-black", "detail", 0.25, (K.LID_GRIP_U - 0.004, K.LID_GRIP_V, w), p1, 0.0075, n1=K.lid_normal(p1))


def pan_profile():
    b, t, r = K.PAN_BOTTOM, K.PAN_TOP, K.PAN_R
    return [(b, 0.0), (b, r - 0.01), (b + 0.006, r - 0.001), (t, r + 0.004), (t, r + 0.0005), (b + 0.007, r - 0.0035), (b + 0.002, r - 0.011), (b + 0.002, 0.0)]


def legs():
    out = []
    for side in ("front", "left", "right"):
        top = K.leg_point(side, K.socket_top_y())
        if side == "front":
            out.append(K.flush_loft("leg-front", "bowl", "steel-black", "body", 0.25, K.leg_point(side, 0.0), top, 0.0135, n0=(0.0, -1.0, 0.0), n1=K.bowl_normal(top), r_mid=0.013, inset=0.0004))
        else:
            out.append(K.flush_loft("leg-%s" % side, "bowl", "steel-black", "body", 0.25, K.leg_point(side, 0.062), top, 0.0125, n1=K.bowl_normal(top), r_mid=0.013))
    return out


def hangers():
    out = []
    y = K.PAN_TOP - 0.003
    for side in ("front", "left", "right"):
        a = math.radians(K.AZ[side])
        rl = K.leg_r(y)
        out.append(K.capsule("pan-hanger-%s" % side, "bowl", "steel-black", "detail", 0.25, (0.146 * math.cos(a), y, 0.146 * math.sin(a)), (rl * math.cos(a), y, rl * math.sin(a)), 0.0075))
    return out


def body():
    b = [K.revolve("bowl-shell", "bowl", "enamel-steel", "body", 0.3, K.shell_profile(K.RHO_B, RIM, n=18), Y, (0, 0, 0))]
    for prefix, y_top, azs in (("grate-rest", K.GRATE_Y - 0.004, (90.0, 210.0, 330.0)), ("fuel-rest", K.FUEL_Y - 0.003, (30.0, 150.0, 270.0))):
        r_in = K.bowl_r(y_top - 0.015, K.RHO_B - K.T_SH)
        for i, az in enumerate(azs):
            a = math.radians(az)
            rc = r_in - 0.0075
            b.append(K.box("%s-%d" % (prefix, i + 1), "bowl", "steel-black", "detail", 0.3, (0.015, 0.015, 0.02), (rc * math.cos(a), y_top - 0.0075, rc * math.sin(a)), corner=0.002, rotation=K.quat_y(-az)))
    for side, s in (("left", 1), ("right", -1)):
        b.append(side_handle("side-handle-%s" % side, s * K.HANDLE_Z))
        for tag, x in (("a", -0.045), ("b", 0.045)):
            b.append(side_bracket("side-bracket-%s-%s" % (side, tag), x, s))
    b.append(K.revolve("ash-pan", "bowl", "alu-cast", "body", 0.65, pan_profile(), Y, (0, 0, 0)))
    b += hangers()
    b += legs()
    for side, sign in (("left", 1), ("right", -1)):
        b.append(K.revolve("wheel-%s" % side, "bowl", "steel-black", "body", 0.15, K.wheel_profile_wide(0.038), [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)))
    b.append(K.capsule("axle", "bowl", "steel-black", "detail", 0.3, (K.axle_x(), K.AXLE_Y, -0.3), (K.axle_x(), K.AXLE_Y, 0.3), 0.005, slender=True))
    v = {s: K.leg_point(s, K.TRI_Y) for s in ("front", "left", "right")}
    for n, a, c in (("front-left", "front", "left"), ("left-right", "left", "right"), ("right-front", "right", "front")):
        b.append(K.capsule("triangle-%s" % n, "bowl", "steel-black", "detail", 0.25, v[a], v[c], 0.004, slender=True))
    b.append(K.revolve("charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(n=8), Y, (K.BED_A, 0, 0)))
    b.append(K.revolve("fuel-grate-ring", "charcoal", "steel-black", "detail", 0.25, K.torus(K.FUEL_Y - K.BED_Y, K.FUEL_R, 0.003, 10), Y, (K.BED_A, 0, 0), slender=True))
    for k in range(-2, 3):
        w = k * 0.07
        half = math.sqrt(K.FUEL_R ** 2 - w * w)
        b.append(K.capsule("fuel-bar-%d" % (k + 3), "charcoal", "steel-black", "detail", 0.25, (K.BED_A - half, K.FUEL_Y - K.BED_Y, w), (K.BED_A + half, K.FUEL_Y - K.BED_Y, w), 0.003, slender=True))
    b.append(K.revolve("grate-ring", "grate", "chrome-steel", "detail", 0.75, K.torus(0.0, K.GRATE_R, 0.004, 10), Y, (K.GRATE_R, 0, 0), slender=True))
    for k in range(-6, 7):
        w = k * 0.039
        half = math.sqrt(K.GRATE_R ** 2 - w * w)
        b.append(K.capsule("grate-bar-%d" % (k + 7), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R - half, 0.0, w), (K.GRATE_R + half, 0.0, w), 0.003, slender=True))
    for i, du in enumerate((-0.09, 0.09)):
        half = math.sqrt(K.GRATE_R ** 2 - du * du)
        b.append(K.capsule("grate-cross-%d" % (i + 1), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R + du, -0.0065, -half), (K.GRATE_R + du, -0.0065, half), 0.0035, slender=True))
    b.append(K.revolve("lid-shell", "lid", "enamel-steel", "body", 0.3, K.lid_shell_to(RIM, n=18), Y, (0.2925, 0, 0)))
    b.append(lid_handle())
    for side, w in (("left", 0.062), ("right", -0.062)):
        b.append(lid_bracket("lid-bracket-%s" % side, w))
    b.append(K.revolve("vent-dial", "lid-vent", "alu-cast", "body", 0.7, K.dial_profile(K.FIN_U1), [1, 0, 0], (0, 0, 0)))
    return b


def thermal(b):
    t = [K.revolve("th-bowl-envelope", "bowl", "enamel-steel", "body", 0.3, K.bowl_envelope_to(RIM, 18), Y, (0, 0, 0), region="bowl-shell", sectionFaces=["+v"])]
    for side in ("left", "right"):
        for tag in ("a", "b"):
            t.append(ref("th-side-bracket-%s-%s" % (side, tag), "side-bracket-%s-%s" % (side, tag), region="bowl-shell"))
    t.append(K.revolve("open-bowl-plume-mouth", "bowl", "enamel-steel", "detail", 0.3, [(0.725, 0.0), (0.725, 0.2), (0.74, 0.2), (0.74, 0.0)], Y, (0, 0, 0), region="bowl-shell", tags=["open-bowl-plume"]))
    t.append(K.revolve("th-lid-envelope", "lid", "enamel-steel", "body", 0.3, K.lid_envelope_from(RIM, 18), Y, (0.2925, 0, 0), region="lid-shell", sectionFaces=["-v"]))
    for s in ("lid-bracket-left", "lid-bracket-right"):
        t.append(ref("th-" + s, s, region="lid-shell"))
    for s in ("leg-front", "leg-left", "leg-right", "wheel-left", "wheel-right", "axle", "triangle-front-left", "triangle-left-right", "triangle-right-front", "pan-hanger-front", "pan-hanger-left", "pan-hanger-right"):
        t.append(ref("th-" + s, s, region="legs-steel"))
    t.append(ref("th-ash-pan", "ash-pan", region="ash-catcher"))
    t.append(ref("th-vent-dial", "vent-dial", region="vents-alu"))
    t.append(K.revolve("th-damper-plate", "ash-damper", "alu-cast", "body", 0.7, K.damper_profile(r_plate=0.065, t=0.0155, n=4), [1, 0, 0], (0, 0, 0), region="vents-alu"))
    t.append(K.revolve("lid-vent-flue-outlet", "lid-vent", "alu-cast", "detail", 0.7, [(K.FIN_U1 - 0.015, 0.0), (K.FIN_U1 - 0.015, 0.025), (K.FIN_U1 + 0.0005, 0.025), (K.FIN_U1 + 0.0005, 0.0)], [1, 0, 0], (0, 0, 0), region="vents-alu", tags=["lid-vent-flue"]))
    for s in ("side-handle-left", "side-handle-right", "lid-handle"):
        t.append(ref("th-" + s, s, region="handle-wood"))
    t.append(hidden_copy(K.revolve("grate-ring", "grate", "chrome-steel", "detail", 0.75, K.torus(0.0, K.GRATE_R, 0.004, 10), Y, (K.GRATE_R, 0, 0), slender=True), "cooking-grate", "rods"))
    for k in range(-13, 14):
        w = k * K.GRATE_PITCH
        half = math.sqrt(K.GRATE_R ** 2 - w * w)
        t.append(hidden_copy(K.capsule("grate-rod-%d" % (k + 14), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R - half, 0.0, w), (K.GRATE_R + half, 0.0, w), 0.003, slender=True), "cooking-grate", "rods"))
    for s in b:
        if s["id"].startswith("grate-cross"):
            t.append(hidden_copy(s, "cooking-grate", "rods"))
    t.append(K.revolve("th-charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(n=8), Y, (K.BED_A, 0, 0), region="charcoal-bed", layer="embers", sectionFaces=["-v"]))
    return t


def structure(b):
    st = [ref("member-bowl-shell", "bowl-shell", member="bowl", part="member")]
    for s in ("leg-front", "leg-left", "leg-right", "triangle-front-left", "triangle-left-right", "triangle-right-front", "axle"):
        st.append(ref("member-" + s, s, member="bowl", part="member"))
    for side in ("front", "left", "right"):
        p = K.leg_point(side, K.socket_top_y() - 0.012)
        st.append(dict(K.ellipsoid("joint-leg-%s" % side, "bowl", "steel-black", "detail", 0.3, (0.0175, 0.0175, 0.0175), p), member="bowl", part="joint"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.revolve("joint-wheel-bearing-%s" % side, "bowl", "steel-black", "detail", 0.3, [(-0.022, 0.0), (-0.022, 0.016), (0.022, 0.016), (0.022, 0.0)], [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)), member="bowl", part="joint"))
    st.append(dict(K.box("support-leg-front", "bowl", "steel-black", "body", 0.3, (0.04, 0.015, 0.04), (K.FOOT_X, 0.0075, 0.0)), member="bowl", part="support", anchor="leg-front"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.box("support-wheel-%s" % side, "bowl", "steel-black", "body", 0.3, (0.03, 0.015, 0.03), (K.axle_x(), 0.0075, sign * K.WHEEL_Z)), member="bowl", part="support", anchor="wheel-%s" % side))
    st.append(dict(K.ellipsoid("mass-bowl", "bowl", "steel-black", "detail", 0.3, (0.03, 0.0075, 0.03), (-0.017, 0.485, 0.0)), member="bowl", part="mass"))
    for s in [x["id"] for x in b]:
        if s == "grate-ring" or s.startswith("grate-bar") or s.startswith("grate-cross"):
            st.append(ref("member-" + s, s, member="grate", part="member"))
    for i in (1, 2, 3):
        st.append(ref("joint-grate-rest-%d" % i, "grate-rest-%d" % i, member="grate", part="joint"))
        st.append(ref("joint-fuel-rest-%d" % i, "fuel-rest-%d" % i, member="charcoal", part="joint"))
    st.append(dict(K.ellipsoid("mass-grate", "grate", "steel-black", "detail", 0.3, (0.03, 0.0075, 0.03), (K.GRATE_R, 0.0, 0.0)), member="grate", part="mass"))
    for s in [x["id"] for x in b]:
        if s == "fuel-grate-ring" or s.startswith("fuel-bar"):
            st.append(ref("member-" + s, s, member="charcoal", part="member"))
    st.append(dict(K.ellipsoid("mass-charcoal", "charcoal", "steel-black", "detail", 0.3, (0.03, 0.0075, 0.03), (K.BED_A, 0.018, 0.0)), member="charcoal", part="mass"))
    st.append(ref("member-lid-shell", "lid-shell", member="lid", part="member"))
    st.append(dict(K.capsule("joint-lid-hinge", "lid", "steel-black", "detail", 0.3, (0.008, 0.003, -0.025), (0.008, 0.003, 0.025), 0.0075), member="lid", part="joint"))
    st.append(dict(K.ellipsoid("mass-lid", "lid", "steel-black", "detail", 0.3, (0.03, 0.03, 0.03), (0.331, 0.11, 0.0)), member="lid", part="mass"))
    st.append(ref("member-vent-dial", "vent-dial", member="lid-vent", part="member"))
    st.append(dict(K.capsule("joint-vent-pivot", "lid-vent", "steel-black", "detail", 0.3, (-0.002, 0, 0), (0.014, 0, 0), 0.0075), member="lid-vent", part="joint"))
    return st


DECISIONS = [
    text("Body authored anew for the 15 mm feature: bowl and lid as 0.8 mm sphere-zone shells meeting at the 742 mm rim (the 8.4 mm beads are below the feature and merged into the shells), side and lid handles as rounded wooden grips on brackets thickened to 15 mm, the vent dial as one dished 80 x 22 mm cap (holes and grip fin dropped), the ash pan as a plain 2 mm pan on 15 mm hangers, legs reaching flush into the bowl with sockets and foot merged, 152 x 38 mm wheels without hub caps, the wire triangle and the axle at their true 8 and 10 mm as slender parts. Inside: the cooking grate as ring and 13 bars at 39 mm (the real 19.5 mm pitch is below the feature), the fuel grate as ring and five bars, the charcoal as one mound.",
         "Körper neu für die kleinste Einzelheit von 15 mm: Kessel und Deckel als Kugelzonenschalen von 0,8 mm, die sich am Rand in 742 mm Höhe treffen (die Wülste von 8,4 mm liegen unter der Einzelheit und gehen in den Schalen auf), Seiten- und Deckelgriff als gerundete Holzgriffe auf Haltern, die auf 15 mm verstärkt sind, die Lüftungsscheibe als eine gewölbte Kappe von 80 x 22 mm (Löcher und Griffrippe entfallen), die Aschewanne als schlichte Wanne von 2 mm an Haltern von 15 mm, Beine, die bündig in den Kessel reichen, mit eingeschlossenen Haltern und Fuß, Räder von 152 x 38 mm ohne Nabenkappen, Drahtdreieck und Achse in wahrer Stärke von 8 und 10 mm als schlanke Teile. Innen: der Grillrost als Ring und 13 Stäbe im Abstand von 39 mm (der wahre Abstand von 19,5 mm liegt unter der Einzelheit), der Kohlerost als Ring und fünf Stäbe, die Kohle als ein Haufen."),
    text("Thermal: bowl and lid as their outer envelopes up to the rim, every other region from its body parts; the bottom damper (6 mm plate) is not drawn at this scale and lives only in the thermal list so the vents region keeps its area; the hidden grate keeps all 27 real rods in the thermal list (the drawing shows every second one) and the charcoal mound is the other hidden region; flue and open-bowl plume as outlet patches.",
         "Thermik: Kessel und Deckel als ihre äußeren Hüllen bis zum Rand, jede andere Region aus ihren Körperteilen; der Bodenschieber (Platte von 6 mm) ist in diesem Maßstab nicht gezeichnet und lebt nur in der Thermikliste, damit die Lüftungsregion ihre Fläche behält; der verdeckte Rost behält in der Thermikliste alle 27 echten Stäbe (die Zeichnung zeigt jeden zweiten), der Kohlehaufen ist die andere verdeckte Region; Abgas und Fahne als Austrittsflächen."),
    text("Structure: bowl shell, three legs, triangle and axle as members; leg-to-bowl joints, wheel bearings, lid hinge, the vent pivot and three grate rests as joints; foot and wheel contacts as supports; masses of bowl assembly, lid, grate and charcoal at their centres of mass.",
         "Tragwerk: Kesselschale, drei Beine, Dreieck und Achse als Stäbe; Bein-Kessel-Anschlüsse, Radlager, Deckeldrehlinie, Drehpunkt der Scheibe und drei Rostauflagen als Gelenke; Fuß- und Radaufstand als Auflager; Massen von Kesselbaugruppe, Deckel, Rost und Kohle in ihren Schwerpunkten."),
]


def volume():
    b = body()
    return {"status": "authored", "denominator": 100, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
