import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


swap("class Window(_engine.VGroup):", '''class Glide(_engine.Animation):
    """🛝 Morph parts of a window in place: the animation's mobject is the window itself, so the scene never lifts the parts out of the window's drawing order (tint stays under the text, frames above)."""

    def __init__(self, window, pairs, **kwargs):
        self.parts = [_engine.Transform(part, target) for part, target in pairs]
        super().__init__(window, **kwargs)

    def begin(self):
        for part in self.parts:
            part.begin()
        super().begin()

    def interpolate_mobject(self, alpha):
        for part in self.parts:
            part.interpolate(alpha)

    def finish(self):
        for part in self.parts:
            part.finish()
        super().finish()

    def clean_up_from_scene(self, scene):
        for part in self.parts:
            part.clean_up_from_scene(scene)
        super().clean_up_from_scene(scene)


class Window(_engine.VGroup):''')
swap('''        return _engine.AnimationGroup(_engine.Transform(self.tint, tint), _engine.Transform(self.active, active), **kwargs)''', '''        return Glide(self, [(self.tint, tint), (self.active, active)], **kwargs)''')
a = src.index("        animations = []\n        for a1, holder in self.cells.items():")
b = src.index("    def edit(self, formula")
src = src[:a] + '''        pairs = []
        for a1, holder in self.cells.items():
            content = self.text_of(a1)
            text = content[0] if content else None
            if holder.bound or text == holder.string:
                continue
            old, new = holder.text, self._cell_label(a1, *content) if content else None
            if old is not None and new is not None:
                pairs.append((old, new))
            elif new is not None:
                stub = new.copy().set_opacity(0)
                holder.add(stub)
                pairs.append((stub, new))
                holder.text = stub
            else:
                pairs.append((old, old.copy().set_opacity(0)))
                holder.text = None
            holder.string = text
        self.state["rev"] += 1
        return Glide(self, pairs, **kwargs)

''' + src[b:]
open(path, "w", encoding="utf-8").write(src)
