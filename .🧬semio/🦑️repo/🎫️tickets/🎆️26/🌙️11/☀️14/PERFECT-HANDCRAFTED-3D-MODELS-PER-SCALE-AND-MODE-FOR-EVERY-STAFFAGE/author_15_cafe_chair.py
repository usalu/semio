from author_15_lib import *
BE, ST = "beech-timber", "steel-tube"
UB = [-0.0871557, 0.9961947, 0]; VB = [-0.9961947, -0.0871557, 0]
BO = [0.066, 0.46, 0]
def bp(u, v=0, z=0): return [BO[0] + u * UB[0] + v * VB[0], BO[1] + u * UB[1] + v * VB[1], z]
UR = [0.0348995, 0.9993908, 0]; UF = [-0.0348995, 0.9993908, 0]
def lp(o, d, s, z): return [o + d[0] * s, d[1] * s, z]
def legpts(which, s0, s1, z0, z1):
    o, d = (0.05, UR) if which == "rear" else (0.42, UF)
    return lp(o, d, s0, z0), lp(o, d, s1, z1)

def legs(c, r0, r1, rf=None, s0=0.0095, **ex):
    for w, bone, zb, zt in (("rear", "leg-rear", 0.2289, 0.18), ("front", "leg-front", 0.2384, 0.19)):
        for sg, n in ((1, "l"), (-1, "r")):
            p0, p1 = legpts(w, s0, 0.42, sg * zb, sg * zt)
            c.cap("leg-%s-tube-%s" % (w, n), bone, ST, "body", 0.35, p0, p1, r0, r1, **ex)

def stretchers(c, r, joints, role="body", **ex):
    for sg, n in ((1, "l"), (-1, "r")):
        c.cap("stretcher-tube-" + n, "stretcher", ST, role, 0.35, [0.0542, 0.12, sg * 0.2157], [0.4158, 0.12, sg * 0.2254], r, **ex)
        if joints:
            for k, x, z in (("a", 0.0542, 0.2157), ("b", 0.4158, 0.2254)):
                c.ell("stretcher-joint-%s-%s" % (k, n), "stretcher", ST, "detail", 0.25, [x, 0.12, sg * z], [0.0085, 0.0095, 0.0085])

def rr(x0, x1, z0, z1, r, n=4):
    pts = []
    for cx, cz, a0 in ((x1 - r, z1 - r, 0), (x0 + r, z1 - r, 90), (x0 + r, z0 + r, 180), (x1 - r, z0 + r, 270)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n); pts.append([cx + r * math.cos(a), cz + r * math.sin(a)])
    return pts

def seat50(c):
    c.ext("seat-pad", "seat", BE, "body", 0.62, rr(0.03, 0.458, -0.215, 0.215, 0.02), "xz", 0.432, 0.46)
    c.bw("seat-apron-side", "seat", BE, "body", 0.45, (0.06, 0.42), (0.398, 0.434), (0.1575, 0.1725), corner=0.004, repeat={"count": 2, "translation": [0, 0, -0.33]})
    c.bw("seat-apron-end", "seat", BE, "body", 0.45, (0.0675, 0.0825), (0.398, 0.434), (-0.165, 0.165), corner=0.004, repeat={"count": 2, "translation": [0.345, 0, 0]})

def structure(c, items, masses, feet_member):
    c.to("structure")
    for sid, m in items: c.ref("m-" + sid, sid, member=m, part="member")
    for sid, bone, mem, centre, rad in masses:
        c.bw(sid, bone, BE if "seat" in sid or "back" in sid else ST, "detail", 0.4, tuple((centre[i] - rad[i], centre[i] + rad[i]) for i in range(3))[0], tuple((centre[i] - rad[i], centre[i] + rad[i]) for i in range(3))[1], tuple((centre[i] - rad[i], centre[i] + rad[i]) for i in range(3))[2], member=mem, part="mass")

