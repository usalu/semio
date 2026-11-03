import os

p = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️lateral-spread/🐍️.py"
s = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")

SOLID = '''def project(view, q):
    return (q[0], q[2]) if view == "plan" else (q[2], q[1])


def solid_geometry(view, shape, rig, rest, now):
    solid = shape["solid"]
    bone = shape["bone"]
    corners = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    origin = np.array(now[bone]["origin"])
    rotation = np.array(now[bone]["rotation"])

    def place(a, b, c):
        return origin + rotation @ np.array([a, b, c])

    if solid["kind"] == "extruded":
        rest_z = rest[bone]["origin"][2]
        points = [project(view, place(a, b, c - rest_z)) for a, b in corners for c in (solid["z0"], solid["z1"])]
        return MultiPoint(points).convex_hull
    axis_bone = solid["axisBone"]
    axis_rest = rest[axis_bone]
    axis_now = now[axis_bone]
    rest_rotation = np.array(axis_rest["rotation"])
    now_rotation = np.array(axis_now["rotation"])
    base = np.array(axis_now["origin"]) + now_rotation @ rest_rotation.T @ (np.array([solid["axisX"], axis_rest["origin"][1], solid.get("axisZ", 0.0)]) - np.array(axis_rest["origin"]))
    axis = now_rotation @ rest_rotation.T @ np.array([0.0, 1.0, 0.0])
    reference = np.array([0.0, 0.0, 1.0]) if abs(axis[2]) < 0.9 else np.array([1.0, 0.0, 0.0])
    e1 = np.cross(axis, reference)
    e1 /= np.linalg.norm(e1)
    e2 = np.cross(axis, e1)
    circles = []
    for a, b in corners:
        q = place(a, b, 0.0) - base
        h = float(q @ axis)
        r = float(np.linalg.norm(q - h * axis))
        circles.append([project(view, base + h * axis + r * (math.cos(t) * e1 + math.sin(t) * e2)) for t in np.linspace(0.0, 2.0 * math.pi, 361)])
    return MultiPoint([point for circle in circles for point in circle]).convex_hull


'''
i = s.index("def lateral_fraction")
s = s[:i] + SOLID + s[i:]
old = "                geometry = moved(view, shape, bone, rest[bone[\"id\"]], now[bone[\"id\"]], flags[bone[\"id\"]])\n"
assert old in s
new = """                if shape.get("solid"):
                    if not flags[bone["id"]]:
                        continue
                    geometry = solid_geometry(view, shape, rig, rest, now)
                else:
                    geometry = moved(view, shape, bone, rest[bone["id"]], now[bone["id"]], flags[bone["id"]])
"""
s = s.replace(old, new)
open(p + ".tmp", "w", encoding="utf-8", newline="").write(s)
os.replace(p + ".tmp", p)
