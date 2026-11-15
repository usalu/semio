import os, sys, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())
B, G, C, LD, V, D = "bowl", "grate", "charcoal", "lid", "lid-vent", "ash-damper"
EN, ST, AL, WD, CR, CC = "enamel-steel", "steel-black", "alu-cast", "wood-handle", "chrome-steel", "charcoal"
R, RIM, BOT = 0.2925, 0.74, 0.5


def bez(p0, p1, p2, p3, n):
    out = []
    for i in range(n + 1):
        u = i / n
        out.append([(1 - u) ** 3 * p0[j] + 3 * (1 - u) ** 2 * u * p1[j] + 3 * (1 - u) * u * u * p2[j] + u ** 3 * p3[j] for j in range(2)])
    return out


def inward(pts, t, ref):
    out = []
    n = len(pts)
    for i, p in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, n - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy) or 1
        nx, ny = -dy / l, dx / l
        if (ref[0] - p[0]) * nx + (ref[1] - p[1]) * ny < 0:
            nx, ny = -nx, -ny
        out.append([max(0.0, p[0] + t * nx), p[1] + t * ny])
    return out


def prof(pts):
    return [[y, r] for r, y in pts]


def shell(id, bone, mat, tone, outer, t, ref, origin, role="body"):
    inner = inward(outer, t, ref)
    p = prof(outer) + prof(inner)[::-1]
    p.append(p[0])
    return revolve(id, bone, mat, tone, "y", origin, p, role=role)


def envelope(id, bone, mat, tone, outer, origin, faces):
    p = prof(outer)
    d = revolve(id, bone, mat, tone, "y", origin, p)
    d["sectionFaces"] = faces
    return d


def ring_rev(id, bone, mat, tone, y0, y1, r0, r1, origin=(0, 0, 0), role="detail", axis="y"):
    return revolve(id, bone, mat, tone, axis, list(origin), [[y0, r0], [y1, r0], [y1, r1], [y0, r1], [y0, r0]], role=role)


def tube(id, bone, mat, tone, p0, p1, ro, ri):
    d = [p1[i] - p0[i] for i in range(3)]
    L = math.sqrt(sum(c * c for c in d))
    return revolve(id, bone, mat, tone, d, p0, [[0, ri], [L, ri], [L, ro], [0, ro], [0, ri]])


def rod(id, bone, mat, tone, p0, p1, r):
    c = capsule(id, bone, mat, tone, p0, p1, r, role="detail")
    c["slender"] = True
    return c


BOWL_O = bez([R, RIM], [R, 0.6], [0.18, BOT], [0, BOT], 28)
LID_O = [[R, -0.005], [R, 0.0]] + bez([R, 0.0], [R, 0.14], [0.17, 0.2], [0, 0.2], 28)[1:]
SPOS = {"bl": (-0.27, 0.14), "br": (-0.27, -0.14), "fl": (0.27, 0.14), "fr": (0.27, -0.14)}
LEG_TOP = {"bl": (-0.17, 0.58, 0.1), "br": (-0.17, 0.58, -0.1), "fl": (0.17, 0.58, 0.1), "fr": (0.17, 0.58, -0.1)}
GRATE_Z = [k * 0.02 for k in range(-13, 14)]
sc = []

# ---- 1:50
s = Scale(50)
s.b(shell("bowl-shell", B, EN, 0.3, BOWL_O, 0.0008, [0, RIM], [0, 0, 0]))
s.b(ring_rev("bowl-rim", B, EN, 0.3, 0.732, 0.74, R - 0.008, R, role="body"))
for sg, nm in ((1, "left"), (-1, "right")):
    s.b(box(f"side-handle-{nm}", B, WD, 0.55, [0.07, 0.03, 0.035], [0, 0.73, sg * 0.3045], 0.004))
s.b(shell("ash-catcher", B, AL, 0.7, [[0.12, 0.5], [0.12, 0.44], [0.1175, 0.4275], [0.1, 0.4205], [0.08, 0.42], [0, 0.42]], 0.002, [0, 0.47], [0, 0, 0]))
for k, (x, z) in SPOS.items():
    t = LEG_TOP[k]
    d = capsule(f"leg-{k}", B, ST, 0.15, [x, 0.012, z], list(t), 0.012)
    d["slender"] = True
    s.b(d)
for k in ("bl", "br"):
    x, z = SPOS[k]
    s.b(ring_rev(f"foot-{k}", B, ST, 0.1, 0.0, 0.008, 0.0, 0.015, origin=(x, 0, z), role="body"))
for k in ("fl", "fr"):
    x, z = SPOS[k]
    zc = 0.1425 if z > 0 else -0.1425
    s.b(revolve(f"wheel-{k}", B, ST, 0.12, "z", [x, 0.05, zc - 0.0125], [[0, 0.016], [0.025, 0.016], [0.025, 0.046], [0.022, 0.05], [0.003, 0.05], [0, 0.046], [0, 0.016]]))
    s.b(revolve(f"wheel-hub-{k}", B, ST, 0.2, "z", [x, 0.05, zc - 0.0125], [[0, 0], [0, 0.016], [0.025, 0.016], [0.025, 0], [0, 0]], role="detail"))
ringp = [(-0.2334, -0.1254), (-0.2334, 0.1254), (0.2334, 0.1254), (0.2334, -0.1254)]
for i in range(4):
    a, b = ringp[i], ringp[(i + 1) % 4]
    s.b(rod(f"leg-ring-{i + 1}", B, ST, 0.15, [a[0], 0.22, a[1]], [b[0], 0.22, b[1]], 0.005))
