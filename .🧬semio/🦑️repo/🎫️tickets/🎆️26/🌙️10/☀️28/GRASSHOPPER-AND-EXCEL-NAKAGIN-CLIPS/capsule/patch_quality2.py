import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    if a not in s:
        print("MISSING", a[:70])
        return
    s = s.replace(a, b, 1)


rep('tower = self.tower_axon(at=VP + np.array([0.0, -0.1, 0.0]), scale=0.12,', 'tower = self.tower_axon(at=VP + np.array([0.0, 0.45, 0.0]), scale=0.105,')
rep('{"hl": (-2.0, 1.55), "hh": (-2.0, 0.55), "two": (-3.2, 1.05), "bp": (-0.8, -0.95), "bo": (0.7, -1.9)}', '{"hl": (-1.6, 1.55), "hh": (-1.6, 0.55), "two": (-2.95, 1.05), "bp": (-0.6, -0.95), "bo": (0.8, -1.9)}')
rep('{"hh": (-2.0, 0.55), "two": (-3.2, 1.05), "wpt": (-0.7, -0.6), "xz": (-1.9, -1.9), "wpl": (0.8, -1.5)}', '{"hh": (-1.6, 0.55), "two": (-2.95, 1.05), "wpt": (-0.5, -0.7), "xz": (-1.9, -1.9), "wpl": (0.9, -1.5)}')
rep('''            if row == 0:
                cells[0].set_fill(opacity=0.22)
''', '')
rep('_engine.FadeIn(table), _engine.FadeIn(minis), self._ramp(vals["w"], width_hi, value("w"), 1.5)', '_engine.FadeIn(table), _engine.FadeIn(minis), _engine.FadeIn(marker), self._ramp(vals["w"], width_hi, value("w"), 1.5)')
rep('self.step("four", _engine.AnimationGroup(_engine.FadeIn(marker), _engine.FadeIn(typetag),', 'self.step("four", _engine.AnimationGroup(_engine.FadeIn(typetag),')
rep('at=(0.3, -0.35, 0), width=3.0)', 'at=(0.3, -1.0, 0), width=3.0)')
rep('(front if capsule.facing >= 0 else ghost).add(capsule if capsule.facing >= 0 else _fade(capsule, 0.7))', '(front if capsule.facing >= 0 else ghost).add(capsule)')
open(p, "w", encoding="utf-8").write(s)

s = open(p, encoding="utf-8").read()
a = s.index("        order = [types[0], types[1], types[3], types[2]]")
b = s.index("        steps = [_engine.AnimationGroup(self._ramp(ty")
block = s[a:b]
s = s[:a] + s[b:]
c = s.index('        self.step("types"')
s = s[:c] + block + s[c:]
open(p, "w", encoding="utf-8").write(s)
