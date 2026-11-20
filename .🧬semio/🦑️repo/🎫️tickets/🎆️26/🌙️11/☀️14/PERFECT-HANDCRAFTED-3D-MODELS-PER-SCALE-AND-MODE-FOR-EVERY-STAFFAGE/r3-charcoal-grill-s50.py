"""🔥 Round 3 charcoal kettle grill, 1:50 (smallest feature 7.5 mm): every part of the real kettle stated by hand."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-charcoal-grill-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

Y = [0, 1, 0]


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def side_handle(sid, z):
    secs = []
    for x, rx, ry in ((-0.065, 0.007, 0.008), (-0.061, 0.0105, 0.012), (-0.053, 0.0125, 0.0145), (-0.03, 0.013, 0.0155), (0.0, 0.0132, 0.0158), (0.03, 0.013, 0.0155), (0.053, 0.0125, 0.0145), (0.061, 0.0105, 0.012), (0.065, 0.007, 0.008)):
        secs.append({"at": (x, K.HANDLE_Y, z), "rx": rx, "ry": ry, "n": 2.6})
    return K.loft(sid, "bowl", "wood-handle", "body", 0.55, secs)


def side_bracket(sid, x, s):
    """🔩 Rod bracket of a side handle: from inside the grip to the bowl, its end cut flush in the shell's tangent plane."""
    zs = math.sqrt(K.bowl_r(K.HANDLE_Y) ** 2 - x * x)
    p1 = (x, K.HANDLE_Y, s * zs)
    return K.flush_loft(sid, "bowl", "steel-black", "detail", 0.25, (x, K.HANDLE_Y, s * (K.HANDLE_Z - 0.004)), p1, 0.005, n1=K.bowl_normal(p1))


def lid_bracket(sid, w, r=0.005):
    """🔩 Rod bracket of the lid handle: from inside the grip back to the dome, its end cut flush in the dome's tangent plane."""
    cv = K.C_L - K.LID_Y0
    us = 0.2925 + math.sqrt(K.RHO_L ** 2 - (K.LID_GRIP_V - cv) ** 2 - w * w)
    p1 = (us, K.LID_GRIP_V, w)
    return K.flush_loft(sid, "lid", "steel-black", "detail", 0.25, (K.LID_GRIP_U - 0.004, K.LID_GRIP_V, w), p1, r, n1=K.lid_normal(p1))


def vent_skirt(r1=0.0405, top=0.0045):
    """🌀 The dished seat of the vent dial that closes the flat dial plate onto the curved dome; under the three holes it is their bottom."""
    return [(K.dome_u(0.0) - 0.0004, 0.0), (K.dome_u(r1 * 0.5) - 0.0004, r1 * 0.5), (K.dome_u(r1) - 0.0004, r1), (top, r1), (top, 0.0)]


def lid_handle(sid="lid-handle"):
    secs = []
    for w, rx, ry in ((-0.08, 0.008, 0.007), (-0.076, 0.0118, 0.0102), (-0.068, 0.0145, 0.0125), (-0.04, 0.015, 0.013), (0.0, 0.0152, 0.0132), (0.04, 0.015, 0.013), (0.068, 0.0145, 0.0125), (0.076, 0.0118, 0.0102), (0.08, 0.008, 0.007)):
        secs.append({"at": (K.LID_GRIP_U, K.LID_GRIP_V, w), "rx": rx, "ry": ry, "n": 2.6})
    return K.loft(sid, "lid", "wood-handle", "body", 0.55, secs)


def bowl_envelope_profile():
    y_end = 0.7345
    a_end = math.asin(K.bowl_r(y_end) / K.RHO_B)
    out = [(K.C_B - K.RHO_B * math.cos(a_end * k / 22), K.RHO_B * math.sin(a_end * k / 22)) for k in range(23)]
    bead = [(K.BEAD_Y + K.BEAD_R * math.sin(math.radians(a)), K.RIM_R + K.BEAD_R * math.cos(math.radians(a))) for a in (-45, -20, 0, 25, 50, 70, 90)]
    return out + bead + [(K.BEAD_Y + K.BEAD_R, 0.0)]


