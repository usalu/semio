"""🛠️ Round 3 wheelchair ramp: every solid of every scale and list, stated by hand, written to volumes/components/wheelchair-ramp.json.

Usage: python3 r3-wheelchair-ramp-model.py
"""
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-wheelchair-ramp-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
B, H, CO, ST = "body", "handrail", "concrete", "steel"
DEG = math.degrees(K.TH)
SIDES = (("a", 1), ("b", -1))
RUNS = (("run-1", 0), ("run-2", 3))
KERB_LINE = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.7], [10.2, 0.7], [6.6, 0.4], [5.1, 0.4], [1.5, 0.1]]
STRAIGHTS = (("ext-start", 0.9), ("run-1", 0.9), ("landing", 0.9), ("run-2", 0.9), ("ext-end", 0.9))
MASS = K.body_mass()


def step():
    """🔁 Three posts of one run."""
    return {"count": 3, "translation": [K.r5(v) for v in K.POST_STEP]}


def foot(index, lift=K.KERB / K.C):
    """🦶 Handrail-frame foot point of a post on the kerb top."""
    x = K.POSTS[index]
    return x - 1.2, K.h(x) + lift


def kerbs50():
    """🧱 The two wheel guards, cast with the pours: chamfered 100 mm upstands that follow both runs and both raised landings."""
    out = []
    reach = K.RUN_LEN + K.KERB * K.S / K.C
    for side, sign in SIDES:
        mid = sign * K.RAIL_Z
        out += [
            K.extrude("kerb-%s-run-1" % side, B, CO, 0.62, K.kerb_section(0), "yz", 0, reach, transform={"rotation": K.qz(DEG), "translation": [1.5, 0, mid]}),
            K.extrude("kerb-%s-landing-middle" % side, B, CO, 0.62, K.kerb_section(mid), "yz", 5.1, 6.6, transform={"translation": [0, 0.3, 0]}),
            K.extrude("kerb-%s-run-2" % side, B, CO, 0.62, K.kerb_section(0), "yz", 0, reach, transform={"rotation": K.qz(DEG), "translation": [6.6, 0.3, mid]}),
            K.extrude("kerb-%s-landing-top" % side, B, CO, 0.62, K.kerb_section(mid), "yz", 10.2, 11.7, transform={"translation": [0, 0.6, 0]}),
        ]
    return out


def body50():
    """🏗️ 1:50 body: four pours, kerbs, bent tube handrails, posts with caps and stems, base plates and anchor nuts."""
    out = [K.extrude("pour-" + name, B, CO, 0.55, profile, "xy", -K.HALF, K.HALF) for name, profile in K.POURS.items()]
    out += kerbs50()
    for side, sign in SIDES:
        z = sign * K.RAIL_Z
        out.append(K.loft("rail-" + side, H, ST, 0.4, K.bend_start(z) + K.kinks(z) + K.bend_end(z)))
    top = K.POST_TOP
    for side, sign in SIDES:
        z = sign * K.RAIL_Z
        for run, first in RUNS:
            x, y = foot(first)
            tag = "%s-%s" % (side, run)
            out.append(K.revolve("post-" + tag, H, ST, 0.36, [[0, 0], [0, K.POST_R], [top - 0.007, K.POST_R], [top - 0.003, K.POST_R - 0.004], [top, K.POST_R - 0.012], [top, 0]], [0, 1, 0], [x, y, z], repeat=step()))
            out.append(K.revolve("stem-" + tag, H, ST, 0.36, [[top - 0.004, 0], [top - 0.004, K.STEM_R], [K.RAIL_H - K.KERB - 0.004, K.STEM_R], [K.RAIL_H - K.KERB - 0.004, 0]], [0, 1, 0], [x, y, z], repeat=step()))
            plate = K.box("plate-" + tag, H, ST, 0.3, K.PLATE, [x - K.S * 0.005, y + K.C * 0.005, z], repeat=step())
            plate["transform"]["rotation"] = K.qz(DEG)
            out.append(plate)
            for name, along in (("low", -0.04), ("high", 0.04)):
                out.append(K.extrude("nut-%s-%s" % (name, tag), H, ST, 0.25, K.hexagon(K.NUT_R), "zx", 0, K.NUT_H, role="detail", transform={"rotation": K.qz(DEG), "translation": [K.r5(x + along * K.C - K.S * 0.01), K.r5(y + along * K.S + K.C * 0.01), z]}, repeat=step()))
    return out


