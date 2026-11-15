import re
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")

helper = '''def _word_over_value(word: str, value: str, at, *, size: float = BODY_FONT_SIZE, color: str = P_WHITE, edge: str = "right"):
    """🏷️ Two stacked right-aligned lines — a quantity name over its symbol and value — ending on ``at``."""
    top = _m(rf"\\text{{{word}}}", size=size, color=color)
    low = _m(value, size=size, color=color)
    group = VGroup(top, low).arrange(DOWN, buff=0.04, aligned_edge=RIGHT if edge == "right" else LEFT)
    anchor = np.array(at, dtype=float)
    x = group.get_right()[0] if edge == "right" else group.get_left()[0]
    return group.shift(np.array([anchor[0] - x, anchor[1] - group.get_center()[1], 0.0]))


'''
marker = "def _readout(fn, at"
assert text.count(marker) == 1
text = text.replace(marker, helper + marker)

pattern = re.compile(r'(\w+_lbl) = _m\(tr\(r"\{\\text\{([^}]*)\} \\atop ([^"]*)\}", r"\{\\text\{([^}]*)\} \\atop [^"]*\}"\), (np\.array\(\[[^\n]*\]\)),\n\s+size=LABEL_FONT_SIZE, color=(\w+), edge="right"\)')


def repl(m):
    name, en, value, de, at, color = m.groups()
    return f'{name} = _word_over_value(tr("{en}", "{de}"), r"{value}", {at}, size=LABEL_FONT_SIZE, color={color})'


text, n = pattern.subn(repl, text)
print(n)
ROOT.write_text(text, encoding="utf-8")
