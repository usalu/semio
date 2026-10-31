import math
def band(p, q, w):
    dx, dy = q[0] - p[0], q[1] - p[1]
    l = math.hypot(dx, dy)
    nx, ny = -dy / l * w / 2, dx / l * w / 2
    return [(p[0] + nx, p[1] + ny), (q[0] + nx, q[1] + ny), (q[0] - nx, q[1] - ny), (p[0] - nx, p[1] - ny)]
def rot(p, c, deg):
    a = math.radians(deg); x, y = p[0] - c[0], p[1] - c[1]
    return (c[0] + x * math.cos(a) - y * math.sin(a), c[1] + x * math.sin(a) + y * math.cos(a))
def ang(p, q):
    return math.degrees(math.atan2(q[1] - p[1], q[0] - p[0]))
def circ_int(p1, a, p2, b, up=True):
    d = math.hypot(p2[0] - p1[0], p2[1] - p1[1])
    x = (d * d + a * a - b * b) / (2 * d)
    h = math.sqrt(max(a * a - x * x, 0))
    ux, uy = (p2[0] - p1[0]) / d, (p2[1] - p1[1]) / d
    bx, by = p1[0] + ux * x, p1[1] + uy * x
    c1 = (bx - uy * h, by + ux * h); c2 = (bx + uy * h, by - ux * h)
    return c1 if (c1[1] > c2[1]) == up else c2