def rail_ends(z, hollow):
    """📍 End points of the straight rail tubes: between the bends at 1:50, to the plain ends at 1:100."""
    first = K.RAIL_R + K.BEND if hollow else 0.0
    return [[first, 0.85]] + K.RAIL_KINKS + [[9.3 - first, 1.45]]


def rail_tubes(prefix, z, bends, more):
    """🚿 One handrail as hollow tubes 42.4 x 2.6 (mitred at the slope changes) and, at 1:50, its two solid bent ends."""
    ends = rail_ends(z, bends)
    out = []
    for i, (name, lap) in enumerate(STRAIGHTS):
        (x0, y0), (x1, y1) = ends[i], ends[i + 1]
        length = math.hypot(x1 - x0, y1 - y0)
        ux, uy = (x1 - x0) / length, (y1 - y0) / length
        a = 0.0009 if i > 0 else 0.0
        b = 0.0009 if i < 4 else 0.0
        out.append(K.revolve("%s-%s" % (prefix, name), H, ST, 0.45, K.tube(length + a + b, K.RAIL_R, K.RAIL_WALL), [ux, uy, 0], [x0 - a * ux, y0 - a * uy, z], **more))
    if bends:
        out.append(K.loft(prefix + "-bend-start", H, ST, 0.45, K.bend_start(z, K.RAIL_R, 0.004), **more))
        out.append(K.loft(prefix + "-bend-end", H, ST, 0.45, K.bend_end(z, K.RAIL_R, 0.004), **more))
    return out


def post_tubes(prefix, length, more, cap):
    """🗼 The posts as hollow tubes 48.3 x 3.2 and, at 1:50, their welded caps."""
    out = []
    for side, sign in SIDES:
        z = sign * K.RAIL_Z
        for run, first in RUNS:
            x, y = foot(first, K.KERB / K.C if cap else K.KERB)
            tag = "%s-%s" % (side, run)
            out.append(K.revolve("%s-%s" % (prefix, tag), H, ST, 0.45, K.tube(length, K.POST_R, K.POST_WALL), [0, 1, 0], [x, y, z], repeat=step(), **more))
            if cap:
                out.append(K.revolve("%s-cap-%s" % (prefix, tag), H, ST, 0.45, [[length - 0.003, 0], [length - 0.003, K.POST_R], [length - 0.001, K.POST_R], [length + 0.003, K.POST_R - 0.004], [length + 0.006, K.POST_R - 0.012], [length + 0.006, 0]], [0, 1, 0], [x, y, z], repeat=step(), **more))
    return out


def refs(prefix, ids, more):
    """🔗 References to body solids."""
    return [dict({"id": prefix + i, "of": i}, **more) for i in ids]


def fittings(prefix, names, more):
    """🔩 References to the per-run fittings of both sides."""
    return refs(prefix, ["%s-%s-%s" % (n, side, run) for n in names for side, _ in SIDES for run, _ in RUNS], more)


SKINS = {
    "run-1": [[1.5, 0], [5.1, 0.3], [5.1, 0.2], [2.7, 0]],
    "landing-middle": [[5.1, 0.2], [5.1, 0.3], [6.6, 0.3], [6.6, 0.2]],
    "run-2": [[6.6, 0.2], [6.6, 0.3], [10.2, 0.6], [10.2, 0.5]],
    "landing-top": [[10.2, 0.5], [10.2, 0.6], [11.7, 0.6], [11.7, 0.5]],
}
SKIN_LINE = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.5], [10.2, 0.5], [6.6, 0.2], [5.1, 0.2], [2.7, 0]]


def surface_skins(more):
    """🟥 The 100 mm walking-surface layer between the kerbs, one piece per pour."""
    return [K.extrude("th-surface-" + name, B, CO, 0.55, profile, "xy", -K.CLEAR, K.CLEAR, region="ramp-surface", sectionFaces=["-v", "-u", "+u", "-w", "+w"], **more) for name, profile in SKINS.items()]


