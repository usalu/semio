from author_15_lib import *
BE, CA, SH = "beech-timber", "cotton-canvas", "steel-hardware"
F = FRAMES["folding-deck-chair"]
def fr(b): return F[b]["frame"]
def pt(b, u, v=0, z=0):
    f = fr(b); return [f["o"][0] + u * f["u"][0] + v * f["v"][0], f["o"][1] + u * f["u"][1] + v * f["v"][1], z]
def ax(b): f = fr(b); return [f["u"], f["v"], Z]
def bar(c, id, b, mat, tone, u0, u1, z, t, tz, corner=None, repeat=True, role="body", **ex):
    s = c.box(id, b, mat, role, tone, pt(b, (u0 + u1) / 2, 0, z), [u1 - u0, t, tz], axes=ax(b), corner=corner, **ex)
    if repeat: s["repeat"] = c.rep(b, {"count": 2, "translation": [0, 0, -2 * z]})
    return s
def rod(c, id, b, mat, tone, u, z0, z1, r, **ex):
    p = pt(b, u); return c.cap(id, b, mat, "body", tone, [p[0], p[1], z0], [p[0], p[1], z1], r, **ex)
def bolt(c, id, side, scale):
    p = pt("rear", 0.726)
    z0, d = (0.2925 + 0.0, -1) if side > 0 else (-0.2925, 1)
    c.rev(id, "rear", SH, "body", 0.2, [[0, 0.0175 if scale == 50 else 0.0125], [0.006, 0.0175 if scale == 50 else 0.0125], [0.006, 0.0125], [0.08, 0.0125]], [0, 0, d], [p[0], p[1], z0 + (0.006 * -d if False else 0)])
SUPPORT = [("foot-rear-left", "rear", 0.0, 0.27), ("foot-rear-right", "rear", 0.0, -0.27), ("foot-front-left", "front", 0.6817, 0.235), ("foot-front-right", "front", 0.6817, -0.235)]

def sling(c, t, sag_a, sag_b, n):
    A, Ca, dirA = (0.4906, 0.9227), None, (-0.2793, -0.9602)
    B, dirB = (0.7704, 0.395), (-0.9942, -0.1077)
    C = (A[0] + dirA[0] * 0.6, A[1] + dirA[1] * 0.6)
    nA, nB = (0.9602, -0.2793), (0.1077, -0.9942)
    def path(P0, nv, sag):
        pts = []
        for i in range(n + 1):
            k = i / n; q = (P0[0] + (C[0] - P0[0]) * k + sag * 4 * k * (1 - k) * nv[0], P0[1] + (C[1] - P0[1]) * k + sag * 4 * k * (1 - k) * nv[1])
            pts.append(q)
        return pts
    def unit(a): l = math.hypot(*a); return (a[0] / l, a[1] / l)
    pa, pb = path(A, nA, sag_a), path(B, nB, sag_b)
    ta = unit((pa[-1][0] - pa[-2][0], pa[-1][1] - pa[-2][1])); tb = unit((pb[-1][0] - pb[-2][0], pb[-1][1] - pb[-2][1]))
    m = unit((ta[0] - tb[0], ta[1] - tb[1]))
    def band(pts, tend):
        h = t / 2; left, right = [], []
        for i, q in enumerate(pts):
            a = pts[max(i - 1, 0)]; b = pts[min(i + 1, len(pts) - 1)]
            tg = unit((b[0] - a[0], b[1] - a[1])); nn = (-tg[1], tg[0])
            if i == len(pts) - 1:
                for side, out in ((1, left), (-1, right)):
                    o = (q[0] + side * h * nn[0], q[1] + side * h * nn[1])
                    s_ = -((o[0] - C[0]) * m[0] + (o[1] - C[1]) * m[1]) / (tend[0] * m[0] + tend[1] * m[1])
                    out.append([o[0] + s_ * tend[0], o[1] + s_ * tend[1]])
            else:
                left.append([q[0] + h * nn[0], q[1] + h * nn[1]]); right.append([q[0] - h * nn[0], q[1] - h * nn[1]])
        return left + right[::-1]
    c.ext("canvas-back", "canvas-a", CA, "body", 0.8, band(pa, ta), "xy", -0.2, 0.2)
    c.ext("canvas-seat", "canvas-b", CA, "body", 0.65, band(pb, tb), "xy", -0.2, 0.2)

def supports(c, h, w):
    c.to("structure")
    for sid, b, x, z in SUPPORT:
        c.box("support-" + sid, b, SH, "ground", 0.2, [x + (0.0225 if b == "rear" else 0), h / 2, z], [w, h, w], member=b, part="support", anchor=sid)