def lid_envelope_profile():
    dome = [(y - K.LID_Y0, r) for r, y in K.lid_profile_outer(22) if r <= 0.2866]
    bv = K.LBEAD_Y - K.LID_Y0
    bead = [(bv + K.BEAD_R * math.sin(math.radians(a)), K.RIM_R + K.BEAD_R * math.cos(math.radians(a))) for a in (60, 35, 10, -15, -40, -65, -90)]
    return dome + bead + [(bv - K.BEAD_R, 0.0)]


def grate_bars(pitch, count_half, r, ids="grate-bar"):
    out = []
    for k in range(-count_half, count_half + 1):
        w = k * pitch
        half = math.sqrt(K.GRATE_R ** 2 - w * w)
        out.append(K.capsule("%s-%d" % (ids, k + count_half + 1), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R - half, 0.0, w), (K.GRATE_R + half, 0.0, w), r, slender=True))
    return out


def fuel_bars(pitch, count_half, r, ids="fuel-bar"):
    out = []
    v = K.FUEL_Y - K.BED_Y
    for k in range(-count_half, count_half + 1):
        w = k * pitch
        half = math.sqrt(K.FUEL_R ** 2 - w * w)
        out.append(K.capsule("%s-%d" % (ids, k + count_half + 1), "charcoal", "steel-black", "detail", 0.25, (K.BED_A - half, v, w), (K.BED_A + half, v, w), r, slender=True))
    return out


def rests(prefix, y_top, inward, azimuths=(90.0, 210.0, 330.0)):
    """🪝 Three riveted rests on the inner bowl wall; their outer face stays on the inner surface so nothing shows outside the 0.8 mm shell."""
    out = []
    y = y_top - 0.004
    r_in = K.bowl_r(y - 0.004, K.RHO_B - K.T_SH)
    for i, az in enumerate(azimuths):
        rc = r_in - inward / 2
        a = math.radians(az)
        out.append(K.box("%s-%d" % (prefix, i + 1), "bowl", "steel-black", "detail", 0.3, (inward, 0.008, 0.02), (rc * math.cos(a), y, rc * math.sin(a)), corner=0.0015, rotation=K.quat_y(-az)))
    return out


def grate_rests():
    return rests("grate-rest", K.GRATE_Y - 0.004, 0.011)


def hangers():
    out = []
    y = K.PAN_TOP + 0.0015
    for side in ("front", "left", "right"):
        a = math.radians(K.AZ[side])
        rl = K.leg_r(y)
        out.append(K.capsule("pan-hanger-%s" % side, "bowl", "steel-black", "detail", 0.25, (0.151 * math.cos(a), y, 0.151 * math.sin(a)), (rl * math.cos(a), y, rl * math.sin(a)), 0.004))
    return out


def triangle(r=K.WIRE_R, prefix="triangle", slender=True):
    v = {s: K.leg_point(s, K.TRI_Y) for s in ("front", "left", "right")}
    extra = {"slender": True} if slender else {}
    return [K.capsule("%s-%s" % (prefix, n), "bowl", "steel-black", "detail", 0.25, v[a], v[b], r, **extra) for n, a, b in (("front-left", "front", "left"), ("left-right", "left", "right"), ("right-front", "right", "front"))]


def wheels(hub=True, prefix="wheel"):
    out = []
    for side, sign in (("left", 1), ("right", -1)):
        out.append(K.revolve("%s-%s" % (prefix, side), "bowl", "steel-black", "body", 0.15, K.wheel_profile(hub), [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)))
    return out


def legs(bottom_front=0.022, bottom_wheel=0.062, r=K.LEG_R, slender=False, prefix="leg"):
    extra = {"slender": True} if slender else {}
    out = []
    for side in ("front", "left", "right"):
        y0 = bottom_front if side == "front" else bottom_wheel
        out.append(K.capsule("%s-%s" % (prefix, side), "bowl", "steel-black", "body", 0.25, K.leg_point(side, y0), K.leg_point(side, K.LEG_TOP_Y), r, **extra))
    return out


