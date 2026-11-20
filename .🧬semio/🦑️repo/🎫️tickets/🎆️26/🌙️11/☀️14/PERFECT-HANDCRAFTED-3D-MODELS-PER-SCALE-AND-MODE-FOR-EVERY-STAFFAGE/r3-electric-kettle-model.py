"""🫖 Round 3 electric jug kettle: every solid of every authored scale and list stated by hand; writes `volumes/appliances/electric-kettle.json` (the decisions come from `r3-electric-kettle-texts.json`)."""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electric-kettle-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

H = K.water_height()


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def st(s, member, part, **extra):
    s.update({"member": member, "part": part})
    s.update(extra)
    return s


def th(s, region, layer):
    s.update({"region": region, "layer": layer})
    return s


def base50():
    return K.revolve("base", "base", "pp-plastic", "body", 0.2, K.BASE["50"])


def shell50():
    inner = [(K.FLOOR_TOP, 0), (K.FLOOR_TOP, K.r_in(K.FLOOR_TOP)), (0.166, K.r_in(0.166)), (0.1712, 0.0688), (0.1736, 0.0668), (0.1741, K.COLLAR_IN), (K.RIM_V, K.COLLAR_IN)]
    outer = [(K.RIM_V, 0.0672), (0.1732, 0.069), (0.1705, 0.0699), (0.167, K.r_out(0.167)), (K.FLOOR_LOW, K.r_out(K.FLOOR_LOW)), (K.FLOOR_LOW, 0)]
    return K.revolve("shell", "body", "stainless", "body", 0.55, inner + outer)


def skirt50():
    lo, top = K.SKIRT_LOW, K.SKIRT_TOP
    loop = [(lo, K.SKIRT_IN), (lo, 0.0737), (lo + 0.0015, K.SKIRT_R), (top - 0.0015, K.SKIRT_R), (top, 0.0746), (top, 0.0739), (K.FLOOR_LOW, 0.0739), (K.FLOOR_LOW, K.SKIRT_IN), (lo, K.SKIRT_IN)]
    return K.revolve("skirt", "body", "pp-plastic", "body", 0.2, loop)


def spout(sid, rows, cap_tip=True):
    return K.planar_loft(sid, "body", "stainless", "body", 0.55, rows, n=2.4, cap=(True, cap_tip))


def gauge50():
    rot = K.quat_from_axes((1.0, 0.0, 0.0), (0.0, 0.0, -1.0), (0.0, 1.0, 0.0))
    secs = []
    for v, rx in ((0.02, 0.0075), (0.0235, 0.0125), (0.0295, 0.014), (0.1305, 0.014), (0.1365, 0.0125), (0.14, 0.0075)):
        secs.append({"at": K.v3((0.0, v, -(K.r_out(v) - 0.001))), "rx": rx, "ry": 0.0022, "exponent": 5, "rotation": rot})
    return K.solid("gauge", "loft", "body", "water", "glazing", 0.8, sections=secs, capStart=True, capEnd=True)


def water(sid, h=H, gap=0.0001):
    lo = K.FLOOR_TOP - K.WATER_Y
    return K.revolve(sid, "water", "water", "body", 0.45, [(lo, 0), (lo, K.r_in(K.FLOOR_TOP) - gap), (lo + h, K.r_in(K.FLOOR_TOP + h) - gap), (lo + h, 0)])


def lid50():
    return K.revolve("lid", "lid", "stainless", "body", 0.6, K.LID["50"])


def knob50():
    return K.revolve("knob", "lid", "pp-plastic", "body", 0.2, K.KNOB)


def handle(sid, path):
    return K.planar_loft(sid, "handle", "pp-plastic", "body", 0.2, path, n=2.3)


def switch50():
    u, v, hu, hv, hw = K.SWITCH
    return K.box("switch", "handle", "pp-plastic", "detail", 0.75, (2 * hu, 2 * hv, 2 * hw), (u, v, 0.0), corner=0.002)


def tip_rows(rows, grow, steps):
    last = rows[-1]
    prev = rows[-2]
    d = (last[0] - prev[0], last[1] - prev[1])
    n = math.hypot(*d)
    d = (d[0] / n, d[1] / n)
    return [(last[0] + d[0] * s, last[1] + d[1] * s, last[2] + grow, last[3] + grow) for s in steps]


def emitters(rows, grow):
    steam = K.planar_loft("th-steam", "body", "water", "body", 0.45, tip_rows(rows, grow, (-0.0015, 0.0012)), cap=(False, True), tags=["steam"])
    sleeve = [(u, v, rx + grow, ry + grow) for u, v, rx, ry in rows[-3:]] + tip_rows(rows, grow + 0.0004, (0.0016,))
    plume = K.planar_loft("th-steam-plume", "body", "stainless", "body", 0.55, sleeve, cap=(False, True), tags=["steam-plume"])
    return [th(steam, "water", "water-skin"), th(plume, "shell", "shell-skin")]