s.b(revolve("charcoal-bed", C, CC, 0.1, "y", [0, 0, 0], prof_bed := [[0.523 - 0.58, 0], [0.523 - 0.58, 0.14], [0.54 - 0.58, 0.19], [0.58 - 0.58, 0.225], [0.62 - 0.58, 0.239], [0.638 - 0.58, 0.2], [0.644 - 0.58, 0.1], [0.646 - 0.58, 0]]))
s.b(ring_rev("grate-rim", G, CR, 0.5, -0.004, 0.004, 0.266, 0.274, role="detail"))
for i, z in enumerate(GRATE_Z):
    h = math.sqrt(max(0.268 ** 2 - z * z, 0))
    s.b(rod(f"grate-rod-{i + 1}", G, CR, 0.5, [-h, 0, z], [h, 0, z], 0.003))
for i, x in enumerate((-0.1, 0.1)):
    s.b(rod(f"grate-brace-{i + 1}", G, CR, 0.5, [x, -0.0075, -0.2], [x, -0.0075, 0.2], 0.0035))
s.b(shell("lid-shell", LD, EN, 0.3, [[r + 0, y] for r, y in LID_O], 0.0008, [0, 0.0], [R, 0, 0]))
s.b(ring_rev("lid-rim-bead", LD, EN, 0.3, 0.008, 0.016, R - 0.008, R + 0.001, origin=(R, 0, 0), role="body"))
s.b(box("lid-handle", LD, WD, 0.55, [0.085, 0.06, 0.06], [0.635, 0.11, 0], 0.012))
for sg, nm in ((1, "a"), (-1, "b")):
    s.b(ext(f"lid-handle-post-{nm}", LD, ST, 0.15, "xy", [[0.5475, 0.095], [0.5925, 0.1], [0.5925, 0.125], [0.5475, 0.13]], sg * 0.015 - 0.0075, sg * 0.015 + 0.0075))
dial = ext("vent-dial", V, AL, 0.7, "yz", circle_pts(0, 0, 0.04, 32), -0.005, 0.03, holes=[circle_pts(y, 0, 0.006, 12) for y in (0.022, 0, -0.022)])
s.b(dial)
s.b(revolve("ash-damper", D, AL, 0.7, "y", [0, -0.025, 0], [[0, 0], [0, 0.05], [0.025, 0.05], [0.025, 0], [0, 0]], role="body"))


def tof(s, id, of, region, layer=None, faces=None, tags=None):
    d = {"id": id, "of": of, "region": region}
    if layer:
        d["layer"] = layer
    if faces:
        d["sectionFaces"] = faces
    if tags:
        d["tags"] = tags
    s.thermal.append(d)


def clone_th(s, src_id, new_id, region, layer):
    c = dict([b for b in s.body if b["id"] == src_id][0])
    c["id"] = new_id
    c["region"] = region
    c["layer"] = layer
    c["role"] = "detail" if c.get("slender") else c.get("role", "body")
    s.thermal.append(c)


def thermal50(s):
    d = envelope("th-bowl-envelope", B, EN, 0.3, BOWL_O, [0, 0, 0], ["+v"])
    s.th(d, "bowl-shell")
    d = ring_rev("open-bowl-plume-mouth", B, EN, 0.9, 0.736, 0.744, 0, 0.2, role="detail")
    d["tags"] = ["open-bowl-plume"]
    d["sectionFaces"] = ["+v", "-v"]
    s.th(d, "bowl-shell")
    s.th(envelope("th-lid-envelope", LD, EN, 0.3, LID_O, [R, 0, 0], ["-v"]), "lid-shell")
    pan = [[0.12, 0.5], [0.12, 0.44], [0.1175, 0.4275], [0.1, 0.4205], [0.08, 0.42], [0, 0.42]]
    s.th(envelope("th-ash-envelope", B, AL, 0.7, pan, [0, 0, 0], []), "ash-catcher")
    for i in ["leg-bl", "leg-br", "leg-fl", "leg-fr", "leg-ring-1", "leg-ring-2", "leg-ring-3", "leg-ring-4", "foot-bl", "foot-br", "wheel-fl", "wheel-fr", "wheel-hub-fl", "wheel-hub-fr"]:
        tof(s, f"th-{i}", i, "legs-steel")
    tof(s, "th-vent-dial", "vent-dial", "vents-alu", None, ["-u"])
    tof(s, "th-ash-damper", "ash-damper", "vents-alu")
    d = revolve("lid-vent-flue-nozzle", V, AL, 0.9, "x", [0.0225, 0, 0], [[0, 0], [0, 0.025], [0.008, 0.025], [0.008, 0], [0, 0]], role="detail")
    d["tags"] = ["lid-vent-flue"]
    d["sectionFaces"] = ["+u", "-u"]
    s.th(d, "vents-alu")
    for i in ["lid-handle", "side-handle-left", "side-handle-right"]:
        tof(s, f"th-{i}", i, "handle-wood")
    for i in ["lid-handle-post-a", "lid-handle-post-b"]:
        tof(s, f"th-{i}", i, "lid-shell")
    for i in range(1, len(GRATE_Z) + 1):
        clone_th(s, f"grate-rod-{i}", f"th-grate-rod-{i}", "cooking-grate", "rods")
    for i in (1, 2):
        clone_th(s, f"grate-brace-{i}", f"th-grate-brace-{i}", "cooking-grate", "rods")
    clone_th(s, "grate-rim", "th-grate-rim", "cooking-grate", "rods")
    d = revolve("th-charcoal-bed", C, CC, 0.1, "y", [0, 0, 0], [[-0.012, 0], [-0.012, 0.17], [0.0, 0.18], [0.018, 0.17], [0.018, 0], [-0.012, 0]])
    d["sectionFaces"] = ["-v"]
    s.th(d, "charcoal-bed", "embers")


