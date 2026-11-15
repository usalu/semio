"""Author 55: person-with-rollator props (frame, seat, backrest, basket tub, bag, wheels). `python3 author-55-rollator-run.py build`."""
import sys; sys.path.insert(0, ".")
from author_55_lib import *
SLUG = "person-with-rollator"; SKIN = "skin-aged"
ALU, NYL, GRIP, TYRE, TOTE = "alu-red-paint", "nylon-seat", "rubber-grip", "pu-tyre", "cotton-tote"
FB = {"frame", "wheel-rear", "wheel-front", "aid-lift", "aid-slide"}
REG = {"frame", "seat-basket", "grips", "tyres", "bag"}
def lr(f):
    return [x for z in (1, -1) for x in f(z, "l" if z > 0 else "r")]
def tub_band(profile, t):
    """closed polygon of the wall strip along an open polyline (the U of a basket tub): offset inward by t."""
    import math as m
    out = []; inn = []
    P = profile
    for i, p in enumerate(P):
        a = P[i - 1] if i > 0 else p; b = P[i + 1] if i < len(P) - 1 else p
        dx, dy = b[0] - a[0], b[1] - a[1]; L = m.hypot(dx, dy) or 1; nx, ny = -dy / L, dx / L
        inn.append([p[0] + nx * t, p[1] + ny * t])
    return P + inn[::-1]
def frame_parts(n):
    """returns list of (kind, args...) body solids on bone frame + wheels, region by name."""
    b = []  # (solid, region)
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[n]
    rt = {50: 0.013, 100: 0.013, 200: 0.013, 500: 0.0065}[n]
    sl = n >= 500
    def tube(id, p0, p1, r=rt, reg="frame", tone=0.4, mat=ALU, role="body", slender=None):
        L = math.dist(p0, p1); s = (L >= 20 * 2 * max(r, rt) * 1.0 and sl) if slender is None else slender
        b.append((cap(id, "frame", mat, p0, p1, r, role=role, tone=tone, **({"slender": True} if s else {})), reg))
    for z, s in ((0.27, "l"), (-0.27, "r")):
        tube(f"rear-upright-{s}", [0.0205, 0.12, z], [0.0, 0.84, z])
        tube(f"front-leg-{s}", [0.462, 0.505, z], [0.5, 0.25, z] if n < 500 else [0.52, 0.12, z])
        tube(f"lower-rail-{s}", [0.015, 0.26, z], [0.5, 0.24, z])
        if n <= 100:
            tube(f"seat-rail-{s}", [0.01, 0.495, z * 0.7037], [0.48, 0.495, z * 0.7037], r=0.017 if n == 100 else 0.018)
            tube(f"caster-fork-{s}", [0.5, 0.3, z], [0.52, 0.1, z], r=0.01 if n == 50 else 0.0085)
        if n == 50:
            tube(f"front-brace-{s}", [0.3, 0.3, z * 0.63], [0.479, 0.5, z * 0.63], r=0.01)
            tube(f"backrest-arm-{s}", [0.012, 0.655, z * 0.74], [0.01, 0.655, z], r=0.008)
            tube(f"brake-lever-{s}", [0.07, 0.84, z * 0.89], [-0.02, 0.8, z * 0.89], r=0.0055, tone=0.15)
            b.append((cap(f"brake-cable-{s}", "frame", "rubber-grip", [0.06, 0.8, z * 0.96], [0.035, 0.15, z * 0.96], 0.0025, role="detail", tone=0.1, slender=True), "frame"))
    tube("handlebar", [-0.005, 0.8525, -0.29], [-0.005, 0.8525, 0.29], r=0.013 if n < 500 else rt)
    tube("front-bar", [0.48, 0.5, -0.27], [0.48, 0.5, 0.27])
    if n >= 200: tube("lower-bar", [0.5, 0.24, -0.27], [0.5, 0.24, 0.27])
    # grips
    for z, s in ((0.29, "l"), (-0.29, "r")):
        if n == 50: b.append((lofts(f"grip-{s}", "frame", GRIP, [(-0.14, 0.012, 0.012, 0, 0), (-0.13, 0.0175, 0.0175), (-0.07, 0.0185, 0.0185), (-0.01, 0.0165, 0.0165), (0.0, 0.0125, 0.0125)], tone=0.15, e=2.0), "grips"))
        else: b.append((cap(f"grip-{s}", "frame", GRIP, [-0.14, 0.848, z], [0.0, 0.848, z], 0.017 if n < 500 else 0.0385, tone=0.15), "grips"))
    # fix grip loft placement (lofts along u at v=0.848, w=z)
    out = []
    for sol, reg in b:
        if sol["id"] in ("grip-l", "grip-r") and sol["kind"] == "loft":
            z = 0.29 if sol["id"] == "grip-l" else -0.29
            for q in sol["sections"]: q["at"] = [q["at"][0], 0.848, z]
        out.append((sol, reg))
    return out