def thermal_structure(c, frame_ids, back, seat, scale, joints):
    c.to("thermal")
    for sid in frame_ids: c.ref("th-" + sid, sid, region="frame-timber")
    c.ref("th-canvas-back", back, region="canvas-back"); c.ref("th-canvas-seat", seat, region="canvas-seat")
    c.to("structure")
    mem = {"rear": "rear", "front": "front", "seat": "seat", "top-rail": "rear", "front-rail": "seat"}
    for sid in frame_ids: c.ref("m-" + sid, sid, member=mem[sid], part="member")
    c.ref("m-canvas-a", back, member="canvas-a", part="member"); c.ref("m-canvas-b", seat, member="canvas-b", part="member")

def zbox(c, id, b, mat, centre, h, z, member, part, role="detail"):
    s = c.bw(id, b, mat, role, 0.3, (centre[0] - h, centre[0] + h), (centre[1] - h, centre[1] + h), (z - h, z + h), member=member, part=part)
    s["repeat"] = c.rep(b, {"count": 2, "translation": [0, 0, -2 * z]}); return s

def masses(c, t):
    c.to("structure")
    zbox(c, "mass-rear", "rear", BE, pt("rear", 0.5), t, 0.27, "rear", "mass")
    zbox(c, "mass-front", "front", BE, pt("front", 0.35), t, 0.235, "front", "mass")
    zbox(c, "mass-seat", "seat", BE, pt("seat", 0.25), t, 0.18, "seat", "mass")

def joints(c, s):
    c.to("structure")
    zbox(c, "j-rear-foot", "rear", SH, pt("rear", 0.15), s, 0.27, "rear", "joint")
    zbox(c, "j-front-foot", "front", SH, pt("front", 0.15), s, 0.235, "front", "joint")
    zbox(c, "j-seat-top", "seat", SH, pt("seat", 0.03), s, 0.18, "seat", "joint")
    sh = max(s, 0.0225)
    zbox(c, "m-front-slide", "front-slide", SH, pt("front", 0.06), sh, 0.235, "front-slide", "member")
    zbox(c, "m-shoe-rear", "rear", SH, pt("rear", 0.07), sh, 0.27, "ground", "member")
    zbox(c, "m-shoe-front", "front", SH, pt("front", 0.07), sh, 0.235, "ground", "member")

def s50():
    c = Ctx("folding-deck-chair", 50); c.to("body")
    bar(c, "rear", "rear", BE, 0.55, 0.025, 1.045, 0.27, 0.045, 0.045, corner=0.0045)
    bar(c, "front", "front", BE, 0.55, 0.025, 0.726, 0.235, 0.045, 0.045, corner=0.0045)
    bar(c, "seat", "seat", BE, 0.55, 0.0, 0.495, 0.18, 0.045, 0.045, corner=0.0045)
    for sg, n in ((1, "l"), (-1, "r")):
        p = pt("rear", 0.726)
        c.rev("bolt-" + n, "rear", SH, "body", 0.2, [[0, 0.0175], [0.006, 0.0175], [0.006, 0.0125], [0.08, 0.0125]], [0, 0, -sg], [p[0], p[1], sg * 0.2925])
    rod(c, "top-rail", "rear", BE, 0.4, 1.045, -0.27, 0.27, 0.0125)
    rod(c, "front-rail", "seat", BE, 0.4, 0.495, -0.18, 0.18, 0.0125)
    sling(c, 0.0075, 0.008, 0.012, 12)
    thermal_structure(c, ["rear", "front", "seat", "top-rail", "front-rail"], "canvas-back", "canvas-seat", 50, True)
    c.to("structure"); c.ref("m-bolt-l", "bolt-l", member="seat", part="joint"); c.ref("m-bolt-r", "bolt-r", member="seat", part="joint")
    joints(c, 0.0225); masses(c, 0.0225); supports(c, 0.008, 0.045)
    for sg, n in ((1, "l"), (-1, "r")):
        pass
    c.dec("1:50 is the full model: rear and front frame posts and the seat bars are 45 x 45 mm beech with 4.5 mm edge rounding set at their true z offsets (270, 235, 180 mm), the top rail and front rail are 25 mm dowels across, two headed steel bolts pin rear and front posts at the crossing, and the sling is two sagging 7.5 mm canvas bands (extruded curves with a belly of 8 mm on the back and 12 mm on the seat) meeting in the low crease at (0.32, 0.35) m, mitred on the bisector so no notch opens.", "1:50 ist das volle Modell: Hinter- und Vorderbeine sowie Sitzholme sind 45 x 45 mm Buche mit 4,5 mm Kantenrundung auf ihren wahren z-Lagen (270, 235, 180 mm), Kopf- und Vorderleiste sind 25 mm Rundhölzer quer, zwei Kopfschrauben aus Stahl verbinden Hinter- und Vorderbein im Kreuzungspunkt, und die Sitzfläche besteht aus zwei durchhängenden 7,5 mm Segeltuchbändern (extrudierte Kurven mit 8 mm Bauch am Rücken und 12 mm an der Sitzfläche), die sich in der tiefen Falte bei (0,32; 0,35) m auf Gehrung treffen, sodass keine Kerbe entsteht.")
    c.dec("Thermal: single-layer regions reference the body solids (frame timber, the two canvas panels; the 0.7 mm canvas is modelled at the 7.5 mm smallest feature). Structure: bars are members of their bones, canvas panels the sling members, bolts and foot/slide blocks the joints, three compact masses, four 8 mm contact pads at the foot anchors.", "Thermik: einschichtige Regionen verweisen auf die Körper (Rahmenholz, die zwei Segeltuchfelder; das 0,7 mm Tuch ist im 7,5 mm Mindestmass modelliert). Tragwerk: Holme sind Glieder ihrer Knochen, die Tuchfelder die Sackglieder, Schrauben und Fuss-/Gleitblöcke die Gelenke, drei kompakte Massen, vier 8 mm Auflager an den Fussankern.")
    return c.out()