def flank_skins(profile):
    """🟩 The 100 mm layer of both flanks."""
    return [
        K.extrude("th-side-a", B, CO, 0.5, profile, "xy", K.CLEAR, K.HALF, region="ramp-side", sectionFaces=["-w", "-v", "+v", "-u", "+u"]),
        K.extrude("th-side-b", B, CO, 0.5, profile, "xy", -K.HALF, -K.CLEAR, region="ramp-side", sectionFaces=["+w", "-v", "+v", "-u", "+u"]),
    ]


def thermal50():
    """🌡️ 1:50 thermal: the storage layer of every region as a shell of its declared thickness."""
    out = surface_skins({"layer": "slab-top"})
    flanks = {
        "run-1": [[1.5, 0], [1.5, 0.09], [5.1, 0.39], [5.1, 0]],
        "landing-middle": [[5.1, 0], [5.1, 0.39], [6.6, 0.39], [6.6, 0]],
        "run-2": [[6.6, 0], [6.6, 0.39], [10.2, 0.69], [10.2, 0]],
        "landing-top": [[10.2, 0], [10.2, 0.69], [11.7, 0.69], [11.7, 0]],
    }
    for side, sign in SIDES:
        inner = "-w" if sign > 0 else "+w"
        outer = "+w" if sign > 0 else "-w"
        lo, hi = (K.CLEAR, K.HALF) if sign > 0 else (-K.HALF, -K.CLEAR)
        for name, profile in flanks.items():
            out.append(K.extrude("th-side-%s-%s" % (side, name), B, CO, 0.5, profile, "xy", lo, hi, region="ramp-side", layer="side-concrete", sectionFaces=[inner, "-v", "+v", "-u", "+u"]))
        for name in K.POURS:
            out.append({"id": "th-kerb-%s-%s" % (side, name), "of": "kerb-%s-%s" % (side, name), "region": "kerb", "layer": "kerb-concrete", "sectionFaces": [outer, "-v", "-u", "+u"]})
    out += refs("th-", ["rail-a", "rail-b"], {"region": "handrail-steel", "layer": "tube-wall"})
    out += fittings("th-", ["post"], {"region": "handrail-steel", "layer": "tube-wall"})
    out += fittings("th-", ["stem", "plate", "nut-low", "nut-high"], {"region": "handrail-steel"})
    return out


def slab_line(pad):
    """🛣️ Outline of the 150 mm reinforced slab that follows the walking surface and rests on the bearing layer of this thickness."""
    toe, heel = 1.5 + 12 * pad, 1.5 + 12 * (pad + K.SLAB_T)
    return [[toe, pad], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.45], [10.2, 0.45], [6.6, 0.15], [5.1, 0.15], [heel, pad]]


def wall_line(pad):
    """🧱 Outline of a side wall under the slab edge."""
    return [[1.5 + 12 * (pad + K.SLAB_T), pad], [5.1, 0.15], [6.6, 0.15], [10.2, 0.45], [11.7, 0.45], [11.7, pad]]


def pads(thick):
    """🟩 The four ground bearings, one under each pour."""
    more = {"member": B, "part": "support"}
    return [
        K.extrude("support-ground-0", B, CO, 0.5, [[1.5, 0], [1.5 + 12 * thick, thick], [5.1, thick], [5.1, 0]], "xy", -K.HALF, K.HALF, anchor="ground-0", **more),
        K.box("support-ground-1", B, CO, 0.5, [1.5, thick, 2 * K.HALF], [5.85, thick / 2, 0], anchor="ground-1", **more),
        K.box("support-ground-2", B, CO, 0.5, [3.6, thick, 2 * K.HALF], [8.4, thick / 2, 0], anchor="ground-2", **more),
        K.box("support-ground-3", B, CO, 0.5, [1.5, thick, 2 * K.HALF], [10.95, thick / 2, 0], anchor="ground-3", **more),
    ]


def mass(radius):
    """🟣 The lumped mass of the concrete body at its centre."""
    out = K.base("mass-body", "ellipsoid", B, CO, 0.4)
    out.update({"radii": [radius, radius, radius], "transform": {"translation": [K.r5(MASS[1]), K.r5(MASS[2]), 0]}, "member": B, "part": "mass"})
    return out


