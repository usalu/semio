"""[DEBUG] One-off edit: LiveGroup with deferred rebuild after animations."""
from pathlib import Path

room = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py")
source = room.read_text(encoding="utf-8")
start = source.index("class LiveGroup(_engine.VGroup):")
end = source.index("def clone(mob):")
new = '''class LiveGroup(_engine.VGroup):
    """♻️ A widget rebuilt from its builder every frame, safe for every animation.

    While an animation runs on it the children stay frozen; the rebuild after the animation is deferred to the next frame, so ``FadeOut`` cleanup and
    every ``interpolate`` see the structure they started with. Copies (used by ``FadeIn``, ``FadeOut``, ``Transform``, ``Indicate``) are fresh static snapshots.
    """

    def __init__(self, builder):
        super().__init__(*builder().submobjects)
        self._builder, self._deferred = builder, False
        self.add_updater(lambda mob: mob.refresh())

    def refresh(self):
        """🔄 Rebuild the children from the current trackers unless an animation has just finished."""
        if not self._deferred:
            self.submobjects = self._builder().submobjects
        return self

    def resume_updating(self, recursive=True):
        """▶️ Resume without rebuilding in the middle of an animation's finish."""
        self._deferred = True
        try:
            return super().resume_updating(recursive)
        finally:
            self._deferred = False

    def __deepcopy__(self, memo):
        self.refresh()
        twin = super().__deepcopy__(memo)
        twin.updaters, twin._builder = [], None
        return twin


'''
source = source[:start] + new + source[end:]
old = '''        holder = LiveGroup(*builder().submobjects)
        holder.add_updater(lambda mob: setattr(mob, "submobjects", builder().submobjects))
        return holder'''
assert old in source
source = source.replace(old, "        return LiveGroup(builder)", 1)
room.write_text(source, encoding="utf-8")
