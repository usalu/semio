"""🔄️ Bone rotations of the staffage rig: R(angle, abduction, twist) = Rw(angle)·Rv(abduction)·Ru(twist) with columns u, v, w (see README "Rest placement of a bone")."""
import math


def mat(cols):
    return [[cols[j][i] for j in range(3)] for i in range(3)]


def mul(A, B):
    return [[sum(A[i][k] * B[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def T(A):
    return [[A[j][i] for j in range(3)] for i in range(3)]


def col(A, j):
    return [A[0][j], A[1][j], A[2][j]]


def Rw(a):
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return mat([[c, s, 0], [-s, c, 0], [0, 0, 1]])


def Rv(b):
    c, s = math.cos(math.radians(b)), math.sin(math.radians(b))
    return mat([[c, 0, s], [0, 1, 0], [-s, 0, c]])


def Ru(t):
    c, s = math.cos(math.radians(t)), math.sin(math.radians(t))
    return mat([[1, 0, 0], [0, c, -s], [0, s, c]])


def R(a=0, b=0, t=0):
    return mul(mul(Rw(a), Rv(b)), Ru(t))


def frame(fr):
    """The 3x3 matrix with columns u, v, w of an fk frame dict."""
    return mat([[fr[k]["x"], fr[k]["y"], fr[k]["z"]] for k in ("u", "v", "w")])


def decompose(M):
    """Angles (a, b, t) with Rw(a)Rv(b)Ru(t) = M."""
    u = col(M, 0)
    b = math.degrees(math.asin(max(-1, min(1, u[2]))))
    a = math.degrees(math.atan2(u[1], u[0]))
    N = mul(T(mul(Rw(a), Rv(b))), M)
    t = math.degrees(math.atan2(-N[2][1], N[1][1]))
    return a, b, t
