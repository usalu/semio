import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


rep('"tot": (-2.3, 1.5), "chh": (-2.3, 0.0), "czz": (-0.7, 0.75), "capital": (1.2, 0.75)}', '"tot": (-1.7, 1.5), "chh": (-1.7, 0.0), "czz": (-0.3, 0.75), "capital": (1.3, 0.75)}')
rep("for i in (1, self._pods, self.B.storeys)])", "for i in (self._pods, self.B.storeys)])")
rep("self.number(i, 0), size=16, length=0.3, side=-1)", "self.number(i, 0), size=16, length=0.4, side=-1, color=TXT)")
rep("self.B.center[0] - self.B.core_side / 2, self.B.center[1] - self.B.core_side / 2, i * self.B.storey_height)", "self.B.center[0] - self.B.core_side / 2, self.B.center[1] + self.B.core_side / 2, i * self.B.storey_height)")
rep("self._axes(closeup, self._plane(p), length=2.6, labels=True) for p in facade_planes", "self._axes(closeup, self._plane(p), length=2.2, labels=False) for p in facade_planes")
rep('"ori": (0.6, 0.2)}, group="capsules")', '"ori": (0.6, 0.2)}, group="capsules", bridges=(("xy", "Plane", "ori", "Source"),))')
i = s.index("        old_key, new_key")
j = s.index("        final = self.tower_axon")
new = '''        old_key, new_key = "B", "D"
        before = {key: types[key].count for key in types}
        after = {key: (0 if key == old_key else before[key] + (before[old_key] if key == new_key else 0)) for key in types}
        old_list = self.gh_value_list(list(types), list(types).index(old_key), at=(-4.6, 0.9), name="Types")
        new_list = self.gh_value_list(list(types), list(types).index(new_key), at=(-4.6, 0.9), name="Types")
        old_panel = self.gh_panel([f"{key}  {before[key]}" for key in types], at=(-1.6, 0.2), width=1.6)
        new_panel = self.gh_panel([f"{key}  {after[key]}" for key in types], at=(-1.6, 0.2), width=1.6)
        swap_group = self.gh_group(e.VGroup(old_list, old_panel), name("capsules"), color=COLORS["capsules"])
        recolor = self.tower_axon(at=(VX, VY), scale=0.1, tint=lambda key: _nakd.type_color(new_key) if kind[key] == old_key else None)
        self._fit(recolor, viewport)
        self.play(e.FadeOut(sum_panel), e.FadeOut(sliders), e.FadeOut(a_panel), e.FadeOut(a_group), e.FadeIn(old_list), e.FadeIn(old_panel), e.FadeIn(swap_group), run_time=1.5)
        self.step("swap", e.ReplacementTransform(old_list, new_list), e.ReplacementTransform(old_panel, new_panel), e.FadeOut(lit_all), e.FadeIn(recolor), seconds=5.0)

'''
s = s[:i] + new + s[j:]
rep("e.FadeOut(count_text), e.FadeOut(swap_group), e.FadeOut(new_list), e.FadeOut(recolor), e.FadeIn(final), e.FadeIn(legend),", "e.FadeOut(swap_group), e.FadeOut(new_list), e.FadeOut(new_panel), e.FadeOut(recolor), e.FadeIn(final), e.FadeIn(part_cards),")
open(p, "w", encoding="utf-8").write(s)
