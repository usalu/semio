def build_50():
    D = Draw(B)
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 3, True, 0.25, quad(P[i], P[i + 1], HW[i], HW[i + 1]), region="culms", material="bamboo-culm")
        for k, z in enumerate(NODE_Z):
            h = hw_at(z) * 1.3
            D.poly(f"node-{n}-{k}", f"culm-{n}-{seg_bone(z)}", "texture", 4, False, 0.2, [culm_side(c, z, h), culm_side(c, z, -h)], closed=False)
        D.poly(f"groove-{n}", f"culm-{n}-b", "detail", 4, False, 0.3, [culm_side(c, 0.30, 0.0), culm_side(c, 0.52, 0.0)], closed=False)
        for k, z in enumerate([0.58, 0.64, 0.70, 0.76, 0.82, 0.88, 0.94]):
            side = 1 if (k + n) % 2 == 0 else -1
            p, a = ctl(c, z)
            sun = z >= 0.76
            Lb = (0.50 - 0.30 * (z - 0.58) / 0.4) * sc
            thb = 90 - side * (58 + 5 * (k % 3))
            tip = pol(p, thb, Lb)
            mid = pol(p, thb + side * 14, Lb * 0.5)
            reg = "leaves-sun" if sun else "leaves-shade"
            tone = 0.55 if sun else 0.72
            zz = 2 if sun else 1
            D.smooth(f"branch-{n}-{k}", f"fol-{n}", "detail", 4, False, 0.3, [p, mid, tip], closed=False)
            D.smooth(f"fan-{n}-{k}-a", f"fol-{n}", "body", zz, True, tone, fan(tip, thb - side * 50, 0.15 * sc + 0.02, 6, 110, 0.015), region=reg)
            D.smooth(f"fan-{n}-{k}-b", f"fol-{n}", "body", zz, True, tone, fan(lerp(p, tip, 0.55), thb - side * 80, 0.12 * sc + 0.02, 5, 90, 0.014), region=reg)
            D.smooth(f"fan-{n}-{k}-c", f"fol-{n}", "body", zz, True, tone, fan(lerp(p, tip, 0.8), thb + side * 24, 0.11 * sc + 0.02, 4, 70, 0.014), region=reg)
        topp, topa = ctl(c, 1.0)
        D.smooth(f"fan-{n}-top", f"fol-{n}", "body", 2, True, 0.55, fan(pol(topp, topa, -0.05), topa + 8, 0.17 * sc, 6, 100, 0.016), region="leaves-sun")
    return D.shapes


def build_100():
    D = Draw(B)
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        hw = (0.0150, 0.0140, 0.0125, 0.0112)
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 3, True, 0.25, quad(P[i], P[i + 1], hw[i], hw[i + 1]), region="culms")
        for k, z in enumerate([0.13, 0.27, 0.41, 0.55]):
            D.poly(f"node-{n}-{k}", f"culm-{n}-{seg_bone(z)}", "texture", 4, False, 0.2, [culm_side(c, z, 0.021), culm_side(c, z, -0.021)], closed=False)
        spec = [(0.62, -1, 0.34, 0.13, False), (0.72, 1, 0.34, 0.13, False), (0.82, -1, 0.32, 0.12, True), (0.90, 1, 0.30, 0.12, True)]
        for k, (z, side, rx, ry, sun) in enumerate(spec):
            p, a = ctl(c, z)
            ctr = pol(p, 90 - side * 62, 0.30 * sc)
            D.smooth(f"spray-{n}-{k}", f"fol-{n}", "body", 2 if sun else 1, True, 0.5 if sun else 0.7, ragged(ctr[0], ctr[1], rx * sc, ry * sc, -side * 22, 7, 0.28, 17 * k + 9 * n), closed=True, region="leaves-sun" if sun else "leaves-shade")
        topp, topa = ctl(c, 1.0)
        D.smooth(f"spray-{n}-top", f"fol-{n}", "body", 2, True, 0.5, ragged(topp[0], topp[1] - 0.02, 0.18 * sc, 0.2 * sc, topa - 90, 6, 0.3, 30), closed=True, region="leaves-sun")
    return D.shapes


def build_200():
    D = Draw(B)
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 3, False, 0.25, [P[i], P[i + 1]], closed=False)
        pc, ac = ctl(c, 0.70)
        D.smooth(f"mass-{n}-shade", f"fol-{n}", "body", 1, True, 0.7, blob(pc[0], pc[1], 0.40 * sc, 0.26 * sc, 10, 5, 0.11, 30 * n), region="leaves-shade")
        pu, au = ctl(c, 0.89)
        D.smooth(f"mass-{n}-sun", f"fol-{n}", "body", 2, True, 0.5, blob(pu[0], pu[1], 0.34 * sc, 0.34 * sc, au - 90, 5, 0.11, 50 * n), region="leaves-sun")
    return D.shapes


def build_500():
    D = Draw(B)
    ps = [(-0.88, 0.0), (0.74, 0.0), (1.10, 1.7), (-1.15, 1.7)]
    D.smooth("stems", "culm-3-a", "body", 2, True, 0.3, ps, closed=True, k=0.3, region="culms")
    for c in culms:
        n, H = c["n"], c["H"]
        sc = 0.8 + 0.2 * H / 4.5
        pc, ac = ctl(c, 0.78)
        D.smooth(f"crown-{n}", f"fol-{n}", "body", 3, True, 0.55, blob(pc[0], pc[1] + 0.02 * H, 0.52 * sc, 0.25 * H, ac - 90, 4, 0.10, 25 * n), regions=["leaves-sun", "leaves-shade"])
    return D.shapes