def structure50(s):
    for k, (x, z) in SPOS.items():
        t = LEG_TOP[k]
        p0 = [x, 0.0, z]
        p1 = [t[0], t[1], t[2]]
        s.st(tube(f"leg-tube-{k}", B, ST, 0.15, p0, p1, 0.0105, 0.009), B, "member")
    s.stof("bowl-shell-member", "bowl-shell", B, "member")
    for i in range(1, 5):
        s.stof(f"leg-ring-{i}-member", f"leg-ring-{i}", B, "member")
    for i in range(1, len(GRATE_Z) + 1):
        s.stof(f"grate-rod-{i}-member", f"grate-rod-{i}", G, "member")
    for i in (1, 2):
        s.stof(f"grate-brace-{i}-member", f"grate-brace-{i}", G, "member")
    s.stof("grate-rim-member", "grate-rim", G, "member")
    s.stof("charcoal-bed-member", "charcoal-bed", C, "member")
    s.stof("lid-shell-member", "lid-shell", LD, "member")
    for n in ("a", "b"):
        s.stof(f"lid-handle-post-{n}-member", f"lid-handle-post-{n}", LD, "member")
    s.stof("vent-dial-member", "vent-dial", V, "member")
    s.stof("ash-damper-member", "ash-damper", D, "member")
    s.st(revolve("lid-hinge-pin", LD, ST, 0.15, "z", [0.004, 0.002, -0.025], [[0, 0], [0, 0.008], [0.05, 0.008], [0.05, 0], [0, 0]], role="detail"), LD, "joint")
    s.st(revolve("vent-pin", V, ST, 0.15, "z", [0, 0, -0.015], [[0, 0], [0, 0.008], [0.03, 0.008], [0.03, 0], [0, 0]], role="detail"), V, "joint")
    s.st(revolve("damper-pin", D, ST, 0.15, "z", [0, -0.01, -0.02], [[0, 0], [0, 0.008], [0.04, 0.008], [0.04, 0], [0, 0]], role="detail"), D, "joint")
    for k in ("fl", "fr"):
        s.stof(f"wheel-axle-{k}", f"wheel-hub-{k}", B, "joint")
    for nm, (x, z) in (("leg-rear-left", (-0.275, 0.145)), ("leg-rear-right", (-0.275, -0.145))):
        s.st(box(f"support-{nm}-pad", B, ST, 0.1, [0.04, 0.0075, 0.04], [x, 0.00375, z]), B, "support")
    for nm, (x, z) in (("wheel-left", (0.27, 0.14)), ("wheel-right", (0.27, -0.14))):
        s.st(box(f"support-{nm}-pad", B, ST, 0.1, [0.03, 0.0075, 0.02], [x, 0.00375, z]), B, "support")
    for nm, bone, at, r in (("bowl", B, [0, 0.455, 0], 0.0625), ("lid", LD, [R, 0.07, 0], 0.045), ("charcoal", C, [0, 0, 0], 0.09), ("grate", G, [0, 0, 0], 0.045)):
        d = ellipsoid(f"mass-{nm}", bone, ST, 0.2, [r * (1.9 if nm == "bowl" else 1), r * (0.45 if nm in ("charcoal", "bowl") else 1), r * (1.9 if nm == "bowl" else 1)], at, role="detail")
        s.st(d, bone, "mass")


thermal50(s)
structure50(s)
sc.append(s)


def closed(id, bone, mat, tone, outer, origin=(0, 0, 0)):
    return revolve(id, bone, mat, tone, "y", list(origin), prof(outer))


def grate_rods(s, spacing, r, tag, thermal=False, kmax=None):
    n = int(round(0.26 / spacing)) if kmax is None else kmax
    out = []
    for i, k in enumerate(range(-n, n + 1)):
        z = k * spacing
        h = math.sqrt(max(0.268 ** 2 - z * z, 0))
        out.append(rod(f"grate-rod-{i + 1}", G, CR, 0.5, [-h, 0, z], [h, 0, z], r))
    return out


def octagon(id, bone, rr, r, n):
    out = []
    for i in range(n):
        a0, a1 = 2 * math.pi * i / n, 2 * math.pi * (i + 1) / n
        out.append(rod(f"{id}-{i + 1}", bone, CR, 0.5, [rr * math.cos(a0), 0, rr * math.sin(a0)], [rr * math.cos(a1), 0, rr * math.sin(a1)], r))
    return out


def lens(id, bone, tone, r, h, y0=0.0, mat=CC):
    return revolve(id, bone, mat, tone, "y", [0, 0, 0], [[y0, 0], [y0, r * 0.9], [y0 + h * 0.6, r], [y0 + h, r * 0.7], [y0 + h, 0], [y0, 0]])


def coarse(den, f, bowl_pts, lid_pts, pan, ring_slender=True):
    s = Scale(den)
    s.b(closed("bowl", B, EN, 0.3, bowl_pts))
    s.b(closed("lid", LD, EN, 0.3, lid_pts, (R, 0, 0)))
    return s


# ---- 1:100
s = Scale(100)
s.b(shell("bowl", B, EN, 0.3, BOWL_O, 0.015, [0, RIM], [0, 0, 0]))
for sg, nm in ((1, "left"), (-1, "right")):
    s.b(box(f"side-handle-{nm}", B, WD, 0.55, [0.07, 0.03, 0.035], [0, 0.73, sg * 0.3075], 0.004))
s.b(closed("ash-catcher", B, AL, 0.7, [[0.12, 0.5], [0.12, 0.44], [0.1, 0.42], [0.0, 0.42]]))
for k, (x, z) in SPOS.items():
    d = capsule(f"leg-{k}", B, ST, 0.15, [x, 0.012, z], list(LEG_TOP[k]), 0.012)
    d["slender"] = True
    s.b(d)
for k in ("bl", "br"):
    x, z = SPOS[k]
    s.b(ring_rev(f"foot-{k}", B, ST, 0.1, 0.0, 0.015, 0.0, 0.015, origin=(x, 0, z), role="body"))
