from lib2 import *
from d_oak import masses, JIT, mblob

FR = ["l1","l2","l3","l4","r1","r2","r3","r4"]

def lens(p0, p1, w0, w1, wm, tpos=0.5, serr=0, serrdepth=0.0):
    dx, dy = p1[0]-p0[0], p1[1]-p0[1]; L = math.hypot(dx, dy); ux, uy = dx/L, dy/L; nx, ny = -uy, ux
    def at(t, w, s): return (p0[0]+dx*t + nx*w*s, p0[1]+dy*t + ny*w*s)
    top = []; bot = []
    N = max(serr, 1)
    top = [at(0, w0, 1)]; bot = [at(0, w0, -1)]
    if serr:
        for i in range(1, serr+1):
            t = i/(serr+1); w = (w0 + (wm-w0)*min(1, t/tpos)) if t < tpos else (wm + (w1-wm)*(t-tpos)/(1-tpos))
            tt = t - 0.5/(serr+1)
            top.append(at(tt, w + serrdepth, 1)); top.append(at(t, w, 1))
            bot.append(at(tt, w + serrdepth, -1)); bot.append(at(t, w, -1))
    else:
        top.append(at(tpos, wm, 1)); bot.append(at(tpos, wm, -1))
    top.append(at(1, w1, 1)); bot.append(at(1, w1, -1))
    return poly(top + bot[::-1])

def design(R, O):
    res = {}
    c = (-0.05, 9.0)
    tips = [(-3.49,8.73),(-3.25,10.5),(-2.17,11.86),(-0.97,12.47)]
    tips = tips + [(-x + 0.0 - 0.1, y) for x, y in tips][::-1]
    pts = [(-0.35,8.9),(-1.9,8.93)]
    for i, t in enumerate(tips):
        pts.append(t)
        if i < len(tips)-1:
            n = tips[i+1]; a1 = math.atan2(t[1]-c[1], t[0]-c[0]); a2 = math.atan2(n[1]-c[1], n[0]-c[0]); dd = (a2-a1+math.pi) % (2*math.pi) - math.pi; ang = a1 + dd/2
            if i == 3: ang = math.pi/2
            pts.append((c[0] + 1.7*math.cos(ang) + (0.35 if i == 3 else 0), c[1] + 1.7*math.sin(ang)))
    pts += [(1.8,8.93),(0.25,8.9)]
    crown = fit_to(poly(pts), (-3.48, 3.39, 8.72, 12.48))
    bunches = masses(R, O, ("fruit",))
    r5 = [shape(R, "bole", "stem-1", "body", chain([(0,0),(0,0.45),(0,3.3),(-0.05,6.0),(-0.05,8.4),(-0.05,9.0)], [0.6,0.27,0.26,0.25,0.23,0.2]), region="stem-bark", tone=0.35, z=1),
          shape(R, "crown", "crown", "body", crown, region="fronds", tone=0.55, z=3)]
    for m in bunches:
        r5.append(shape(R, m["id"], m["bone"], "body", mblob(m, 6, 0.1, 0.3, JIT["a"]), region="fruit", tone=0.3, z=2))
    res["500"] = r5
    def stem(fine):
        o = [shape(R, "bole", "stem-1", "body", chain([(0,0),(0,0.45),(0,3.3),(-0.05,6.0),(-0.05,8.4),(-0.05,9.0)], [0.6,0.27,0.26,0.25,0.23,0.2]), region="stem-bark", tone=0.35, z=1)]
        for m in bunches:
            o.append(shape(R, m["id"], m["bone"], "body", mblob(m, 7 if not fine else 10, 0.1, 0.3, JIT["a"]), region="fruit", tone=0.3, z=2))
        return o
    r2 = stem(False)
    for y in (2.4, 4.4, 6.4):
        r2.append(shape(R, f"ring-{y}", "stem-1" if y < 3.3 else "stem-2", "detail", stroke([(-0.27, y), (0, y-0.12), (0.27, y)]), filled=False, tone=0.2, z=2))
    r1 = stem(True)
    for k, y in enumerate((0.9, 1.8, 2.7, 3.6, 4.5, 5.4, 6.3, 7.2)):
        b = "stem-1" if y < 3.3 else ("stem-2" if y < 6.0 else "stem-3")
        r1.append(shape(R, f"leafbase-{k+1}", b, "texture", stroke([(-0.26, y), (-0.02, y-0.18), (0.24, y)]), filled=False, tone=0.2, z=2))
    for k, side in enumerate(("l", "r")):
        for j, f_ in enumerate((1, 2, 3, 4)):
            a = f"fr-{side}{f_}a"; b = f"fr-{side}{f_}b"
            sa, ea, _, _ = woody(R, O, a); sb, eb, _, _ = woody(R, O, b)
            ea2 = mid(sb, eb, 0.04)
            r2.append(shape(R, f"{a}-blade", a, "body", lens(sa, ea2, 0.06, 0.3, 0.42, 0.55), region="fronds", tone=0.55, z=3))
            r2.append(shape(R, f"{b}-blade", b, "body", lens(sb, eb, 0.3, 0.0, 0.36, 0.35), region="fronds", tone=0.55, z=3))
            r2.append(shape(R, f"{a}-rachis", a, "detail", stroke([sa, ea]), filled=False, tone=0.2, z=4))
            r1.append(shape(R, f"{a}-blade", a, "body", lens(sa, ea2, 0.06, 0.3, 0.42, 0.55, 6, 0.1), region="fronds", tone=0.55, z=3))
            r1.append(shape(R, f"{b}-blade", b, "body", lens(sb, eb, 0.3, 0.0, 0.36, 0.35, 6, 0.1), region="fronds", tone=0.55, z=3))
            r1.append(shape(R, f"{a}-rachis", a, "detail", stroke([sa, ea]), filled=False, tone=0.2, z=4))
            r1.append(shape(R, f"{b}-rachis", b, "detail", stroke([sb, eb]), filled=False, tone=0.2, z=4))
    res["200"] = r2; res["100"] = r1
    return res