def structure50():
    """🦴 1:50 structure: slab on side walls per pour, dowelled joints, kerbs, tube rails on capped posts, plates and anchors, bearings, mass."""
    mem = {"member": B, "part": "member"}
    pad = 0.03
    slabs = {
        "run-1": [[1.5 + 12 * pad, pad], [5.1, 0.3], [5.1, 0.15], [1.5 + 12 * (pad + K.SLAB_T), pad]],
        "landing-middle": [[5.1, 0.15], [5.1, 0.3], [6.6, 0.3], [6.6, 0.15]],
        "run-2": [[6.6, 0.15], [6.6, 0.3], [10.2, 0.6], [10.2, 0.45]],
        "landing-top": [[10.2, 0.45], [10.2, 0.6], [11.7, 0.6], [11.7, 0.45]],
    }
    walls = {
        "run-1": [[1.5 + 12 * (pad + K.SLAB_T), pad], [5.1, 0.15], [5.1, pad]],
        "landing-middle": [[5.1, pad], [5.1, 0.15], [6.6, 0.15], [6.6, pad]],
        "run-2": [[6.6, pad], [6.6, 0.15], [10.2, 0.45], [10.2, pad]],
        "landing-top": [[10.2, pad], [10.2, 0.45], [11.7, 0.45], [11.7, pad]],
    }
    out = [K.extrude("m-slab-" + name, B, CO, 0.5, profile, "xy", -K.HALF, K.HALF, **mem) for name, profile in slabs.items()]
    for side, sign in SIDES:
        lo, hi = (K.HALF - K.WALL_T, K.HALF) if sign > 0 else (-K.HALF, -K.HALF + K.WALL_T)
        out += [K.extrude("m-wall-%s-%s" % (side, name), B, CO, 0.5, profile, "xy", lo, hi, **mem) for name, profile in walls.items()]
    out += refs("m-", ["kerb-%s-%s" % (side, name) for side, _ in SIDES for name in K.POURS], mem)
    for x in K.JOINTS:
        out.append(K.revolve("j-dowels-%d" % round(x * 10), B, ST, 0.3, K.rod(0.4, 0.01), [1, 0, 0], [x - 0.2, K.h(x) - K.SLAB_T / 2, -0.375], repeat={"count": 4, "translation": [0, 0, 0.25]}, member=B, part="joint"))
    hm = {"member": H, "part": "member"}
    for side, sign in SIDES:
        out += rail_tubes("m-rail-" + side, sign * K.RAIL_Z, True, hm)
    out += post_tubes("m-post", K.POST_TOP - 0.006, hm, True)
    out += fittings("m-", ["stem"], hm)
    out += fittings("j-", ["plate", "nut-low", "nut-high"], {"member": H, "part": "joint"})
    return out + pads(pad) + [mass(0.12)]


def coarse_rail(z, slender):
    """〰️ One handrail of a coarser scale: the tube mitred at the four slope changes with plain ends."""
    more = {"slender": True} if slender else {}
    return [[0, 0.85, z, K.RAIL_R, 0]] + K.kinks(z) + [[9.3, 1.45, z, K.RAIL_R, 0]], more


def coarse_posts(radius, lift, length):
    """📍 The posts of a coarser scale as plain round bars."""
    out = []
    for side, sign in SIDES:
        for run, first in RUNS:
            x, y = foot(first, lift)
            out.append(K.revolve("post-%s-%s" % (side, run), H, ST, 0.36, K.rod(length, radius), [0, 1, 0], [x, y, sign * K.RAIL_Z], repeat=step()))
    return out


def coarse_rails(slender):
    """🤝 Both handrails of a coarser scale."""
    out = []
    for side, sign in SIDES:
        sections, more = coarse_rail(sign * K.RAIL_Z, slender)
        out.append(K.loft("rail-" + side, H, ST, 0.4, sections, **more))
    return out


def kerb_strips(more=None):
    """➖ The kerbs of a coarser scale: one square strip per side."""
    return [K.extrude("kerb-" + side, B, CO, 0.62, KERB_LINE, "xy", *((K.CLEAR, K.HALF) if sign > 0 else (-K.HALF, -K.CLEAR)), **(more or {})) for side, sign in SIDES]


def post_ids():
    """🏷️ Ids of the post solids of the coarser scales."""
    return ["post-%s-%s" % (side, run) for side, _ in SIDES for run, _ in RUNS]


