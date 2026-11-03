"""EN 1999 v4 reference for the free-outline stability: mirror of Rust `free_outline_stability` (principal frame, cubic moment k, z_j, M_cr, N_cr,TF) checked against the sectionproperties Wagner coefficients and shear centre. Usage: /home/deck/semio/.venv/bin/python -I wagner_ref.py"""
import math
import numpy as np
from shapely.geometry import Polygon
from sectionproperties.pre.geometry import Geometry
from sectionproperties.analysis import Section

E, G = 70000e6, 27000e6


def ring_props(pts):
    a = 0.0; sy = 0.0; sz = 0.0; izz = 0.0; iyy = 0.0; iyz = 0.0
    n = len(pts)
    for i in range(n):
        p, q = pts[i], pts[(i + 1) % n]
        c = p[0] * q[1] - q[0] * p[1]
        a += c / 2; sy += (p[0] + q[0]) * c / 6; sz += (p[1] + q[1]) * c / 6
        iyy += (p[1]**2 + p[1] * q[1] + q[1]**2) * c / 12
        izz += (p[0]**2 + p[0] * q[0] + q[0]**2) * c / 12
        iyz += (p[0] * q[1] + 2 * p[0] * p[1] + 2 * q[0] * q[1] + q[0] * p[1]) * c / 24
    s = 1 if a > 0 else -1
    return s * a, s * sy, s * sz, s * iyy, s * izz, s * iyz


def cubic(pts):
    tot = 0.0; area = 0.0
    f = lambda y, z: z * (y * y + z * z)
    n = len(pts)
    for i in range(n):
        p, q = pts[i], pts[(i + 1) % n]
        s = (p[0] * q[1] - q[0] * p[1]) / 2
        c = ((p[0] + q[0]) / 3, (p[1] + q[1]) / 3)
        o = (0.2 * (p[0] + q[0]), 0.2 * (p[1] + q[1]))
        a = (0.6 * p[0] + 0.2 * q[0], 0.6 * p[1] + 0.2 * q[1])
        b = (0.6 * q[0] + 0.2 * p[0], 0.6 * q[1] + 0.2 * p[1])
        tot += s * (-27 / 48 * f(*c) + 25 / 48 * (f(*o) + f(*a) + f(*b)))
        area += s
    return -tot if area < 0 else tot


def frame(outer, holes, ysc, zsc, theta):
    A, sy, sz, iyy, izz, iyz = ring_props(outer)
    for h in holes:
        r = ring_props(h); A -= r[0]; sy -= r[1]; sz -= r[2]; iyy -= r[3]; izz -= r[4]; iyz -= r[5]
    yg, zg = sy / A, sz / A
    Iy, Iz, Iyz = iyy - A * zg**2, izz - A * yg**2, iyz - A * yg * zg
    c, s = math.cos(theta), math.sin(theta)
    mp = lambda ring: [((p[0] - yg) * c + (p[1] - zg) * s, -(p[0] - yg) * s + (p[1] - zg) * c) for p in ring]
    k = cubic(mp(outer)) - sum(cubic(mp(h)) for h in holes)
    return dict(A=A, yg=yg, zg=zg, Iy=Iy, Iz=Iz, Iyz=Iyz,
                iy=Iy * c * c + Iz * s * s - 2 * Iyz * s * c, iz=Iz * c * c + Iy * s * s + 2 * Iyz * s * c,
                y0=ysc * c + zsc * s, z0=-ysc * s + zsc * c, k=k)


def principal_angle(Iy, Iz, Iyz):
    th = 0.5 * math.atan2(-2 * Iyz, Iy - Iz)
    return th - math.pi / 2 if th > math.pi / 4 else th + math.pi / 2 if th < -math.pi / 4 else th


def zj(f, up=True):
    v = f["z0"] - 0.5 * f["k"] / f["iy"]
    return v if up else -v


def mcr(c1, f, it, iw, l, up=True):
    z = zj(f, up)
    return c1 * math.pi**2 * E * f["iz"] / l**2 * (math.sqrt(iw / f["iz"] + l * l * G * it / (math.pi**2 * E * f["iz"]) + z * z) + z)


def ncr_tf(f, a, it, iw, lz, ly, lt):
    i0 = (f["iy"] + f["iz"]) / a + f["y0"]**2 + f["z0"]**2
    pu, pv = math.pi**2 * E * f["iz"] / lz**2, math.pi**2 * E * f["iy"] / ly**2
    pt = (G * it + math.pi**2 * E * iw / lt**2) / i0
    fn = lambda n: i0 * (n - pu) * (n - pv) * (n - pt) - n * n * f["z0"]**2 * (n - pv) - n * n * f["y0"]**2 * (n - pu)
    lo, hi = 0.0, min(pu, pv, pt)
    for _ in range(200):
        m = 0.5 * (lo + hi)
        lo, hi = (m, hi) if fn(m) < 0 else (lo, m)
    return pt, 0.5 * (lo + hi)


