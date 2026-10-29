import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
old = "        centre = P(at) if at is not None else np.array([window.right - width / 2 - 0.3, window.top - 1.2 - height / 2, 0.0])\n"
assert old in src
new = '''        beside = window.right + 0.3 + width / 2
        centre = P(at) if at is not None else np.array([beside, 0.0, 0.0]) if beside + width / 2 <= 6.9 else np.array([window.right - width / 2 - 0.3, window.top - 1.2 - height / 2, 0.0])
'''
src = src.replace(old, new)
src = src.replace('''        """🔍 Lift one cell out as a card: address, stored value, displayed text, formula and number format. Parts: ``card``, ``rows[name]``, ``leader``, ``cell_ring``."""''', '''        """🔍 Lift one cell out as a card: address, stored value, displayed text, formula and number format. Parts: ``card``, ``rows[name]``, ``leader``, ``cell_ring``.
        Without ``at`` the card stands beside the window when the frame has room (use a window of about 7 units), otherwise it covers the window's upper right."""''')
open(path, "w", encoding="utf-8").write(src)
p2 = sys.argv[2]
s = open(p2, encoding="utf-8").read()
s = s.replace('''        window = self.window(book, selected="B5")
        self.add(window, self.xl_cell_pop(window, "B9", at=(2.6, 0.2, 0)))''', '''        window = self.xl_window(book, SHEET, at=(-3.2, 0, 0), width=7.2, height=5.1, cols="A:B", rows=(1, 10), selected="B5")
        self.add(window, self.xl_cell_pop(window, "B9"))''')
s = s.replace('kwargs.setdefault("rows", (1, 11))', 'kwargs.setdefault("rows", (1, 10))')
open(p2, "w", encoding="utf-8").write(s)
