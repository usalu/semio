"""🦾 Z-bar loader linkage of the wheel loader: the pin geometry stated by hand and the closed-loop solution (lift cylinder, bell crank, tilt link, tilt cylinder) for a lift-arm angle and a bucket angle. Imported by the other r3-wheel-loader scripts; run it to print the pose table."""
import math

WB = 3.35
AX = WB / 2
TYRE_R = 0.8075
A = (1.06, 2.30)
L = 2.74
E = (0.45, 0.90)
G_ARM = (0.95, -0.08)
F_ARM = (1.90, 0.50)
R1 = 0.70
R2 = 0.80
BEND = 205.0
LINK = 0.90
K_BKT = (0.06, 0.40)
C0 = (0.70, 1.95)
TIP_BKT = (1.47, -0.42)


def rot(p, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1])


def ang(v):
    return math.degrees(math.atan2(v[1], v[0]))


def norm(v):
    return math.hypot(v[0], v[1])


def solve(theta, beta):
    """🧷 World pins for the arm angle theta and the world bucket angle beta (degrees, counter-clockwise, 0 = floor level)."""
    arm = lambda p: add(A, rot(p, theta))
    B = arm((L, 0))
    K = add(B, rot(K_BKT, beta))
    F = arm(F_ARM)
    G = arm(G_ARM)
    d = sub(K, F)
    dist = norm(d)
    if dist > R2 + LINK or dist < abs(R2 - LINK):
        return None
    a = (R2 * R2 - LINK * LINK + dist * dist) / (2 * dist)
    h = math.sqrt(max(0.0, R2 * R2 - a * a))
    ex = (d[0] / dist, d[1] / dist)
    J1 = (F[0] + a * ex[0] + h * ex[1], F[1] + a * ex[1] - h * ex[0])
    J2 = (F[0] + a * ex[0] - h * ex[1], F[1] + a * ex[1] + h * ex[0])
    J = J1 if rot(sub(J1, A), -theta)[1] < rot(sub(J2, A), -theta)[1] else J2
    lever = ang(sub(J, F))
    H = add(F, rot((R1, 0), lever + BEND))
    tj = abs((ang(sub(K, J)) - ang(sub(F, J)) + 180) % 360 - 180)
    tk = abs((ang(sub(J, K)) - ang(sub(B, K)) + 180) % 360 - 180)
    return dict(theta=theta, beta=beta, B=B, K=K, F=F, G=G, J=J, H=H, lever=lever, link=ang(sub(K, J)), lift=norm(sub(G, E)), liftAng=ang(sub(G, E)), tilt=norm(sub(H, C0)), tiltAng=ang(sub(H, C0)), tj=tj, tk=tk, tip=add(B, rot(TIP_BKT, beta)))


POSES = {"carry": (-36.5, 45.0), "dig": (-39.7885, -5.0), "dump-high": (45.0, -45.0), "parked-off": (-43.3246, 0.0), "raising": (5.0, 50.0)}
KEYS = {"crowd": (-42.0, 25.0), "rolled": (-36.5, 48.0), "top": (45.0, 55.0), "lowering": (-25.0, 10.0)}

if __name__ == "__main__":
    for name, (t, b) in POSES.items():
        s = solve(t, b)
        if s is None:
            print(name, "UNREACHABLE")
            continue
        print("%-13s th %6.1f be %6.1f B (%.3f %.3f) tip (%.3f %.3f) lift %.3f tilt %.3f lever %7.1f link %7.1f tj %5.1f tk %5.1f H (%.2f %.2f) J (%.2f %.2f) K (%.2f %.2f)" % (name, t, b, *s["B"], *s["tip"], s["lift"], s["tilt"], s["lever"], s["link"], s["tj"], s["tk"], *s["H"], *s["J"], *s["K"]))
