from lib2 import *
from d_oak import masses, arcs, JIT, mblob

def curtain_path(cx, top, bot, hw, teeth):
    hr = hw*0.94
    right = smooth([(cx+hw*0.55, top), (cx+hw*0.95, top-0.9), (cx+hr, (top+bot)/2+1.0), (cx+hw*0.9, bot+1.9)], closed=False)
    d = f"M{f(cx-hw*0.55)} {f(top)}L" + right[1:]
    span = 2*hw*0.9
    for i in range(teeth):
        xr = cx + hw*0.9 - span*i/teeth; xm = xr - span/teeth*0.5; xl = xr - span/teeth
        d += f"L{f(xm)} {f(bot)}L{f(xl)} {f(bot+1.9)}"
    left = smooth([(cx-hw*0.9, bot+1.9), (cx-hr, (top+bot)/2+1.0), (cx-hw*0.95, top-0.9), (cx-hw*0.55, top)], closed=False)
    return d + "L" + left[1:] + "Z"

def design(R, O):
    ms = masses(R, O, ("leaf-curtain",))
    res = {}
    half = [(2.2,0.5),(2.9,0.1),(3.65,0.5),(4.4,0.1),(5.15,0.5),(5.9,0.1),(6.65,0.5),(7.4,0.1),(8.2,0.5),(8.3,1.5),(8.4,3.5),(8.2,6.0),(8.3,8.2),(7.8,10.2),(6.5,11.0),(5.0,11.5),(3.6,11.2),(2.2,10.3),(0.9,9.7)]
    Lh = [(-1.5,0.0)] + [(-x, y) for x, y in half]
    Rh = [(x*0.965, y) for x, y in half][::-1] + [(1.5,0.0)]
    arch = [(1.5,3.0),(0.0,3.9),(-1.5,3.0)]
    crown = fit_y(fit_x(lobes(Lh + Rh + arch, 0.16), -8.26, 7.97), 0.0, 11.05)
    res["500"] = [
        shape(R, "bole", "trunk-1", "body", chain([(0,0),(0,0.5),(0,2.2),(-0.09,3.5)], [1.3,0.5,0.5,0.44]), region="trunk-bark", tone=0.35, z=1),
        shape(R, "crown", "limb-w-a", "body", crown, regions=["leaf-curtain", "limb-bark"], tone=0.6, z=2)]
    def wood():
        o = [shape(R, "bole", "trunk-1", "body", chain([(0,0),(0,0.5),(0,2.2),(-0.09,3.5)], [1.3,0.5,0.5,0.44]), region="trunk-bark", tone=0.35, z=2)]
        for s in ("w", "e"):
            o.append(shape(R, f"limb-{s}", f"limb-{s}-a", "body", limb_path(R, O, [f"limb-{s}-a", f"limb-{s}-b", f"limb-{s}-c"]), region="limb-bark", tone=0.4, z=1))
        return o
    r2 = wood()
    r1 = wood()
    for i, m in enumerate(ms):
        if m["id"].startswith("cap"):
            r2.append(shape(R, m["id"], m["bone"], "body", mblob(m, 9, 0.07, 0.32, JIT["a"], 0.3*i), region=m["region"], tone=0.55, z=6))
            r1.append(shape(R, m["id"], m["bone"], "body", mblob(m, 13, 0.06, 0.34, JIT["b"], 0.2*i), region=m["region"], tone=0.55, z=6))
            r1 += arcs(R, m, "100")
        else:
            bx = m["box"]; cx = (bx[0]+bx[1])/2; hw = (bx[1]-bx[0])/2
            top = bx[3] if False else min(bx[3], 9.9); bot = max(bx[2], 0.0)
            r2.append(shape(R, m["id"], m["bone"], "body", fit_to(curtain_path(cx, top, bot, hw*0.96, 3), (bx[0], bx[1], bot, top)), region=m["region"], tone=0.5, z=3+0.1*i))
            r1.append(shape(R, m["id"], m["bone"], "body", fit_to(curtain_path(cx, top, bot, hw*0.96, 4), (bx[0], bx[1], bot, top)), region=m["region"], tone=0.5, z=3+0.1*i))
            for k, u in enumerate((-0.45, 0.0, 0.45)):
                x = cx + u*hw
                r1.append(shape(R, f"{m['id']}-strand{k+1}", m["bone"], "detail", stroke([(x, top-1.0), (x+0.08, (top+bot)/2), (x-0.05, bot+1.6)]), filled=False, tone=0.2, z=9))
    res["200"] = r2; res["100"] = r1
    return res
