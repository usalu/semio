"""Independent exact-integration oracle for the EN 1994 encased I-section interaction polygon (piecewise-constant widths integrated analytically, bisection on the neutral axis)."""
fyd, fcd0, fsd = 355e6, 0.85 * 30e6 / 1.5, 500e6 / 1.15
bc = hc = 0.4
h, b, tw, tf = 0.2, 0.2, 0.0085, 0.013
as_total = 4 * 314.16e-6
cover = 0.05
level = hc / 2 - cover
aa = 2 * b * tf + (h - 2 * tf) * tw
fcd = fcd0 * (bc * hc - aa - as_total) / (bc * hc - aa)


def pieces():
    out = []
    out.append((h / 2 - tf, h / 2, b, 'steel'))
    out.append((-h / 2, -h / 2 + tf, b, 'steel'))
    out.append((-h / 2 + tf, h / 2 - tf, tw, 'steel'))
    out.append((-hc / 2, -h / 2, bc, 'conc'))
    out.append((h / 2, hc / 2, bc, 'conc'))
    out.append((-h / 2, -h / 2 + tf, bc - b, 'conc'))
    out.append((h / 2 - tf, h / 2, bc - b, 'conc'))
    out.append((-h / 2 + tf, h / 2 - tf, bc - tw, 'conc'))
    return out


def resultant(yna):
    n = m = 0.0
    for y0, y1, w, kind in pieces():
        a, c = max(y0, yna), y1
        if c > a:
            f = w * (c - a) * (fyd if kind == 'steel' else fcd)
            n += f
            m += f * 0.5 * (a + c)
        if kind == 'steel':
            a, c = y0, min(y1, yna)
            if c > a:
                f = -w * (c - a) * fyd
                n += f
                m += f * 0.5 * (a + c)
    for y in (level, -level):
        f = as_total / 2 * fsd * (1 if y > yna else -1)
        n += f
        m += f * y
    return n, m


lo, hi = -hc / 2, hc / 2
for _ in range(100):
    mid = (lo + hi) / 2
    if resultant(mid)[0] > 0:
        lo = mid
    else:
        hi = mid
nb, mb = resultant((lo + hi) / 2)
nd, md = resultant(0.0)
ac = bc * hc - aa - as_total
print('N_pl', aa * fyd + ac * fcd + as_total * fsd)
print('N_pm', ac * fcd)
print('B: N=%.3f M=%.1f yna=%.5f' % (nb, mb, (lo + hi) / 2))
print('D: N=%.1f M=%.1f' % (nd, md))
