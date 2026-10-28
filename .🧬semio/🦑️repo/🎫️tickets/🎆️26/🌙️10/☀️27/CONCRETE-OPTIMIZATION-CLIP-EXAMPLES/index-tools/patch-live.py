from pathlib import Path

p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py")
s = p.read_text(encoding="utf-8")


def rep(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:80])
    s = s.replace(old, new)


rep('''def freeze(item):
    """🧊 Stop every updater in a group so it can fade out while the shared trackers move on."""
    for member in item.get_family():
        member.clear_updaters()
    return item
''', '''def has_live(item):
    """♻️ Whether a group contains a widget that rebuilds its children every frame."""
    return any(getattr(member, "is_live", False) or isinstance(member, getattr(_room, "LiveGroup", ())) for member in item.get_family())
''')
rep('''    holder = builder()
    holder.add_updater(lambda item: setattr(item, "submobjects", builder().submobjects))
    return holder''', '''    holder = builder()
    holder.is_live = True
    holder.add_updater(lambda item: setattr(item, "submobjects", builder().submobjects))
    return holder''')
rep('''        group = mn.VGroup(self.line, self.ticks) if only is None else mn.VGroup()
        for identifier in (self.compared if only is None else only):
            group.add(*self.members[identifier].values())
        if only is None:
            group.add(self.ring)
        return group
''', '''        group = mn.VGroup(self.line, self.ticks) if only is None else mn.VGroup()
        for identifier in (self.compared if only is None else only):
            group.add(*[self.members[identifier][key] for key in ("slot", "dot", "leader", "tag")])
        if only is None:
            group.add(self.ring)
        return group

    def norms(self, only=None):
        """🔢 The live normalized-value tags of the ruler (or of the given variants)."""
        return [self.members[identifier]["norm"] for identifier in (self.compared if only is None else only)]
''')
rep('''    def name(self, identifier, size=S0, width=1.5, color=INK, limit=14):''', '''    def show(self, *items, run_time=1.0):
        """✨ Fade widgets in by opacity: live widgets rebuild every frame, so they must not go through a Transform-based fade."""
        for item in items:
            item.suspend_updating()
            item.set_opacity(0)
            self.add(item)
        return mn.AnimationGroup(*[mn.UpdateFromAlphaFunc(item, lambda m, a: m.set_opacity(a), run_time=run_time, suspend_mobject_updating=True) for item in items])

    def hide(self, *items, run_time=1.0):
        """🫥 Fade widgets out by opacity and remove them afterwards."""
        fades = [mn.UpdateFromAlphaFunc(item, lambda m, a: m.set_opacity(1 - a), run_time=run_time, suspend_mobject_updating=True) for item in items]
        gone = [mn.UpdateFromFunc(item, lambda m: self.remove(m), run_time=.01, suspend_mobject_updating=True) for item in items]
        return mn.Succession(mn.AnimationGroup(*fades), mn.AnimationGroup(*gone))

    def clear_stage(self) -> None:
        """🧹 Fade everything but title and caption; live widgets fade by opacity."""
        content = [item for item in self.mobjects if item is not self._title and item is not self._caption]
        live_items = [item for item in content if has_live(item)]
        if content:
            self.play(*[mn.FadeOut(item) for item in content if item not in live_items], self.hide(*live_items, run_time=.5) if live_items else mn.Wait(.5), run_time=.5)

    def name(self, identifier, size=S0, width=1.5, color=INK, limit=14):''')
rep('''mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(pool)), mn.FadeIn(panel), ruler.appear(1.2), seconds=2.2)''',
    '''mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(pool)), self.show(panel, *ruler.norms()), ruler.appear(1.2), seconds=2.2)''')
