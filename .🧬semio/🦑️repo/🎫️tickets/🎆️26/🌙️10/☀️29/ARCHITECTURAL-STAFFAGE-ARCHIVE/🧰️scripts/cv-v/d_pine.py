from lib2 import *
from d_oak import masses, arcs, JIT, mblob

def design(R, O):
    ms = masses(R, O, ("needles-sun", "needles-shade"))
    res = {}
    C = [(-1.6,12.8),(-4.6,12.5),(-6.3,13.4),(-6.2,14.5),(-3.2,15.2),(-3.5,15.9),(-6.4,16.2),(-6.5,17.6),(-5.0,18.4),(-3.8,18.7),(-3.8,19.6),(-3.0,20.4),(-2.2,20.8),(-1.9,22.0),(0.2,22.3),(1.8,21.9),(2.3,21.0),(4.0,21.0),(4.6,20.2),(4.1,19.3),(6.8,19.3),(6.9,17.5),(5.6,16.6),(4.4,16.3),(4.6,15.9),(6.4,15.5),(6.5,14.0),(4.8,13.3),(1.6,13.0)]
    crown = fit_x(fit_y(lobes(C, 0.12), 12.4, 22.2), -5.95, 6.2)
    res["500"] = [
        shape(R, "bole-lower", "trunk-1", "body", chain([(0,0),(0,0.45),(0,7.4)], [0.9,0.3,0.28]), region="bark-lower", tone=0.15, z=1),
        shape(R, "bole-upper", "trunk-2", "body", chain([(0,7.4),(-0.1,13.4),(-0.07,17.6),(-0.09,20.4)], [0.27,0.22,0.17,0.1]), region="bark-upper", tone=0.4, z=1),
        shape(R, "crown", "trunk-3", "body", crown, regions=["needles-sun", "needles-shade", "limb-bark"], tone=0.65, z=2)]
    def wood(fine):
        o = [shape(R, "bole-lower", "trunk-1", "body", chain([(0,0),(0,0.45),(0,7.4)], [0.9,0.3,0.28]), region="bark-lower", tone=0.15, z=2),
             shape(R, "bole-upper", "trunk-2", "body", chain([(0,7.4),(-0.1,13.4),(-0.07,17.6),(-0.09,20.4)], [0.27,0.22,0.17,0.1]), region="bark-upper", tone=0.4, z=2)]
        for b in ("limb-w3","limb-e3","limb-w1","limb-e1","limb-w2","limb-e2"):
            o.append(shape(R, b, b, "body", limb_path(R, O, [b]), region="limb-bark", tone=0.4, z=1))
        if fine:
            for b in ("stub-w","stub-e"):
                o.append(shape(R, b, b, "body", limb_path(R, O, [b]), region="bark-upper", tone=0.4, z=1))
        return o
    r2 = wood(False)
    for i, m in enumerate(ms):
        r2.append(shape(R, m["id"], m["bone"], "body", mblob(m, 9, 0.09, 0.32, JIT["a"], 0.3*i), region=m["region"], tone=0.45 if m["region"] == "needles-shade" else 0.6, z=5+i*0.1))
    res["200"] = r2
    r1 = wood(True)
    for i, m in enumerate(ms):
        shade = m["region"] == "needles-shade"
        s = shape(R, m["id"], m["bone"], "body", mblob(m, 14, 0.07, 0.34, JIT["b"], 0.2*i), region=m["region"], tone=0.45 if shade else 0.6, z=5+i*0.1)
        if shade: s["hatch"] = "shade-lines"
        r1.append(s)
        for k, (u, v) in enumerate(((-0.45, -0.1), (0.1, 0.15), (0.45, -0.05))):
            x = m["cx"] + u*m["a"]; y = m["cy"] + v*m["b"]
            r1.append(shape(R, f"{m['id']}-needles{k+1}", m["bone"], "detail", f"M{f(x-0.7)} {f(y)}Q{f(x)} {f(y-0.25)} {f(x+0.7)} {f(y)}", filled=False, tone=0.2, z=9))
    for j, y in enumerate((0.9, 2.2, 3.6, 5.0, 6.4)):
        r1.append(shape(R, f"plate-{j+1}", "trunk-1", "texture", stroke([(-0.2, y), (-0.02, y+0.12), (0.2, y)], False), filled=False, tone=0.1, z=3))
    res["100"] = r1
    return res