def foot_joints(radius, lift, height):
    """🟠 The fixed post feet as flange joints."""
    out = []
    for side, sign in SIDES:
        for run, first in RUNS:
            x, y = foot(first, lift)
            out.append(K.revolve("j-foot-%s-%s" % (side, run), H, ST, 0.3, K.rod(height, radius), [0, 1, 0], [x, y, sign * K.RAIL_Z], repeat=step(), member=H, part="joint"))
    return out


def scale100():
    """📐 1:100: pours with their joints, square kerbs, plain-ended tube rails on round posts."""
    body = [K.extrude("pour-" + name, B, CO, 0.55, profile, "xy", -K.HALF, K.HALF) for name, profile in K.POURS.items()]
    body += kerb_strips() + coarse_rails(False) + coarse_posts(K.POST_R, K.KERB, 0.733)
    thermal = surface_skins({}) + flank_skins([[1.5, 0], [1.5, 0.09], [5.1, 0.39], [6.6, 0.39], [10.2, 0.69], [11.7, 0.69], [11.7, 0]])
    thermal += [{"id": "th-kerb-a", "of": "kerb-a", "region": "kerb", "sectionFaces": ["+w", "-v", "-u", "+u"]}, {"id": "th-kerb-b", "of": "kerb-b", "region": "kerb", "sectionFaces": ["-w", "-v", "-u", "+u"]}]
    thermal += refs("th-", ["rail-a", "rail-b"] + post_ids(), {"region": "handrail-steel"})
    mem = {"member": B, "part": "member"}
    pad = 0.03
    structure = [K.extrude("m-slab", B, CO, 0.5, slab_line(pad), "xy", -K.HALF, K.HALF, **mem)]
    for side, sign in SIDES:
        lo, hi = (K.HALF - K.WALL_T, K.HALF) if sign > 0 else (-K.HALF, -K.HALF + K.WALL_T)
        structure.append(K.extrude("m-wall-" + side, B, CO, 0.5, wall_line(pad), "xy", lo, hi, **mem))
    structure += refs("m-", ["kerb-a", "kerb-b"], mem)
    for x in K.JOINTS:
        structure.append(K.box("j-dowel-row-%d" % round(x * 10), B, ST, 0.3, [0.4, 0.02, 1.0], [x, K.h(x) - K.SLAB_T / 2, 0], member=B, part="joint"))
    hm = {"member": H, "part": "member"}
    for side, sign in SIDES:
        structure += rail_tubes("m-rail-" + side, sign * K.RAIL_Z, False, hm)
    structure += post_tubes("m-post", 0.733, hm, False)
    structure += foot_joints(0.035, K.KERB, 0.015) + pads(pad) + [mass(0.14)]
    return body, thermal, structure


def scale200():
    """📏 1:200: one wedge, two kerb strips, the rails as true tubes on their posts."""
    body = [K.extrude("slab", B, CO, 0.55, K.WEDGE, "xy", -K.HALF, K.HALF)] + kerb_strips() + coarse_rails(True) + coarse_posts(K.POST_R, K.KERB, 0.733)
    thermal = surface_skins({}) + flank_skins(K.WEDGE)
    thermal += [{"id": "th-kerb-a", "of": "kerb-a", "region": "kerb", "sectionFaces": ["-v", "-u", "+u"]}, {"id": "th-kerb-b", "of": "kerb-b", "region": "kerb", "sectionFaces": ["-v", "-u", "+u"]}]
    thermal += refs("th-", ["rail-a", "rail-b"] + post_ids(), {"region": "handrail-steel"})
    mem = {"member": B, "part": "member"}
    pad = 0.04
    structure = [K.extrude("m-slab", B, CO, 0.5, slab_line(pad), "xy", -K.HALF, K.HALF, **mem)]
    for side, sign in SIDES:
        lo, hi = (K.HALF - K.WALL_T, K.HALF) if sign > 0 else (-K.HALF, -K.HALF + K.WALL_T)
        structure.append(K.extrude("m-wall-" + side, B, CO, 0.5, wall_line(pad), "xy", lo, hi, **mem))
    structure += refs("m-", ["kerb-a", "kerb-b"], mem)
    structure += refs("m-", ["rail-a", "rail-b"] + post_ids(), {"member": H, "part": "member"})
    structure += foot_joints(0.04, K.KERB, 0.03) + pads(pad) + [mass(0.16)]
    return body, thermal, structure