def body():
    b = []
    b.append(K.revolve("bowl-shell", "bowl", "enamel-steel", "body", 0.3, K.shell_profile(K.RHO_B, K.bowl_y(K.RIM_R)), Y, (0, 0, 0)))
    b.append(K.revolve("bowl-rim-bead", "bowl", "enamel-steel", "body", 0.3, K.torus(K.BEAD_Y, K.RIM_R, K.BEAD_R), Y, (0, 0, 0)))
    b += grate_rests()
    b += rests("fuel-rest", K.FUEL_Y - 0.003, 0.011, (30.0, 150.0, 270.0))
    for side, z in (("left", K.HANDLE_Z), ("right", -K.HANDLE_Z)):
        b.append(side_handle("side-handle-%s" % side, z))
        s = 1 if z > 0 else -1
        for tag, x in (("a", -0.045), ("b", 0.045)):
            b.append(side_bracket("side-bracket-%s-%s" % (side, tag), x, s))
    b.append(K.revolve("ash-pan", "bowl", "alu-cast", "body", 0.65, K.pan_profile(), Y, (0, 0, 0)))
    b += hangers()
    b += legs()
    for side in ("front", "left", "right"):
        b.append(K.socket_loft("leg-socket-%s" % side, side, 0.0165))
    b.append(K.foot_loft("foot-front"))
    b += wheels()
    b.append(K.capsule("axle", "bowl", "steel-black", "detail", 0.3, (K.axle_x(), K.AXLE_Y, -0.3), (K.axle_x(), K.AXLE_Y, 0.3), 0.005, slender=True))
    b += triangle()
    b.append(K.revolve("damper-plate", "ash-damper", "alu-cast", "body", 0.7, K.damper_profile(), [1, 0, 0], (0, 0, 0)))
    b.append(K.loft("damper-lever", "ash-damper", "alu-cast", "detail", 0.7, K.lever_sections()))
    b.append(K.ellipsoid("damper-knob", "ash-damper", "steel-black", "detail", 0.2, (0.009, 0.011, 0.0105), K.lever_point(K.LEVER_END + 0.007, 0.0105)))
    b.append(K.revolve("charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(), Y, (K.BED_A, 0, 0)))
    for i, (x, z, ang) in enumerate(K.BRIQUETTES):
        r = math.hypot(x, z)
        b.append(K.ellipsoid("briquette-%d" % (i + 1), "charcoal", "charcoal", "texture", 0.1, (0.024, 0.012, 0.019), (K.BED_A + x, K.bed_surface(r) + 0.004, z), rotation=K.quat_y(ang), texture=True))
    b.append(K.revolve("fuel-grate-ring", "charcoal", "steel-black", "detail", 0.25, K.torus(K.FUEL_Y - K.BED_Y, K.FUEL_R, 0.003, 12), Y, (K.BED_A, 0, 0), slender=True))
    b += fuel_bars(0.042, 4, 0.003)
    b.append(K.revolve("grate-ring", "grate", "chrome-steel", "detail", 0.75, K.torus(0.0, K.GRATE_R, 0.004, 12), Y, (K.GRATE_R, 0, 0), slender=True))
    b += grate_bars(K.GRATE_PITCH, 13, 0.003)
    for i, du in enumerate((-0.09, 0.09)):
        half = math.sqrt(K.GRATE_R ** 2 - du * du)
        b.append(K.capsule("grate-cross-%d" % (i + 1), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R + du, -0.0065, -half), (K.GRATE_R + du, -0.0065, half), 0.0035, slender=True))
    b.append(K.revolve("lid-shell", "lid", "enamel-steel", "body", 0.3, K.lid_shell_profile(), Y, (0.2925, 0, 0)))
    b.append(K.revolve("lid-rim-bead", "lid", "enamel-steel", "body", 0.3, K.torus(K.LBEAD_Y - K.LID_Y0, K.RIM_R, K.BEAD_R), Y, (0.2925, 0, 0)))
    b.append(lid_handle())
    for side, w in (("left", 0.062), ("right", -0.062)):
        b.append(lid_bracket("lid-bracket-%s" % side, w))
    holes = [K.circle(K.HOLE_R, 16, v, w) for v, w in K.HOLES_VW]
    b.append(K.extrude("vent-dial", "lid-vent", "alu-cast", "body", 0.7, K.circle(K.DIAL_R, 32), "yz", 0.0, K.DIAL_U1, holes=holes))
    b.append(K.revolve("vent-skirt", "lid-vent", "alu-cast", "body", 0.7, vent_skirt(), [1, 0, 0], (0, 0, 0)))
    b.append(K.box("vent-fin", "lid-vent", "alu-cast", "body", 0.7, (K.FIN_U1 - 0.007, 0.07, 0.008), ((K.FIN_U1 + 0.007) / 2, 0.0, 0.0), corner=0.003))
    return b


def thermal(b):
    ids = {s["id"] for s in b}
    t = []
    t.append(K.revolve("th-bowl-envelope", "bowl", "enamel-steel", "body", 0.3, bowl_envelope_profile(), Y, (0, 0, 0), region="bowl-shell", sectionFaces=["+v"]))
    for s in sorted(i for i in ids if i.startswith("side-bracket")):
        t.append(ref("th-" + s, s, region="bowl-shell"))
    t.append(K.revolve("open-bowl-plume-mouth", "bowl", "enamel-steel", "detail", 0.3, [(0.7305, 0.0), (0.7305, 0.2), (0.738, 0.2), (0.738, 0.0)], Y, (0, 0, 0), region="bowl-shell", tags=["open-bowl-plume"]))
    t.append(K.revolve("th-lid-envelope", "lid", "enamel-steel", "body", 0.3, lid_envelope_profile(), Y, (0.2925, 0, 0), region="lid-shell", sectionFaces=["-v"]))
    for s in ("lid-bracket-left", "lid-bracket-right"):
        t.append(ref("th-" + s, s, region="lid-shell"))
    for s in ("leg-front", "leg-left", "leg-right", "leg-socket-front", "leg-socket-left", "leg-socket-right", "foot-front", "wheel-left", "wheel-right", "axle", "triangle-front-left", "triangle-left-right", "triangle-right-front", "pan-hanger-front", "pan-hanger-left", "pan-hanger-right"):
        t.append(ref("th-" + s, s, region="legs-steel"))
    t.append(ref("th-ash-pan", "ash-pan", region="ash-catcher"))
    for s in ("vent-dial", "vent-skirt", "vent-fin", "damper-plate", "damper-lever", "damper-knob"):
        t.append(ref("th-" + s, s, region="vents-alu"))
    t.append(K.revolve("lid-vent-flue-outlet", "lid-vent", "alu-cast", "detail", 0.7, [(K.DIAL_U1 - 0.0005, 0.0), (K.DIAL_U1 - 0.0005, 0.025), (K.DIAL_U1 + 0.0075, 0.025), (K.DIAL_U1 + 0.0075, 0.0)], [1, 0, 0], (0, 0, 0), region="vents-alu", tags=["lid-vent-flue"]))
    for s in ("side-handle-left", "side-handle-right", "lid-handle"):
        t.append(ref("th-" + s, s, region="handle-wood"))
    for s in b:
        if s["id"].startswith("grate-") and not s["id"].startswith("grate-rest"):
            t.append(hidden_copy(s, "cooking-grate", "rods"))
    t.append(K.revolve("th-charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(), Y, (K.BED_A, 0, 0), region="charcoal-bed", layer="embers", sectionFaces=["-v"]))
    return t


def structure(b):
    ids = [s["id"] for s in b]
    st = [ref("member-bowl-shell", "bowl-shell", member="bowl", part="member")]
    for s in ("leg-front", "leg-left", "leg-right", "triangle-front-left", "triangle-left-right", "triangle-right-front", "axle"):
        st.append(ref("member-" + s, s, member="bowl", part="member"))
    for s in ("leg-socket-front", "leg-socket-left", "leg-socket-right"):
        st.append(ref("joint-" + s, s, member="bowl", part="joint"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.revolve("joint-wheel-bearing-%s" % side, "bowl", "steel-black", "detail", 0.3, [(-0.022, 0.0), (-0.022, 0.012), (0.026, 0.012), (0.026, 0.0)], [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)), member="bowl", part="joint"))
    st.append(dict(K.box("support-leg-front", "bowl", "steel-black", "body", 0.3, (0.04, 0.008, 0.04), (K.FOOT_X, 0.004, 0.0)), member="bowl", part="support", anchor="leg-front"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.box("support-wheel-%s" % side, "bowl", "steel-black", "body", 0.3, (0.03, 0.008, 0.03), (K.axle_x(), 0.004, sign * K.WHEEL_Z)), member="bowl", part="support", anchor="wheel-%s" % side))
    st.append(dict(K.ellipsoid("mass-bowl", "bowl", "steel-black", "detail", 0.3, (0.03, 0.0075, 0.03), (-0.017, 0.485, 0.0)), member="bowl", part="mass"))
    for s in ids:
        if s == "grate-ring" or s.startswith("grate-bar") or s.startswith("grate-cross"):
            st.append(ref("member-" + s, s, member="grate", part="member"))
    for s in ("grate-rest-1", "grate-rest-2", "grate-rest-3"):
        st.append(ref("joint-" + s, s, member="grate", part="joint"))
    for s in ("fuel-rest-1", "fuel-rest-2", "fuel-rest-3"):
        st.append(ref("joint-" + s, s, member="charcoal", part="joint"))
    st.append(dict(K.ellipsoid("mass-grate", "grate", "steel-black", "detail", 0.3, (0.03, 0.01, 0.03), (K.GRATE_R, 0.0, 0.0)), member="grate", part="mass"))
    for s in ids:
        if s == "fuel-grate-ring" or s.startswith("fuel-bar"):
            st.append(ref("member-" + s, s, member="charcoal", part="member"))
    st.append(dict(K.ellipsoid("mass-charcoal", "charcoal", "steel-black", "detail", 0.3, (0.03, 0.015, 0.03), (K.BED_A, 0.018, 0.0)), member="charcoal", part="mass"))
    st.append(ref("member-lid-shell", "lid-shell", member="lid", part="member"))
    st.append(dict(K.capsule("joint-lid-hinge", "lid", "steel-black", "detail", 0.3, (0.004, 0.006, -0.02), (0.004, 0.006, 0.02), 0.004), member="lid", part="joint"))
    st.append(dict(K.ellipsoid("mass-lid", "lid", "steel-black", "detail", 0.3, (0.03, 0.03, 0.03), (0.331, 0.11, 0.0)), member="lid", part="mass"))
    st.append(ref("member-vent-dial", "vent-dial", member="lid-vent", part="member"))
    st.append(dict(K.capsule("joint-vent-pivot", "lid-vent", "steel-black", "detail", 0.3, (-0.006, 0, 0), (0.012, 0, 0), 0.004), member="lid-vent", part="joint"))
    st.append(ref("member-damper-plate", "damper-plate", member="ash-damper", part="member"))
    st.append(dict(K.capsule("joint-damper-pivot", "ash-damper", "steel-black", "detail", 0.3, (-0.008, 0, 0), (0.0, 0, 0), 0.004), member="ash-damper", part="joint"))
    return st


def hidden_copy(solid, region, layer):
    """🙈 A hidden region is its own solid inside the body (B4): the same rod stated again in the thermal list, never a reference to the body."""
    c = {k: v for k, v in solid.items() if k != "role"}
    c.update({"id": "th-" + solid["id"], "role": "detail", "region": region, "layer": layer})
    return c


def text(en, de):
    return {"en": en, "de": de}


DECISIONS = [
    text("Body: a 57 cm kettle of the Weber Original Kettle class as a real thin-walled object. Bowl and lid are 0.8 mm enamelled sphere zones (bowl radius 290 mm, 257 mm deep; lid dome radius 294 mm, 236 mm high) with a rolled 8.4 mm rim bead each, so the raised lid shows the inside. Inside: the cooking grate (546 mm ring of 8 mm rod, 27 bars of 6 mm at 19.5 mm pitch on two 7 mm cross bars) resting on three rests, the fuel grate (416 mm ring, nine bars) and the charcoal as a 390 mm mound with briquettes. Outside: two wooden side handles on rod brackets, the wooden lid handle on two flat brackets, the 80 mm vent dial with three 12 mm holes and a grip fin, the dished bottom damper with its lever, a 300 mm aluminium ash pan hung 65 mm below the bowl from the three legs.",
         "Körper: ein 57-cm-Kugelgrill der Klasse Weber Original Kettle als echtes dünnwandiges Objekt. Kessel und Deckel sind emaillierte Kugelzonen von 0,8 mm (Kesselradius 290 mm, 257 mm tief; Deckelkuppel Radius 294 mm, 236 mm hoch) mit je einem gerollten Randwulst von 8,4 mm, so dass der angehobene Deckel das Innere zeigt. Innen: der Grillrost (Ring 546 mm aus 8-mm-Stab, 27 Stäbe von 6 mm im Abstand von 19,5 mm auf zwei Querstäben von 7 mm) auf drei Auflagen, der Kohlerost (Ring 416 mm, neun Stäbe) und die Kohle als Haufen von 390 mm mit Briketts. Außen: zwei hölzerne Seitengriffe auf Rundstabhaltern, der hölzerne Deckelgriff auf zwei Flachhaltern, die Lüftungsscheibe von 80 mm mit drei Löchern von 12 mm und einer Griffrippe, der gewölbte Bodenschieber mit Hebel, eine Aluminium-Aschewanne von 300 mm, 65 mm unter dem Kessel an den drei Beinen aufgehängt."),
    text("Legs: a tripod of 25 mm tubes in flush leg sockets, splayed 10.8 degrees: one front leg on a plastic foot under the lid handle, two rear legs ending at the 10 mm axle of two 152 mm (6 in) wheels with hub caps; a triangle of 8 mm wire braces the legs 240 mm above the ground. Thermal: bowl and lid are their outer envelopes (rim faces as section faces, the shells are thin enough to count once), every other region references its body parts; the charcoal mound and the grate rods are the hidden regions inside the bowl; the flue leaves through a 50 mm outlet patch on the dial, the open-bowl plume through a 400 mm disc in the mouth.",
         "Beine: ein Dreibein aus 25-mm-Rohren in bündigen Beinhaltern, um 10,8 Grad gespreizt: ein vorderes Bein auf Kunststofffuß unter dem Deckelgriff, zwei hintere Beine, die an der 10-mm-Achse zweier Räder von 152 mm (6 Zoll) mit Nabenkappen enden; ein Dreieck aus 8-mm-Draht steift die Beine 240 mm über dem Boden aus. Thermik: Kessel und Deckel sind ihre äußeren Hüllen (Randflächen als Schnittflächen, die Schalen sind dünn genug, um einmal zu zählen), jede andere Region verweist auf ihre Körperteile; Kohlehaufen und Roststäbe sind die verdeckten Regionen im Kessel; das Abgas tritt durch eine Austrittsfläche von 50 mm auf der Scheibe aus, die Fahne über dem offenen Kessel durch eine Scheibe von 400 mm in der Öffnung."),
    text("Structure: the load path of the real kettle. The bowl shell is the member that carries grate, charcoal and lid; it stands on the three leg tubes through their sockets (joints) and the triangle and axle brace them; the supports are the foot pad and the two wheel contacts; the wheel bearings, the lid hinge line at the rear rim, the vent and damper pivots and the three grate rests are joints; the cooking grate and the fuel grate are their own rod members; masses sit at the centres of mass of bowl assembly, lid, grate and charcoal.",
         "Tragwerk: der Lastpfad des echten Grills. Die Kesselschale ist das Bauteil, das Rost, Kohle und Deckel trägt; sie steht über die Beinhalter (Gelenke) auf den drei Beinrohren, die Dreieck und Achse aussteifen; die Auflager sind die Fußplatte und die beiden Radaufstandspunkte; Radlager, Deckeldrehlinie am hinteren Rand, Drehpunkte von Scheibe und Schieber und die drei Rostauflagen sind Gelenke; Grill- und Kohlerost sind eigene Stabbauteile; Massen sitzen in den Schwerpunkten von Kesselbaugruppe, Deckel, Rost und Kohle."),
]


def volume():
    b = body()
    return {"status": "authored", "denominator": 50, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
