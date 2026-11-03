import os

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()
helper = '''

def path_points(d, steps=10):
    """Flatten an SVG path of M, L, C and Z commands (absolute) to a polygon."""
    t = d.replace(",", " ").split()
    pts, i, cur, cmd = [], 0, (0.0, 0.0), None
    while i < len(t):
        if t[i] in "MLCQZ":
            cmd = t[i]
            i += 1
            if cmd == "Z":
                continue
        if cmd in "ML":
            cur = (float(t[i]), float(t[i + 1]))
            pts.append(cur)
            i += 2
        elif cmd == "C":
            c1, c2, e = [(float(t[i + 2 * k]), float(t[i + 2 * k + 1])) for k in range(3)]
            for k in range(1, steps + 1):
                u = k / steps
                a, b, c, dd = (1 - u) ** 3, 3 * (1 - u) ** 2 * u, 3 * (1 - u) * u * u, u ** 3
                pts.append((a * cur[0] + b * c1[0] + c * c2[0] + dd * e[0], a * cur[1] + b * c1[1] + c * c2[1] + dd * e[1]))
            cur = e
            i += 6
        elif cmd == "Q":
            c1, e = [(float(t[i + 2 * k]), float(t[i + 2 * k + 1])) for k in range(2)]
            for k in range(1, steps + 1):
                u = k / steps
                pts.append(((1 - u) ** 2 * cur[0] + 2 * (1 - u) * u * c1[0] + u * u * e[0], (1 - u) ** 2 * cur[1] + 2 * (1 - u) * u * c1[1] + u * u * e[1]))
            cur = e
            i += 4
        else:
            raise ValueError(cmd)
    return pts
'''
s = s.replace("\n\ndef _radial(self", helper + "\n\ndef _radial(self", 1)
s = s.replace('    elif lk["kind"] == "ellipse":\n        a = math.radians(lk.get("rotationDeg", 0))\n        pts = []\n        for k in range(48):', '    elif lk["kind"] == "path":\n        pts = path_points(lk["d"])\n    elif lk["kind"] == "ellipse":\n        a = math.radians(lk.get("rotationDeg", 0))\n        pts = []\n        for k in range(48):', 1)
s = s.replace('            if lk["kind"] == "ellipse":\n                a = math.radians', '            if lk["kind"] == "path":\n                return [[r5(x), r5(y)] for x, y in path_points(lk["d"])]\n            if lk["kind"] == "ellipse":\n                a = math.radians', 1)
open(p, "w", encoding="utf-8").write(s)
print(s.count("path_points"))
