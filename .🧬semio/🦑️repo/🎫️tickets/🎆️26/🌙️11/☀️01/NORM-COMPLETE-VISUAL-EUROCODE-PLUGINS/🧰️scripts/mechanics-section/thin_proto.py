"""Python mirror of the thin-walled open-section solver (centreline walls, open tree)."""
import math

G3 = [(0.5 - math.sqrt(0.6) / 2, 5 / 18), (0.5, 8 / 18), (0.5 + math.sqrt(0.6) / 2, 5 / 18)]


def analyse(walls):
    nodes = []

    def node(p):
        for i, q in enumerate(nodes):
            if abs(p[0] - q[0]) < 1e-9 and abs(p[1] - q[1]) < 1e-9:
                return i
        nodes.append(p)
        return len(nodes) - 1

    raw = [(node((w[0], w[1])), node((w[2], w[3])), w[4]) for w in walls]
    segs = []
    for a, b, t in raw:
        d = math.hypot(nodes[b][0] - nodes[a][0], nodes[b][1] - nodes[a][1])
        cuts = []
        for k, p in enumerate(nodes):
            if k in (a, b):
                continue
            u = ((p[0] - nodes[a][0]) * (nodes[b][0] - nodes[a][0]) + (p[1] - nodes[a][1]) * (nodes[b][1] - nodes[a][1])) / (d * d)
            off = abs((nodes[b][0] - nodes[a][0]) * (p[1] - nodes[a][1]) - (nodes[b][1] - nodes[a][1]) * (p[0] - nodes[a][0])) / d
            if off < 1e-9 and 0.0 < u < 1.0:
                cuts.append((u, k))
        chain = [a] + [k for _, k in sorted(cuts)] + [b]
        segs += [(chain[i], chain[i + 1], t) for i in range(len(chain) - 1)]
    assert len(segs) == len(nodes) - 1
    ln = lambda a, b: math.hypot(nodes[b][0] - nodes[a][0], nodes[b][1] - nodes[a][1])
    A = sum(ln(a, b) * t for a, b, t in segs)
    cy = sum(ln(a, b) * t * (nodes[a][0] + nodes[b][0]) / 2 for a, b, t in segs) / A
    cz = sum(ln(a, b) * t * (nodes[a][1] + nodes[b][1]) / 2 for a, b, t in segs) / A
    P = [(p[0] - cy, p[1] - cz) for p in nodes]
    syy = szz = syz = 0.0
    for a, b, t in segs:
        l = ln(a, b)
        c, s_ = (P[b][0] - P[a][0]) / l, (P[b][1] - P[a][1]) / l
        for u, w in G3:
            y = P[a][0] + u * (P[b][0] - P[a][0])
            z = P[a][1] + u * (P[b][1] - P[a][1])
            syy += w * l * t * y * y
            szz += w * l * t * z * z
            syz += w * l * t * y * z
        i_nn = l * t ** 3 / 12
        syy += i_nn * s_ * s_
        szz += i_nn * c * c
        syz -= i_nn * c * s_
    det = syy * szz - syz * syz
    # tree order
    adj = [[] for _ in nodes]
    for si, (a, b, t) in enumerate(segs):
        adj[a].append((si, b))
        adj[b].append((si, a))
    parent = [None] * len(nodes)
    order = [0]
    seen = [False] * len(nodes)
    seen[0] = True
    om = [0.0] * len(nodes)
    walk = []  # (segment index, from, to)
    k = 0
    while k < len(order):
        v = order[k]
        k += 1
        for si, w in adj[v]:
            if not seen[w]:
                seen[w] = True
                om[w] = om[v] + P[v][0] * P[w][1] - P[v][1] * P[w][0]
                order.append(w)
                walk.append((si, v, w))
    assert all(seen)
    q_om = i_oy = i_oz = 0.0
    for si, v, w in walk:
        t = segs[si][2]
        l = ln(v, w)
        for u, wt in G3:
            y = P[v][0] + u * (P[w][0] - P[v][0])
            z = P[v][1] + u * (P[w][1] - P[v][1])
            o = om[v] + u * (om[w] - om[v])
            q_om += wt * l * t * o
            i_oy += wt * l * t * o * y
            i_oz += wt * l * t * o * z
    mean = q_om / A
    a_s = (syy * i_oz - syz * i_oy) / det
    b_s = (syz * i_oz - szz * i_oy) / det
    iw = 0.0
    for si, v, w in walk:
        t = segs[si][2]
        l = ln(v, w)
        for u, wt in G3:
            y = P[v][0] + u * (P[w][0] - P[v][0])
            z = P[v][1] + u * (P[w][1] - P[v][1])
            o = om[v] + u * (om[w] - om[v]) - mean + b_s * y - a_s * z
            iw += wt * l * t * o * o
    j = sum(ln(a, b) * t ** 3 / 3 for a, b, t in segs)
    # shear flow
    sub_y = [0.0] * len(nodes)
    sub_z = [0.0] * len(nodes)
    for si, v, w in reversed(walk):
        t = segs[si][2]
        l = ln(v, w)
        sub_y[v] += sub_y[w] + t * l * (P[v][1 - 1] + P[w][0]) / 2
        sub_z[v] += sub_z[w] + t * l * (P[v][1] + P[w][1]) / 2
    e_z = e_y = 0.0
    for si, v, w in walk:
        t = segs[si][2]
        l = ln(v, w)
        for u, wt in G3:
            my = sub_y[w] + t * l * (P[v][0] * (1 - u) + (P[w][0] - P[v][0]) * (1 - u * u) / 2)
            mz = sub_z[w] + t * l * (P[v][1] * (1 - u) + (P[w][1] - P[v][1]) * (1 - u * u) / 2)
            qz = (syy * mz - syz * my) / det
            qy = (szz * my - syz * mz) / det
            e_z += wt * l * qz * qz / t
            e_y += wt * l * qy * qy / t
    return dict(area=A, c=(cy, cz), ic=(szz, syy, syz), j=j, gamma=iw, sc=(a_s + cy, b_s + cz), a_s=(1 / e_y, 1 / e_z))
