"""🍞 Round 3 two-slice toaster: every solid of every authored scale and list stated by hand; writes `volumes/appliances/toaster.json` (the decisions come from `r3-toaster-texts.json`)."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-toaster-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

CHROME, DARK, BREAD, PP, RUBBER = "chrome-steel", "mica-steel", "bread-crust", "pp-black", "rubber-foot"
ALL_FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]
ZF = -K.HZ


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def st(s, member, part, **extra):
    s.update({"member": member, "part": part})
    s.update(extra)
    return s


def th(s, region, layer=None, **extra):
    s["region"] = region
    if layer:
        s["layer"] = layer
    s.update(extra)
    return s


def local(profile):
    """🍞 A bread outline from world heights into the slices frame (origin at the seat)."""
    return K.mirror(profile, -K.SEAT)


def feet(radius_low, radius_top, height):
    prof = [(0.0, 0.0), (0.0, radius_low), (0.0015, radius_low + 0.0012), (height, radius_top), (height, 0.0)] if radius_low != radius_top else [(0.0, 0.0), (0.0, radius_low), (height, radius_top), (height, 0.0)]
    out = []
    for sid, x in (("foot-left", -K.FOOT_X), ("foot-right", K.FOOT_X)):
        out.append(K.revolve(sid, "base", RUBBER, "body", 0.15, prof, transform={"translation": K.v3((x, 0.0, -K.FOOT_Z))}, repeat={"count": 2, "translation": [0, 0, 2 * K.FOOT_Z]}))
    return out


def deck(sid, low, top=K.TOP, per_quarter=12, steps=3, **extra):
    return K.extrude(sid, "base", DARK, "body", 0.4, "zx", K.deck_outline(per_quarter), low, top, holes=[K.slot(-1, 0.0, steps), K.slot(1, 0.0, steps)], **extra)


def slot_air(sid, sign, bread_t, low, top):
    """💨 The outlet patch of the hot slot air (D2): the open mouth of one slot around its slice."""
    z = sign * K.SLOT_Z
    hole = [(z - bread_t / 2 - 0.0004, -0.0595), (z + bread_t / 2 + 0.0004, -0.0595), (z + bread_t / 2 + 0.0004, 0.0595), (z - bread_t / 2 - 0.0004, 0.0595)]
    s = K.extrude(sid, "base", DARK, "detail", 0.4, "zx", K.slot(sign, -0.0002), low, top, holes=[hole], tags=["slot-air"], sectionFaces=ALL_FACES)
    return th(s, "top-plate", "plate")


def envelope(sid, rows):
    """🧥 The closed outer envelope of plinth and wrap up to the cap seam: the stand-in for C31 (the product has no cavity flood), its top face lies under the cap."""
    return th(K.stack(sid, "base", CHROME, "body", 0.82, rows), "casing", "sheet", sectionFaces=["+v"])


def scale50():
    body = [
        K.stack("base-tray", "base", PP, "body", 0.2, [(K.FOOT_H, 0.14, 0.066), (0.016, K.TRAY_HX, K.TRAY_HZ), (K.TRAY_TOP, K.TRAY_HX, K.TRAY_HZ)]),
        *feet(K.FOOT_R0, K.FOOT_R1, 0.013),
        K.box("crumb-tray", "base", CHROME, "detail", 0.7, (0.19, 0.012, 0.0075), (0.0, 0.021, -0.0705), corner=0.002),
        K.revolve("cord-bush", "base", PP, "detail", 0.2, [(0.064, 0.0), (0.064, K.BUSH[2]), (0.0745, K.BUSH[2]), (0.08, 0.0045), (0.08, 0.0)], axis=(0, 0, 1), origin=(K.BUSH[0], K.BUSH[1], 0.0)),
        K.stack("casing", "base", CHROME, "body", 0.82, K.shell_rows("50"), caps=(False, False)),
        K.stack("top-rim", "base", DARK, "body", 0.5, K.rim_rows("50"), caps=(False, False)),
        deck("top-plate", K.DECK_LOW),
        K.extrude("chamber", "base", DARK, "body", 0.22, "zx", [(-K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, K.CHAMBER_HX), (-K.CHAMBER_HZ, K.CHAMBER_HX)], K.TRAY_TOP + 0.0075, K.DECK_LOW, holes=[K.well(-1), K.well(1)]),
        K.box("crumb-pan", "base", CHROME, "detail", 0.7, (0.15, 0.0075, 0.104), (0.0, K.TRAY_TOP + 0.00375, 0.0)),
        K.extrude("lever-bezel", "base", PP, "texture", 0.15, "xy", K.rounded_rect(K.LEVER_X - 0.01, 0.091, K.LEVER_X + 0.01, 0.159, 0.01), ZF - 0.0012, ZF + 0.001, texture=True),
        K.along_z("cancel-button", "base", PP, "body", 0.2, [(ZF + 0.004, K.BUTTON[2], K.BUTTON[3], K.BUTTON[0], K.BUTTON[1]), (ZF - 0.0025, K.BUTTON[2], K.BUTTON[3], K.BUTTON[0], K.BUTTON[1]), (ZF - 0.0036, K.BUTTON[2] - 0.0012, K.BUTTON[3] - 0.0012, K.BUTTON[0], K.BUTTON[1])]),
        K.along_z("lever", "lever", PP, "body", 0.2, [(ZF + 0.004, 0.005, 0.004), (ZF - 0.003, 0.005, 0.004), (ZF - 0.0035, 0.015, 0.0115), (ZF - 0.0115, 0.015, 0.012), (ZF - 0.0135, 0.0138, 0.0105), (ZF - 0.014, 0.012, 0.009)]),
        K.revolve("dial", "dial", PP, "body", 0.25, [(ZF + 0.0015, 0.0), (ZF + 0.0015, K.DIAL_R), (ZF - 0.0025, K.DIAL_R), (ZF - 0.0045, 0.0122), (ZF - 0.0112, 0.0115), (ZF - 0.0125, 0.0102), (ZF - 0.0125, 0.0)], axis=(0, 0, 1)),
        K.box("dial-pointer", "dial", CHROME, "texture", 0.9, (0.002, 0.009, 0.0012), (0.0, 0.0062, ZF - 0.0128), texture=True),
    ]
    for sid, sign in (("front", -1), ("rear", 1)):
        z0, z1 = sign * K.SLOT_Z - K.BREAD_T / 2, sign * K.SLOT_Z + K.BREAD_T / 2
        body.append(K.extrude(f"slice-{sid}", "slices", BREAD, "body", 0.55, "xy", local(K.BREAD), z0, z1, holes=[local(K.CRUMB)]))
        body.append(K.extrude(f"crumb-{sid}", "slices", BREAD, "body", 0.82, "xy", local(K.CRUMB), z0, z1))
    thermal = [
        envelope("th-casing", [(K.FOOT_H, 0.14, 0.066), (0.016, K.TRAY_HX, K.TRAY_HZ), *K.shell_rows("50")]),
        th(ref("th-crumb-tray", "crumb-tray"), "casing", "sheet"),
        th(ref("th-cord-bush", "cord-bush"), "casing", "sheet"),
        th(ref("th-top-rim", "top-rim"), "top-plate", "plate"),
        th(ref("th-top-plate", "top-plate"), "top-plate", "plate", sectionFaces=["-v"]),
        *[th(ref(f"th-{part}-{sid}", f"{part}-{sid}"), "bread", "crust") for sid in ("front", "rear") for part in ("slice", "crumb")],
        th(ref("th-lever", "lever"), "control", "knob"),
        th(ref("th-dial", "dial"), "control", "knob"),
        th(ref("th-cancel-button", "cancel-button"), "control", "knob"),
        th(ref("th-foot-left", "foot-left"), "feet", "foot"),
        th(ref("th-foot-right", "foot-right"), "feet", "foot"),
        slot_air("th-slot-air-front", -1, K.BREAD_T, K.TOP - 0.0008, K.TOP + 0.0004),
        slot_air("th-slot-air-rear", 1, K.BREAD_T, K.TOP - 0.0008, K.TOP + 0.0004),
    ]
    inner = [(-K.CHAMBER_HZ + 0.001, -K.CHAMBER_HX + 0.001), (K.CHAMBER_HZ - 0.001, -K.CHAMBER_HX + 0.001), (K.CHAMBER_HZ - 0.001, K.CHAMBER_HX - 0.001), (-K.CHAMBER_HZ + 0.001, K.CHAMBER_HX - 0.001)]
    structure = [
        st(ref("base-frame", "base-tray"), "base", "member"),
        st(K.extrude("chassis", "base", DARK, "body", 0.5, "zx", [(-K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, K.CHAMBER_HX), (-K.CHAMBER_HZ, K.CHAMBER_HX)], K.TRAY_TOP, K.DECK_LOW, holes=[inner]), "base", "member"),
        st(K.capsule("guide-rod", "base", CHROME, "detail", 0.5, (K.ROD_X, 0.0325, 0.0), (K.ROD_X, 0.1775, 0.0), 0.0025, slender=True), "base", "member"),
        st(K.box("lifter-front", "slices", CHROME, "body", 0.5, (0.17, 0.0075, 0.01), (0.019, -0.00375, -K.SLOT_Z)), "slices", "member"),
        st(K.box("lifter-rear", "slices", CHROME, "body", 0.5, (0.17, 0.0075, 0.01), (0.019, -0.00375, K.SLOT_Z)), "slices", "member"),
        st(K.box("yoke", "slices", CHROME, "body", 0.5, (0.008, 0.0315, 0.076), (K.YOKE_X, 0.00825, 0.0)), "slices", "member"),
        st(K.box("joint-slices", "slices", CHROME, "detail", 0.5, (0.012, 0.03, 0.012), (K.ROD_X - 0.004, 0.008, 0.0)), "slices", "joint"),
        st(K.box("lever-arm", "lever", CHROME, "body", 0.5, (0.008, 0.008, 0.046), (0.0, 0.0, ZF + 0.021)), "lever", "member"),
        st(K.ellipsoid("joint-lever", "lever", CHROME, "detail", 0.5, (0.005, 0.005, 0.005), (0.0, 0.0, -0.038)), "lever", "joint"),
        st(K.capsule("dial-spindle", "dial", CHROME, "body", 0.5, (0.0, 0.0, ZF - 0.004), (0.0, 0.0, ZF + 0.026), 0.004), "dial", "member"),
        st(K.box("dial-bracket", "base", DARK, "body", 0.5, (0.034, 0.04, 0.0075), (-K.CHAMBER_HX - 0.016, K.DIAL_Y, ZF + 0.0295)), "base", "member"),
        st(K.ellipsoid("joint-dial", "dial", CHROME, "detail", 0.5, (0.0065, 0.0065, 0.004), (0.0, 0.0, ZF + 0.002)), "dial", "joint"),
        st(ref("support-foot-left", "foot-left"), "base", "support", anchor="foot-left"),
        st(ref("support-foot-right", "foot-right"), "base", "support", anchor="foot-right"),
        st(K.box("mass-base", "base", DARK, "detail", 0.5, (0.05, 0.03, 0.018), (0.0, 0.09, 0.0)), "base", "mass"),
        st(K.ellipsoid("mass-slices", "slices", BREAD, "detail", 0.5, (0.006, 0.006, 0.006), (0.0, 0.055, 0.0)), "slices", "mass"),
    ]
    return body, thermal, structure


def scale100():
    body = [
        K.stack("base-tray", "base", PP, "body", 0.2, [(K.FOOT_H, K.TRAY_HX, K.TRAY_HZ), (K.TRAY_TOP, K.TRAY_HX, K.TRAY_HZ)]),
        *feet(0.0125, 0.0125, 0.013),
        K.stack("casing", "base", CHROME, "body", 0.82, K.shell_rows("100"), caps=(False, False)),
        K.stack("top-rim", "base", DARK, "body", 0.5, K.rim_rows("100"), caps=(False, False)),
        deck("top-plate", 0.173, per_quarter=8, steps=2),
        K.extrude("chamber", "base", DARK, "body", 0.22, "zx", [(-K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, K.CHAMBER_HX), (-K.CHAMBER_HZ, K.CHAMBER_HX)], 0.04, 0.173, holes=[[(-0.047, -0.07), (0.047, -0.07), (0.047, 0.07), (-0.047, 0.07)]]),
        K.box("lever", "lever", PP, "body", 0.2, (0.03, 0.024, 0.018), (0.0, 0.0, ZF - 0.006)),
        K.revolve("dial", "dial", PP, "body", 0.25, [(ZF + 0.003, 0.0), (ZF + 0.003, K.DIAL_R), (ZF - 0.012, K.DIAL_R), (ZF - 0.012, 0.0)], axis=(0, 0, 1)),
    ]
    for sid, sign in (("front", -1), ("rear", 1)):
        body.append(K.extrude(f"slice-{sid}", "slices", BREAD, "body", 0.7, "xy", local(K.BREAD_100), sign * K.SLOT_Z - 0.0075, sign * K.SLOT_Z + 0.0075))
    air = slot_air("th-slot-air", -1, 0.015, K.TOP - 0.0008, K.TOP + 0.0004)
    air["repeat"] = {"count": 2, "translation": [0, 0, 2 * K.SLOT_Z]}
    thermal = [
        envelope("th-casing", [(K.FOOT_H, K.TRAY_HX, K.TRAY_HZ), *K.shell_rows("100")]),
        th(ref("th-top-rim", "top-rim"), "top-plate", "plate"),
        th(ref("th-top-plate", "top-plate"), "top-plate", "plate", sectionFaces=["-v"]),
        th(ref("th-slice-front", "slice-front"), "bread", "crust"),
        th(ref("th-slice-rear", "slice-rear"), "bread", "crust"),
        th(ref("th-lever", "lever"), "control", "knob"),
        th(ref("th-dial", "dial"), "control", "knob"),
        th(ref("th-foot-left", "foot-left"), "feet", "foot"),
        th(ref("th-foot-right", "foot-right"), "feet", "foot"),
        air,
    ]
    wall = 0.002
    inner = [(-K.CHAMBER_HZ + wall, -K.CHAMBER_HX + wall), (K.CHAMBER_HZ - wall, -K.CHAMBER_HX + wall), (K.CHAMBER_HZ - wall, K.CHAMBER_HX - wall), (-K.CHAMBER_HZ + wall, K.CHAMBER_HX - wall)]
    structure = [
        st(ref("base-frame", "base-tray"), "base", "member"),
        st(K.extrude("chassis", "base", DARK, "body", 0.5, "zx", [(-K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, K.CHAMBER_HX), (-K.CHAMBER_HZ, K.CHAMBER_HX)], K.TRAY_TOP, 0.165, holes=[inner]), "base", "member"),
        st(K.capsule("guide-rod", "base", CHROME, "detail", 0.5, (K.ROD_X, 0.034, 0.0), (K.ROD_X, 0.162, 0.0), 0.0032, slender=True), "base", "member"),
        st(K.extrude("carriage", "slices", CHROME, "body", 0.5, "xy", [(-0.066, -0.015), (K.YOKE_X + 0.0075, -0.015), (K.YOKE_X + 0.0075, 0.0275), (K.YOKE_X - 0.0075, 0.0275), (K.YOKE_X - 0.0075, 0.0), (-0.066, 0.0)], -0.038, 0.038), "slices", "member"),
        st(K.box("lever-arm", "lever", CHROME, "body", 0.5, (0.015, 0.015, 0.05), (0.0, 0.0, ZF + 0.025)), "lever", "member"),
        st(K.capsule("dial-spindle", "dial", CHROME, "body", 0.5, (0.0, 0.0, ZF - 0.002), (0.0, 0.0, ZF + 0.03), 0.0075), "dial", "member"),
        st(K.box("dial-bracket", "base", DARK, "body", 0.5, (0.04, 0.04, 0.015), (-K.CHAMBER_HX - 0.018, K.DIAL_Y, ZF + 0.0375)), "base", "member"),
        st(ref("support-foot-left", "foot-left"), "base", "support", anchor="foot-left"),
        st(ref("support-foot-right", "foot-right"), "base", "support", anchor="foot-right"),
        st(K.box("mass-base", "base", DARK, "detail", 0.5, (0.06, 0.03, 0.018), (0.0, 0.09, 0.0)), "base", "mass"),
    ]
    return body, thermal, structure


def scale200():
    body = [
        K.stack("casing", "base", CHROME, "body", 0.82, K.housing_rows("200"), caps=(True, False)),
        K.extrude("top-plate", "base", DARK, "body", 0.4, "zx", K.deck_outline(6), 0.158, K.TOP + 0.0005, holes=[K.slot(-1, 0.0, 2), K.slot(1, 0.0, 2)]),
    ]
    body.append(K.box("lever", "lever", PP, "body", 0.2, (0.03, 0.03, 0.03), (0.0, 0.0, ZF)))
    for sid, sign in (("front", -1), ("rear", 1)):
        body.append(K.extrude(f"slice-{sid}", "slices", BREAD, "body", 0.7, "xy", local(K.BREAD_200), sign * K.SLOT_Z - 0.015, sign * K.SLOT_Z + 0.015))
    thermal = [
        th(ref("th-casing", "casing"), "casing", "sheet"),
        th(ref("th-top-plate", "top-plate"), "top-plate", "plate", sectionFaces=["-v"]),
        th(ref("th-slice-front", "slice-front"), "bread", "crust"),
        th(ref("th-slice-rear", "slice-rear"), "bread", "crust"),
        th(ref("th-lever", "lever"), "control", "knob"),
        th(K.revolve("th-dial", "dial", PP, "detail", 0.25, [(ZF, 0.0), (ZF, K.DIAL_R), (ZF - 0.003, K.DIAL_R), (ZF - 0.003, 0.0)], axis=(0, 0, 1)), "control", "knob", sectionFaces=["+w"]),
        th(K.box("th-feet", "base", RUBBER, "detail", 0.15, (0.03, 0.008, 0.126), (-K.FOOT_X, 0.004, 0.0), repeat={"count": 2, "translation": [2 * K.FOOT_X, 0, 0]}), "feet", "foot", sectionFaces=["+v"]),
        th(K.box("th-slot-air", "base", DARK, "detail", 0.4, (0.136, 0.001, 0.09), (0.0, K.TOP + 0.001, 0.0), tags=["slot-air"], sectionFaces=ALL_FACES), "top-plate", "plate"),
    ]
    wall = 0.006
    outer = [(-K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, -K.CHAMBER_HX), (K.CHAMBER_HZ, K.CHAMBER_HX), (-K.CHAMBER_HZ, K.CHAMBER_HX)]
    inner = [(-K.CHAMBER_HZ + wall, -K.CHAMBER_HX + wall), (K.CHAMBER_HZ - wall, -K.CHAMBER_HX + wall), (K.CHAMBER_HZ - wall, K.CHAMBER_HX - wall), (-K.CHAMBER_HZ + wall, K.CHAMBER_HX - wall)]
    structure = [
        st(K.box("base-frame", "base", PP, "body", 0.3, (0.27, 0.03, 0.13), (0.0, 0.027, 0.0)), "base", "member"),
        st(K.extrude("chassis", "base", DARK, "body", 0.5, "zx", outer, 0.042, 0.158, holes=[inner]), "base", "member"),
        st(K.extrude("carriage", "slices", CHROME, "body", 0.5, "xy", [(-0.066, -0.03), (K.YOKE_X + 0.015, -0.03), (K.YOKE_X + 0.015, 0.035), (K.YOKE_X - 0.015, 0.035), (K.YOKE_X - 0.015, 0.0), (-0.066, 0.0)], -0.038, 0.038), "slices", "member"),
        st(K.box("lever-arm", "lever", CHROME, "body", 0.5, (0.03, 0.03, 0.05), (0.0, 0.0, ZF + 0.025)), "lever", "member"),
        st(K.revolve("dial-spindle", "dial", CHROME, "body", 0.5, [(ZF, 0.0), (ZF, 0.015), (ZF + 0.04, 0.015), (ZF + 0.04, 0.0)], axis=(0, 0, 1)), "dial", "member"),
        st(K.box("support-foot-left", "base", RUBBER, "ground", 0.1, (0.03, 0.03, 0.126), (-K.FOOT_X, 0.015, 0.0)), "base", "support", anchor="foot-left"),
        st(K.box("support-foot-right", "base", RUBBER, "ground", 0.1, (0.03, 0.03, 0.126), (K.FOOT_X, 0.015, 0.0)), "base", "support", anchor="foot-right"),
        st(K.ellipsoid("mass-base", "base", DARK, "detail", 0.5, (0.015, 0.015, 0.015), (0.0, 0.09, 0.0)), "base", "mass"),
    ]
    return body, thermal, structure


def scale500():
    body = [
        K.stack("casing", "base", CHROME, "body", 0.82, K.housing_rows("500")),
        K.extrude("slice", "slices", BREAD, "body", 0.7, "xy", local(K.BREAD_500), -0.045, 0.045),
    ]
    thermal = [
        th(ref("th-casing", "casing"), "casing", "sheet"),
        th(K.extrude("th-top-plate", "base", DARK, "body", 0.4, "zx", K.deck_outline(4), K.TOP, K.TOP + 0.001), "top-plate", "plate", sectionFaces=["-v"]),
        th(ref("th-slice", "slice"), "bread", "crust"),
        th(K.box("th-control", "lever", PP, "detail", 0.2, (0.03, 0.024, 0.003), (0.0, 0.0, ZF - 0.0015)), "control", "knob", sectionFaces=["+w"]),
        th(K.box("th-feet", "base", RUBBER, "detail", 0.15, (0.23, 0.008, 0.03), (0.0, 0.004, -K.FOOT_Z), repeat={"count": 2, "translation": [0, 0, 2 * K.FOOT_Z]}), "feet", "foot", sectionFaces=["+v"]),
        th(K.box("th-slot-air", "base", DARK, "detail", 0.4, (0.104, 0.001, 0.09), (0.0, K.TOP + 0.0015, 0.0), tags=["slot-air"], sectionFaces=ALL_FACES), "top-plate", "plate"),
    ]
    structure = [
        st(K.extrude("chassis", "base", DARK, "body", 0.5, "zx", [(-0.06, -0.13), (0.06, -0.13), (0.06, 0.13), (-0.06, 0.13)], 0.012, 0.16, holes=[[(-0.05, -0.12), (0.05, -0.12), (0.05, 0.12), (-0.05, 0.12)]]), "base", "member"),
        st(K.box("carriage", "slices", CHROME, "body", 0.5, (0.18, 0.02, 0.08), (0.02, -0.01, 0.0)), "slices", "member"),
        st(K.box("lever-arm", "lever", CHROME, "body", 0.5, (0.02, 0.06, 0.04), (0.0, -0.02, ZF + 0.02)), "lever", "member"),
        st(K.revolve("dial-spindle", "dial", CHROME, "body", 0.5, [(ZF, 0.0), (ZF, 0.01), (ZF + 0.03, 0.01), (ZF + 0.03, 0.0)], axis=(0, 0, 1)), "dial", "member"),
        st(K.box("support-foot-left", "base", RUBBER, "ground", 0.1, (0.03, 0.012, 0.126), (-K.FOOT_X, 0.006, 0.0)), "base", "support", anchor="foot-left"),
        st(K.box("support-foot-right", "base", RUBBER, "ground", 0.1, (0.03, 0.012, 0.126), (K.FOOT_X, 0.006, 0.0)), "base", "support", anchor="foot-right"),
        st(K.box("mass-base", "base", DARK, "detail", 0.5, (0.04, 0.04, 0.04), (0.0, 0.09, 0.0)), "base", "mass"),
    ]
    return body, thermal, structure


def main():
    texts = json.load(open(os.path.join(HERE, "r3-toaster-texts.json"), encoding="utf-8"))["volumes"]
    volumes = {}
    for den, make in (("50", scale50), ("100", scale100), ("200", scale200), ("500", scale500)):
        body, thermal, structure = make()
        volumes[den] = {"status": "authored", "denominator": int(den), "body": body, "thermal": thermal, "structure": structure, "decisions": texts[den]}
        print(den, len(body), len(thermal), len(structure))
    with open(os.path.join(HERE, "volumes", "appliances", "toaster.json"), "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")


if __name__ == "__main__":
    main()
