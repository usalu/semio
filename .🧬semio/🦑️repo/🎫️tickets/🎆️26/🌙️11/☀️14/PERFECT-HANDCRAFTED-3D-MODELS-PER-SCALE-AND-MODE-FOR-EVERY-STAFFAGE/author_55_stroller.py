"""Author 55: parent-with-stroller props (tube frame, fabric seat sling, hood, basket, four wheels)."""
import sys; sys.path.insert(0, ".")
from author_55_lib import *
from author_55_rollator import tub_band
SLUG = "parent-with-stroller"; SKIN = "skin"
ALU, TEX, RUB, GRP = "aluminium", "stroller-textile", "rubber", "rubber"
FB = {"stroller", "stroller-hood", "wheel-rear", "wheel-front"}; REG = {"stroller-fabric", "stroller-frame", "tyres"}
ZC = 0.1925
FEAT = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
HOOD = [[0, 0], [0.002, 0.014], [0.006, 0.027], [0.01, 0.041], [0.015, 0.053], [0.022, 0.065], [0.029, 0.077], [0.037, 0.088], [0.045, 0.099], [0.055, 0.109], [0.065, 0.118], [0.087, 0.136], [0.111, 0.15], [0.138, 0.161], [0.166, 0.17], [0.18, 0.172], [0.195, 0.174], [0.21, 0.175], [0.225, 0.176], [0.24, 0.175], [0.256, 0.173], [0.272, 0.17], [0.287, 0.166], [0.303, 0.161], [0.319, 0.155], [0.334, 0.148], [0.35, 0.14], [0.365, 0.131], [0.38, 0.12], [0.401, 0.1], [0.419, 0.079], [0.434, 0.057], [0.446, 0.034], [0.455, 0.01], [0.46, -0.014], [0.462, -0.037], [0.461, -0.049], [0.46, -0.06], [0.4, -0.06], [0.396, -0.049], [0.39, -0.038], [0.383, -0.027], [0.374, -0.016], [0.365, -0.005], [0.354, 0.005], [0.343, 0.015], [0.33, 0.024], [0.316, 0.032], [0.302, 0.04], [0.287, 0.048], [0.271, 0.054], [0.254, 0.059], [0.236, 0.064], [0.218, 0.068], [0.2, 0.07], [0.164, 0.069], [0.131, 0.065], [0.101, 0.058], [0.074, 0.05], [0.05, 0.04], [0.03, 0.028], [0.013, 0.015]]
def tube(b, n, id, p0, p1, r, reg="stroller-frame", bone="stroller", tone=0.4, slender=None, mat=ALU):
    L = math.dist(p0, p1); s = (L >= 20 * 2 * r) if slender is None else slender
    b.append((cap(id, bone, mat, p0, p1, r, tone=tone, **({"slender": True} if s and n >= 100 else {})), reg))
