import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()


def swap(old, new):
    global src
    assert old in src, old[:70]
    src = src.replace(old, new, 1)


a = src.index("class Glide(_engine.Animation):")
b = src.index("class Window(_engine.VGroup):")
src = src[:a] + '''class Glide(_engine.Animation):
    """🛝 Morph parts of a window in place. It never adds anything to the scene (the parts keep their place in the window's drawing order, so the tint stays under the text and the frames above), it does not suspend the window's live parts, and the whole window counts as moving so Cairo draws it in one pass."""

    def __init__(self, window, anchor, pairs, **kwargs):
        self.window = window
        self.parts = [_engine.Transform(part, target) for part, target in pairs]
        super().__init__(anchor, introducer=True, **kwargs)

    @property
    def animations(self) -> list:
        """🎞️ The animation list of a group, so scenes may splice it into a step."""
        return [self]

    def _setup_scene(self, scene):
        return None

    def get_all_mobjects(self):
        return (self.window, self.mobject)

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


''' + src[b:]
swap("return Glide(self, [(self.tint, tint), (self.active, active)], **kwargs)", "return Glide(self, self.tint, [(self.tint, tint), (self.active, active)], **kwargs)")
swap("return Glide(self, pairs, **kwargs)", "return Glide(self, self.gridlines, pairs, **kwargs)")
open(path, "w", encoding="utf-8").write(src)