def s50():
    c = Ctx("cafe-chair", 50); c.to("body")
    legs(c, 0.0095, 0.0125)
    for w, bone, zb in (("rear", "leg-rear", 0.23), ("front", "leg-front", 0.2395)):
        for sg, n in ((1, "l"), (-1, "r")):
            o, d = (0.05, UR) if w == "rear" else (0.42, UF)
            c.rev("leg-foot-%s-%s" % (w, n), bone, ST, "detail", 0.15, [[0, 0.0095], [0.003, 0.0112], [0.006, 0.0115], [0.016, 0.0115]], d, [o, 0, sg * zb])
    for sg, n in ((1, "l"), (-1, "r")):
        for w, bone, zt in (("rear", "leg-rear", 0.18), ("front", "leg-front", 0.19)):
            p = legpts(w, 0.42, 0.42, zt, zt)[0]
            c.ell("leg-%s-knuckle-%s" % (w, n), bone, ST, "detail", 0.3, [p[0], p[1], sg * zt], [0.012, 0.014, 0.0125])
    stretchers(c, 0.007, True)
    seat50(c)
    for sg, n in ((1, "l"), (-1, "r")):
        c.cap("back-post-" + n, "back", BE, "body", 0.42, bp(0, 0, sg * 0.19), bp(0.405, 0, sg * 0.19), 0.013, 0.009)
        p = bp(0.412, 0, sg * 0.19); c.ell("back-finial-" + n, "back", BE, "body", 0.38, p, [0.012, 0.011, 0.012])
    c.box("back-panel", "back", BE, "body", 0.52, bp(0.3, -0.006), [0.17, 0.028, 0.37], axes=[UB, VB, Z], corner=0.009)
    c.box("back-lower-rail", "back", BE, "body", 0.5, bp(0.14, -0.007), [0.06, 0.02, 0.37], axes=[UB, VB, Z], corner=0.006)
    c.to("thermal")
    for sid in ("seat-pad", "seat-apron-side", "seat-apron-end"): c.ref("th-" + sid, sid, region="seat-beech", layer="beech")
    for sid in ("back-post-l", "back-post-r", "back-finial-l", "back-finial-r", "back-panel", "back-lower-rail"): c.ref("th-" + sid, sid, region="back-beech", layer="beech")
    for sid in [s["id"] for s in c.lists["body"] if s["material"] == ST]: c.ref("th-" + sid, sid, region="steel-tube", layer="tube-wall")
    structure_common(c, 50)
    c.dec("1:50 is the full model: four splayed 19 to 25 mm steel tubes with domed feet and knuckles under the seat, two stretcher tubes with joint balls, a seat pad with rounded plan corners on a four-board apron frame, two tapering back posts with finials, and a tilted back panel and lower rail whose axes follow the back bone (5 degrees); the legs splay sideways as drawn in the plan.", "1:50 ist das volle Modell: vier gespreizte 19 bis 25 mm Stahlrohre mit gewölbten Füssen und Knöcheln unter dem Sitz, zwei Quersteg-Rohre mit Gelenkkugeln, ein Sitzpolster mit gerundeten Grundrissecken auf einem Vier-Brett-Zargenrahmen, zwei verjüngte Lehnenpfosten mit Endknöpfen sowie ein geneigtes Rückenbrett und eine untere Traverse, deren Achsen der Lehnenknochen folgen (5 Grad); die Beine spreizen seitlich wie im Grundriss gezeichnet.")
    return c.out()

def structure_common(c, scale):
    body = {s["id"]: s for s in c.lists["body"]}
    c.to("structure")
    for sid, s in body.items():
        if sid.startswith("leg-rear-tube"): c.ref("m-" + sid, sid, member="leg-rear", part="member")
        elif sid.startswith("leg-front-tube"): c.ref("m-" + sid, sid, member="leg-front", part="member")
        elif sid.startswith("stretcher-tube"): c.ref("m-" + sid, sid, member="stretcher", part="member")
        elif sid.startswith("seat"): c.ref("m-" + sid, sid, member="seat", part="member")
        elif sid.startswith("back-post") or sid in ("back-panel", "back-lower-rail"): c.ref("m-" + sid, sid, member="back", part="member")
        elif sid.startswith("leg-foot"): c.ref("m-" + sid, sid, member="ground", part="member")
    if scale in (100, 200, 500):
        pass
    h = {50: 0.012, 100: 0.0125, 200: 0.025, 500: 0.0375}[scale]
    def m(id, bone, mem, c0, rad):
        c.bw(id, bone, ST, "detail", 0.4, (c0[0] - rad[0], c0[0] + rad[0]), (c0[1] - rad[1], c0[1] + rad[1]), (c0[2] - rad[2], c0[2] + rad[2]), member=mem, part="mass")
    m("mass-seat", "seat", "seat", [0.244, 0.446, 0], [0.06 if scale < 200 else 0.07, max(h, 0.012), 0.06 if scale < 200 else 0.07])
    mid = lambda a, b: [(a[i] + b[i]) / 2 for i in range(3)]
    for w, bone in (("rear", "leg-rear"), ("front", "leg-front")):
        a, b = legpts(w, 0.0095, 0.42, 0.2289 if w == "rear" else 0.2384, 0.18 if w == "rear" else 0.19)
        m("mass-%s-leg" % w, bone, bone, mid(a, b), [h, 0.04, h])
    m("mass-back", "back", "back", bp(0.3, 0, 0), [0.015 if scale < 200 else 0.0375 if scale == 500 else 0.03, 0.05 if scale < 500 else 0.0375, 0.03 if scale < 500 else 0.0375])

