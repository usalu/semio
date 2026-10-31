from lib2 import *
from d_oak import masses, arcs, JIT, mblob

def design(R, O):
    ms = masses(R, O, ("leaf-crown",))
    res = {}
    FAN = [(-0.32,0),(0.32,0),(0.46,0.5),(0.9,2.0),(1.55,4.9),(1.66,5.7),(1.30,5.8),(0.5,2.4),(0.22,1.5),(0.2,3.5),(0.2,6.0),(0.2,8.2),(-0.5,8.2),(-0.4,6.0),(-0.3,3.5),(-0.25,1.5),(-0.5,2.4),(-1.3,5.5),(-1.7,5.7),(-1.65,4.9),(-0.95,2.0),(-0.45,0.5)]
    C = [(-4.5,5.6),(-3.8,4.6),(-2.3,4.45),(-0.9,5.2),(-0.75,5.8),(-0.5,7.4),(0.0,7.9),(0.5,7.4),(0.7,5.8),(1.0,4.8),(2.4,4.45),(3.9,4.6),(4.55,5.8),(4.1,7.0),(4.9,8.3),(4.65,9.8),(4.0,10.6),(4.1,11.6),(3.5,12.5),(2.6,13.0),(1.5,13.3),(0.0,13.0),(-1.5,13.05),(-2.8,12.5),(-4.0,11.5),(-4.2,10.3),(-4.6,9.5),(-4.7,8.2),(-4.0,7.2),(-4.6,6.2)]
    crown = fit_y(fit_x(lobes(C, 0.3), -4.59, 4.67), 4.4, 13.03)
    res["500"] = [
        shape(R, "stems", "stem-c-1", "body", poly(FAN), region="stem-bark", tone=0.15, z=1),
        shape(R, "crown", "stem-c-2", "body", crown, regions=["leaf-crown", "branch-bark"], tone=0.6, z=2)]
    def stems(fine):
        o = []
        for s, a, b in (("l", "stem-l-1", "stem-l-2"), ("c", "stem-c-1", "stem-c-2"), ("r", "stem-r-1", "stem-r-2")):
            o.append(shape(R, f"stem-{s}", a, "body", limb_path(R, O, [a, b]), region="stem-bark", tone=0.15, z=2))
        for b in ("br-l-a","br-l-b","br-c-a","br-c-b","br-r-a","br-r-b"):
            o.append(shape(R, b, b, "body", limb_path(R, O, [b]), region="branch-bark", tone=0.3, z=1))
        o.append(shape(R, "stool", "root-plate", "body", chain([(0,0),(0,0.35)], [0.5,0.3]), region="stem-bark", tone=0.1, z=2))
        return o
    r2 = stems(False)
    for i, m in enumerate(ms):
        r2.append(shape(R, m["id"], m["bone"], "body", mblob(m, 9, 0.07, 0.32, JIT["a"], 0.3*i), region=m["region"], tone=0.6, z=5+i*0.1))
    res["200"] = r2
    r1 = stems(True)
    for i, m in enumerate(ms):
        r1.append(shape(R, m["id"], m["bone"], "body", mblob(m, 13, 0.06, 0.34, JIT["b"], 0.2*i), region=m["region"], tone=0.6, z=5+i*0.1))
        for k, u in enumerate((-0.5, 0.0, 0.5)):
            x = m["cx"] + u*m["a"]; y = m["cy"] - m["b"]*0.75
            r1.append(shape(R, f"{m['id']}-twig{k+1}", m["bone"], "detail", stroke([(x, y), (x+0.1*(k-1), y-0.6), (x+0.05*(k-1), y-1.3)]), filled=False, tone=0.2, z=9))
    for s, a in (("l","stem-l-1"),("c","stem-c-1"),("r","stem-r-1")):
        s0, e0, _, _ = woody(R, O, a)
        for j, t in enumerate((0.3, 0.55, 0.8)):
            p = mid(s0, e0, t)
            r1.append(shape(R, f"lenticel-{s}-{j+1}", a, "texture", stroke([(p[0]-0.06, p[1]), (p[0]+0.06, p[1])], False), filled=False, tone=0.1, z=3))
    res["100"] = r1
    return res
