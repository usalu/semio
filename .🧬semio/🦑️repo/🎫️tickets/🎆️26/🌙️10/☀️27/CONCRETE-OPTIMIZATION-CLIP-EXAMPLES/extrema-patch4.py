"""🩹 Patch the extrema scene: fold two clauses away and shorten travels to land near four minutes."""
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")


def swap(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:70])
    text = text.replace(old, new)


text = re.sub(r'        \["domain_2", .*\],\n', "", text)
text = re.sub(r'        \["maximize_2", .*\],\n', "", text)
text = re.sub(r'        \["domain", .*\],\n', '        ["domain", "We turn the façade from east, 90°, through all bearings and record the electricity in kWh/a.", "Wir drehen die Fassade ab Osten, 90°, durch alle Richtungen und messen den Strom in kWh/a."],\n', text)
text = re.sub(r'        \["maximize", .*\],\n', '        ["maximize", "Now the opposite: how many kWh and CHF per year do we save against an east façade?", "Nun umgekehrt: Wie viele kWh und CHF pro Jahr sparen wir gegenüber einer Ostfassade?"],\n', text)
swap('''self._travel(dial, [90, 180, 270, 360, 180]).set_run_time(5)), seconds=5)''', '''self._travel(dial, [90, 180, 270, 360, 180]).set_run_time(4.5)), seconds=4.5)''')
swap('''        self.step("formulate_2", _Appear(m["strip"]), seconds=1.5)
        self.play(self._travel(dial, [180, 90, 450, 180]).set_run_time(3.5))''', '''        self.step("formulate_2", _mn.AnimationGroup(_Appear(m["strip"], run_time=1.2), self._travel(dial, [180, 90, 450, 180]).set_run_time(3.2)), seconds=3.2)''')
swap('''_Appear(x_label), _Appear(ticks),
                  self.set_design(self.dials, {"orientation": 90.0}, 1.2), seconds=1.4)''', '''_Appear(x_label), _Appear(ticks), _Appear(y_labels),
                  self.set_design(self.dials, {"orientation": 90.0}, 1.2), seconds=1.6)''')
swap('''        self.step("domain_2", _Appear(y_labels), seconds=1.2)\n''', '')
swap('''follow=[self.reveal]), seconds=4)''', '''follow=[self.reveal]), seconds=3)''')
swap('''self._travel(dial, [180, 270], follow=[self.reveal]), seconds=3.5)''', '''self._travel(dial, [180, 270], follow=[self.reveal]), seconds=2.5)''')
swap('''self._travel(dial, [270, 360], follow=[self.reveal]), seconds=3.2)''', '''self._travel(dial, [270, 360], follow=[self.reveal]), seconds=2.2)''')
swap('''self._travel(dial, [360, 450], follow=[self.reveal]), seconds=2.5)''', '''self._travel(dial, [360, 450], follow=[self.reveal]), seconds=1.8)''')
swap('''set_run_time(4.5), seconds=4.5)''', '''set_run_time(3.5), seconds=3.5)''')
swap('''self._travel(dial, [stop, study["global_minimum"]]).set_run_time(3), self._out(probes),''', '''self._travel(dial, [stop, study["global_minimum"]]).set_run_time(2.5), self._out(probes),''')
swap('''_Appear(legend), _mn.Create(level), seconds=3)''', '''_Appear(legend), _mn.Create(level), seconds=2.5)''')
swap('''beat=.5).set_run_time(4), seconds=4)''', '''beat=.5).set_run_time(3), seconds=3)''')
swap('''_mn.Create(line), _Appear(line_tag), seconds=1.6)''', '''_mn.Create(line), _Appear(line_tag), _Appear(formula), seconds=1.8)''')
swap('''        self.step("maximize_2", _Appear(formula), seconds=1.4)\n''', '')
swap('''.set_run_time(3), seconds=3)
        m.update(curve=curve''', '''.set_run_time(2.5), seconds=2.5)
        m.update(curve=curve''')
swap('''.set_run_time(3.5), _mn.Indicate(band, color=WARN), seconds=3.5)''', '''.set_run_time(3), _mn.Indicate(band, color=WARN), seconds=3)''')
path.write_text(text, encoding="utf-8")
