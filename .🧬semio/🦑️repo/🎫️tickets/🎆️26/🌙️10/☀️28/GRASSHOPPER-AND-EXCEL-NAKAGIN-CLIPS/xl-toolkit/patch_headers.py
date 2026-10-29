import re, sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
start = src.index("    def _headers(self):")
end = src.index("    def _edit_frames(self):")
new = '''    def _headers(self):
        """🔢 Column letters and row numbers on grey bands; the selected ones sit on a green-tinted band with a green edge."""
        c1, r1, c2, r2 = parse_range(self.state["selected"])
        group = _engine.VGroup(_box(self.x0, self.grid_top + HEADER_H, self.right, self.grid_top, HEADER),
                               _box(self.x0, self.grid_top, self.x0 + ROW_HEADER_W, self.grid_bottom, HEADER))
        for i, col in enumerate(self.cols):
            if c1 <= col <= c2:
                group.add(_box(self.col_x[i], self.grid_top + HEADER_H, self.col_x[i + 1], self.grid_top, SELECT_HEADER))
        for i, row in enumerate(self.rows):
            if r1 <= row <= r2:
                group.add(_box(self.x0, self.grid_top - i * self.row_h, self.x0 + ROW_HEADER_W, self.grid_top - (i + 1) * self.row_h, SELECT_HEADER))
        group.add(segments([((self.x0, self.grid_top), (self.right, self.grid_top)), ((self.x0 + ROW_HEADER_W, self.grid_top + HEADER_H), (self.x0 + ROW_HEADER_W, self.grid_bottom))], EDGE, W_THIN))
        group.add(segments([((x, self.grid_top + HEADER_H), (x, self.grid_top)) for x in self.col_x[1:]], EDGE, W_HAIR))
        group.add(segments([((self.x0, self.grid_top - i * self.row_h), (self.x0 + ROW_HEADER_W, self.grid_top - i * self.row_h)) for i in range(1, len(self.rows) + 1)], EDGE, W_HAIR))
        group.add(polygon([(self.x0 + ROW_HEADER_W - 0.06, self.grid_top + 0.06), (self.x0 + ROW_HEADER_W - 0.06, self.grid_top + HEADER_H - 0.07), (self.x0 + 0.1, self.grid_top + 0.06)], "#C6C6C6", 1.0))
        for i, col in enumerate(self.cols):
            on = c1 <= col <= c2
            x = (self.col_x[i] + self.col_x[i + 1]) / 2
            group.add(_put(column_letters(col), x, self.grid_top + HEADER_H / 2, "c", self.font, GREEN_DARK if on else HEADER_INK, weight="BOLD" if on else None))
            if on:
                group.add(segments([((self.col_x[i], self.grid_top), (self.col_x[i + 1], self.grid_top))], GREEN, 3.0))
        for i, row in enumerate(self.rows):
            on = r1 <= row <= r2
            y = self.grid_top - (i + 0.5) * self.row_h
            group.add(_put(str(row), self.x0 + ROW_HEADER_W / 2, y, "c", self.font, GREEN_DARK if on else HEADER_INK, weight="BOLD" if on else None))
            if on:
                group.add(segments([((self.x0 + ROW_HEADER_W, y + self.row_h / 2), (self.x0 + ROW_HEADER_W, y - self.row_h / 2))], GREEN, 3.0))
        return group

'''
src = src[:start] + new + src[end:]
src = src.replace('''        self.headers = self._headers()
        self.marks = self.scene.live(self._header_marks, lambda: self.state["selected"])
''', '''        self.headers = self.scene.live(self._headers, lambda: self.state["selected"])
''').replace("self.add(self.headers, self.marks, self.frames, self.active)", "self.add(self.headers, self.frames, self.active)")
src = src.replace("shadow = _box(self.x0 + 0.07, self.top - 0.09, self.right + 0.07", "shadow = _box(self.x0 + 0.07, self.top, self.right + 0.07")
open(path, "w", encoding="utf-8").write(src)
