"""Independent oracle for the EN 1994 continuous-beam solver (flexibility method with cracked EI2 zones, cl. 5.4.2.3) against PyNite.

Run: /home/deck/semio/.venv/bin/python -I en1994-continuous-oracle.py
Prints the reference values hard-coded in the Rust tests of the catalogue module `continuous`.
"""
import numpy as np
from Pynite import FEModel3D

E = 210e9


def ei_at(x_local, l, ei1, ei2, frac, left_internal, right_internal):
    if left_internal and x_local < frac * l - 1e-12:
        return ei2
    if right_internal and x_local > l - frac * l + 1e-12:
        return ei2
    return ei1


def solve(spans, qs, ei1, ei2, frac, cantilever=False, n=400):
    k = len(spans)
    left_int = [i > 0 for i in range(k)]
    right_int = [i < k - 1 for i in range(k)]
    if cantilever:
        right_int[k - 2] = True
        right_int[k - 1] = False
    unknown = [i for i in range(1, k) if not (cantilever and i == k - 1)]

    def span_integrals(i, m_left, m_right):
        l = spans[i]
        h = l / n
        xs = (np.arange(n) + 0.5) * h
        m = qs[i] * xs * (l - xs) / 2 + m_left * (1 - xs / l) + m_right * xs / l
        ei = np.array([ei_at(x, l, ei1, ei2, frac, left_int[i], right_int[i]) for x in xs])
        kap = m / ei
        theta0 = -np.sum((l - xs) * kap) * h / l
        theta1 = theta0 + np.sum(kap) * h
        return theta0, theta1, xs, kap, h

    def residual(ms):
        sup = [0.0] * (k + 1)
        for idx, i in enumerate(unknown):
            sup[i] = ms[idx]
        if cantilever:
            sup[k - 1] = -qs[k - 1] * spans[k - 1] ** 2 / 2
        r = []
        for i in unknown:
            t_right = span_integrals(i - 1, sup[i - 1], sup[i])[1]
            t_left = span_integrals(i, sup[i], sup[i + 1])[0]
            r.append(t_right - t_left)
        return np.array(r), sup

    m = len(unknown)
    r0, _ = residual(np.zeros(m))
    a = np.zeros((m, m))
    for j in range(m):
        e = np.zeros(m)
        e[j] = 1.0
        a[:, j] = residual(e)[0] - r0
    ms = np.linalg.solve(a, -r0) if m else np.zeros(0)
    _, sup = residual(ms)
    return sup


def deflection(spans, qs, sup, ei1, ei2, frac, cantilever=False, n=400):
    k = len(spans)
    left_int = [i > 0 for i in range(k)]
    right_int = [i < k - 1 for i in range(k)]
    if cantilever:
        right_int[k - 2] = True
        right_int[k - 1] = False
    out = []
    slope_in = 0.0
    for i in range(k):
        l = spans[i]
        h = l / n
        xs = (np.arange(n) + 0.5) * h
        m = qs[i] * xs * (l - xs) / 2 + sup[i] * (1 - xs / l) + sup[i + 1] * xs / l
        ei = np.array([ei_at(x, l, ei1, ei2, frac, left_int[i], right_int[i]) for x in xs])
        kap = m / ei
        is_cant = cantilever and i == k - 1
        theta0 = slope_in if (is_cant) else -np.sum((l - xs) * kap) * h / l
        pts = np.linspace(0, l, 201)
        d = []
        for x in pts:
            mask = xs < x
            d.append(theta0 * x + np.sum((x - xs[mask]) * kap[mask]) * h)
        out.append((pts, np.array(d)))
        slope_in = theta0 + np.sum(kap) * h
    return out


