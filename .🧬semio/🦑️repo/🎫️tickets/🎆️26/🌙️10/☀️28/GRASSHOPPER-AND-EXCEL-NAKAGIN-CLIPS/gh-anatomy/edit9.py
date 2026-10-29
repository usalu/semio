import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('''            ring = _engine.SurroundingRectangle(slider.body, buff=0.04, color=color, stroke_width=4, corner_radius=0.09)
            return _engine.VGroup(slider, ring).__class__(slider, ring) if False else self._ringed(slider, ring)
        return slider

    def _ringed(self, slider, ring):
        """💍 A slider widget that carries its colour ring and keeps grip access."""
        slider.add(ring)
        return slider
''', '''            slider.add(_engine.SurroundingRectangle(slider.body, buff=0.04, color=color, stroke_width=4, corner_radius=0.09))
        return slider
''')
rep("With nothing usable she turns red", "With nothing usable it turns red")
rep("self._tie(1, _at(-4.2, 1.5) + np.array([-0.25, 0, 0]), mini_chip.get_left())", "self._tie(1, mini_chip.get_right() + np.array([0.55, 0.0, 0.0]), mini_chip.get_right())")
open(p, "w", encoding="utf-8").write(s)
