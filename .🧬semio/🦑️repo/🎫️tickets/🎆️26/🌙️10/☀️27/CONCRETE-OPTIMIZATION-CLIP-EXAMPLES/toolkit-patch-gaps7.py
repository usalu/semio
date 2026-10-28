"""[DEBUG] One-off edit: merge many small lines into single shapes."""
from pathlib import Path

room = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py")
source = room.read_text(encoding="utf-8")


def swap(old, new):
    global source
    assert old in source, old[:80]
    source = source.replace(old, new, 1)


start = source.index("def _hatch(")
end = source.index("def label_size(")
swap(source[start:end], '''def strokes(segments, color=GRAY, width=1.4, opacity=1.0):
    """〰️ Many separate line segments drawn as one shape."""
    shape = _engine.VMobject(stroke_color=color, stroke_width=width, stroke_opacity=opacity, fill_opacity=0)
    for a, b in segments:
        shape.start_new_path(np.array(a, dtype=float))
        shape.add_line_to(np.array(b, dtype=float))
    return shape


def _hatch(box, spacing, project, color, width=1.2, opacity=.7):
    x0, y0, x1, y1 = box
    segments, c = [], x0 - y1
    while c < x1 - y0:
        low, high = max(x0, y0 + c), min(x1, y1 + c)
        if high - low > 1e-3:
            segments.append((project(low, low - c), project(high, high - c)))
        c += spacing
    return strokes(segments, color, width, opacity)


''')
start = source.index("def _dimension(")
end = source.index("def _ring_polygon(")
swap(source[start:end], '''def _dimension(a, b, color=GRAY, tick=0.05, width=1.4):
    a, b = np.array(a, dtype=float), np.array(b, dtype=float)
    direction = (b - a) / max(np.linalg.norm(b - a), 1e-9)
    normal = np.array([-direction[1], direction[0], 0.0]) * tick
    return strokes([(a, b), (a - normal, a + normal), (b - normal, b + normal)], color, width)


''')
swap('''            ground = mn.VGroup(mn.Line(P(x_o - 2.4, 0), P(x_o, 0), color=GRAY, stroke_width=2))
            for k in range(1, 8):
                ground.add(mn.Line(P(x_o - 0.35 * k, 0), P(x_o - 0.35 * k - 0.25, -0.25), color=GRAY, stroke_width=1, stroke_opacity=.6))''',
     '''            ground = mn.VGroup(strokes([(P(x_o - 2.4, 0), P(x_o, 0))], GRAY, 2),
                               strokes([(P(x_o - 0.35 * k, 0), P(x_o - 0.35 * k - 0.25, -0.25)) for k in range(1, 8)], GRAY, 1, .6))''')
room.write_text(source, encoding="utf-8")
