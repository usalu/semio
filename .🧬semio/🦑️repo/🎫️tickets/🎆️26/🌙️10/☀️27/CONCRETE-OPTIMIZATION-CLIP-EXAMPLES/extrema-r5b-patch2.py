import sys
sys.stdout.reconfigure(encoding="utf-8")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py"
s = open(p, encoding="utf-8").read()
def swap(a, b):
    global s
    assert s.count(a) == 1, a[:60]
    s = s.replace(a, b)
swap("def _conversion(self, part, point, size=15, width=6.4):", "def _conversion(self, part, point, size=15, width=6.4, short=False):")
swap('''            return [[(self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), soft), self.live(part + "_demand", soft, 0, "kWh/a"), "÷", (label, INK), self.live(factor, INK, 1), sign,''',
     '''            name = [] if short else [(self.tr("Heating demand", "Heizbedarf") if winter else self.tr("Cooling demand", "Kühlbedarf"), soft)]
            return [[*name, self.live(part + "_demand", soft, 0, "kWh/a"), "÷", (label, INK), self.live(factor, INK, 1), sign,''')
swap('self._conversion(part, (-.55, y, 0), width=5.2)', 'self._conversion(part, (-.55, y, 0), width=5.2, short=True)')
open(p, "w", encoding="utf-8").write(s)