rep('''        fold = mn.Succession(mn.AnimationGroup(mn.FadeOut(new.line), mn.FadeOut(new.ticks), mn.FadeOut(new_header), mn.FadeOut(new_panel),
                                               *[mn.FadeOut(new.members[i][k]) for i in pool for k in ("dot", "leader", "tag", "norm")], *moves),
                             mn.AnimationGroup(*swaps, mn.FadeIn(board.live), mn.FadeIn(legend), board.fill.animate(run_time=2.0).set_value(1.0)))
        self.step("matrix", mn.Succession(*run, mn.UpdateFromFunc(new_panel, freeze, run_time=.01), fold), seconds=5.6)''',
    '''        fold = mn.Succession(mn.AnimationGroup(mn.FadeOut(new.line), mn.FadeOut(new.ticks), mn.FadeOut(new_header), self.hide(new_panel, *new.norms()),
                                               *[mn.FadeOut(new.members[i][k]) for i in pool for k in ("dot", "leader", "tag")], *moves),
                             mn.AnimationGroup(*swaps, self.show(board.live), mn.FadeIn(legend), board.fill.animate(run_time=2.0).set_value(1.0)))
        self.step("matrix", mn.Succession(*run, fold), seconds=5.6)''')
rep('''        freeze(panel)
        keep = ("dot", "leader", "tag", "norm")''', '''        keep = ("dot", "leader", "tag")''')
rep('''            *[mn.FadeOut(old.members[i][k]) for i in pool for k in keep], mn.ReplacementTransform(header, new_header),
            mn.FadeOut(panel), mn.FadeIn(new_panel), mn.FadeIn(new.line), *[mn.FadeIn(new.members[i][k]) for i in pool for k in keep],''',
    '''            *[mn.FadeOut(old.members[i][k]) for i in pool for k in keep], mn.ReplacementTransform(header, new_header),
            self.hide(panel, *old.norms()), self.show(new_panel, *new.norms()), mn.FadeIn(new.line), *[mn.FadeIn(new.members[i][k]) for i in pool for k in keep],''')
rep('self.step("winner", mn.FadeIn(panel), seconds=1.6)', 'self.step("winner", self.show(panel), seconds=1.6)')
rep('''        self.add(share)
        self.step("sweep", mn.FadeIn(static), mn.Create(lines_), mn.FadeIn(names), mn.FadeIn(strip), mn.FadeIn(live(marker)), mn.FadeIn(panel),''',
    '''        self.add(share)
        self.step("sweep", mn.FadeIn(static), mn.Create(lines_), mn.FadeIn(names), mn.FadeIn(strip), self.show(live(marker), panel),''')
rep('''        self.step("range", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(five)), mn.FadeIn(ghosts),
                  mn.Succession(mn.Wait(.8), mn.FadeIn(ruler.mobjects([extra])), mn.FadeIn(panel)), seconds=3.0)''',
    '''        self.step("range", mn.FadeIn(header), mn.FadeIn(ruler.line), mn.FadeIn(ruler.mobjects(five)), mn.FadeIn(ghosts), self.show(*ruler.norms(five)),
                  mn.Succession(mn.Wait(.8), mn.AnimationGroup(mn.FadeIn(ruler.mobjects([extra])), self.show(panel, *ruler.norms([extra])))), seconds=3.0)''')
rep('''        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header), mn.FadeOut(panel), mn.FadeOut(ruler.members[extra]["slot"]),
                                *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag", "norm")])''',
    '''        out = mn.AnimationGroup(mn.FadeOut(ruler.line), mn.FadeOut(ruler.ticks), mn.FadeOut(ruler.ring), mn.FadeOut(ghosts), mn.FadeOut(header), mn.FadeOut(ruler.members[extra]["slot"]),
                                self.hide(panel, *ruler.norms(six)), *[mn.FadeOut(ruler.members[i][k]) for i in six for k in ("dot", "leader", "tag")])''')
rep('''        fold = mn.Succession(mn.AnimationGroup(*moves), mn.AnimationGroup(*swaps, mn.FadeIn(board_parts)))''',
    '''        fold = mn.Succession(mn.AnimationGroup(*moves), mn.AnimationGroup(*swaps, self.show(board.live), mn.FadeIn(mn.VGroup(axis, notes, heading))))''')
rep('''        board_parts = mn.VGroup(board.live, axis, notes, heading)
''', '')
p.write_text(s, encoding="utf-8")
print("patched")