def envelope(sid, outline, low):
    """🌡️ The shell region as the closed outer envelope of the jug (skin, skirt and rim), its open top under the lid and its inward faces standing in for the C31 cavity: only the faces the room air reaches remain (the rim disc under the lid is marked +v)."""
    prof = [(low, 0)] + [(v, r) for v, r in outline] + [(outline[-1][0], 0)]
    return th(K.revolve(sid, "body", "stainless", "body", 0.55, prof, sectionFaces=["+v"]), "shell", "shell-skin")


def lid_skin(sid, profile):
    """🌡️ The lid region: flange and dome with the top 5.5 mm of the skirt, which is what the room air reaches when the lid rattles up to 5 mm; the skirt's cut end looks into the sealed jug (C31 stand-in, marked -v)."""
    prof = [(-0.0055, 0), (-0.0055, K.LID_SKIRT)] + [(a, r) for a, r in profile if a >= 0.0 and r > 0]
    return th(K.revolve(sid, "lid", "stainless", "body", 0.6, prof + [(prof[-1][0], 0)], sectionFaces=["-v"]), "lid", "lid-skin")


def housing(sid, top_profile, deck, wall, bottom_r):
    inner = [(0.0, bottom_r - wall), (deck, bottom_r - wall), (deck, 0)]
    return K.revolve(sid, "base", "pp-plastic", "detail", 0.3, top_profile + [(0.0, bottom_r)] + inner)


def connector(sid, lo, hi, r):
    return K.revolve(sid, "body", "stainless", "detail", 0.3, [(lo, 0), (lo, r), (hi, r), (hi, 0)])


def ring(sid, bone, a0, a1, r0, r1, material="stainless", tone=0.3):
    return K.revolve(sid, bone, material, "detail", tone, [(a0, r0), (a0, r1), (a1, r1), (a1, r0), (a0, r0)])


def disc(sid, bone, a0, a1, r, material="stainless", tone=0.3):
    return K.revolve(sid, bone, material, "detail", tone, [(a0, 0), (a0, r), (a1, r), (a1, 0)])


def capsule(sid, bone, p0, p1, r, material="stainless", tone=0.3):
    return K.solid(sid, "capsule", bone, material, "detail", tone, p0=K.v3(p0), p1=K.v3(p1), r0=r, r1=r)


def wall_cup(sid, top, floor=(K.FLOOR_LOW, K.FLOOR_TOP)):
    """🥤 The jug's load-bearing stainless cup: floor plate and tapered wall up to the seat, real sheet thickness (C30)."""
    lo, hi = floor
    return K.revolve(sid, "body", "stainless", "detail", 0.3, [(lo, 0), (lo, K.r_out(lo)), (top, K.r_out(top)), (top, K.r_in(top)), (hi, K.r_in(hi)), (hi, 0)])


def handle_core(sid, path, f):
    """✊ The handle's load-bearing core along its centre line (the moulded rib section inside the grip shell)."""
    return K.planar_loft(sid, "handle", "pp-plastic", "detail", 0.3, [(u, v, rx * f, ry * f) for u, v, rx, ry in path], n=2.0)


def lid_plate(sid, profile):
    """🫕 The lid as the plate that spans the opening and rests on the seat ring (flange and dome, without skirt and knob)."""
    prof = [(0.0, 0)] + [(a, r) for a, r in profile if a >= 0.0 and r > 0]
    return K.revolve(sid, "lid", "stainless", "detail", 0.3, prof + [(prof[-1][0], 0)])


TOP50 = [(K.RECESS_FLOOR, 0), (K.RECESS_FLOOR, 0.0762), (0.0272, 0.0775), (0.03, 0.0785), (0.03, 0.0815), (0.0285, 0.0858), (0.0255, 0.0889), (0.021, 0.09), (0.004, 0.09), (0.0015, 0.0885)]
HANDLE50, HANDLE100, HANDLE200 = K.HANDLE["50"], K.HANDLE["100"], K.HANDLE["200"]
SPOUT100, SPOUT200 = K.SPOUTS["100"], K.SPOUTS["200"]
WATER_MID = K.FLOOR_TOP - K.WATER_Y + H / 2


