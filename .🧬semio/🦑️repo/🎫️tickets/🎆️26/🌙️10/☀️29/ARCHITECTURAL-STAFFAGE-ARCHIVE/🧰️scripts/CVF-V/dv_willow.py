from wl import *

TOP = {"w1": (-1.56, 7.75, 0.55), "w2": (-3.97, 9.85, 0.35), "w3": (-5.47, 9.88, 0.28), "w4": (-6.97, 9.90, 0.20)}
VAR = {"w1": (0.35, 1.05, 3), "w2": (0.0, 1.0, 3), "w3": (0.5, 0.96, 3), "w4": (0.15, 1.04, 4), "e1": (0.2, 0.98, 3), "e2": (0.55, 1.06, 3), "e3": (0.0, 1.0, 3), "e4": (0.4, 0.95, 4)}
HW = {"1": 1.0, "2": 1.1, "3": 0.9, "4": 0.95}

def curtain(cx, top, bot, hw, nl, notch, sway=0.0):
    t = top + 0.7
    pts = [(cx - hw*0.80, t), (cx - hw*0.97, t - 1.6), (cx - hw*1.0, (t + bot)/2 + 0.3), (cx - hw*0.93, bot + notch + 0.9)]
    bx = [cx - hw*0.90 + (1.8*hw)*i/nl for i in range(nl + 1)]
    low = [(bx[0], bot + notch)]
    for i in range(nl):
        low.append(((bx[i] + bx[i+1])/2, bot)); low.append((bx[i+1], bot + notch)) if i < nl - 1 else None
    low = low[:-1] if False else low
    right = [(cx + hw*0.93, bot + notch + 0.9), (cx + hw*1.0, (t + bot)/2 + 0.3), (cx + hw*0.97, t - 1.6), (cx + hw*0.80, t)]
    return pts, low, right

def curtain_path(cx, top, bot, hw, nl, notch, bulge=0.0):
    pts, low, right = curtain(cx, top, bot, hw, nl, notch)
    out = pts[:3] + [pts[3]] + low + right[::1]
    return smooth(out, closed=True)

def strand(cx, top, bot, dx, wob):
    return stroke([(cx + dx, top - 0.5), (cx + dx + wob, (top + bot)/2 + 0.8), (cx + dx*0.9 - wob*0.5, bot + 0.9)])

def dome(side, fine):
    s = 1 if side == "e" else -1
    x0, x1 = 0.55, 8.0
    top = 10.8
    notches = [(x0, 8.5), (x0 + 0.5, 9.9), (x0 + 1.9, 10.8), (x0 + 3.6, top - 0.2), (x0 + 5.3, 10.9), (x1 - 0.7, 10.1), (x1, 9.0), (x1 - 0.2, 8.15), (x1 - 2.1, 8.55), (x0 + 3.9, 8.35), (x0 + 1.8, 8.5)]
    pts = [(s*x, y) for x, y in notches]
    bul = [0.34, 0.34, 0.38, 0.38, 0.36, 0.34, 0.30, 0.22, 0.20, 0.22, 0.20]
    return lobes(pts, bul)