def frame(n):
    b = []; r = {50: (0.012, 0.013, 0.011, 0.01), 100: (0.0115, 0.0125, 0.0105, 0.0095), 200: (0.012, 0.013, 0.0155, 0.0155), 500: (0.0065, 0.0065, 0.0065, 0.0065)}[n]
    for s, z in (("l", 0.24), ("r", -0.24)):
        zz = ZC + z
        tube(b, n, f"push-bar-{s}", [0.03, -0.01, zz], [0.22, -0.58, zz], r[0])
        tube(b, n, f"chassis-{s}", [0.21, -0.575, zz], [0.79, -0.62, zz], r[1])
        zs = ZC + z * 1.1667
        tube(b, n, f"rear-strut-{s}", [0.24, -0.58, zz], [0.28, -0.841, zs], r[2])
        zf = ZC + z * 0.8333
        tube(b, n, f"front-fork-{s}", [0.77, -0.62, zf], [0.8, -0.92, zf], r[3])
        if n == 50:
            tube(b, n, f"footrest-arm-{s}", [0.73, -0.665, ZC + z * 0.5], [0.77, -0.64, zf], 0.006, tone=0.5)
            tube(b, n, f"seat-hanger-{s}", [0.36, -0.59, zz], [0.36, -0.59, ZC + z * 0.4375], 0.006, tone=0.5)
            tube(b, n, f"seat-hanger2-{s}", [0.6, -0.59, zz], [0.6, -0.59, ZC + z * 0.4375], 0.006, tone=0.5)
            tube(b, n, f"back-hanger-{s}", [0.12, -0.3, zz], [0.2, -0.3, ZC + z * 0.4375], 0.006, tone=0.5)
            tube(b, n, f"basket-strap-{s}", [0.31, -0.6, zz], [0.31, -0.7, ZC + z * 0.8333], 0.005, tone=0.5)
            tube(b, n, f"basket-strap2-{s}", [0.65, -0.6, zz], [0.65, -0.7, ZC + z * 0.8333], 0.005, tone=0.5)
    if n <= 100: tube(b, n, "cross-front", [0.79, -0.62, ZC - 0.24], [0.79, -0.62, ZC + 0.24], r[1], tone=0.45)
    if n == 50:
        tube(b, n, "cross-rear", [0.28, -0.841, ZC - 0.28], [0.28, -0.841, ZC + 0.28], 0.008, tone=0.5)
        tube(b, n, "cross-fork", [0.8, -0.92, ZC - 0.2], [0.8, -0.92, ZC + 0.2], 0.007, tone=0.5)
        b.append((lofts("handle-grip", "stroller", GRP, [(-0.0, 0.0, 0.0)], tone=0.15), "stroller-frame"))
        g = b[-1][0]; g["kind"] = "loft"
        g["sections"] = [{"at": [0.0, 0.0, w], "rx": rr, "ry": rr, "exponent": 2.0} for w, rr in ((ZC - 0.2575, 0.0105), (ZC - 0.24, 0.017), (ZC - 0.12, 0.0175), (ZC + 0.12, 0.0175), (ZC + 0.24, 0.017), (ZC + 0.2575, 0.0105))]
        g["sections"][0]["at"] = [0.0, 0.0, ZC - 0.2575]; g["sections"][-1]["at"] = [0.0, 0.0, ZC + 0.2575]
        b.append((box("footrest", "stroller", ALU, [0.67, -0.645, ZC], [0.148, 0.012, 0.24], tone=0.45, rot=quat_axis([0, 0, 1], -19.6)), "stroller-frame"))
    else:
        rg = {100: 0.017, 200: 0.0175, 500: 0.0375}[n]
        b.append((cap("handle-grip", "stroller", GRP, [0.0, 0.0, ZC - 0.2575 + (0 if n < 500 else 0.0)], [0.0, 0.0, ZC + 0.2575], rg, tone=0.15), "stroller-frame"))
        if n == 100: b.append((box("footrest", "stroller", ALU, [0.67, -0.645, ZC], [0.148, 0.015, 0.24], tone=0.45, rot=quat_axis([0, 0, 1], -19.6)), "stroller-frame"))
    return b
PAD = [[0.19, -0.58], [0.17, -0.14], [0.23, -0.12], [0.261, -0.354], [0.279, -0.436], [0.3, -0.49], [0.64, -0.5], [0.66, -0.56], [0.3, -0.58], [0.229, -0.58], [0.22, -0.585], [0.21, -0.581]]
def seat(n):
    t = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[n]
    cl = [[0.185, -0.135], [0.20, -0.2], [0.245, -0.35], [0.27, -0.435], [0.30, -0.50], [0.45, -0.51], [0.64, -0.515], [0.66, -0.56]]
    z0, z1 = 0.0725, 0.3125
    if n <= 100:
        return [(extrude("seat-sling", "stroller", TEX, "xy", tub_band(cl, t), z0 + t, z1 - t, tone=0.55), "stroller-fabric"), (extrude("seat-side-l", "stroller", TEX, "xy", PAD, z1 - t, z1, tone=0.6), "stroller-fabric"), (extrude("seat-side-r", "stroller", TEX, "xy", PAD, z0, z0 + t, tone=0.6), "stroller-fabric")]
    if n == 200: return [(extrude("seat-pad", "stroller", TEX, "xy", PAD, z0, z1, tone=0.55), "stroller-fabric")]
    return [(extrude("seat-pad", "stroller", TEX, "xy", [[0.17, -0.58], [0.17, -0.14], [0.285, -0.14], [0.3, -0.5], [0.66, -0.5], [0.66, -0.58]], 0.045, 0.34, tone=0.55), "stroller-fabric")]
