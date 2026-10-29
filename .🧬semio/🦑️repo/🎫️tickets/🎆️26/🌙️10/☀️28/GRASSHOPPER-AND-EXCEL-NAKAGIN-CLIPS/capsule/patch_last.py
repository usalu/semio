import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    if a not in s:
        print("MISSING", a[:70])
        return
    s = s.replace(a, b, 1)


rep('"lift", "wrong", "front", "dr")}', '"lift", "wrong", "front", "dr", "over")}')
rep('tone = "#C77700" if rv["hit"] > 0.5 else "#5E6A72"', 'tone = RED if rv["over"] > 0.5 else "#C77700" if rv["hit"] > 0.5 else "#5E6A72"')
rep('wires.append(self.gh_wire(self._grip(known[wire.src], wire.out, "out"), self._grip(known[wire.dst], wire.inp, "in"), kind=kind))',
    'wires.append(self.gh_wire(self._grip(known[wire.src], wire.out, "out"), self._grip(known[wire.dst], wire.inp, "in"), kind=kind))\n                wires[-1].pair = (wire.src, wire.dst)')

rep('"Pull the width slider to its upper limit: the capsule reaches the width of the envelope and the slider goes no further."',
    '"Push the width past its domain: the slider clamps at its upper limit and the envelope turns red. The domain is a design rule."')
rep('"Ziehen Sie den Breiten-Slider ans obere Ende: Die Kapsel erreicht die Breite der Hülle, weiter geht der Slider nicht."',
    '"Schieben Sie die Breite über die Domain hinaus: Der Slider hält am oberen Ende an, die Hülle wird rot. Die Domain ist eine Entwurfsregel."')

rep('''        self.step("limit", _engine.Succession(self._ramp(vals["w"], value("w"), width_hi, 3.0), _engine.AnimationGroup(self._to(rv["hit"], 1.0, 0.4), _engine.FadeIn(limit_badge))), seconds=6.5)''',
    '''        slider_box = sliders["w"].view.body
        red_box = _engine.Rectangle(width=slider_box.width + 0.1, height=slider_box.height + 0.1, stroke_color=RED, stroke_width=4, fill_opacity=0).move_to(slider_box.get_center())
        request = self._tag(self.tr("requested ", "angefragt ") + self.quantity(width_hi + 0.5, "m") + " → " + self.quantity(width_hi, "m"), (-1.15, -2.15, 0.0), color="#B3261E")
        self.step("limit", _engine.Succession(self._ramp(vals["w"], value("w"), width_hi, 2.4), _engine.AnimationGroup(self._to(rv["hit"], 1.0, 0.4), _engine.FadeIn(limit_badge)),
                                              _engine.AnimationGroup(self._to(rv["over"], 1.0, 0.5), _engine.FadeIn(red_box), _engine.FadeIn(request)), _engine.Wait(1.0)), seconds=7.5)''')
rep('self.step("types", _engine.AnimationGroup(*fout([card, limit_badge]),', 'self.step("types", _engine.AnimationGroup(*fout([card, limit_badge, red_box, request]), self._to(rv["over"], 0.0, 0.5),')

pulse = '''
        def pulse(wires, steps):
            found = []
            for pair, color in steps:
                for wire in wires:
                    if getattr(wire, "pair", None) == pair:
                        found.append(_engine.Indicate(wire, color=color, scale_factor=1.0, run_time=0.8))
            return _engine.LaggedStart(*found, lag_ratio=0.6)

        flow = "#3F6FB5"
'''
rep('        mode["engine"] = True\n        lifted, lift_wires =', pulse + '        mode["engine"] = True\n        lifted, lift_wires =')
rep('''self._to(rv["lift"], 1.0, 2.0), self._to(rv["plane2"], 1.0, 1.2)),
                  seconds=7.0)''', '''self._to(rv["lift"], 1.0, 2.0), self._to(rv["plane2"], 1.0, 1.2),
                                                 _engine.Succession(_engine.Wait(1.5), pulse(lift_wires, [(("l", "hl"), PARAM_COLOR["l"]), (("h", "hh"), PARAM_COLOR["h"]), (("hl", "bp"), PARAM_COLOR["l"]),
                                                                                                       (("hh", "bp"), PARAM_COLOR["h"]), (("bp", "bo"), flow), (("bo", "box"), flow)]))),
                  seconds=8.0)''')
rep('''_engine.Succession(_engine.Wait(1.6), self._ramp(vals["wu"], 0.0, offset, 1.8), self._ramp(vals["wu"], offset, 0.0, 1.6))), seconds=7.0)''',
    '''_engine.Succession(_engine.Wait(1.6), self._ramp(vals["wu"], 0.0, offset, 1.8), self._ramp(vals["wu"], offset, 0.0, 1.6)),
                                                       _engine.Succession(_engine.Wait(1.2), pulse(front_wires, [(("l", "wpt"), PARAM_COLOR["l"]), (("wu", "wpt"), PARAM_COLOR["wu"]), (("hh", "wpt"), PARAM_COLOR["h"]),
                                                                                                                  (("wpt", "wpl"), flow), (("xz", "wpl"), flow)]))), seconds=8.0)''')

rep('''        self.step("recap", _engine.AnimationGroup(*fout(cur), _engine.LaggedStart(*fin(legend), lag_ratio=0.35)), seconds=9.0)''',
    '''        core = self._core_wrap
        slot = self._wrap_slots[0]
        ring = lambda point, radius=0.4: _engine.Circle(radius=radius, stroke_color="#FFB000", stroke_width=6, fill_opacity=0).move_to(point)
        strip_box = _engine.Rectangle(width=4.5, height=0.95, stroke_color="#FFB000", stroke_width=5, fill_opacity=0).move_to((VP[0], VP[1] - 2.05, 0))
        targets = [None, None, ring(core.at(*(np.array(slot.facade_plane.origin) + np.array([0.0, 2.0, 1.25]))), 0.6), None, ring(core.at(*slot.facade_plane.origin), 0.3),
                   ring(core.at(*self._wrap_slots[1].facade_plane.origin), 0.3), strip_box, ring((VP[0] - 1.9 + 3.8 / 7 * 4, VP[1] - 2.0, 0), 0.35)]
        glow = [_engine.Succession(_engine.FadeIn(legend[i], run_time=0.5), _engine.AnimationGroup(_engine.Indicate(legend[i], color="#FFB000", scale_factor=1.06, run_time=0.9),
                                                                                               *([_engine.Succession(_engine.FadeIn(targets[i], run_time=0.25), _engine.Wait(0.4), _engine.FadeOut(targets[i], run_time=0.25))] if targets[i] is not None else [])))
                for i in range(len(legend))]
        self.step("recap", _engine.AnimationGroup(*fout(cur), _engine.Succession(*glow)), seconds=11.0)''')
open(p, "w", encoding="utf-8").write(s)
