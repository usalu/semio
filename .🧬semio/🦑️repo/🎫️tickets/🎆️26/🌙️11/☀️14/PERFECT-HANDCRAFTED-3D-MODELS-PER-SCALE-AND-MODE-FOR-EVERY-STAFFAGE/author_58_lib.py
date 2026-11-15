"""Author 58: shared two-wheeler helpers on the author-43 kit (bone-local construction, thermal by region, structure parts)."""
import os, sys, math, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_43_lib import *

GEN = os.path.join(T, "🗑️generated", "author-58")
os.makedirs(GEN, exist_ok=True)


def cl(id, bone, mat, tone, a, b, r, r1=None, role="body", **kw):
    s = capsule(id, bone, mat, tone, a, b, r, r1, role=role)
    s.update(kw)
    return s


def tubel(id, bone, mat, tone, a, b, ro, ri=None, role="body", **kw):
    d = sub3(b, a)
    L = math.sqrt(dot(d, d))
    ax = [x / L for x in d]
    prof = [[0, ri], [L, ri], [L, ro], [0, ro], [0, ri]] if ri else [[0, 0], [0, ro], [L, ro], [L, 0], [0, 0]]
    s = revolve(id, bone, mat, tone, ax, a, prof, role=role)
    s.update(kw)
    return s


def oval(id, bone, mat, tone, a, b, rx, ry, e=2, lat=(0, 0, 1), n=2, role="body", caps=True, **kw):
    """Straight oval/rounded-rect bar from a to b (bone-local), rx along `lat` (lateral), ry the other in-plane normal."""
    pts = [[a[i] + (b[i] - a[i]) * k / (n - 1) for i in range(3)] for k in range(n)]
    secs = []
    for i, c in enumerate(pts):
        t = norm3(sub3(pts[min(n - 1, i + 1)], pts[max(0, i - 1)]))
        ex = norm3(sub3(list(lat), mul3(t, dot(list(lat), t))))
        ey = cross3(t, ex)
        secs.append(sec(c, rx, ry, exponent=e if e != 2 else None, rotation=basis_quat(ex, ey, t)))
    s = loft(id, bone, mat, tone, secs, role=role, capStart=caps, capEnd=caps)
    s.update(kw)
    return s


def path_sweep(id, bone, mat, tone, pts, rx, ry=None, e=2, lat=(0, 0, 1), role="body", caps=True, step=0.02, **kw):
    pts = resample(pts, step) if step else pts
    n = len(pts)
    secs = []
    for i, c in enumerate(pts):
        t = norm3(sub3(pts[min(n - 1, i + 1)], pts[max(0, i - 1)]))
        ex = norm3(sub3(list(lat), mul3(t, dot(list(lat), t))))
        ey = cross3(t, ex)
        a = rx(i / (n - 1)) if callable(rx) else rx
        b = (ry(i / (n - 1)) if callable(ry) else ry) if ry is not None else a
        secs.append(sec(c, a, b, exponent=e if e != 2 else None, rotation=basis_quat(ex, ey, t)))
    s = loft(id, bone, mat, tone, secs, role=role, capStart=caps, capEnd=caps)
    s.update(kw)
    return s


def rbar_hollow(id, bone, mat, tone, a, b, hw, hh, wall, role="body"):
    """Hollow rectangular tube between two bone-local points (x-y plane bar, rotated about z), lateral half width hw, half height hh."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy)
    ang = math.degrees(math.atan2(dy, dx))
    outer = [[-hh, -hw], [hh, -hw], [hh, hw], [-hh, hw]]
    inner = [[-hh + wall, -hw + wall], [hh - wall, -hw + wall], [hh - wall, hw - wall], [-hh + wall, hw - wall]]
    d = ext(id, bone, mat, tone, "yz", outer, 0, L, holes=[inner], role=role)
    d["transform"] = {"translation": vec(list(a) + [0.0] * (3 - len(a))), "rotation": qaxis([0, 0, 1], ang)}
    return d


def rot_place(d, at, deg, axis=(0, 0, 1)):
    d["transform"] = {"translation": vec(at), "rotation": qaxis(list(axis), deg)}
    return d


def annulus(id, bone, mat, tone, r_in, r_out, z0, z1, role="detail", **kw):
    s = revolve(id, bone, mat, tone, "z", [0, 0, 0], [[z0, r_in], [z1, r_in], [z1, r_out], [z0, r_out], [z0, r_in]], role=role)
    s.update(kw)
    return s


def thermal_by(B, rule, layers=None, shell=()):
    """thermal list: every body solid for which rule(id) returns a region gets a reference; regions in `shell` (with layers) get a nested shell for the inner layer."""
    for s in B.body:
        reg = rule(s["id"])
        if reg is None:
            continue
        B.thof(f"th-{s['id']}", s["id"], reg)


def dec(B, texts):
    for k in range(0, len(texts), 2):
        B.dec(texts[k], texts[k + 1])


def saddle_loft(P, bone, xs, bot, top, hw, e=2.6):
    secs = []
    for x in xs:
        secs.append(sec([x, (bot[x] + top[x]) / 2, 0], hw[x], (top[x] - bot[x]) / 2, exponent=e, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])))
    return loft("saddle", bone, "plastic", 0.2, secs)


def save_scale(slug, den, B):
    return write_scale(slug, den, B)


def check(slug, den, extra=()):
    out = run_check(slug, den, None, extra)
    open(os.path.join(GEN, f"{slug}.s{den}.check.txt"), "w").write(out)
    return out


def render(slug, den):
    bun = subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1", shell=True, text=True).strip()
    r = subprocess.run([bun, os.path.join(T, "volumes-render.ts"), "vehicles/" + slug, "--scale", str(den)], cwd=T, capture_output=True, text=True)
    return r.stdout + r.stderr


def set_areas(slug, values):
    """Text-level: sets areaM2 of thermal regions by id (the areaM2 lines sit in region order); returns {id: (old, new)}."""
    import re as _re
    path = CATV + slug + "/🔣️.json"
    L = open(path).read().split("\n")
    a = json.load(open(path))["asset"]
    ids = [r["id"] for r in a["thermal"]["regions"]]
    idx = [i for i, l in enumerate(L) if _re.match(r'^\s*"areaM2": ', l)]
    cand = idx[-len(ids):] if len(idx) >= len(ids) else idx
    assert len(cand) == len(ids), (len(idx), len(ids))
    out = {}
    for rid, v in values.items():
        i = cand[ids.index(rid)]
        old = float(_re.search(r'"areaM2": ([-\d.eE+]+)', L[i]).group(1))
        L[i] = _re.sub(r'("areaM2": )[-\d.eE+]+', lambda m: m.group(1) + repr(float("%.3g" % v)), L[i])
        out[rid] = (old, v)
    txt = "\n".join(L)
    json.loads(txt)
    open(path, "w").write(txt)
    return out
