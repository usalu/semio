import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap('''def _tabs(sheets, active: str, *, at, width: float | None, size: float = S_LABEL):
    """🗂️ The sheet tab strip: the active tab white with a green underline, arrows left and a plus right."""
    labels = [_label(name, size, CELL_INK) for name in sheets]
    widths = [item.width + 0.42 for item in labels]''', '''def _tabs(sheets, active: str, *, at, width: float | None, size: float = S_SMALL, titles: dict | None = None):
    """🗂️ The compact sheet tab strip: the active tab white with a green underline, scroll arrows left and a plus right; ``titles`` maps sheet name to tab text."""
    titles = titles or {}
    labels = [_label(titles.get(name, name), size, CELL_INK, weight="BOLD" if name == active else None) for name in sheets]
    widths = [item.width + 0.3 for item in labels]''')
swap("    room = width - 0.9 - 0.7\n", "    room = width - 0.9 - 0.5\n")
swap('''        text = _put(name, x + extent / 2,''', '''        text = _put(titles.get(name, name), x + extent / 2,''')
swap('''self.tabs_strip = _tabs(self.view.sheets, self.sheet, at=((self.x0 + self.right) / 2, self.bottom + STATUS_H + TABS_H / 2), width=width, size=S_LABEL)''', '''titles = {name: self.view.tab_title(name) for name in self.view.sheets} if hasattr(self.view, "tab_title") else {}
        self.tabs_strip = _tabs(self.view.sheets, self.sheet, at=((self.x0 + self.right) / 2, self.bottom + STATUS_H + TABS_H / 2), width=width, titles=titles)''')
swap('''    def column_width(self, sheet: str, col: str) -> float:
        return float(self.workbook.sheet(sheet).widths.get(col, 8.43))''', '''    def column_width(self, sheet: str, col: str) -> float:
        return float(self.workbook.sheet(sheet).widths.get(col, 8.43))

    def tab_title(self, sheet: str) -> str:
        return _model().tab_name(self.workbook, sheet, self.language)''')
swap('''    def xl_tabs(self, sheets, active: str, *, at=(0, 0, 0), width: float | None = None):
        """🗂️ A standalone sheet tab strip: ``.tabs[name]`` per tab, the active one white with a green underline."""
        return _tabs(list(sheets), active, at=at, width=width)''', '''    def xl_tabs(self, sheets, active: str, *, at=(0, 0, 0), width: float | None = None, titles: dict | None = None):
        """🗂️ A standalone sheet tab strip: ``.tabs[name]`` per tab (keyed by canonical sheet name), the active one white with a green underline; ``titles`` maps names to tab text, e.g. ``engine.sheet_titles(language)``."""
        return _tabs(list(sheets), active, at=at, width=width, titles=titles)''')
open(path, "w", encoding="utf-8").write(src)
