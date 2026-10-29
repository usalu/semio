import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("    def _cell_label(self, a1: str, text: str, align: str):")
b = src.index("    def _build(self):")
new = '''    def _room(self, a1: str, align: str) -> float:
        """↔️ Width a cell text may use: its own cell, and for left-aligned text also the empty cells to its right."""
        left, _, right, _ = self.rect(a1)
        column, row = split_address(a1)
        end = right
        if align == "l":
            for next_column in range(column + 1, self.cols[-1] + 1):
                if self.text_of(cell_address(next_column, row)) is not None:
                    break
                end = self.col_x[next_column - self.cols[0] + 1]
        return end - left - 0.22

    def _cell_label(self, a1: str, text: str, align: str):
        """🏷️ A cell string on the common baseline; text longer than its room is cut with an ellipsis and numbers become ###, as Excel does."""
        left, top, right, bottom = self.rect(a1)
        style = self.styles(self.sheet, a1) if self.styles else {}
        align = style.get("align", align)
        room, weight = self._room(a1, align), "BOLD" if style.get("bold") else None
        if _label(text, self.font, CELL_INK, weight=weight).width > room:
            text = "###" if align == "r" else _clip(text, room, self.font, weight)
        x = {"l": left + 0.1, "r": right - 0.12, "c": (left + right) / 2}[align]
        return _put(text, x, (top + bottom) / 2, align, self.font, style.get("color", CELL_INK), weight=weight)

'''
src = src[:a] + new + src[b:]
src = src.replace('''def _cap(size: float) -> float:''', '''def _clip(text: str, room: float, size: float, weight: str | None = None) -> str:
    """✂️ The longest start of ``text`` that fits ``room`` once an ellipsis is appended."""
    keep = max(1, int(len(text) * room / max(_label(text, size, CELL_INK, weight=weight).width, 1e-6)) - 1)
    while keep > 1 and _label(text[:keep].rstrip() + "…", size, CELL_INK, weight=weight).width > room:
        keep -= 1
    return text[:keep].rstrip() + "…"


def _cap(size: float) -> float:''', 1)
open(path, "w", encoding="utf-8").write(src)