def design(R, A):
    res = {}
    bw = pieces(R, [(0, 0), (0, 0.5), (0, 2.2), (-0.09, 3.5)], [1.3, 0.52, 0.5, 0.44], ["root-plate", "trunk-1", "trunk-2"], ["root-flare", "trunk-1", "trunk-2"], "trunk-bark", z=2, tone=0.35, ext=0.5)
    def limbs(z):
        o = []
        for s, sg in (("w", -1), ("e", 1)):
            pts = [(-0.09, 3.5), (sg*1.56 if sg > 0 else -1.56, 7.75), (sg*3.97 if sg > 0 else -3.97, 9.85), (sg*6.97 if sg > 0 else -6.97, 9.90)]
            if sg > 0: pts[0] = (-0.09, 3.5); pts = [(0.0 if i == 0 else x, y) for i, (x, y) in enumerate(pts)]
            pts = [(-0.09, 3.5), (1.37 if sg > 0 else -1.56, 7.75), (3.79 if sg > 0 else -3.97, 9.85), (6.79 if sg > 0 else -6.97, 9.90)]
            o += pieces(R, [(-0.09, 3.5)] + pts[1:], [0.28, 0.2, 0.14, 0.06], [f"limb-{s}-a", f"limb-{s}-b", f"limb-{s}-c"], [f"limb-{s}-a", f"limb-{s}-b", f"limb-{s}-c"], "limb-bark", z=z, tone=0.4, ext=0.6, back=True)
        return o
    # ---- 1:200
    r2 = bw + limbs(1)
    for s, sg in (("w", -1), ("e", 1)):
        for k in "1234":
            cx0, top, bot = TOP["w" + k]; cx = sg*abs(cx0) if s == "e" else cx0
            if s == "e": cx = {"1": 1.37, "2": 3.79, "3": 5.29, "4": 6.79}[k]
            d = curtain_path(cx, top, bot + VAR[s+k][0], HW[k]*VAR[s+k][1], max(2, VAR[s+k][2]-1), 0.30)
            r2.append(shape(R, f"curt-{s}{k}", f"curt-{s}{k}", "body", d, region="leaf-curtain", tone=0.5, z=3 + 0.1*int(k) + (0.05 if s == "e" else 0)))
        r2.append(shape(R, f"cap-{s}", f"cap-{s}", "body", dome(s, False), region="leaf-curtain", tone=0.6, z=6))
    res["200"] = r2
    # ---- 1:100
    r1 = bw + limbs(1)
    for s in ("w", "e"):
        for k in "1234":
            cx = {"w": {"1": -1.56, "2": -3.97, "3": -5.47, "4": -6.97}, "e": {"1": 1.37, "2": 3.79, "3": 5.29, "4": 6.79}}[s][k]
            top, bot = TOP["w" + k][1], TOP["w" + k][2]
            d = curtain_path(cx, top, bot + VAR[s+k][0], HW[k]*VAR[s+k][1], VAR[s+k][2], 0.30)
            z = 3 + 0.1*int(k) + (0.05 if s == "e" else 0)
            r1.append(shape(R, f"curt-{s}{k}", f"curt-{s}{k}", "body", d, region="leaf-curtain", tone=0.5, z=z))
            hw = HW[k]
            for j, (dx, wob) in enumerate(((-0.45*hw, 0.12), (0.0, -0.10), (0.45*hw, 0.10))):
                r1.append(shape(R, f"curt-{s}{k}-strand{j+1}", f"curt-{s}{k}", "texture", strand(cx, top, bot, dx, wob), filled=False, tone=0.2, z=z + 0.01))
        r1.append(shape(R, f"cap-{s}", f"cap-{s}", "body", dome(s, True), region="leaf-curtain", tone=0.6, z=6))
        sg = 1 if s == "e" else -1
        for j, (x, y, ang) in enumerate(((2.6, 10.1, 0.12), (4.6, 10.6, -0.1), (6.2, 9.9, 0.1))):
            r1.append(shape(R, f"cap-{s}-arc{j+1}", f"cap-{s}", "detail", stroke([(sg*(x-0.9), y), (sg*x, y + 0.3 + ang), (sg*(x + 0.9), y - 0.1)]), filled=False, tone=0.3, z=7))
    r1.append(shape(R, "bark-1", "trunk-1", "texture", stroke([(-0.12, 0.9), (-0.02, 1.5), (-0.1, 2.1)]), filled=False, tone=0.2, z=2.5))
    r1.append(shape(R, "bark-2", "trunk-1", "texture", stroke([(0.14, 1.1), (0.2, 1.7), (0.12, 2.4)]), filled=False, tone=0.2, z=2.5))
    res["100"] = r1
    return res