def sp(outer, holes, mesh=2e-6):
    g = Geometry(geom=Polygon(outer, holes))
    g.create_mesh(mesh_sizes=[mesh])
    s = Section(g)
    s.calculate_geometric_properties(); s.calculate_warping_properties()
    return s


def tee(b=0.1, tf=0.01, h=0.1, tw=0.01):
    return [(-tw / 2, 0), (tw / 2, 0), (tw / 2, h), (b / 2, h), (b / 2, h + tf), (-b / 2, h + tf), (-b / 2, h), (-tw / 2, h)]


def zsec(b=0.06, h=0.12, t=0.008):
    return [(0, 0), (b, 0), (b, t), (t, t), (t, h - t), (-b + t, h - t) if False else (t, h), (-b, h), (-b, h - t), (0, h - t)]


def angle(a=0.1, b=0.06, t=0.008):
    return [(0, 0), (a, 0), (a, t), (t, t), (t, b), (0, b)]


if __name__ == "__main__":
    for name, outer in (("tee", tee()), ("angle", angle()), ("tee-flipped", [(y, -z) for y, z in tee()])):
        s = sp(outer, [])
        ysc, zsc = s.get_sc()[0] - s.get_c()[0], s.get_sc()[1] - s.get_c()[1]
        it, iw = s.get_j(), s.get_gamma()
        base = frame(outer, [], ysc, zsc, 0.0)
        th = principal_angle(base["Iy"], base["Iz"], base["Iyz"])
        near = frame(outer, [], ysc, zsc, th)
        strong = near if near["iy"] >= near["iz"] else frame(outer, [], ysc, zsc, th + math.pi / 2)
        bp = s.get_beta_p()
        print(name, "theta_deg", round(math.degrees(th), 3), "I1,I2 sp", [round(v, 9) for v in s.get_ip()], "mine", round(strong["iy"], 9), round(strong["iz"], 9))
        print("  beta_p sp", [round(v, 6) for v in bp], " mine beta_up=-2zj", round(-2 * zj(strong, True), 6), "down", round(-2 * zj(strong, False), 6))
        print("  Mcr(1m? L=3) up/down", mcr(1, strong, it, iw, 3.0, True), mcr(1, strong, it, iw, 3.0, False))
        print("  Ncr T, TF", ncr_tf(near, near["A"], it, iw, 3.0, 3.0, 3.0))


def ncr_eigen(f, a, it, iw, lz, ly, lt):
    """3x3 generalized eigenproblem of the sine-mode (u, v, phi) system with scipy, independent of the cubic."""
    from scipy.linalg import eigh
    i0 = (f["iy"] + f["iz"]) / a + f["y0"]**2 + f["z0"]**2
    pu, pv = math.pi**2 * E * f["iz"] / lz**2, math.pi**2 * E * f["iy"] / ly**2
    pt = (G * it + math.pi**2 * E * iw / lt**2) / i0
    K = np.diag([pu, pv, pt * i0])
    Gm = np.array([[1, 0, f["z0"]], [0, 1, -f["y0"]], [f["z0"], -f["y0"], i0]])
    return eigh(K, Gm, eigvals_only=True)[0]


def selftest():
    for name, outer in (("tee", tee()), ("angle", angle())):
        s = sp(outer, [])
        ysc, zsc = s.get_sc()[0] - s.get_c()[0], s.get_sc()[1] - s.get_c()[1]
        it, iw = s.get_j(), s.get_gamma()
        base = frame(outer, [], ysc, zsc, 0.0)
        near = frame(outer, [], ysc, zsc, principal_angle(base["Iy"], base["Iz"], base["Iyz"]))
        for ls in ((3.0, 3.0, 3.0), (1.2, 2.5, 4.0)):
            _, tf = ncr_tf(near, near["A"], it, iw, *ls)
            e = ncr_eigen(near, near["A"], it, iw, *ls)
            assert abs(tf / e - 1) < 1e-9, (name, ls, tf, e)
        bp = s.get_beta_p()
        strong = near if near["iy"] >= near["iz"] else frame(outer, [], ysc, zsc, principal_angle(base["Iy"], base["Iz"], base["Iyz"]) + math.pi / 2)
        assert strong and sorted([-2 * zj(strong, True), -2 * zj(strong, False)]) == sorted(bp[:2]) or max(abs(x - y) for x, y in zip(sorted([-2 * zj(strong, True), -2 * zj(strong, False)]), sorted(bp[:2]))) < 1e-5, name
    print("selftest ok")


if __name__ == "__main__":
    selftest()