for k in ("fl", "fr"):
    x, z = SPOS[k]
    zc = 0.1425 if z > 0 else -0.1425
    s.b(revolve(f"wheel-{k}", B, ST, 0.12, "z", [x, 0.05, zc - 0.0125], [[0, 0], [0.025, 0], [0.025, 0.05], [0, 0.05], [0, 0]]))
for i in range(4):
    a, b = ringp[i], ringp[(i + 1) % 4]
    s.b(rod(f"leg-ring-{i + 1}", B, ST, 0.15, [a[0], 0.22, a[1]], [b[0], 0.22, b[1]], 0.005))
s.b(lens("charcoal-bed", C, 0.1, 0.239, 0.123, -0.057))
s.body.extend(octagon("grate-rim", G, 0.27, 0.004, 8))
s.body.extend(grate_rods(s, 0.04, 0.005, "x"))
s.b(shell("lid", LD, EN, 0.3, LID_O, 0.015, [0, 0.0], [R, 0, 0]))
s.b(ext("lid-handle", LD, WD, 0.55, "xy", [[0.5475, 0.095], [0.5925, 0.1], [0.5925, 0.08], [0.6775, 0.08], [0.6775, 0.14], [0.5925, 0.14], [0.5925, 0.125], [0.5475, 0.13]], -0.03, 0.03))
s.b(ext("vent-dial", V, AL, 0.7, "yz", circle_pts(0, 0, 0.04, 28), -0.005, 0.03, holes=[circle_pts(0, 0, 0.012, 12)]))
s.th(envelope("th-bowl-envelope", B, EN, 0.3, BOWL_O, [0, 0, 0], ["+v"]), "bowl-shell")
d = ring_rev("open-bowl-plume-mouth", B, EN, 0.9, 0.735, 0.75, 0, 0.2, role="detail")
d["tags"] = ["open-bowl-plume"]
d["sectionFaces"] = ["+v", "-v"]
s.th(d, "bowl-shell")
s.th(envelope("th-lid-envelope", LD, EN, 0.3, LID_O, [R, 0, 0], ["-v"]), "lid-shell")
tof(s, "th-ash-catcher", "ash-catcher", "ash-catcher")
for i in ["leg-bl", "leg-br", "leg-fl", "leg-fr", "foot-bl", "foot-br", "wheel-fl", "wheel-fr"] + [f"leg-ring-{i}" for i in range(1, 5)]:
    tof(s, f"th-{i}", i, "legs-steel")
tof(s, "th-vent-dial", "vent-dial", "vents-alu")
d = revolve("lid-vent-flue-nozzle", V, AL, 0.9, "x", [0.0155, 0, 0], [[0, 0], [0, 0.025], [0.015, 0.025], [0.015, 0], [0, 0]], role="detail")
d["tags"] = ["lid-vent-flue"]
d["sectionFaces"] = ["+u", "-u"]
s.th(d, "vents-alu")
tof(s, "th-lid-handle", "lid-handle", "handle-wood")
for i in ("left", "right"):
    tof(s, f"th-side-handle-{i}", f"side-handle-{i}", "handle-wood")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    clone_th(s, b["id"], "th-" + b["id"], "cooking-grate", "rods")
d = lens("th-charcoal-bed", C, 0.1, 0.18, 0.03)
d["sectionFaces"] = ["-v"]
s.th(d, "charcoal-bed", "embers")


def struct_coarse(s, hollow, pad, pin_r, rod_ids, extra_members=()):
    for k, (x, z) in SPOS.items():
        tt = LEG_TOP[k]
        if hollow:
            s.st(tube(f"leg-tube-{k}", B, ST, 0.15, [x, 0.0, z], list(tt), 0.0105, 0.009), B, "member")
        else:
            d = tube(f"leg-tube-{k}", B, ST, 0.15, [x, 0.0, z], list(tt), 0.0125, 0.0)
            d["profile"] = [[0, 0], [0, 0.0125], [d["profile"][1][0], 0.0125], [d["profile"][1][0], 0], [0, 0]]
            d["slender"] = True
            s.st(d, B, "member")
    for i in extra_members:
        pass


def mass_solids(s, rr):
    for nm, bone, at, r in (("bowl", B, [0, 0.455, 0], rr), ("lid", LD, [R, 0.07, 0], rr * 0.72), ("charcoal", C, [0, 0, 0], rr * 1.44), ("grate", G, [0, 0, 0], rr * 0.72)):
        d = ellipsoid(f"mass-{nm}", bone, ST, 0.2, [r * (1.9 if nm == "bowl" else 1), r * (0.45 if nm in ("charcoal", "bowl") else 1), r * (1.9 if nm == "bowl" else 1)], at, role="detail")
        s.st(d, bone, "mass")


def supports(s, h, fw, rw):
    for nm, (x, z) in (("leg-rear-left", (-0.275, 0.145)), ("leg-rear-right", (-0.275, -0.145))):
        s.st(box(f"support-{nm}-pad", B, ST, 0.1, [fw, h, fw], [x, h / 2, z]), B, "support")
    for nm, (x, z) in (("wheel-left", (0.27, 0.14)), ("wheel-right", (0.27, -0.14))):
        s.st(box(f"support-{nm}-pad", B, ST, 0.1, [rw[0], h, rw[1]], [x, h / 2, z]), B, "support")


def pins(s, r, ln):
    s.st(revolve("lid-hinge-pin", LD, ST, 0.15, "z", [0.004, 0.002, -ln / 2], [[0, 0], [0, r], [ln, r], [ln, 0], [0, 0]], role="detail"), LD, "joint")
    s.st(revolve("vent-pin", V, ST, 0.15, "z", [0, 0, -ln / 2 + 0.01], [[0, 0], [0, r], [ln - 0.02, r], [ln - 0.02, 0], [0, 0]], role="detail"), V, "joint")
    s.st(revolve("damper-pin", D, ST, 0.15, "z", [0, -0.01, -ln / 2], [[0, 0], [0, r], [ln, r], [ln, 0], [0, 0]], role="detail"), D, "joint")


