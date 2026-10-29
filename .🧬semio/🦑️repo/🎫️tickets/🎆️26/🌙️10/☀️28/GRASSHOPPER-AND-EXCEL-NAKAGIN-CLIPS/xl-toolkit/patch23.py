import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("def _advance(text: str, size: float) -> float:")
b = src.index("\n\n\n", a)
new = '''def _advance(text: str, size: float) -> float:
    """📏 Horizontal advance of a string: measured in chunks of 60 characters at 16.2 against a bar (so trailing spaces count) and scaled, because pango drops glyphs of very wide texts."""
    bar = _label("|", S_SMALL, CELL_INK).width
    return sum(_label(text[i:i + 60] + "|", S_SMALL, CELL_INK).width - bar for i in range(0, max(len(text), 1), 60)) * size / S_SMALL'''
src = src[:a] + new + src[b:]
open(path, "w", encoding="utf-8").write(src)
