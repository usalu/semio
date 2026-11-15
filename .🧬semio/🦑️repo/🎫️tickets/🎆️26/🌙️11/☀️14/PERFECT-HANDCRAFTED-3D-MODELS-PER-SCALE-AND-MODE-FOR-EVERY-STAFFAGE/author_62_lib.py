"""Author 62: shared builders for the vehicle polish pass (wheels as loops, hulls with tumblehome, thermal layer shells)."""
import copy, json, math, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import author_32_lib as L32
from author_32_kit import *

T = L32.T
G = os.path.join(T, '🗑️generated', 'author-62')
os.makedirs(G, exist_ok=True)
L32.G = G


def save(asset, d):
    p = src(asset)
    cur = json.load(open(p))
    for s, v in d['volumes'].items():
        cur['volumes'][s] = v
    tmp = p + '.a62.tmp'
    json.dump(cur, open(tmp, 'w'))
    os.replace(tmp, p)


def quat_x(phi):
    return [rr(math.sin(phi / 2)), 0, 0, rr(math.cos(phi / 2))]


def tyre_loop(w, r_out, r_bead, r_seat, shoulder=0.12, side=0.12):
    """closed loop profile (axial, radius) of a tyre: flat sidewalls from the bead seat to the shoulder, rounded shoulders, flat tread"""
    h = w / 2
    sw = r_out - 0.16 * (r_out - r_seat)
    return [[-h, r_seat + 0.02], [-h, sw], [-h + 0.07 * w, r_out - 0.04 * (r_out - r_seat)], [-h + 0.17 * w, r_out], [h - 0.17 * w, r_out],
            [h - 0.07 * w, r_out - 0.04 * (r_out - r_seat)], [h, sw], [h, r_seat + 0.02], [h - 0.04 * w, r_seat], [-h + 0.04 * w, r_seat], [-h, r_seat + 0.02]]


def annulus_loop(a0, a1, r0, r1):
    return [[a0, r0], [a1, r0], [a1, r1], [a0, r1], [a0, r0]]


def dished_rim(side, w, r_rim, r_dish, recess, hub_r):
    """open profile touching the axis at both ends: a rim with a recessed centre disc, outer side = +axial for side=+1 (left), -axial for side=-1"""
    h = w / 2
    pts = [[-h, 0.0], [-h, hub_r], [-h + recess, hub_r + 0.01], [-h + recess, r_dish], [-h * 0.55, r_rim * 0.93], [-h * 0.55, r_rim], [h, r_rim], [h, 0.0]]
    if side < 0:
        pts = [[-a, r] for a, r in pts]
        pts = pts
    return pts


def solid_by_id(lst, i):
    return next(s for s in lst if s['id'] == i)


def replace(lst, i, new):
    for k, s in enumerate(lst):
        if s['id'] == i:
            lst[k] = new
            return
    raise KeyError(i)


def vloft(id, bone, mat, role, tone, rings, e=20, caps=True):
    """loft along the vertical (path along v): rings = (y, x_centre, half_length, half_width); rx is u (length), ry is w (width)"""
    secs = [{"at": [rr(cx), rr(y), 0], "rx": rr(a), "ry": rr(b), "exponent": e, "rotation": [0.70711, 0, 0, 0.70711]} for (y, cx, a, b) in rings]
    return loft(id, bone, mat, role, tone, secs, caps)


def xsec(x, bottom, top, half_w, e=24):
    return {"at": [rr(x), rr((bottom + top) / 2), 0], "rx": rr(half_w), "ry": rr((top - bottom) / 2), "exponent": e, "rotation": [0.70711, 0, 0.70711, 0]}


def corner(b, r, x):
    """half width at distance x from an end of a plan outline with corner radius r"""
    if x >= r:
        return b
    return b - (r - math.sqrt(max(r * r - (r - x) ** 2, 0)))


def checkq(asset, scale, fast=False):
    a = ['volumes-check.ts', asset, '--scale', str(scale)]
    a += ['--fast'] if fast else ['--product']
    out = L32.run(a)
    lines = [l for l in out.splitlines() if l.startswith(('FAIL', 'ok  '))]
    for l in lines:
        if l.startswith('FAIL'):
            l = re.sub(r' info:\S+', '', l)
            print(l[:900])
    tail = [l for l in out.splitlines() if l.startswith(('FAIL:', 'PASS'))]
    print(tail[-1:] or out[-400:])
    return out


def render(asset, scale):
    out = L32.run(['volumes-render.ts', asset, '--scale', str(scale)])
    print(out.strip()[-200:])
