import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)


rep("""        self._fit(build_a, viewport)
        build_b =""", """        self._fit(build_a, viewport, 0.7)
        build_a.shift(np.array([0.0, -0.2, 0.0]))
        build_b =""")
rep("""        self._fit(build_b, viewport)
        facade_count""", """        self._fit(build_b, viewport, 0.7)
        build_b.shift(np.array([0.0, -0.2, 0.0]))
        facade_count""")
rep("self._fit(lit_all, viewport)", "self._fit(lit_all, viewport, 0.7)\n        lit_all.shift(np.array([0.0, -0.2, 0.0]))")
rep("np.array([VX - 2.0, VY + 1.95, 0.0])", "np.array([VX - 2.05, VY + 2.05, 0.0])")
rep("np.array([VX - 2.0, VY + 1.65, 0.0])", "np.array([VX - 2.05, VY + 1.78, 0.0])")
open(p, "w", encoding="utf-8").write(s)
