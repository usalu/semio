import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()


def rep(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b, 1)


rep('"Podium length": "Länge Sockel", "Podium width": "Breite Sockel"}', '"Podium length": "Länge Sockel", "Podium width": "Breite Sockel",\n          "Capsule per slot": "Kapsel pro Slot", "Cull indices": "Cull-Indizes"}')
rep('return self.gh_param(spec.outputs[0].type, at=at, label=s.get("label"), selected=selected)',
    'label = s.get("label")\n            label = label if self.language() == "en" else GERMAN.get(label, label)\n            return self.gh_param(spec.outputs[0].type, at=at, label=label, selected=selected)')
rep('["Plane", "O  "', '[g("Plane", "Ebene"), "O  "')
rep("    def _custom(self, defn, sol, layout, group, extra=(), widths=None):", "    def _custom(self, defn, sol, layout, group, extra=(), widths=None, panels=None):")
rep("self._widget(oid, at, sol, {}, (widths or {}).get(oid)) for oid, at in layout.items()}", "self._widget(oid, at, sol, {}, (widths or {}).get(oid), lines=(panels or {}).get(oid)) for oid, at in layout.items()}")
rep('''            net = self._custom(defn, sol, swap_layout, "capsules", widths={"all": 0.8})''', '''            labelled = {oid: {(0,): [f"{self.tr('Type', 'Typ')} {key}: {self._fmt(sol.items(oid)[0])}"]} for oid, key in (("rb", old_key), ("rd", new_key))}
            net = self._custom(defn, sol, swap_layout, "capsules", widths={"all": 0.8, "rb": 1.9, "rd": 1.9}, panels=labelled)''')
rep('"rb": (1.6, 1.7), "rd": (1.6, -0.5)}', '"rb": (1.3, 1.7), "rd": (1.3, -0.5)}')
# nowire: no Indicate on a live object, settle before capture
rep('self.step("nowire", e.Indicate(e.VGroup(*net_live.wires), color=GOOD, scale_factor=1.0), k.animate.set_value(high // 2), seconds=4.0)',
    'same = self._say(g("same wires, other value", "gleiche Verbindungen, anderer Wert"), (-3.0, -2.35), 24)\n        self.step("nowire", e.FadeIn(same), k.animate.set_value(high // 2), seconds=4.0)\n        self.wait(0.1)')
rep("        self.play(k.animate.set_value(high), run_time=2.5)\n", "        self.play(k.animate.set_value(high), e.FadeOut(same), run_time=2.5)\n")
# sigma inside the group
s = s.replace("(-2.3, -2.4), 30)", "(-2.6, -1.45), 30)")
# wires off slider labels
rep('"ser": (-1.9, 1.4), "ori": (0.3, 1.4)', '"ser": (-1.4, 1.4), "ori": (0.7, 1.4)')
rep('"cw": (-1.9, 1.0), "capital": (1.0, 0.6)', '"cw": (-1.3, 1.0), "capital": (1.2, 0.6)')
rep('"tot": (-1.7, 1.5), "chh": (-1.7, 0.0), "czz": (-0.3, 0.75), "capital": (1.3, 0.75)', '"tot": (-1.4, 1.5), "chh": (-1.4, 0.0), "czz": (-0.1, 0.75), "capital": (1.4, 0.75)')
rep('"uzp": (0.2, -0.9), "ext": (1.4, 0.3)', '"uzp": (0.2, -1.0), "ext": (1.0, 0.2)')
open(p, "w", encoding="utf-8").write(s)