def scale500():
    """🗺️ 1:500: the wedge with the kerbs merged into it, the rails on posts at the size floor."""
    body = [K.extrude("slab", B, CO, 0.55, K.WEDGE, "xy", -K.HALF, K.HALF)] + coarse_rails(True) + coarse_posts(0.0375, 0, 0.833)
    thermal = [K.extrude("th-surface", B, CO, 0.55, SKIN_LINE, "xy", -K.CLEAR, K.CLEAR, region="ramp-surface", sectionFaces=["-v", "-u", "+u", "-w", "+w"])] + flank_skins(K.WEDGE)
    thermal += [dict(k, id="th-" + k["id"], region="kerb", sectionFaces=["-v", "-u", "+u"]) for k in kerb_strips({"slender": True})]
    thermal += refs("th-", ["rail-a", "rail-b"] + post_ids(), {"region": "handrail-steel"})
    mem = {"member": B, "part": "member"}
    pad = 0.08
    structure = [K.extrude("m-slab", B, CO, 0.5, slab_line(pad), "xy", -K.HALF, K.HALF, **mem)]
    for side, sign in SIDES:
        lo, hi = (K.HALF - K.WALL_T, K.HALF) if sign > 0 else (-K.HALF, -K.HALF + K.WALL_T)
        structure.append(K.extrude("m-wall-" + side, B, CO, 0.5, wall_line(pad), "xy", lo, hi, **mem))
    structure += refs("m-", ["rail-a", "rail-b"] + post_ids(), {"member": H, "part": "member"})
    structure += foot_joints(0.06, 0, 0.075) + pads(pad) + [mass(0.2)]
    return body, thermal, structure


