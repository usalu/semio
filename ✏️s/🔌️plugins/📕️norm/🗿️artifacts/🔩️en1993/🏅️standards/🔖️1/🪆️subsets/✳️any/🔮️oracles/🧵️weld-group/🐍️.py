"""🧵️ Independent oracle of the weld group elastic vector method: the line moments of inertia come from numerical integration along the weld lines (not the closed forms of the Rust code). Run with `python3 -I`; prints the showcase result."""
import json
import math


def segments(pattern, b, d):
    if pattern == "line":
        return [((0.0, -d / 2), (0.0, d / 2))]
    lines = [((-b / 2, -d / 2), (-b / 2, d / 2)), ((b / 2, -d / 2), (b / 2, d / 2))]
    if pattern == "box":
        lines += [((-b / 2, -d / 2), (b / 2, -d / 2)), ((-b / 2, d / 2), (b / 2, d / 2))]
    return lines


def polar(lines, n=20000):
    total, ip = 0.0, 0.0
    for (x0, y0), (x1, y1) in lines:
        length = math.hypot(x1 - x0, y1 - y0)
        ds = length / n
        for i in range(n):
            t = (i + 0.5) / n
            x, y = x0 + t * (x1 - x0), y0 + t * (y1 - y0)
            ip += (x * x + y * y) * ds
        total += length
    return total, ip


def main():
    b, d, a, f, e, fu, beta, gm2 = 100.0, 200.0, 5.0, 100e3, 150.0, 510.0, 0.9, 1.25
    length, ip = polar(segments("twoLines", b, d))
    t = f * e
    q = math.hypot(f / length + t * (b / 2) / ip, t * (d / 2) / ip)
    rd = fu / math.sqrt(3) / (beta * gm2) * a
    print(json.dumps({"length": length, "ip": ip, "q": q, "rd": rd, "util": q / rd}))


main()
