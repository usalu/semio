# [scratch] shared helpers for the dairy-cow scratch scripts: rest frames and world-to-bone coordinate conversion
import math, json, re
def mat(x, y, deg):
    r = math.radians(deg); c, s = math.cos(r), math.sin(r)
    return (c, s, -s, c, x, y)
def mul(p, l):
    return (p[0]*l[0]+p[2]*l[1], p[1]*l[0]+p[3]*l[1], p[0]*l[2]+p[2]*l[3], p[1]*l[2]+p[3]*l[3],
            p[0]*l[4]+p[2]*l[5]+p[4], p[1]*l[4]+p[3]*l[5]+p[5])
def inv(m):
    d = m[0]*m[3]-m[1]*m[2]
    a, b, c, e = m[3]/d, -m[1]/d, -m[2]/d, m[0]/d
    return (a, b, c, e, -(a*m[4]+c*m[5]), -(b*m[4]+e*m[5]))
def app(m, x, y): return (m[0]*x+m[2]*y+m[4], m[1]*x+m[3]*y+m[5])
GY = 1.10
def rest_world(bones):
    w = {}
    by = {b["id"]: b for b in bones}
    def get(i):
        if i in w: return w[i]
        b = by[i]
        p = mat(0, GY, 0) if b["parent"] is None else get(b["parent"])
        w[i] = mul(p, mat(b["x"], b["y"], b["angleDeg"])); return w[i]
    for b in bones: get(b["id"])
    return w
def conv_path(d, m):
    t = d.split(); out = []; i = 0
    while i < len(t):
        c = t[i]; i += 1
        if c == "Z": out.append("Z"); continue
        n = {"M": 2, "L": 2, "C": 6}[c]
        v = [float(x) for x in t[i:i+n]]; i += n
        pts = []
        for k in range(0, n, 2):
            x, y = app(m, v[k], v[k+1]); pts += [round(x, 4), round(y, 4)]
        out.append(c + " " + " ".join(f"{p:g}" for p in pts))
    return " ".join(out)
