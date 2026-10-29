import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("    def _room(self, a1: str, align: str) -> float:")
b = src.index("    def _build(self):")
new = '''    def _room(self, a1: str, align: str) -> float:
        """↔️ Width a cell text may use: its own cell, plus the empty cells to its right (left-aligned) or the same number of empty cells on both sides (centred)."""
        left, _, right, _ = self.rect(a1)
        column, row = split_address(a1)

        def reach(step: int) -> float:
            end = right if step > 0 else left
            for index in range(column + step, self.cols[-1] + 1 if step > 0 else self.cols[0] - 1, step):
                if self.text_of(cell_address(index, row)) is not None:
                    break
                end = self.col_x[index - self.cols[0] + (1 if step > 0 else 0)]
            return abs(end - (right if step > 0 else left))

        extra = reach(1) if align == "l" else min(reach(1), reach(-1)) * 2 if align == "c" else 0.0
        return right - left + extra - 0.22

    def _cell_label(self, a1: str, text: str, align: str):
        """🏷️ A cell string on the common baseline. Left text and error codes overflow into empty neighbours like Excel; centred text that still does not fit is scaled to fit; left text is cut with an ellipsis and numbers become ###."""
        left, top, right, bottom = self.rect(a1)
        style = self.styles(self.sheet, a1) if self.styles else {}
        align = style.get("align", align)
        room, weight = self._room(a1, align), "BOLD" if style.get("bold") else None
        x = {"l": left + 0.1, "r": right - 0.12, "c": (left + right) / 2}[align]
        fits = _label(text, self.font, CELL_INK, weight=weight).width <= room
        if not fits and align == "r":
            text = "###"
        elif not fits and align == "l":
            text = _clip(text, room, self.font, weight)
        label = _put(text, x, (top + bottom) / 2, align, self.font, style.get("color", CELL_INK), weight=weight)
        if align == "c" and label.width > room:
            label.scale_to_fit_width(room).set_x(x)
        if label.get_right()[0] > right - 0.02 or label.get_left()[0] < left + 0.02:
            return _engine.VGroup(_box(min(left, label.get_left()[0] - 0.1) + 0.03, top - 0.02, max(right, label.get_right()[0] + 0.1), bottom + 0.02, PAPER), label)
        return label

'''
src = src[:a] + new + src[b:]
open(path, "w", encoding="utf-8").write(src)
