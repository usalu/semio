import re, sys, time
from pathlib import Path
scene = next(Path.cwd().glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
t = scene.read_text(encoding="utf-8")
def sub(old, new, count=1):
    global t
    n = t.count(old)
    if n != count:
        sys.stdout.buffer.write(f"MISS {n}: {old[:70]!r}\n".encode("utf-8")); return
    t = t.replace(old, new)

sub('loads = (round(RUNS[low]["cooling_demand"], 1), round(RUNS[high]["cooling_demand"], 1))', 'loads = (round(RUNS[low]["cooling_demand"]), round(RUNS[high]["cooling_demand"]))')
sub("{self.number(loads[0], 1)} → {self._kwh(loads[1])} ({self.number(loads[1] - loads[0], 1)})", "{self.number(loads[0])} → {self._kwh(loads[1], 0)} ({self.number(loads[1] - loads[0])})")
sub('(_room.chip([self.tr("window", "Fenster"), *_room.parts(design["wwr"]', '(_room.chip(["A ·", self.tr("window", "Fenster"), *_room.parts(design["wwr"]')
sub('over = _room.chip([self.tr("overhang", "Vordach"), *_room.parts(d, "m", 1, german)]', 'over = _room.chip(["B ·", self.tr("overhang", "Vordach"), *_room.parts(d, "m", 1, german)]')
sub('''reason = self._lines((self.tr("at this design (70 %, 1.0 m) a thicker wall keeps summer heat in:", "bei diesem Entwurf (70 %, 1,0 m) hält eine dickere Wand Sommerwärme drin:"), INK, 15),
                             (f"U-{self.tr('wall', 'Wand')} {self.number(thin['u_wall'], 2)} → {self.quantity(thick['u_wall'], 'W/(m²·K)', 2)} · {self.tr('cooling', 'Kühlung')} {self.number(thin['cooling'], 1)} → {self._kwh(thick['cooling'])}", COOL, 16), top=-2.0, width=6.4)''',
'''reason = self._lines((self.tr("at this design (70 %, 1.0 m) a thicker wall keeps summer heat in:", "bei diesem Entwurf (70 %, 1,0 m) hält eine dickere Wand Sommerwärme drin:"), INK, 15),
                             (f"U-{self.tr('wall', 'Wand')} {self.number(thin['u_wall'], 2)} → {self.quantity(thick['u_wall'], 'W/(m²·K)', 2)}", COOL, 16),
                             (f"{self.tr('cooling', 'Kühlung')} {self.number(thin['cooling'], 1)} → {self._kwh(thick['cooling'])}", COOL, 16), top=-1.82, gap=.3, width=5.7)''')
sub('(PANEL_X, -.5, 0), 19, ALERT, 6.4)', '(PANEL_X, -.4, 0), 19, ALERT, 5.7)')
sub('(PANEL_X, -1.0, 0), 17, INK, 6.4)', '(PANEL_X, -.88, 0), 16, INK, 5.7)')
sub('(PANEL_X, -1.5, 0), 16, OPEN, 6.4)', '(PANEL_X, -1.36, 0), 16, OPEN, 5.7)')

tags = '''
    def _column_tags(self, items):
        """🏷️ Coloured tags above or beside card frames with a short connector to the frame edge: the formula term and the drawn rooms it averages share one colour."""
        group = _mn.VGroup()
        for text, color, x, y, edge_y, side in items:
            label = _room.place(self._t(text, (0, 0, 0), 15, color, 2.6), (x, y, 0), "center" if side == "up" else "left")
            if side == "up":
                group.add(label, _mn.Line((x, y - .14, 0), (x, edge_y, 0), color=color, stroke_width=2.4))
            else:
                group.add(label, _mn.Line((x - .02, y, 0), (edge_y, y, 0), color=color, stroke_width=2.4))
        return group
'''
sub("    def _waterfall(self, trackers):", tags.lstrip("\n") + "\n    def _waterfall(self, trackers):")

sub('''        self.step("main", *self._head("Effects from the four runs", "Effekte aus den vier Läufen"), _mn.FadeOut(self.savings)''',
'''        top_edge = (CARD_Y[0] + CARD_Y[1]) / 2 + CARD_DY + (CARD_Y[0] - CARD_Y[1] + CARD_H) / 2
        tags_a = self._column_tags([(self.tr("mean at 30 %", "Mittel bei 30 %"), DESIGN, CARD_X[0], 2.98, top_edge, "up"), (self.tr("mean at 70 %", "Mittel bei 70 %"), DATA, CARD_X[1], 2.98, top_edge, "up")])
        tags_b = self._column_tags([(self.tr("mean with 1.0 m", "Mittel mit 1,0 m"), SHADED, CARD_X[0], 2.98, top_edge, "up"), (self.tr("mean without", "Mittel ohne"), OPEN, CARD_X[1] + CARD_W / 2 + .1, CARD_Y[1] + CARD_DY, CARD_X[1] + CARD_W / 2, "side")])
        tags_ab = self._column_tags([(self.tr("effect at 30 %", "Effekt bei 30 %"), MIXED, CARD_X[0], 2.98, top_edge, "up"), (self.tr("effect at 70 %", "Effekt bei 70 %"), MIXED, CARD_X[1], 2.98, top_edge, "up")])
        self.step("main", *self._head("Effects from the four runs", "Effekte aus den vier Läufen"), _mn.FadeIn(tags_a), _mn.FadeOut(self.savings)''')
sub('_mn.FadeOut(_mn.VGroup(*columns)), _mn.FadeOut(overlay_a), _mn.FadeOut(form_a), _mn.FadeIn(_mn.VGroup(*rows))', '_mn.FadeOut(_mn.VGroup(*columns)), _mn.FadeOut(tags_a), _mn.FadeIn(tags_b), _mn.FadeOut(overlay_a), _mn.FadeOut(form_a), _mn.FadeIn(_mn.VGroup(*rows))')
sub('self.step("difference", _mn.FadeOut(_mn.VGroup(*rows)),', 'self.step("difference", _mn.FadeOut(tags_b), _mn.FadeIn(tags_ab), _mn.FadeOut(_mn.VGroup(*rows)),')
sub('self.cleanup = _mn.VGroup(self.savings, _mn.VGroup(*diagonals), signs_ab, meaning, self.convention)', 'self.cleanup = _mn.VGroup(self.savings, _mn.VGroup(*diagonals), signs_ab, meaning, self.convention, tags_ab)')
for attempt in range(40):
    try:
        scene.write_bytes(t.replace("\r\n", "\n").encode("utf-8")); break
    except OSError:
        time.sleep(3)
sys.stdout.buffer.write(b"done\n")