def seat_etc(n):
    b = []
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[n]
    th = {50: 0.04, 100: 0.04, 200: 0.04, 500: 0.075}[n]
    b.append((lofts("seat", "frame", NYL, [(0.031, 0.19, th / 2), (0.05, 0.2, th / 2), (0.45, 0.2, th / 2), (0.475, 0.19, th / 2)], tone=0.35, e=4.0, **{}), "seat-basket"))
    for q in b[-1][0]["sections"]: q["at"] = [q["at"][0], 0.52, 0.0]
    if n <= 100:
        s = lofts("backrest", "frame", NYL, [(0.0, 0, 0)], tone=0.35); s["kind"] = "loft"
        s["sections"] = [{"at": [0.014, 0.655, w], "rx": 0.026, "ry": 0.075, "exponent": 3.0} for w in (-0.2, -0.19, 0.0, 0.19, 0.2)]
        s["sections"][0]["ry"] = s["sections"][-1]["ry"] = 0.07
        s["sections"][0]["rx"] = s["sections"][-1]["rx"] = 0.024
        b.append((s, "seat-basket"))
    return b
def basket(n):
    """tub: walls from the elevation outline; U-strip (floor, front, back) + two side sheets; wire rim slender."""
    t = {50: 0.0076, 100: 0.0152, 200: 0.031, 500: 0.076}[n]
    prof = [[0.10, 0.475], [0.44, 0.475], [0.4477, 0.4205], [0.4497, 0.3529], [0.4456, 0.3203], [0.4362, 0.2932], [0.42, 0.275], [0.394, 0.2641], [0.3581, 0.2563], [0.3157, 0.2516], [0.27, 0.25], [0.2025, 0.2535], [0.146, 0.2641], [0.1136, 0.2932], [0.1, 0.3203], [0.0944, 0.3529], [0.0924, 0.4205], [0.10, 0.475]]
    if n == 500: prof = [[0.08, 0.5], [0.44, 0.5], [0.44, 0.27], [0.08, 0.27]]
    elif n == 200: prof = [[0.10, 0.52], [0.44, 0.52], [0.44, 0.30], [0.40, 0.27], [0.14, 0.27], [0.10, 0.30], [0.10, 0.52]]
    elif n == 100: prof = [p for i, p in enumerate(prof) if i % 2 == 0] + [prof[-1]]
    hw = 0.17
    closed = prof[:-1] if prof[0] == prof[-1] else prof
    U = closed[1:] if n in (50, 100) else closed[1:] + [closed[0]]
    if n == 500: U = [closed[1], closed[2], closed[3]]
    strip = tub_band(U, t)
    s1 = extrude("basket-tub", "frame", NYL, "xy", strip, -hw, hw, tone=0.6)
    sides = [extrude(f"basket-side-{k}", "frame", NYL, "xy", closed, w0, w1, tone=0.6) for k, (w0, w1) in (("l", (hw - t, hw)), ("r", (-hw, -hw + t)))]
    out = [(s1, "seat-basket")] + [(x, "seat-basket") for x in sides]
    if n == 50:
        top = prof[0][1]
        for i, (p0, p1) in enumerate((([0.10, top, -hw], [0.44, top, -hw]), ([0.10, top, hw], [0.44, top, hw]), ([0.10, top, -hw], [0.10, top, hw]), ([0.44, top, -hw], [0.44, top, hw])), 1):
            out.append((cap(f"basket-wire-{i}", "frame", ALU, p0, p1, 0.0025, role="detail", tone=0.7, slender=True), "seat-basket"))
    return out
def bag(n):
    ry = {50: 0.0935, 100: 0.0935, 200: 0.0745, 500: 0.0745}[n]; cy = {50: 0.4905, 100: 0.4885, 200: 0.5075, 500: 0.5075}[n]
    x0, x1 = {50: (0.132, 0.421), 100: (0.13, 0.421), 200: (0.132, 0.416), 500: (0.08, 0.44)}[n]; hw = {50: 0.12, 100: 0.12, 200: 0.12, 500: 0.14}[n]
    xs = [x0, x0 + 0.04 * (x1 - x0), 0.5 * (x0 + x1), x1 - 0.04 * (x1 - x0), x1]
    k = [0.55, 0.97, 1.0, 0.97, 0.55]
    s = lofts("bag", "frame", TOTE, [(x, hw * kk, ry * kk) for x, kk in zip(xs, k)], tone=0.78, e=3.0)
    for q in s["sections"]: q["at"] = [q["at"][0], cy, 0.0]
    out = [(s, "bag")]
    if n == 50:
        out.append((cap("bag-handle-1", "frame", TOTE, [0.2, 0.575, 0.0], [0.26, 0.675, 0.0], 0.006, role="detail", tone=0.7), "bag"))
        out.append((cap("bag-handle-2", "frame", TOTE, [0.26, 0.675, 0.0], [0.32, 0.575, 0.0], 0.006, role="detail", tone=0.7), "bag"))
    return out
