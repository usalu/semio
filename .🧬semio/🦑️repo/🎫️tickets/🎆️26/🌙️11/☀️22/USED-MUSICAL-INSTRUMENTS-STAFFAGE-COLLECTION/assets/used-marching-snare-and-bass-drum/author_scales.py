"""🔭 The coarse scales of the used marching drums: the parts of 1:100 (feature 15 mm), 1:200 (30 mm) and 1:500 (75 mm), each a new abstraction of the same object on the same bones.

@see author_parts.py — the numbers and the 1:50 parts
"""
import math

from author_geometry import Box, Capsule, Prism, Ring, Tube, circle_pts
from author_parts import BAR_R, BAR_Z, BASS_JS, BASS_LUGS, BASS_XP, HUB, PAD_R, SNARE_JS, SNARE_LUGS, SNARE_XP, XS, YB, Part, arm_point, bass_box, j_path, pad_path, snare_box, snare_lug


def stand_tubes(r_upright, r_cross, sleeves, caps_merged, slender_feet=False):
    """🛠 The tubular cradle at a coarse scale: feet with merged end caps, uprights and cross tubes at the given radii, the arms with or without their rubber sleeves."""
    P = []
    for side, z in (("r", -0.08), ("l", 0.08)):
        P.append(Part(f"foot-{side}", "stand", [Capsule((-0.274, 0.011, z), (0.274, 0.011, z), 0.011)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member", flags={"slender": True} if slender_feet else {}))
        P.append(Part(f"upright-{side}", "stand", [Capsule((0.0, max(0.011, r_upright), z), (0.0, HUB, z), r_upright)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
        for arm, sign in (("front", 1), ("rear", -1)):
            a, b = arm_point(sign, 0.0), arm_point(sign, 0.30)
            if sleeves:
                P.append(Part(f"arm-{arm}-{side}", "stand", [Capsule((a[0], a[1], z), (b[0], b[1], z), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
                s0, s1 = arm_point(sign, 0.215), arm_point(sign, 0.293)
                P.append(Part(f"sleeve-{arm}-{side}", "stand", [Capsule((s0[0], s0[1], z), (s1[0], s1[1], z), 0.0145)], "rubber", "stand-frame", role="detail", tone=0.12, struct="joint"))
        if not sleeves:
            fa, hub, ra = arm_point(1, 0.30), arm_point(1, 0.0), arm_point(-1, 0.30)
            path = [ra, arm_point(-1, 0.15), hub, arm_point(1, 0.15), fa]
            P.append(Part(f"arms-{side}", "stand", [Tube(path, z, 0.0145, loft=True)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member", flags={"slender": True}))
    P.append(Part("foot-cross", "stand", [Capsule((0.0, max(0.011, r_cross), -0.08), (0.0, max(0.011, r_cross), 0.08), r_cross)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
    P.append(Part("hub-cross", "stand", [Capsule((0.0, HUB, -0.08), (0.0, HUB, 0.08), r_cross)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
    return P


def carrier(prefix, bone, xp, y0, y_top, js, plate_x0, plate_y, block_t, arm_box, sling_segments, loft, pad_sheet=False):
    """🎽 A shoulder carrier at a coarse scale: plate and belly pad merged into one block (or the plate as a sheet), the mount arm, the two J-bars and their slings."""
    P = []
    if block_t is None or block_t < 0.02:
        pad = 0.010 if block_t is None else block_t
        P.append(Part(f"{prefix}-plate", bone, [Box((plate_x0 - 0.0015, plate_y, 0.0), (0.003, 0.22, 0.24))], "aluminium", "carrier-frames", tone=0.2, flags={"sheet": True}, struct="member"))
        P.append(Part(f"{prefix}-belly-pad", bone, [Box((plate_x0 - 0.003 - pad / 2, plate_y, 0.0), (pad, 0.20, 0.22))], "foam-pad", "carrier-pads", role="detail", tone=0.1, flags={"sheet": True} if block_t is None else {}))
    else:
        P.append(Part(f"{prefix}-plate", bone, [Box((plate_x0 - block_t / 2, plate_y, 0.0), (block_t, 0.22, 0.24))], "aluminium", "carrier-frames", tone=0.2, struct="member"))
    P.append(Part(f"{prefix}-arm", bone, [arm_box], "aluminium", "carrier-frames", role="detail", tone=0.2, struct="joint"))
    for side, z in (("r", -BAR_Z), ("l", BAR_Z)):
        tube = Tube(j_path(xp, y0, y_top, js, n_arc=8 if loft else 6), z, BAR_R, loft=loft)
        P.append(Part(f"{prefix}-bar-{side}", bone, [tube] if loft else tube.segments(), "aluminium", "carrier-frames", tone=0.2, density_factor=0.331, struct="member", flags={"slender": True} if loft else {}))
        if sling_segments:
            P.append(Part(f"{prefix}-sling-{side}", bone, Tube(pad_path(xp, js, sling_segments), z, PAD_R).segments(), "foam-pad", "carrier-pads", role="detail", tone=0.1, density_factor=0.6))
    return P


def sticks(slender):
    """🥢 The two sticks as one tapered round solid each, butt to tip."""
    P = []
    for side, z in (("r", -0.035), ("l", 0.035)):
        f = (XS - 0.073, 0.3365)
        P.append(Part(f"stick-{side}", f"stick-{side}", [Capsule((f[0] - 0.1225, f[1], z), (f[0] + 0.2715, f[1], z), 0.0075, 0.0045)], "hickory", "sticks", tone=0.68, struct="member", flags={"slender": True} if slender else {}))
    return P


def parts_100():
    """🔎 The parts at 1:100 (feature 15 mm): end caps in the feet, the lug, rod, claw and T-handle of the bass drum as two blocks per tension point, the snare hoop without its flange lip and ears, Ø15 tube lugs, plate and pad as one 15 mm block, mounts at 15 mm."""
    P = stand_tubes(0.010, 0.010, True, True)
    P.append(Part("bass-shell", "bass", [Ring("z", (0.0, YB, 0.0), -0.178, 0.178, 0.297, 0.305)], "birch-lacquer", "bass-shell", tone=0.62, struct="member"))
    for side, s in (("r", -1), ("l", 1)):
        P.append(Part(f"bass-head-{side}", "bass", [Ring("z", (0.0, YB, 0.0), (0.178 if s > 0 else -0.1785), (0.1785 if s > 0 else -0.178), 0.0, 0.3055)], "calfskin", "bass-heads", tone=0.82, flags={"sheet": True}, views=("elevation",)))
        P.append(Part(f"bass-hoop-{side}", "bass", [Ring("z", (0.0, YB, 0.0), (0.158 if s > 0 else -0.200), (0.200 if s > 0 else -0.158), 0.3065, 0.3245)], "beech-lacquer", "bass-hoops", tone=0.5, struct="member"))
        zl = (0.108, 0.155) if s > 0 else (-0.155, -0.108)
        zc = (0.153, 0.216) if s > 0 else (-0.216, -0.153)
        P.append(Part(f"bass-lugs-{side}", "bass", [bass_box(BASS_LUGS[0], 0.3045, 0.339, zl[0], zl[1], 0.020)], "zinc-nickel", "bass-hardware", role="detail", tone=0.7, repeat=("z", (0.0, YB, 0.0), 8, 45.0), density_factor=0.5))
        P.append(Part(f"bass-claws-{side}", "bass", [bass_box(BASS_LUGS[0], 0.324, 0.342, zc[0], zc[1], 0.018)], "steel-nickel", "bass-hardware", role="detail", tone=0.75, repeat=("z", (0.0, YB, 0.0), 8, 45.0)))
    P.append(Part("bass-mount", "bass", [Box((-0.3105, YB, 0.0), (0.015, 0.060, 0.080))], "aluminium", "bass-hardware", role="detail", tone=0.55, struct="joint"))
    P += carrier("bass", "bass-carrier", BASS_XP, 0.545, 0.655, BASS_JS, -0.355, 0.545, 0.015, Box((-0.3355, YB, 0.0), (0.041, 0.015, 0.050)), 6, False)
    P.append(Part("snare-shell", "snare", [Ring("y", (XS, 0.0, 0.0), 0.012, 0.317, 0.172, 0.178)], "birch-ply", "snare-wrap", tone=0.78, struct="member", density_factor=1.069))
    P.append(Part("snare-batter", "snare", [Ring("y", (XS, 0.0, 0.0), 0.317, 0.31725, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, views=("elevation", "plan")))
    P.append(Part("snare-reso", "snare", [Ring("y", (XS, 0.0, 0.0), 0.01175, 0.012, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, views=("elevation", "plan")))
    P.append(Part("snare-hoop-top", "snare", [Ring("y", (XS, 0.0, 0.0), 0.305, 0.329, 0.1795, 0.1905)], "chrome-steel", "snare-hardware", tone=0.85, density_factor=0.38, struct="member"))
    P.append(Part("snare-hoop-bottom", "snare", [Ring("y", (XS, 0.0, 0.0), 0.0, 0.024, 0.1795, 0.1905)], "chrome-steel", "snare-hardware", tone=0.85, density_factor=0.38, struct="member"))
    P.append(Part("snare-lugs", "snare", [snare_lug(SNARE_LUGS[0], 0.185, 0.024, 0.305, 0.0075)], "chrome-brass", "snare-hardware", role="detail", tone=0.88, repeat=("y", (XS, 0.0, 0.0), 10, 36.0), density_factor=0.265))
    P.append(Part("snare-strainer-body", "snare", [snare_box(270.0, 0.1775, 0.204, 0.03, 0.10, 0.036)], "chrome-steel", "snare-hardware", role="detail", tone=0.8))
    P.append(Part("snare-butt", "snare", [snare_box(90.0, 0.1775, 0.198, 0.03, 0.07, 0.030)], "chrome-steel", "snare-hardware", role="detail", tone=0.8))
    P.append(Part("snare-mount", "snare", [snare_box(180.0, 0.1775, 0.1925, 0.17, 0.23, 0.040)], "aluminium", "snare-hardware", role="detail", tone=0.55, struct="joint"))
    P += carrier("snare", "snare-carrier", SNARE_XP, 0.29, 0.40, SNARE_JS, XS - 0.260, 0.29, 0.015, Box((XS - 0.2265, 0.20, 0.0), (0.0675, 0.015, 0.040)), 6, False)
    P += sticks(True)
    return P


def snare_body(r_hoop, hoop_h):
    """🥁 The snare as one hollow turned shell with the steps of its two hoops of `hoop_h` at `r_hoop` around the wrapped shell, inside radius 0.172 m."""
    prof = [(0.0, 0.172), (0.0, r_hoop), (hoop_h, r_hoop), (hoop_h, 0.178), (0.329 - hoop_h, 0.178), (0.329 - hoop_h, r_hoop), (0.329, r_hoop), (0.329, 0.172)]
    return Ring("y", (XS, 0.0, 0.0), 0.0, 0.329, 0.172, r_hoop, profile=prof)


def bass_body(z_hoop, r_hoop, z_end=0.200):
    """🥁 The bass drum as one hollow turned shell with the steps of its two hoop bands from |z| = z_hoop to z_end, inside radius 0.297 m."""
    prof = [(-z_end, 0.297), (-z_end, r_hoop), (-z_hoop, r_hoop), (-z_hoop, 0.305), (z_hoop, 0.305), (z_hoop, r_hoop), (z_end, r_hoop), (z_end, 0.297)]
    return Ring("z", (0.0, YB, 0.0), -z_end, z_end, 0.297, r_hoop, profile=prof)


def heads(scale_drawn=False):
    """🪘 The four heads as declared sheets that close the coarse drum shells (drawn with the drum faces)."""
    P = []
    for side, s in (("r", -1), ("l", 1)):
        P.append(Part(f"bass-head-{side}", "bass", [Ring("z", (0.0, YB, 0.0), (0.178 if s > 0 else -0.1785), (0.1785 if s > 0 else -0.178), 0.0, 0.3055)], "calfskin", "bass-heads", tone=0.82, flags={"sheet": True}, draw=scale_drawn))
    P.append(Part("snare-batter", "snare", [Ring("y", (XS, 0.0, 0.0), 0.317, 0.31725, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, draw=scale_drawn))
    P.append(Part("snare-reso", "snare", [Ring("y", (XS, 0.0, 0.0), 0.01175, 0.012, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, draw=scale_drawn))
    return P


def parts_200():
    """🗺 The parts at 1:200 (feature 30 mm): each drum one turned body with its hoop steps, the bass tension points as eight 30 mm blocks per side, slender true-section bars, feet and arms, 30 mm uprights, cross tubes, mounts and plate blocks."""
    P = stand_tubes(0.015, 0.015, False, True, slender_feet=True)
    P.append(Part("bass-body", "bass", [bass_body(0.158, 0.3245)], "birch-lacquer", "bass-shell", tone=0.62, struct="member", solid=True, flags={"solidbody": True}))
    for side, s in (("r", -1), ("l", 1)):
        z = (0.108, 0.216) if s > 0 else (-0.216, -0.108)
        P.append(Part(f"bass-tension-{side}", "bass", [bass_box(BASS_LUGS[0], 0.3045, 0.342, z[0], z[1], 0.030)], "steel-nickel", "bass-hardware", role="detail", tone=0.72, repeat=("z", (0.0, YB, 0.0), 8, 45.0)))
    P.append(Part("bass-mount", "bass", [Box((-0.3295, YB, 0.0), (0.053, 0.030, 0.050))], "aluminium", "bass-hardware", role="detail", tone=0.55, struct="joint"))
    P += carrier("bass", "bass-carrier", BASS_XP, 0.545, 0.655, BASS_JS, -0.355, 0.545, 0.030, None, 4, True)
    P.append(Part("snare-body", "snare", [snare_body(0.1905, 0.030)], "birch-ply", "snare-wrap", tone=0.78, struct="member", flags={"solidbody": True}))
    P.append(Part("snare-mount", "snare", [Box((XS - 0.2195, 0.20, 0.0), (0.085, 0.030, 0.040))], "aluminium", "snare-hardware", role="detail", tone=0.55, struct="joint"))
    P += carrier("snare", "snare-carrier", SNARE_XP, 0.29, 0.40, SNARE_JS, XS - 0.260, 0.29, 0.030, None, 4, True)
    P += sticks(True)
    P += heads()
    return [p for p in P if p.prims and p.prims[0] is not None]


def saddle_profile():
    """🪑 The envelope of the tubular cradle at 1:500: the outline of feet, uprights, arms and sleeves in the side elevation, its top following the bass shell between the two sleeve contacts."""
    top = circle_pts(0.0, YB, 0.3045, 16, 310.0, 230.0, closed=False)
    a_front = arm_point(1, 0.30)
    a_rear = arm_point(-1, 0.30)
    return [(-0.289, 0.0), (0.289, 0.0), (0.289, 0.022), (a_front[0] + 0.012, a_front[1] + 0.002), (a_front[0] - 0.004, a_front[1] + 0.016)] + [tuple(p) for p in top[1:-1]] + [(a_rear[0] + 0.004, a_rear[1] + 0.016), (a_rear[0] - 0.012, a_rear[1] + 0.002), (-0.289, 0.022)]


def parts_500():
    """🌍 The parts at 1:500 (feature 75 mm): two turned drum bodies with 75 mm hoop bands, the cradle as its envelope saddle, slender bars and sticks, sheet plates and pads, 75 mm mount blocks."""
    P = [Part("stand-saddle", "stand", [Prism(saddle_profile(), -0.09, 0.09)], "steel-coated", "stand-frame", tone=0.25, flags={"envelope": True}, struct="member")]
    P.append(Part("bass-body", "bass", [bass_body(0.141, 0.342, 0.216)], "birch-lacquer", "bass-shell", tone=0.62, struct="member", flags={"solidbody": True}))
    P.append(Part("bass-mount", "bass", [Box((-0.318, YB, 0.0), (0.075, 0.075, 0.075))], "aluminium", "bass-hardware", role="body", tone=0.55, struct="joint"))
    P += carrier("bass", "bass-carrier", BASS_XP, 0.545, 0.655, BASS_JS, -0.355, 0.545, None, None, 0, True)
    P.append(Part("snare-body", "snare", [Ring("y", (XS, 0.0, 0.0), 0.0, 0.329, 0.172, 0.1905)], "birch-ply", "snare-wrap", tone=0.78, struct="member", flags={"solidbody": True}))
    P.append(Part("snare-mount", "snare", [Box((XS - 0.218, 0.20, 0.0), (0.084, 0.075, 0.075))], "aluminium", "snare-hardware", role="body", tone=0.55, struct="joint"))
    P += carrier("snare", "snare-carrier", SNARE_XP, 0.29, 0.40, SNARE_JS, XS - 0.260, 0.29, None, None, 0, True)
    P += sticks(True)
    P += heads()
    return [p for p in P if p.prims and p.prims[0] is not None]
