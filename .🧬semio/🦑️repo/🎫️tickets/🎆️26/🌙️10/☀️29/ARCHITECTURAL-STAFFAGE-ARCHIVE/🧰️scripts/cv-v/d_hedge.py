from lib2 import *

def scallop_block(x0, x1, y0, y1, nt, h, r=0.08):
    w = (x1-x0)/nt
    d = f"M{f(x0)} {f(y0)}L{f(x0)} {f(y1-r)}Q{f(x0)} {f(y1)} {f(x0+r)} {f(y1)}"
    xa = x0 + r
    seg = (x1 - r - xa)/nt
    for i in range(nt):
        xb = xa + seg
        d += f"C{f(xa+seg*0.15)} {f(y1+h)} {f(xb-seg*0.15)} {f(y1+h)} {f(xb)} {f(y1)}"
        xa = xb
    d += f"Q{f(x1)} {f(y1)} {f(x1)} {f(y1-r)}L{f(x1)} {f(y0)}Z"
    return d

def design(R, O):
    res = {}
    top = [1.82,1.85,1.8,1.86,1.83,1.84,1.81]
    P = [(-1.57,0.34)] + [(-1.57 + 3.14*i/18, 1.85 - 0.04*((i*7) % 3)) for i in range(19)] + [(1.57,0.34)]
    P = [(-1.57,0.34),(-1.57,1.7)] + [(-1.57 + 3.14*i/18, 1.88 - 0.04*((i*5) % 3)) for i in range(1, 18)] + [(1.57,1.7),(1.57,0.34)]
    body = fit_y(fit_x(lobes(P, 0.12), -1.57, 1.57), 0.34, 1.88)
    res["500"] = [
        shape(R, "plinth", "root-plate", "body", poly([(-1.5,0.0),(1.5,0.0),(1.5,0.4),(-1.5,0.4)]), region="stem-bark", tone=0.15, z=1),
        shape(R, "hedge", "root-plate", "body", body, regions=["leaf-face", "leaf-core"], tone=0.6, z=2)]
    def blocks(fine):
        o = [shape(R, "plinth", "root-plate", "body", poly([(-1.5,0.0),(1.5,0.0),(1.5,0.4),(-1.5,0.4)]), region="stem-bark", tone=0.15, z=1)]
        for i in range(1, 7):
            b = f"block-{i}"; cx = -1.75 + 0.5*i
            x0, x1 = cx-0.32, cx+0.32
            nt = 4 if fine else 2
            yt = 1.84 if i % 2 else 1.81
            o.append(shape(R, b, b, "body", scallop_block(x0, x1, 0.34, yt, nt, 0.05), region="leaf-face", tone=0.6, z=2+0.1*i))
            core = poly([(x0+0.1, 0.55), (x1-0.1, 0.55), (x1-0.1, 1.55), (x0+0.1, 1.55)])
            o.append(shape(R, f"core-{i}", b, "body", core, region="leaf-core", tone=0.4, z=2.5+0.1*i))
        return o
    r2 = blocks(False)
    r1 = blocks(True)
    for i in range(1, 7):
        b = f"block-{i}"; cx = -1.75 + 0.5*i
        for k, (u, v) in enumerate(((-0.12, 1.45), (0.1, 1.15), (-0.08, 0.85))):
            x, y = cx+u, v
            r1.append(shape(R, f"leaf-{i}-{k+1}", b, "texture", f"M{f(x-0.1)} {f(y)}Q{f(x)} {f(y-0.07)} {f(x+0.1)} {f(y)}", filled=False, tone=0.2, z=9))
        r1.append(shape(R, f"stem-{i}", f"stem-{i}", "detail", stroke([(cx-0.5+0.5, 0.0), (cx-0.5+0.5, 0.34)], False) if False else stroke([(-1.75+0.5*i, 0.0), (-1.75+0.5*i, 0.34)], False), filled=False, tone=0.2, z=3))
    res["200"] = r2; res["100"] = r1
    return res