def wheels(n):
    out = []
    for w, bone in (("rear", "wheel-rear"), ("front", "wheel-front")):
        for z, s in ((0.29, "l"), (-0.29, "r")):
            if n == 50:
                tr = [[-0.02, 0.092], [-0.0195, 0.097], [-0.012, 0.1], [0.012, 0.1], [0.0195, 0.097], [0.02, 0.092]]
                out.append((revolve(f"tyre-{w}-{s}", bone, TYRE, [[a, r] for a, r in tr], "w", at=[0, 0, z], tone=0.12), "tyres"))
                out.append((revolve(f"rim-{w}-{s}", bone, ALU, [[-0.021, 0.0], [-0.021, 0.0925], [0.021, 0.0925], [0.021, 0.0]], "w", at=[0, 0, z], tone=0.8), "frame"))
                out.append((ell(f"hub-{w}-{s}", bone, ALU, [0, 0, z + (0.015 if z > 0 else -0.015)], [0.016, 0.016, 0.0075], tone=0.3), "frame"))
            elif n == 100:
                out.append((revolve(f"tyre-{w}-{s}", bone, TYRE, [[-0.02, 0.085], [-0.0185, 0.097], [-0.012, 0.1], [0.012, 0.1], [0.0185, 0.097], [0.02, 0.085]], "w", at=[0, 0, z], tone=0.12), "tyres"))
                out.append((revolve(f"rim-{w}-{s}", bone, ALU, [[-0.021, 0.0], [-0.021, 0.0865], [0.021, 0.0865], [0.021, 0.0]], "w", at=[0, 0, z], tone=0.8), "frame"))
            else:
                wk = 1.0 if n == 200 else 0.0385 / 0.02
                zz = z if n == 200 else (0.275 if z > 0 else -0.275)
                out.append((revolve(f"wheel-{w}-{s}", bone, TYRE, [[-0.02 * wk, 0.0], [-0.02 * wk, 0.092], [-0.0155 * wk, 0.098], [-0.01 * wk, 0.1], [0.01 * wk, 0.1], [0.0155 * wk, 0.098], [0.02 * wk, 0.092], [0.02 * wk, 0.0]], "w", at=[0, 0, zz], tone=0.12), "tyres"))
    return out