struct_coarse(s, True, 0.015, 0.0085, [])
s.stof("bowl-member", "bowl", B, "member")
for i in range(1, 5):
    s.stof(f"leg-ring-{i}-member", f"leg-ring-{i}", B, "member")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    s.stof(b["id"] + "-member", b["id"], G, "member")
s.stof("charcoal-bed-member", "charcoal-bed", C, "member")
s.stof("lid-member", "lid", LD, "member")
s.stof("lid-handle-member", "lid-handle", LD, "member")
s.stof("vent-dial-member", "vent-dial", V, "member")
s.st(revolve("ash-damper-member", D, AL, 0.7, "y", [0, -0.025, 0], [[0, 0], [0, 0.05], [0.025, 0.05], [0.025, 0], [0, 0]], role="detail"), D, "member")
pins(s, 0.0085, 0.05)
for k in ("fl", "fr"):
    s.stof(f"wheel-axle-{k}", f"wheel-{k}", B, "joint")
supports(s, 0.015, 0.04, (0.03, 0.02))
mass_solids(s, 0.0625)
sc.append(s)


def thermal_coarse(s, bowl_o, lid_o, pan_pts, plume_t, nozzle_t, y_mouth, with_ash_body):
    s.th(envelope("th-bowl-envelope", B, EN, 0.3, bowl_o, [0, 0, 0], ["+v"]), "bowl-shell")
    d = ring_rev("open-bowl-plume-mouth", B, EN, 0.9, y_mouth, y_mouth + plume_t, 0, 0.2, role="detail")
    d["tags"] = ["open-bowl-plume"]
    d["sectionFaces"] = ["+v", "-v"]
    s.th(d, "bowl-shell")
    s.th(envelope("th-lid-envelope", LD, EN, 0.3, lid_o, [R, 0, 0], ["-v"]), "lid-shell")
    if with_ash_body:
        tof(s, "th-ash-catcher", "ash-catcher", "ash-catcher")
    else:
        s.th(envelope("th-ash-catcher", B, AL, 0.7, pan_pts, [0, 0, 0], []), "ash-catcher")


# ---- 1:200
s = Scale(200)
s.b(shell("bowl", B, EN, 0.3, BOWL_O, 0.03, [0, RIM], [0, 0, 0]))
for sg, nm in ((1, "left"), (-1, "right")):
    s.b(box(f"side-handle-{nm}", B, WD, 0.55, [0.08, 0.03, 0.0275], [0, 0.73, sg * 0.30625], 0.004))
s.b(closed("ash-catcher", B, AL, 0.7, [[0.12, 0.5], [0.12, 0.43], [0.0, 0.43]]))
for k, (x, z) in SPOS.items():
    d = capsule(f"leg-{k}", B, ST, 0.15, [x, 0.012, z], list(LEG_TOP[k]), 0.012)
    d["slender"] = True
    s.b(d)
s.b(lens("charcoal-bed", C, 0.1, 0.239, 0.123, -0.057))
s.body.extend(octagon("grate-rim", G, 0.27, 0.004, 8))
s.body.extend(grate_rods(s, 0.09, 0.003, "x", kmax=2))
s.b(shell("lid", LD, EN, 0.3, LID_O, 0.03, [0, 0.0], [R, 0, 0]))
s.b(box("lid-handle", LD, WD, 0.55, [0.13, 0.06, 0.06], [0.6125, 0.11, 0], 0.012))
s.b(revolve("vent-dial", V, AL, 0.7, "x", [-0.005, 0, 0], [[0, 0], [0, 0.04], [0.035, 0.04], [0.035, 0], [0, 0]]))
thermal_coarse(s, BOWL_O, LID_O, None, 0.03, 0.03, 0.73, True)
for i in ["leg-bl", "leg-br", "leg-fl", "leg-fr"]:
    tof(s, f"th-{i}", i, "legs-steel")
tof(s, "th-vent-dial", "vent-dial", "vents-alu")
d = revolve("lid-vent-flue-nozzle", V, AL, 0.9, "x", [0.0, 0, 0], [[0, 0], [0, 0.025], [0.03, 0.025], [0.03, 0], [0, 0]], role="detail")
d["tags"] = ["lid-vent-flue"]
d["sectionFaces"] = ["+u", "-u"]
s.th(d, "vents-alu")
tof(s, "th-lid-handle", "lid-handle", "handle-wood")
for i in ("left", "right"):
    tof(s, f"th-side-handle-{i}", f"side-handle-{i}", "handle-wood")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    clone_th(s, b["id"], "th-" + b["id"], "cooking-grate", "rods")
d = lens("th-charcoal-bed", C, 0.1, 0.18, 0.03)
d["sectionFaces"] = ["-v"]
s.th(d, "charcoal-bed", "embers")
struct_coarse(s, False, 0.03, 0.016, [])
s.stof("bowl-member", "bowl", B, "member")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    s.stof(b["id"] + "-member", b["id"], G, "member")
s.stof("charcoal-bed-member", "charcoal-bed", C, "member")
s.stof("lid-member", "lid", LD, "member")
s.stof("lid-handle-member", "lid-handle", LD, "member")
s.stof("vent-dial-member", "vent-dial", V, "member")
s.st(revolve("ash-damper-member", D, AL, 0.7, "y", [0, -0.03, 0], [[0, 0], [0, 0.05], [0.03, 0.05], [0.03, 0], [0, 0]], role="detail"), D, "member")
pins(s, 0.016, 0.06)
supports(s, 0.03, 0.04, (0.04, 0.03))
mass_solids(s, 0.0625)
sc.append(s)

