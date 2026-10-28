"""Round 3 patch of the experiments scene: block replacement plus literal replacements; retries while another process holds the file."""
import re
import sys
import time
from pathlib import Path

T = Path(__file__).resolve().parent
scene = next(Path.cwd().glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
text = scene.read_text(encoding="utf-8")

start = text.index("    def _term_rows(self, trackers):")
end = text.index("    def _replicates(self):")
text = text[:start] + (T / "exp-r3-model-block.txt").read_text(encoding="utf-8") + "\n" + text[end:]

REPLACEMENTS = [
    ('_text = _visuals._text\n', '_text = _visuals._text\n_fonts = _visuals._fonts\n'),
    ('.85, (-3.6, -1.85, 0), (-.25, -1.85, 0), 3.2', '.85, (-3.6, -1.85, 0), (.1, -1.85, 0), 3.2'),
    ('.6, (-4.2, -1.5, 0), -1.75, -1.5, 2.5, .8, 4.35', '.6, (-4.2, -1.5, 0), -1.75, -1.5, 2.5, .9, 4.6'),
    ('AXES_POINT, AXES_WIDTH, AXES_HEIGHT, X_RANGE = (4.05, -.6, 0), 4.6, 2.3', 'AXES_POINT, AXES_WIDTH, AXES_HEIGHT, X_RANGE = (4.2, -.6, 0), 4.4, 2.3'),
    ('        keep = {id(self._caption), id(self.source)}\n', '        keep = {id(self._caption), id(self.source), id(getattr(self, "heading", None))}\n'),
    ('        keep = {id(self.big), id(self._caption), id(self.source)}\n', '        keep = {id(self.big), id(self._caption), id(self.source), id(self.heading)}\n'),
    ('    def _t(self, value, point, size=18, color=INK, width=6.0):', '''    def _head(self, en, de):
        """📑 Swap the standalone instructional heading chip of the current act."""
        new = _fonts.section_heading(self.tr(en, de))
        old, self.heading = getattr(self, "heading", None), new
        return [_mn.FadeOut(old), _mn.FadeIn(new)] if old is not None else [_mn.FadeIn(new)]

    def _t(self, value, point, size=18, color=INK, width=6.0):'''),
    ('self.step("question", _mn.FadeIn(self.big), _mn.FadeIn(compass), seconds=2)', 'self.step("question", *self._head("Question: two design factors, one response", "Frage: zwei Entwurfsfaktoren, eine Zielgröße"), _mn.FadeIn(self.big), _mn.FadeIn(compass), seconds=2)'),
    ('title = self._t(self.tr("How do window share and overhang change\\nthe summer cooling electricity?", "Wie verändern Fensteranteil und Vordach\\nden sommerlichen Kühlstrom?"), (PANEL_X, 2.0, 0), 26, DATA, 6.2)',
     'title = self._t(self.tr("Cooling electricity = f(A, B)", "Kühlstrom = f(A, B)"), (PANEL_X, 1.8, 0), 30, DATA, 6.2)'),
    ('facade = _mn.VGroup(_mn.Rectangle(width=.42, height=.3, color=INK, stroke_width=2), _mn.Line((-.21, -.15, 0), (.21, -.15, 0), color=_room.GLASS, stroke_width=5))',
     'facade = _mn.VGroup(_mn.Rectangle(width=.5, height=.375, color=INK, stroke_width=2), _mn.Line((-.25, -.1875, 0), (.25, -.1875, 0), color=_room.GLASS, stroke_width=5), _mn.Arrow((0, -.2, 0), (0, -.36, 0), buff=0, color=_room.GLASS, stroke_width=3, tip_length=.08))'),
    ('self.step("factors", _mn.FadeOut(self.question_card)', 'self.step("factors", *self._head("Factors and levels", "Faktoren und Stufen"), _mn.FadeOut(self.question_card)'),
    ('self.step("plan", _mn.LaggedStart(', 'self.step("plan", *self._head("Full plan: 2² = 4 simulations", "Vollständiger Plan: 2² = 4 Simulationen"), _mn.LaggedStart('),
    ('        self.runs_chip = self._chip(self.tr("2² = 4 simulations", "2² = 4 Simulationen"), (PANEL_X, -.75, 0), 20, GOOD)\n',
     '        self.runs_chip = self._chip(self.tr("2² = 4 simulations", "2² = 4 Simulationen"), (PANEL_X, -.85, 0), 20, GOOD)\n        self.order_note = self._t(self.tr("cards: A grows to the right, B grows upwards", "Karten: A wächst nach rechts, B nach oben"), (PANEL_X, -.3, 0), 15, MUTED, 6.0)\n'),
    ('self.step("plan_2", _mn.Succession(windows, slabs), _mn.FadeIn(self.runs_chip), seconds=4)', 'self.step("plan_2", _mn.Succession(windows, slabs), _mn.FadeIn(self.runs_chip), _mn.FadeIn(self.order_note), seconds=4)'),
    ('self.step("response", _mn.LaggedStart(*[self._shine(card)', 'self.step("response", *self._head("Simulate each room: sun in, cooling electricity out", "Jeden Raum simulieren: Sonne rein, Kühlstrom raus"), _mn.LaggedStart(*[self._shine(card)'),
    ('_mn.FadeOut(self.runs_chip), _mn.FadeOut(self.low_chip)', '_mn.FadeOut(self.runs_chip), _mn.FadeOut(self.order_note), _mn.FadeOut(self.low_chip)'),
    ('self._spot(run), ("up", "upleft", "left") if run["design"]["overhang"] == 0 else ("down", "downright", "right"))\n                            for i, run in enumerate(RUNS)])',
     'self._spot(run), ("up", "upleft", "left") if run["design"]["overhang"] == 0 else ("down", "downright", "right"), gap=.27 if run["design"]["overhang"] == 0 else .42)\n                            for i, run in enumerate(RUNS)])'),
    ('self.step("saving_30", _mn.FadeIn(veil_large)', 'self.step("saving_30", *self._head("The overhang saves more at a large window", "Das Vordach spart mehr bei großem Fenster"), _mn.FadeIn(veil_large)'),
    ('{self.tr(\'Direct sun stopped\', \'Direktsonne gestoppt\')}: {rays} {self.tr(\'rays\', \'Strahlen\')} × {self._kwh(RAY_BEAM)} × {self.number(shade, 3)} = {self._kwh(stopped, 0)}"',
     '{self.tr(\'Direct sun stopped\', \'Direktsonne gestoppt\')}: {rays} {self.tr(\'rays\', \'Strahlen\')} × {self._kwh(RAY_BEAM)} × {self.number(shade, 3)} {self.tr(\'shaded\', \'beschattet\')} = {self._kwh(stopped, 0)}"'),
    ('(f"{self.tr(\'cooling electricity saved\', \'Kühlstrom gespart\')} {self.number(saved[0], 1)} ÷ {self.number(saved[1], 1)} = {self.number(saved[0] / saved[1], 1)}", SHADED, 16)',
     '(f"{self.tr(\'cooling electricity effect\', \'Kühlstrom-Effekt\')} {self.number(-saved[0], 1)} ÷ ({self.number(-saved[1], 1)}) = {self.number(saved[0] / saved[1], 1)}", SHADED, 16)'),
    ('self.step("main", _mn.FadeOut(self.savings)', 'self.step("main", *self._head("Effects from the four runs", "Effekte aus den vier Läufen"), _mn.FadeOut(self.savings)'),
    ('self.step("answer", _mn.FadeIn(static)', 'self.step("answer", *self._head("Answer for this room", "Antwort für diesen Raum"), _mn.FadeIn(static)'),
    ('        names = [(self.tr("mean", "Mittel"), COOL), (self.tr("window", "Fenster"), DESIGN), (self.tr("overhang", "Vordach"), SHADED), ("A·B", MIXED), (self.tr("result", "Ergebnis"), COOL)]',
     '        names = [(self.tr("mean", "Mittel"), COOL), (self.tr("window", "Fenster"), DESIGN), (self.tr("overhang", "Vordach"), OPEN), ("A·B", MIXED), (self.tr("result", "Ergebnis"), COOL)]'),
    ('_mn.Line((left, top - height, 0), (edges[-1], top - height, 0), color=MUTED, stroke_width=1.4)', '_mn.Line((left, top - height - .05, 0), (edges[-1], top - height - .05, 0), color=MUTED, stroke_width=1.4)'),
    ('x0, floor = -5.3 + 6.6 * index, -1.6', 'x0, floor = -5.3 + 6.6 * index, -1.6'),
    ('base = np.array([x0 + 2.9, floor, 0.0])', 'base = np.array([x0 + 3.05, floor, 0.0])'),
    ('layers, _ = self._section(run["design"], (x - .2, -1.5, 0), .3, person=False)', 'layers, _ = self._section(run["design"], (x - .3, -1.5, 0), .34, person=False)'),
    ('self.step("purpose", _mn.LaggedStart(', 'self.step("purpose", *self._head("Purpose of the plan", "Zweck des Plans"), _mn.LaggedStart('),
    ('self.step("purpose_2", _mn.LaggedStart(', 'self.step("purpose_2", *self._head("Fractional plan: 4 instead of 8 simulations", "Teilplan: 4 statt 8 Simulationen"), _mn.LaggedStart('),
    ('self.step("repeat", _mn.FadeOut(self.big[1])', 'self.step("repeat", *self._head("Repeat: simulation versus measurement", "Wiederholen: Simulation gegen Messung"), _mn.FadeOut(self.big[1])'),
    ('self.step("repeat_2", _mn.FadeOut(pair)', 'self.step("repeat_2", _mn.FadeIn(self.big[1]), _mn.FadeIn(self.big[2]), _mn.FadeOut(pair)'),
    ('self._t(self.tr("measured kWh/a", "gemessene kWh/a"), (3.95, .62, 0), 15, MIXED, 2.4))', 'self._t(self.tr("assumed spread (schematic)", "angenommene Streuung (Schema)"), (3.95, .62, 0), 15, MIXED, 3.0))'),
    ('self._t(self.tr("each repeat measures a different value (schematic, no values)", "jede Wiederholung misst anders (Schema, keine Werte)"), (3.9, 1.65, 0), 16, INK, 6.0)',
     'self._t(self.tr("each repeat measures a different value\\n(schematic: the room on the left is the simulated one)", "jede Wiederholung misst anders\\n(Schema: der Raum links ist der simulierte)"), (3.9, 1.6, 0), 16, INK, 6.0)'),
    ('(self.tr("thicker wall keeps summer heat in (70 %, 1.0 m):", "dickere Wand hält Sommerwärme drin (70 %, 1,0 m):"), INK, 16)', '(self.tr("at this design (70 %, 1.0 m) a thicker wall keeps summer heat in:", "bei diesem Entwurf (70 %, 1,0 m) hält eine dickere Wand Sommerwärme drin:"), INK, 15)'),
    ('self.tr("all eight simulations: ", "alle acht Simulationen: ") + f"A·B {self._kwh(truth[\'ab\'], 1, True)} · C {self._kwh(truth[\'c\'], 1, True)}"', 'self.tr("all eight simulations (mean effects): ", "alle acht Simulationen (mittlere Effekte): ") + f"A·B {self._kwh(truth[\'ab\'], 1, True)} · C {self._kwh(truth[\'c\'], 1, True)}"'),
]

blocks = {}
form_start = text.index('        kwh = self._kwh\n        form_a = ')
form_end = text.index('        meaning = _mn.VGroup(')
new_forms = '''        kwh = self._kwh
        num = lambda value, digits=1: self.number(value, digits)
        diff = round(large - small, 1)
        form_a = self._formula_rows([[(self.tr("Effect A", "Effekt A"), DATA), "=", (self.tr("mean at 70 %", "Mittel bei 70 %"), DATA), "−", (self.tr("mean at 30 %", "Mittel bei 30 %"), DESIGN)],
                                     ["=", (f"({num(r3)} + {num(r4)}) ÷ 2", DATA), "−", (f"({num(r1)} + {num(r2)}) ÷ 2", DESIGN)],
                                     ["=", (num(a_high, 2), DATA), "−", (num(a_low, 2), DESIGN), "=", (kwh(a_effect, 1, True), DATA)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
        form_b = self._formula_rows([[(self.tr("Effect B", "Effekt B"), SHADED), "=", (self.tr("mean with 1.0 m", "Mittel mit 1,0 m"), SHADED), "−", (self.tr("mean without", "Mittel ohne"), OPEN)],
                                     ["=", (f"({num(r2)} + {num(r4)}) ÷ 2", SHADED), "−", (f"({num(r1)} + {num(r3)}) ÷ 2", OPEN)],
                                     ["=", (num(b_high, 2), SHADED), "−", (num(b_low, 2), OPEN), "=", (kwh(b_effect, 1, True), SHADED)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
        form_ab = self._formula_rows([[(self.tr("difference", "Differenz"), MIXED), "=", (self.tr("effect at 70 %", "Effekt bei 70 %"), MIXED), "−", (self.tr("effect at 30 %", "Effekt bei 30 %"), MIXED)],
                                      ["=", (kwh(large, 1, True), INK), "−", (f"({kwh(small, 1, True)})", INK), "=", (kwh(diff, 1, True), MIXED)],
                                      [("A·B", MIXED), "=", "½", "×", (f"({kwh(diff, 1, True)})", MIXED), "=", (kwh(ab_effect, 1, True), MIXED)]], (PANEL_X, 2.4, 0), size=20, width=6.3)
'''
text = text[:form_start] + new_forms + text[form_end:]

for old, new in REPLACEMENTS:
    count = text.count(old)
    if count != 1:
        sys.stdout.buffer.write(f"MISS {count}: {old[:80]!r}\n".encode("utf-8"))
        continue
    text = text.replace(old, new)

row = re.compile(r'^(\s+\["[^"]+", "(?:[^"\\]|\\.)*", ")((?:[^"\\]|\\.)*)("\],)$', re.M)
text = row.sub(lambda m: m.group(1) + re.sub(r"(\d) (m|%|kWh/a)\b", lambda n: n.group(1) + "\\u00a0" + n.group(2), m.group(2)) + m.group(3), text)

for attempt in range(40):
    try:
        scene.write_bytes(text.replace("\r\n", "\n").encode("utf-8"))
        break
    except OSError:
        time.sleep(3)
else:
    sys.stdout.buffer.write(b"WRITE FAILED\n")
sys.stdout.buffer.write(b"done\n")
