import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b, 1)


rep('f"{count} × {self.number(self.B.storey_height)} = {self.number(height)} m", offset=0.6, color=WARN)', 'self.quantity(height, "m"), offset=0.6, color=WARN)')
rep('''        self.step("core", e.FadeOut(part_cards),''', '''        equation = self.live_label(lambda: f"{step_n() + self._pods} × {self.number(self.B.storey_height)} m = {self.number((step_n() + self._pods) * self.B.storey_height)} m", (-3.0, -2.3), size=24, color=TXT)
        self.step("core", e.FadeIn(equation), e.FadeOut(part_cards),''')
rep("self.step(\"series\", e.FadeOut(core_net),", "self.step(\"series\", e.FadeOut(equation), e.FadeOut(core_net),")
# items: one pitch dimension
a = s.index("        step_dims = e.VGroup(")
b = s.index("\n", s.index("for i in range(items)])", a))
s = s[:a] + '''        step_dims = e.VGroup(self.dimension(self._pt(closeup, *self._plane(first[0]).origin), self._pt(closeup, *self._plane(first[1]).origin),
                                            self.quantity(abs(float(first[1].origin.t[0] - first[0].origin.t[0])), "m"), offset=-0.55, color=DATA))''' + s[b:]
# capital stub
rep("stub = self.core_axon(self.B, at=(VX, VY), scale=0.4, storeys=2)\n        self._pin(stub, (self.B.center[0], self.B.center[1], 2 * self.B.storey_height), (VX, VY - 0.7, 0.0))",
    "stub = self.core_axon(self.B, at=(VX, VY), scale=0.4, storeys=1)\n        self._pin(stub, (self.B.center[0], self.B.center[1], self.B.storey_height), (VX, VY - 0.2, 0.0))")
rep("(VX, VY - 0.7, 0.0))\n\n        cap_live", "(VX, VY - 0.2, 0.0))\n\n        cap_live")
# why: no arrows, wider cards
rep("box = e.RoundedRectangle(corner_radius=0.12, width=3.6, height=1.9,", "box = e.RoundedRectangle(corner_radius=0.12, width=5.4, height=1.9,")
rep("move_to(np.array([-3.4, 0.2, 0.0]))", "move_to(np.array([-2.45, 0.2, 0.0]))")
rep("        arrows = e.VGroup(*[e.Arrow(c.get_right(), c.get_right() + np.array([0.9, 0, 0]), color=TXT, buff=0.05) for c in cards])\n", "")
rep("e.FadeIn(podium_outline), e.LaggedStart(*[e.FadeIn(c) for c in cards], lag_ratio=0.4), e.FadeIn(arrows), seconds=7.0)", "e.FadeIn(podium_outline), e.LaggedStart(*[e.FadeIn(c) for c in cards], lag_ratio=0.4), seconds=7.0)")
rep("e.FadeOut(*cards), e.FadeOut(arrows), e.FadeOut(podium_outline)", "e.FadeOut(*cards), e.FadeOut(podium_outline)")
# tower tags in the viewport corner
rep('build_b.cores[0].get_top() + np.array([-0.2, 0.35, 0]), 18, tint_a)', 'np.array([VX - 2.0, VY + 1.95, 0.0]), 18, tint_a, "l")')
rep('build_b.cores[1].get_top() + np.array([0.3, 0.35, 0]), 18, tint_b)', 'np.array([VX - 2.0, VY + 1.65, 0.0]), 18, tint_b, "l")')
rep("at=(-1.0, -1.7), width=1.6)", "at=(-1.0, -2.0), width=1.6)")
rep("recolor.scale_to_fit_width(VW - 1.2).move_to(np.array([VX, VY - 0.9, 0.0]))", "recolor.scale_to_fit_width(VW - 1.7).move_to(np.array([VX, VY - 0.75, 0.0]))")
open(p, "w", encoding="utf-8").write(s)
