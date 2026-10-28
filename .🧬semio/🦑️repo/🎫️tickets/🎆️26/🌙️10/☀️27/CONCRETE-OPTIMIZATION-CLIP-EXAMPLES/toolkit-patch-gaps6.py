"""[DEBUG] One-off edit: cheap single-shape dashed lines."""
from pathlib import Path

base = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals")
room, main = base / "🏠️room/🐍️.py", base / "🐍️.py"
source = room.read_text(encoding="utf-8")
source = source.replace("def _dimension(a, b,", '''def dashed(a, b, color=GRAY, width=1.6, dash=0.09, gap=0.07, opacity=1.0):
    """┄ A dashed line drawn as one shape (``DashedLine`` builds one mobject per dash and is far slower)."""
    a, b = np.array(a, dtype=float), np.array(b, dtype=float)
    length = float(np.linalg.norm(b - a))
    line = _engine.VMobject(stroke_color=color, stroke_width=width, stroke_opacity=opacity, fill_opacity=0)
    if length < 1e-6:
        return line
    unit = (b - a) / length
    position = 0.0
    while position < length:
        stop = min(position + dash, length)
        line.start_new_path(a + unit * position)
        line.add_line_to(a + unit * stop)
        position += dash + gap
    return line


def _dimension(a, b,''', 1)
source = source.replace("mn.DashedLine(", "dashed(")
room.write_text(source, encoding="utf-8")
text = main.read_text(encoding="utf-8")
text = text.replace("mn.DashedLine(", "_room.dashed(")
main.write_text(text, encoding="utf-8")