def s100():
    c = Ctx("cafe-chair", 100); c.to("body")
    legs(c, 0.01, 0.01, slender=True)
    stretchers(c, 0.007, False, "detail", slender=True)
    c.bw("seat-pad", "seat", BE, "body", 0.6, (0.03, 0.458), (0.432, 0.46), (-0.21, 0.21), corner=0.012)
    c.ext("seat-apron", "seat", BE, "body", 0.45, [[0.06, -0.19], [0.42, -0.19], [0.42, 0.19], [0.06, 0.19]], "xz", 0.398, 0.432, holes=[[[0.075, -0.175], [0.405, -0.175], [0.405, 0.175], [0.075, 0.175]]])
    for sg, n in ((1, "l"), (-1, "r")): c.cap("back-post-" + n, "back", BE, "body", 0.42, bp(0, 0, sg * 0.19), bp(0.418, 0, sg * 0.19), 0.01, slender=True)
    c.box("back-panel", "back", BE, "body", 0.52, bp(0.3, -0.006), [0.198, 0.028, 0.37], axes=[UB, VB, Z], corner=0.014)
    c.box("back-lower-rail", "back", BE, "body", 0.5, bp(0.14, -0.007), [0.08, 0.02, 0.37], axes=[UB, VB, Z], corner=0.01)
    c.to("thermal")
    for sid in ("seat-pad", "seat-apron"): c.ref("th-" + sid, sid, region="seat-beech")
    for sid in ("back-post-l", "back-post-r", "back-panel", "back-lower-rail"): c.ref("th-" + sid, sid, region="back-beech")
    for sid in [s["id"] for s in c.lists["body"] if s["material"] == ST]: c.ref("th-" + sid, sid, region="steel-tube")
    structure_common(c, 100)
    for sg, n in ((1, "l"), (-1, "r")):
        for w, bone, zb in (("rear", "leg-rear", 0.23), ("front", "leg-front", 0.2395)):
            o, d = (0.05, UR) if w == "rear" else (0.42, UF)
            c.rev("m-cap-%s-%s" % (w, n), bone, ST, "detail", 0.15, [[0, 0.016], [0.03, 0.016]], d, [o, 0, sg * zb], member="ground", part="member")
    c.dec("1:100 drops feet, knuckles, joint balls, finials, apron boards and the taper: legs and posts are constant 20 mm slender tubes at true size (the real tubes taper between 19 and 26 mm; drawn as pen strokes, C24) and the stretchers 14 mm slender tubes, the seat is a 28 mm plate on one 15 mm hollow apron frame, panel and rail are rounded slabs sized to the drawn capsules; positions and the 5 degree back tilt are those of 1:50.", "1:100 lässt Füsse, Knöchel, Gelenkkugeln, Endknöpfe, Zargenbretter und die Verjüngung weg: Beine und Pfosten sind gleichbleibende schlanke 20 mm Rohre in echter Grösse (die echten Rohre verjüngen sich zwischen 19 und 26 mm; als Federstriche gezeichnet, C24) und die Querstege schlanke 14 mm Rohre, der Sitz eine 28 mm Platte auf einem 15 mm Hohlrahmen, Rückenbrett und Traverse gerundete Platten nach den gezeichneten Kapseln; Lagen und die 5 Grad Lehnenneigung wie bei 1:50.")
    c.dec("Thermal references the body solids; structure: tubes, apron, panel and posts are the members of their bones, 32 mm cap collars at the leg feet are the ground member, three compact masses.", "Thermik verweist auf die Körper; Tragwerk: Rohre, Zarge, Brett und Pfosten sind die Glieder ihrer Knochen, 32 mm Kappenkragen an den Beinfüssen sind das Bodenglied, drei kompakte Massen.")
    return c.out()

