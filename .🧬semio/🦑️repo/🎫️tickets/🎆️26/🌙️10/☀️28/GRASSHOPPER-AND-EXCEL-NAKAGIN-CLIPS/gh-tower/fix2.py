import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)


rep("LC, LW, LH = (-2.55, -0.03), 8.9, 5.05\nVX, VY, VW, VH = 4.5, -0.03, 4.85, 5.05", "LC, LW, LH = (-2.45, -0.03), 9.15, 5.05\nVX, VY, VW, VH = 4.6, -0.03, 4.6, 5.05\nTXT = \"#1B1B1B\"")
rep("CAXON = np.array([VX, VY - 1.95, 0.0])\n", "")
rep("width=width or 3.0, selected=selected)", "width=width or 3.9, selected=selected)")
rep("group=None, panels=None):", "group=None, panels=None, extra=()):")
rep("frame = self.gh_group(_engine.VGroup(*made.values()),", "frame = self.gh_group(_engine.VGroup(*made.values(), *extra),")
i = s.index("    def present_tower(self):")
open(p, "w", encoding="utf-8").write(s[:i])