def caps(c, id, b, mat, tone, u0, u1, z, r, **ex):
    p0, p1 = pt(b, u0), pt(b, u1)
    s = c.cap(id, b, mat, "body", tone, [p0[0], p0[1], z], [p1[0], p1[1], z], r, **ex)
    s["repeat"] = c.rep(b, {"count": 2, "translation": [0, 0, -2 * z]}); return s

def s100():
    c = Ctx("folding-deck-chair", 100); c.to("body")
    caps(c, "rear", "rear", BE, 0.55, 0.0225, 1.0225, 0.27, 0.0225)
    caps(c, "front", "front", BE, 0.55, 0.0225, 0.7035, 0.235, 0.0225)
    caps(c, "seat", "seat", BE, 0.55, 0.0225, 0.4725, 0.18, 0.0225)
    p = pt("rear", 0.726)
    for sg, n in ((1, "l"), (-1, "r")): c.rev("bolt-" + n, "rear", SH, "body", 0.2, [[0, 0.0125], [0.07, 0.0125]], [0, 0, -sg], [p[0], p[1], sg * 0.2925])
    rod(c, "top-rail", "rear", SH, 0.4, 1.045, -0.27, 0.27, 0.0125)
    rod(c, "front-rail", "seat", SH, 0.4, 0.495, -0.18, 0.18, 0.0125)
    sling(c, 0.015, 0.008, 0.012, 6)
    thermal_structure(c, ["rear", "front", "seat", "top-rail", "front-rail"], "canvas-back", "canvas-seat", 100, True)
    c.to("structure"); c.ref("m-bolt-l", "bolt-l", member="seat", part="joint"); c.ref("m-bolt-r", "bolt-r", member="seat", part="joint")
    joints(c, 0.0225); masses(c, 0.0225); supports(c, 0.015, 0.045)
    c.dec("1:100 turns the posts and seat bars into round 45 mm rods (the drawn capsules), the rails into 25 mm steel rods, drops the edge rounding and the bolt heads and thickens the sagging canvas bands to the 15 mm smallest feature (six segments each, same belly of 8 and 12 mm); positions, crossing point and angles are those of 1:50.", "1:100 macht Beine und Sitzholme zu runden 45 mm Stäben (die gezeichneten Kapseln), die Leisten zu 25 mm Stahlstäben, lässt Kantenrundung und Schraubenköpfe weg und verdickt die durchhängenden Tuchbänder auf das 15 mm Mindestmass (je sechs Segmente, derselbe Bauch von 8 und 12 mm); Lagen, Kreuzungspunkt und Winkel wie bei 1:50.")
    return c.out()