def s200():
    c = Ctx("cafe-chair", 200); c.to("body")
    legs(c, 0.01, 0.01, slender=True)
    stretchers(c, 0.007, False, "detail", slender=True)
    c.bw("seat-pad", "seat", BE, "body", 0.6, (0.03, 0.458), (0.43, 0.46), (-0.21, 0.21))
    c.ext("seat-apron", "seat", BE, "body", 0.45, [[0.03, -0.21], [0.458, -0.21], [0.458, 0.21], [0.03, 0.21]], "xz", 0.4, 0.43, holes=[[[0.06, -0.18], [0.428, -0.18], [0.428, 0.18], [0.06, 0.18]]])
    for sg, n in ((1, "l"), (-1, "r")): c.cap("back-post-" + n, "back", BE, "body", 0.42, bp(0, 0, sg * 0.19), bp(0.418, 0, sg * 0.19), 0.01, slender=True)
    c.box("back-panel", "back", BE, "body", 0.52, bp(0.292, -0.006), [0.185, 0.032, 0.37], axes=[UB, VB, Z])
    c.to("thermal")
    for sid in ("seat-pad", "seat-apron"): c.ref("th-" + sid, sid, region="seat-beech")
    for sid in ("back-post-l", "back-post-r", "back-panel"): c.ref("th-" + sid, sid, region="back-beech")
    for sid in [s["id"] for s in c.lists["body"] if s["material"] == ST]: c.ref("th-" + sid, sid, region="steel-tube")
    structure_common(c, 200)
    for sg, n in ((1, "l"), (-1, "r")):
        for w, bone, zb in (("rear", "leg-rear", 0.23), ("front", "leg-front", 0.2395)):
            o, d = (0.05, UR) if w == "rear" else (0.42, UF)
            c.rev("m-cap-%s-%s" % (w, n), bone, ST, "detail", 0.15, [[0, 0.02], [0.06, 0.02]], d, [o, 0, sg * zb], member="ground", part="member")
    c.dec("1:200 keeps legs and posts as 20 mm and stretchers as 14 mm slender tubes at true size (strokes at the scale pen, C24), merges the lower rail into the 32 mm back panel and builds the seat as a 30 mm plate on a 30 mm perimeter frame flush with its edge.", "1:200 behält Beine und Pfosten als schlanke 20 mm Rohre und die Querstege als 14 mm Rohre in echter Grösse (Striche im Stiftmass des Massstabs, C24), fasst die untere Traverse im 32 mm Rückenbrett zusammen und baut den Sitz als 30 mm Platte auf einem 30 mm Randrahmen bündig zur Kante.")
    c.dec("Thermal and structure reference the body solids; the feet are 40 mm collars.", "Thermik und Tragwerk verweisen auf die Körper; die Füsse sind 40 mm Kragen.")
    return c.out()

def s500():
    c = Ctx("cafe-chair", 500); c.to("body")
    legs(c, 0.01, 0.01, slender=True)
    stretchers(c, 0.007, False, "body", slender=True)
    c.bw("seat-pad", "seat", BE, "body", 0.6, (0.03, 0.458), (0.36, 0.46), (-0.21, 0.21))
    for sg, n in ((1, "l"), (-1, "r")): c.cap("back-post-" + n, "back", BE, "body", 0.42, bp(0, 0, sg * 0.19), bp(0.418, 0, sg * 0.19), 0.01, slender=True)
    c.box("back-panel", "back", BE, "body", 0.52, bp(0.29, -0.006), [0.19, 0.075, 0.34], axes=[UB, VB, Z])
    c.to("thermal")
    c.ref("th-seat-pad", "seat-pad", region="seat-beech")
    for sid in ("back-post-l", "back-post-r", "back-panel"): c.ref("th-" + sid, sid, region="back-beech")
    for sid in [s["id"] for s in c.lists["body"] if s["material"] == ST]: c.ref("th-" + sid, sid, region="steel-tube")
    structure_common(c, 500)
    for sg, n in ((1, "l"), (-1, "r")):
        for w, bone, zb in (("rear", "leg-rear", 0.23), ("front", "leg-front", 0.2395)):
            o, d = (0.05, UR) if w == "rear" else (0.42, UF)
            c.rev("m-cap-%s-%s" % (w, n), bone, ST, "detail", 0.15, [[0, 0.0375], [0.075, 0.0375]], d, [o, 0, sg * zb], member="ground", part="member")
    c.dec("Minimal: 1:500 is a 100 mm seat block on four slender 20 mm legs joined by two slender 14 mm stretchers, two slender 20 mm back posts and one 75 mm back panel; the tubes stay at true size (drawn as 90 mm pen strokes, C24), only the seat and the panel are generalised to blocks of the 75 mm smallest feature.", "Minimal: 1:500 ist ein 100 mm Sitzblock auf vier schlanken 20 mm Beinen mit zwei schlanken 14 mm Querstegen, zwei schlanken 20 mm Lehnenpfosten und einem 75 mm Rückenbrett; die Rohre bleiben in echter Grösse (als 90 mm Federstriche gezeichnet, C24), nur Sitz und Brett sind zu Blöcken im 75 mm Mindestmass verallgemeinert.")
    c.dec("Thermal and structure reference the body solids; the feet are the leg ends themselves.", "Thermik und Tragwerk verweisen auf die Körper; die Füsse sind die Beinenden selbst.")
    return c.out()

if __name__ == "__main__":
    save("cafe-chair", {50: s50(), 100: s100(), 200: s200(), 500: s500()})
