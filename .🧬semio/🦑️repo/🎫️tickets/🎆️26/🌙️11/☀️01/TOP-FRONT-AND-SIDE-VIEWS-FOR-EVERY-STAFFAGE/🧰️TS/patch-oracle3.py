import os

p = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️lateral-spread/🐍️.py"
s = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
i = s.index("def moved(")
j = s.index("def main():")
NEW = '''def moved(view, shape, bone, rest, now, is_affected):
    corners = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    front = view == "front"
    stretches = shape.get("followTip") is True
    span = (rest["tip"][1] - rest["origin"][1]) if front else (rest["tip"][0] - rest["origin"][0])
    in_view = abs(span) >= 0.5 * bone["lengthM"]
    result = []
    for x, y in corners:
        q = carry(rest, now, [rest["origin"][0], y, x]) if front else carry(rest, now, [x, rest["origin"][1], y])
        result.append([q[2], q[1]] if front else [q[0], q[2]])
    if stretches and not in_view:
        origin = [now["origin"][k] - rest["origin"][k] for k in range(3)]
        tip = [now["tip"][k] - rest["tip"][k] for k in range(3)]
        outward = np.sign(tip[2] - origin[2])
        if outward != 0:
            across = [c[0] if front else c[1] for c in corners]
            low, high = min(across), max(across)
            for index, c in enumerate(across):
                u = 1.0 if high == low else (c - low) / (high - low) if outward > 0 else (high - c) / (high - low)
                if front:
                    result[index][0] += (tip[2] - origin[2]) * u
                    result[index][1] += (tip[1] - origin[1]) * u
                else:
                    result[index][0] += (tip[0] - origin[0]) * u
                    result[index][1] += (tip[2] - origin[2]) * u
    return MultiPoint([tuple(point) for point in result]).convex_hull


'''
s = s[:i] + NEW + s[j:]
open(p + ".tmp", "w", encoding="utf-8", newline="").write(s)
os.replace(p + ".tmp", p)
