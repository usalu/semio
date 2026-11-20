"""🧯 Round 3, appliances/gas-meter: every solid of every authored scale and list, stated by hand for the compact G4 diaphragm meter of `r3-gas-meter-kit.py`. Writes `volumes/appliances/gas-meter.json`.

Usage: python3 r3-gas-meter-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes/appliances/gas-meter.json")
TEXTS = os.path.join(HERE, "r3-gas-meter-texts.json")
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-gas-meter-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

ST, ABS, PIPE, MU = "sheet-steel", "abs-plastic", "steel-pipe", "polyester-gf"
T_UP, T_LOW, T_SEAM, T_HEAD, T_STUB, T_NUT = 0.76, 0.72, 0.5, 0.9, 0.5, 0.4
MUY = -0.047


def stub_tube(sid, y0, y1, boss, bore=True):
    """🚰 One connection stub about its vertical axis at the inlet: the brazed boss, the threaded tube and, with `bore`, the open bore."""
    inner = K.BORE_R if bore else 0.0
    prof = [(y0, inner), (y0, K.BOSS_R), (0.100, K.BOSS_R), (0.100, K.STUB_R), (y1, K.STUB_R), (y1, inner)] if boss else [(y0, inner), (y0, K.STUB_R), (y1, K.STUB_R), (y1, inner)]
    return K.twice(K.rev(sid, "body", PIPE, "body", T_STUB, K.loop(prof) if bore else prof, (-K.XS, 0, 0), K.YAX))


def tail_tube(sid, bore=True, **kw):
    """🔧 The tailpiece of the union at the inlet: its collar on the sealing face inside the nut and the pipe end above it."""
    inner = K.TAIL_BORE if bore else 0.0
    prof = [(K.Y_STUB_TOP, inner), (K.Y_STUB_TOP, K.STUB_R), (K.NUT_Y1, K.STUB_R), (K.NUT_Y1, K.TAIL_R), (K.Y_TOP, K.TAIL_R), (K.Y_TOP, inner)]
    return K.rev(sid, "body", PIPE, "body", T_STUB, K.loop(prof) if bore else prof, (-K.XS, 0, 0), K.YAX, **kw)


def needle(sid, width, tail, tip, z0, z1):
    """🧭 The pointer of the test dial on its bone: a tapered blade along the bone, from the tail behind the pivot to the tip."""
    return K.ext(sid, "pointer", ABS, "body", 0.1, "xy", [(-tail, -width / 2), (tip, -0.001), (tip, 0.001), (-tail, width / 2)], z0, z1, texture=True)


def disc(sid, bone, material, role, tone, radius, at, z0, z1, **kw):
    """💿 A flat disc facing the front, from z0 to z1."""
    return K.rev(sid, bone, material, role, tone, [(0, 0), (0, radius), (z1 - z0, radius), (z1 - z0, 0)], (at[0], at[1], z0), K.ZAX, **kw)


def body50():
    """🧱 1:50 body: both pressed shells, the folded seam, the index head with counter, dial and pointer, the data plate, the flow arrow, stubs, union nuts and tailpieces."""
    zf = K.IZ_FACE
    window = K.rrect(-K.WIN[0], K.WIN[1], K.WIN[0], K.WIN[2], 0.003)
    bw, bd, bh = K.BEAD
    arrow = [(-0.002, -0.02), (-0.002, 0.008), (-0.006, 0.008), (0.0, 0.02), (0.006, 0.008), (0.002, 0.008), (0.002, -0.02)]
    cw, ch, cy = K.COUNTER
    dw, dh, pitch, x0 = K.DIGIT
    return [
        K.loft("lower-shell", "body", ST, "body", T_LOW, K.shell_sections(False)),
        K.loft("upper-shell", "body", ST, "body", T_UP, K.shell_sections(True)),
        K.loft("seam", "body", ST, "detail", T_SEAM, [((0, -bh, 0), bw - 0.002, bd - 0.002, K.N), ((0, -0.0025, 0), bw, bd, K.N), ((0, 0.0025, 0), bw, bd, K.N), ((0, bh, 0), bw - 0.002, bd - 0.002, K.N)]),
        K.ext("index-base", "body", ABS, "body", T_HEAD, "xy", K.rrect(-K.IX, K.IY0, K.IX, K.IY1, K.IR), zf, K.IZ_BACK),
        K.ext("index-frame", "body", ABS, "body", T_HEAD, "xy", K.rrect(-K.IX, K.IY0, K.IX, K.IY1, K.IR), K.IZ_FRONT, zf, holes=[window]),
        K.box("counter", "body", ABS, "texture", 0.1, (cw, ch, 0.001), (0, cy, zf - 0.0005), texture=True),
        dict(K.box("digits-black", "body", ABS, "texture", 0.92, (dw, dh, 0.0008), (x0, cy, zf - 0.0014), texture=True), repeat={"count": 5, "translation": [pitch, 0, 0]}),
        dict(K.box("digits-red", "body", ABS, "texture", 0.55, (dw, dh, 0.0008), (x0 + 5 * pitch, cy, zf - 0.0014), texture=True), repeat={"count": 3, "translation": [pitch, 0, 0]}),
        disc("dial", "body", ABS, "texture", 0.97, K.DIAL[2], K.DIAL, zf - 0.0008, zf, texture=True),
        dict(K.box("ticks", "body", ABS, "texture", 0.2, (0.004, 0.0012, 0.0005), (0.015, K.DIAL[1], zf - 0.00105), texture=True), repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": 1.5708, "origin": [0, K.DIAL[1], 0]}),
        needle("needle", 0.005, 0.004, 0.0155, zf - 0.0023, zf - 0.0013),
        disc("pivot", "pointer", ABS, "detail", 0.1, 0.004, (0, 0), zf - 0.003, zf - 0.0023, texture=True),
        K.box("plate", "body", ST, "texture", 0.88, (2 * K.PLATE[0], K.PLATE[2] - K.PLATE[1], 0.003), (0, (K.PLATE[1] + K.PLATE[2]) / 2, -0.0603), radius=0.001, texture=True),
        K.box("plate-line-1", "body", ST, "texture", 0.25, (0.09, 0.0008, 0.0005), (0, -0.047, -0.06195), texture=True),
        K.box("plate-line-2", "body", ST, "texture", 0.25, (0.07, 0.0008, 0.0005), (-0.01, -0.062, -0.06195), texture=True),
        K.ext("flow-arrow", "body", ST, "texture", 0.45, "zx", arrow, K.YT, K.YT + 0.0006, texture=True),
        stub_tube("stubs", 0.090, K.Y_STUB_TOP, True),
        K.twice(K.ext("unions", "body", PIPE, "body", T_NUT, "zx", K.hexagon(0, -K.XS, K.NUT_R), K.NUT_Y0, K.NUT_Y1, holes=[[(z, x) for x, z in K.circle(-K.XS, 0, K.STUB_R)]])),
        K.twice(tail_tube("tails")),
    ]


def unit50():
    """🫁 1:50 hidden measuring unit: the frame with its partition, the two diaphragm pans, the valve deck with two slide covers, the gear tower with its crank, the drive shaft to the index and the outlet duct to the outlet stub."""
    pan = lambda s: [((0, MUY, s * z), rx, ry, 3) for z, rx, ry in ((0.015, 0.070, 0.040), (0.025, 0.0695, 0.0395), (0.033, 0.066, 0.0365), (0.039, 0.059, 0.031), (0.043, 0.048, 0.023), (0.045, 0.034, 0.014))]
    return [
        K.th(K.box("th-unit-frame", "body", MU, "body", 0.35, (0.150, 0.086, 0.030), (0, MUY, 0), radius=0.004), "measuring-unit"),
        K.th(K.loft("th-unit-pan-front", "body", MU, "body", 0.3, pan(-1)), "measuring-unit"),
        K.th(K.loft("th-unit-pan-rear", "body", MU, "body", 0.3, pan(1)), "measuring-unit"),
        K.th(K.box("th-unit-valve-deck", "body", MU, "body", 0.4, (0.150, 0.008, 0.090), (0, 0, 0), radius=0.002), "measuring-unit"),
        K.th(dict(K.box("th-unit-valve-covers", "body", MU, "body", 0.45, (0.034, 0.012, 0.050), (-0.040, 0.010, 0), radius=0.003), repeat={"count": 2, "translation": [0.080, 0, 0]}), "measuring-unit"),
        K.th(K.rev("th-unit-gear", "body", MU, "body", 0.5, [(0.004, 0), (0.004, 0.016), (0.034, 0.016), (0.034, 0.022), (0.042, 0.022), (0.042, 0)], (0, 0, 0.010), K.YAX), "measuring-unit"),
        K.th(K.rev("th-unit-drive-shaft", "body", MU, "body", 0.5, [(-0.0575, 0), (-0.0575, 0.004), (0.010, 0.004), (0.010, 0)], (0, K.DIAL[1], 0), K.ZAX), "measuring-unit"),
        K.th(K.rev("th-unit-outlet-duct", "body", MU, "body", 0.4, [(0.004, 0), (0.004, 0.013), (0.090, 0.013), (0.090, 0)], (K.XS, 0, 0), K.YAX), "measuring-unit"),
    ]


def thermal50():
    """🌡️ 1:50 thermal: the housing surfaces, the index head, the closed gas-filled connections with their cut pipe ends, and the hidden measuring unit."""
    stub = K.twice(K.rev("th-stubs", "body", PIPE, "body", T_STUB, [(K.YT, 0), (K.YT, K.BOSS_R), (0.100, K.BOSS_R), (0.100, K.STUB_R), (K.NUT_Y1, K.STUB_R), (K.NUT_Y1, 0)], (-K.XS, 0, 0), K.YAX))
    tail = K.twice(K.rev("th-tails", "body", PIPE, "body", T_STUB, [(K.NUT_Y1, 0), (K.NUT_Y1, K.TAIL_R), (K.Y_TOP, K.TAIL_R), (K.Y_TOP, 0)], (-K.XS, 0, 0), K.YAX))
    return [
        K.ref("th-lower-shell", "lower-shell", region="shell"),
        K.ref("th-upper-shell", "upper-shell", region="shell"),
        K.ref("th-seam", "seam", region="shell"),
        K.ref("th-plate", "plate", region="shell"),
        K.ref("th-index-base", "index-base", region="index"),
        K.ref("th-index-frame", "index-frame", region="index"),
        K.ref("th-needle", "needle", region="index"),
        K.th(stub, "pipes"),
        K.ref("th-unions", "unions", region="pipes"),
        K.th(tail, "pipes", sectionFaces=["+v"]),
    ] + unit50()


def supports(y0=K.Y_STUB_TOP, collar=True):
    """⚓ The two fixed tailpieces of the meter bar that carry the meter: at the inlet the pin, at the outlet the roller."""
    out = []
    for sid, anchor, x in (("support-union-inlet", "union-inlet", -K.XS), ("support-union-outlet", "union-outlet", K.XS)):
        prof = [(y0, K.TAIL_BORE), (y0, K.STUB_R), (K.NUT_Y1, K.STUB_R), (K.NUT_Y1, K.TAIL_R), (K.Y_TOP, K.TAIL_R), (K.Y_TOP, K.TAIL_BORE)] if collar else [(y0, 0), (y0, K.STUB_R), (K.Y_TOP, K.STUB_R), (K.Y_TOP, 0)]
        out.append(K.st(K.rev(sid, "body", PIPE, "body", 0.3, K.loop(prof) if collar else prof, (x, 0, 0), K.YAX), "body", "support", anchor=anchor))
    return out


def cup(sid, upper, tone=0.7):
    """🥣 One shell as the sheet it is: the loft surface from the open seam edge to the closed crown."""
    return K.st(K.loft(sid, "body", ST, "body", tone, K.shell_sections(upper), cap_start=False, cap_end=True), "body", "member")


def structure50():
    """🏗️ 1:50 structure: the weight hangs on the two unions; the stubs carry it into the crown of the upper shell, the folded seam ties both shells, the lower shell carries the measuring unit, whose outlet duct also hangs on the outlet stub."""
    zf = K.IZ_FACE
    return [
        K.ref("member-stubs", "stubs", member="body", part="member"),
        K.ref("joint-unions", "unions", member="body", part="joint"),
        cup("member-upper-shell", True),
        cup("member-lower-shell", False),
        K.ref("member-seam", "seam", member="body", part="member"),
        K.st(K.rev("member-outlet-duct", "body", MU, "body", 0.5, K.loop([(0.004, 0.010), (0.004, 0.013), (0.090, 0.013), (0.090, 0.010)]), (K.XS, 0, 0), K.YAX), "body", "member"),
        K.st(K.box("member-unit-frame", "body", MU, "body", 0.5, (0.150, 0.086, 0.030), (0, MUY, 0), radius=0.004), "body", "member"),
        K.st(K.box("member-valve-deck", "body", MU, "body", 0.5, (0.150, 0.008, 0.090), (0, 0, 0), radius=0.002), "body", "member"),
        K.st(K.rev("member-pointer-spindle", "pointer", ABS, "body", 0.3, [(0, 0), (0, 0.004), (0.0195, 0.004), (0.0195, 0)], (0, 0, zf - 0.003), K.ZAX), "pointer", "member"),
        K.st(disc("joint-pointer", "pointer", ABS, "detail", 0.3, 0.007, (0, 0), zf, zf + 0.008), "pointer", "joint"),
    ] + supports() + [
        K.st(K.ell("mass-body", "body", MU, "detail", 0.5, (0.02, 0.02, 0.02), (0, K.COM_Y, 0)), "body", "mass"),
    ]


def housing_sections():
    """🧴 1:100 housing: one pressing at the width of the seam bead, shoulders in three steps."""
    bw, bd, _ = K.BEAD
    low = [((0, y, 0), bw - i, bd - i, K.N) for y, i in reversed(K.shoulder(K.YSB, -1, 3))]
    top = [((0, y, 0), bw - i, bd - i, K.N) for y, i in K.shoulder(K.YST, 1, 3)]
    return low + [((0, K.YSB, 0), bw, bd, K.N), ((0, K.YST, 0), bw, bd, K.N)] + top


def body100():
    """🧱 1:100 body: one housing, the index head as a block with counter strip, dial and pointer, the data plate, two stubs with round union drums."""
    zi = K.IZ_FRONT
    cw, ch, cy = K.COUNTER
    return [
        K.loft("housing", "body", ST, "body", 0.74, housing_sections()),
        K.box("index", "body", ABS, "body", T_HEAD, (2 * K.IX, K.IY1 - K.IY0, 0.030), (0, (K.IY0 + K.IY1) / 2, zi + 0.015), radius=0.005),
        K.box("counter", "body", ABS, "texture", 0.1, (cw, ch, 0.001), (0, cy, zi - 0.0005), texture=True),
        disc("dial", "body", ABS, "texture", 0.97, K.DIAL[2], K.DIAL, zi - 0.001, zi, texture=True),
        needle("needle", 0.005, 0.004, 0.0155, zi - 0.002, zi - 0.001),
        K.box("plate", "body", ST, "texture", 0.88, (2 * K.PLATE[0], K.PLATE[2] - K.PLATE[1], 0.002), (0, (K.PLATE[1] + K.PLATE[2]) / 2, -K.BEAD[1] - 0.0005), texture=True),
        K.twice(K.rev("stubs", "body", PIPE, "body", T_STUB, [(0.090, 0), (0.090, K.STUB_R), (K.Y_TOP, K.STUB_R), (K.Y_TOP, 0)], (-K.XS, 0, 0), K.YAX)),
        K.twice(K.rev("unions", "body", PIPE, "body", T_NUT, K.loop([(K.NUT_Y0, K.STUB_R), (K.NUT_Y0, 0.020), (K.NUT_Y1, 0.020), (K.NUT_Y1, K.STUB_R)]), (-K.XS, 0, 0), K.YAX)),
    ]


def thermal100():
    """🌡️ 1:100 thermal: housing, index block, closed connections, and the measuring unit as block, gear and outlet duct."""
    stub = K.twice(K.rev("th-stubs", "body", PIPE, "body", T_STUB, [(K.YT, 0), (K.YT, K.STUB_R), (K.Y_TOP, K.STUB_R), (K.Y_TOP, 0)], (-K.XS, 0, 0), K.YAX))
    return [
        K.ref("th-housing", "housing", region="shell"),
        K.ref("th-index", "index", region="index"),
        K.th(stub, "pipes", sectionFaces=["+v"]),
        K.ref("th-unions", "unions", region="pipes"),
        K.th(K.box("th-unit-frame", "body", MU, "body", 0.35, (0.150, 0.086, 0.030), (0, MUY, 0)), "measuring-unit"),
        K.th(dict(K.box("th-unit-pans", "body", MU, "body", 0.3, (0.136, 0.076, 0.028), (0, MUY, -0.029), radius=0.012), repeat={"count": 2, "translation": [0, 0, 0.058]}), "measuring-unit"),
        K.th(K.box("th-unit-valve-deck", "body", MU, "body", 0.4, (0.150, 0.015, 0.090), (0, 0.0035, 0)), "measuring-unit"),
        K.th(K.rev("th-unit-gear", "body", MU, "body", 0.5, [(0.011, 0), (0.011, 0.020), (0.040, 0.020), (0.040, 0)], (0, 0, 0.010), K.YAX), "measuring-unit"),
        K.th(K.rev("th-unit-outlet-duct", "body", MU, "body", 0.4, [(0.011, 0), (0.011, 0.013), (0.090, 0.013), (0.090, 0)], (K.XS, 0, 0), K.YAX), "measuring-unit"),
    ]


def structure100():
    """🏗️ 1:100 structure: stubs under the unions, the housing sheet as two cups with the seam, the measuring unit block on the lower cup, its outlet duct, the pointer spindle, the two supports and the mass."""
    zi = K.IZ_FRONT
    bw, bd, bh = K.BEAD
    return [
        K.st(K.twice(K.rev("member-stubs", "body", PIPE, "body", T_STUB, K.loop([(0.090, K.BORE_R), (0.090, K.STUB_R), (K.NUT_Y1, K.STUB_R), (K.NUT_Y1, K.BORE_R)]), (-K.XS, 0, 0), K.YAX)), "body", "member"),
        K.ref("joint-unions", "unions", member="body", part="joint"),
        cup("member-upper-shell", True),
        cup("member-lower-shell", False),
        K.st(K.loft("member-seam", "body", ST, "body", 0.6, [((0, -0.0075, 0), bw, bd, K.N), ((0, 0.0075, 0), bw, bd, K.N)]), "body", "member"),
        K.st(K.rev("member-outlet-duct", "body", MU, "body", 0.5, K.loop([(0.004, 0.0055), (0.004, 0.013), (0.090, 0.013), (0.090, 0.0055)]), (K.XS, 0, 0), K.YAX), "body", "member"),
        K.st(K.box("member-unit", "body", MU, "body", 0.5, (0.150, 0.094, 0.030), (0, -0.043, 0)), "body", "member"),
        K.st(K.rev("member-pointer-spindle", "pointer", ABS, "body", 0.3, [(0, 0), (0, 0.004), (0.024, 0.004), (0.024, 0)], (0, 0, zi - 0.002), K.ZAX), "pointer", "member"),
    ] + supports(K.NUT_Y1, False) + [
        K.st(K.ell("mass-body", "body", MU, "detail", 0.5, (0.02, 0.02, 0.02), (0, K.COM_Y, 0)), "body", "mass"),
    ]


def body200():
    """🧱 1:200 body: a rounded block, the index block with counter bar, dial and pointer, two solid connection pins."""
    bw, bd, _ = K.BEAD
    zi = K.IZ_FRONT
    return [
        K.box("housing", "body", ST, "body", 0.74, (2 * bw, K.YT - K.YB, 2 * bd), (0, (K.YT + K.YB) / 2, 0), radius=K.R),
        K.box("index", "body", ABS, "body", T_HEAD, (2 * K.IX, K.IY1 - K.IY0, 0.026), (0, (K.IY0 + K.IY1) / 2, zi + 0.013)),
        K.box("counter", "body", ABS, "texture", 0.1, (0.104, 0.022, 0.001), (0, 0.059, zi - 0.0005), texture=True),
        disc("dial", "body", ABS, "texture", 0.97, 0.02, K.DIAL, zi - 0.001, zi, texture=True),
        needle("needle", 0.006, 0.005, 0.017, zi - 0.002, zi - 0.001),
        K.twice(K.rev("stubs", "body", PIPE, "body", 0.45, [(K.YT, 0), (K.YT, 0.020), (K.Y_TOP, 0.020), (K.Y_TOP, 0)], (-K.XS, 0, 0), K.YAX)),
    ]


def thermal200():
    """🌡️ 1:200 thermal: block, index block, pins with cut tops, the measuring unit as one core."""
    return [
        K.ref("th-housing", "housing", region="shell"),
        K.ref("th-index", "index", region="index"),
        K.ref("th-stubs", "stubs", region="pipes", sectionFaces=["+v"]),
        K.th(K.box("th-unit", "body", MU, "body", 0.35, (0.150, 0.120, 0.090), (0, -0.030, 0)), "measuring-unit"),
    ]


def structure200():
    """🏗️ 1:200 structure: two hanger tubes, the housing sheet as two plain cups with chamfered crowns, the unit slab in the lower one, the pointer spindle, two supports and the mass."""
    bw, bd, _ = K.BEAD
    zi = K.IZ_FRONT
    cup200 = lambda sid, y_s, y_c: K.st(K.loft(sid, "body", ST, "body", 0.7, [((0, 0, 0), bw, bd, K.N), ((0, y_s, 0), bw, bd, K.N), ((0, y_c, 0), bw - K.R, bd - K.R, K.N)], cap_start=False, cap_end=True), "body", "member")
    return [
        K.st(K.twice(K.rev("member-stubs", "body", PIPE, "body", T_STUB, [(0.090, 0), (0.090, K.STUB_R), (K.Y_STUB_TOP, K.STUB_R), (K.Y_STUB_TOP, 0)], (-K.XS, 0, 0), K.YAX)), "body", "member"),
        cup200("member-upper-shell", K.YST, K.YT),
        cup200("member-lower-shell", K.YSB, K.YB),
        K.st(K.box("member-unit", "body", MU, "body", 0.5, (0.150, 0.094, 0.030), (0, -0.049, 0)), "body", "member"),
        K.st(K.rev("member-pointer-spindle", "pointer", ABS, "body", 0.3, [(0, 0), (0, 0.004), (0.024, 0.004), (0.024, 0)], (0, 0, zi - 0.002), K.ZAX), "pointer", "member"),
    ] + supports(K.Y_STUB_TOP, False) + [
        K.st(K.ell("mass-body", "body", MU, "detail", 0.5, (0.02, 0.02, 0.02), (0, K.COM_Y, 0)), "body", "mass"),
    ]


def main():
    """💾 Writes the volumes source."""
    texts = json.load(open(TEXTS, encoding="utf-8"))
    vols = {
        "50": {"status": "authored", "denominator": 50, "body": body50(), "thermal": thermal50(), "structure": structure50(), "decisions": texts["50"]},
        "100": {"status": "authored", "denominator": 100, "body": body100(), "thermal": thermal100(), "structure": structure100(), "decisions": texts["100"]},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": texts["200"]},
        "500": {"status": "unavailable", "denominator": 500},
    }
    for s, v in vols.items():
        if v["status"] == "authored":
            for lst in ("body", "thermal", "structure"):
                ids = [x["id"] for x in v[lst]]
                if len(ids) != len(set(ids)):
                    raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items() if v["status"] == "authored"})


if __name__ == "__main__":
    main()
