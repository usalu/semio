from crowns import *
from decs import std_dec
from spec_lombardy_poplar import branch_front

# curtain ring: (key, bone_w, bone_e, R along the axis, R across, zc, rv, [(az, imp)])
RINGS = [
    ("1", "curt-w1", "curt-e1", 1.6, 1.6, 4.15, 3.76, [(0, 1), (180, 1), (90, 1), (270, 1)]),
    ("2", "curt-w2", "curt-e2", 4.0, 3.5, 5.02, 4.88, [(0, 1), (180, 1), (90, 1), (270, 1), (45, 3), (135, 3), (225, 3), (315, 3)]),
    ("3", "curt-w3", "curt-e3", 5.5, 4.7, 4.98, 4.92, [(0, 1), (180, 1), (90, 1), (270, 1), (45, 2), (135, 2), (225, 2), (315, 2)]),
    ("4", "curt-w4", "curt-e4", (6.65, 6.35), 5.6, 4.95, 4.95, [(0, 1), (180, 1), (90, 1), (270, 1), (30, 2), (60, 2), (120, 2), (150, 2), (210, 2), (240, 2), (300, 2), (330, 2), (15, 3), (75, 3), (105, 3), (165, 3), (195, 3), (255, 3), (285, 3), (345, 3)]),
]
RT = 1.24


def curtains():
    out = []
    for key, bw, be, ra, rl, zc, rv, azs in RINGS:
        for az, imp in azs:
            a = math.radians(az)
            c, s = math.cos(a), math.sin(a)

            ra = ra if not isinstance(ra, tuple) else (ra[1] if c >= 0 else ra[0])
            x, y = ra * c, rl * s
            east = c > 0.05 or (abs(c) <= 0.05 and s > 0)
            out.append(dict(name="c%s-%d" % (key, az), bone=be if east else bw, x=x, y=y, zc=zc + 0.15 * (seed(key, az) - 0.5), rv=rv * (0.96 + 0.08 * seed(key, az, "v")), imp=imp, az=a))
    return out