def hood(n):
    t = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[n]
    outer = HOOD[:38]; inner = HOOD[38:]
    out = []
    side = [p for p in HOOD]
    z0, z1 = -0.0675 + 0.0, 0.4525
    if n == 50:
        out.append((extrude("hood-side-l", "stroller-hood", TEX, "xy", side, z1 - t, z1, tone=0.6), "stroller-fabric"))
        out.append((extrude("hood-side-r", "stroller-hood", TEX, "xy", side, z0, z0 + t, tone=0.6), "stroller-fabric"))
        out.append((extrude("hood-top", "stroller-hood", TEX, "xy", tub_band(outer[2:-1], t), z0, z1, tone=0.6), "stroller-fabric"))
    elif n == 100:
        out.append((extrude("hood-side-l", "stroller-hood", TEX, "xy", side, z1 - t, z1, tone=0.6), "stroller-fabric"))
        out.append((extrude("hood-side-r", "stroller-hood", TEX, "xy", side, z0, z0 + t, tone=0.6), "stroller-fabric"))
        out.append((extrude("hood-top", "stroller-hood", TEX, "xy", tub_band(outer[2:-1:2], t), z0, z1, tone=0.6), "stroller-fabric"))
    else:
        out.append((extrude("hood", "stroller-hood", TEX, "xy", [p for i, p in enumerate(HOOD) if i % (3 if n == 200 else 5) == 0], z0 if n == 200 else -0.0675, z1, tone=0.6), "stroller-fabric"))
    return out
def basket(n):
    t = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[n]
    x0, x1, y0, y1 = 0.3, 0.68, -0.8, -0.7
    zh = 0.2; out = []
    if n >= 200:
        out.append((box("basket", "stroller", TEX, [0.49, -0.75, ZC], [0.38, 0.1 if n == 200 else 0.1, 0.4], tone=0.6), "stroller-fabric")); return out
    out.append((box("basket-floor", "stroller", TEX, [0.49, y0 + t / 2, ZC], [0.38, t, 0.4], tone=0.6), "stroller-fabric"))
    for k, x in (("f", x1 - t / 2), ("b", x0 + t / 2)): out.append((box(f"basket-wall-{k}", "stroller", TEX, [x, -0.75, ZC], [t, 0.1, 0.4], tone=0.6), "stroller-fabric"))
    for k, z in (("l", ZC + zh - t / 2), ("r", ZC - zh + t / 2)): out.append((box(f"basket-wall-{k}", "stroller", TEX, [0.49, -0.75, z], [0.38 - 2 * t, 0.1, t], tone=0.6), "stroller-fabric"))
    if n == 50:
        for i, (p0, p1) in enumerate((([x0, y1, ZC - zh], [x1, y1, ZC - zh]), ([x0, y1, ZC + zh], [x1, y1, ZC + zh]), ([x0, y1, ZC - zh], [x0, y1, ZC + zh]), ([x1, y1, ZC - zh], [x1, y1, ZC + zh])), 1):
            out.append((cap(f"basket-wire-{i}", "stroller", ALU, p0, p1, 0.0025, role="detail", tone=0.7, slender=True), "stroller-fabric"))
    return out
