"""🔥 Round 3 charcoal kettle grill, 1:200 (smallest feature 30 mm): the kettle authored anew for 30 mm: shells, block handles, dial cap, pan against the bowl, slender legs on true wheels."""
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


def pan_profile():
    """🍳 The pan raised against the bowl: 30 mm deep, its top following the bowl underside."""
    top = [(K.bowl_y(r) - 0.0004, r) for r in (0.15, 0.1, 0.05, 0.0)]
    return [(K.BOWL_BOTTOM - 0.03, 0.0), (K.BOWL_BOTTOM - 0.03, 0.14), (K.BOWL_BOTTOM - 0.022, 0.15)] + top


def legs():
    out = []
    for side in ("front", "left", "right"):
        top = K.leg_point(side, K.socket_top_y())
        if side == "front":
            out.append(K.flush_loft("leg-front", "bowl", "steel-black", "body", 0.25, K.leg_point(side, 0.0), top, 0.0125, n0=(0.0, -1.0, 0.0), n1=K.bowl_normal(top), inset=0.0004, slender=True))
        else:
            out.append(K.flush_loft("leg-%s" % side, "bowl", "steel-black", "body", 0.25, K.leg_point(side, K.AXLE_Y - 0.012), top, 0.0125, n1=K.bowl_normal(top), slender=True))
    return out


