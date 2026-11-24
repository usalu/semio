"""Turns the isometric light-line SVG exports of the staffage catalogue (cover-staffage-export.sh) into compact TikZ macros.

Coordinates are metres on the picture plane relative to the asset origin (x right, y up), so a macro is placed with
`\\begin{scope}[shift={(x,y,z)},x=1cm,y=1cm]` inside the isometric picture of `model.tex`.
Usage (from the repo root): python <ticket>/cover-staffage-tikz.py [contact]
Writes `🗑️generated/staffage/staffage.tex` (macros) and, with `contact`, `🗑️generated/staffage/contact.tex` (one cell per export).
"""
import re
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
cover = ticket / "🗑️generated" / "staffage"

# macro name, export folder, line tolerance in m, fill tolerance in m, page-coloured occluder behind the lines
CHOSEN = [
    ("Person", "person-drinking-coffee", 0.016, 0.04, True),
    ("Sofa", "three-seat-sofa", 0.015, 0.04, True),
    ("DeckChair", "folding-deck-chair", 0.015, 0.04, True),
    ("Tree", "japanese-maple", 0.05, 0.1, False),
    ("Lavender", "lavender-bush", 0.02, 0.05, True),
    ("Boxwood", "boxwood-ball", 0.015, 0.03, True),
    ("Cat", "domestic-cat", 0.012, 0.03, True),
]


def simplify(points, tolerance):
    if len(points) < 3:
        return points
    (ax, ay), (bx, by) = points[0], points[-1]
    dx, dy = bx - ax, by - ay
    norm = (dx * dx + dy * dy) ** 0.5
    worst, index = 0.0, 0
    for i in range(1, len(points) - 1):
        px, py = points[i]
        distance = abs(dy * (px - ax) - dx * (py - ay)) / norm if norm else ((px - ax) ** 2 + (py - ay) ** 2) ** 0.5
        if distance > worst:
            worst, index = distance, i
    if worst <= tolerance:
        return [points[0], points[-1]]
    return simplify(points[: index + 1], tolerance)[:-1] + simplify(points[index:], tolerance)


def number(value):
    text = f"{value:.2f}".rstrip("0").rstrip(".")
    text = text.replace("0.", ".", 1) if text.startswith(("0.", "-0.")) else text
    return "0" if text in ("", "-", "-0") else text


def subpaths(d, origin, per_metre):
    for chunk in re.findall(r"M[^M]*", d):
        closed = "Z" in chunk
        values = [float(v) for v in re.findall(r"-?\d+(?:\.\d+)?", chunk)]
        yield [((values[i] - origin[0]) / per_metre, (origin[1] - values[i + 1]) / per_metre) for i in range(0, len(values) - 1, 2)], closed


def emit(points, closed, tolerance):
    if closed and points[0] != points[-1]:
        points = points + [points[0]]
    points = simplify(points, tolerance)
    out = []
    for point in points:
        text = "(" + number(point[0]) + "," + number(point[1]) + ")"
        if not out or out[-1] != text:
            out.append(text)
    if closed:
        if out[0] == out[-1]:
            out.pop()
        return "--".join(out) + "--cycle" if len(out) > 2 else ""
    return "--".join(out) if len(out) > 1 else ""


def layer(svg, name):
    match = re.search(r'<g id="staffage-' + name + r'".*?(?=<g id="staffage-|</svg>)', svg, re.S)
    return match.group(0) if match else ""


def convert(folder, line_tolerance, fill_tolerance, occlude):
    svg = next((cover / folder).glob("*.svg")).read_text(encoding="utf8")
    origin = (float(re.search(r'origin-x-mm="([^"]+)"', svg).group(1)), float(re.search(r'origin-y-mm="([^"]+)"', svg).group(1)))
    per_metre = float(re.search(r'mm-per-metre="([^"]+)"', svg).group(1))
    fills = [] if not occlude else [emit(p, True, fill_tolerance) for d in re.findall(r' d="([^"]*)"', layer(svg, "fill")) for p, _ in subpaths(d, origin, per_metre)]
    strong, fine = [], []
    for name in ("body", "detail"):
        for path in re.findall(r"<path[^>]*>", layer(svg, name)):
            width = float(re.search(r'stroke-width="([^"]+)"', path).group(1))
            for points, closed in subpaths(re.search(r' d="([^"]*)"', path).group(1), origin, per_metre):
                (strong if width >= 0.2 else fine).append(emit(points, closed, line_tolerance))
    parts = []
    for option, paths in (("\\fill[semio-light]", fills), ("\\draw[line width=.4pt]", strong), ("\\draw[line width=.2pt]", fine)):
        paths = list(dict.fromkeys(p for p in paths if p))
        if paths:
            parts.append(option + " " + " ".join(paths) + ";")
    return "\n".join(parts)


def macro(name, body):
    return "\\newcommand{\\EnergyStaffage" + name + "}{%\n" + body + "}\n"


if len(sys.argv) > 1 and sys.argv[1] == "contact":
    cells = []
    for folder in sorted(p.name for p in cover.iterdir() if p.is_dir() and any(p.glob("*.svg"))):
        body = convert(folder, 0.01, 0.03, True)
        cells.append("\\begin{tikzpicture}[scale=1.2,line join=round,line cap=round]\\draw[red] (0,0)--(.866,-.5) node[right]{\\tiny x} (0,0)--(.866,.5) node[right]{\\tiny y};" + body + "\\node[below] at (0,-.6){\\tiny " + folder + " " + str(len(body) // 1000) + "k};\\end{tikzpicture}")
    (cover / "contact.tex").write_text("\\documentclass{article}\\usepackage[margin=1cm]{geometry}\\usepackage{tikz}\\definecolor{semio-light}{HTML}{f7f3e3}\\begin{document}\\noindent\n" + "\n".join(cells) + "\n\\end{document}\n", encoding="utf8")
else:
    macros = [macro(name, convert(folder, line, fill, occlude)) for name, folder, line, fill, occlude in CHOSEN]
    for (name, *_), body in zip(CHOSEN, macros):
        print("[DEBUG]", name, len(body), "bytes")
    text = "".join(macros)
    (cover / "staffage.tex").write_text(text, encoding="utf8")
    print("[DEBUG] staffage.tex", len(text), "bytes")
