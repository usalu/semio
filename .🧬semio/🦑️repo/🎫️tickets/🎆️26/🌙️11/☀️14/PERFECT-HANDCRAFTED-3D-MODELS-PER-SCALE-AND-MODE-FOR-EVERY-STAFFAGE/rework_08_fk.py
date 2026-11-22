"""🦴 Planar forward kinematics of a staffage rig for hand-solving poses (rework author 08)."""
import json, math, sys, glob

CATALOG = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'


def path(slug):
    """📁 Catalog file of an animal slug."""
    return glob.glob(f'{CATALOG}/*/{slug}/🔣️.json')[0]


def load(slug):
    """📖 Catalog document of a slug."""
    return json.load(open(path(slug)))


def fk(asset, pose=None, override=None):
    """📐 Joint, tip and world angle per bone for a pose id (or rest) with optional per-bone angle overrides."""
    bones = {b['id']: b for b in asset['rig']['bones']}
    p = next((q for q in asset['poses'] if q['id'] == pose), {}) if pose else {}
    ang = {k: v.get('angleDeg', 0) for k, v in p.get('bones', {}).items()}
    ang.update(override or {})
    root = (override or {}).get('__root', p.get('root', {}).get('angleDeg', 0))
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = bones[i]
        if b['parent'] is None:
            px, py, pa = 0.0, 0.0, root
        else:
            j, t, pa = solve(b['parent'])
            px, py = j
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        jx, jy = px + c * b.get('x', 0) - s * b.get('y', 0), py + s * b.get('x', 0) + c * b.get('y', 0)
        a = pa + b['angleDeg'] + ang.get(i, 0)
        tip = (jx + math.cos(math.radians(a)) * b['lengthM'], jy + math.sin(math.radians(a)) * b['lengthM'])
        out[i] = ((jx, jy), tip, a)
        return out[i]

    for i in bones:
        solve(i)
    return out


def point(asset, frames, bone, x, y):
    """📍 World position of a bone-local point."""
    (jx, jy), _, a = frames[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (jx + c * x - s * y, jy + s * x + c * y)


if __name__ == '__main__':
    a = load(sys.argv[1])['asset']
    f = fk(a, sys.argv[2] if len(sys.argv) > 2 else None)
    low = min(point(a, f, an['bone'], an['x'], an['y'])[1] for an in a['anchors'] if an.get('kind') == 'contact')
    for i, (j, t, ang) in f.items():
        print(f'{i:16s} joint {j[0]:7.3f} {j[1] - low:7.3f}  tip {t[0]:7.3f} {t[1] - low:7.3f}  world {ang:8.2f}')
    for an in a['anchors']:
        if an.get('kind') == 'contact':
            q = point(a, f, an['bone'], an['x'], an['y'])
            print('contact', an['id'], round(q[0], 3), round(q[1] - low, 3))