def body():
    b = [K.revolve("bowl-shell", "bowl", "enamel-steel", "body", 0.3, K.shell_profile(K.RHO_B, RIM, n=14), Y, (0, 0, 0))]
    for side, s in (("left", 1), ("right", -1)):
        b.append(K.box("side-handle-%s" % side, "bowl", "wood-handle", "body", 0.55, (0.13, 0.03, 0.335 - 0.2826), (0.0, K.HANDLE_Y, s * (0.335 + 0.2826) / 2), corner=0.008))
    b.append(K.revolve("ash-pan", "bowl", "alu-cast", "body", 0.65, pan_profile(), Y, (0, 0, 0)))
    b += legs()
    for side, sign in (("left", 1), ("right", -1)):
        b.append(K.revolve("wheel-%s" % side, "bowl", "steel-black", "body", 0.15, K.wheel_profile_wide(0.038), [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)))
    b.append(K.capsule("axle", "bowl", "steel-black", "detail", 0.3, (K.axle_x(), K.AXLE_Y, -0.3), (K.axle_x(), K.AXLE_Y, 0.3), 0.005, slender=True))
    v = {s: K.leg_point(s, K.TRI_Y) for s in ("front", "left", "right")}
    for n, a, c in (("front-left", "front", "left"), ("left-right", "left", "right"), ("right-front", "right", "front")):
        b.append(K.capsule("triangle-%s" % n, "bowl", "steel-black", "detail", 0.25, v[a], v[c], 0.004, slender=True))
    b.append(K.revolve("charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(h=0.042, n=6), Y, (K.BED_A, 0, 0)))
    b.append(K.revolve("fuel-grate-ring", "charcoal", "steel-black", "detail", 0.25, K.torus(K.FUEL_Y - K.BED_Y, K.FUEL_R, 0.003, 8), Y, (K.BED_A, 0, 0), slender=True))
    b.append(K.revolve("grate-ring", "grate", "chrome-steel", "detail", 0.75, K.torus(0.0, 0.2773, 0.004, 8), Y, (K.GRATE_R, 0, 0), slender=True))
    for k in range(-3, 4):
        w = k * 0.068
        half = math.sqrt(0.2773 ** 2 - w * w)
        b.append(K.capsule("grate-bar-%d" % (k + 4), "grate", "chrome-steel", "detail", 0.75, (K.GRATE_R - half, 0.0, w), (K.GRATE_R + half, 0.0, w), 0.003, slender=True))
    b.append(K.revolve("lid-shell", "lid", "enamel-steel", "body", 0.3, K.lid_shell_to(RIM, n=14), Y, (0.2925, 0, 0)))
    u_in = 0.2925 + math.sqrt(K.RHO_L ** 2 - (K.LID_GRIP_V - (K.C_L - K.LID_Y0)) ** 2) - 0.0006
    b.append(K.box("lid-handle", "lid", "wood-handle", "body", 0.55, (0.64 - u_in, 0.03, 0.16), ((0.64 + u_in) / 2, K.LID_GRIP_V, 0.0), corner=0.008))
    b.append(K.revolve("vent-dial", "lid-vent", "alu-cast", "body", 0.7, K.dial_profile(K.FIN_U1), [1, 0, 0], (0, 0, 0)))
    return b


def thermal(b):
    t = [K.revolve("th-bowl-envelope", "bowl", "enamel-steel", "body", 0.3, K.bowl_envelope_to(RIM, 14), Y, (0, 0, 0), region="bowl-shell", sectionFaces=["+v"])]
    t.append(K.revolve("open-bowl-plume-mouth", "bowl", "enamel-steel", "detail", 0.3, [(0.709, 0.0), (0.709, 0.2), (0.74, 0.2), (0.74, 0.0)], Y, (0, 0, 0), region="bowl-shell", tags=["open-bowl-plume"]))
    t.append(K.revolve("th-lid-envelope", "lid", "enamel-steel", "body", 0.3, K.lid_envelope_from(RIM, 14), Y, (0.2925, 0, 0), region="lid-shell", sectionFaces=["-v"]))
    for s in ("leg-front", "leg-left", "leg-right", "wheel-left", "wheel-right", "axle", "triangle-front-left", "triangle-left-right", "triangle-right-front"):
        t.append(ref("th-" + s, s, region="legs-steel"))
    t.append(ref("th-ash-pan", "ash-pan", region="ash-catcher"))
    t.append(ref("th-vent-dial", "vent-dial", region="vents-alu"))
    t.append(K.revolve("th-damper-plate", "ash-damper", "alu-cast", "body", 0.7, K.damper_profile(r_plate=0.065, t=0.0305, n=4), [1, 0, 0], (0, 0, 0), region="vents-alu"))
    t.append(K.revolve("lid-vent-flue-outlet", "lid-vent", "alu-cast", "detail", 0.7, [(K.FIN_U1 - 0.0215, 0.0), (K.FIN_U1 - 0.0215, 0.025), (K.FIN_U1 + 0.0005, 0.025), (K.FIN_U1 + 0.0005, 0.0)], [1, 0, 0], (0, 0, 0), region="vents-alu", tags=["lid-vent-flue"]))
    for s in ("side-handle-left", "side-handle-right", "lid-handle"):
        t.append(ref("th-" + s, s, region="handle-wood"))
    for s in b:
        if s["id"].startswith("grate-"):
            t.append(hidden_copy(s, "cooking-grate", "rods"))
    t.append(K.revolve("th-charcoal-bed", "charcoal", "charcoal", "body", 0.15, K.bed_profile(h=0.042, n=6), Y, (K.BED_A, 0, 0), region="charcoal-bed", layer="embers", sectionFaces=["-v"]))
    return t


def structure(b):
    st = [ref("member-bowl-shell", "bowl-shell", member="bowl", part="member")]
    for s in ("leg-front", "leg-left", "leg-right", "triangle-front-left", "triangle-left-right", "triangle-right-front", "axle"):
        st.append(ref("member-" + s, s, member="bowl", part="member"))
    for side in ("front", "left", "right"):
        st.append(dict(K.ellipsoid("joint-leg-%s" % side, "bowl", "steel-black", "detail", 0.3, (0.02, 0.02, 0.02), K.leg_point(side, K.socket_top_y() - 0.018)), member="bowl", part="joint"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.ellipsoid("joint-wheel-bearing-%s" % side, "bowl", "steel-black", "detail", 0.3, (0.02, 0.02, 0.02), (K.axle_x(), K.AXLE_Y, sign * K.WHEEL_Z)), member="bowl", part="joint"))
    st.append(dict(K.box("support-leg-front", "bowl", "steel-black", "body", 0.3, (0.04, 0.03, 0.04), (K.FOOT_X, 0.015, 0.0)), member="bowl", part="support", anchor="leg-front"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.box("support-wheel-%s" % side, "bowl", "steel-black", "body", 0.3, (0.04, 0.03, 0.04), (K.axle_x(), 0.015, sign * K.WHEEL_Z)), member="bowl", part="support", anchor="wheel-%s" % side))
    st.append(dict(K.ellipsoid("mass-bowl", "bowl", "steel-black", "detail", 0.3, (0.03, 0.015, 0.03), (-0.017, 0.485, 0.0)), member="bowl", part="mass"))
    for s in [x["id"] for x in b if x["id"].startswith("grate-")]:
        st.append(ref("member-" + s, s, member="grate", part="member"))
    st.append(dict(K.ellipsoid("mass-grate", "grate", "steel-black", "detail", 0.3, (0.03, 0.015, 0.03), (K.GRATE_R, 0.0, 0.0)), member="grate", part="mass"))
    st.append(ref("member-fuel-grate-ring", "fuel-grate-ring", member="charcoal", part="member"))
    st.append(dict(K.ellipsoid("mass-charcoal", "charcoal", "steel-black", "detail", 0.3, (0.03, 0.015, 0.03), (K.BED_A, 0.018, 0.0)), member="charcoal", part="mass"))
    st.append(ref("member-lid-shell", "lid-shell", member="lid", part="member"))
    st.append(dict(K.ellipsoid("joint-lid-hinge", "lid", "steel-black", "detail", 0.3, (0.015, 0.015, 0.03), (0.012, 0.004, 0.0)), member="lid", part="joint"))
    st.append(dict(K.ellipsoid("mass-lid", "lid", "steel-black", "detail", 0.3, (0.03, 0.03, 0.03), (0.331, 0.11, 0.0)), member="lid", part="mass"))
    st.append(ref("member-vent-dial", "vent-dial", member="lid-vent", part="member"))
    return st


DECISIONS = [
    text("Body authored anew for the 30 mm feature: bowl and lid as sphere-zone shells meeting at the rim, the side handles and the lid handle as rounded 30 mm wooden blocks that reach the shell (their 25 to 44 mm hand gaps would print as slivers below the 0.15 mm gap of the band and are closed), the vent dial as an 80 x 18 mm cap, the ash pan as a 30 mm mass against the bowl underside (the real 65 mm hanging gap is closed for the same reason), the three 25 mm legs at true size as slender tubes running flush into the bowl, 152 x 38 mm wheels on the true 10 mm axle, the 8 mm wire triangle as a slender part. Inside: the cooking grate as ring and seven bars, the fuel grate as its ring, the charcoal as one mound.",
         "Körper neu für die kleinste Einzelheit von 30 mm: Kessel und Deckel als Kugelzonenschalen, die sich am Rand treffen, Seitengriffe und Deckelgriff als gerundete Holzblöcke von 30 mm bis an die Schale (ihre Griffspalte von 25 bis 44 mm würden als Streifen unter der Lücke von 0,15 mm des Bands gedruckt und sind geschlossen), die Lüftungsscheibe als Kappe von 80 x 18 mm, die Aschewanne als Masse von 30 mm am Kesselboden (der wahre Hängeabstand von 65 mm ist aus demselben Grund geschlossen), die drei Beine von 25 mm in wahrer Stärke als schlanke Rohre bündig in den Kessel, Räder von 152 x 38 mm auf der wahren Achse von 10 mm, das Drahtdreieck von 8 mm als schlankes Teil. Innen: der Grillrost als Ring und sieben Stäbe, der Kohlerost als sein Ring, die Kohle als ein Haufen."),
    text("Thermal: bowl and lid envelopes, every other region from its body parts; the bottom damper only in the thermal list; at this scale only the total area is compared.",
         "Thermik: Hüllen von Kessel und Deckel, jede andere Region aus ihren Körperteilen; der Bodenschieber nur in der Thermikliste; in diesem Maßstab wird nur die Gesamtfläche verglichen."),
    text("Structure: bowl shell, legs, triangle and axle as members, leg joints, wheel bearings and lid hinge as joints, foot and wheel contacts as supports, masses at the centres of mass; the grate rests and pivots are below the feature and merged into the shells.",
         "Tragwerk: Kesselschale, Beine, Dreieck und Achse als Stäbe, Beinanschlüsse, Radlager und Deckeldrehlinie als Gelenke, Fuß- und Radaufstand als Auflager, Massen in den Schwerpunkten; Rostauflagen und Drehpunkte liegen unter der Einzelheit und gehen in den Schalen auf."),
]


def volume():
    b = body()
    return {"status": "authored", "denominator": 200, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
