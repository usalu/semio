import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("    def _formula_content(self, cy: float):")
b = src.index("    def _status_bar(self):")
new = '''    def _formula_content(self, cy: float):
        """📝 The formula bar text: the formula typed, else the active cell's formula or its stored constant; long text shrinks to 16.2 and then wraps into an expanded bar."""
        editing, address = self.state["editing"], self.active_address()
        raw = editing if editing is not None else self.view.formula(self.sheet, address)
        if raw is None:
            value = self.view.value(self.sheet, address)
            return _engine.VGroup() if value is None else self._field(self.raw_text(value), cy, {}, False)
        text = self.scene.xl_localize(raw, self.language)
        return self._field(text, cy, reference_colors(text) if editing is not None else {}, editing is not None)

    def _field(self, text: str, cy: float, colors: dict, caret: bool):
        """↕️ Place text in the formula field: one line at 19.2, else one line at 16.2, else wrapped at operators and separators into a panel that expands downward over the grid (Excel's expand chevron)."""
        room = self.right - self.field_left - 0.55
        size = S_LABEL if _advance(text, S_LABEL) <= room else S_SMALL
        lines = [text] if _advance(text, size) <= room else _wrap(text, size, room)
        chevron_x = self.right - 0.3
        up = len(lines) > 1
        chevron = polyline([(chevron_x - 0.07, cy + (-0.03 if up else 0.03)), (chevron_x, cy + (0.03 if up else -0.03)), (chevron_x + 0.07, cy + (-0.03 if up else 0.03))], HEADER_INK, W_EDGE)
        group = _engine.VGroup(chevron)
        pitch = _cap(size) + 0.17
        if up:
            floor = cy - FORMULA_H / 2 + 0.04 - pitch * (len(lines) - 1) - 0.14
            group.add(_box(self.field_left - 0.06, cy + FORMULA_H / 2 - 0.05, self.right - 0.08, floor, PAPER, EDGE, W_THIN))
            group.add(chevron)
        for index, line in enumerate(lines):
            group.add(formula_group(line, at=(self.field_left, cy - _cap(size) / 2 - index * pitch, 0), size=size, colors=colors, caret=caret and index == len(lines) - 1))
        group.lines = lines
        return group

'''
src = src[:a] + new + src[b:]
src = src.replace('''def _cap(size: float) -> float:''', '''def _wrap(text: str, size: float, room: float) -> list[str]:
    """↩️ Greedy wrap at operators and separators (never inside a reference): the longest start that fits ``room`` per line, found by bisection over break points."""
    lines = []
    while text:
        if _advance(text, size) <= room:
            lines.append(text)
            break
        breaks = [i + 1 for i, char in enumerate(text) if char in ",;(+-*/&=<>" and 0 < i + 1 < len(text)] or [max(1, len(text) // 2)]
        low, high, best = 0, len(breaks) - 1, breaks[0]
        while low <= high:
            middle = (low + high) // 2
            if _advance(text[:breaks[middle]], size) <= room:
                best, low = breaks[middle], middle + 1
            else:
                high = middle - 1
        lines.append(text[:best])
        text = text[best:]
    return lines


def _cap(size: float) -> float:''', 1)
old = "        extent = max(right, left + run.width_total + 0.3)\n        return _engine.VGroup(_box(left, top, extent, bottom, PAPER, GREEN, 3.4), run)"
assert old in src
src = src.replace(old, '''        extent = max(right, left + run.width_total + 0.3)
        if extent > self.right - 0.05:
            return _engine.VGroup(_box(left, top, right, bottom, PAPER, GREEN, 3.4))
        return _engine.VGroup(_box(left, top, extent, bottom, PAPER, GREEN, 3.4), run)''')
open(path, "w", encoding="utf-8").write(src)
