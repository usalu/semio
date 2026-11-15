import math

A = (4.0, 0.0)
B = (0.0, 2.88)
_dx, _dy = B[0] - A[0], B[1] - A[1]
LEN = math.hypot(_dx, _dy)
D = (_dx / LEN, _dy / LEN)
N = (-D[1], D[0])


def along(p, s):
    return (p[0] + D[0] * s, p[1] + D[1] * s)


E_END = along(A, 0.19)
P2 = along(A, 0.23)
F_END = along(B, -0.06)
P_WALL = along(B, -0.10)
TUBE_LEN = LEN - 0.19 - 0.06
R_TUBE = 0.03015


def disc(cx, cy, r, n=20):
    return [[cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)] for k in range(n)]


def hull(pts):
    p = sorted(set((round(a, 6), round(b, 6)) for a, b in pts))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lo = []
    for q in p:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 0:
            lo.pop()
        lo.append(q)
    up = []
    for q in reversed(p):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 0:
            up.pop()
        up.append(q)
    return [list(q) for q in lo[:-1] + up[:-1]]


CHEEK_BEAM = hull(disc(A[0], A[1], 0.06) + disc(P2[0], P2[1], 0.04))
CHEEK_WALL = hull([(0.0, 2.78), (0.0, 2.98)] + disc(P_WALL[0], P_WALL[1], 0.04))