def s200():
    c = Ctx("folding-deck-chair", 200); c.to("body")
    bar(c, "rear", "rear", BE, 0.55, 0.0, 1.045, 0.27, 0.045, 0.045)
    bar(c, "front", "front", BE, 0.55, 0.0, 0.726, 0.235, 0.045, 0.045)
    bar(c, "seat", "seat", BE, 0.55, 0.0, 0.495, 0.18, 0.045, 0.045)
    c.box("canvas-back", "canvas-a", CA, "body", 0.8, pt("canvas-a", 0.3), [0.6, 0.03, 0.4], axes=ax("canvas-a"))
    c.box("canvas-seat", "canvas-b", CA, "body", 0.65, pt("canvas-b", 0.225), [0.45, 0.03, 0.4], axes=ax("canvas-b"))
    thermal_structure(c, ["rear", "front", "seat"], "canvas-back", "canvas-seat", 200, False)
    c.to("thermal")
    rod(c.to("thermal"), "th-rails", "rear", BE, 0.4, 1.045, -0.27, 0.27, 0.0125, region="frame-timber", slender=True)
    c.bw("th-rail-front", "seat", BE, "body", 0.4, (0.3408 + 0.495 * 0.8678 - 0.0225, 0.3408 + 0.495 * 0.8678 + 0.0225), (0.641 - 0.495 * 0.497 - 0.0225, 0.641 - 0.495 * 0.497 + 0.0225), (-0.18, 0.18), region="frame-timber")
    joints(c, 0.03); masses(c, 0.03); supports(c, 0.03, 0.045)
    c.dec("1:200 keeps three square 45 mm post and bar pairs (the 30 mm feature is below their true size), leaves out the dowel rails and bolts from the body and all edge rounding, and draws the sling as two flat 30 mm panels (the 8 to 12 mm belly is below the 30 mm feature). The elevation drawing shows the three bars as filled 45 mm outlines, so the bars stay at their true 45 mm (C13); the thermal list adds the 25 mm top dowel as a slender rod and the front rail as a 45 mm block.", "1:200 behält drei quadratische 45 mm Holmpaare (das 30 mm Mindestmass liegt unter ihrer wahren Grösse), lässt Rundhölzer und Schrauben im Körper sowie alle Kantenrundungen weg und zeichnet den Sack als zwei ebene 30 mm Felder (der Bauch von 8 bis 12 mm liegt unter dem 30 mm Mindestmass). Die Ansicht zeigt die drei Holme als gefüllte 45 mm Umrisse, die Holme bleiben daher in wahrer Grösse von 45 mm (C13); die Thermikliste ergänzt das 25 mm Kopfrundholz als schlanken Stab und die Vorderleiste als 45 mm Block.")
    return c.out()

def s500():
    c = Ctx("folding-deck-chair", 500); c.to("body")
    bar(c, "rear", "rear", BE, 0.55, 0.0, 1.045, 0.27, 0.045, 0.045)
    bar(c, "front", "front", BE, 0.55, 0.0, 0.726, 0.235, 0.045, 0.045)
    bar(c, "seat", "seat", BE, 0.55, 0.0, 0.495, 0.18, 0.045, 0.045)
    c.box("canvas-back", "canvas-a", CA, "body", 0.8, pt("canvas-a", 0.3), [0.6, 0.075, 0.4], axes=ax("canvas-a"))
    c.box("canvas-seat", "canvas-b", CA, "body", 0.65, pt("canvas-b", 0.225), [0.45, 0.075, 0.4], axes=ax("canvas-b"))
    thermal_structure(c, ["rear", "front", "seat"], "canvas-back", "canvas-seat", 500, False)
    c.to("thermal")
    c.bw("th-rails", "rear", BE, "body", 0.4, (0.4906 - 0.0225, 0.4906 + 0.0225), (0.9227 - 0.0225, 0.9227 + 0.0225), (-0.27, 0.27), region="frame-timber")
    c.bw("th-rail-front", "seat", BE, "body", 0.4, (0.3408 + 0.495 * 0.8678 - 0.0225, 0.3408 + 0.495 * 0.8678 + 0.0225), (0.641 - 0.495 * 0.497 - 0.0225, 0.641 - 0.495 * 0.497 + 0.0225), (-0.18, 0.18), region="frame-timber")
    joints(c, 0.0375); masses(c, 0.0375); supports(c, 0.075, 0.075)
    c.dec("Minimal: 1:500 is three post and bar pairs at their true 45 mm (the elevation draws them as filled 45 mm outlines, C13) and two flat 75 mm canvas panels; only the 0.7 mm canvas is forced to the 75 mm smallest feature because it is drawn as a stroke. The thermal list carries the top and front rails as 45 mm blocks.", "Minimal: 1:500 sind drei Holmpaare in wahrer Grösse von 45 mm (die Ansicht zeichnet sie als gefüllte 45 mm Umrisse, C13) und zwei ebene 75 mm Tuchfelder; nur das 0,7 mm Tuch wird auf das 75 mm Mindestmass gezwungen, weil es als Strich gezeichnet ist. Die Thermikliste führt Kopf- und Vorderleiste als 45 mm Blöcke.")
    return c.out()

if __name__ == "__main__":
    save("folding-deck-chair", {50: s50(), 100: s100(), 200: s200(), 500: s500()})
