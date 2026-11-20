"""🔥 Round 3 charcoal kettle grill, 1:500 (smallest feature 75 mm): the kettle as its two shells, the pan mass, three slender legs and two wheels."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-charcoal-grill-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

Y = [0, 1, 0]
RIM = 0.742
PAN_LOW = 0.405
WHEEL_ZC = 0.2955


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
    top = [(K.bowl_y(r) - 0.0004, r) for r in (0.15, 0.1, 0.05, 0.0)]
    return [(PAN_LOW, 0.0), (PAN_LOW, 0.13), (PAN_LOW + 0.02, 0.15)] + top


def bed_profile():
    """🔥 Charcoal and fuel grate as one 75 mm mass lying in the bowl: its underside follows the inner shell, its top is the mound."""
    rho = K.RHO_B - K.T_SH
    low = [(K.bowl_y(r, rho) + 0.0004 - K.BED_Y, r) for r in (0.0, 0.08, 0.15, 0.2)]
    return low + [(0.03, 0.2), (0.055, 0.15), (0.066, 0.08), (0.068, 0.0)]


def legs():
    out = []
    for side in ("front", "left", "right"):
        top = K.leg_point(side, K.socket_top_y())
        if side == "front":
            out.append(K.flush_loft("leg-front", "bowl", "steel-black", "body", 0.25, K.leg_point(side, 0.0), top, 0.0125, n0=(0.0, -1.0, 0.0), n1=K.bowl_normal(top), inset=0.0004, slender=True))
        else:
            out.append(K.flush_loft("leg-%s" % side, "bowl", "steel-black", "body", 0.25, K.leg_point(side, K.AXLE_Y), top, 0.0125, n1=K.bowl_normal(top), slender=True))
    return out


def body():
    b = [K.revolve("bowl-shell", "bowl", "enamel-steel", "body", 0.3, K.shell_profile(K.RHO_B, RIM, n=10), Y, (0, 0, 0))]
    b.append(K.revolve("ash-pan", "bowl", "alu-cast", "body", 0.65, pan_profile(), Y, (0, 0, 0)))
    b += legs()
    for side, sign in (("left", 1), ("right", -1)):
        b.append(K.revolve("wheel-%s" % side, "bowl", "steel-black", "body", 0.15, K.wheel_profile_wide(0.075), [0, 0, sign], (K.axle_x(), K.AXLE_Y, sign * WHEEL_ZC)))
    b.append(K.revolve("charcoal-bed", "charcoal", "charcoal", "body", 0.15, bed_profile(), Y, (K.BED_A, 0, 0)))
    b.append(K.revolve("grate-ring", "grate", "chrome-steel", "detail", 0.75, K.torus(0.0, 0.2773, 0.004, 8), Y, (K.GRATE_R, 0, 0), slender=True))
    b.append(K.revolve("lid-shell", "lid", "enamel-steel", "body", 0.3, K.lid_shell_to(RIM, n=10), Y, (0.2925, 0, 0)))
    u_in = 0.2925 + math.sqrt(K.RHO_L ** 2 - (K.LID_GRIP_V - (K.C_L - K.LID_Y0)) ** 2) - 0.0006
    b.append(K.box("lid-handle", "lid", "wood-handle", "body", 0.55, (0.64 - u_in, 0.075, 0.16), ((0.64 + u_in) / 2, 0.055, 0.0), corner=0.012))
    return b


def thermal(b):
    t = [K.revolve("th-bowl-envelope", "bowl", "enamel-steel", "body", 0.3, K.bowl_envelope_to(RIM, 10), Y, (0, 0, 0), region="bowl-shell", sectionFaces=["+v"])]
    t.append(K.revolve("open-bowl-plume-mouth", "bowl", "enamel-steel", "detail", 0.3, [(0.664, 0.0), (0.664, 0.2), (0.74, 0.2), (0.74, 0.0)], Y, (0, 0, 0), region="bowl-shell", tags=["open-bowl-plume"]))
    t.append(K.revolve("th-lid-envelope", "lid", "enamel-steel", "body", 0.3, K.lid_envelope_from(RIM, 10), Y, (0.2925, 0, 0), region="lid-shell", sectionFaces=["-v"]))
    for s in ("leg-front", "leg-left", "leg-right", "wheel-left", "wheel-right"):
        t.append(ref("th-" + s, s, region="legs-steel"))
    t.append(ref("th-ash-pan", "ash-pan", region="ash-catcher"))
    t.append(K.revolve("th-vents", "lid-vent", "alu-cast", "body", 0.7, [(K.FIN_U1 - 0.0755, 0.0), (K.FIN_U1 - 0.0755, 0.0405), (K.FIN_U1, 0.0405), (K.FIN_U1, 0.0)], [1, 0, 0], (0, 0, 0), region="vents-alu"))
    t.append(K.revolve("lid-vent-flue-outlet", "lid-vent", "alu-cast", "detail", 0.7, [(K.FIN_U1 - 0.0755, 0.0), (K.FIN_U1 - 0.0755, 0.0375), (K.FIN_U1 + 0.0005, 0.0375), (K.FIN_U1 + 0.0005, 0.0)], [1, 0, 0], (0, 0, 0), region="vents-alu", tags=["lid-vent-flue"]))
    for side, s in (("left", 1), ("right", -1)):
        t.append(K.box("th-side-handle-%s" % side, "bowl", "wood-handle", "body", 0.55, (0.13, 0.075, 0.075), (0.0, K.HANDLE_Y, s * 0.2975), corner=0.012, region="handle-wood"))
    t.append(ref("th-lid-handle", "lid-handle", region="handle-wood"))
    for s in b:
        if s["id"] == "grate-ring":
            t.append(hidden_copy(s, "cooking-grate", "rods"))
        if s["id"] == "charcoal-bed":
            t.append(hidden_copy(s, "charcoal-bed", "embers"))
    return t


def structure(b):
    st = [ref("member-bowl-shell", "bowl-shell", member="bowl", part="member")]
    for s in ("leg-front", "leg-left", "leg-right"):
        st.append(ref("member-" + s, s, member="bowl", part="member"))
    st.append(dict(K.capsule("member-axle", "bowl", "steel-black", "detail", 0.3, (K.axle_x(), K.AXLE_Y, -0.29), (K.axle_x(), K.AXLE_Y, 0.29), 0.005, slender=True), member="bowl", part="member"))
    st.append(dict(K.box("support-leg-front", "bowl", "steel-black", "body", 0.3, (0.075, 0.075, 0.075), (K.FOOT_X - 0.01, 0.0375, 0.0)), member="bowl", part="support", anchor="leg-front"))
    for side, sign in (("left", 1), ("right", -1)):
        st.append(dict(K.box("support-wheel-%s" % side, "bowl", "steel-black", "body", 0.3, (0.075, 0.075, 0.075), (K.axle_x(), 0.0375, sign * K.WHEEL_Z)), member="bowl", part="support", anchor="wheel-%s" % side))
    st.append(dict(K.ellipsoid("mass-bowl", "bowl", "steel-black", "detail", 0.3, (0.0375, 0.0375, 0.0375), (-0.017, 0.485, 0.0)), member="bowl", part="mass"))
    st.append(ref("member-grate-ring", "grate-ring", member="grate", part="member"))
    st.append(ref("member-charcoal-bed", "charcoal-bed", member="charcoal", part="member"))
    st.append(dict(K.ellipsoid("mass-charcoal", "charcoal", "steel-black", "detail", 0.3, (0.0375, 0.0375, 0.0375), (K.BED_A, 0.03, 0.0)), member="charcoal", part="mass"))
    st.append(ref("member-lid-shell", "lid-shell", member="lid", part="member"))
    st.append(dict(K.ellipsoid("mass-lid", "lid", "steel-black", "detail", 0.3, (0.0375, 0.0375, 0.0375), (0.331, 0.11, 0.0)), member="lid", part="mass"))
    return st


DECISIONS = [
    text("Body authored anew for the 75 mm feature: bowl and lid as their two sphere-zone shells, the lid handle as one 75 mm block, the ash pan merged with its hanging gap into one 75 mm mass under the bowl, the three legs at their true 25 mm as slender tubes (drawn as pen lines), the two wheels at their true 152 mm diameter and 75 mm wide. Side handles (30 mm), vent dial (18 mm), damper, triangle and axle are below the feature and not drawn (the wheels are widened outward to 75 mm, so the drawing keeps the 0.67 m width of the handles); the charcoal and the fuel grate are one mass in the bowl, the cooking grate its ring.",
         "Körper neu für die kleinste Einzelheit von 75 mm: Kessel und Deckel als ihre zwei Kugelzonenschalen, der Deckelgriff als ein Block von 75 mm, die Aschewanne mit ihrem Hängeabstand zu einer Masse von 75 mm unter dem Kessel zusammengefasst, die drei Beine in ihrer wahren Stärke von 25 mm als schlanke Rohre (als Federlinien gezeichnet), die beiden Räder mit ihrem wahren Durchmesser von 152 mm und 75 mm breit. Seitengriffe (30 mm), Lüftungsscheibe (18 mm), Schieber, Dreieck und Achse liegen unter der Einzelheit und werden nicht gezeichnet (die Räder sind nach außen auf 75 mm verbreitert, so dass die Zeichnung die Breite von 0,67 m der Griffe behält); Kohle und Kohlerost sind eine Masse im Kessel, der Grillrost sein Ring."),
    text("Thermal: every region keeps a solid (side handles, vent dial and damper as thermal-only solids at their true place); only the total area is compared at this scale.",
         "Thermik: jede Region behält einen Körper (Seitengriffe, Lüftungsscheibe und Schieber als reine Thermikkörper an ihrer wahren Stelle); in diesem Maßstab wird nur die Gesamtfläche verglichen."),
    text("Structure: bowl shell and three legs as members with the axle that carries the wheels, foot and wheel contacts as supports, masses of bowl assembly, lid and charcoal; joints are below the feature.",
         "Tragwerk: Kesselschale und drei Beine als Stäbe mit der Achse, die die Räder trägt, Fuß- und Radaufstand als Auflager, Massen von Kesselbaugruppe, Deckel und Kohle; Gelenke liegen unter der Einzelheit."),
]


def volume():
    b = body()
    return {"status": "authored", "denominator": 500, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
