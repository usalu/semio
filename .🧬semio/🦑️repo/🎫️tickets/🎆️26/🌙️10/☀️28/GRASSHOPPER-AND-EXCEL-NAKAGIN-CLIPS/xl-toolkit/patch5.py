import sys

path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:60]
    src = src.replace(old, new)


swap("self.font = float(min(S_LABEL, max(S_SMALL, self.row_h * 60)))", "self.font = S_LABEL if self.row_h >= 0.34 else S_SMALL")
swap('''    x = left + 0.9
    tabs = {}
    for name, item, extent in zip(sheets, labels, widths):''', '''    sheets, room = list(sheets), width - 0.9 - 0.7
    names = list(sheets)
    while len(names) > 1 and sum(widths[sheets.index(n)] for n in names[:names.index(active) + 1]) > room:
        names.pop(0)
    kept, spent = [], 0.0
    for n in names:
        spent += widths[sheets.index(n)]
        if spent <= room:
            kept.append((n, labels[sheets.index(n)], widths[sheets.index(n)]))
    x = left + 0.9
    tabs = {}
    for name, item, extent in kept:''')
swap('''        for entry in DATA["ribbon"]:
            text = _put(entry[self.language], x, cy, "l", S_LABEL, GREEN if entry["key"] == self.ribbon_key else CELL_INK, weight="BOLD" if entry["key"] == self.ribbon_key else None)
            group.add(text)''', '''        wide = self.size_box[0] >= 9.0
        for entry in DATA["ribbon"]:
            text = _put(entry[self.language], x, cy, "l", S_LABEL, GREEN if entry["key"] == self.ribbon_key else CELL_INK, weight="BOLD" if entry["key"] == self.ribbon_key else None)
            if text.get_right()[0] > self.right - (1.7 if wide else 0.2):
                break
            group.add(text)''')
swap('''        share = _put(DATA["ui"]["share"][self.language], self.right - 0.5, cy, "r", S_SMALL, PAPER, weight="BOLD")
        button = _engine.RoundedRectangle(corner_radius=0.06, width=share.width + 0.34, height=RIBBON_H - 0.08, stroke_width=0, fill_color=GREEN, fill_opacity=1).move_to(P((share.get_center()[0], cy)))
        group.add(button, share)
        return group''', '''        if wide:
            share = _put(DATA["ui"]["share"][self.language], self.right - 0.5, cy, "r", S_SMALL, PAPER, weight="BOLD")
            button = _engine.RoundedRectangle(corner_radius=0.06, width=share.width + 0.34, height=RIBBON_H - 0.08, stroke_width=0, fill_color=GREEN, fill_opacity=1).move_to(P((share.get_center()[0], cy)))
            group.add(button, share)
        return group''')
swap('''            x = self.right - 2.0
            for key, value in reversed(parts):
                item = _put(f"{DATA['ui'][key][self.language]}: {show(value)}", x, cy, "r", S_SMALL, HEADER_INK)
                group.add(item)
                x = item.get_left()[0] - 0.35''', '''            x = self.right - 2.0
            for key, value in reversed(parts):
                item = _put(f"{DATA['ui'][key][self.language]}: {show(value)}", x, cy, "r", S_SMALL, HEADER_INK)
                if item.get_left()[0] < self.x0 + 1.2:
                    break
                group.add(item)
                x = item.get_left()[0] - 0.35''')
swap('''        plot_t, plot_b = top - (0.78 if title else 0.4), top - height + 0.55''', '''        plot_t, plot_b = top - (0.95 if title else 0.55), top - height + 0.55''')
swap('''            group.add(_put(unit, plot_l - 0.1, plot_t + 0.17, "r", S_SMALL, HEADER_INK))''', '''            group.add(_put(unit, plot_l - 0.1, plot_t + 0.36, "r", S_SMALL, HEADER_INK))''')
open(path, "w", encoding="utf-8").write(src)
