from lib2 import *

def bbox_pts(p):
    X=[q[0] for q in p]; Y=[q[1] for q in p]
    return (min(X),max(X),min(Y),max(Y))

def mblob(m, n, depth, bulge, jit, phase=0.0):
    return fit_to(blob(m['cx'], m['cy'], m['a'], m['b'], m['ang'], n, depth, bulge, jit, phase), m['box'])

def masses(R, O, regions):
    out = []
    for s in O["geometry"]["elevation"]:
        if s["role"] == "body" and s.get("region") in regions:
            fr = pca_frame(shape_pts(R, s)); fr.update(box=bbox_pts(shape_pts(R, s)), bone=s["bone"], region=s["region"], id=s["id"], z=s["z"]); out.append(fr)
    return out

JIT = {"a": (0.0, 0.6, 0.2, 0.9, 0.35, 0.7, 0.1), "b": (0.3, 0.0, 0.8, 0.4, 1.0, 0.1, 0.6, 0.2)}
ARC = [((-0.55, -0.15), (-0.2, -0.5), (0.3, -0.25)), ((-0.1, 0.3), (0.25, 0.05), (0.55, 0.3))]

def arcs(R, m, tag, pen=None):
    out = []
    ca, sa = math.cos(math.radians(m["ang"])), math.sin(math.radians(m["ang"]))
    for k, arc in enumerate(ARC):
        pts = [(m["cx"] + (u*m["a"])*ca - (v*m["b"])*sa, m["cy"] + (u*m["a"])*sa + (v*m["b"])*ca) for u, v in arc]
        d = f"M{f(pts[0][0])} {f(pts[0][1])}Q{f(pts[1][0])} {f(pts[1][1])} {f(pts[2][0])} {f(pts[2][1])}"
        out.append(shape(R, f"{m['id']}-arc{k+1}-{tag}", m["bone"], "detail", d, filled=False, tone=0.3, z=9))
    return out

def design(R, O):
    ms = masses(R, O, ("leaf-sun", "leaf-shade"))
    res = {}
    # ---------------- 1:500
    P = [(100,1065),(52,945),(115,850),(195,775),(118,600),(150,505),(225,470),(300,380),(338,290),(362,215),(440,205),(505,130),(565,185),(640,95),(705,40),(800,55),(905,38),(965,100),(1030,200),(1100,125),(1180,170),(1208,255),(1190,340),(1365,350),(1420,420),(1458,520),(1440,640),(1330,712),(1385,740),(1490,760),(1520,915),(1480,1000),(1350,1070),(1240,1090),(1100,1075),(1010,985),(960,975),(770,975),(600,910),(530,1090),(400,1150),(250,1150)]
    W = px(P, -9.72, 0, 80, 18.65)
    crown = lobes(W, 0.26)
    bb = fit_bbox(crown, None)
    sx = (9.25 + 9.1)/(bb[1]-bb[0]); sy = 18.22/(bb[3]-18.65+18.65-0) if False else None
    W = [((p[0]-bb[0])*sx - 9.1, p[1]) for p in W]
    crown = lobes(W, 0.26)
    bb = fit_bbox(crown, None); k = 18.22/bb[3]
    W = [(p[0], p[1]*k) for p in W]; crown = lobes(W, 0.26)
    r5 = [
        shape(R, "limb-w", "limb-w-low", "body", chain([(0,3.0),(-4.21,4.19),(-6.6,4.35)], [0.42,0.26,0.1]), region="limb-bark", tone=0.45, z=1),
        shape(R, "limb-e", "limb-e-low", "body", chain([(0,3.3),(4.51,4.84),(7.2,5.25)], [0.42,0.26,0.1]), region="limb-bark", tone=0.45, z=1),
        shape(R, "bole", "trunk", "body", chain([(0,0),(0,0.45),(0,3.4),(0.14,6.0),(0.17,6.6)], [1.15,0.48,0.46,0.37,0.34]), region="trunk-bark", tone=0.35, z=2),
        shape(R, "crown", "leader", "body", crown, regions=["leaf-sun", "leaf-shade"], tone=0.65, z=3),
    ]
    res["500"] = r5
    # ---------------- 1:200 and 1:100
    def woodyset(tag, fine):
        o = []
        o.append(shape(R, "bole", "trunk", "body", chain([(0,0),(0,0.45),(0,3.4),(0.14,6.0),(0.17,6.6)], [1.15,0.48,0.46,0.37,0.34]), region="trunk-bark", tone=0.35, z=2))
        o.append(shape(R, "leader", "leader", "body", chain([(0.14,6.0),(-0.32,10.37),(0.15,13.74)], [0.37,0.29,0.14]), region="trunk-bark", tone=0.35, z=2))
        o.append(shape(R, "limb-w-low", "limb-w-low", "body", limb_path(R, O, ["limb-w-low","br-w-low"]), region="limb-bark", tone=0.4, z=1))
        o.append(shape(R, "limb-e-low", "limb-e-low", "body", limb_path(R, O, ["limb-e-low","br-e-low"]), region="limb-bark", tone=0.4, z=1))
        o.append(shape(R, "limb-w-mid", "limb-w-mid", "body", limb_path(R, O, ["limb-w-mid","br-w-mid"]), region="limb-bark", tone=0.4, z=1))
        o.append(shape(R, "limb-e-mid", "limb-e-mid", "body", limb_path(R, O, ["limb-e-mid","br-e-mid"]), region="limb-bark", tone=0.4, z=1))
        if fine:
            for b in ["br-leader-w","br-leader-e","br-low-w-up","br-low-e-up"]:
                o.append(shape(R, b, b, "body", limb_path(R, O, [b]), region="limb-bark", tone=0.4, z=1))
        return o
    r2 = woodyset("200", False)
    for i, m in enumerate(ms):
        shade = m["region"] == "leaf-shade"
        d = mblob(m, 9, 0.07, 0.34, JIT["a"], 0.3*i)
        r2.append(shape(R, m["id"], m["bone"], "body", d, region=m["region"], tone=0.45 if shade else 0.6, z=3 if shade else 5+i*0.1))
    r2.append(shape(R, "bole-fissure", "trunk", "detail", stroke([(0.1,0.7),(-0.05,1.8),(0.08,3.0)]), filled=False, tone=0.2, z=3))
    res["200"] = r2
    r1 = woodyset("100", True)
    for i, m in enumerate(ms):
        shade = m["region"] == "leaf-shade"
        d = mblob(m, 15, 0.055, 0.36, JIT["b"], 0.2*i)
        s = shape(R, m["id"], m["bone"], "body", d, region=m["region"], tone=0.45 if shade else 0.6, z=3 if shade else 5+i*0.1)
        if shade: s["hatch"] = "shade-lines"
        r1.append(s)
        r1 += arcs(R, m, "100")
    r1.append(shape(R, "bole-fissure-1", "trunk", "texture", stroke([(0.15,0.6),(0.02,1.6),(0.12,2.8)]), filled=False, tone=0.2, z=3))
    r1.append(shape(R, "bole-fissure-2", "trunk", "texture", stroke([(-0.18,0.9),(-0.12,2.0),(-0.2,3.1)]), filled=False, tone=0.2, z=3))
    r1.append(shape(R, "bole-fissure-3", "trunk-upper", "texture", stroke([(0.1,3.8),(0.18,4.8),(0.1,5.7)]), filled=False, tone=0.2, z=3))
    res["100"] = r1
    return res
