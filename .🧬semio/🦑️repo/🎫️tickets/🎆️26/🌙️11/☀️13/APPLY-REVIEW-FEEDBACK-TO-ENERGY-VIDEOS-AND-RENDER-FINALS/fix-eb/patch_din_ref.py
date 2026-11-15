import re
import sys
from pathlib import Path

NEW = '''def _din_ref(text: str, title=None):
    """📖 Standards citation for the beat in the top-right corner, flowed into lines that stay right of the beat title."""
    room = config.frame_width / 2 - 0.30 - (title.get_right()[0] + 0.30) if title is not None else 9.0

    def line(content: str):
        return body_text(content, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)

    lines = []
    for part in text.split(" · "):
        if lines and line(f"{lines[-1]} · {part}").width <= room:
            lines[-1] += f" · {part}"
        else:
            lines.append(part)
    ref = VGroup(*[line(content) for content in lines]).arrange(DOWN, buff=0.06, aligned_edge=RIGHT)
    if ref.width > room:
        ref.scale_to_fit_width(room)
    ref.set_opacity(0.72)
    ref.to_corner(UR, buff=0.30)
    return ref
'''

DEF = re.compile(r"def _din_ref\(text: str\):\n.*?\n    return ref\n", re.S)
CALL = re.compile(r"^(\s+din = _din_ref\(.*\))$", re.M)

for name in sys.argv[1:]:
    path = Path(name)
    source = path.read_text(encoding="utf-8")
    source, defs = DEF.subn(lambda _: NEW, source, count=1)
    source, calls = CALL.subn(lambda m: m.group(1)[:-1] + ", subtitle)", source)
    path.write_text(source, encoding="utf-8", newline="\n")
    print(ascii(name), defs, calls)