def scale50():
    body = [base50(), skirt50(), shell50(), spout("spout", K.SPOUT), gauge50(), water("water"), handle("handle", HANDLE50), switch50(), lid50(), knob50()]
    thermal = [
        ref("th-base", "base", region="plate", layer="plate-skin"),
        th(dict(disc("th-element", "body", K.FLOOR_LOW - 0.0065, K.FLOOR_LOW, 0.062), sectionFaces=["+v", "-v"]), "plate", "plate-skin"),
        envelope("th-shell", [(K.SKIRT_LOW, 0.0737), (K.SKIRT_LOW + 0.0015, K.SKIRT_R)] + K.SHELL_OUTLINE["50"][1:], K.SKIRT_LOW),
        ref("th-spout", "spout", region="shell", layer="shell-skin"),
        ref("th-water", "water", region="water", layer="water-skin"),
        ref("th-gauge", "gauge", region="water", layer="water-skin"),
        lid_skin("th-lid", K.LID["50"]),
        ref("th-knob", "knob", region="lid", layer="lid-skin"),
        ref("th-handle", "handle", region="handle", layer="handle-skin"),
        ref("th-switch", "switch", region="handle", layer="handle-skin"),
    ] + emitters(K.SPOUT, 0.0012)
    structure = [
        st(ring("member-base-wall", "base", 0.0, K.RECESS_FLOOR, 0.081, 0.0885), "base", "member"),
        st(ring("member-base-deck", "base", K.RECESS_FLOOR - 0.0075, K.RECESS_FLOOR, 0.016, 0.0815), "base", "member"),
        st(K.box("member-base-rib", "base", "pp-plastic", "detail", 0.3, (0.064, 0.0075, 0.0075), (0.049, K.RECESS_FLOOR - 0.0112, 0), repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": 1.570796}), "base", "member"),
        st(connector("joint-base", K.RECESS_FLOOR - 0.0075 - 0.03, K.FLOOR_LOW + 0.0005, 0.016), "body", "joint"),
        ref("member-body-skirt", "skirt", member="body", part="member"),
        st(wall_cup("member-body-wall", 0.168), "body", "member"),
        st(ring("joint-lid", "body", 0.168, 0.1755, 0.0612, 0.07), "lid", "joint"),
        st(lid_plate("member-lid", K.LID["50"]), "lid", "member"),
        st(handle_core("member-handle", HANDLE50, 0.55), "handle", "member"),
        st(capsule("joint-handle-top", "handle", (-0.009, 0.068, 0), (0.007, 0.068, 0), 0.0045), "handle", "joint"),
        st(capsule("joint-handle-bottom", "handle", (-0.006, -0.0605, 0), (0.01, -0.0605, 0), 0.0045), "handle", "joint"),
        st(disc("member-water", "water", K.FLOOR_TOP - K.WATER_Y, K.FLOOR_TOP - K.WATER_Y + 0.0075, 0.0725, "water", 0.45), "water", "member"),
        st(K.ellipsoid("mass-base", "base", "stainless", "detail", 0.15, (0.03, 0.0055, 0.03), (0, 0.009, 0)), "base", "mass"),
        st(K.ellipsoid("mass-body", "body", "stainless", "detail", 0.15, (0.014, 0.014, 0.014), (0, 0.09, 0)), "body", "mass"),
        st(K.ellipsoid("mass-water", "water", "stainless", "detail", 0.15, (0.032, 0.016, 0.032), (0, WATER_MID, 0)), "water", "mass"),
        st(ring("support-base-contact", "base", 0.0, 0.0075, 0.0785, 0.0855), "base", "support", anchor="base-contact", tags=["base-contact"]),
    ]
    return body, thermal, structure


def scale100():
    base = K.revolve("base", "base", "pp-plastic", "body", 0.2, K.BASE["100"])
    shell = K.revolve("shell", "body", "stainless", "body", 0.55, [(0.01, 0), (0.01, K.r_in(0.01)), (0.172, K.r_in(0.172)), (0.1735, K.COLLAR_IN), (K.RIM_V, K.COLLAR_IN), (K.RIM_V, 0.0685), (0.17, K.r_out(0.17)), (0.0, K.R_BOT), (0.0, 0)])
    lid = K.revolve("lid", "lid", "stainless", "body", 0.6, K.LID["100"])
    body = [base, shell, spout("spout", SPOUT100), water("water"), handle("handle", HANDLE100), lid]
    thermal = [
        ref("th-base", "base", region="plate", layer="plate-skin"),
        envelope("th-shell", K.SHELL_OUTLINE["100"], 0.0),
        ref("th-spout", "spout", region="shell", layer="shell-skin"),
        ref("th-water", "water", region="water", layer="water-skin"),
        lid_skin("th-lid", K.LID["100"]),
        ref("th-handle", "handle", region="handle", layer="handle-skin"),
    ] + emitters(SPOUT100, 0.0015)
    top = [(0.03, 0), (0.03, 0.085), (0.025, 0.09), (0.004, 0.09)]
    structure = [
        st(housing("member-base", top, 0.015, 0.015, 0.086), "base", "member"),
        st(connector("joint-base", -0.008, 0.0105, 0.02), "body", "joint"),
        st(wall_cup("member-body", 0.161, (0.0, 0.015)), "body", "member"),
        st(handle_core("member-handle", HANDLE100, 0.7), "handle", "member"),
        st(capsule("joint-handle", "handle", (-0.011, 0.068, 0), (0.011, 0.068, 0), 0.0078), "handle", "joint"),
        st(lid_plate("member-lid", K.LID["100"][:-3] + [(0.016, 0.0)]), "lid", "member"),
        st(ring("joint-lid", "body", 0.161, 0.176, 0.06, 0.0705), "lid", "joint"),
        st(disc("member-water", "water", K.FLOOR_TOP - K.WATER_Y, K.FLOOR_TOP - K.WATER_Y + 0.015, 0.0715, "water", 0.45), "water", "member"),
        st(K.ellipsoid("mass-base", "base", "stainless", "detail", 0.15, (0.03, 0.0078, 0.03), (0, 0.012, 0)), "base", "mass"),
        st(K.ellipsoid("mass-water", "water", "stainless", "detail", 0.15, (0.032, 0.016, 0.032), (0, WATER_MID, 0)), "water", "mass"),
        st(ring("support-base-contact", "base", 0.0, 0.015, 0.07, 0.086), "base", "support", anchor="base-contact", tags=["base-contact"]),
    ]
    return body, thermal, structure


def scale200():
    base = K.revolve("base", "base", "pp-plastic", "body", 0.2, K.BASE["200"])
    jug = K.revolve("jug", "body", "stainless", "body", 0.55, [(0, 0), (0, 0.078), (0.17, 0.0722), (0.175, 0.068), (0.175, 0)])
    lid = K.revolve("lid", "lid", "stainless", "body", 0.6, K.LID["200"])
    body = [base, jug, spout("spout", SPOUT200), handle("handle", HANDLE200), lid]
    thermal = [
        ref("th-base", "base", region="plate", layer="plate-skin"),
        ref("th-jug", "jug", region="shell", layer="shell-skin"),
        ref("th-spout", "spout", region="shell", layer="shell-skin"),
        th(water("th-water", gap=0.004), "water", "water-skin"),
        ref("th-lid", "lid", region="lid", layer="lid-skin"),
        ref("th-handle", "handle", region="handle", layer="handle-skin"),
    ] + emitters(SPOUT200, 0.002)
    top = [(0.04, 0), (0.04, 0.091), (0.036, 0.095), (0.004, 0.095)]
    wall = K.revolve("member-body", "body", "stainless", "detail", 0.3, [(0.03, 0), (0.03, 0.0705), (0.175, 0.0655), (0.175, 0.0705), (0.0, 0.0775), (0.0, 0)])
    structure = [
        st(housing("member-base", top, 0.01, 0.03, 0.091), "base", "member"),
        st(wall, "body", "member"),
        st(handle_core("member-handle", HANDLE200, 1.0), "handle", "member"),
        st(lid_plate("member-lid", K.LID["200"]), "lid", "member"),
        st(disc("member-water", "water", 0.021, 0.051, 0.06, "water", 0.45), "water", "member"),
        st(K.ellipsoid("mass-water", "water", "stainless", "detail", 0.15, (0.04, 0.016, 0.04), (0, 0.065, 0)), "water", "mass"),
        st(ring("support-base-contact", "base", 0.0, 0.03, 0.055, 0.091), "base", "support", anchor="base-contact", tags=["base-contact"]),
    ]
    return body, thermal, structure


def main():
    texts = json.load(open(os.path.join(HERE, "r3-electric-kettle-texts.json")))["volumes"]
    out = {}
    for den, fn in (("50", scale50), ("100", scale100), ("200", scale200)):
        body, thermal, structure = fn()
        out[den] = {"status": "authored", "denominator": int(den), "body": body, "thermal": thermal, "structure": structure, "decisions": texts[den]}
    out["500"] = {"status": "unavailable", "denominator": 500}
    path = os.path.join(HERE, "volumes", "appliances", "electric-kettle.json")
    with open(path, "w") as f:
        json.dump({"volumes": out}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("[DEBUG] water fill height %.4f m; wrote %s" % (H, path))


if __name__ == "__main__":
    main()
