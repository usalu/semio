import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap('''    def __init__(self, window, anchor, pairs, **kwargs):
        self.window = window
        self.parts = [_engine.Transform(part, target) for part, target in pairs]
        super().__init__(anchor, introducer=True, **kwargs)''', '''    def __init__(self, window, anchor, pairs=(), retexts=(), **kwargs):
        self.window, self.retexts = window, list(retexts)
        self.parts = [_engine.Transform(part, target) for part, target in pairs]
        super().__init__(anchor, introducer=True, **kwargs)''')
swap('''    def interpolate_mobject(self, alpha):
        for part in self.parts:
            part.interpolate(alpha)

    def finish(self):
        for part in self.parts:
            part.finish()
        super().finish()''', '''    def interpolate_mobject(self, alpha):
        for part in self.parts:
            part.interpolate(alpha)
        for holder, old, new, glow in self.retexts:
            if old is not None:
                old.set_opacity(max(0.0, 1.0 - 2.0 * alpha))
            if new is not None:
                new.set_opacity(max(0.0, 2.0 * alpha - 1.0))
            glow.set_fill(GREEN, 0.3 * math.sin(math.pi * alpha))

    def finish(self):
        for part in self.parts:
            part.finish()
        for holder, old, new, glow in self.retexts:
            if new is not None:
                new.set_opacity(1.0)
            holder.remove(*[item for item in (old, glow) if item is not None])
        super().finish()''')
a = src.index("        pairs = []\n        for a1, holder in self.cells.items():\n            content = self.text_of(a1)")
b = src.index("    def edit(self, formula")
src = src[:a] + '''        retexts = []
        for a1, holder in self.cells.items():
            content = self.text_of(a1)
            text = content[0] if content else None
            if holder.bound or text == holder.string:
                continue
            old, new = holder.text, self._cell_label(a1, *content) if content else None
            glow = _box(*self.rect(a1), GREEN, None, 0, 0.0)
            holder.add(glow)
            if new is not None:
                new.set_opacity(0)
                holder.add(new)
            retexts.append((holder, old, new, glow))
            holder.text, holder.string = new, text
        self.state["rev"] += 1
        return Glide(self, self.gridlines, retexts=retexts, **kwargs)

''' + src[b:]
swap('''"""♻️ Show a new workbook state: every cell whose text changed morphs into its new text (cells filled by ``bind`` stay live); returns one animation."""''', '''"""♻️ Show a new workbook state: every cell whose text changed is retyped in place (the old text fades out, the new text fades in at the same position, the cell flashes green once); glyphs never leave their cell. Cells filled by ``bind`` stay live; returns one ``Glide``."""''')
open(path, "w", encoding="utf-8").write(src)