# ---- 1:500
BOWL5 = bez([R, 0.735], [R, 0.6], [0.18, 0.46], [0, 0.46], 24)
LID5 = [[R, -0.015], [R, 0.0]] + bez([R, 0.0], [R, 0.14], [0.17, 0.22], [0, 0.22], 24)[1:]
s = Scale(500)
s.b(shell("bowl", B, EN, 0.3, BOWL5, 0.075, [0, 0.735], [0, 0, 0]))
for sg, nm in ((1, "left"), (-1, "right")):
    s.b(box(f"side-handle-{nm}", B, WD, 0.55, [0.08, 0.04, 0.0275], [0, 0.73, sg * 0.30625], 0.004))
for k, (x, z) in SPOS.items():
    d = capsule(f"leg-{k}", B, ST, 0.15, [x, 0.012, z], list(LEG_TOP[k]), 0.012)
    d["slender"] = True
    s.b(d)
s.b(lens("charcoal-bed", C, 0.1, 0.239, 0.123, -0.057))
s.body.extend(octagon("grate-rim", G, 0.27, 0.004, 8))
s.b(shell("lid", LD, EN, 0.3, LID5, 0.075, [0, 0.0], [R, 0, 0]))
s.b(box("lid-handle", LD, WD, 0.55, [0.13, 0.06, 0.06], [0.6125, 0.11, 0], 0.012))
thermal_coarse(s, BOWL5, LID5, [[0.12, 0.5], [0.12, 0.43], [0.0, 0.43]], 0.075, 0.075, 0.7, False)
for i in ["leg-bl", "leg-br", "leg-fl", "leg-fr"]:
    tof(s, f"th-{i}", i, "legs-steel")
d = ring_rev("th-vent-block", V, AL, 0.7, -0.04, 0.0, 0, 0.0, role="body")
s.th(box("th-vent-block", V, AL, 0.7, [0.075, 0.08, 0.08], [0.0375 - 0.005, 0, 0]), "vents-alu")
d = box("lid-vent-flue-nozzle", V, AL, 0.9, [0.075, 0.075, 0.075], [0.0375 - 0.01, 0, 0], role="detail")
d["tags"] = ["lid-vent-flue"]
d["sectionFaces"] = ["+u", "-u"]
s.th(d, "vents-alu")
tof(s, "th-lid-handle", "lid-handle", "handle-wood")
for i in ("left", "right"):
    tof(s, f"th-side-handle-{i}", f"side-handle-{i}", "handle-wood")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    clone_th(s, b["id"], "th-" + b["id"], "cooking-grate", "rods")
d = lens("th-charcoal-bed", C, 0.1, 0.18, 0.075)
d["sectionFaces"] = ["-v"]
s.th(d, "charcoal-bed", "embers")
struct_coarse(s, False, 0.075, 0.04, [])
s.stof("bowl-member", "bowl", B, "member")
for b in [b for b in s.body if b["id"].startswith("grate-")]:
    s.stof(b["id"] + "-member", b["id"], G, "member")
s.stof("charcoal-bed-member", "charcoal-bed", C, "member")
s.stof("lid-member", "lid", LD, "member")
s.stof("lid-handle-member", "lid-handle", LD, "member")
s.st(box("vent-member", V, AL, 0.7, [0.075, 0.08, 0.08], [0.0375 - 0.005, 0, 0]), V, "member")
s.st(revolve("ash-damper-member", D, AL, 0.7, "y", [0, -0.075, 0], [[0, 0], [0, 0.05], [0.075, 0.05], [0.075, 0], [0, 0]], role="detail"), D, "member")
pins(s, 0.04, 0.1)
supports(s, 0.075, 0.075, (0.075, 0.075))
mass_solids(s, 0.0625)
sc.append(s)