def props(n):
    parts = frame_parts(n) + seat_etc(n) + basket(n) + bag(n) + wheels(n)
    body = [p for p, _ in parts]; reg = {p["id"]: r for p, r in parts}
    thermal = [th(p["id"], reg[p["id"]]) for p in body if p.get("role", "body") != "x"]
    st = []
    rm = {50: 0.0085, 100: 0.0085, 200: 0.0125, 500: 0.0065}[n]
    members = [("rear-upright-l", [0.0205, 0.12, 0.27], [0.0, 0.84, 0.27]), ("rear-upright-r", [0.0205, 0.12, -0.27], [0.0, 0.84, -0.27]), ("lower-rail-l", [0.015, 0.26, 0.27], [0.5, 0.24, 0.27]), ("lower-rail-r", [0.015, 0.26, -0.27], [0.5, 0.24, -0.27]), ("handlebar", [-0.005, 0.8525, -0.29], [-0.005, 0.8525, 0.29]), ("front-leg-l", [0.462, 0.505, 0.27], [0.5, 0.25, 0.27]), ("front-leg-r", [0.462, 0.505, -0.27], [0.5, 0.25, -0.27])]
    if n <= 100: members += [("seat-rail-l", [0.01, 0.495, 0.19], [0.48, 0.495, 0.19]), ("seat-rail-r", [0.01, 0.495, -0.19], [0.48, 0.495, -0.19])]
    if n == 500: members = members[:2] + members[4:5]
    for mid, p0, p1 in members:
        st.append({**cap(f"member-frame-{mid}", "frame", ALU, p0, p1, rm, tone=0.5), "member": "frame", "part": "member"})
        if n >= 200 and math.dist(p0, p1) >= 40 * rm: st[-1]["slender"] = True
    for w in ("rear", "front"):
        st.append({**cap(f"member-wheel-{w}", f"wheel-{w}", ALU, [0, 0, -0.29], [0, 0, 0.29], 0.0095 if n < 500 else 0.0065, tone=0.5, **({"slender": True} if n >= 100 else {})), "member": f"wheel-{w}", "part": "member"})
    sr = {50: 0.0045, 100: 0.0085, 200: 0.016, 500: 0.0385}[n]
    st.append({**ell("support-wheel-rear-contact", "frame", ALU, [0.0, sr, 0.29], [0.03 if n < 500 else 0.0385, sr, 0.03 if n < 500 else 0.0385], role="detail", tone=0.2), "member": "frame", "part": "support", "anchor": "wheel-rear-contact"})
    st.append({**ell("support-wheel-front-contact", "frame", ALU, [0.52, sr, 0.29], [0.03 if n < 500 else 0.0385, sr, 0.03 if n < 500 else 0.0385], role="detail", tone=0.2), "member": "frame", "part": "support", "anchor": "wheel-front-contact"})
    st.append({**ell("mass-frame", "frame", ALU, [0.26, 0.4, 0.0], [0.0544 if n < 500 else 0.0544] * 3, role="detail", tone=0.6), "member": "frame", "part": "mass"})
    st.append({**ell("mass-wheel-rear", "wheel-rear", ALU, [0, 0, 0.29 if n < 500 else 0.275], [0.04 if n < 500 else 0.0385, 0.04 if n < 500 else 0.0385, 0.015 if n < 500 else 0.0385], role="detail", tone=0.6), "member": "wheel-rear", "part": "mass"})
    return body, thermal, st
D = {50: ("Rollator as a real object: red aluminium tube frame (rear uprights, lower rails, front legs, seat rails, braces, caster forks, handlebar with moulded grips, brake levers and slender cables), nylon seat pad and back strap, basket as a hollow tub (U-strip, two side sheets, slender rim wire), cotton tote bag with handles, four wheels as tyre ring, rim and hub; thermal solids are the same solids by region, structure members are tube cores, supports at both contact patches.", "Rollator als echter Gegenstand: rot lackiertes Aluminium-Rohrgestell (hintere Rahmen, untere Holme, vordere Beine, Sitzholme, Streben, Lenkrollen-Gabeln, Lenker mit angeformten Griffen, Bremshebel und schlanke Seilzüge), Nylon-Sitzpolster und Rückengurt, Korb als hohle Wanne (U-Streifen, zwei Seitenbleche, schlanker Randdraht), Baumwoll-Tasche mit Henkeln, vier Räder aus Reifenring, Felge und Nabe; Thermik-Solids sind dieselben Solids je Region, Tragelemente sind Rohrkerne, Auflager an beiden Aufstandsflächen."),
     100: ("Rollator at 1:100: tubes as capsules, seat pad, back strap, basket tub with 15 mm walls, tote, wheels as tyre ring plus rim; brake levers, cables, braces and hubs merged away.", "Rollator 1:100: Rohre als Kapseln, Sitzpolster, Rückengurt, Korbwanne mit 15 mm Wand, Tasche, Räder als Reifenring mit Felge; Bremshebel, Seilzüge, Streben und Naben entfallen."),
     200: ("Rollator at 1:200: long tubes slender at true size, seat pad, basket tub, tote, wheels as solid discs as the drawing fills them.", "Rollator 1:200: lange Rohre schlank in wahrer Größe, Sitzpolster, Korbwanne, Tasche, Räder als Vollscheiben wie gezeichnet."),
     500: ("Rollator at 1:500: slender tubes at true size for uprights, rails and handlebar, seat, basket and tote as blocks at the feature size, wheels as discs.", "Rollator 1:500: schlanke Rohre in wahrer Größe für Rahmen, Holme und Lenker, Sitz, Korb und Tasche als Blöcke in Mindestgröße, Räder als Scheiben.")}
def post(d):
    v = d["volumes"]["500"]
    v["body"] = [x for x in v["body"] if x["id"] != "forearm-r-coat"]; v["thermal"] = [x for x in v["thermal"] if x["id"] != "th-forearm-r-coat"]
    if not any(x["bone"] == "forearm-r" for x in v["body"]):
        h = lofts("forearm-r-coat", "forearm-r", "loden-wool", [(0.0, 0.041, 0.041), (0.247, 0.041, 0.041)], tone=0.45)
        v["body"].append(h); v["thermal"].append(th("forearm-r-coat", "coat"))
if __name__ == "__main__":
    pass