def wheels(n):
    out = []
    for w, bone, R, zws in (("rear", "wheel-rear", 0.159, (0.28, -0.28)), ("front", "wheel-front", 0.08, (0.2, -0.2))):
        for zw, s in zip(zws, ("l", "r")):
            z = zw + 0.193
            if n == 50:
                ri = R - (0.045 if w == "rear" else 0.02)
                tr = [[-0.02, ri], [-0.0195, R - 0.004], [-0.012, R], [0.012, R], [0.0195, R - 0.004], [0.02, ri]]
                out.append((revolve(f"tyre-{w}-{s}", bone, RUB, tr, "w", at=[0, 0, z], tone=0.1), "tyres"))
                out.append((revolve(f"rim-{w}-{s}", bone, ALU, [[-0.0215, 0.0], [-0.0215, ri + 0.002], [0.0215, ri + 0.002], [0.0215, 0.0]], "w", at=[0, 0, z], tone=0.75), "tyres"))
                out.append((ell(f"hub-{w}-{s}", bone, ALU, [0, 0, z + (0.0155 if zw > 0 else -0.0155)], [0.02 if w == "rear" else 0.014] * 2 + [0.005], tone=0.3), "tyres"))
            elif n == 100:
                ri = R - (0.045 if w == "rear" else 0.025)
                tr = [[-0.02, ri], [-0.0185, R - 0.006], [-0.012, R], [0.012, R], [0.0185, R - 0.006], [0.02, ri]]
                out.append((revolve(f"tyre-{w}-{s}", bone, RUB, tr, "w", at=[0, 0, z], tone=0.1), "tyres"))
                out.append((revolve(f"rim-{w}-{s}", bone, ALU, [[-0.0215, ri - 0.015], [-0.0215, ri + 0.002], [0.0215, ri + 0.002], [0.0215, ri - 0.015]], "w", at=[0, 0, z], tone=0.75), "tyres"))
                out.append((revolve(f"hub-{w}-{s}", bone, ALU, [[-0.0215, 0.0], [-0.0215, 0.02], [0.0215, 0.02], [0.0215, 0.0]], "w", at=[0, 0, z], tone=0.3), "tyres"))
                for k in range(4):
                    a = math.pi / 4 + k * math.pi / 2; c, sn = math.cos(a), math.sin(a)
                    out.append((cap(f"spoke-{w}-{s}-{k}", bone, ALU, [0.018 * c, 0.018 * sn, z], [(ri - 0.012) * c, (ri - 0.012) * sn, z], 0.0078, tone=0.6), "tyres"))
            else:
                k = 1.0 if n == 200 else 0.0385 / 0.02
                zz = z if n == 200 else (zw + 0.193) + (0.0 if zw > 0 else 0.0)
                out.append((revolve(f"wheel-{w}-{s}", bone, RUB, [[-0.02 * k, 0.0], [-0.02 * k, R - 0.006], [-0.012 * k, R], [0.012 * k, R], [0.02 * k, R - 0.006], [0.02 * k, 0.0]], "w", at=[0, 0, zz], tone=0.1), "tyres"))
    return out
def props(n):
    parts = frame(n) + seat(n) + hood(n) + basket(n) + wheels(n)
    body = [p for p, _ in parts]; reg = {p["id"]: r for p, r in parts}
    thermal = [th(p["id"], reg[p["id"]]) for p in body]
    st = []
    rm = {50: 0.0085, 100: 0.0085, 200: 0.0125, 500: 0.0065}[n]
    mem = [(f"push-bar-{s}", [0.03, -0.01, ZC + z], [0.22, -0.58, ZC + z]) for s, z in (("l", 0.24), ("r", -0.24))] + [(f"chassis-{s}", [0.21, -0.575, ZC + z], [0.79, -0.62, ZC + z]) for s, z in (("l", 0.24), ("r", -0.24))]
    for mid, p0, p1 in mem:
        st.append({**cap(f"member-{mid}", "stroller", ALU, p0, p1, rm, tone=0.5), "member": "stroller", "part": "member"})
        if n >= 200 and math.dist(p0, p1) >= 40 * rm: st[-1]["slender"] = True
    for s in ("l", "r"):
        for ids in ([f"rear-strut-{s}", f"front-fork-{s}"] if n <= 100 else []):
            pass
    hid = {50: ["hood-top"], 100: ["hood-top"], 200: ["hood"], 500: ["hood"]}[n]
    for h in hid: st.append({"id": f"member-{h}", "of": h, "member": "stroller-hood", "part": "member"})
    for w in ("rear", "front"):
        z = 0.28 if w == "rear" else 0.2
        R = 0.159 if w == "rear" else 0.08
        st.append({**cap(f"member-wheel-{w}", f"wheel-{w}", ALU, [0, 0, 0.193 - z], [0, 0, 0.193 + z], 0.0095 if n < 500 else 0.0375, tone=0.5), "member": f"wheel-{w}", "part": "member"})
        if n >= 100 and n < 500: st[-1]["slender"] = True
    st.append({**ell("mass-stroller", "stroller", ALU, [0.45, -0.58, ZC], [0.05, 0.03, 0.05] if n < 500 else [0.0385, 0.0385, 0.0385], role="detail", tone=0.6), "member": "stroller", "part": "mass"})
    st.append({**ell("mass-wheel-rear", "wheel-rear", ALU, [0, 0, 0.473], [0.04, 0.04, 0.015] if n < 500 else [0.0385, 0.0385, 0.0385], role="detail", tone=0.6), "member": "wheel-rear", "part": "mass"})
    st.append({**ell("mass-wheel-front", "wheel-front", ALU, [0, 0, 0.393], [0.03, 0.03, 0.015] if n < 500 else [0.0385, 0.0385, 0.0385], role="detail", tone=0.6), "member": "wheel-front", "part": "mass"})
    return body, thermal, st