DECISIONS = {
    "50": [
        K.text(
            "Body: the ramp as built. Four concrete pours (run 1 at 1:12 from 1.5 to 5.1 m, middle landing to 6.6 m, run 2 to 10.2 m, top landing to 11.7 m; 1.4 m wide, 0.30 and 0.60 m high) meet at three movement joints. The 100 x 100 mm wheel guards with 10 mm chamfered arrises are cast on both edges of both runs and both raised landings. Each handrail is one 42.4 mm tube, mitred at the four slope changes, with 0.30 m level extensions that bend down with a 60 mm radius into a capped 40 mm leg. Six round posts 48.3 mm per side stand 1.7 m apart, 100 mm clear of every joint and kerb end, on 120 x 70 x 10 mm base plates that lie on the sloping kerb top with two M10 anchor nuts each; a welded cap closes each post 60 mm below the rail and a 16 mm stem carries the rail, so the hand slides past every post.",
            "Körper: die Rampe wie gebaut. Vier Betonierabschnitte (Lauf 1 mit 1:12 von 1,5 bis 5,1 m, Zwischenpodest bis 6,6 m, Lauf 2 bis 10,2 m, oberes Podest bis 11,7 m; 1,4 m breit, 0,30 und 0,60 m hoch) stoßen an drei Bewegungsfugen aneinander. Die Radabweiser 100 x 100 mm mit 10 mm gefasten Kanten sind an beiden Rändern beider Läufe und beider erhöhten Podeste anbetoniert. Jeder Handlauf ist ein Rohr 42,4 mm, an den vier Neigungswechseln auf Gehrung gestoßen, mit waagerechten Verlängerungen von 0,30 m, die mit 60 mm Radius nach unten in einen verschlossenen Schenkel von 40 mm abbiegen. Je Seite stehen sechs Rundrohrpfosten 48,3 mm im Abstand von 1,7 m, 100 mm frei von jeder Fuge und jedem Bordende, auf Fußplatten 120 x 70 x 10 mm, die auf der geneigten Bordoberseite liegen und je zwei Ankermuttern M10 tragen; ein aufgeschweißter Deckel schließt jeden Pfosten 60 mm unter dem Handlauf und ein Stift von 16 mm trägt das Rohr, sodass die Hand an jedem Pfosten vorbeigleitet.",
        ),
        K.text(
            "Thermal: every region is the shell of its storage layer. The walking surface is a 100 mm slab layer between the kerbs on all four pours (12.27 m2), the side region a 100 mm layer on both flanks from the ground to the outer kerb arris (8.86 m2), the kerb region the kerbs themselves with their top, chamfers and inner face (their 50 mm layer from both faces is the whole kerb), the steel region the outer surface of rails, posts, stems, plates and nuts (the tube wall of 2.6 mm is the layer; the bores are closed by caps and welds and exchange no heat). The retained core under the layers belongs to no region and is not modelled.",
            "Thermik: jede Region ist die Schale ihrer Speicherschicht. Die Gehfläche ist eine Plattenschicht von 100 mm zwischen den Borden auf allen vier Abschnitten (12,27 m2), die Seitenregion eine Schicht von 100 mm an beiden Flanken vom Boden bis zur äußeren Bordkante (8,86 m2), die Bordregion die Borde selbst mit Oberseite, Fasen und Innenseite (ihre Schicht von 50 mm von beiden Seiten ist der ganze Bord), die Stahlregion die Außenfläche von Handläufen, Pfosten, Stiften, Platten und Muttern (die Rohrwand von 2,6 mm ist die Schicht; die Hohlräume sind durch Deckel und Nähte geschlossen und tauschen keine Wärme). Der hinterfüllte Kern unter den Schichten gehört zu keiner Region und ist nicht modelliert.",
        ),
        K.text(
            "Structure: the load path of an earth-retained ramp. A 150 mm reinforced slab (the section of the body bone) follows the surface on each pour and spans between two 200 mm side walls that rise from the bearing layer; the toe of run 1 is slab on grade. Four 20 mm dowels cross each of the three joints at mid slab. The kerbs carry the post feet. Rails and posts are hollow tubes with their real walls, stems are members, base plates and anchor nuts are the fixed joints of the handrail. Each pour bears on its own 30 mm contact layer at its support anchor, and the lumped mass sits at the centre of mass (8.07 m, 0.23 m).",
            "Tragwerk: der Lastpfad einer hinterfüllten Rampe. Eine bewehrte Platte von 150 mm (der Querschnitt des Körperknochens) folgt auf jedem Abschnitt der Oberfläche und spannt zwischen zwei Seitenwänden von 200 mm, die von der Auflagerschicht aufsteigen; der Fuß von Lauf 1 ist Platte auf dem Boden. Je vier Dübel 20 mm queren die drei Fugen in Plattenmitte. Die Borde tragen die Pfostenfüße. Handläufe und Pfosten sind Hohlrohre mit ihren wahren Wänden, die Stifte sind Stäbe, Fußplatten und Ankermuttern die eingespannten Knoten des Handlaufs. Jeder Abschnitt liegt auf seiner eigenen Kontaktschicht von 30 mm an seinem Auflageranker, und die Ersatzmasse sitzt im Schwerpunkt (8,07 m, 0,23 m).",
        ),
    ],
    "100": [
        K.text(
            "Body: the four pours with their joints, each kerb as one square 100 mm strip without chamfers, each handrail as one tube with plain ends (the bends are below what this scale draws), and the posts as plain 48.3 mm round bars from the kerb top into the rail; plates, nuts, caps and stems are merged into the posts.",
            "Körper: die vier Betonierabschnitte mit ihren Fugen, jeder Bord als ein quadratischer Streifen 100 mm ohne Fasen, jeder Handlauf als ein Rohr mit glatten Enden (die Bögen liegen unter dem, was dieser Maßstab zeichnet) und die Pfosten als glatte Rundstäbe 48,3 mm von der Bordoberseite bis in das Rohr; Platten, Muttern, Deckel und Stifte gehen in den Pfosten auf.",
        ),
        K.text(
            "Thermal: the walking surface as a 100 mm skin on every pour, each flank as one 100 mm skin up to the outer kerb arris, the kerbs with top and inner face, rails and posts as the body solids; the layers are merged into their regions.",
            "Thermik: die Gehfläche als Haut von 100 mm auf jedem Abschnitt, jede Flanke als eine Haut von 100 mm bis zur äußeren Bordkante, die Borde mit Oberseite und Innenseite, Handläufe und Pfosten als die Körper selbst; die Schichten sind in ihren Regionen zusammengefasst.",
        ),
        K.text(
            "Structure: one continuous 150 mm slab on two 200 mm side walls, each dowel row as one bar across its joint, the kerbs, rails and posts as hollow tubes with their walls, a flange at every fixed post foot, four bearing layers and the lumped mass.",
            "Tragwerk: eine durchgehende Platte von 150 mm auf zwei Seitenwänden von 200 mm, jede Dübelreihe als ein Stab über ihrer Fuge, die Borde, Handläufe und Pfosten als Hohlrohre mit ihren Wänden, ein Flansch an jedem eingespannten Pfostenfuß, vier Auflagerschichten und die Ersatzmasse.",
        ),
    ],
    "200": [
        K.text(
            "Body: the ramp as one wedge (the joints are no longer forms), two kerb strips, the handrails as true 42.4 mm tubes (`slender`, drawn as lines) on their twelve 48.3 mm posts.",
            "Körper: die Rampe als ein Keil (die Fugen sind keine Formen mehr), zwei Bordstreifen, die Handläufe als wahre Rohre 42,4 mm (`slender`, als Linien gezeichnet) auf ihren zwölf Pfosten 48,3 mm.",
        ),
        K.text(
            "Thermal: a 100 mm surface skin on every pour, a 100 mm skin on each flank of the wedge, the kerb strips with their three free faces, rails and posts as the body solids; only the total area is compared at this scale.",
            "Thermik: eine Oberflächenhaut von 100 mm auf jedem Abschnitt, eine Haut von 100 mm an jeder Flanke des Keils, die Bordstreifen mit ihren drei freien Seiten, Handläufe und Pfosten als die Körper selbst; in diesem Maßstab wird nur die Gesamtfläche verglichen.",
        ),
        K.text(
            "Structure: slab on two side walls, kerbs, rails and posts as solid bars (the bores are below the feature), one joint at every fixed post foot, four 40 mm bearing layers and the lumped mass; the dowels are below the feature.",
            "Tragwerk: Platte auf zwei Seitenwänden, Borde, Handläufe und Pfosten als Vollstäbe (die Hohlräume liegen unter dem Merkmal), ein Knoten an jedem eingespannten Pfostenfuß, vier Auflagerschichten von 40 mm und die Ersatzmasse; die Dübel liegen unter dem Merkmal.",
        ),
    ],
    "500": [
        K.text(
            "Body: the wedge with the kerbs merged into it, the two handrails as true tubes (`slender`) and the twelve posts at the size floor of 75 mm, standing on the slab edge so that no rail floats.",
            "Körper: der Keil mit den darin aufgegangenen Borden, die zwei Handläufe als wahre Rohre (`slender`) und die zwölf Pfosten an der Größenuntergrenze von 75 mm, auf dem Plattenrand stehend, sodass kein Handlauf schwebt.",
        ),
        K.text(
            "Thermal: one 100 mm surface skin over runs and landings, a 100 mm skin on each flank, the kerbs as their own 100 mm strips (kept for the region although the body merges them), rails and posts as the body solids.",
            "Thermik: eine Oberflächenhaut von 100 mm über Läufe und Podeste, eine Haut von 100 mm an jeder Flanke, die Borde als eigene Streifen 100 mm (für die Region behalten, obwohl der Körper sie aufnimmt), Handläufe und Pfosten als die Körper selbst.",
        ),
        K.text(
            "Structure: slab on two side walls above 80 mm bearing layers, rails and posts as bars, one joint at every fixed post foot and the lumped mass.",
            "Tragwerk: Platte auf zwei Seitenwänden über Auflagerschichten von 80 mm, Handläufe und Pfosten als Stäbe, ein Knoten an jedem eingespannten Pfostenfuß und die Ersatzmasse.",
        ),
    ],
}


if __name__ == "__main__":
    volumes = {"50": {"status": "authored", "denominator": 50, "body": body50(), "thermal": thermal50(), "structure": structure50(), "decisions": DECISIONS["50"]}}
    for key, make in (("100", scale100), ("200", scale200), ("500", scale500)):
        body, thermal, structure = make()
        volumes[key] = {"status": "authored", "denominator": int(key), "body": body, "thermal": thermal, "structure": structure, "decisions": DECISIONS[key]}
    path = os.path.join(T, "volumes", "components", "wheelchair-ramp.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for key, v in volumes.items():
        print(key, {name: sum(s.get("repeat", {}).get("count", 1) for s in v[name]) for name in ("body", "thermal", "structure")})
