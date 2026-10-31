from lib2 import *
from d_oak import masses, arcs, JIT, mblob

def design(R, O):
    ms = masses(R, O, ("leaf-crown",))
    for m in ms:
        if m['id'] == 'fol-top': m['box'] = (m['box'][0], m['box'][1], m['box'][2], 26.0)
    res = {}
    L = [(-0.5,2.5),(-1.3,3.6),(-1.9,5.2),(-2.05,7.0),(-1.6,8.2),(-2.1,9.8),(-2.05,12.2),(-1.6,13.2),(-2.0,15.0),(-2.05,17.0),(-1.6,18.2),(-2.0,19.8),(-1.9,21.6),(-1.35,23.2),(-0.8,24.7),(-0.2,26.05)]
    Rr = [(0.5,25.0),(1.1,24.1),(1.8,22.3),(1.7,20.3),(2.05,18.5),(1.6,17.0),(2.0,15.5),(1.55,14.0),(2.0,12.0),(1.6,10.5),(2.0,8.3),(1.5,6.7),(1.8,5.0),(1.2,3.4),(0.5,2.6)]
    crown = fit_x(lobes(L + Rr, 0.13), -2.11, 2.05)
    res["500"] = [
        shape(R, "bole", "trunk-1", "body", chain([(0,0),(0,0.3),(0,3.0)], [0.62,0.3,0.28]), region="stem-bark", tone=0.35, z=1),
        shape(R, "crown", "trunk-2", "body", crown, regions=["leaf-crown", "branch-bark", "stem-bark"], tone=0.65, z=2)]
    stem = lambda z: shape(R, "bole", "trunk-1", "body", chain([(0,0),(0,0.3),(0,6.3),(-0.1,12.3),(-0.1,18.3),(-0.16,24.6)], [0.62,0.3,0.27,0.22,0.17,0.08]), region="stem-bark", tone=0.35, z=z)
    brs = [("br-l1",),("br-r1",),("br-l2",),("br-r2",),("br-l3",),("br-r3",),("br-l4",),("br-r4",)]
    def woodset(fine):
        o = [stem(2)]
        for (b,) in brs:
            o.append(shape(R, b, b, "body", limb_path(R, O, [b, "fol-" + b[3:]] if False else [b]), region="branch-bark", tone=0.4, z=1))
        return o
    r2 = woodset(False)
    for i, m in enumerate(ms):
        d = mblob(m, 8, 0.08, 0.3, JIT["a"], 0.25*i)
        r2.append(shape(R, m["id"], m["bone"], "body", d, region=m["region"], tone=0.6, z=5+i*0.1))
    res["200"] = r2
    r1 = woodset(True)
    for i, m in enumerate(ms):
        d = mblob(m, 13, 0.06, 0.34, JIT["b"], 0.2*i)
        r1.append(shape(R, m["id"], m["bone"], "body", d, region=m["region"], tone=0.6, z=5+i*0.1))
        r1 += arcs(R, m, "100")
    r1.append(shape(R, "bole-furrow-1", "trunk-1", "texture", stroke([(0.1,0.6),(0.0,1.5),(0.1,2.5)]), filled=False, tone=0.2, z=3))
    r1.append(shape(R, "bole-furrow-2", "trunk-1", "texture", stroke([(-0.1,0.9),(-0.05,1.9),(-0.1,2.6)]), filled=False, tone=0.2, z=3))
    res["100"] = r1
    return res