D = {50: ("Stroller as a real object: aluminium tube frame (push bars, chassis rails, splayed rear struts, front forks, cross tubes, footrest, hangers and basket straps), moulded grip, fabric seat as a 8 mm sling shell, hood as a three-part shell (two side panels and a curved top sheet), basket as a tray of thin sheets with a slender rim wire, four wheels as tyre ring, rim disc and hub cap; thermal solids are the same solids by region, structure members are tube cores, hood shell references and wheel axles.", "Kinderwagen als echter Gegenstand: Aluminium-Rohrgestell (Schiebebügel, Chassisholme, gespreizte hintere Streben, Vordergabeln, Querrohre, Fußstütze, Aufhängungen und Korbgurte), angeformter Griff, Stoffsitz als 8-mm-Hängeschale, Verdeck als dreiteilige Schale (zwei Seitenteile und gewölbte Decke), Korb als Wanne aus dünnen Blechen mit schlankem Randdraht, vier Räder aus Reifenring, Felgenscheibe und Nabenkappe; Thermik-Solids sind dieselben Solids je Region, Tragelemente sind Rohrkerne, Verdeckschalen-Verweise und Radachsen."),
     100: ("Stroller at 1:100: tubes as capsules (long ones slender), 15 mm sling, hood as two side panels and a top sheet, basket tray, wheels as tyre ring plus rim; hangers, straps and hub caps merged away.", "Kinderwagen 1:100: Rohre als Kapseln (lange schlank), 15-mm-Sitzschale, Verdeck aus zwei Seitenteilen und Decke, Korbwanne, Räder als Reifenring mit Felge; Aufhängungen, Gurte und Nabenkappen entfallen."),
     200: ("Stroller at 1:200: four slender tubes at true size, 30 mm seat sling, hood as one shell, basket block, wheels as solid discs as the drawing fills them.", "Kinderwagen 1:200: vier schlanke Rohre in wahrer Größe, 30-mm-Sitzschale, Verdeck als eine Schale, Korbblock, Räder als Vollscheiben wie gezeichnet."),
     500: ("Stroller at 1:500: slender tubes at true size, seat, hood and basket as blocks at the feature size, wheels as 77 mm discs.", "Kinderwagen 1:500: schlanke Rohre in wahrer Größe, Sitz, Verdeck und Korb als Blöcke in Mindestgröße, Räder als 77-mm-Scheiben.")}
def post(d):
    for n in (50, 100, 200, 500):
        for x in d["volumes"][str(n)]["structure"]:
            if x["id"] == "mass-child":
                x["transform"]["translation"] = [-0.01, 0.12, ZC]; x["radii"] = [max(r, 0.6 * FEAT[n]) for r in x["radii"]]
            if x["id"] == "mass-stroller" and n < 500: x["transform"]["translation"] = [0.49, -0.75, ZC]; x["radii"] = [0.04, 0.03, 0.04]
    for x in d["volumes"]["500"]["structure"]:
        if x["id"] in ("member-upper-arm-l", "member-forearm-l"):
            if "sections" in x:
                for q in x["sections"]: q["at"][2] -= 0.1
            else: x["p0"][2] -= 0.1; x["p1"][2] -= 0.1
    ensure_bones(SLUG, d, 200, {"forearm-r": ("forearm-r-sleeve", "cotton-clothing", "arm-clothing"), "forearm-l": ("forearm-l-sleeve", "cotton-clothing", "arm-clothing")})
    ensure_bones(SLUG, d, 500, {"torso": ("torso-jacket", "cotton-clothing", "torso-clothing"), "forearm-r": ("forearm-r-sleeve", "cotton-clothing", "arm-clothing")})