def pynite(spans, qs, ei1, ei2, frac, cantilever=False):
    fe = FEModel3D()
    fe.add_material('s', E, 80e9, 0.3, 7850)
    x = 0.0
    nodes = [0.0]
    for l in spans:
        x += l
        nodes.append(x)
    for i, xn in enumerate(nodes):
        fe.add_node(f'N{i}', xn, 0, 0)
    k = len(spans)
    for i in range(k):
        l = spans[i]
        left_i = i > 0
        right_i = (i < k - 1) and not (cantilever and i == k - 1)
        if cantilever and i == k - 2:
            right_i = True
        cuts = [0.0, l]
        if left_i:
            cuts.insert(1, frac * l)
        if right_i:
            cuts.insert(len(cuts) - 1, l - frac * l)
        prev = f'N{i}'
        for j in range(len(cuts) - 1):
            a, b = cuts[j], cuts[j + 1]
            mid = 0.5 * (a + b)
            ei = ei_at(mid, l, ei1, ei2, frac, left_i, right_i)
            sec = f's{i}_{j}'
            fe.add_section(sec, 1e-2, 1e-4, ei / E, 1e-4)
            nxt = f'N{i + 1}' if j == len(cuts) - 2 else f'M{i}_{j}'
            if j != len(cuts) - 2:
                fe.add_node(nxt, nodes[i] + b, 0, 0)
            name = f'E{i}_{j}'
            fe.add_member(name, prev, nxt, 's', sec)
            fe.add_member_dist_load(name, 'Fy', -qs[i], -qs[i])
            prev = nxt
    for i in range(len(nodes)):
        free = cantilever and i == len(nodes) - 1
        fe.def_support(f'N{i}', i == 0, not free, True, True, False, False)
    fe.analyze(check_statics=False)
    return fe, nodes


if __name__ == '__main__':
    q = 10e3
    L = 8.0
    ei1 = E * 6e-4
    ei2 = 0.5 * ei1
    cases = {
        'two-equal-uncracked': dict(spans=[L, L], qs=[q, q], frac=0.15, ei2=ei1),
        'two-equal-cracked': dict(spans=[L, L], qs=[q, q], frac=0.15, ei2=ei2),
        'end-unequal-uncracked': dict(spans=[8.0, 6.0], qs=[q, q], frac=0.15, ei2=ei1),
        'end-unequal-cracked': dict(spans=[8.0, 6.0], qs=[q, q], frac=0.15, ei2=ei2),
        'interior-uncracked': dict(spans=[6.0, 8.0, 6.0], qs=[q, q, q], frac=0.15, ei2=ei1),
        'interior-cracked': dict(spans=[6.0, 8.0, 6.0], qs=[q, q, q], frac=0.15, ei2=ei2),
        'cantilever-uncracked': dict(spans=[8.0, 3.0], qs=[q, q], frac=0.15, ei2=ei1, cantilever=True),
        'cantilever-cracked': dict(spans=[8.0, 3.0], qs=[q, q], frac=0.15, ei2=ei2, cantilever=True),
        'cantilever-pattern-cracked': dict(spans=[8.0, 3.0], qs=[0.0, q], frac=0.15, ei2=ei2, cantilever=True),
        'cantilever-pattern-uncracked': dict(spans=[8.0, 3.0], qs=[0.0, q], frac=0.15, ei2=ei1, cantilever=True),
        'end-pattern-cracked': dict(spans=[8.0, 8.0], qs=[q, 0.0], frac=0.15, ei2=ei2),
    }
    for name, c in cases.items():
        cant = c.get('cantilever', False)
        sup = solve(c['spans'], c['qs'], ei1, c['ei2'], c['frac'], cant)
        defl = deflection(c['spans'], c['qs'], sup, ei1, c['ei2'], c['frac'], cant)
        own = len(c['spans']) - 1 if cant else (1 if len(c['spans']) == 3 else 0)
        d_own = defl[own][1]
        peak = d_own[int(np.argmax(np.abs(d_own)))]
        fe, nodes = pynite(c['spans'], c['qs'], ei1, c['ei2'], c['frac'], cant)
        pn_peak = 0.0
        pn_sup = []
        for mname, mem in fe.members.items():
            if not mname.startswith(f'E{own}_'):
                continue
            hi, lo = -mem.min_deflection('dy', 'Combo 1'), -mem.max_deflection('dy', 'Combo 1')
            pn_peak = max([pn_peak, hi, lo], key=abs)
        for i2 in range(1, len(c['spans'])):
            last = max(int(n.split('_')[1]) for n in fe.members if n.startswith(f'E{i2 - 1}_'))
            pn_sup.append(round(fe.members[f'E{i2 - 1}_{last}'].moment('Mz', fe.members[f'E{i2 - 1}_{last}'].L(), 'Combo 1'), 1))
        print(f'  PyNite: peak_mm={pn_peak * 1e3:.4f} support_moment(Nm)={pn_sup}')
        print(f'{name}: supports(Nm)={[round(s, 1) for s in sup]} peak_own_span_mm={peak * 1e3:.4f}')