def build(b):
    CU = curtains()
    for sc in ("50", "100", "200"):
        p = P[sc]
        use = [q for q in CU if q["imp"] <= p["imp"]]
        for i, q in enumerate(use):
            sd = seed(q["name"], sc)
            k = p["k"] + int(sd * 3) - 1
            pts = outline(q["y"], q["zc"], RT * (0.95 + 0.1 * sd), q["rv"], n=p["n"], k=k, amp=p["amp"] * 0.8, ph=sd * 6.28)
            pts = [(x, max(y, 0.05)) for x, y in pts]
            zf = 4.0 + 2.0 * (q["x"] + 8.0) / 16.0 + 0.001 * i
            taper(b, sc, "front", "f-" + q["name"], q["bone"], zf, "curt-" + q["bone"][5:], pts, tone=None)
            rr = 0.35 + 0.1 * sd
            x0, y0 = q["x"] - rr * math.cos(q["az"]), q["y"] - rr * math.sin(q["az"])
            x1, y1 = q["x"] + rr * math.cos(q["az"]), q["y"] + rr * math.sin(q["az"])
            like = "curt-" + q["bone"][5:]
            b.add(sc, "plan", "p-" + q["name"], q["bone"], 4.0 + 2.0 * (1 - math.hypot(q["x"], q["y"]) / 8.0) + 0.001 * i, like=like, kind="capsule", x0=round(x0, 4), y0=round(y0, 4), x1=round(x1, 4), y1=round(y1, 4), r0=round(RT * (0.9 + 0.1 * sd), 4), r1=round(RT * (0.9 + 0.1 * sd), 4), followTip=True)
        capl = [lobe("cap-a", "cap-w", "cap-w", -3.11, 0.0, 9.11, 3.34, 1.92, "cap-w", 1), lobe("cap-b", "cap-e", "cap-e", 3.14, 0.0, 9.14, 3.49, 1.91, "cap-e", 1),
                lobe("cap-c", "cap-w", "cap-w", -0.3, 3.0, 9.0, 3.0, 1.8, "cap-w", 1), lobe("cap-d", "cap-e", "cap-e", 0.3, -3.0, 9.1, 3.0, 1.8, "cap-e", 1),
                lobe("cap-m", "cap-w", "cap-w", 0.0, 0.0, 9.9, 2.6, 1.3, "cap-w", 2)]
        for l in capl:
            l["att"] = 7.0
        emit_lobes(b, sc, capl, base_front=7.0, base_plan=7.0)
        limbs(b, sc, capl, "limb-bark", lim=1)
    # bole and limbs in front
    for sc in ("50",):
        taper(b, sc, "front", "root-flare", "root-plate", 3.0, "root-flare", [(-1.3, 0.0), (1.3, 0.0), (0.65, 0.35), (-0.65, 0.35)])
        taper(b, sc, "front", "trunk-1", "trunk-1", 3.0, "trunk-1", [(-0.65, 0.3), (0.65, 0.3), (0.5, 2.2), (-0.5, 2.2)])
        taper(b, sc, "front", "trunk-2", "trunk-2", 3.05, "trunk-2", [(-0.5, 2.2), (0.5, 2.2), (0.4, 3.5), (-0.4, 3.5)])
    for sc in ("100", "200", "500"):
        taper(b, sc, "front", "bole", "trunk-1", 2.0, "bole", [(-1.3, 0.0), (1.3, 0.0), (0.6, 0.4), (0.5, 2.2), (0.4, 3.5), (-0.4, 3.5), (-0.5, 2.2), (-0.6, 0.4)])
    for sc in ("50", "100", "200"):
        disc(b, sc, "plan", "bole", "trunk-1", 1.0, 0.0, 0.0, 0.9, 0.9, "bole" if sc != "50" else "trunk-1")
    for sc in ("50", "100", "200"):
        a = {"50": ("limb-w-a", "limb-w-a"), "100": ("limb-w", "limb-e"), "200": ("limb-w", "limb-e")}[sc]
        segs = [("w-a", "limb-w-a", 0.0, 3.5, 1.6, 7.75, 0.45, 0.3, 1), ("w-b", "limb-w-b", 1.6, 7.75, 4.0, 9.85, 0.3, 0.2, 1), ("w-c", "limb-w-c", 4.0, 9.85, 5.5, 9.9, 0.2, 0.12, 1),
                ("e-a", "limb-e-a", 0.0, 3.5, -1.6, 7.75, 0.45, 0.3, 1), ("e-b", "limb-e-b", -1.6, 7.75, -4.0, 9.85, 0.3, 0.2, 1), ("e-c", "limb-e-c", -4.0, 9.85, -5.5, 9.9, 0.2, 0.12, 1)]
        for nm, bone, y0, z0, y1, z1, w0, w1, _ in segs:
            if sc != "50" and not nm.endswith("a"):
                continue
            like = "limb-" + nm if sc == "50" else ("limb-w" if nm[0] == "w" else "limb-e")
            if sc != "50":
                y1, z1 = y1 * 2.0 if False else (y1 + (4.0 if y1 > 0 else -4.0)), z1 + 2.2
            branch_front(b, sc, "limb-" + nm, bone, like, y0, z0, y1, z1, w0, w1, 2.4)
    for sc in ("50", "100"):
        for nm, bone, ax, ay, bx, by, w in (("w-a", "limb-w-a", 0.0, 0.0, -1.56, 0.0, 0.34), ("w-b", "limb-w-b", -1.56, 0.0, -3.97, 0.0, 0.25), ("w-c", "limb-w-c", -3.97, 0.0, -5.4, 0.0, 0.18),
                                       ("e-a", "limb-e-a", 0.0, 0.0, 1.37, 0.0, 0.34), ("e-b", "limb-e-b", 1.37, 0.0, 3.79, 0.0, 0.25), ("e-c", "limb-e-c", 3.79, 0.0, 5.3, 0.0, 0.18),
                                       ("l-a", "limb-w-a", 0.0, 0.0, 0.0, 1.6, 0.3), ("l-b", "limb-w-b", 0.0, 1.6, 0.0, 3.5, 0.22), ("r-a", "limb-e-a", 0.0, 0.0, 0.0, -1.6, 0.3), ("r-b", "limb-e-b", 0.0, -1.6, 0.0, -3.5, 0.22)):
            if sc == "100" and nm[2] in "bc":
                continue
            b.add(sc, "plan", "pl-limb-" + nm, bone, 7.5, like="limb-w-a" if bone.startswith("limb-w") else "limb-e-a", role=P[sc]["arc"], filled=False, tone=0.3, kind="capsule", x0=ax, y0=ay, x1=bx, y1=by, r0=w, r1=w * 0.7, followTip=True)
    fr = fit([(-7.0, 0.0), (-7.1, 4.0), (-6.2, 8.5), (-3.5, 10.9), (0.0, 11.05), (3.5, 10.9), (6.2, 8.5), (7.1, 4.0), (7.0, 0.0), (5.2, 0.5), (3.5, 0.0), (1.7, 0.5), (0.0, 0.0), (-1.7, 0.5), (-3.5, 0.0), (-5.2, 0.5)], -7.0, 7.0, 0.0, 11.05)
    b.add("500", "front", "crown", "limb-w-a", 3.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 7.5, 6.5, n=18, k=8, amp=0.05, ph=0.6), -8.26, 7.97, -7.0, 7.0)
    b.add("500", "plan", "crown", "limb-w-a", 3.0, d=smooth(pl), like="crown")
    disc(b, "500", "plan", "bole", "trunk-1", 1.0, 0.0, 0.0, 0.9, 0.9, "bole")


def decisions():
    return std_dec(dict(
        front=("the crown seen from the side as a row of hanging curtains in front of each other, ordered by depth, under two cap masses", "die Krone von der Seite als Reihe hintereinanderhängender Vorhänge, nach der Tiefe geordnet, unter zwei Kappenmassen"),
        trunk=("the short bole and the lateral limbs", "dem kurzen Schaft und den seitlichen Ästen"),
        plan=("the crown from above as a ring of curtain footprints round the cap masses, the limbs as thin see-through lines", "die Krone von oben als Ring aus Vorhang-Grundrissen um die Kappenmassen, die Äste als dünne Durchsichtslinien"),
        base=("the bole disc and the limb lines", "die Schaftscheibe und die Astlinien"),
        front100=("the bole, two limbs, the curtains as tall lobes and the caps", "dem Schaft, zwei Ästen, den Vorhängen als hohen Lappen und den Kappen"),
        plan100=("the ring of curtain footprints round the caps", "dem Ring der Vorhang-Grundrisse um die Kappen"),
        main200=("the main curtains and the cap masses", "den Hauptvorhängen und den Kappenmassen"),
        trunk500=("a short bole", "einem kurzen Schaft")))
