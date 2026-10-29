import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap('''        x = {"l": left + 0.1, "r": right - 0.12, "c": (left + right) / 2}[align]
        return _put(text, x, (top + bottom) / 2, align, self.font, style.get("color", CELL_INK), weight=weight)''', '''        x = {"l": left + 0.1, "r": right - 0.12, "c": (left + right) / 2}[align]
        label = _put(text, x, (top + bottom) / 2, align, self.font, style.get("color", CELL_INK), weight=weight)
        if label.get_right()[0] > right - 0.02:
            return _engine.VGroup(_box(left + 0.03, top - 0.02, label.get_right()[0] + 0.1, bottom + 0.02, PAPER), label)
        return label''')
swap('''            group.cursor = segments([((x - 0.09, y - 0.1), (x + 0.09, y - 0.1)), ((x, y - 0.19), (x, y - 0.01))], CELL_INK, 3.0)''', '''            group.cursor = segments([((x - 0.09, y), (x + 0.09, y)), ((x, y - 0.09), (x, y + 0.09))], CELL_INK, 3.0)''')
open(path, "w", encoding="utf-8").write(src)
