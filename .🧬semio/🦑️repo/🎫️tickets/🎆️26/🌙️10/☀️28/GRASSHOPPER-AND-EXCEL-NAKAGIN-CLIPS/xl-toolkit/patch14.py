import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap('''    shape = _fonts.body_text(text, font_size=size, color=color, **kwargs)
    shape.descent = _drafting.descent(size) if _drafting.DESCENDERS & set(text) else 0.0''', '''    if any(char.isalnum() for char in text):
        shape = _fonts.body_text(text, font_size=size, color=color, **kwargs)
        shape.descent = _drafting.descent(size) if _drafting.DESCENDERS & set(text) else 0.0
    else:
        full = _fonts.body_text("H" + text + "H", font_size=size, color=color, **kwargs)
        shape = _engine.VGroup(*full.submobjects[1:-1])
        shape.descent = full.submobjects[0].get_bottom()[1] - shape.get_bottom()[1]''')
swap("import math\nimport re\n", "import math\nimport re\nimport weakref\n")
swap('''        self.scene, self.view, self.sheet, self.language, self.show, self.ribbon_key = scene, view, sheet, language, show, ribbon''', '''        self._scene = weakref.ref(scene)
        self.view, self.sheet, self.language, self.show, self.ribbon_key = view, sheet, language, show, ribbon''')
swap('''    @property
    def right(self) -> float:''', '''    @property
    def scene(self):
        """🎬 The owning scene, held weakly so animations can deep-copy the window."""
        return self._scene()

    @property
    def right(self) -> float:''')
open(path, "w", encoding="utf-8").write(src)
