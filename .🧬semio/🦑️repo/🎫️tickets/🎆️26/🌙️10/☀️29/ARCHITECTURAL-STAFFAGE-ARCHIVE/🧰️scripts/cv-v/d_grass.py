from lib2 import *
from d_palm import lens
from d_oak import masses, mblob, JIT

def design(R, O):
    res = {}
    pts = [(-1.07,0.1),(-1.0,0.45),(-0.85,0.85),(-0.78,1.3),(-0.74,1.5),(-0.55,1.35),(-0.28,1.7),(0.0,1.5),(0.28,1.7),(0.55,1.35),(0.73,1.5),(0.78,1.3),(0.85,0.85),(1.0,0.45),(1.07,0.1),(0.6,0.06),(0.0,0.02),(-0.6,0.06)]
    out = fit_to(lobes(pts, [0.2]*4 + [0.0]*6 + [0.2]*4 + [0.05]*4), (-1.07, 1.07, 0.0, 1.75))
    res["500"] = [shape(R, "tuft", "root-plate", "body", out, regions=["blades", "plumes", "culms"], tone=0.6, z=1)]
    sides = ("L", "R")
    def blades(fine):
        o = [shape(R, "tuft", "root-plate", "body", poly([(-0.22,0.0),(0.22,0.0),(0.2,0.2),(-0.2,0.2)]), region="culms", tone=0.2, z=2)]
        for s in sides:
            for n in (1, 2, 3):
                a = f"bl-{s}{n}a"; b = f"bl-{s}{n}b"
                sa, ea, _, _ = woody(R, O, a); sb, eb, _, _ = woody(R, O, b)
                for k, off in enumerate(((0.0,) if not fine else (-0.05, 0.05))):
                    o.append(shape(R, f"{a}-{k}", a, "body", lens((sa[0], sa[1]+off), (mid(sb, eb, 0.03)[0], mid(sb, eb, 0.03)[1]+off), 0.02, 0.07, 0.09, 0.5), region="blades", tone=0.5, z=3+n*0.1))
                    o.append(shape(R, f"{b}-{k}", b, "body", lens((sb[0], sb[1]+off), (eb[0]+(eb[0]-sb[0])*0.09, eb[1]+off+(eb[1]-sb[1])*0.09), 0.07, 0.0, 0.08, 0.35), region="blades", tone=0.5, z=3+n*0.1))
        for m in masses(R, O, ("plumes",)):
            if m['id'] in ('plume-2','plume-3'): m['box'] = (m['box'][0], m['box'][1], m['box'][2], 1.75)
            o.append(shape(R, m["id"], m["bone"], "body", mblob(m, 6, 0.12, 0.3, JIT["a"]), region="plumes", tone=0.3, z=4))
        return o
    r2 = blades(False)
    for n in (1, 2, 3, 4):
        a = f"culm-{n}a"; b = f"culm-{n}b"
        sa, ea, _, _ = woody(R, O, a); sb, eb, _, _ = woody(R, O, b)
        r2.append(shape(R, f"{a}-axis", a, "detail", stroke([sa, ea, eb]), filled=False, tone=0.2, z=5))
    r1 = blades(True)
    for n in (1, 2, 3, 4):
        a = f"culm-{n}a"; b = f"culm-{n}b"
        sa, ea, _, _ = woody(R, O, a); sb, eb, _, _ = woody(R, O, b)
        r1.append(shape(R, f"{a}-axis", a, "body", chain([sa, ea], [0.018, 0.014]), region="culms", tone=0.2, z=5))
        r1.append(shape(R, f"{b}-axis", b, "body", chain([sb, eb], [0.014, 0.01]), region="culms", tone=0.2, z=5))
    res["200"] = r2; res["100"] = r1
    return res