DEC = {
50: [
("Body: the kettle as a real thin-walled object instead of the seed's stack of extruded discs: the bowl and the lid are 0.8 mm enamel shells (revolves of the drawn profile: 240 mm deep bowl, 200 mm dome with a 5 mm skirt, each with a rolled rim bead) so that the open-lid poses show the inside, the ash catcher is a 2 mm pan hanging under the bowl, the four 24 mm legs are slender tubes with a leg ring, two rear feet and two front wheels (tyre ring and hub), the two side handles and the lid handle on two brackets are wood, the lid vent is a 80 mm dial of 35 mm with three through holes, the ash damper a 100 mm disc. Inside the bowl: the charcoal mound and the cooking grate as an 11 m long set of 27 Ø6 mm rods at 20 mm spacing in a Ø8 mm ring with two braces (the 2D draws five sample bars as texture; a real grate has about 25). The seed's disc stacks are replaced; the grate rods are slender (C2) and drawn in the thermal list as the hidden grate.",
 "Körper: der Kugelgrill als echtes dünnwandiges Objekt statt des Scheibenstapels der Vorlage: Kessel und Deckel sind Emailleschalen von 0,8 mm (Rotationskörper des gezeichneten Profils: Kessel 240 mm tief, Deckelhaube 200 mm mit 5 mm Schürze, je mit Randwulst), damit die Posen mit offenem Deckel das Innere zeigen, der Aschefänger ist eine Wanne von 2 mm unter dem Kessel, die vier Beine von 24 mm sind schlanke Rohre mit Beinring, zwei hinteren Füßen und zwei vorderen Rädern (Reifenring und Nabe), die zwei Seitengriffe und der Deckelgriff auf zwei Haltern sind aus Holz, das Deckelventil ist eine Scheibe von 80 mm x 35 mm mit drei Durchgangslöchern, die Aschenklappe eine Scheibe von 100 mm. Im Kessel: der Holzkohlehaufen und der Rost als Satz von 27 Stäben Ø6 mm im Abstand von 20 mm (11 m Stablänge) in einem Ring Ø8 mm mit zwei Streben (die 2D-Zeichnung zeigt fünf Beispielstäbe als Textur; ein echter Rost hat etwa 25). Die Scheibenstapel der Vorlage sind ersetzt; die Roststäbe sind schlank (C2)."),
("Thermal: the shells are not exact body surfaces (the body is hollow, the region is the outer surface), so bowl-shell and lid-shell are real closed envelope solids of the outer profile whose rim faces are section faces (+v / -v), the ash catcher is its closed envelope, legs, vent and handles reference the body; both emitters are tagged discs on the region they leave from (the 400 mm mouth of the open bowl, the 50 mm vent outlet) with their disc faces declared section faces; the hidden regions are real solids of their own: the grate (a copy of the 27 rods, ring and braces) and a 30 mm ember lens of 180 mm radius whose underside is a section face. Areas corrected at the root (all within a factor 1.5, C16): bowl-shell 0.45 -> 0.4 (the pan buries part of the bowl bottom), ash-catcher 0.12 -> 0.14, handle-wood 0.03 -> 0.045; vents-alu (0.01 against 0.0172 m2, ratio 1.7) is not applied and listed in area-review.md.",
 "Thermisch: die Schalen sind keine exakten Körperflächen (der Körper ist hohl, die Region ist die Außenfläche), daher sind bowl-shell und lid-shell eigene geschlossene Hüllkörper des Außenprofils, deren Randflächen Schnittflächen sind (+v / -v), der Aschefänger ist seine geschlossene Hülle, Beine, Ventil und Griffe verweisen auf den Körper; beide Emitter sind markierte Scheiben auf der Region, aus der sie austreten (die Mündung von 400 mm des offenen Kessels, der Auslass von 50 mm am Ventil) mit Scheibenflächen als Schnittflächen; die verdeckten Regionen sind eigene Körper: der Rost (eine Kopie der 27 Stäbe, des Rings und der Streben) und eine Glutlinse von 30 mm mit 180 mm Radius, deren Unterseite eine Schnittfläche ist. An der Wurzel berichtigte Flächen (alle innerhalb des Faktors 1,5, C16): bowl-shell 0,45 -> 0,4 (die Wanne begräbt einen Teil des Kesselbodens), ash-catcher 0,12 -> 0,14, handle-wood 0,03 -> 0,045; vents-alu (0,01 gegen 0,0172 m2, Verhältnis 1,7) ist nicht angewandt und in area-review.md aufgeführt."),
("Structure: member solids for all six load-bearing bones: four hollow 21 x 1.5 mm leg tubes (real sections inside the 24 mm skin) plus the bowl shell and the four ring rods for the bowl, the grate rim and rods, the ember mound, the lid shell and the two handle brackets, the vent dial and the ash damper (all referenced from the body). Joints: the lid hinge pin, the vent pin, the damper pin (Ø16 mm) and the two wheel hubs; supports: a 40 x 7.5 x 40 mm contact pad under each rear foot and a 30 x 7.5 x 20 mm pad under each wheel at the four contact anchors; mass solids for the four concentrated masses (bowl 8 kg as a flat ellipsoid inside the ash pan, lid 3 kg, charcoal 1.5 kg, grate 1.3 kg; the authored bowl centre of mass at 0.42 m lies below the body, so its solid sits at 0.455 m).",
 "Tragwerk: Stabkörper für alle sechs tragenden Knochen: vier Hohlbeinrohre 21 x 1,5 mm (echte Querschnitte innerhalb der Haut von 24 mm) sowie Kesselschale und die vier Ringstäbe für den Kessel, Rostring und Roststäbe, der Glutberg, Deckelschale und die zwei Griffhalter, die Ventilscheibe und die Aschenklappe (aus dem Körper referenziert). Gelenke: der Deckelscharnierbolzen, der Ventilbolzen, der Klappenbolzen (Ø16 mm) und die zwei Radnaben; Auflager: eine Kontaktplatte 40 x 7,5 x 40 mm unter jedem hinteren Fuß und eine 30 x 7,5 x 20 mm unter jedem Rad an den vier Kontaktankern; Massenkörper für die vier konzentrierten Massen (Kessel 8 kg als flaches Ellipsoid in der Aschenwanne, Deckel 3 kg, Holzkohle 1,5 kg, Rost 1,3 kg; der vorgegebene Schwerpunkt des Kessels bei 0,42 m liegt unter dem Körper, sein Körper sitzt bei 0,455 m)."),
],
100: [
("Body: the shells are thickened to the 15 mm feature (a 0.8 mm wall cannot be resolved; a closed solid dish would triple the volume), the rim beads and the hub, the 2 mm pan wall, the bracket pair of the lid handle and the three vent holes are merged (one hole remains, as drawn): bowl and lid shells, the closed ash pan, four slender legs, a leg ring of four rods, two feet of 15 mm height, two wheel discs without hubs, the two side handles, the lid handle as the drawn extruded outline, the vent dial with its central hole, a 13-rod grate of Ø10 mm rods at 40 mm spacing inside an octagon ring, and the ember mound.",
 "Körper: die Schalen sind auf die Einzelheit 15 mm verdickt (eine Wand von 0,8 mm ist nicht auflösbar, eine geschlossene Vollschale verdreifachte das Volumen), die Randwülste und die Naben, die 2 mm der Wannenwand, das Halterpaar des Deckelgriffs und die drei Ventillöcher sind zusammengefasst (ein Loch bleibt, wie gezeichnet): Kessel- und Deckelschale, die geschlossene Aschenwanne, vier schlanke Beine, ein Beinring aus vier Stäben, zwei Füße von 15 mm Höhe, zwei Radscheiben ohne Naben, die beiden Seitengriffe, der Deckelgriff als gezeichneter extrudierter Umriss, die Ventilscheibe mit ihrem Mittelloch, ein Rost aus 13 Stäben Ø10 mm im Abstand von 40 mm in einem Achteckring und der Glutberg."),
("Thermal: as at 1:50 (outer-profile envelopes for the two shells and the ash pan, references for legs, vent and handles, tagged emitter discs of 15 mm, a copy of the 13-rod grate and the ember lens as hidden solids). vents-alu measures 0.0155 m2 against 0.01 (ratio 1.55, not applied, area-review.md).",
 "Thermisch: wie bei 1:50 (Hüllkörper des Außenprofils für die beiden Schalen und die Aschenwanne, Verweise für Beine, Ventil und Griffe, markierte Emitterscheiben von 15 mm, eine Kopie des Rosts aus 13 Stäben und die Glutlinse als verdeckte Körper). vents-alu misst 0,0155 m2 gegen 0,01 (Verhältnis 1,55, nicht angewandt, area-review.md)."),
("Structure: leg tubes stay hollow (21 x 1.5 mm), the shell, ring rods, grate rods, ember mound, lid shell and handle are the referenced members, the vent dial a member, the ash damper a real 100 x 25 mm member (the 1:100 drawing does not draw it, so it has no body solid), pins Ø17 mm, wheel hubs merged into the wheels as the axle joints, pads of 15 mm.",
 "Tragwerk: die Beinrohre bleiben hohl (21 x 1,5 mm), Schale, Ringstäbe, Roststäbe, Glutberg, Deckelschale und Griff sind die referenzierten Stäbe, die Ventilscheibe ein Stab, die Aschenklappe ein eigener Stab von 100 x 25 mm (die Zeichnung 1:100 zeichnet sie nicht, daher hat sie keinen Körper), Bolzen Ø17 mm, die Radnaben als Achsgelenke in den Rädern aufgegangen, Platten von 15 mm."),
],
200: [
("Body: shells thickened to the 30 mm feature, the drawing has no wheels, feet or leg ring any more: bowl and lid shells, a 70 mm ash cylinder (the drawn rectangle), four slender legs (the drawing is a line; true Ø24 mm), the side handles at the drawn 27.5 mm (their thinnest drawn extent, C13), the lid handle as a 130 x 60 x 60 mm block, the vent dial as a plain 35 mm cylinder, a 5-bar grate of Ø6 mm rods at 90 mm in an octagon ring, and the ember mound.",
 "Körper: Schalen auf die Einzelheit 30 mm verdickt, die Zeichnung hat keine Räder, Füße und keinen Beinring mehr: Kessel- und Deckelschale, ein Aschenzylinder von 70 mm (das gezeichnete Rechteck), vier schlanke Beine (die Zeichnung ist eine Linie; wahres Ø24 mm), die Seitengriffe in den gezeichneten 27,5 mm (ihre dünnste gezeichnete Ausdehnung, C13), der Deckelgriff als Block 130 x 60 x 60 mm, die Ventilscheibe als glatter Zylinder von 35 mm, ein Rost aus 5 Stäben Ø6 mm im Abstand von 90 mm in einem Achteckring und der Glutberg."),
("Thermal: references and envelopes as at 1:100 with 30 mm emitter discs; only the total over all regions is compared.",
 "Thermisch: Verweise und Hüllkörper wie bei 1:100 mit Emitterscheiben von 30 mm; verglichen wird nur die Summe über alle Regionen."),
("Structure: the legs become slender solid rods (Ø25 mm; a 1.5 mm wall is below the feature), the other members are referenced, the pins grow to Ø32 mm, pads to 30 mm.",
 "Tragwerk: die Beine werden schlanke Vollstäbe (Ø25 mm; eine Wand von 1,5 mm liegt unter der Einzelheit), die übrigen Stäbe sind referenziert, die Bolzen wachsen auf Ø32 mm, die Platten auf 30 mm."),
],
500: [
("Body: the 1:500 drawing is the kettle on four lines: bowl and lid shells thickened to 75 mm with the drawn 275 mm deep bowl and 220 mm lid, four slender legs, the side handles and the lid handle (60 mm block, the front and plan rectangle; the elevation stroke was repaired as a filled rectangle at the root), the ember mound and the grate ring; ash pan, vent and hub are below the feature and not drawn.",
 "Körper: die Zeichnung 1:500 ist der Kessel auf vier Linien: Kessel- und Deckelschale auf 75 mm verdickt mit dem gezeichneten 275 mm tiefen Kessel und 220 mm Deckel, vier schlanke Beine, die Seitengriffe und der Deckelgriff (Block 60 mm, das Rechteck von Ansicht und Grundriss; der Strich im Aufriss ist an der Wurzel als gefülltes Rechteck korrigiert), der Glutberg und der Rostring; Aschenwanne, Ventil und Nabe liegen unter der Einzelheit und sind nicht gezeichnet."),
("Thermal: envelopes of the two shells, a 70 mm ash pan solid and 75 mm vent block, emitter blocks of 75 mm, the handles by reference, the grate ring and a 75 mm ember lens as hidden solids; only the total is compared.",
 "Thermisch: Hüllkörper der beiden Schalen, eine Aschenwanne von 70 mm und ein Ventilblock von 75 mm, Emitterblöcke von 75 mm, die Griffe als Verweis, der Rostring und eine Glutlinse von 75 mm als verdeckte Körper; verglichen wird nur die Summe."),
("Structure: slender solid legs, the shell, grate ring, ember mound, lid and handle referenced, vent and ash damper as 75 mm members, 100 mm pins, 75 mm pads.",
 "Tragwerk: schlanke Vollbeine, Schale, Rostring, Glutberg, Deckel und Griff referenziert, Ventil und Aschenklappe als Stäbe von 75 mm, Bolzen 100 mm, Platten 75 mm."),
],
}
for _s in sc:
    for en, de in DEC[_s.den]:
        _s.dec(en, de)
save_asset("charcoal-grill", sc)
